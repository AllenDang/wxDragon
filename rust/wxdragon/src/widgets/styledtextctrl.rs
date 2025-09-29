//! Safe wrapper for wxStyledTextCtrl (STC).

use crate::color::Colour;
use crate::event::WindowEvents;
use crate::event::{Event, EventType};
use crate::font::Font;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// STC Enums for type-safe parameter handling

/// Marker symbol types for StyledTextCtrl markers
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerSymbol {
    Circle = 0,
    RoundRect = 1,
    Arrow = 2,
    SmallRect = 3,
    ShortArrow = 4,
    Empty = 5,
    ArrowDown = 6,
    Minus = 7,
    Plus = 8,
    VLine = 9,
    LCorner = 10,
    TCorner = 11,
    BoxPlus = 12,
    BoxPlusConnected = 13,
    BoxMinus = 14,
    BoxMinusConnected = 15,
    LCornerCurve = 16,
    TCornerCurve = 17,
    CirclePlus = 18,
    CirclePlusConnected = 19,
    CircleMinus = 20,
    CircleMinusConnected = 21,
    Background = 22,
    DotDotDot = 23,
    Arrows = 24,
    Pixmap = 25,
    FullRect = 26,
    LeftRect = 27,
    Available = 28,
    Underline = 29,
    RgbaImage = 30,
    Bookmark = 31,
    Character = 10000,
}

impl From<MarkerSymbol> for i32 {
    fn from(val: MarkerSymbol) -> Self {
        val as i32
    }
}

/// Selection mode types for StyledTextCtrl
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    Stream = 0,
    Rectangle = 1,
    Lines = 2,
    Thin = 3,
}

impl From<SelectionMode> for i32 {
    fn from(val: SelectionMode) -> Self {
        val as i32
    }
}

/// Margin types for StyledTextCtrl
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarginType {
    Symbol = 0,
    Number = 1,
    Back = 2,
    Fore = 3,
    Text = 4,
    RText = 5,
    Colour = 6,
}

impl From<MarginType> for i32 {
    fn from(val: MarginType) -> Self {
        val as i32
    }
}

widget_style_enum!(
    name: FindFlags,
    doc: "Search flags for find operations in StyledTextCtrl.",
    variants: {
        None: 0, "No special flags.",
        WholeWord: 0x2, "Match whole words only.",
        MatchCase: 0x4, "Case-sensitive matching.",
        WordStart: 0x00100000, "Match at word start.",
        RegExp: 0x00200000, "Use regular expressions.",
        Posix: 0x00400000, "Use POSIX regular expressions."
    },
    default_variant: None
);

impl FindFlags {
    /// Convert to i32 for FFI calls
    pub fn bits_i32(self) -> i32 {
        self.bits() as i32
    }
}

impl From<FindFlags> for i32 {
    fn from(val: FindFlags) -> Self {
        val.bits() as i32
    }
}

impl From<i32> for FindFlags {
    fn from(bits: i32) -> Self {
        unsafe { std::mem::transmute(bits as i64) }
    }
}

/// Whitespace visibility modes for StyledTextCtrl
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhiteSpaceView {
    Invisible = 0,
    VisibleAlways = 1,
    VisibleAfterIndent = 2,
    VisibleOnlyInIndent = 3,
}

impl From<WhiteSpaceView> for i32 {
    fn from(val: WhiteSpaceView) -> Self {
        val as i32
    }
}

/// Lexer types for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lexer {
    Null = 0,
    Scintilla = 1,
    Container = 2,
    Cpp = 3,
    Python = 4,
    Html = 5,
    Xml = 6,
    Perl = 7,
    Sql = 8,
    Vb = 9,
    Properties = 10,
    Errorlist = 11,
    Makefile = 12,
    Batch = 13,
    Xcode = 14,
    Latex = 15,
    Lua = 16,
    Diff = 17,
    Conf = 18,
    Pascal = 19,
    Ave = 20,
    Ada = 21,
    Lisp = 22,
    Ruby = 23,
    Eiffel = 24,
    Eiffelkw = 25,
    Tcl = 26,
    Nncrontab = 27,
    Bullant = 28,
    Vbscript = 29,
    Baan = 30,
    Matlab = 31,
    Scriptol = 32,
    Asm = 33,
    Cppnocase = 34,
    Fortran = 35,
    F77 = 36,
    Css = 37,
    Pov = 38,
    Lout = 39,
    Escript = 40,
    Ps = 41,
    Nsis = 42,
    Mmixal = 43,
    Clw = 44,
    Clwnocase = 45,
    Lot = 46,
    Yaml = 47,
    Tex = 48,
    Metapost = 49,
    Powerbasic = 50,
    Forth = 51,
    Erlang = 52,
    Octave = 53,
    Mssql = 54,
    Verilog = 55,
    Kix = 56,
    Gui4cli = 57,
    Specman = 58,
    Au3 = 59,
    Apdl = 60,
    Bash = 61,
    Asn1 = 62,
    Vhdl = 63,
    Caml = 64,
    Blitzbasic = 65,
    Purebasic = 66,
    Haskell = 67,
    Phpscript = 68,
    Tads3 = 69,
    Rebol = 70,
    Smalltalk = 71,
    Flagship = 72,
    Csound = 73,
    Freebasic = 74,
    Innosetup = 75,
    Opal = 76,
    Spice = 77,
    D = 78,
    Javascript = 79, // JavaScript now has its own lexer
    Java = 80,       // Java now has its own lexer
}

