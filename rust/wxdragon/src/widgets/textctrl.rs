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

    fn read_string_with_retry(mut getter: impl FnMut(*mut c_char, i32) -> i32) -> String {
        let mut buffer: Vec<c_char> = vec![0; 1024];
        let mut len = getter(buffer.as_mut_ptr(), buffer.len() as i32);
        if len < 0 {
            return String::new();
        }
        if len as usize >= buffer.len() {
            buffer = vec![0; len as usize + 1];
            len = getter(buffer.as_mut_ptr(), buffer.len() as i32);
            if len < 0 {
                return String::new();
            }
        }
        let byte_slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize) };
        String::from_utf8_lossy(byte_slice).to_string()
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
        unsafe { Self::read_string_with_retry(|buf, len| ffi::wxd_TextCtrl_GetValue(ptr, buf, len)) }
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

    /// Writes text into the control at the current insertion point.
    /// No-op if the control has been destroyed.
    pub fn write_text(&self, text: &str) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        let c_text = CString::new(text).unwrap_or_default();
        unsafe { ffi::wxd_TextCtrl_WriteText(ptr, c_text.as_ptr()) };
    }

    /// Sets the value of the control without generating a TextChanged event.
    /// No-op if the control has been destroyed.
    pub fn change_value(&self, value: &str) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        let c_value = CString::new(value).unwrap_or_default();
        unsafe { ffi::wxd_TextCtrl_ChangeValue(ptr, c_value.as_ptr()) };
    }

    /// Removes the text in the given range.
    /// No-op if the control has been destroyed.
    pub fn remove(&self, from: i64, to: i64) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TextCtrl_Remove(ptr, from, to) };
    }

    /// Replaces the text in the given range with the new value.
    /// No-op if the control has been destroyed.
    pub fn replace(&self, from: i64, to: i64, value: &str) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        let c_value = CString::new(value).unwrap_or_default();
        unsafe { ffi::wxd_TextCtrl_Replace(ptr, from, to, c_value.as_ptr()) };
    }

    /// Converts given text position to client coordinates in pixels.
    /// Returns (x, y) if successful, or None if the position is invalid or the control is destroyed.
    pub fn position_to_xy(&self, pos: i64) -> Option<(i64, i64)> {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return None;
        }
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let result = unsafe { ffi::wxd_TextCtrl_PositionToXY(ptr, pos, &mut x, &mut y) };
        if result { Some((x, y)) } else { None }
    }

    /// Converts given client coordinates in pixels to text position.
    /// Returns 0 if the coordinates are invalid or the control is destroyed.
    pub fn xy_to_position(&self, x: i64, y: i64) -> i64 {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_TextCtrl_XYToPosition(ptr, x, y) }
    }

    /// Returns the number of lines in the text control.
    /// Returns 0 if the control has been destroyed.
    pub fn get_number_of_lines(&self) -> i32 {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_TextCtrl_GetNumberOfLines(ptr) }
    }

    /// Returns the length of the specified line (not including the trailing newline character).
    /// Returns 0 if the line number is invalid or the control is destroyed.
    pub fn get_line_length(&self, line_no: i64) -> i32 {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_TextCtrl_GetLineLength(ptr, line_no) }
    }

    /// Returns the contents of the given line.
    /// Returns an empty string if the control has been destroyed.
    pub fn get_line_text(&self, line_no: i64) -> String {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe { Self::read_string_with_retry(|buf, len| ffi::wxd_TextCtrl_GetLineText(ptr, line_no, buf, len)) }
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
        unsafe { Self::read_string_with_retry(|buf, len| ffi::wxd_TextCtrl_GetStringSelection(ptr, buf, len)) }
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

    /// Sets the style for the given text range.
    /// No-op if the control has been destroyed.
    pub fn set_style(&self, start: i64, end: i64, style: &TextAttr) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TextCtrl_SetStyle(ptr, start, end, style.as_ptr()) };
    }

    /// Returns the default style currently used for new text.
    /// Returns None if the control has been destroyed.
    pub fn get_default_style(&self) -> Option<TextAttr> {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return None;
        }
        let attr_ptr = unsafe { ffi::wxd_TextCtrl_GetDefaultStyle(ptr) };
        if attr_ptr.is_null() {
            return None;
        }
        Some(TextAttr { ptr: attr_ptr })
    }

    /// Sets the default style used for new text.
    /// No-op if the control has been destroyed.
    pub fn set_default_style(&self, style: &TextAttr) {
        let ptr = self.textctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TextCtrl_SetDefaultStyle(ptr, style.as_ptr()) };
    }

    /// Returns the underlying WindowHandle for this textctrl.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

/// Represents text attributes such as colors and fonts.
pub struct TextAttr {
    ptr: *mut ffi::wxd_TextAttr_t,
}

