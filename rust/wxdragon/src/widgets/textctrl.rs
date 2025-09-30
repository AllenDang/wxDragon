//!
//! Safe wrapper for wxTextCtrl.

use crate::event::TextEvents;
use crate::event::{Event, EventType};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// --- Text Control Styles ---
widget_style_enum!(
    name: TextCtrlStyle,
    doc: "Style flags for TextCtrl widget.",
    variants: {
        Default: 0, "Default style (single line, editable, left-aligned).",
        MultiLine: ffi::WXD_TE_MULTILINE, "Multi-line text control.",
        Password: ffi::WXD_TE_PASSWORD, "Password entry control (displays characters as asterisks).",
        ReadOnly: ffi::WXD_TE_READONLY, "Read-only text control.",
        Rich: ffi::WXD_TE_RICH, "For rich text content (implies multiline). Use with care, may require specific handling.",
        Rich2: ffi::WXD_TE_RICH2, "For more advanced rich text content (implies multiline). Use with care.",
        AutoUrl: ffi::WXD_TE_AUTO_URL, "Automatically detect and make URLs clickable.",
        ProcessEnter: ffi::WXD_TE_PROCESS_ENTER, "Generate an event when Enter key is pressed.",
        ProcessTab: ffi::WXD_TE_PROCESS_TAB, "Process TAB key in the control instead of using it for navigation.",
        NoHideSel: ffi::WXD_TE_NOHIDESEL, "Always show selection, even when control doesn't have focus (Windows only).",
        Centre: ffi::WXD_TE_CENTRE, "Center-align text.",
        Right: ffi::WXD_TE_RIGHT, "Right-align text.",
        CharWrap: ffi::WXD_TE_CHARWRAP, "Wrap at any position, splitting words if necessary.",
        WordWrap: ffi::WXD_TE_WORDWRAP, "Wrap at word boundaries.",
        NoVScroll: ffi::WXD_TE_NO_VSCROLL, "No vertical scrollbar (multiline only).",
        DontWrap: ffi::WXD_TE_DONTWRAP, "Don't wrap at all, show horizontal scrollbar instead."
    },
    default_variant: Default
);

/// Events emitted by TextCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextCtrlEvent {
    /// Emitted when the text in the control changes
    TextChanged,
    /// Emitted when the user presses Enter in the control
    TextEnter,
}

/// Event data for a TextCtrl event
#[derive(Debug)]
pub struct TextCtrlEventData {
    event: Event,
}

impl TextCtrlEventData {
    /// Create a new TextCtrlEventData from a generic Event
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

    /// Get the current text in the control
    pub fn get_string(&self) -> Option<String> {
        self.event.get_string()
    }
}

/// Represents a wxTextCtrl widget.
#[derive(Clone)]
pub struct TextCtrl {
    window: Window,
}

