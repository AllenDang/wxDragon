//! Parser for the `.wxd` spec format described in `mod.rs`.

use winnow::ascii::{line_ending, space0, space1};
use winnow::combinator::{alt, cut_err, delimited, eof, not, opt, peek, preceded, repeat, separated, terminated};
use winnow::error::{ContextError, StrContext, StrContextValue};
use winnow::prelude::*;
use winnow::token::{any, none_of, take_till, take_while};

use super::{Class, Func, Param, Spec, Type, parse_type};

/// Parses a whole spec file. Errors point at the offending line and column.
pub fn parse(text: &str) -> Result<Spec, String> {
    spec.parse(text).map_err(|e| e.to_string())
}

fn spec(input: &mut &str) -> ModalResult<Spec> {
    blank_lines(input)?;
    let includes = repeat(0.., include).parse_next(input)?;
    let classes = repeat(0.., class).parse_next(input)?;
    (space0, opt(comment), eof)
        .context(expected("a `class` line (includes come before the first class)"))
        .parse_next(input)?;
    Ok(Spec { includes, classes })
}

/// `include <path>`
fn include(input: &mut &str) -> ModalResult<String> {
    preceded(
        (space0, "include", space1),
        cut_err(terminated(take_till(1.., [' ', '\t', '#', '\r', '\n']), end_of_line)),
    )
    .map(String::from)
    .context(label("include"))
    .parse_next(input)
}

/// `class <Name> : <wxClass>` followed by its function lines.
fn class(input: &mut &str) -> ModalResult<Class> {
    let (name, wx_class) = preceded(
        (space0, "class", space1),
        cut_err(terminated((ident, space0, ':', space0, cpp_name), end_of_line)),
    )
    .map(|(name, _, _, _, wx_class)| (name, wx_class))
    .context(label("class"))
    .context(expected("`class <Name> : <wxClass>`"))
    .parse_next(input)?;
    let funcs = repeat(0.., func_line).parse_next(input)?;
    Ok(Class {
        name: name.into(),
        wx_class: wx_class.into(),
        funcs,
    })
}

/// Any line inside a class that does not start the next `class` (or a misplaced `include`)
/// must be a function.
fn func_line(input: &mut &str) -> ModalResult<Func> {
    preceded(
        (space0, not((alt(("class", "include")), space1)), not(eof)),
        cut_err(terminated(func, end_of_line)),
    )
    .parse_next(input)
}

/// `<ret> <Name>(<params>) [const] [= <null default>] [=> <call>]` or `create(<params>) ...`
fn func(input: &mut &str) -> ModalResult<Func> {
    let (ret, name) = alt((
        terminated("create", peek((space0, '('))).value((None, "Create")),
        (ty, space1, ident).map(|(ret, _, name)| (Some(ret), name)),
    ))
    .context(expected("`<return type> <Name>(...)` or `create(...)`"))
    .parse_next(input)?;
    let params = delimited(
        (space0, '(', space0),
        alt((
            peek(')').map(|_| Vec::new()),
            separated(1.., cut_err(param), (space0, ',', space0)),
        )),
        (space0, ')').context(expected("`)`")),
    )
    .parse_next(input)?;
    let is_const = opt((space0, "const")).parse_next(input)?.is_some();
    let null_default = opt(preceded((space0, '=', not('>'), space0), expr)).parse_next(input)?;
    let call = opt(preceded((space0, "=>", space0), expr)).parse_next(input)?;
    Ok(Func {
        ret,
        name: name.into(),
        params,
        is_const,
        null_default: null_default.map(String::from),
        call: call.map(String::from),
    })
}

/// `<type> <name>`, or `string? <name> = <fallback>`.
fn param(input: &mut &str) -> ModalResult<Param> {
    (ty, space1, ident, opt(preceded((space0, '=', space0), expr)))
        .verify(|(ty, _, _, fallback)| fallback.is_some() == (*ty == Type::OptString))
        .map(|(ty, _, name, fallback)| Param {
            ty,
            name: name.into(),
            fallback: fallback.map(String::from),
        })
        .context(label("parameter"))
        .context(expected("`<type> <name>`, or `string? <name> = <fallback>`"))
        .parse_next(input)
}

fn ty(input: &mut &str) -> ModalResult<Type> {
    (ident, opt('?'))
        .take()
        .verify_map(parse_type)
        .context(label("type"))
        .parse_next(input)
}

fn ident<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    take_while(1.., |c: char| c.is_ascii_alphanumeric() || c == '_').parse_next(input)
}

/// A C++ class name, which may be qualified (`ns::Foo`).
fn cpp_name<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    take_while(1.., |c: char| c.is_ascii_alphanumeric() || c == '_' || c == ':').parse_next(input)
}

/// A C++ expression. It ends at a `,`, `)`, `=>`, `#` or line ending that is not inside
/// brackets or a literal, so `wxString("a, b")` and `f(x, y)` read as one expression.
/// Angle brackets are not tracked, so a template argument list must not contain a comma.
fn expr<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    repeat(
        1..,
        alt((
            group,
            quoted('"'),
            quoted('\''),
            ('=', not('>')).void(),
            take_till(1.., ['(', ')', '[', ']', '{', '}', '"', '\'', '=', ',', '#', '\r', '\n']).void(),
        )),
    )
    .map(|()| ())
    .take()
    .map(str::trim_end)
    .context(label("expression"))
    .parse_next(input)
}

/// A bracketed group, which may contain anything except unbalanced brackets.
fn group(input: &mut &str) -> ModalResult<()> {
    let inner = || {
        repeat(
            0..,
            alt((
                group,
                quoted('"'),
                quoted('\''),
                take_till(1.., ['(', ')', '[', ']', '{', '}', '"', '\'', '\r', '\n']).void(),
            )),
        )
        .map(|()| ())
    };
    alt((
        delimited('(', inner(), cut_err(')')),
        delimited('[', inner(), cut_err(']')),
        delimited('{', inner(), cut_err('}')),
    ))
    .parse_next(input)
}

/// A string or character literal, with backslash escapes.
fn quoted<'s>(quote: char) -> impl ModalParser<&'s str, (), ContextError> {
    delimited(
        quote,
        repeat(0.., alt((('\\', any).void(), none_of([quote, '\\', '\r', '\n']).void()))).map(|()| ()),
        cut_err(quote),
    )
}

fn comment(input: &mut &str) -> ModalResult<()> {
    ('#', take_till(0.., ['\r', '\n'])).void().parse_next(input)
}

/// The end of a line, with an optional comment, and any blank or comment lines after it.
fn end_of_line(input: &mut &str) -> ModalResult<()> {
    (space0, opt(comment), alt((line_ending.void(), eof.void())))
        .context(expected("end of line"))
        .parse_next(input)?;
    blank_lines(input)
}

fn blank_lines(input: &mut &str) -> ModalResult<()> {
    repeat(0.., (space0, opt(comment), line_ending))
        .map(|()| ())
        .parse_next(input)
}

fn label(text: &'static str) -> StrContext {
    StrContext::Label(text)
}

fn expected(text: &'static str) -> StrContext {
    StrContext::Expected(StrContextValue::Description(text))
}
