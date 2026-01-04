//!
//! Safe wrapper for wxScrolledWindow.

use crate::event::{MenuEvents, ScrollEvents, WindowEvents, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use wxdragon_sys as ffi;

// --- Style enum using macro ---
widget_style_enum!(
    name: ScrolledWindowStyle,
    doc: "Style flags for ScrolledWindow",
    variants: {
        Default: 0, "Default style.",
        HScroll: ffi::WXD_HSCROLL, "Includes horizontal scrollbar.",
        VScroll: ffi::WXD_VSCROLL, "Includes vertical scrollbar."
    },
    default_variant: Default
);

/// Configuration for setting up scrollbars
pub struct ScrollBarConfig {
    pub pixels_per_unit_x: i32,
    pub pixels_per_unit_y: i32,
    pub no_units_x: i32,
    pub no_units_y: i32,
    pub x_pos: i32,
    pub y_pos: i32,
    pub no_refresh: bool,
}

/// Represents a wxScrolledWindow widget.
/// A window that can scroll its contents.
///
/// ScrolledWindow uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let scrolled = ScrolledWindow::builder(&frame).build();
///
/// // ScrolledWindow is Copy - no clone needed for closures!
/// scrolled.bind_event(move |_| {
///     // Safe: if scrolled was destroyed, this is a no-op
///     scrolled.set_scroll_rate(10, 10);
/// });
///
/// // After parent destruction, scrolled operations are safe no-ops
/// frame.destroy();
/// assert!(!scrolled.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct ScrolledWindow {
    /// Safe handle to the underlying wxScrolledWindow - automatically invalidated on destroy
    handle: WindowHandle,
}

impl ScrolledWindow {
    /// Creates a new builder for a ScrolledWindow.
    pub fn builder(parent: &dyn WxWidget) -> ScrolledWindowBuilder<'_> {
        ScrolledWindowBuilder::new(parent)
    }

    /// Creates a new ScrolledWindow wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_ScrolledWindow_t` pointer.
    /// Ownership is typically managed by the parent window in wxWidgets.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_ScrolledWindow_t) -> Self {
        assert!(!ptr.is_null());
        ScrolledWindow {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the raw underlying scrolled window pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_ScrolledWindow_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_ScrolledWindow_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Helper to get raw scrolled window pointer, returns null if widget has been destroyed
    #[inline]
    fn scrolled_window_ptr(&self) -> *mut ffi::wxd_ScrolledWindow_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_ScrolledWindow_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Returns the underlying WindowHandle for this scrolled window.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    /// Sets the scroll rate (pixels per scroll unit).
    pub fn set_scroll_rate(&self, x_step: i32, y_step: i32) {
        let ptr = self.scrolled_window_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ScrolledWindow_SetScrollRate(ptr, x_step, y_step) }
    }

    /// Sets up the scrollbars.
    pub fn set_scrollbars(&self, config: ScrollBarConfig) {
        let ptr = self.scrolled_window_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_ScrolledWindow_SetScrollbars(
                ptr,
                config.pixels_per_unit_x,
                config.pixels_per_unit_y,
                config.no_units_x,
                config.no_units_y,
                config.x_pos,
                config.y_pos,
                config.no_refresh,
            )
        }
    }

    /// Enables or disables scrolling for the specified orientation(s).
    pub fn enable_scrolling(&self, x_scrolling: bool, y_scrolling: bool) {
        let ptr = self.scrolled_window_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ScrolledWindow_EnableScrolling(ptr, x_scrolling, y_scrolling) }
    }

    /// Scrolls the window to the given position (in scroll units).
    pub fn scroll_coords(&self, x: i32, y: i32) {
        let ptr = self.scrolled_window_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ScrolledWindow_Scroll_Coord(ptr, x, y) }
    }

    /// Scrolls the window to the given position (in scroll units).
    pub fn scroll_point(&self, pt: Point) {
        let ptr = self.scrolled_window_ptr();
        if ptr.is_null() {
            return;
        }
        let c_pt = ffi::wxd_Point { x: pt.x, y: pt.y };
        unsafe { ffi::wxd_ScrolledWindow_Scroll_Point(ptr, c_pt) }
    }

    /// Gets the size of the scrollable virtual area in pixels.
    pub fn get_virtual_size(&self) -> Size {
        let ptr = self.scrolled_window_ptr();
        if ptr.is_null() {
            return Size { width: 0, height: 0 };
        }
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        unsafe { ffi::wxd_ScrolledWindow_GetVirtualSize(ptr, &mut w, &mut h) };
        Size { width: w, height: h }
    }

    /// Gets the number of pixels per scroll unit.
    pub fn get_scroll_pixels_per_unit(&self) -> (i32, i32) {
        let ptr = self.scrolled_window_ptr();
        if ptr.is_null() {
            return (0, 0);
        }
        let mut x_unit: i32 = 0;
        let mut y_unit: i32 = 0;
        unsafe { ffi::wxd_ScrolledWindow_GetScrollPixelsPerUnit(ptr, &mut x_unit, &mut y_unit) };
        (x_unit, y_unit)
    }
}

// Manual WxWidget implementation for ScrolledWindow (using WindowHandle)
impl WxWidget for ScrolledWindow {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for ScrolledWindow {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl WindowEvents for ScrolledWindow {}
impl MenuEvents for ScrolledWindow {}

// Implement scrolling functionality for ScrolledWindow
impl crate::scrollable::WxScrollable for ScrolledWindow {}

// Use widget_builder macro for the builder implementation
widget_builder!(
    name: ScrolledWindow,
    parent_type: &'a dyn WxWidget,
    style_type: ScrolledWindowStyle,
    fields: {},
    build_impl: |slf| {
        let scrolled_window_ptr = unsafe {
            ffi::wxd_ScrolledWindow_Create(
                slf.parent.handle_ptr(),
                slf.id,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            )
        };

        if scrolled_window_ptr.is_null() {
            panic!("Failed to create ScrolledWindow: FFI returned null pointer.");
        }

        unsafe { ScrolledWindow::from_ptr(scrolled_window_ptr) }
    }
);

impl ScrollEvents for ScrolledWindow {}

// Add XRC Support - enables ScrolledWindow to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for ScrolledWindow {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ScrolledWindow {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Widget casting support for ScrolledWindow
impl crate::window::FromWindowWithClassName for ScrolledWindow {
    fn class_name() -> &'static str {
        "wxScrolledWindow"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ScrolledWindow {
            handle: WindowHandle::new(ptr),
        }
    }
}
