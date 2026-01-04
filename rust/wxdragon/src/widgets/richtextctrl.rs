//!
//! Safe wrapper for wxRichTextCtrl.

use crate::event::TextEvents;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by impl_xrc_support for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::CString;
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// --- Rich Text Control Styles ---
widget_style_enum!(
    name: RichTextCtrlStyle,
    doc: "Style flags for RichTextCtrl widget.",
    variants: {
        Default: 0, "Default style.",
        ReadOnly: ffi::WXD_TE_READONLY, "Read-only rich text control.",
        MultiLine: ffi::WXD_TE_MULTILINE, "Multi-line rich text control.",
        NoVScroll: ffi::WXD_TE_NO_VSCROLL, "No vertical scrollbar.",
        AutoUrl: ffi::WXD_TE_AUTO_URL, "Automatically detect and make URLs clickable.",
        ProcessEnter: ffi::WXD_TE_PROCESS_ENTER, "Generate an event when Enter key is pressed.",
        ProcessTab: ffi::WXD_TE_PROCESS_TAB, "Process TAB key in the control instead of using it for navigation.",
        WordWrap: ffi::WXD_TE_WORDWRAP, "Wrap at word boundaries.",
        CharWrap: ffi::WXD_TE_CHARWRAP, "Wrap at any position, splitting words if necessary.",
        DontWrap: ffi::WXD_TE_DONTWRAP, "Don't wrap at all, show horizontal scrollbar instead."
    },
    default_variant: Default
);

/// File types for loading and saving rich text documents
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichTextFileType {
    /// Any file type (let wxWidgets determine)
    Any = 0,
    /// Plain text file
    Text = 1,
    /// XML format
    Xml = 2,
    /// HTML format
    Html = 3,
    /// RTF format
    Rtf = 4,
    /// PDF format (if supported)
    Pdf = 5,
}

impl From<RichTextFileType> for i32 {
    fn from(val: RichTextFileType) -> Self {
        val as i32
    }
}

/// Events emitted by RichTextCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichTextCtrlEvent {
    /// Text content has changed
    TextChanged,
    /// Enter key was pressed
    TextEnter,
    /// Left mouse click
    LeftClick,
    /// Right mouse click
    RightClick,
    /// Middle mouse click
    MiddleClick,
    /// Left mouse double-click
    LeftDoubleClick,
    /// Return key pressed
    Return,
    /// Character input
    Character,
    /// Delete operation
    Delete,
    /// Content was inserted
    ContentInserted,
    /// Content was deleted
    ContentDeleted,
    /// Style changed
    StyleChanged,
    /// Selection changed
    SelectionChanged,
}

/// Event data for a RichTextCtrl event
#[derive(Debug)]
pub struct RichTextCtrlEventData {
    event: Event,
}

impl RichTextCtrlEventData {
    /// Create a new RichTextCtrlEventData from a generic Event
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

    /// Get the position for position-related events
    pub fn get_position(&self) -> Option<i32> {
        self.event.get_int()
    }
}

