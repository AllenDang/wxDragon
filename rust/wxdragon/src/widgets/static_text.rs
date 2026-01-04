//! Safe wrapper for wxStaticText.

use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::{CStr, CString};
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
///
/// StaticText uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct StaticText {
    /// Safe handle to the underlying wxStaticText - automatically invalidated on destroy
    handle: WindowHandle,
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
                    handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
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

    /// Helper to get raw static text pointer, returns null if widget has been destroyed
    #[inline]
    fn widget_ptr(&self) -> *mut ffi::wxd_StaticText_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_StaticText_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Sets the text control's label.
    /// No-op if the widget has been destroyed.
    pub fn set_label(&self, label: &str) {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return;
        }
        let c_label = CString::new(label).unwrap_or_default();
        unsafe { ffi::wxd_StaticText_SetLabel(ptr, c_label.as_ptr()) };
    }

    /// Gets the text control's label.
    /// Returns empty string if the widget has been destroyed.
    pub fn get_label(&self) -> String {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_StaticText_GetLabel(ptr, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_StaticText_GetLabel(ptr, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() }
    }

    /// Wraps the text to the specified width in pixels.
    /// This enables automatic word wrapping for multi-line text display.
    /// No-op if the widget has been destroyed.
    pub fn wrap(&self, width: i32) {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_StaticText_Wrap(ptr, width);
        }
    }

    /// Returns the underlying WindowHandle for this static text.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for StaticText (using WindowHandle)
impl WxWidget for StaticText {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for StaticText {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for StaticText {}

// XRC Support - enables StaticText to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for StaticText {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        StaticText {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for StaticText
impl crate::window::FromWindowWithClassName for StaticText {
    fn class_name() -> &'static str {
        "wxStaticText"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        StaticText {
            handle: WindowHandle::new(ptr),
        }
    }
}
