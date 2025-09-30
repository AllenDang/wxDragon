//!
//! Safe wrapper for wxBitmapButton.

use crate::bitmap::Bitmap;
use crate::event::button_events::ButtonEvents;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// Define BitmapButtonStyle using the widget_style_enum macro
widget_style_enum!(
    name: BitmapButtonStyle,
    doc: "Style flags for BitmapButton.",
    variants: {
        Default: 0, "Default style (no specific alignment or flags).",
        Left: ffi::WXD_BU_LEFT, "Align the bitmap and/or label to the left.",
        Top: ffi::WXD_BU_TOP, "Align the bitmap and/or label to the top.",
        Right: ffi::WXD_BU_RIGHT, "Align the bitmap and/or label to the right.",
        Bottom: ffi::WXD_BU_BOTTOM, "Align the bitmap and/or label to the bottom.",
        ExactFit: ffi::WXD_BU_EXACTFIT, "Button size will be adjusted to exactly fit the bitmap.",
        NoText: ffi::WXD_BU_NOTEXT, "Do not display a label (useful for bitmap-only buttons).",
        BorderNone: ffi::WXD_BORDER_NONE, "No border."
    },
    default_variant: Default
);

/// Represents a wxBitmapButton widget.
/// This is a button that displays a bitmap instead of a text label.
pub struct BitmapButton {
    window: Window, // Inherits basic window properties
}

// Implement ButtonEvents trait for BitmapButton
impl ButtonEvents for BitmapButton {}

/// Configuration for creating a BitmapButton
#[derive(Debug)]
struct BitmapButtonConfig {
    pub parent_ptr: *mut ffi::wxd_Window_t,
    pub id: Id,
    pub bitmap_ptr: *mut ffi::wxd_Bitmap_t,
    pub pos: Point,
    pub size: Size,
    pub style: i64,
    pub name: String,
    pub bmp_disabled_ptr: *mut ffi::wxd_Bitmap_t,
    pub bmp_focus_ptr: *mut ffi::wxd_Bitmap_t,
    pub bmp_hover_ptr: *mut ffi::wxd_Bitmap_t,
}

impl BitmapButton {
    /// Creates a new BitmapButton builder.
    pub fn builder(parent: &dyn WxWidget) -> BitmapButtonBuilder<'_> {
        BitmapButtonBuilder::new(parent)
    }

    /// Creates a new BitmapButton wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_BitmapButton_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_BitmapButton_t) -> Self {
        BitmapButton {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Low-level constructor used by the builder.
    fn new_impl(config: BitmapButtonConfig) -> Self {
        let c_name = CString::new(config.name).unwrap_or_default();

        unsafe {
            let ptr = ffi::wxd_BitmapButton_Create(
                config.parent_ptr,
                config.id as c_int,
                config.bitmap_ptr,
                config.pos.into(),
                config.size.into(),
                config.style as ffi::wxd_Style_t,
                c_name.as_ptr(),
                config.bmp_disabled_ptr,
                config.bmp_focus_ptr,
                config.bmp_hover_ptr,
            );

            if ptr.is_null() {
                panic!("Failed to create BitmapButton widget");
            } else {
                BitmapButton::from_ptr(ptr)
            }
        }
    }
}

widget_builder!(
    name: BitmapButton,
    parent_type: &'a dyn WxWidget,
    style_type: BitmapButtonStyle,
    fields: {
        bitmap: Option<Bitmap> = None,
        bitmap_disabled: Option<Bitmap> = None,
        bitmap_focus: Option<Bitmap> = None,
        bitmap_hover: Option<Bitmap> = None,
        name: String = "BitmapButton".to_string()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let bitmap_ptr = match &slf.bitmap {
            Some(bitmap) => bitmap.as_ptr(),
            None => panic!("BitmapButton requires a bitmap to be set"),
        };

        let bmp_disabled_ptr = slf.bitmap_disabled
            .as_ref()
            .map_or(std::ptr::null_mut(), |b| b.as_ptr());
        let bmp_focus_ptr = slf.bitmap_focus
            .as_ref()
            .map_or(std::ptr::null_mut(), |b| b.as_ptr());
        let bmp_hover_ptr = slf.bitmap_hover
            .as_ref()
            .map_or(std::ptr::null_mut(), |b| b.as_ptr());

        // For BitmapButton, size is often best derived from the bitmap if not explicitly set
        // and if a bitmap is provided
        let final_size = if slf.size.width == -1 && slf.size.height == -1 {
            if let Some(bmp) = &slf.bitmap {
                Size::new(bmp.get_width(), bmp.get_height())
            } else {
                slf.size
            }
        } else {
            slf.size
        };

        let config = BitmapButtonConfig {
            parent_ptr,
            id: slf.id,
            bitmap_ptr,
            pos: slf.pos,
            size: final_size,
            style: slf.style.bits(),
            name: slf.name,
            bmp_disabled_ptr,
            bmp_focus_ptr,
            bmp_hover_ptr,
        };

        BitmapButton::new_impl(config)
    }
);

implement_widget_traits_with_target!(BitmapButton, window, Window);

// Add XRC Support - enables BitmapButton to be created from XRC-managed pointers
impl_xrc_support!(BitmapButton, { window });

// Widget casting support for BitmapButton
impl_widget_cast!(BitmapButton, "wxBitmapButton", { window });
