//!
//! Safe wrapper for wxToggleButton.

use crate::event::WxEvtHandler;
use crate::event::button_events::ButtonEvents;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by XRC support for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

// --- Toggle Button Styles ---
widget_style_enum!(
    name: ToggleButtonStyle,
    doc: "Style flags for ToggleButton widget.",
    variants: {
        Default: 0, "Default style (no specific alignment, standard border).",
        Left: ffi::WXD_BU_LEFT, "Align label to the left.",
        Top: ffi::WXD_BU_TOP, "Align label to the top.",
        Right: ffi::WXD_BU_RIGHT, "Align label to the right.",
        Bottom: ffi::WXD_BU_BOTTOM, "Align label to the bottom.",
        ExactFit: ffi::WXD_BU_EXACTFIT, "Button size will be adjusted to exactly fit the label.",
        NoText: ffi::WXD_BU_NOTEXT, "Do not display the label string (useful for buttons with only an image).",
        BorderNone: ffi::WXD_BORDER_NONE, "No border."
    },
    default_variant: Default
);

/// Represents a wxToggleButton control.
///
/// ToggleButton uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let toggle = ToggleButton::builder(&frame).label("Toggle me").build();
///
/// // ToggleButton is Copy - no clone needed for closures!
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
pub struct ToggleButton {
    /// Safe handle to the underlying wxToggleButton - automatically invalidated on destroy
    handle: WindowHandle,
}

impl ToggleButton {
    /// Creates a new ToggleButton builder.
    pub fn builder(parent: &dyn WxWidget) -> ToggleButtonBuilder<'_> {
        ToggleButtonBuilder::new(parent)
    }

    /// Creates a new ToggleButton from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Creates a new ToggleButton (low-level constructor used by the builder)
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, label: &str, pos: Point, size: Size, style: i64) -> Self {
        assert!(!parent_ptr.is_null(), "ToggleButton requires a parent");
        let c_label = CString::new(label).expect("CString::new failed");

        let ptr = unsafe {
            ffi::wxd_ToggleButton_Create(
                parent_ptr,
                id,
                c_label.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create ToggleButton widget");
        }

        // Create a WindowHandle which automatically registers for destroy events
        ToggleButton {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw toggle button pointer, returns null if widget has been destroyed
    #[inline]
    fn togglebutton_ptr(&self) -> *mut ffi::wxd_ToggleButton_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_ToggleButton_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Gets the current state of the toggle button (true if pressed/down, false if not).
    /// Returns false if the button has been destroyed.
    pub fn get_value(&self) -> bool {
        let ptr = self.togglebutton_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ToggleButton_GetValue(ptr) }
    }

    /// Sets the state of the toggle button.
    /// No-op if the button has been destroyed.
    pub fn set_value(&self, state: bool) {
        let ptr = self.togglebutton_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ToggleButton_SetValue(ptr, state) }
    }

    /// Sets the button label.
    /// No-op if the button has been destroyed.
    pub fn set_label(&self, label: &str) {
        let ptr = self.togglebutton_ptr();
        if ptr.is_null() {
            return;
        }
        let c_label = CString::new(label).expect("CString::new failed");
        unsafe {
            ffi::wxd_ToggleButton_SetLabel(ptr, c_label.as_ptr());
        }
    }

    /// Gets the button label.
    /// Returns empty string if the button has been destroyed.
    pub fn get_label(&self) -> String {
        let ptr = self.togglebutton_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_ToggleButton_GetLabel(ptr, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_ToggleButton_GetLabel(ptr, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() }
    }

    /// Returns the underlying WindowHandle for this toggle button.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Implement ButtonEvents trait for ToggleButton
impl ButtonEvents for ToggleButton {}

// Manual WxWidget implementation for ToggleButton (using WindowHandle)
impl WxWidget for ToggleButton {
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
impl WxEvtHandler for ToggleButton {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for ToggleButton {}

// Use the widget_builder macro for ToggleButton
widget_builder!(
    name: ToggleButton,
    parent_type: &'a dyn WxWidget,
    style_type: ToggleButtonStyle,
    fields: {
        label: String = String::new()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        ToggleButton::new_impl(
            parent_ptr,
            slf.id,
            &slf.label,
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
);

// XRC Support - enables ToggleButton to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for ToggleButton {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ToggleButton {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for ToggleButton
impl crate::window::FromWindowWithClassName for ToggleButton {
    fn class_name() -> &'static str {
        "wxToggleButton"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ToggleButton {
            handle: WindowHandle::new(ptr),
        }
    }
}
