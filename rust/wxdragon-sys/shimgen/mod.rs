//! Generates C shim functions from the `.wxd` spec files in `cpp/spec`.
//!
//! Most of the shim is mechanical: cast the handle, check it for null, convert the
//! arguments, call one wxWidgets method and convert the result. The spec files describe
//! those functions and this module writes their declarations (`wxd_generated.h`, included
//! by `wxdragon.h` and so seen by bindgen) and definitions (one `.cpp` per spec file, built
//! by CMake). Anything that is not mechanical stays hand-written in `cpp/src`.
//!
//! `parse.rs` reads the spec files. This module decides what each type means in C and C++,
//! and the templates in `templates/` lay out the generated files.
//!
//! # Spec format
//!
//! ```text
//! include wx/checkbox.h                  # extra header for the generated .cpp
//!
//! class CheckBox : wxCheckBox            # handle wxd_CheckBox_t, wraps wxCheckBox
//!     create(Window parent, Id id, string label, Point pos, Size size, Style style)
//!     bool IsChecked()                    # wxd_CheckBox_IsChecked(wxd_CheckBox_t* self)
//!     string GetLabel() const             # const self, returned through a buffer
//!     int GetValue() = -1                 # value returned for a null handle
//!     void Foo(int a) => self->Bar(a * 2) # custom call expression
//! ```
//!
//! `include` lines come before the first `class`. `#` starts a comment anywhere outside a
//! C++ expression.
//!
//! Parameter types:
//! - `int`, `bool`, `double`, `Id`, `Style`, ... are passed through unchanged.
//! - `string` is a `const char*` converted with `wxString::FromUTF8` (null reads as "").
//! - `string? name = EXPR` falls back to the C++ expression `EXPR` when null.
//! - `Point` and `Size` are `wxd_Point`/`wxd_Size` converted to `wxPoint`/`wxSize`.
//! - `Window` is a `wxd_Window_t*` handle; a null handle makes the function return early.
//!
//! Return types are the pass-through types, `void`, `Point`, `Size` and `string`. A
//! `string` return becomes an `int` result plus trailing `char* buffer, size_t buffer_len`
//! parameters, filled by `wxd_cpp_utils::copy_wxstring_to_buffer` (-1 for a null handle).
//!
//! In a `=> EXPR` override, `self` is the typed wxWidgets pointer and each parameter name
//! refers to its converted value. For `create`, `EXPR` is the `new` expression.
//!
//! A C++ expression (a fallback, a null default or an override) may contain commas, `=` and
//! `#` inside brackets or string literals. Angle brackets are not tracked, so a template
//! argument list must not contain a comma.

mod parse;

use minijinja::syntax::SyntaxConfig;
use minijinja::{Environment, UndefinedBehavior, Value, context};
use std::fs;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, String>;

/// Generates the shim from every `.wxd` file in `spec_dir` into `out_dir/include` and
/// `out_dir/src`. Files are only rewritten when their content changes.
pub fn generate(spec_dir: &Path, out_dir: &Path) -> Result<()> {
    let mut specs: Vec<PathBuf> = fs::read_dir(spec_dir)
        .map_err(|e| format!("{}: {e}", spec_dir.display()))?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "wxd"))
        .collect();
    specs.sort();

    let include_dir = out_dir.join("include");
    let src_dir = out_dir.join("src");
    fs::create_dir_all(&include_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&src_dir).map_err(|e| e.to_string())?;

    let env = environment()?;
    let mut header_specs = Vec::new();
    let mut expected_sources = Vec::new();

    for path in &specs {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let spec = parse::parse(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        let stem = path.file_stem().unwrap().to_string_lossy();
        let spec_name = format!("{stem}.wxd");
        let funcs = spec
            .classes
            .iter()
            .flat_map(|class| class.funcs.iter().map(move |f| func_context(class, f)))
            .collect::<Result<Vec<_>>>()?;
        let ctx = context! { spec_name, includes => spec.includes, funcs };
        let source = render(&env, "shim.cpp", ctx.clone())?;
        header_specs.push(ctx);

        let name = format!("{stem}.cpp");
        write_if_changed(&src_dir.join(&name), &source)?;
        expected_sources.push(name);
    }

    let header = render(&env, "shim.h", context! { specs => header_specs })?;
    write_if_changed(&include_dir.join("wxd_generated.h"), &header)?;

    // Remove sources whose spec file was deleted so CMake does not keep building them.
    for entry in fs::read_dir(&src_dir).map_err(|e| e.to_string())?.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !expected_sources.contains(&name) {
            let _ = fs::remove_file(entry.path());
        }
    }
    Ok(())
}