impl Default for TextAttr {
    fn default() -> Self {
        Self::new()
    }
}

impl TextAttr {
    /// Creates a new default TextAttr.
    pub fn new() -> Self {
        Self {
            ptr: unsafe { ffi::wxd_TextAttr_Create() },
        }
    }

    /// Sets the text color.
    pub fn set_text_colour(&mut self, color: crate::color::Colour) {
        unsafe { ffi::wxd_TextAttr_SetTextColour(self.ptr, color.to_raw()) };
    }

    /// Sets the background color.
    pub fn set_background_colour(&mut self, color: crate::color::Colour) {
        unsafe { ffi::wxd_TextAttr_SetBackgroundColour(self.ptr, color.to_raw()) };
    }

    /// Sets the font.
    pub fn set_font(&mut self, font: &crate::font::Font) {
        unsafe { ffi::wxd_TextAttr_SetFont(self.ptr, font.as_ptr()) };
    }

    /// Sets the text alignment.
    pub fn set_alignment(&mut self, alignment: i32) {
        unsafe { ffi::wxd_TextAttr_SetAlignment(self.ptr, alignment) };
    }

    /// Sets the left indent and sub-indent in tenths of a millimetre.
    pub fn set_left_indent(&mut self, indent: i32, sub_indent: i32) {
        unsafe { ffi::wxd_TextAttr_SetLeftIndent(self.ptr, indent, sub_indent) };
    }

    /// Sets the right indent in tenths of a millimetre.
    pub fn set_right_indent(&mut self, indent: i32) {
        unsafe { ffi::wxd_TextAttr_SetRightIndent(self.ptr, indent) };
    }

    /// Sets the line spacing. 10 is normal, 15 is 1.5, 20 is double.
    pub fn set_line_spacing(&mut self, spacing: i32) {
        unsafe { ffi::wxd_TextAttr_SetLineSpacing(self.ptr, spacing) };
    }

    /// Sets the paragraph spacing after in tenths of a millimetre.
    pub fn set_paragraph_spacing_after(&mut self, spacing: i32) {
        unsafe { ffi::wxd_TextAttr_SetParagraphSpacingAfter(self.ptr, spacing) };
    }

    /// Sets the paragraph spacing before in tenths of a millimetre.
    pub fn set_paragraph_spacing_before(&mut self, spacing: i32) {
        unsafe { ffi::wxd_TextAttr_SetParagraphSpacingBefore(self.ptr, spacing) };
    }

    /// Sets the bullet style.
    pub fn set_bullet_style(&mut self, style: i32) {
        unsafe { ffi::wxd_TextAttr_SetBulletStyle(self.ptr, style) };
    }

    pub fn set_flags(&mut self, flags: i64) {
        unsafe { ffi::wxd_TextAttr_SetFlags(self.ptr, flags) };
    }
    pub fn get_flags(&self) -> i64 {
        unsafe { ffi::wxd_TextAttr_GetFlags(self.ptr) }
    }
    pub fn has_flag(&self, flag: i64) -> bool {
        unsafe { ffi::wxd_TextAttr_HasFlag(self.ptr, flag) }
    }

    pub fn set_font_size(&mut self, point_size: i32) {
        unsafe { ffi::wxd_TextAttr_SetFontSize(self.ptr, point_size) };
    }
    pub fn get_font_size(&self) -> i32 {
        unsafe { ffi::wxd_TextAttr_GetFontSize(self.ptr) }
    }

    pub fn set_font_style(&mut self, font_style: crate::font::FontStyle) {
        unsafe { ffi::wxd_TextAttr_SetFontStyle(self.ptr, font_style as i32) };
    }
    pub fn get_font_style(&self) -> crate::font::FontStyle {
        unsafe { std::mem::transmute(ffi::wxd_TextAttr_GetFontStyle(self.ptr)) }
    }

    pub fn set_font_weight(&mut self, font_weight: crate::font::FontWeight) {
        unsafe { ffi::wxd_TextAttr_SetFontWeight(self.ptr, font_weight as i32) };
    }
    pub fn get_font_weight(&self) -> crate::font::FontWeight {
        unsafe { std::mem::transmute(ffi::wxd_TextAttr_GetFontWeight(self.ptr)) }
    }