impl From<Lexer> for i32 {
    fn from(val: Lexer) -> Self {
        val as i32
    }
}

/// End of line mode types
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EolMode {
    CrLf = 0,
    Cr = 1,
    Lf = 2,
}

impl From<EolMode> for i32 {
    fn from(val: EolMode) -> Self {
        val as i32
    }
}

/// Wrap mode types for text wrapping
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapMode {
    None = 0,
    Word = 1,
    Char = 2,
    Whitespace = 3,
}

impl From<WrapMode> for i32 {
    fn from(val: WrapMode) -> Self {
        val as i32
    }
}

impl From<i32> for WrapMode {
    fn from(val: i32) -> Self {
        match val {
            0 => WrapMode::None,
            1 => WrapMode::Word,
            2 => WrapMode::Char,
            3 => WrapMode::Whitespace,
            _ => WrapMode::None,
        }
    }
}

// --- Styled Text Control Styles ---
widget_style_enum!(
    name: StyledTextCtrlStyle,
    doc: "Style flags for StyledTextCtrl widget.",
    variants: {
        Default: 0, "Default style."
    },
    default_variant: Default
);

/// Events emitted by StyledTextCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyledTextCtrlEvent {
    /// The text has changed
    Change,
    /// A style is needed for a range of text
    StyleNeeded,
    /// A character has been added to the text
    CharAdded,
    /// The save point has been reached
    SavePointReached,
    /// The save point has been left
    SavePointLeft,
    /// An attempt was made to change read-only text
    RoModifyAttempt,
    /// The text was double-clicked
    DoubleClick,
    /// The UI needs to be updated
    UpdateUI,
    /// The text has been modified
    Modified,
    /// A macro recording event
    MacroRecord,
    /// A margin was clicked
    MarginClick,
    /// Text needs to be shown
    NeedShown,
    /// The control has been painted
    Painted,
    /// A user list selection was made
    UserListSelection,
    /// Mouse dwelling started
    DwellStart,
    /// Mouse dwelling ended
    DwellEnd,
    /// Drag operation started
    StartDrag,
    /// Drag operation over the control
    DragOver,
    /// Drop operation completed
    DoDrop,
    /// Zoom level changed
    Zoom,
    /// A hotspot was clicked
    HotspotClick,
    /// A hotspot was double-clicked
    HotspotDoubleClick,
    /// A call tip was clicked
    CallTipClick,
    /// An autocompletion selection was made
    AutoCompSelection,
    /// An indicator was clicked
    IndicatorClick,
    /// An indicator was released
    IndicatorRelease,
    /// Autocompletion was cancelled
    AutoCompCancelled,
    /// A character was deleted from autocompletion
    AutoCompCharDeleted,
}

/// Event data for a StyledTextCtrl event
#[derive(Debug)]
pub struct StyledTextCtrlEventData {
    event: Event,
}

impl StyledTextCtrlEventData {
    /// Create a new StyledTextCtrlEventData from a generic Event
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

    /// Get the key code for key events
    pub fn get_key(&self) -> Option<i32> {
        self.event.get_key_code()
    }
}

/// Represents a wxStyledTextCtrl widget.
///
/// StyledTextCtrl is a text editor control based on the Scintilla editing component.
/// It provides syntax highlighting, code folding, and many advanced text editing features.
#[derive(Clone)]
pub struct StyledTextCtrl {
    window: Window,
}

