//!
//! wxStaticBox wrapper
//!

use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

widget_style_enum!(
    name: StaticBoxStyle,
    doc: "Style flags for the StaticBox widget.",
    variants: {
        Default: 0, "Default style with no special behavior."
    },
    default_variant: Default
);

/// Represents the wxStaticBox widget.
#[derive(Clone)]
pub struct StaticBox {
    window: Window,
}

impl StaticBox {
    /// Creates a new StaticBox builder.
    pub fn builder<W: WxWidget>(parent: &W) -> StaticBoxBuilder<'_> {
        StaticBoxBuilder::new(parent)
    }

    /// Creates a new StaticBox from a raw pointer.
    /// Does NOT assume ownership.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StaticBox_t) -> Self {
        StaticBox {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

widget_builder!(
    name: StaticBox,
    parent_type: &'a dyn WxWidget,
    style_type: StaticBoxStyle,
    fields: {
        label: String = String::new()
    },
    build_impl: |slf| {
        let c_label = CString::new(&slf.label[..]).unwrap_or_default();
        let ptr = unsafe {
            ffi::wxd_StaticBox_Create(
                slf.parent.handle_ptr(),
                slf.id as c_int,
                c_label.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create StaticBox");
        }
        unsafe { StaticBox::from_ptr(ptr) }
    }
);

// Now we can use the implement_widget_traits_with_target macro
implement_widget_traits_with_target!(StaticBox, window, Window);

// Add XRC Support - enables StaticBox to be created from XRC-managed pointers
impl_xrc_support!(StaticBox, { window });

// Widget casting support for StaticBox
impl_widget_cast!(StaticBox, "wxStaticBox", { window });
