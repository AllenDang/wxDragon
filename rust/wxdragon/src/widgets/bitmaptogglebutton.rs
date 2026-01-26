//!
//! Safe wrapper for wxBitmapToggleButton.

use crate::bitmap::Bitmap;
use crate::event::WxEvtHandler;
use crate::event::button_events::ButtonEvents;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// --- BitmapToggleButton Styles ---
widget_style_enum!(
    name: BitmapToggleButtonStyle,
    doc: "Style flags for BitmapToggleButton widget.",
    variants: {
        Default: 0, "Default style (no specific alignment, standard border).",
        Left: ffi::WXD_BU_LEFT, "Align bitmap to the left.",
        Top: ffi::WXD_BU_TOP, "Align bitmap to the top.",
        Right: ffi::WXD_BU_RIGHT, "Align bitmap to the right.",
        Bottom: ffi::WXD_BU_BOTTOM, "Align bitmap to the bottom.",
        ExactFit: ffi::WXD_BU_EXACTFIT, "Button size will be adjusted to exactly fit the bitmap.",
        NoText: ffi::WXD_BU_NOTEXT, "Do not display the label string (useful for buttons with only an image).",
        BorderNone: ffi::WXD_BORDER_NONE, "No border."
    },
    default_variant: Default
);

/// Represents a wxBitmapToggleButton control.
///
/// A toggle button that displays a bitmap instead of a text label.
/// Combines the toggle functionality of `ToggleButton` with the bitmap display
/// of `BitmapButton`.
///
/// BitmapToggleButton uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let toggle = BitmapToggleButton::builder(&frame)
///     .bitmap(&my_bitmap)
///     .build();
///
/// // BitmapToggleButton is Copy - no clone needed for closures!
/// toggle.bind_click(move |_| {
///     // Safe: if toggle was destroyed, this is a no-op
///     toggle.set_value(!toggle.get_value());
/// });
///
/// // After parent destruction, toggle operations are safe no-ops
/// frame.destroy();
/// assert!(!toggle.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct BitmapToggleButton {
    /// Safe handle to the underlying wxBitmapToggleButton - automatically invalidated on destroy
    handle: WindowHandle,
}

/// Configuration for creating a BitmapToggleButton
#[derive(Debug)]
struct BitmapToggleButtonConfig {
    pub parent_ptr: *mut ffi::wxd_Window_t,
    pub id: Id,
    pub bitmap_ptr: *const ffi::wxd_Bitmap_t,
    pub pos: Point,
    pub size: Size,
    pub style: i64,
    pub name: String,
    pub bmp_disabled_ptr: *const ffi::wxd_Bitmap_t,
    pub bmp_focus_ptr: *const ffi::wxd_Bitmap_t,
    pub bmp_pressed_ptr: *const ffi::wxd_Bitmap_t,
}

