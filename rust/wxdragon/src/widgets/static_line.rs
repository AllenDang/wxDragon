//!
//! Safe wrapper for wxStaticLine.
//!

use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
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
///
/// StaticLine uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let line = StaticLine::builder(&frame).build();
///
/// // StaticLine is Copy - no clone needed for closures!
/// // After parent destruction, line operations are safe no-ops
/// frame.destroy();
/// assert!(!line.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct StaticLine {
    /// Safe handle to the underlying wxStaticLine - automatically invalidated on destroy
    handle: WindowHandle,
}

impl StaticLine {
    /// Creates a new StaticLine builder.
    pub fn builder<W: WxWidget>(parent: &W) -> StaticLineBuilder<'_> {
        StaticLineBuilder::new(parent)
    }

    /// Returns the underlying WindowHandle for this static line.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for StaticLine (using WindowHandle)
impl WxWidget for StaticLine {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for StaticLine {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for StaticLine {}

widget_builder!(
    name: StaticLine,
    parent_type: &'a dyn WxWidget,
    style_type: StaticLineStyle,
    fields: {
        name: String = "staticLine".to_string()
    },
    build_impl: |slf| {
        let c_name = CString::new(slf.name.as_str()).expect("CString::new failed for name");
        let ptr = unsafe {
            ffi::wxd_StaticLine_Create(
                slf.parent.handle_ptr(),
                slf.id as c_int,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
                c_name.as_ptr(),
            )
        };

        if ptr.is_null() {
            panic!("wxd_StaticLine_Create returned null");
        }

        // Create a WindowHandle which automatically registers for destroy events
        StaticLine {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// XRC Support - enables StaticLine to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for StaticLine {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        StaticLine {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for StaticLine
impl crate::window::FromWindowWithClassName for StaticLine {
    fn class_name() -> &'static str {
        "wxStaticLine"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        StaticLine {
            handle: WindowHandle::new(ptr),
        }
    }
}
