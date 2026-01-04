use crate::bitmap::Bitmap;
use crate::bitmap_bundle::BitmapBundle;
use crate::event::WxEvtHandler;
use crate::event::button_events::ButtonEvents;
use crate::prelude::*;
use crate::window::{WindowHandle, WxWidget};
// Window is used by new_from_composition for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

/// Enum for specifying bitmap position on a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
#[derive(Default)]
pub enum ButtonBitmapPosition {
    #[default]
    Left = ffi::wxd_ButtonBitmapPosition_t_WXD_BUTTON_BITMAP_LEFT,
    Right = ffi::wxd_ButtonBitmapPosition_t_WXD_BUTTON_BITMAP_RIGHT,
    Top = ffi::wxd_ButtonBitmapPosition_t_WXD_BUTTON_BITMAP_TOP,
    Bottom = ffi::wxd_ButtonBitmapPosition_t_WXD_BUTTON_BITMAP_BOTTOM,
}

/// Represents a wxButton.
///
/// Button uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let button = Button::builder(&frame).label("Click me").build();
///
/// // Button is Copy - no clone needed for closures!
/// button.bind_click(move |_| {
///     // Safe: if button was destroyed, this is a no-op
///     button.set_label("Clicked!");
/// });
///
/// // After parent destruction, button operations are safe no-ops
/// frame.destroy();
/// assert!(!button.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct Button {
    /// Safe handle to the underlying wxButton - automatically invalidated on destroy
    handle: WindowHandle,
}

