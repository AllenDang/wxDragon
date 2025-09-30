//! Safe wrapper for wxStaticText.

use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

widget_style_enum!(
    name: StaticTextStyle,
    doc: "Style flags for StaticText.",
    variants: {
        Default: ffi::WXD_ALIGN_LEFT, "Default style (left-aligned, auto-resizing).",
        AlignRight: ffi::WXD_ALIGN_RIGHT, "Align text to the right.",
        AlignCenterHorizontal: ffi::WXD_ALIGN_CENTRE_HORIZONTAL, "Align text to the center horizontally."
    },
    default_variant: Default
);

/// Represents a wxStaticText control.
#[derive(Clone)] // Allow cloning the wrapper
pub struct StaticText {
    window: Window, // Composition: StaticText IS a Window
}

widget_builder!(
    name: StaticText,
    parent_type: &'a dyn WxWidget,
    style_type: StaticTextStyle,
    fields: {
        label: String = String::new()
    },
    build_impl: |slf| {
        let c_label = CString::new(&slf.label[..]).unwrap_or_default();
        unsafe {
            let parent_ptr = slf.parent.handle_ptr();
            if parent_ptr.is_null() {
                panic!("Parent widget must not be null");
            }
            let ptr = ffi::wxd_StaticText_Create(
                parent_ptr as *mut _,
                slf.id,
                c_label.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits(),
            );
            if ptr.is_null() {
                panic!("Failed to create StaticText widget");
            } else {
                StaticText {
                    window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
                }
            }
        }
    }
);

impl StaticText {
    /// Creates a new StaticText builder.
    pub fn builder<W: WxWidget>(parent: &W) -> StaticTextBuilder<'_> {
        StaticTextBuilder::new(parent)
    }

    /// Sets the text control's label.
    pub fn set_label(&self, label: &str) {
        let c_label = CString::new(label).unwrap_or_default();
        unsafe {
            ffi::wxd_StaticText_SetLabel(
                self.window.as_ptr() as *mut ffi::wxd_StaticText_t,
                c_label.as_ptr(),
            );
        }
    }

    /// Gets the text control's label.
    pub fn get_label(&self) -> String {
        let mut buffer: [c_char; 256] = [0; 256];
        let len_needed = unsafe {
            ffi::wxd_StaticText_GetLabel(
                self.window.as_ptr() as *mut ffi::wxd_StaticText_t,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            )
        };

        if len_needed > 0 && (len_needed as usize) <= buffer.len() {
            unsafe {
                CStr::from_ptr(buffer.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            }
        } else if len_needed > (buffer.len() as i32) {
            let mut vec_buffer: Vec<c_char> = vec![0; len_needed as usize];
            let len_needed_2 = unsafe {
                ffi::wxd_StaticText_GetLabel(
                    self.window.as_ptr() as *mut ffi::wxd_StaticText_t,
                    vec_buffer.as_mut_ptr(),
                    vec_buffer.len() as i32,
                )
            };
            if len_needed_2 == len_needed {
                unsafe {
                    CStr::from_ptr(vec_buffer.as_ptr())
                        .to_string_lossy()
                        .into_owned()
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }

    /// Wraps the text to the specified width in pixels.
    /// This enables automatic word wrapping for multi-line text display.
    pub fn wrap(&self, width: i32) {
        unsafe {
            ffi::wxd_StaticText_Wrap(self.window.as_ptr() as *mut ffi::wxd_StaticText_t, width);
        }
    }
}

// Use the macro to implement all the standard traits
implement_widget_traits_with_target!(StaticText, window, Window);

// XRC Support - enables StaticText to be created from XRC-managed pointers
impl_xrc_support!(StaticText, { window });

// Enable widget casting for StaticText
impl_widget_cast!(StaticText, "wxStaticText", { window });
