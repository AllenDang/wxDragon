//!
//! Safe wrapper for wxTextCtrl.

use crate::event::TextEvents;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by new_from_composition for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;
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
///
/// TextCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let textctrl = TextCtrl::builder(&frame).value("Enter text here").build();
///
/// // TextCtrl is Copy - no clone needed for closures!
/// textctrl.bind_text_changed(move |_| {
///     // Safe: if textctrl was destroyed, this is a no-op
///     let text = textctrl.get_value();
///     println!("Text changed: {}", text);
/// });
///
/// // After parent destruction, textctrl operations are safe no-ops
/// frame.destroy();
/// assert!(!textctrl.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct TextCtrl {
    /// Safe handle to the underlying wxTextCtrl - automatically invalidated on destroy
    handle: WindowHandle,
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
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Creates a new TextCtrl from a raw window pointer.
    /// This is for backwards compatibility with widgets that compose TextCtrl.
    /// The parent_ptr parameter is ignored (kept for API compatibility).
    #[allow(dead_code)]
    pub(crate) fn new_from_composition(_window: Window, _parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        // Use the window's pointer to create a new WindowHandle
        Self {
            handle: WindowHandle::new(_window.as_ptr()),
        }
    }

    /// Internal implementation used by the builder.
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, value: &str, pos: Point, size: Size, style: i64) -> Self {
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

    /// Helper to get raw textctrl pointer, returns null if widget has been destroyed
    #[inline]
    fn textctrl_ptr(&self) -> *mut ffi::wxd_TextCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_TextCtrl_t)
            .unwrap_or(null_mut())
    }

    /// Sets the text value of the control.
    /// No-op if the control has been destroyed.
    pub fn set_value(&self, value: &str) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        let c_value = CString::new(value).unwrap_or_default();
        unsafe { ffi::wxd_TextCtrl_SetValue(ptr, c_value.as_ptr()) };
    }

    /// Gets the current text value of the control.
    /// Returns empty string if the control has been destroyed.
    pub fn get_value(&self) -> String {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_TextCtrl_GetValue(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    /// Appends text to the end of the control.
    /// No-op if the control has been destroyed.
    pub fn append_text(&self, text: &str) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        let c_text = CString::new(text).unwrap_or_default();
        unsafe { ffi::wxd_TextCtrl_AppendText(ptr, c_text.as_ptr()) };
    }

    /// Clears the text in the control.
    /// No-op if the control has been destroyed.
    pub fn clear(&self) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_TextCtrl_Clear(ptr);
        }
    }

    /// Returns whether the text control has been modified by the user since the last
    /// time MarkDirty() or DiscardEdits() was called.
    /// Returns false if the control has been destroyed.
    pub fn is_modified(&self) -> bool {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_TextCtrl_IsModified(ptr) }
    }

    /// Marks the control as modified or unmodified.
    /// No-op if the control has been destroyed.
    pub fn set_modified(&self, modified: bool) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TextCtrl_SetModified(ptr, modified) };
    }

    /// Makes the text control editable or read-only, overriding the style setting.
    /// No-op if the control has been destroyed.
    pub fn set_editable(&self, editable: bool) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TextCtrl_SetEditable(ptr, editable) };
    }

    /// Returns true if the control is editable.
    /// Returns false if the control has been destroyed.
    pub fn is_editable(&self) -> bool {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_TextCtrl_IsEditable(ptr) }
    }

    /// Gets the insertion point of the control.
    /// The insertion point is the position at which the caret is currently positioned.
    /// Returns 0 if the control has been destroyed.
    pub fn get_insertion_point(&self) -> i64 {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_TextCtrl_GetInsertionPoint(ptr) }
    }

    /// Sets the insertion point of the control.
    /// No-op if the control has been destroyed.
    pub fn set_insertion_point(&self, pos: i64) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TextCtrl_SetInsertionPoint(ptr, pos) };
    }

    /// Sets the maximum number of characters that may be entered in the control.
    ///
    /// If `len` is 0, the maximum length limit is removed.
    /// No-op if the control has been destroyed.
    pub fn set_max_length(&self, len: usize) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TextCtrl_SetMaxLength(ptr, len as i64) };
    }

    /// Returns the last position in the control.
    /// Returns 0 if the control has been destroyed.
    pub fn get_last_position(&self) -> i64 {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_TextCtrl_GetLastPosition(ptr) }
    }

    /// Returns true if this is a multi-line text control.
    /// Returns false if the control has been destroyed.
    pub fn is_multiline(&self) -> bool {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_TextCtrl_IsMultiLine(ptr) }
    }

    /// Returns true if this is a single-line text control.
    /// Returns false if the control has been destroyed.
    pub fn is_single_line(&self) -> bool {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_TextCtrl_IsSingleLine(ptr) }
    }

    // --- Selection Operations ---

    /// Sets the selection in the text control.
    ///
    /// # Arguments
    /// * `from` - The start position of the selection
    /// * `to` - The end position of the selection
    ///
    /// No-op if the control has been destroyed.
    pub fn set_selection(&self, from: i64, to: i64) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TextCtrl_SetSelection(ptr, from, to) };
    }

    /// Gets the current selection range.
    ///
    /// Returns a tuple (from, to) representing the selection range.
    /// If there's no selection, both values will be equal to the insertion point.
    /// Returns (0, 0) if the control has been destroyed.
    pub fn get_selection(&self) -> (i64, i64) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return (0, 0);
        }
        let mut from = 0i64;
        let mut to = 0i64;
        unsafe { ffi::wxd_TextCtrl_GetSelection(ptr, &mut from, &mut to) };
        (from, to)
    }

    /// Selects all text in the control.
    /// No-op if the control has been destroyed.
    pub fn select_all(&self) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TextCtrl_SelectAll(ptr) };
    }

    /// Gets the currently selected text.
    ///
    /// Returns an empty string if no text is selected or if the control has been destroyed.
    pub fn get_string_selection(&self) -> String {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_TextCtrl_GetStringSelection(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    /// Sets the insertion point to the end of the text control.
    /// No-op if the control has been destroyed.
    pub fn set_insertion_point_end(&self) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TextCtrl_SetInsertionPointEnd(ptr) };
    }

    /// Returns the underlying WindowHandle for this textctrl.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Implement TextEvents trait for TextCtrl
impl TextEvents for TextCtrl {}

// Manual WxWidget implementation for TextCtrl (using WindowHandle)
impl WxWidget for TextCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for TextCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for TextCtrl {}

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

// XRC Support - enables TextCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for TextCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        TextCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Widget casting support for TextCtrl
impl crate::window::FromWindowWithClassName for TextCtrl {
    fn class_name() -> &'static str {
        "wxTextCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        TextCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
