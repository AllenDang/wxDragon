//!
//! Safe wrapper for wxScrolledWindow.

use crate::event::ScrollEvents;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::widgets::panel::Panel; // Inherits from Panel (used for Deref)
use crate::window::WxWidget; // Used for builder parent type constraint
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
pub struct ScrolledWindow {
    panel: Panel, // Composition: ScrolledWindow "is a" Panel
}

impl ScrolledWindow {
    /// Creates a new builder for a ScrolledWindow.
    pub fn builder(parent: &dyn WxWidget) -> ScrolledWindowBuilder<'_> {
        ScrolledWindowBuilder::new(parent)
    }

    /// Creates a new ScrolledWindow wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_ScrolledWindow_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_ScrolledWindow_t) -> Self {
        ScrolledWindow {
            panel: Panel::from_ptr(ptr as *mut ffi::wxd_Panel_t),
        }
    }

    /// Returns the raw underlying scrolled window pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_ScrolledWindow_t {
        self.panel.as_ptr() as *mut ffi::wxd_ScrolledWindow_t
    }

    /// Sets the scroll rate (pixels per scroll unit).
    pub fn set_scroll_rate(&self, x_step: i32, y_step: i32) {
        unsafe { ffi::wxd_ScrolledWindow_SetScrollRate(self.as_ptr(), x_step, y_step) }
    }

    /// Sets up the scrollbars.
    pub fn set_scrollbars(&self, config: ScrollBarConfig) {
        unsafe {
            ffi::wxd_ScrolledWindow_SetScrollbars(
                self.as_ptr(),
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
        unsafe { ffi::wxd_ScrolledWindow_EnableScrolling(self.as_ptr(), x_scrolling, y_scrolling) }
    }

    /// Scrolls the window to the given position (in scroll units).
    pub fn scroll_coords(&self, x: i32, y: i32) {
        unsafe { ffi::wxd_ScrolledWindow_Scroll_Coord(self.as_ptr(), x, y) }
    }

    /// Scrolls the window to the given position (in scroll units).
    pub fn scroll_point(&self, pt: Point) {
        let c_pt = ffi::wxd_Point { x: pt.x, y: pt.y };
        unsafe { ffi::wxd_ScrolledWindow_Scroll_Point(self.as_ptr(), c_pt) }
    }

    /// Gets the size of the scrollable virtual area in pixels.
    pub fn get_virtual_size(&self) -> Size {
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        unsafe { ffi::wxd_ScrolledWindow_GetVirtualSize(self.as_ptr(), &mut w, &mut h) };
        Size {
            width: w,
            height: h,
        }
    }

    /// Gets the number of pixels per scroll unit.
    pub fn get_scroll_pixels_per_unit(&self) -> (i32, i32) {
        let mut x_unit: i32 = 0;
        let mut y_unit: i32 = 0;
        unsafe {
            ffi::wxd_ScrolledWindow_GetScrollPixelsPerUnit(self.as_ptr(), &mut x_unit, &mut y_unit)
        };
        (x_unit, y_unit)
    }
}

// Apply common trait implementations
implement_widget_traits_with_target!(ScrolledWindow, panel, Panel);

// Implement scrolling functionality for ScrolledWindow
impl crate::scrollable::WxScrollable for ScrolledWindow {}

// Use widget_builder macro for the builder implementation
widget_builder!(
    name: ScrolledWindow,
    parent_type: &'a dyn WxWidget,
    style_type: ScrolledWindowStyle,
    fields: {},
    build_impl: |slf| {
        if slf.parent.handle_ptr().is_null() {
            panic!("Cannot create ScrolledWindow with a null parent");
        }

        let ptr = unsafe {
            ffi::wxd_ScrolledWindow_Create(
                slf.parent.handle_ptr(),
                slf.id,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create wxScrolledWindow");
        }

        unsafe { ScrolledWindow::from_ptr(ptr) }
    }
);

impl ScrollEvents for ScrolledWindow {}

// Add XRC Support - enables ScrolledWindow to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for ScrolledWindow {
    unsafe fn from_xrc_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
        ScrolledWindow {
            panel: unsafe { Panel::from_ptr(ptr as *mut ffi::wxd_Panel_t) },
        }
    }
}

// Manual widget casting support for ScrolledWindow - composition structure needs custom handling
impl crate::window::FromWindowWithClassName for ScrolledWindow {
    fn class_name() -> &'static str {
        "wxScrolledWindow"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ScrolledWindow {
            panel: Panel::from_ptr(ptr as *mut ffi::wxd_Panel_t),
        }
    }
}