impl Button {
    /// Creates a new `ButtonBuilder` for constructing a button.
    pub fn builder(parent: &dyn WxWidget) -> ButtonBuilder<'_> {
        ButtonBuilder::new(parent)
    }

    /// Creates a new Button from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Creates a new Button from a raw window pointer.
    /// This is for backwards compatibility with widgets that compose Button.
    /// The parent_ptr parameter is ignored (kept for API compatibility).
    #[allow(dead_code)]
    pub(crate) fn new_from_composition(_window: Window, _parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        // Use the window's pointer to create a new WindowHandle
        Self {
            handle: WindowHandle::new(_window.as_ptr()),
        }
    }

    /// Creates a new Button (low-level constructor used by the builder)
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, label: &str, pos: Point, size: Size, style: i64) -> Self {
        assert!(!parent_ptr.is_null(), "Button requires a parent");
        let c_label = CString::new(label).expect("CString::new failed");

        let ptr = unsafe {
            ffi::wxd_Button_Create(
                parent_ptr,
                id,
                c_label.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create Button widget");
        }

        // Create a WindowHandle which automatically registers for destroy events
        Button {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw button pointer, returns null if widget has been destroyed
    #[inline]
    fn button_ptr(&self) -> *mut ffi::wxd_Button_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_Button_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Sets the button's label.
    /// No-op if the button has been destroyed.
    pub fn set_label(&self, label: &str) {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return;
        }
        let c_label = CString::new(label).expect("CString::new failed");
        unsafe {
            ffi::wxd_Button_SetLabel(ptr, c_label.as_ptr());
        }
    }

    /// Gets the button's label.
    /// Returns empty string if the button has been destroyed.
    pub fn get_label(&self) -> String {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_Button_GetLabel(ptr, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_Button_GetLabel(ptr, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() }
    }

    // --- Bitmap Methods ---

    /// Sets the bitmap to be displayed by the button.
    /// No-op if the button has been destroyed.
    pub fn set_bitmap(&self, bitmap: &Bitmap, dir: ButtonBitmapPosition) {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Button_SetBitmap(ptr, bitmap.as_const_ptr(), dir as ffi::wxd_ButtonBitmapPosition_t) };
    }

    /// Sets the bitmap for the label (main bitmap, default position Left).
    pub fn set_bitmap_label(&self, bitmap: &Bitmap) {
        self.set_bitmap(bitmap, ButtonBitmapPosition::Left);
    }

    /// Sets the bitmap for the disabled state.
    pub fn set_bitmap_disabled(&self, bitmap: &Bitmap) {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Button_SetBitmapDisabled(ptr, bitmap.as_const_ptr()) };
    }

    /// Sets the bitmap for the focused state.
    pub fn set_bitmap_focus(&self, bitmap: &Bitmap) {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Button_SetBitmapFocus(ptr, bitmap.as_const_ptr()) };
    }

    /// Sets the bitmap for the current (hover) state.
    pub fn set_bitmap_current(&self, bitmap: &Bitmap) {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Button_SetBitmapCurrent(ptr, bitmap.as_const_ptr()) };
    }

    /// Sets the bitmap for the pressed state.
    pub fn set_bitmap_pressed(&self, bitmap: &Bitmap) {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Button_SetBitmapPressed(ptr, bitmap.as_const_ptr()) };
    }

    // --- BitmapBundle Methods ---

    /// Sets the bitmap bundle to be displayed by the button.
    pub fn set_bitmap_bundle(&self, bundle: &BitmapBundle, dir: ButtonBitmapPosition) {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Button_SetBitmapBundle(ptr, bundle.as_ptr(), dir as i32) };
    }

    /// Sets the bitmap bundle for the label (main bitmap, default position Left).
    pub fn set_bitmap_bundle_label(&self, bundle: &BitmapBundle) {
        self.set_bitmap_bundle(bundle, ButtonBitmapPosition::Left);
    }

    /// Sets the bitmap bundle for the disabled state.
    pub fn set_bitmap_bundle_disabled(&self, bundle: &BitmapBundle) {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_Button_SetBitmapBundleDisabled(ptr, bundle.as_ptr());
        }
    }

    /// Sets the bitmap bundle for the focused state.
    pub fn set_bitmap_bundle_focus(&self, bundle: &BitmapBundle) {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_Button_SetBitmapBundleFocus(ptr, bundle.as_ptr());
        }
    }

    /// Sets the bitmap bundle for the hover state.
    pub fn set_bitmap_bundle_hover(&self, bundle: &BitmapBundle) {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_Button_SetBitmapBundleHover(ptr, bundle.as_ptr());
        }
    }

    /// Sets the bitmap bundle for the pressed state.
    pub fn set_bitmap_bundle_pressed(&self, bundle: &BitmapBundle) {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_Button_SetBitmapBundlePressed(ptr, bundle.as_ptr());
        }
    }

    // Getters return Option<Bitmap>

    pub fn get_bitmap(&self) -> Option<Bitmap> {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return None;
        }
        let bmp_ptr = unsafe { ffi::wxd_Button_GetBitmap(ptr) };
        if bmp_ptr.is_null() {
            None
        } else {
            Some(Bitmap::from(bmp_ptr))
        }
    }

    pub fn get_bitmap_disabled(&self) -> Option<Bitmap> {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return None;
        }
        let bmp_ptr = unsafe { ffi::wxd_Button_GetBitmapDisabled(ptr) };
        if bmp_ptr.is_null() {
            None
        } else {
            Some(Bitmap::from(bmp_ptr))
        }
    }

    pub fn get_bitmap_focus(&self) -> Option<Bitmap> {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return None;
        }
        let bmp_ptr = unsafe { ffi::wxd_Button_GetBitmapFocus(ptr) };
        if bmp_ptr.is_null() {
            None
        } else {
            Some(Bitmap::from(bmp_ptr))
        }
    }

    pub fn get_bitmap_current(&self) -> Option<Bitmap> {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return None;
        }
        let bmp_ptr = unsafe { ffi::wxd_Button_GetBitmapCurrent(ptr) };
        if bmp_ptr.is_null() {
            None
        } else {
            Some(Bitmap::from(bmp_ptr))
        }
    }

    pub fn get_bitmap_pressed(&self) -> Option<Bitmap> {
        let ptr = self.button_ptr();
        if ptr.is_null() {
            return None;
        }
        let bmp_ptr = unsafe { ffi::wxd_Button_GetBitmapPressed(ptr) };
        if bmp_ptr.is_null() {
            None
        } else {
            Some(Bitmap::from(bmp_ptr))
        }
    }

    /// Returns the underlying WindowHandle for this button.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Implement ButtonEvents trait for Button