fn write_if_changed(path: &Path, content: &str) -> Result<()> {
    if fs::read_to_string(path).is_ok_and(|old| old == content) {
        return Ok(());
    }
    fs::write(path, content).map_err(|e| format!("{}: {e}", path.display()))
}

fn environment() -> Result<Environment<'static>> {
    let mut env = Environment::new();
    let syntax = SyntaxConfig::builder()
        .trim_blocks(true)
        .lstrip_blocks(true)
        .keep_trailing_newline(true)
        .build()
        .map_err(|e| e.to_string())?;
    env.set_syntax(syntax);
    env.set_undefined_behavior(UndefinedBehavior::Strict);
    env.add_template("shim.h", include_str!("templates/shim.h.j2"))
        .map_err(|e| e.to_string())?;
    env.add_template("shim.cpp", include_str!("templates/shim.cpp.j2"))
        .map_err(|e| e.to_string())?;
    Ok(env)
}

fn render(env: &Environment, name: &str, ctx: Value) -> Result<String> {
    env.get_template(name)
        .and_then(|t| t.render(ctx))
        .map_err(|e| format!("{name}: {e:#}"))
}

// --- Spec model -------------------------------------------------------------

struct Spec {
    includes: Vec<String>,
    classes: Vec<Class>,
}

struct Class {
    /// Name used in `wxd_<name>_t` and `wxd_<name>_<Method>`.
    name: String,
    /// The wxWidgets class the handle points to.
    wx_class: String,
    funcs: Vec<Func>,
}

struct Func {
    /// `None` for `create`.
    ret: Option<Type>,
    name: String,
    params: Vec<Param>,
    is_const: bool,
    null_default: Option<String>,
    call: Option<String>,
}

struct Param {
    ty: Type,
    name: String,
    /// For `string?`: the C++ expression used when the argument is null.
    fallback: Option<String>,
}

#[derive(Clone, PartialEq)]
enum Type {
    Void,
    /// A C type passed through unchanged, e.g. `int` or `wxd_Id`.
    Plain(String),
    String,
    OptString,
    Point,
    Size,
    /// A handle to another wrapped object: (`wxd_X_t` name, wxWidgets class).
    Handle(String, String),
}

fn parse_type(word: &str) -> Option<Type> {
    let plain = |c: &str| Some(Type::Plain(c.to_string()));
    match word {
        "void" => Some(Type::Void),
        "bool" | "int" | "double" | "size_t" | "int32_t" | "int64_t" | "uint32_t" | "uint64_t" => plain(word),
        "unsigned" => plain("unsigned int"),
        "long" => plain("wxd_Long_t"),
        "Id" => plain("wxd_Id"),
        "Style" => plain("wxd_Style_t"),
        "string" => Some(Type::String),
        "string?" => Some(Type::OptString),
        "Point" => Some(Type::Point),
        "Size" => Some(Type::Size),
        "Window" => Some(Type::Handle("Window".into(), "wxWindow".into())),
        _ => None,
    }
}

impl Type {
    /// The C type used in the declaration.
    fn c_type(&self) -> String {
        match self {
            Type::Void => "void".into(),
            Type::Plain(c) => c.clone(),
            Type::String | Type::OptString => "const char*".into(),
            Type::Point => "wxd_Point".into(),
            Type::Size => "wxd_Size".into(),
            Type::Handle(name, _) => format!("wxd_{name}_t*"),
        }
    }

    /// The value returned for a null handle when the spec does not give one.
    fn null_default(&self) -> Option<&'static str> {
        match self {
            Type::Void => None,
            Type::Plain(c) if c == "bool" => Some("false"),
            Type::Plain(_) => Some("0"),
            Type::String => Some("-1"),
            Type::Point => Some("wxd_Point{0, 0}"),
            Type::Size => Some("wxd_Size{0, 0}"),
            Type::Handle(..) => Some("nullptr"),
            Type::OptString => None,
        }
    }
}

// --- Template context -------------------------------------------------------