    pub fn set_font_face_name(&mut self, face_name: &str) {
        let c_str = std::ffi::CString::new(face_name).unwrap();
        unsafe { ffi::wxd_TextAttr_SetFontFaceName(self.ptr, c_str.as_ptr()) };
    }
    pub fn set_font_underlined(&mut self, underlined: bool) {
        unsafe { ffi::wxd_TextAttr_SetFontUnderlined(self.ptr, underlined) };
    }
    pub fn set_font_strikethrough(&mut self, strikethrough: bool) {
        unsafe { ffi::wxd_TextAttr_SetFontStrikethrough(self.ptr, strikethrough) };
    }
    pub fn set_font_encoding(&mut self, encoding: i32) {
        unsafe { ffi::wxd_TextAttr_SetFontEncoding(self.ptr, encoding) };
    }
    pub fn set_font_family(&mut self, family: crate::font::FontFamily) {
        unsafe { ffi::wxd_TextAttr_SetFontFamily(self.ptr, family as i32) };
    }

    pub fn set_character_style_name(&mut self, name: &str) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe { ffi::wxd_TextAttr_SetCharacterStyleName(self.ptr, c_str.as_ptr()) };
    }
    pub fn set_paragraph_style_name(&mut self, name: &str) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe { ffi::wxd_TextAttr_SetParagraphStyleName(self.ptr, c_str.as_ptr()) };
    }
    pub fn set_list_style_name(&mut self, name: &str) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe { ffi::wxd_TextAttr_SetListStyleName(self.ptr, c_str.as_ptr()) };
    }

    pub fn set_bullet_number(&mut self, n: i32) {
        unsafe { ffi::wxd_TextAttr_SetBulletNumber(self.ptr, n) };
    }
    pub fn set_bullet_text(&mut self, text: &str) {
        let c_str = std::ffi::CString::new(text).unwrap();
        unsafe { ffi::wxd_TextAttr_SetBulletText(self.ptr, c_str.as_ptr()) };
    }
    pub fn set_bullet_font(&mut self, bullet_font: &str) {
        let c_str = std::ffi::CString::new(bullet_font).unwrap();
        unsafe { ffi::wxd_TextAttr_SetBulletFont(self.ptr, c_str.as_ptr()) };
    }
    pub fn set_bullet_name(&mut self, name: &str) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe { ffi::wxd_TextAttr_SetBulletName(self.ptr, c_str.as_ptr()) };
    }
    pub fn set_url(&mut self, url: &str) {
        let c_str = std::ffi::CString::new(url).unwrap();
        unsafe { ffi::wxd_TextAttr_SetURL(self.ptr, c_str.as_ptr()) };
    }
    pub fn set_page_break(&mut self, page_break: bool) {
        unsafe { ffi::wxd_TextAttr_SetPageBreak(self.ptr, page_break) };
    }
    pub fn set_text_effects(&mut self, effects: i32) {
        unsafe { ffi::wxd_TextAttr_SetTextEffects(self.ptr, effects) };
    }
    pub fn set_text_effect_flags(&mut self, effects: i32) {
        unsafe { ffi::wxd_TextAttr_SetTextEffectFlags(self.ptr, effects) };
    }
    pub fn set_outline_level(&mut self, level: i32) {
        unsafe { ffi::wxd_TextAttr_SetOutlineLevel(self.ptr, level) };
    }

    pub fn get_text_colour(&self) -> crate::color::Colour {
        crate::color::Colour::from(unsafe { ffi::wxd_TextAttr_GetTextColour(self.ptr) })
    }
    pub fn get_background_colour(&self) -> crate::color::Colour {
        crate::color::Colour::from(unsafe { ffi::wxd_TextAttr_GetBackgroundColour(self.ptr) })
    }
    pub fn get_alignment(&self) -> i32 {
        unsafe { ffi::wxd_TextAttr_GetAlignment(self.ptr) }
    }
    pub fn get_left_indent(&self) -> i32 {
        unsafe { ffi::wxd_TextAttr_GetLeftIndent(self.ptr) }
    }
    pub fn get_left_sub_indent(&self) -> i32 {
        unsafe { ffi::wxd_TextAttr_GetLeftSubIndent(self.ptr) }
    }
    pub fn get_right_indent(&self) -> i32 {
        unsafe { ffi::wxd_TextAttr_GetRightIndent(self.ptr) }
    }
    pub fn get_line_spacing(&self) -> i32 {
        unsafe { ffi::wxd_TextAttr_GetLineSpacing(self.ptr) }
    }
    pub fn get_paragraph_spacing_after(&self) -> i32 {
        unsafe { ffi::wxd_TextAttr_GetParagraphSpacingAfter(self.ptr) }
    }
    pub fn get_paragraph_spacing_before(&self) -> i32 {
        unsafe { ffi::wxd_TextAttr_GetParagraphSpacingBefore(self.ptr) }
    }
    pub fn get_bullet_style(&self) -> i32 {
        unsafe { ffi::wxd_TextAttr_GetBulletStyle(self.ptr) }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::wxd_TextAttr_t {
        self.ptr
    }
}

impl Drop for TextAttr {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_TextAttr_Delete(self.ptr) };
            self.ptr = null_mut();
        }
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
