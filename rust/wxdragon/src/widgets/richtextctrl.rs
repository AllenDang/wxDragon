//!
//! Safe wrapper for wxRichTextCtrl.

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
#[derive(Clone)]
pub struct RichTextCtrl {
    window: Window,
}

impl RichTextCtrl {
    /// Creates a new RichTextCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> RichTextCtrlBuilder<'_> {
        RichTextCtrlBuilder::new(parent)
    }

    /// Creates a new RichTextCtrl wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_RichTextCtrl_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_RichTextCtrl_t) -> Self {
        RichTextCtrl {
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

        unsafe { RichTextCtrl::from_ptr(ptr) }
    }

    // --- Text Content Operations ---

    /// Sets the text value of the control.
    pub fn set_value(&self, value: &str) {
        let c_value = CString::new(value).unwrap_or_default();
        unsafe {
            ffi::wxd_RichTextCtrl_SetValue(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                c_value.as_ptr(),
            );
        }
    }

    /// Gets the current text value of the control.
    pub fn get_value(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_RichTextCtrl_GetValue(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
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

    /// Writes text at the current insertion point.
    pub fn write_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_RichTextCtrl_WriteText(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                c_text.as_ptr(),
            );
        }
    }

    /// Appends text to the end of the control.
    pub fn append_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_RichTextCtrl_AppendText(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                c_text.as_ptr(),
            );
        }
    }

    /// Clears all text in the control.
    pub fn clear(&self) {
        unsafe {
            ffi::wxd_RichTextCtrl_Clear(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t);
        }
    }

    /// Returns the length of the text.
    pub fn get_length(&self) -> i32 {
        unsafe {
            ffi::wxd_RichTextCtrl_GetLength(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t)
        }
    }

    // --- Text Range Operations ---

    /// Gets text in the specified range.
    pub fn get_range(&self, from: i64, to: i64) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_RichTextCtrl_GetRange(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                from,
                to,
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

    /// Sets the selection range.
    pub fn set_selection(&self, from: i64, to: i64) {
        unsafe {
            ffi::wxd_RichTextCtrl_SetSelection(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                from,
                to,
            );
        }
    }

    /// Gets the current selection range.
    pub fn get_selection(&self) -> (i64, i64) {
        let mut from = 0i64;
        let mut to = 0i64;
        unsafe {
            ffi::wxd_RichTextCtrl_GetSelection(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                &mut from,
                &mut to,
            );
        }
        (from, to)
    }

    /// Gets the currently selected text.
    pub fn get_selected_text(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_RichTextCtrl_GetSelectedText(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
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

    // --- Editing Operations ---

    /// Cuts the selected text to the clipboard.
    pub fn cut(&self) {
        unsafe {
            ffi::wxd_RichTextCtrl_Cut(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t);
        }
    }

    /// Copies the selected text to the clipboard.
    pub fn copy(&self) {
        unsafe {
            ffi::wxd_RichTextCtrl_Copy(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t);
        }
    }

    /// Pastes text from the clipboard.
    pub fn paste(&self) {
        unsafe {
            ffi::wxd_RichTextCtrl_Paste(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t);
        }
    }

    /// Undoes the last operation.
    pub fn undo(&self) {
        unsafe {
            ffi::wxd_RichTextCtrl_Undo(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t);
        }
    }

    /// Redoes the last undone operation.
    pub fn redo(&self) {
        unsafe {
            ffi::wxd_RichTextCtrl_Redo(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t);
        }
    }

    /// Returns true if undo is available.
    pub fn can_undo(&self) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_CanUndo(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t)
        }
    }

    /// Returns true if redo is available.
    pub fn can_redo(&self) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_CanRedo(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t)
        }
    }

    // --- State Operations ---

    /// Makes the text control editable or read-only.
    pub fn set_editable(&self, editable: bool) {
        unsafe {
            ffi::wxd_RichTextCtrl_SetEditable(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                editable,
            );
        }
    }

    /// Returns true if the control is editable.
    pub fn is_editable(&self) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_IsEditable(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t)
        }
    }

    /// Returns true if the control has been modified.
    pub fn is_modified(&self) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_IsModified(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t)
        }
    }

    /// Marks the control as dirty (modified).
    pub fn mark_dirty(&self) {
        unsafe {
            ffi::wxd_RichTextCtrl_MarkDirty(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t);
        }
    }

    /// Discards any edits and marks the control as unmodified.
    pub fn discard_edits(&self) {
        unsafe {
            ffi::wxd_RichTextCtrl_DiscardEdits(self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t);
        }
    }

    // --- Position Operations ---

    /// Gets the insertion point of the control.
    pub fn get_insertion_point(&self) -> i64 {
        unsafe {
            ffi::wxd_RichTextCtrl_GetInsertionPoint(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t
            )
        }
    }

    /// Sets the insertion point of the control.
    pub fn set_insertion_point(&self, pos: i64) {
        unsafe {
            ffi::wxd_RichTextCtrl_SetInsertionPoint(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                pos,
            );
        }
    }

    /// Sets the insertion point to the end of the text.
    pub fn set_insertion_point_end(&self) {
        unsafe {
            ffi::wxd_RichTextCtrl_SetInsertionPointEnd(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t
            );
        }
    }

    /// Returns the last position in the control.
    pub fn get_last_position(&self) -> i64 {
        unsafe {
            ffi::wxd_RichTextCtrl_GetLastPosition(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t
            )
        }
    }

    // --- File Operations ---

    /// Loads a file into the control.
    pub fn load_file(&self, filename: &str, file_type: RichTextFileType) -> bool {
        let c_filename = CString::new(filename).unwrap_or_default();
        unsafe {
            ffi::wxd_RichTextCtrl_LoadFile(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                c_filename.as_ptr(),
                file_type.into(),
            )
        }
    }

    /// Saves the content to a file.
    pub fn save_file(&self, filename: &str, file_type: RichTextFileType) -> bool {
        let c_filename = CString::new(filename).unwrap_or_default();
        unsafe {
            ffi::wxd_RichTextCtrl_SaveFile(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                c_filename.as_ptr(),
                file_type.into(),
            )
        }
    }

    // --- Style Operations ---

    /// Sets style for a range of text.
    pub fn set_style_range(
        &self,
        start: i64,
        end: i64,
        bold: bool,
        italic: bool,
        underline: bool,
    ) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_SetStyleRange(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                start,
                end,
                bold,
                italic,
                underline,
            )
        }
    }

    /// Applies bold formatting to the selection.
    pub fn apply_bold_to_selection(&self) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_ApplyBoldToSelection(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t
            )
        }
    }

    /// Applies italic formatting to the selection.
    pub fn apply_italic_to_selection(&self) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_ApplyItalicToSelection(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t
            )
        }
    }

    /// Applies underline formatting to the selection.
    pub fn apply_underline_to_selection(&self) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_ApplyUnderlineToSelection(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t
            )
        }
    }

    /// Returns true if the selection is bold.
    pub fn is_selection_bold(&self) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_IsSelectionBold(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t
            )
        }
    }

    /// Returns true if the selection is italic.
    pub fn is_selection_italic(&self) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_IsSelectionItalics(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t
            )
        }
    }

    /// Returns true if the selection is underlined.
    pub fn is_selection_underlined(&self) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_IsSelectionUnderlined(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t
            )
        }
    }

    // --- Font Operations ---

    /// Sets the font size for a range of text.
    pub fn set_font_size(&self, start: i64, end: i64, size: i32) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_SetFontSize(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                start,
                end,
                size,
            )
        }
    }

    /// Sets the font size for the current selection.
    pub fn set_font_size_selection(&self, size: i32) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_SetFontSizeSelection(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                size,
            )
        }
    }

    // --- Color Operations ---

    /// Sets text color for a range of text.
    pub fn set_text_color(&self, start: i64, end: i64, color: crate::Colour) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_SetTextColor(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                start,
                end,
                color.into(),
            )
        }
    }

    /// Sets text color for the selection.
    pub fn set_text_color_selection(&self, color: crate::Colour) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_SetTextColorSelection(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                color.into(),
            )
        }
    }

    /// Sets background color for a range of text.
    pub fn set_background_color(&self, start: i64, end: i64, color: crate::Colour) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_SetBackgroundColor(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                start,
                end,
                color.into(),
            )
        }
    }

    /// Sets background color for the selection.
    pub fn set_background_color_selection(&self, color: crate::Colour) -> bool {
        unsafe {
            ffi::wxd_RichTextCtrl_SetBackgroundColorSelection(
                self.window.as_ptr() as *mut ffi::wxd_RichTextCtrl_t,
                color.into(),
            )
        }
    }
}

// Apply common trait implementations for this widget
implement_widget_traits_with_target!(RichTextCtrl, window, Window);

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

// Implement standard events traits
impl TextEvents for RichTextCtrl {}

// XRC Support - enables RichTextCtrl to be created from XRC-managed pointers
impl_xrc_support!(RichTextCtrl, { window });