/// Everything the templates need to write one function. The type-dependent parts are
/// decided here, so the templates only lay them out.
fn func_context(class: &Class, f: &Func) -> Result<Value> {
    let (decl_params, def_params) = signature(class, f);
    let ret = match &f.ret {
        None => format!("wxd_{}_t*", class.name),
        Some(Type::String) => "int".to_string(),
        Some(t) => t.c_type(),
    };
    let null_return = match &f.null_default {
        Some(d) => Some(d.clone()),
        None => match &f.ret {
            None => Some("nullptr".to_string()),
            Some(t) => t.null_default().map(String::from),
        },
    };

    // Null checks for the handle and every required handle parameter.
    let mut null_checks = Vec::new();
    if f.ret.is_some() {
        null_checks.push("!self_in".to_string());
    }
    for p in &f.params {
        if matches!(p.ty, Type::Handle(..)) {
            null_checks.push(format!("!{}_in", p.name));
        }
    }

    let self_type = f.ret.as_ref().map(|_| {
        let constness = if f.is_const { "const " } else { "" };
        format!("{constness}{}", class.wx_class)
    });

    let conversions: Vec<String> = f
        .params
        .iter()
        .filter_map(|p| {
            let n = &p.name;
            Some(match &p.ty {
                Type::Plain(_) | Type::Void => return None,
                Type::String => format!("wxString {n} = wxString::FromUTF8({n}_in ? {n}_in : \"\");"),
                Type::OptString => format!(
                    "wxString {n} = {n}_in ? wxString::FromUTF8({n}_in) : wxString({});",
                    p.fallback.as_deref().unwrap()
                ),
                Type::Point => format!("wxPoint {n}({n}_in.x, {n}_in.y);"),
                Type::Size => format!("wxSize {n}({n}_in.width, {n}_in.height);"),
                Type::Handle(_, wx) => format!("{wx}* {n} = reinterpret_cast<{wx}*>({n}_in);"),
            })
        })
        .collect();

    let args = f.params.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ");
    let call = match (&f.call, &f.ret) {
        (Some(c), _) => c.clone(),
        (None, None) => format!("new {}({args})", class.wx_class),
        (None, Some(_)) => format!("self->{}({args})", f.name),
    };
    let result = match &f.ret {
        None => vec![format!("return reinterpret_cast<wxd_{}_t*>({call});", class.name)],
        Some(Type::Void) => vec![format!("{call};")],
        Some(Type::Plain(_)) => vec![format!("return {call};")],
        Some(Type::String) => vec![format!(
            "return static_cast<int>(wxd_cpp_utils::copy_wxstring_to_buffer({call}, buffer, buffer_len));"
        )],
        Some(Type::Point) => vec![
            format!("wxPoint result = {call};"),
            "return wxd_Point{result.x, result.y};".to_string(),
        ],
        Some(Type::Size) => vec![
            format!("wxSize result = {call};"),
            "return wxd_Size{result.GetWidth(), result.GetHeight()};".to_string(),
        ],
        Some(Type::Handle(..)) | Some(Type::OptString) => {
            return Err(format!("{}: unsupported return type", f.name));
        }
    };

    Ok(context! {
        ret,
        name => format!("wxd_{}_{}", class.name, f.name),
        decl_params,
        def_params,
        null_checks,
        null_return,
        self_type,
        conversions,
        result,
    })
}

/// Parameter lists for the declaration and the definition. The definition renames the
/// parameters that need converting (`label` -> `label_in`) so the converted locals can take
/// the spec names, which is what a `=> EXPR` override refers to.
fn signature(class: &Class, f: &Func) -> (Vec<String>, Vec<String>) {
    let mut decl = Vec::new();
    let mut def = Vec::new();
    if f.ret.is_some() {
        let constness = if f.is_const { "const " } else { "" };
        decl.push(format!("{constness}wxd_{}_t* self", class.name));
        def.push(format!("{constness}wxd_{}_t* self_in", class.name));
    }
    for p in &f.params {
        let c = p.ty.c_type();
        decl.push(format!("{c} {}", p.name));
        let suffix = if matches!(p.ty, Type::Plain(_)) { "" } else { "_in" };
        def.push(format!("{c} {}{suffix}", p.name));
    }
    if f.ret == Some(Type::String) {
        for list in [&mut decl, &mut def] {
            list.push("char* buffer".into());
            list.push("size_t buffer_len".into());
        }
    }
    (decl, def)
}