impl StyledTextCtrl {
    /// Creates a new StyledTextCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> StyledTextCtrlBuilder<'_> {
        StyledTextCtrlBuilder::new(parent)
    }

    /// Creates a new StyledTextCtrl wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_StyledTextCtrl_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StyledTextCtrl_t) -> Self {
        StyledTextCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Internal implementation used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        let ptr = unsafe {
            ffi::wxd_StyledTextCtrl_Create(
                parent_ptr,
                id,
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create StyledTextCtrl widget");
        }

        unsafe { StyledTextCtrl::from_ptr(ptr) }
    }

    // --- Text Content Operations ---

    /// Sets the text content of the control.
    pub fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_StyledTextCtrl_SetText(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                c_text.as_ptr(),
            );
        }
    }

    /// Gets the current text content of the control.
    pub fn get_text(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_StyledTextCtrl_GetText(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
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
            ffi::wxd_StyledTextCtrl_AppendText(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                c_text.as_ptr(),
            );
        }
    }

    /// Inserts text at the specified position.
    pub fn insert_text(&self, pos: i32, text: &str) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_StyledTextCtrl_InsertText(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pos,
                c_text.as_ptr(),
            );
        }
    }

    /// Clears all text in the control.
    pub fn clear_all(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_ClearAll(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t);
        }
    }

    /// Deletes a range of text.
    pub fn delete_range(&self, start: i32, length: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_DeleteRange(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                start,
                length,
            );
        }
    }

    /// Returns the length of the text.
    pub fn get_length(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetLength(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t)
        }
    }

    /// Returns the number of lines in the control.
    pub fn get_line_count(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetLineCount(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Returns the character at the specified position.
    pub fn get_char_at(&self, pos: i32) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetCharAt(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pos,
            )
        }
    }

    /// Returns the style at the specified position.
    pub fn get_style_at(&self, pos: i32) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetStyleAt(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pos,
            )
        }
    }

    // --- Clipboard Operations ---

    /// Cuts the selected text to the clipboard.
    pub fn cut(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_Cut(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t);
        }
    }

    /// Copies the selected text to the clipboard.
    pub fn copy(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_Copy(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t);
        }
    }

    /// Pastes text from the clipboard.
    pub fn paste(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_Paste(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t);
        }
    }

    /// Undoes the last action.
    pub fn undo(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_Undo(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t);
        }
    }

    /// Selects all text in the control.
    pub fn select_all(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SelectAll(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            );
        }
    }

    // --- Read-only State ---

    /// Makes the text control editable or read-only.
    pub fn set_read_only(&self, read_only: bool) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetReadOnly(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                read_only,
            );
        }
    }

    /// Returns true if the control is read-only.
    pub fn is_read_only(&self) -> bool {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetReadOnly(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    // --- Position and Selection Operations ---

    /// Returns the current position.
    pub fn get_current_pos(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetCurrentPos(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Sets the current position.
    pub fn set_current_pos(&self, pos: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetCurrentPos(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pos,
            );
        }
    }

    /// Gets the selection range.
    pub fn get_selection(&self) -> (i32, i32) {
        let mut start = 0;
        let mut end = 0;
        unsafe {
            ffi::wxd_StyledTextCtrl_GetSelection(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                &mut start,
                &mut end,
            );
        }
        (start, end)
    }

    /// Sets the selection range.
    pub fn set_selection(&self, start: i32, end: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetSelection(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                start,
                end,
            );
        }
    }

    /// Returns the start of the selection.
    pub fn get_selection_start(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetSelectionStart(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Returns the end of the selection.
    pub fn get_selection_end(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetSelectionEnd(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Gets the currently selected text.
    pub fn get_selected_text(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_StyledTextCtrl_GetSelectedText(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
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

    /// Set the selection mode (stream, rectangle, lines, etc.)
    pub fn set_selection_mode(&self, selection_mode: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetSelectionMode(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                selection_mode,
            );
        }
    }

    /// Set selection mode with type-safe enum
    pub fn set_selection_mode_typed(&self, selection_mode: SelectionMode) {
        self.set_selection_mode(selection_mode.into());
    }

    /// Gets the current selection mode.
    pub fn get_selection_mode(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetSelectionMode(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    // --- Navigation and View Operations ---

    /// Ensures the caret is visible in the view.
    pub fn ensure_caret_visible(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_EnsureCaretVisible(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            );
        }
    }

    /// Scrolls the view by the specified number of columns and lines.
    pub fn line_scroll(&self, columns: i32, lines: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_LineScroll(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                columns,
                lines,
            );
        }
    }

    /// Scrolls to make the specified line visible.
    pub fn scroll_to_line(&self, line: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_ScrollToLine(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
            );
        }
    }

    /// Scrolls to make the specified column visible.
    pub fn scroll_to_column(&self, column: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_ScrollToColumn(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                column,
            );
        }
    }

    // --- Line Operations ---

    /// Returns the line number for a position.
    pub fn line_from_position(&self, pos: i32) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_LineFromPosition(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pos,
            )
        }
    }

    /// Returns the position at the start of a line.
    pub fn position_from_line(&self, line: i32) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_PositionFromLine(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
            )
        }
    }

    /// Gets the text for a specific line.
    pub fn get_line_text(&self, line: i32) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_StyledTextCtrl_GetLineText(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
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

    /// Returns the length of a specific line.
    pub fn get_line_length(&self, line: i32) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetLineLength(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
            )
        }
    }

    // --- Marker Operations ---

    /// Define a marker with the specified symbol and colors
    pub fn marker_define(
        &self,
        marker_number: i32,
        marker_symbol: i32,
        foreground: Colour,
        background: Colour,
    ) {
        unsafe {
            ffi::wxd_StyledTextCtrl_MarkerDefine(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                marker_number,
                marker_symbol,
                foreground.into(),
                background.into(),
            );
        }
    }

    /// Define a marker with type-safe marker symbol
    pub fn marker_define_symbol(
        &self,
        marker_number: i32,
        marker_symbol: MarkerSymbol,
        foreground: Colour,
        background: Colour,
    ) {
        self.marker_define(marker_number, marker_symbol.into(), foreground, background);
    }

    /// Adds a marker to a line.
    pub fn marker_add(&self, line: i32, marker_number: i32) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_MarkerAdd(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
                marker_number,
            )
        }
    }

    /// Deletes a marker from a line.
    pub fn marker_delete(&self, line: i32, marker_number: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_MarkerDelete(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
                marker_number,
            );
        }
    }

    /// Deletes all markers of a specific type.
    pub fn marker_delete_all(&self, marker_number: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_MarkerDeleteAll(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                marker_number,
            );
        }
    }

    /// Gets the markers on a line.
    pub fn marker_get(&self, line: i32) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_MarkerGet(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
            )
        }
    }

    /// Finds the next line with a marker.
    pub fn marker_next(&self, line_start: i32, marker_mask: i32) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_MarkerNext(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line_start,
                marker_mask,
            )
        }
    }

    /// Finds the previous line with a marker.
    pub fn marker_previous(&self, line_start: i32, marker_mask: i32) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_MarkerPrevious(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line_start,
                marker_mask,
            )
        }
    }

    /// Sets the foreground color for a marker.
    pub fn marker_set_foreground(&self, marker_number: i32, color: Colour) {
        unsafe {
            ffi::wxd_StyledTextCtrl_MarkerSetForeground(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                marker_number,
                color.into(),
            );
        }
    }

    /// Sets the background color for a marker.
    pub fn marker_set_background(&self, marker_number: i32, color: Colour) {
        unsafe {
            ffi::wxd_StyledTextCtrl_MarkerSetBackground(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                marker_number,
                color.into(),
            );
        }
    }

    // --- Styling Operations ---

    /// Sets the font for a specific style.
    pub fn style_set_font(&self, style: i32, font: &Font) {
        unsafe {
            ffi::wxd_StyledTextCtrl_StyleSetFont(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                style,
                font.as_ptr(),
            );
        }
    }

    /// Sets the foreground color for a specific style.
    pub fn style_set_foreground(&self, style: i32, color: Colour) {
        unsafe {
            ffi::wxd_StyledTextCtrl_StyleSetForeground(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                style,
                color.into(),
            );
        }
    }

    /// Sets the background color for a specific style.
    pub fn style_set_background(&self, style: i32, color: Colour) {
        unsafe {
            ffi::wxd_StyledTextCtrl_StyleSetBackground(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                style,
                color.into(),
            );
        }
    }

    /// Sets the bold attribute for a specific style.
    pub fn style_set_bold(&self, style: i32, bold: bool) {
        unsafe {
            ffi::wxd_StyledTextCtrl_StyleSetBold(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                style,
                bold,
            );
        }
    }

    /// Sets the italic attribute for a specific style.
    pub fn style_set_italic(&self, style: i32, italic: bool) {
        unsafe {
            ffi::wxd_StyledTextCtrl_StyleSetItalic(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                style,
                italic,
            );
        }
    }

    /// Sets the underline attribute for a specific style.
    pub fn style_set_underline(&self, style: i32, underline: bool) {
        unsafe {
            ffi::wxd_StyledTextCtrl_StyleSetUnderline(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                style,
                underline,
            );
        }
    }

    /// Sets the font size for a specific style.
    pub fn style_set_size(&self, style: i32, size: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_StyleSetSize(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                style,
                size,
            );
        }
    }

    /// Clears all style definitions and sets them to default.
    pub fn style_clear_all(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_StyleClearAll(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            );
        }
    }

    /// Prepares to set styling for text starting at the given position.
    pub fn start_styling(&self, start: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_StartStyling(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                start,
            );
        }
    }

    /// Sets styling for a range of text.
    pub fn set_styling(&self, length: i32, style: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetStyling(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                length,
                style,
            );
        }
    }

    // --- Lexer and Language Support ---

    /// Set the lexer for syntax highlighting
    pub fn set_lexer(&self, lexer: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetLexer(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                lexer,
            );
        }
    }

    /// Set lexer with type-safe enum
    pub fn set_lexer_typed(&self, lexer: Lexer) {
        self.set_lexer(lexer.into());
    }

    /// Gets the current lexer.
    pub fn get_lexer(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetLexer(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t)
        }
    }

    /// Sets the lexer language.
    pub fn set_lexer_language(&self, language: &str) {
        let c_language = CString::new(language).unwrap_or_default();
        unsafe {
            ffi::wxd_StyledTextCtrl_SetLexerLanguage(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                c_language.as_ptr(),
            );
        }
    }

    // --- Margin Operations ---

    /// Set the type of margin (symbol, number, text, etc.)
    pub fn set_margin_type(&self, margin: i32, margin_type: MarginType) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetMarginType(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                margin,
                margin_type as i32,
            );
        }
    }

    /// Set margin type with type-safe enum
    pub fn set_margin_type_typed(&self, margin: i32, margin_type: MarginType) {
        self.set_margin_type(margin, margin_type);
    }

    /// Sets the width of a margin in pixels.
    pub fn set_margin_width(&self, margin: i32, pixel_width: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetMarginWidth(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                margin,
                pixel_width,
            );
        }
    }

    /// Sets whether a margin displays line numbers.
    pub fn set_margin_line_numbers(&self, margin: i32, line_numbers: bool) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetMarginLineNumbers(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                margin,
                line_numbers,
            );
        }
    }

    // --- Zoom Operations ---

    /// Zooms in (increases font size).
    pub fn zoom_in(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_ZoomIn(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t);
        }
    }

    /// Zooms out (decreases font size).
    pub fn zoom_out(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_ZoomOut(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t);
        }
    }

    /// Sets the zoom level.
    pub fn set_zoom(&self, zoom_level: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetZoom(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                zoom_level,
            );
        }
    }

    /// Gets the current zoom level.
    pub fn get_zoom(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetZoom(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t)
        }
    }

    // --- Modified State ---

    /// Returns true if the text has been modified.
    pub fn is_modified(&self) -> bool {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetModify(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t)
        }
    }

    /// Sets the save point (marks the document as saved).
    pub fn set_save_point(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetSavePoint(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            );
        }
    }

    // --- Find and Replace ---

    pub fn search_anchor(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SearchAnchor(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Find text in the document with specified flags
    pub fn find_text(
        &self,
        min_pos: i32,
        max_pos: i32,
        text: &str,
        flags: FindFlags,
    ) -> Option<i32> {
        let c_text = CString::new(text).unwrap();
        unsafe {
            let result = ffi::wxd_StyledTextCtrl_FindText(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                min_pos,
                max_pos,
                c_text.as_ptr(),
                flags.bits_i32(),
            );
            if result >= 0 {
                Some(result)
            } else {
                None
            }
        }
    }

    /// Find text with type-safe flags
    pub fn find_text_typed(
        &self,
        min_pos: i32,
        max_pos: i32,
        text: &str,
        flags: FindFlags,
    ) -> Option<i32> {
        self.find_text(min_pos, max_pos, text, flags)
    }

    /// Find text with combined flags
    pub fn find_text_combined_flags(
        &self,
        min_pos: i32,
        max_pos: i32,
        text: &str,
        flags: i32,
    ) -> Option<i32> {
        self.find_text(min_pos, max_pos, text, FindFlags::from(flags))
    }

    /// Search for text forwards from current position
    pub fn search_next(&self, search_flags: FindFlags, text: &str) -> Option<i32> {
        let c_text = CString::new(text).unwrap();
        unsafe {
            let result = ffi::wxd_StyledTextCtrl_SearchNext(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                search_flags.bits_i32(),
                c_text.as_ptr(),
            );
            if result >= 0 {
                Some(result)
            } else {
                None
            }
        }
    }

    /// Search for text backwards from current position
    pub fn search_prev(&self, search_flags: FindFlags, text: &str) -> Option<i32> {
        let c_text = CString::new(text).unwrap();
        unsafe {
            let result = ffi::wxd_StyledTextCtrl_SearchPrev(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                search_flags.bits_i32(),
                c_text.as_ptr(),
            );
            if result >= 0 {
                Some(result)
            } else {
                None
            }
        }
    }

    /// Search next with type-safe flags
    pub fn search_next_typed(&self, search_flags: FindFlags, text: &str) -> Option<i32> {
        self.search_next(search_flags, text)
    }

    /// Search previous with type-safe flags
    pub fn search_prev_typed(&self, search_flags: FindFlags, text: &str) -> Option<i32> {
        self.search_prev(search_flags, text)
    }

    /// Replace the current selection with text
    pub fn replace_selection(&self, text: &str) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_StyledTextCtrl_ReplaceSelection(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                c_text.as_ptr(),
            );
        }
    }

    /// Replace text in the target range
    pub fn replace_target(&self, text: &str) -> i32 {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_StyledTextCtrl_ReplaceTarget(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                c_text.as_ptr(),
            )
        }
    }

    /// Set the start of the target range for search/replace operations
    pub fn set_target_start(&self, start: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetTargetStart(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                start,
            );
        }
    }

    /// Set the end of the target range for search/replace operations
    pub fn set_target_end(&self, end: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetTargetEnd(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                end,
            );
        }
    }

    /// Get the start of the target range
    pub fn get_target_start(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetTargetStart(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Get the end of the target range
    pub fn get_target_end(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetTargetEnd(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    // --- Navigation Operations ---

    /// Get the line number containing the caret
    pub fn get_current_line(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetCurrentLine(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Move the caret to the start of a line
    pub fn goto_line(&self, line: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_GotoLine(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
            );
        }
    }

    /// Move the caret to a specific position
    pub fn goto_pos(&self, pos: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_GotoPos(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pos,
            );
        }
    }

    // --- Tab and Indentation ---

    /// Set the width of tabs in characters
    pub fn set_tab_width(&self, tab_width: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetTabWidth(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                tab_width,
            );
        }
    }

    /// Get the width of tabs in characters
    pub fn get_tab_width(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetTabWidth(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Set the number of spaces used for one level of indentation
    pub fn set_indent(&self, indent_size: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetIndent(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                indent_size,
            );
        }
    }

    /// Get the number of spaces used for one level of indentation
    pub fn get_indent(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetIndent(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t)
        }
    }

    /// Set whether to use tabs for indentation
    pub fn set_use_tabs(&self, use_tabs: bool) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetUseTabs(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                use_tabs,
            );
        }
    }

    /// Get whether tabs are used for indentation
    pub fn get_use_tabs(&self) -> bool {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetUseTabs(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Set the indentation of a specific line
    pub fn set_line_indentation(&self, line: i32, indentation: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetLineIndentation(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
                indentation,
            );
        }
    }

    /// Get the indentation of a specific line
    pub fn get_line_indentation(&self, line: i32) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetLineIndentation(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
            )
        }
    }

    // --- View Options ---

    /// Set whether end-of-line characters are visible
    pub fn set_view_eol(&self, visible: bool) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetViewEOL(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                visible,
            );
        }
    }

    /// Get whether end-of-line characters are visible
    pub fn get_view_eol(&self) -> bool {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetViewEOL(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Set whitespace visibility mode
    pub fn set_view_white_space(&self, view_ws: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetViewWhiteSpace(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                view_ws,
            );
        }
    }

    /// Set whitespace visibility with type-safe enum
    pub fn set_view_white_space_typed(&self, view_ws: WhiteSpaceView) {
        self.set_view_white_space(view_ws.into());
    }

    /// Get how white space is displayed
    pub fn get_view_white_space(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetViewWhiteSpace(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    // --- Caret Operations ---

    /// Set the blink period of the caret in milliseconds
    pub fn set_caret_period(&self, period_ms: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetCaretPeriod(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                period_ms,
            );
        }
    }

    /// Get the blink period of the caret in milliseconds
    pub fn get_caret_period(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetCaretPeriod(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Set the width of the caret in pixels
    pub fn set_caret_width(&self, pixel_width: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetCaretWidth(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pixel_width,
            );
        }
    }

    /// Get the width of the caret in pixels
    pub fn get_caret_width(&self) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetCaretWidth(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Set whether the line containing the caret is highlighted
    pub fn set_caret_line_visible(&self, show: bool) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetCaretLineVisible(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                show,
            );
        }
    }

    /// Get whether the line containing the caret is highlighted
    pub fn get_caret_line_visible(&self) -> bool {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetCaretLineVisible(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Set the background color of the line containing the caret
    pub fn set_caret_line_background(&self, color: Colour) {
        let c_color = color.to_raw();
        unsafe {
            ffi::wxd_StyledTextCtrl_SetCaretLineBackground(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                c_color,
            );
        }
    }

    // --- Undo/Redo Operations ---

    /// Redo the next action
    pub fn redo(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_Redo(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t);
        }
    }

    /// Check if there are actions that can be undone
    pub fn can_undo(&self) -> bool {
        unsafe {
            ffi::wxd_StyledTextCtrl_CanUndo(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t)
        }
    }

    /// Check if there are actions that can be redone
    pub fn can_redo(&self) -> bool {
        unsafe {
            ffi::wxd_StyledTextCtrl_CanRedo(self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t)
        }
    }

    /// Clear the undo buffer
    pub fn empty_undo_buffer(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_EmptyUndoBuffer(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            );
        }
    }

    // --- Autocompletion ---

    /// Display an auto-completion list
    pub fn auto_comp_show(&self, length_entered: i32, item_list: &str) {
        let c_item_list = CString::new(item_list).unwrap_or_default();
        unsafe {
            ffi::wxd_StyledTextCtrl_AutoCompShow(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                length_entered,
                c_item_list.as_ptr(),
            );
        }
    }

    /// Cancel any displayed auto-completion list
    pub fn auto_comp_cancel(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_AutoCompCancel(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            );
        }
    }

    /// Check if an auto-completion list is currently displayed
    pub fn auto_comp_active(&self) -> bool {
        unsafe {
            ffi::wxd_StyledTextCtrl_AutoCompActive(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Complete the word being entered
    pub fn auto_comp_complete(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_AutoCompComplete(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            );
        }
    }

    /// Set the separator character for auto-completion lists
    pub fn auto_comp_set_separator(&self, separator_char: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_AutoCompSetSeparator(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                separator_char,
            );
        }
    }

    /// Select an item in the auto-completion list
    pub fn auto_comp_select(&self, select: &str) {
        let c_select = CString::new(select).unwrap_or_default();
        unsafe {
            ffi::wxd_StyledTextCtrl_AutoCompSelect(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                c_select.as_ptr(),
            );
        }
    }

    // --- Bracket Matching ---

    /// Highlight matching braces
    pub fn brace_highlight(&self, pos_a: i32, pos_b: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_BraceHighlight(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pos_a,
                pos_b,
            );
        }
    }

    /// Highlight an unmatched brace
    pub fn brace_bad_light(&self, pos: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_BraceBadLight(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pos,
            );
        }
    }

    /// Find the matching brace for the character at the given position
    pub fn brace_match(&self, pos: i32) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_BraceMatch(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pos,
            )
        }
    }

    // --- Call Tips ---

    /// Show a call tip at the specified position
    pub fn call_tip_show(&self, pos: i32, definition: &str) {
        let c_definition = CString::new(definition).unwrap_or_default();
        unsafe {
            ffi::wxd_StyledTextCtrl_CallTipShow(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pos,
                c_definition.as_ptr(),
            );
        }
    }

    /// Cancel any displayed call tip
    pub fn call_tip_cancel(&self) {
        unsafe {
            ffi::wxd_StyledTextCtrl_CallTipCancel(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            );
        }
    }

    /// Check if a call tip is currently displayed
    pub fn call_tip_active(&self) -> bool {
        unsafe {
            ffi::wxd_StyledTextCtrl_CallTipActive(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        }
    }

    /// Set the highlight range in a call tip
    pub fn call_tip_set_highlight(&self, highlight_start: i32, highlight_end: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_CallTipSetHighlight(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                highlight_start,
                highlight_end,
            );
        }
    }

    // --- Folding Operations ---

    /// Set the fold level of a line
    pub fn set_fold_level(&self, line: i32, level: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetFoldLevel(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
                level,
            );
        }
    }

    /// Get the fold level of a line
    pub fn get_fold_level(&self, line: i32) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetFoldLevel(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
            )
        }
    }

    /// Toggle the fold state of a line
    pub fn toggle_fold(&self, line: i32) {
        unsafe {
            ffi::wxd_StyledTextCtrl_ToggleFold(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
            );
        }
    }

    /// Set whether a fold header line is expanded
    pub fn set_fold_expanded(&self, line: i32, expanded: bool) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetFoldExpanded(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
                expanded,
            );
        }
    }

    /// Get whether a fold header line is expanded
    pub fn get_fold_expanded(&self, line: i32) -> bool {
        unsafe {
            ffi::wxd_StyledTextCtrl_GetFoldExpanded(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                line,
            )
        }
    }

    // --- Word Operations ---

    /// Find the start position of a word
    pub fn word_start_position(&self, pos: i32, only_word_chars: bool) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_WordStartPosition(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pos,
                only_word_chars,
            )
        }
    }

    /// Find the end position of a word
    pub fn word_end_position(&self, pos: i32, only_word_chars: bool) -> i32 {
        unsafe {
            ffi::wxd_StyledTextCtrl_WordEndPosition(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                pos,
                only_word_chars,
            )
        }
    }

    // --- Wrap Mode Operations ---

    /// Set the wrap mode for long lines
    pub fn set_wrap_mode(&self, wrap_mode: WrapMode) {
        unsafe {
            ffi::wxd_StyledTextCtrl_SetWrapMode(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t,
                wrap_mode.into(),
            );
        }
    }

    /// Get the current wrap mode
    pub fn get_wrap_mode(&self) -> WrapMode {
        let mode = unsafe {
            ffi::wxd_StyledTextCtrl_GetWrapMode(
                self.window.as_ptr() as *mut ffi::wxd_StyledTextCtrl_t
            )
        };
        WrapMode::from(mode)
    }
}

