//!
//! wxStaticBox wrapper
//!

use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by XRC support for backwards compatibility
use crate::event::WxEvtHandler;
#[allow(unused_imports)]
use crate::window::Window;
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
///
/// StaticBox uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let staticbox = StaticBox::builder(&frame).label("Group").build();
///
/// // StaticBox is Copy - no clone needed for closures!
/// // After parent destruction, staticbox operations are safe no-ops
/// frame.destroy();
/// assert!(!staticbox.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct StaticBox {
    /// Safe handle to the underlying wxStaticBox - automatically invalidated on destroy
    handle: WindowHandle,
}

impl StaticBox {
    /// Creates a new StaticBox builder.
    pub fn builder<W: WxWidget>(parent: &W) -> StaticBoxBuilder<'_> {
        StaticBoxBuilder::new(parent)
    }

    /// Creates a new StaticBox from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StaticBox_t) -> Self {
        StaticBox {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the underlying WindowHandle for this staticbox.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for StaticBox (using WindowHandle)
impl WxWidget for StaticBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for StaticBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for StaticBox {}

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
        // Create a WindowHandle which automatically registers for destroy events
        StaticBox {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// XRC Support - enables StaticBox to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for StaticBox {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        StaticBox {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for StaticBox
impl crate::window::FromWindowWithClassName for StaticBox {
    fn class_name() -> &'static str {
        "wxStaticBox"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        StaticBox {
            handle: WindowHandle::new(ptr),
        }
    }
}
