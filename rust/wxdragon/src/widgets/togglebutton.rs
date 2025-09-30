//!
//! Safe wrapper for wxToggleButton.

use crate::event::button_events::ButtonEvents;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
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
#[derive(Clone)]
pub struct ToggleButton {
    window: Window,
}

impl ToggleButton {
    /// Creates a new ToggleButton builder.
    pub fn builder(parent: &dyn WxWidget) -> ToggleButtonBuilder<'_> {
        ToggleButtonBuilder::new(parent)
    }

    /// Creates a new ToggleButton wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_ToggleButton_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_ToggleButton_t) -> Self {
        ToggleButton {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Internal implementation used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        label: &str,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        let c_label = CString::new(label).unwrap_or_default();

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

        unsafe { ToggleButton::from_ptr(ptr) }
    }

    /// Gets the current state of the toggle button (true if pressed/down, false if not).
    pub fn get_value(&self) -> bool {
        unsafe { ffi::wxd_ToggleButton_GetValue(self.window.as_ptr() as *mut _) }
    }

    /// Sets the state of the toggle button.
    pub fn set_value(&self, state: bool) {
        unsafe { ffi::wxd_ToggleButton_SetValue(self.window.as_ptr() as *mut _, state) }
    }

    /// Sets the button label.
    pub fn set_label(&self, label: &str) {
        let c_label = CString::new(label).expect("Invalid CString for ToggleButton label");
        unsafe {
            ffi::wxd_ToggleButton_SetLabel(self.window.as_ptr() as *mut _, c_label.as_ptr());
        }
    }

    /// Gets the button label.
    pub fn get_label(&self) -> String {
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed = ffi::wxd_ToggleButton_GetLabel(
                self.window.as_ptr() as *mut _,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return String::new(); // Error
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                CStr::from_ptr(buffer.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_ToggleButton_GetLabel(
                    self.window.as_ptr() as *mut _,
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop(); // Remove null terminator
                    String::from_utf8(vec_buffer).unwrap_or_default()
                } else {
                    String::new() // Error on second call
                }
            }
        }
    }
}

// Implement ButtonEvents trait for ToggleButton
impl ButtonEvents for ToggleButton {}

// Apply common trait implementations for this widget
implement_widget_traits_with_target!(ToggleButton, window, Window);

// Use the widget_builder macro for ToggleButton
widget_builder!(
    name: ToggleButton,
    parent_type: &'a dyn WxWidget,
    style_type: ToggleButtonStyle,
    fields: {
        label: String = String::new()
    },
    build_impl: |slf| {
        ToggleButton::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            &slf.label,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
);

// Add XRC Support - enables ToggleButton to be created from XRC-managed pointers
impl_xrc_support!(ToggleButton, { window });

// Widget casting support for ToggleButton
impl_widget_cast!(ToggleButton, "wxToggleButton", { window });