impl TextCtrl {
    /// Creates a new TextCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> TextCtrlBuilder<'_> {
        TextCtrlBuilder::new(parent)
    }

    /// Creates a new TextCtrl wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_TextCtrl_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_TextCtrl_t) -> Self {
        TextCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Internal implementation used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        value: &str,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        let c_value = CString::new(value).unwrap_or_default();

        let ptr = unsafe {
            ffi::wxd_TextCtrl_Create(
                parent_ptr,
                id,
                c_value.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create TextCtrl widget");
        }

        unsafe { TextCtrl::from_ptr(ptr) }
    }

    /// Sets the text value of the control.
    pub fn set_value(&self, value: &str) {
        let c_value = CString::new(value).unwrap_or_default();
        unsafe {
            ffi::wxd_TextCtrl_SetValue(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                c_value.as_ptr(),
            );
        }
    }

    /// Gets the current text value of the control.
    pub fn get_value(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_TextCtrl_GetValue(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );
            if len >= 0 {
                let byte_slice =
                    std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    /// Appends text to the end of the control.
    pub fn append_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_TextCtrl_AppendText(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                c_text.as_ptr(),
            );
        }
    }

    /// Clears the text in the control.
    pub fn clear(&self) {
        unsafe {
            ffi::wxd_TextCtrl_Clear(self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t);
        }
    }

    /// Returns whether the text control has been modified by the user since the last
    /// time MarkDirty() or DiscardEdits() was called.
    pub fn is_modified(&self) -> bool {
        unsafe { ffi::wxd_TextCtrl_IsModified(self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t) }
    }

    /// Marks the control as modified or unmodified.
    pub fn set_modified(&self, modified: bool) {
        unsafe {
            ffi::wxd_TextCtrl_SetModified(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                modified,
            );
        }
    }

    /// Makes the text control editable or read-only, overriding the style setting.
    pub fn set_editable(&self, editable: bool) {
        unsafe {
            ffi::wxd_TextCtrl_SetEditable(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                editable,
            );
        }
    }

    /// Returns true if the control is editable.
    pub fn is_editable(&self) -> bool {
        unsafe { ffi::wxd_TextCtrl_IsEditable(self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t) }
    }

    /// Gets the insertion point of the control.
    /// The insertion point is the position at which the caret is currently positioned.
    pub fn get_insertion_point(&self) -> i64 {
        unsafe {
            ffi::wxd_TextCtrl_GetInsertionPoint(self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t)
        }
    }

    /// Sets the insertion point of the control.
    pub fn set_insertion_point(&self, pos: i64) {
        unsafe {
            ffi::wxd_TextCtrl_SetInsertionPoint(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                pos,
            );
        }
    }

    /// Sets the maximum number of characters that may be entered in the control.
    ///
    /// If `len` is 0, the maximum length limit is removed.
    pub fn set_max_length(&self, len: usize) {
        unsafe {
            ffi::wxd_TextCtrl_SetMaxLength(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                len as i64,
            );
        }
    }

    /// Returns the last position in the control.
    pub fn get_last_position(&self) -> i64 {
        unsafe {
            ffi::wxd_TextCtrl_GetLastPosition(self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t)
        }
    }

    /// Returns true if this is a multi-line text control.
    pub fn is_multiline(&self) -> bool {
        unsafe { ffi::wxd_TextCtrl_IsMultiLine(self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t) }
    }

    /// Returns true if this is a single-line text control.
    pub fn is_single_line(&self) -> bool {
        unsafe { ffi::wxd_TextCtrl_IsSingleLine(self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t) }
    }

    // --- Selection Operations ---

    /// Sets the selection in the text control.
    ///
    /// # Arguments
    /// * `from` - The start position of the selection
    /// * `to` - The end position of the selection
    pub fn set_selection(&self, from: i64, to: i64) {
        unsafe {
            ffi::wxd_TextCtrl_SetSelection(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                from,
                to,
            );
        }
    }

    /// Gets the current selection range.
    ///
    /// Returns a tuple (from, to) representing the selection range.
    /// If there's no selection, both values will be equal to the insertion point.
    pub fn get_selection(&self) -> (i64, i64) {
        let mut from = 0i64;
        let mut to = 0i64;
        unsafe {
            ffi::wxd_TextCtrl_GetSelection(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                &mut from,
                &mut to,
            );
        }
        (from, to)
    }

    /// Selects all text in the control.
    pub fn select_all(&self) {
        unsafe {
            ffi::wxd_TextCtrl_SelectAll(self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t);
        }
    }

    /// Gets the currently selected text.
    ///
    /// Returns an empty string if no text is selected.
    pub fn get_string_selection(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_TextCtrl_GetStringSelection(
                self.window.as_ptr() as *mut ffi::wxd_TextCtrl_t,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );
            if len >= 0 {
                let byte_slice =
                    std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }
}

// Apply common trait implementations for this widget
implement_widget_traits_with_target!(TextCtrl, window, Window);

// Implement scrolling functionality for TextCtrl (useful for multiline text)
impl crate::scrollable::WxScrollable for TextCtrl {}

// Use the widget_builder macro for TextCtrl
widget_builder!(
    name: TextCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: TextCtrlStyle,
    fields: {
        value: String = String::new()
    },
    build_impl: |slf| {
        TextCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            &slf.value,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
);

// Implement TextCtrl-specific event handlers using the standard macro
crate::implement_widget_local_event_handlers!(
    TextCtrl,
    TextCtrlEvent,
    TextCtrlEventData,
    TextChanged => text_changed, EventType::TEXT,
    TextEnter => text_enter, EventType::TEXT_ENTER
);

// Implement standard events traits
impl TextEvents for TextCtrl {}

// XRC Support - enables TextCtrl to be created from XRC-managed pointers
impl_xrc_support!(TextCtrl, { window });

// Widget casting support for TextCtrl
impl_widget_cast!(TextCtrl, "wxTextCtrl", { window });