impl BitmapToggleButton {
    /// Creates a new BitmapToggleButton builder.
    pub fn builder(parent: &dyn WxWidget) -> BitmapToggleButtonBuilder<'_> {
        BitmapToggleButtonBuilder::new(parent)
    }

    /// Creates a new BitmapToggleButton from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Low-level constructor used by the builder.
    fn new_impl(config: BitmapToggleButtonConfig) -> Self {
        let c_name = CString::new(config.name).unwrap_or_default();

        unsafe {
            let ptr = ffi::wxd_BitmapToggleButton_Create(
                config.parent_ptr,
                config.id as c_int,
                config.bitmap_ptr,
                config.pos.into(),
                config.size.into(),
                config.style as ffi::wxd_Style_t,
                c_name.as_ptr(),
                config.bmp_disabled_ptr,
                config.bmp_focus_ptr,
                config.bmp_pressed_ptr,
            );

            if ptr.is_null() {
                panic!("Failed to create BitmapToggleButton widget");
            } else {
                BitmapToggleButton {
                    handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
                }
            }
        }
    }

    /// Helper to get raw bitmap toggle button pointer, returns null if widget has been destroyed
    #[inline]
    fn bitmaptogglebutton_ptr(&self) -> *mut ffi::wxd_BitmapToggleButton_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_BitmapToggleButton_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Gets the current state of the toggle button (true if pressed/down, false if not).
    /// Returns false if the button has been destroyed.
    pub fn get_value(&self) -> bool {
        let ptr = self.bitmaptogglebutton_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_BitmapToggleButton_GetValue(ptr) }
    }

    /// Sets the state of the toggle button.
    /// No-op if the button has been destroyed.
    pub fn set_value(&self, state: bool) {
        let ptr = self.bitmaptogglebutton_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_BitmapToggleButton_SetValue(ptr, state) }
    }

    /// Sets the main bitmap (label bitmap) for the button.
    /// No-op if the button has been destroyed.
    pub fn set_bitmap_label(&self, bitmap: &Bitmap) {
        let ptr = self.bitmaptogglebutton_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_BitmapToggleButton_SetBitmapLabel(ptr, bitmap.as_const_ptr()) }
    }

    /// Sets the bitmap shown when the button is disabled.
    /// No-op if the button has been destroyed.
    pub fn set_bitmap_disabled(&self, bitmap: &Bitmap) {
        let ptr = self.bitmaptogglebutton_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_BitmapToggleButton_SetBitmapDisabled(ptr, bitmap.as_const_ptr()) }
    }

    /// Sets the bitmap shown when the button has focus.
    /// No-op if the button has been destroyed.
    pub fn set_bitmap_focus(&self, bitmap: &Bitmap) {
        let ptr = self.bitmaptogglebutton_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_BitmapToggleButton_SetBitmapFocus(ptr, bitmap.as_const_ptr()) }
    }

    /// Sets the bitmap shown when the button is pressed (toggled on).
    /// No-op if the button has been destroyed.
    pub fn set_bitmap_pressed(&self, bitmap: &Bitmap) {
        let ptr = self.bitmaptogglebutton_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_BitmapToggleButton_SetBitmapPressed(ptr, bitmap.as_const_ptr()) }
    }

    /// Gets the main bitmap (label bitmap) for the button.
    /// Returns None if the button has been destroyed or has no valid bitmap.
    pub fn get_bitmap_label(&self) -> Option<Bitmap> {
        let ptr = self.bitmaptogglebutton_ptr();
        if ptr.is_null() {
            return None;
        }
        let bmp_ptr = unsafe { ffi::wxd_BitmapToggleButton_GetBitmapLabel(ptr) };
        if bmp_ptr.is_null() {
            None
        } else {
            Some(Bitmap::from(bmp_ptr))
        }
    }

    /// Gets the bitmap shown when the button is disabled.
    /// Returns None if the button has been destroyed or has no valid disabled bitmap.
    pub fn get_bitmap_disabled(&self) -> Option<Bitmap> {
        let ptr = self.bitmaptogglebutton_ptr();
        if ptr.is_null() {
            return None;
        }
        let bmp_ptr = unsafe { ffi::wxd_BitmapToggleButton_GetBitmapDisabled(ptr) };
        if bmp_ptr.is_null() {
            None
        } else {
            Some(Bitmap::from(bmp_ptr))
        }
    }

    /// Gets the bitmap shown when the button has focus.
    /// Returns None if the button has been destroyed or has no valid focus bitmap.
    pub fn get_bitmap_focus(&self) -> Option<Bitmap> {
        let ptr = self.bitmaptogglebutton_ptr();
        if ptr.is_null() {
            return None;
        }
        let bmp_ptr = unsafe { ffi::wxd_BitmapToggleButton_GetBitmapFocus(ptr) };
        if bmp_ptr.is_null() {
            None
        } else {
            Some(Bitmap::from(bmp_ptr))
        }
    }

    /// Gets the bitmap shown when the button is pressed (toggled on).
    /// Returns None if the button has been destroyed or has no valid pressed bitmap.
    pub fn get_bitmap_pressed(&self) -> Option<Bitmap> {
        let ptr = self.bitmaptogglebutton_ptr();
        if ptr.is_null() {
            return None;
        }
        let bmp_ptr = unsafe { ffi::wxd_BitmapToggleButton_GetBitmapPressed(ptr) };
        if bmp_ptr.is_null() {
            None
        } else {
            Some(Bitmap::from(bmp_ptr))
        }
    }

    /// Returns the underlying WindowHandle for this bitmap toggle button.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Implement ButtonEvents trait for BitmapToggleButton
impl ButtonEvents for BitmapToggleButton {}

// Manual WxWidget implementation for BitmapToggleButton (using WindowHandle)
impl WxWidget for BitmapToggleButton {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for BitmapToggleButton {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for BitmapToggleButton {}

// Use the widget_builder macro for BitmapToggleButton
widget_builder!(
    name: BitmapToggleButton,
    parent_type: &'a dyn WxWidget,
    style_type: BitmapToggleButtonStyle,
    fields: {
        bitmap: Option<Bitmap> = None,
        bitmap_disabled: Option<Bitmap> = None,
        bitmap_focus: Option<Bitmap> = None,
        bitmap_pressed: Option<Bitmap> = None,
        name: String = "BitmapToggleButton".to_string()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let bitmap_ptr = match &slf.bitmap {
            Some(bitmap) => bitmap.as_const_ptr(),
            None => panic!("BitmapToggleButton requires a bitmap to be set"),
        };

        let bmp_disabled_ptr = slf.bitmap_disabled
            .as_ref()
            .map_or(std::ptr::null(), |b| b.as_const_ptr());
        let bmp_focus_ptr = slf.bitmap_focus
            .as_ref()
            .map_or(std::ptr::null(), |b| b.as_const_ptr());
        let bmp_pressed_ptr = slf.bitmap_pressed
            .as_ref()
            .map_or(std::ptr::null(), |b| b.as_const_ptr());

        // For BitmapToggleButton, size is often best derived from the bitmap if not explicitly set
        let final_size = if slf.size.width == -1 && slf.size.height == -1 {
            if let Some(bmp) = &slf.bitmap {
                Size::new(bmp.get_width(), bmp.get_height())
            } else {
                slf.size
            }
        } else {
            slf.size
        };

        let config = BitmapToggleButtonConfig {
            parent_ptr,
            id: slf.id,
            bitmap_ptr,
            pos: slf.pos,
            size: final_size,
            style: slf.style.bits(),
            name: slf.name,
            bmp_disabled_ptr,
            bmp_focus_ptr,
            bmp_pressed_ptr,
        };

        BitmapToggleButton::new_impl(config)
    }
);

// XRC Support - enables BitmapToggleButton to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for BitmapToggleButton {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        BitmapToggleButton {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for BitmapToggleButton
impl crate::window::FromWindowWithClassName for BitmapToggleButton {
    fn class_name() -> &'static str {
        "wxBitmapToggleButton"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        BitmapToggleButton {
            handle: WindowHandle::new(ptr),
        }
    }
}
