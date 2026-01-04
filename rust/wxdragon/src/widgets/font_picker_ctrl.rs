/* This is a new file */
//! Safe wrapper for wxFontPickerCtrl.

use std::ffi::c_longlong;
use wxdragon_sys as ffi;

use crate::event::{Event, EventType, WxEvtHandler};
use crate::font::Font;
use crate::prelude::*;
use crate::window::{WindowHandle, WxWidget};
// Window is used by impl_xrc_support for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;

// --- Style enum using macro ---
widget_style_enum!(
    name: FontPickerCtrlStyle,
    doc: "Style flags for FontPickerCtrl widgets.",
    variants: {
        Default: ffi::WXD_FNTP_DEFAULT_STYLE, "Default style, includes `UseTextCtrl`.",
        UseTextCtrl: ffi::WXD_FNTP_USE_TEXTCTRL, "Use a text control to display the font description.",
        FontDescAsLabel: ffi::WXD_FNTP_FONTDESC_AS_LABEL, "Show the font description (e.g., \"Times New Roman Bold 10\") as the label.",
        UseFontForLabel: ffi::WXD_FNTP_USEFONT_FOR_LABEL, "Use the selected font itself to draw the label."
    },
    default_variant: Default
);

/// Events emitted by FontPickerCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontPickerCtrlEvent {
    /// Emitted when the font is changed
    FontChanged,
}

/// Event data for a FontChanged event
#[derive(Debug)]
pub struct FontChangedEventData {
    event: Event,
}

impl FontChangedEventData {
    /// Create a new FontChangedEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the ID of the control that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Skip this event (allow it to be processed by the parent window)
    pub fn skip(&self, skip: bool) {
        self.event.skip(skip);
    }
}

// --- FontPickerCtrl ---

/// Represents a wxFontPickerCtrl, which allows the user to select a font.
///
/// FontPickerCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let picker = FontPickerCtrl::builder(&frame).build();
///
/// // FontPickerCtrl is Copy - no clone needed for closures!
/// picker.bind_font_changed(move |_| {
///     // Safe: if picker was destroyed, this is a no-op
///     let font = picker.get_selected_font();
///     println!("Selected font: {:?}", font);
/// });
///
/// // After parent destruction, picker operations are safe no-ops
/// frame.destroy();
/// assert!(!picker.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct FontPickerCtrl {
    /// Safe handle to the underlying wxFontPickerCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl FontPickerCtrl {
    /// Creates a new FontPickerCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> FontPickerCtrlBuilder<'_> {
        FontPickerCtrlBuilder::new(parent)
    }

    /// Helper to get raw font picker pointer, returns null if widget has been destroyed
    #[inline]
    fn font_picker_ptr(&self) -> *mut ffi::wxd_FontPickerCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_FontPickerCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Gets the currently selected font.
    /// Returns `None` if no font is selected, the font is invalid, or the widget has been destroyed.
    pub fn get_selected_font(&self) -> Option<Font> {
        let ptr = self.font_picker_ptr();
        if ptr.is_null() {
            return None;
        }
        let font_ptr = unsafe { ffi::wxd_FontPickerCtrl_GetSelectedFont(ptr) };
        if font_ptr.is_null() {
            None
        } else {
            // The C++ code creates a new wxFont that we take ownership of
            Some(unsafe { Font::from_ptr(font_ptr, true) })
        }
    }

    /// Sets the currently selected font.
    /// No-op if the widget has been destroyed.
    pub fn set_selected_font(&self, font: &Font) {
        let ptr = self.font_picker_ptr();
        if ptr.is_null() {
            return;
        }
        // Create a new font to ensure proper ownership
        let font_copy = font.to_owned();
        // The C++ code makes a copy of the font, so we can just pass the pointer
        unsafe { ffi::wxd_FontPickerCtrl_SetSelectedFont(ptr, font_copy.as_ptr()) };
        // Intentionally leak the font as the C++ side now owns it
        std::mem::forget(font_copy);
    }

    /// Creates a FontPickerCtrl from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Use the widget_builder macro to generate the FontPickerCtrlBuilder implementation
widget_builder!(
    name: FontPickerCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: FontPickerCtrlStyle,
    fields: {
        initial_font: Option<Font> = None
    },
    build_impl: |slf| {
        assert!(!slf.parent.handle_ptr().is_null(), "FontPickerCtrl requires a parent");

        let initial_font_ptr = slf
            .initial_font
            .as_ref()
            .map_or(std::ptr::null(), |f| f.as_ptr());

        let ptr = unsafe {
            ffi::wxd_FontPickerCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                initial_font_ptr,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as c_longlong,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create FontPickerCtrl: FFI returned null pointer.");
        }

        // Create a WindowHandle which automatically registers for destroy events
        FontPickerCtrl {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// Manual WxWidget implementation for FontPickerCtrl (using WindowHandle)
impl WxWidget for FontPickerCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for FontPickerCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for FontPickerCtrl {}

// Use the implement_widget_local_event_handlers macro to implement event handling
crate::implement_widget_local_event_handlers!(
    FontPickerCtrl,
    FontPickerCtrlEvent,
    FontChangedEventData,
    FontChanged => font_changed, EventType::FONT_PICKER_CHANGED
);

// XRC Support - enables FontPickerCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for FontPickerCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        FontPickerCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for FontPickerCtrl
impl crate::window::FromWindowWithClassName for FontPickerCtrl {
    fn class_name() -> &'static str {
        "wxFontPickerCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        FontPickerCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