impl ButtonEvents for Button {}

// Manual WxWidget implementation for Button (using WindowHandle)
impl WxWidget for Button {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Note: We don't implement Deref to Window because returning a reference
// to a temporary Window is unsound. Users can access window methods through
// the WxWidget trait methods directly.

// Implement WxEvtHandler for event binding
impl WxEvtHandler for Button {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for Button {}

// Use the widget_builder macro to generate the ButtonBuilder implementation
widget_builder!(
    name: Button,
    parent_type: &'a dyn WxWidget,
    style_type: ButtonStyle,
    fields: {
        label: String = String::new(),
        bitmap_label: Option<Bitmap> = None,
        bitmap_position: Option<ButtonBitmapPosition> = None,
        bitmap_disabled: Option<Bitmap> = None,
        bitmap_focus: Option<Bitmap> = None,
        bitmap_current: Option<Bitmap> = None,
        bitmap_pressed: Option<Bitmap> = None,
        bitmap_bundle_label: Option<BitmapBundle> = None,
        bitmap_bundle_disabled: Option<BitmapBundle> = None,
        bitmap_bundle_focus: Option<BitmapBundle> = None,
        bitmap_bundle_hover: Option<BitmapBundle> = None,
        bitmap_bundle_pressed: Option<BitmapBundle> = None
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let button = Button::new_impl(
            parent_ptr,
            slf.id,
            &slf.label,
            slf.pos,
            slf.size,
            slf.style.bits(),
        );

        // Prioritize BitmapBundle over Bitmap if both are set
        if let Some(ref bundle) = slf.bitmap_bundle_label {
            button.set_bitmap_bundle(bundle, slf.bitmap_position.unwrap_or_default());
        } else if let Some(ref bmp) = slf.bitmap_label {
            button.set_bitmap(bmp, slf.bitmap_position.unwrap_or_default());
        }

        if let Some(ref bundle) = slf.bitmap_bundle_disabled {
            button.set_bitmap_bundle_disabled(bundle);
        } else if let Some(ref bmp) = slf.bitmap_disabled {
            button.set_bitmap_disabled(bmp);
        }

        if let Some(ref bundle) = slf.bitmap_bundle_focus {
            button.set_bitmap_bundle_focus(bundle);
        } else if let Some(ref bmp) = slf.bitmap_focus {
            button.set_bitmap_focus(bmp);
        }

        if let Some(ref bundle) = slf.bitmap_bundle_hover {
            button.set_bitmap_bundle_hover(bundle);
        } else if let Some(ref bmp) = slf.bitmap_current {
            button.set_bitmap_current(bmp);
        }

        if let Some(ref bundle) = slf.bitmap_bundle_pressed {
            button.set_bitmap_bundle_pressed(bundle);
        } else if let Some(ref bmp) = slf.bitmap_pressed {
            button.set_bitmap_pressed(bmp);
        }

        button
    }
);

// XRC Support - enables Button to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for Button {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Button {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Define the ButtonStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: ButtonStyle,
    doc: "Style flags for `Button`.",
    variants: {
        Default: 0, "Default style (no specific alignment, standard border).",
        Left: ffi::WXD_BU_LEFT, "Align label to the left.",
        Top: ffi::WXD_BU_TOP, "Align label to the top.",
        Right: ffi::WXD_BU_RIGHT, "Align label to the right.",
        Bottom: ffi::WXD_BU_BOTTOM, "Align label to the bottom.",
        ExactFit: ffi::WXD_BU_EXACTFIT, "Button size will be adjusted to exactly fit the label.",
        NoText: ffi::WXD_BU_NOTEXT, "Do not display the label string (useful for buttons with only an image).",
        BorderNone: ffi::WXD_BORDER_NONE, "No border.",
        BorderSimple: ffi::WXD_BORDER_SIMPLE, "A simple border (rarely used for buttons, which have a default look)."
    },
    default_variant: Default
);

// Enable widget casting for Button
impl crate::window::FromWindowWithClassName for Button {
    fn class_name() -> &'static str {
        "wxButton"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Button {
            handle: WindowHandle::new(ptr),
        }
    }
}