/// Represents a wxRichTextCtrl widget.
///
/// RichTextCtrl is a rich text editor that supports formatted text with different fonts,
/// colors, styles, and other formatting options. It provides a comprehensive set of
/// editing and formatting capabilities.
///
/// RichTextCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let rtc = RichTextCtrl::builder(&frame).build();
///
/// // RichTextCtrl is Copy - no clone needed for closures!
/// rtc.bind_text_changed(move |_| {
///     // Safe: if rtc was destroyed, this is a no-op
///     rtc.append_text("Changed!");
/// });
///
/// // After parent destruction, rtc operations are safe no-ops
/// frame.destroy();
/// assert!(!rtc.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct RichTextCtrl {
    /// Safe handle to the underlying wxRichTextCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl RichTextCtrl {
    /// Creates a new RichTextCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> RichTextCtrlBuilder<'_> {
        RichTextCtrlBuilder::new(parent)
    }

    /// Creates a new RichTextCtrl from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Internal implementation used by the builder.
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, value: &str, pos: Point, size: Size, style: i64) -> Self {
        assert!(!parent_ptr.is_null(), "RichTextCtrl requires a parent");
        let c_value = CString::new(value).unwrap_or_default();

        let ptr = unsafe {
            ffi::wxd_RichTextCtrl_Create(
                parent_ptr,
                id,
                c_value.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create RichTextCtrl widget");
        }

        // Create a WindowHandle which automatically registers for destroy events
        RichTextCtrl {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw richtextctrl pointer, returns null if widget has been destroyed
    #[inline]
    fn richtextctrl_ptr(&self) -> *mut ffi::wxd_RichTextCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_RichTextCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    // --- Text Content Operations ---

    /// Sets the text value of the control.
    /// No-op if the control has been destroyed.
    pub fn set_value(&self, value: &str) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        let c_value = CString::new(value).unwrap_or_default();
        unsafe { ffi::wxd_RichTextCtrl_SetValue(ptr, c_value.as_ptr()) };
    }

    /// Gets the current text value of the control.
    /// Returns empty string if the control has been destroyed.
    pub fn get_value(&self) -> String {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_RichTextCtrl_GetValue(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    /// Writes text at the current insertion point.
    /// No-op if the control has been destroyed.
    pub fn write_text(&self, text: &str) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        let c_text = CString::new(text).unwrap_or_default();
        unsafe { ffi::wxd_RichTextCtrl_WriteText(ptr, c_text.as_ptr()) };
    }

    /// Appends text to the end of the control.
    /// No-op if the control has been destroyed.
    pub fn append_text(&self, text: &str) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        let c_text = CString::new(text).unwrap_or_default();
        unsafe { ffi::wxd_RichTextCtrl_AppendText(ptr, c_text.as_ptr()) };
    }

    /// Clears all text in the control.
    /// No-op if the control has been destroyed.
    pub fn clear(&self) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RichTextCtrl_Clear(ptr) };
    }

    /// Returns the length of the text.
    /// Returns 0 if the control has been destroyed.
    pub fn get_length(&self) -> i32 {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_RichTextCtrl_GetLength(ptr) }
    }

    // --- Text Range Operations ---

    /// Gets text in the specified range.
    /// Returns empty string if the control has been destroyed.
    pub fn get_range(&self, from: i64, to: i64) -> String {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_RichTextCtrl_GetRange(ptr, from, to, buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    /// Sets the selection range.
    /// No-op if the control has been destroyed.
    pub fn set_selection(&self, from: i64, to: i64) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RichTextCtrl_SetSelection(ptr, from, to) };
    }

    /// Gets the current selection range.
    /// Returns (0, 0) if the control has been destroyed.
    pub fn get_selection(&self) -> (i64, i64) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return (0, 0);
        }
        let mut from = 0i64;
        let mut to = 0i64;
        unsafe { ffi::wxd_RichTextCtrl_GetSelection(ptr, &mut from, &mut to) };
        (from, to)
    }

    /// Gets the currently selected text.
    /// Returns empty string if the control has been destroyed.
    pub fn get_selected_text(&self) -> String {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_RichTextCtrl_GetSelectedText(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    // --- Editing Operations ---

    /// Cuts the selected text to the clipboard.
    /// No-op if the control has been destroyed.
    pub fn cut(&self) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RichTextCtrl_Cut(ptr) };
    }

    /// Copies the selected text to the clipboard.
    /// No-op if the control has been destroyed.
    pub fn copy(&self) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RichTextCtrl_Copy(ptr) };
    }

    /// Pastes text from the clipboard.
    /// No-op if the control has been destroyed.
    pub fn paste(&self) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RichTextCtrl_Paste(ptr) };
    }

    /// Undoes the last operation.
    /// No-op if the control has been destroyed.
    pub fn undo(&self) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RichTextCtrl_Undo(ptr) };
    }

    /// Redoes the last undone operation.
    /// No-op if the control has been destroyed.
    pub fn redo(&self) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RichTextCtrl_Redo(ptr) };
    }

    /// Returns true if undo is available.
    /// Returns false if the control has been destroyed.
    pub fn can_undo(&self) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_CanUndo(ptr) }
    }

    /// Returns true if redo is available.
    /// Returns false if the control has been destroyed.
    pub fn can_redo(&self) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_CanRedo(ptr) }
    }

    // --- State Operations ---

    /// Makes the text control editable or read-only.
    /// No-op if the control has been destroyed.
    pub fn set_editable(&self, editable: bool) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RichTextCtrl_SetEditable(ptr, editable) };
    }

    /// Returns true if the control is editable.
    /// Returns false if the control has been destroyed.
    pub fn is_editable(&self) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_IsEditable(ptr) }
    }

    /// Returns true if the control has been modified.
    /// Returns false if the control has been destroyed.
    pub fn is_modified(&self) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_IsModified(ptr) }
    }

    /// Marks the control as dirty (modified).
    /// No-op if the control has been destroyed.
    pub fn mark_dirty(&self) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RichTextCtrl_MarkDirty(ptr) };
    }

    /// Discards any edits and marks the control as unmodified.
    /// No-op if the control has been destroyed.
    pub fn discard_edits(&self) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RichTextCtrl_DiscardEdits(ptr) };
    }

    // --- Position Operations ---

    /// Gets the insertion point of the control.
    /// Returns 0 if the control has been destroyed.
    pub fn get_insertion_point(&self) -> i64 {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_RichTextCtrl_GetInsertionPoint(ptr) }
    }

    /// Sets the insertion point of the control.
    /// No-op if the control has been destroyed.
    pub fn set_insertion_point(&self, pos: i64) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RichTextCtrl_SetInsertionPoint(ptr, pos) };
    }

    /// Sets the insertion point to the end of the text.
    /// No-op if the control has been destroyed.
    pub fn set_insertion_point_end(&self) {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RichTextCtrl_SetInsertionPointEnd(ptr) };
    }

    /// Returns the last position in the control.
    /// Returns 0 if the control has been destroyed.
    pub fn get_last_position(&self) -> i64 {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_RichTextCtrl_GetLastPosition(ptr) }
    }

    // --- File Operations ---

    /// Loads a file into the control.
    /// Returns false if the control has been destroyed.
    pub fn load_file(&self, filename: &str, file_type: RichTextFileType) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_filename = CString::new(filename).unwrap_or_default();
        unsafe { ffi::wxd_RichTextCtrl_LoadFile(ptr, c_filename.as_ptr(), file_type.into()) }
    }

    /// Saves the content to a file.
    /// Returns false if the control has been destroyed.
    pub fn save_file(&self, filename: &str, file_type: RichTextFileType) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_filename = CString::new(filename).unwrap_or_default();
        unsafe { ffi::wxd_RichTextCtrl_SaveFile(ptr, c_filename.as_ptr(), file_type.into()) }
    }

    // --- Style Operations ---

    /// Sets style for a range of text.
    /// Returns false if the control has been destroyed.
    pub fn set_style_range(&self, start: i64, end: i64, bold: bool, italic: bool, underline: bool) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_SetStyleRange(ptr, start, end, bold, italic, underline) }
    }

    /// Applies bold formatting to the selection.
    /// Returns false if the control has been destroyed.
    pub fn apply_bold_to_selection(&self) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_ApplyBoldToSelection(ptr) }
    }

    /// Applies italic formatting to the selection.
    /// Returns false if the control has been destroyed.
    pub fn apply_italic_to_selection(&self) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_ApplyItalicToSelection(ptr) }
    }

    /// Applies underline formatting to the selection.
    /// Returns false if the control has been destroyed.
    pub fn apply_underline_to_selection(&self) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_ApplyUnderlineToSelection(ptr) }
    }

    /// Returns true if the selection is bold.
    /// Returns false if the control has been destroyed.
    pub fn is_selection_bold(&self) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_IsSelectionBold(ptr) }
    }

    /// Returns true if the selection is italic.
    /// Returns false if the control has been destroyed.
    pub fn is_selection_italic(&self) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_IsSelectionItalics(ptr) }
    }

    /// Returns true if the selection is underlined.
    /// Returns false if the control has been destroyed.
    pub fn is_selection_underlined(&self) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_IsSelectionUnderlined(ptr) }
    }

    // --- Font Operations ---

    /// Sets the font size for a range of text.
    /// Returns false if the control has been destroyed.
    pub fn set_font_size(&self, start: i64, end: i64, size: i32) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_SetFontSize(ptr, start, end, size) }
    }

    /// Sets the font size for the current selection.
    /// Returns false if the control has been destroyed.
    pub fn set_font_size_selection(&self, size: i32) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_SetFontSizeSelection(ptr, size) }
    }

    // --- Color Operations ---

    /// Sets text color for a range of text.
    /// Returns false if the control has been destroyed.
    pub fn set_text_color(&self, start: i64, end: i64, color: crate::Colour) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_SetTextColor(ptr, start, end, color.into()) }
    }

    /// Sets text color for the selection.
    /// Returns false if the control has been destroyed.
    pub fn set_text_color_selection(&self, color: crate::Colour) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_SetTextColorSelection(ptr, color.into()) }
    }

    /// Sets background color for a range of text.
    /// Returns false if the control has been destroyed.
    pub fn set_background_color(&self, start: i64, end: i64, color: crate::Colour) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_SetBackgroundColor(ptr, start, end, color.into()) }
    }

    /// Sets background color for the selection.
    /// Returns false if the control has been destroyed.
    pub fn set_background_color_selection(&self, color: crate::Colour) -> bool {
        let ptr = self.richtextctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RichTextCtrl_SetBackgroundColorSelection(ptr, color.into()) }
    }

    /// Returns the underlying WindowHandle for this control.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Implement TextEvents trait for RichTextCtrl