// Apply common trait implementations for this widget
implement_widget_traits_with_target!(StyledTextCtrl, window, Window);

// Implement scrolling functionality for StyledTextCtrl
impl crate::scrollable::WxScrollable for StyledTextCtrl {}

// Use the widget_builder macro for StyledTextCtrl
widget_builder!(
    name: StyledTextCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: StyledTextCtrlStyle,
    fields: {},
    build_impl: |slf| {
        StyledTextCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
);

// Implement StyledTextCtrl-specific event handlers using the standard macro
crate::implement_widget_local_event_handlers!(
    StyledTextCtrl,
    StyledTextCtrlEvent,
    StyledTextCtrlEventData,
    Change => stc_change, EventType::STC_CHANGE,
    StyleNeeded => stc_style_needed, EventType::STC_STYLENEEDED,
    CharAdded => stc_char_added, EventType::STC_CHARADDED,
    SavePointReached => stc_save_point_reached, EventType::STC_SAVEPOINTREACHED,
    SavePointLeft => stc_save_point_left, EventType::STC_SAVEPOINTLEFT,
    RoModifyAttempt => stc_ro_modify_attempt, EventType::STC_ROMODIFYATTEMPT,
    DoubleClick => stc_double_click, EventType::STC_DOUBLECLICK,
    UpdateUI => stc_update_ui, EventType::STC_UPDATEUI,
    Modified => stc_modified, EventType::STC_MODIFIED,
    MacroRecord => stc_macro_record, EventType::STC_MACRORECORD,
    MarginClick => stc_margin_click, EventType::STC_MARGINCLICK,
    NeedShown => stc_need_shown, EventType::STC_NEEDSHOWN,
    Painted => stc_painted, EventType::STC_PAINTED,
    UserListSelection => stc_user_list_selection, EventType::STC_USERLISTSELECTION,
    DwellStart => stc_dwell_start, EventType::STC_DWELLSTART,
    DwellEnd => stc_dwell_end, EventType::STC_DWELLEND,
    StartDrag => stc_start_drag, EventType::STC_START_DRAG,
    DragOver => stc_drag_over, EventType::STC_DRAG_OVER,
    DoDrop => stc_do_drop, EventType::STC_DO_DROP,
    Zoom => stc_zoom, EventType::STC_ZOOM,
    HotspotClick => stc_hotspot_click, EventType::STC_HOTSPOT_CLICK,
    HotspotDoubleClick => stc_hotspot_double_click, EventType::STC_HOTSPOT_DCLICK,
    CallTipClick => stc_call_tip_click, EventType::STC_CALLTIP_CLICK,
    AutoCompSelection => stc_autocomp_selection, EventType::STC_AUTOCOMP_SELECTION,
    IndicatorClick => stc_indicator_click, EventType::STC_INDICATOR_CLICK,
    IndicatorRelease => stc_indicator_release, EventType::STC_INDICATOR_RELEASE,
    AutoCompCancelled => stc_autocomp_cancelled, EventType::STC_AUTOCOMP_CANCELLED,
    AutoCompCharDeleted => stc_autocomp_char_deleted, EventType::STC_AUTOCOMP_CHAR_DELETED
);

// Implement standard window events trait
impl WindowEvents for StyledTextCtrl {}

// Implement XRC support for loading StyledTextCtrl from XRC files
crate::impl_xrc_support!(StyledTextCtrl, { window });
