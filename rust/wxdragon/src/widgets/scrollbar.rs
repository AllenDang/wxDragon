//!
//! Safe wrapper for wxScrollBar.
//!

use crate::event::ScrollEvents;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
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
#[derive(Clone, Debug)]
pub struct ScrollBar {
    window: Window,
}

impl ScrollBar {
    /// Creates a new ScrollBar builder.
    pub fn builder(parent: &dyn WxWidget) -> ScrollBarBuilder<'_> {
        ScrollBarBuilder::new(parent)
    }

    /// Creates a ScrollBar from a raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is valid and relates to a wxScrollBar.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_ScrollBar_t) -> Self {
        ScrollBar {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
        }
    }

    /// Sets the scrollbar properties.
    pub fn set_scrollbar(
        &self,
        position: i32,
        thumb_size: i32,
        range: i32,
        page_size: i32,
        refresh: bool,
    ) {
        unsafe {
            ffi::wxd_ScrollBar_SetScrollbar(
                self.window.as_ptr() as *mut ffi::wxd_ScrollBar_t,
                position as c_int,
                thumb_size as c_int,
                range as c_int,
                page_size as c_int,
                refresh,
            );
        }
    }

    /// Gets the current position of the scrollbar thumb.
    pub fn thumb_position(&self) -> i32 {
        unsafe {
            ffi::wxd_ScrollBar_GetThumbPosition(self.window.as_ptr() as *mut ffi::wxd_ScrollBar_t)
        }
    }

    // TODO: Add GetThumbSize, GetPageSize, GetRange if needed via FFI calls.
}

// Apply common trait implementations for ScrollBar
implement_widget_traits_with_target!(ScrollBar, window, Window);

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

        unsafe { ScrollBar::from_ptr(ptr) }
    }
);

// At the bottom of the file, add the ScrollEvents trait implementation
impl ScrollEvents for ScrollBar {}

// Add XRC Support - enables ScrollBar to be created from XRC-managed pointers
impl_xrc_support!(ScrollBar, { window });

// Widget casting support for ScrollBar
impl_widget_cast!(ScrollBar, "wxScrollBar", { window });
