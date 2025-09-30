//!
//! Safe wrapper for wxStaticLine.
//!

use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

widget_style_enum!(
    name: StaticLineStyle,
    doc: "Style flags for StaticLine.",
    variants: {
        Default: ffi::WXD_HORIZONTAL, "Default style (horizontal line).",
        Vertical: ffi::WXD_VERTICAL, "Vertical line."
    },
    default_variant: Default
);

/// Represents a wxStaticLine widget.
pub struct StaticLine {
    window: Window,
}

impl StaticLine {
    /// Creates a new StaticLine builder.
    pub fn builder<W: WxWidget>(parent: &W) -> StaticLineBuilder<'_> {
        StaticLineBuilder::new(parent)
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StaticLine_t) -> Self {
        StaticLine {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

widget_builder!(
    name: StaticLine,
    parent_type: &'a dyn WxWidget,
    style_type: StaticLineStyle,
    fields: {
        name: String = "staticLine".to_string()
    },
    build_impl: |slf| {
        let c_name = CString::new(slf.name.as_str()).expect("CString::new failed for name");
        unsafe {
            let ptr = ffi::wxd_StaticLine_Create(
                slf.parent.handle_ptr(),
                slf.id as c_int,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
                c_name.as_ptr(),
            );
            if ptr.is_null() {
                panic!("wxd_StaticLine_Create returned null");
            }
            StaticLine::from_ptr(ptr)
        }
    }
);

// Use the macro to implement all the standard traits
implement_widget_traits_with_target!(StaticLine, window, Window);

// Add XRC Support - enables StaticLine to be created from XRC-managed pointers
impl_xrc_support!(StaticLine, { window });

// Widget casting support for StaticLine
impl_widget_cast!(StaticLine, "wxStaticLine", { window });