impl TextEvents for RichTextCtrl {}

// Manual WxWidget implementation for RichTextCtrl (using WindowHandle)
impl WxWidget for RichTextCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for RichTextCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for RichTextCtrl {}

// Implement scrolling functionality for RichTextCtrl
impl crate::scrollable::WxScrollable for RichTextCtrl {}

// Use the widget_builder macro for RichTextCtrl
widget_builder!(
    name: RichTextCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: RichTextCtrlStyle,
    fields: {
        value: String = String::new()
    },
    build_impl: |slf| {
        RichTextCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            &slf.value,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
);

// Implement RichTextCtrl-specific event handlers using the standard macro
crate::implement_widget_local_event_handlers!(
    RichTextCtrl,
    RichTextCtrlEvent,
    RichTextCtrlEventData,
    TextChanged => text_changed, EventType::TEXT,
    TextEnter => text_enter, EventType::TEXT_ENTER,
    LeftClick => left_click, EventType::RICHTEXT_LEFT_CLICK,
    RightClick => right_click, EventType::RICHTEXT_RIGHT_CLICK,
    MiddleClick => middle_click, EventType::RICHTEXT_MIDDLE_CLICK,
    LeftDoubleClick => left_double_click, EventType::RICHTEXT_LEFT_DCLICK,
    Return => return_key, EventType::RICHTEXT_RETURN,
    Character => character, EventType::RICHTEXT_CHARACTER,
    Delete => delete, EventType::RICHTEXT_DELETE,
    ContentInserted => content_inserted, EventType::RICHTEXT_CONTENT_INSERTED,
    ContentDeleted => content_deleted, EventType::RICHTEXT_CONTENT_DELETED,
    StyleChanged => style_changed, EventType::RICHTEXT_STYLE_CHANGED,
    SelectionChanged => selection_changed, EventType::RICHTEXT_SELECTION_CHANGED
);

// XRC Support - enables RichTextCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for RichTextCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        RichTextCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for RichTextCtrl
impl crate::window::FromWindowWithClassName for RichTextCtrl {
    fn class_name() -> &'static str {
        "wxRichTextCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        RichTextCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
