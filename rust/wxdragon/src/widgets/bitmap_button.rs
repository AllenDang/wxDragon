//!
//! Safe wrapper for wxBitmapButton.

use crate::bitmap::Bitmap;
use crate::event::WxEvtHandler;
use crate::event::button_events::ButtonEvents;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
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
///
/// BitmapButton uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let button = BitmapButton::builder(&frame).bitmap(&my_bitmap).build();
///
/// // BitmapButton is Copy - no clone needed for closures!
/// button.bind_click(move |_| {
///     // Safe: if button was destroyed, this is a no-op
///     println!("Clicked!");
/// });
///
/// // After parent destruction, button operations are safe no-ops
/// frame.destroy();
/// assert!(!button.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct BitmapButton {
    /// Safe handle to the underlying wxBitmapButton - automatically invalidated on destroy
    handle: WindowHandle,
}

// Implement ButtonEvents trait for BitmapButton
impl ButtonEvents for BitmapButton {}

/// Configuration for creating a BitmapButton
#[derive(Debug)]
struct BitmapButtonConfig {
    pub parent_ptr: *mut ffi::wxd_Window_t,
    pub id: Id,
    pub bitmap_ptr: *const ffi::wxd_Bitmap_t,
    pub pos: Point,
    pub size: Size,
    pub style: i64,
    pub name: String,
    pub bmp_disabled_ptr: *const ffi::wxd_Bitmap_t,
    pub bmp_focus_ptr: *const ffi::wxd_Bitmap_t,
    pub bmp_hover_ptr: *const ffi::wxd_Bitmap_t,
}

impl BitmapButton {
    /// Creates a new BitmapButton builder.
    pub fn builder(parent: &dyn WxWidget) -> BitmapButtonBuilder<'_> {
        BitmapButtonBuilder::new(parent)
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
                BitmapButton {
                    handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
                }
            }
        }
    }

    /// Returns the underlying WindowHandle for this bitmap button.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
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
            Some(bitmap) => bitmap.as_const_ptr(),
            None => panic!("BitmapButton requires a bitmap to be set"),
        };

        let bmp_disabled_ptr = slf.bitmap_disabled
            .as_ref()
            .map_or(std::ptr::null(), |b| b.as_const_ptr());
        let bmp_focus_ptr = slf.bitmap_focus
            .as_ref()
            .map_or(std::ptr::null(), |b| b.as_const_ptr());
        let bmp_hover_ptr = slf.bitmap_hover
            .as_ref()
            .map_or(std::ptr::null(), |b| b.as_const_ptr());

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

// Manual WxWidget implementation for BitmapButton (using WindowHandle)
impl WxWidget for BitmapButton {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for BitmapButton {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for BitmapButton {}

// XRC Support - enables BitmapButton to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for BitmapButton {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        BitmapButton {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for BitmapButton
impl crate::window::FromWindowWithClassName for BitmapButton {
    fn class_name() -> &'static str {
        "wxBitmapButton"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        BitmapButton {
            handle: WindowHandle::new(ptr),
        }
    }
}
