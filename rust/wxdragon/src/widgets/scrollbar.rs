//!
//! Safe wrapper for wxScrollBar.
//!

use crate::event::{ScrollEvents, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by impl_xrc_support for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// --- Style enum using macro ---
widget_style_enum!(
    name: ScrollBarStyle,
    doc: "Style flags for ScrollBar",
    variants: {
        Default: ffi::WXD_SB_HORIZONTAL, "Default style (horizontal).",
        Vertical: ffi::WXD_SB_VERTICAL, "Vertical scrollbar."
    },
    default_variant: Default
);

/// Represents a wxScrollBar widget.
///
/// ScrollBar uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let scrollbar = ScrollBar::builder(&frame).build();
///
/// // ScrollBar is Copy - no clone needed for closures!
/// scrollbar.bind_scroll(move |_| {
///     // Safe: if scrollbar was destroyed, this is a no-op
///     let pos = scrollbar.thumb_position();
/// });
///
/// // After parent destruction, scrollbar operations are safe no-ops
/// frame.destroy();
/// assert!(!scrollbar.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct ScrollBar {
    /// Safe handle to the underlying wxScrollBar - automatically invalidated on destroy
    handle: WindowHandle,
}

impl ScrollBar {
    /// Creates a new ScrollBar builder.
    pub fn builder(parent: &dyn WxWidget) -> ScrollBarBuilder<'_> {
        ScrollBarBuilder::new(parent)
    }

    /// Helper to get raw scrollbar pointer, returns null if widget has been destroyed
    #[inline]
    fn scrollbar_ptr(&self) -> *mut ffi::wxd_ScrollBar_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_ScrollBar_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Sets the scrollbar properties.
    /// No-op if the scrollbar has been destroyed.
    pub fn set_scrollbar(&self, position: i32, thumb_size: i32, range: i32, page_size: i32, refresh: bool) {
        let ptr = self.scrollbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_ScrollBar_SetScrollbar(
                ptr,
                position as c_int,
                thumb_size as c_int,
                range as c_int,
                page_size as c_int,
                refresh,
            )
        };
    }

    /// Gets the current position of the scrollbar thumb.
    /// Returns 0 if the scrollbar has been destroyed.
    pub fn thumb_position(&self) -> i32 {
        let ptr = self.scrollbar_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_ScrollBar_GetThumbPosition(ptr) }
    }

    /// Returns the underlying WindowHandle for this scrollbar.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    // TODO: Add GetThumbSize, GetPageSize, GetRange if needed via FFI calls.
}

// Implement ScrollEvents trait for ScrollBar
impl ScrollEvents for ScrollBar {}

// Manual WxWidget implementation for ScrollBar (using WindowHandle)
impl WxWidget for ScrollBar {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for ScrollBar {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for ScrollBar {}

// Use the widget_builder macro to generate the ScrollBarBuilder implementation
widget_builder!(
    name: ScrollBar,
    parent_type: &'a dyn WxWidget,
    style_type: ScrollBarStyle,
    fields: {
        name: String = "scrollBar".to_string()
    },
    build_impl: |slf| {
        let c_name = CString::new(slf.name.as_str()).expect("CString::new failed for name");

        // Call the FFI function
        let ptr = unsafe {
            ffi::wxd_ScrollBar_Create(
                slf.parent.handle_ptr(),
                slf.id,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
                c_name.as_ptr(),
            )
        };

        if ptr.is_null() {
            panic!("Failed to create ScrollBar: FFI returned null pointer");
        }

        // Create a WindowHandle which automatically registers for destroy events
        ScrollBar {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// XRC Support - enables ScrollBar to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for ScrollBar {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ScrollBar {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Widget casting support for ScrollBar
impl crate::window::FromWindowWithClassName for ScrollBar {
    fn class_name() -> &'static str {
        "wxScrollBar"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ScrollBar {
            handle: WindowHandle::new(ptr),
        }
    }
}
