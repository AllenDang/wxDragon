/* This is a new file */
//! Safe wrapper for wxFontPickerCtrl.

use std::ffi::c_longlong;
use wxdragon_sys as ffi;

use crate::event::{Event, EventType};
use crate::font::Font;
use crate::implement_widget_traits_with_target;
use crate::prelude::*;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};

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
#[derive(Clone)]
pub struct FontPickerCtrl {
    window: Window, // Embed Window
}

impl FontPickerCtrl {
    /// Creates a new FontPickerCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> FontPickerCtrlBuilder<'_> {
        FontPickerCtrlBuilder::new(parent)
    }

    /// Gets the currently selected font.
    /// Returns `None` if no font is selected or the font is invalid.
    pub fn get_selected_font(&self) -> Option<Font> {
        unsafe {
            let font_ptr = ffi::wxd_FontPickerCtrl_GetSelectedFont(
                self.window.as_ptr() as *mut ffi::wxd_FontPickerCtrl_t
            );
            if font_ptr.is_null() {
                None
            } else {
                // The C++ code creates a new wxFont that we take ownership of
                Some(Font::from_ptr(font_ptr, true))
            }
        }
    }

    /// Sets the currently selected font.
    pub fn set_selected_font(&self, font: &Font) {
        // Create a new font to ensure proper ownership
        let font_copy = font.to_owned();
        unsafe {
            // The C++ code makes a copy of the font, so we can just pass the pointer
            ffi::wxd_FontPickerCtrl_SetSelectedFont(
                self.window.as_ptr() as *mut ffi::wxd_FontPickerCtrl_t,
                font_copy.as_ptr(),
            );
        }
        // Intentionally leak the font as the C++ side now owns it
        std::mem::forget(font_copy);
    }

    /// Creates a FontPickerCtrl from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_FontPickerCtrl_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_FontPickerCtrl_t) -> Self {
        FontPickerCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
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
        } else {
            unsafe { FontPickerCtrl::from_ptr(ptr) }
        }
    }
);

// Use the implement_widget_traits_with_target macro to implement traits
implement_widget_traits_with_target!(FontPickerCtrl, window, Window);

// Use the implement_widget_local_event_handlers macro to implement event handling
crate::implement_widget_local_event_handlers!(
    FontPickerCtrl,
    FontPickerCtrlEvent,
    FontChangedEventData,
    FontChanged => font_changed, EventType::FONT_PICKER_CHANGED
);

// Add XRC Support - enables FontPickerCtrl to be created from XRC-managed pointers
impl_xrc_support!(FontPickerCtrl, { window });
