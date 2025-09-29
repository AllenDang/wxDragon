#ifndef WXD_STYLEDTEXTCTRL_H
#define WXD_STYLEDTEXTCTRL_H

#include "../wxd_types.h"

// --- StyledTextCtrl Functions ---

// Creation and basic operations
WXD_EXPORTED wxd_StyledTextCtrl_t* wxd_StyledTextCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);

// Text content operations
WXD_EXPORTED void wxd_StyledTextCtrl_SetText(wxd_StyledTextCtrl_t* self, const char* text);
WXD_EXPORTED int wxd_StyledTextCtrl_GetText(wxd_StyledTextCtrl_t* self, char* buffer, int buffer_len);
WXD_EXPORTED void wxd_StyledTextCtrl_AppendText(wxd_StyledTextCtrl_t* self, const char* text);
WXD_EXPORTED void wxd_StyledTextCtrl_InsertText(wxd_StyledTextCtrl_t* self, int pos, const char* text);
WXD_EXPORTED void wxd_StyledTextCtrl_ClearAll(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_DeleteRange(wxd_StyledTextCtrl_t* self, int start, int length);
WXD_EXPORTED int wxd_StyledTextCtrl_GetLength(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED int wxd_StyledTextCtrl_GetLineCount(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED int wxd_StyledTextCtrl_GetCharAt(wxd_StyledTextCtrl_t* self, int pos);
WXD_EXPORTED int wxd_StyledTextCtrl_GetStyleAt(wxd_StyledTextCtrl_t* self, int pos);

// Clipboard operations
WXD_EXPORTED void wxd_StyledTextCtrl_Cut(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_Copy(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_Paste(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_Undo(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_SelectAll(wxd_StyledTextCtrl_t* self);

// Read-only state
WXD_EXPORTED void wxd_StyledTextCtrl_SetReadOnly(wxd_StyledTextCtrl_t* self, bool readOnly);
WXD_EXPORTED bool wxd_StyledTextCtrl_GetReadOnly(wxd_StyledTextCtrl_t* self);

// Position and selection operations
WXD_EXPORTED int wxd_StyledTextCtrl_GetCurrentPos(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_SetCurrentPos(wxd_StyledTextCtrl_t* self, int pos);
WXD_EXPORTED void wxd_StyledTextCtrl_GetSelection(wxd_StyledTextCtrl_t* self, int* start, int* end);
WXD_EXPORTED void wxd_StyledTextCtrl_SetSelection(wxd_StyledTextCtrl_t* self, int start, int end);
WXD_EXPORTED int wxd_StyledTextCtrl_GetSelectionStart(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED int wxd_StyledTextCtrl_GetSelectionEnd(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED int wxd_StyledTextCtrl_GetSelectedText(wxd_StyledTextCtrl_t* self, char* buffer, int buffer_len);
WXD_EXPORTED void wxd_StyledTextCtrl_SetSelectionMode(wxd_StyledTextCtrl_t* self, int selectionMode);
WXD_EXPORTED int wxd_StyledTextCtrl_GetSelectionMode(wxd_StyledTextCtrl_t* self);

// Navigation and view operations
WXD_EXPORTED void wxd_StyledTextCtrl_EnsureCaretVisible(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_LineScroll(wxd_StyledTextCtrl_t* self, int columns, int lines);
WXD_EXPORTED void wxd_StyledTextCtrl_ScrollToLine(wxd_StyledTextCtrl_t* self, int line);
WXD_EXPORTED void wxd_StyledTextCtrl_ScrollToColumn(wxd_StyledTextCtrl_t* self, int column);

// Line operations
WXD_EXPORTED int wxd_StyledTextCtrl_LineFromPosition(wxd_StyledTextCtrl_t* self, int pos);
WXD_EXPORTED int wxd_StyledTextCtrl_PositionFromLine(wxd_StyledTextCtrl_t* self, int line);
WXD_EXPORTED int wxd_StyledTextCtrl_GetLineText(wxd_StyledTextCtrl_t* self, int line, char* buffer, int buffer_len);
WXD_EXPORTED int wxd_StyledTextCtrl_GetLineLength(wxd_StyledTextCtrl_t* self, int line);

// Marker operations
WXD_EXPORTED void wxd_StyledTextCtrl_MarkerDefine(wxd_StyledTextCtrl_t* self, int markerNumber, int markerSymbol, wxd_Colour_t foreground, wxd_Colour_t background);
WXD_EXPORTED int wxd_StyledTextCtrl_MarkerAdd(wxd_StyledTextCtrl_t* self, int line, int markerNumber);
WXD_EXPORTED void wxd_StyledTextCtrl_MarkerDelete(wxd_StyledTextCtrl_t* self, int line, int markerNumber);
WXD_EXPORTED void wxd_StyledTextCtrl_MarkerDeleteAll(wxd_StyledTextCtrl_t* self, int markerNumber);
WXD_EXPORTED int wxd_StyledTextCtrl_MarkerGet(wxd_StyledTextCtrl_t* self, int line);
WXD_EXPORTED int wxd_StyledTextCtrl_MarkerNext(wxd_StyledTextCtrl_t* self, int lineStart, int markerMask);
WXD_EXPORTED int wxd_StyledTextCtrl_MarkerPrevious(wxd_StyledTextCtrl_t* self, int lineStart, int markerMask);
WXD_EXPORTED void wxd_StyledTextCtrl_MarkerSetForeground(wxd_StyledTextCtrl_t* self, int markerNumber, wxd_Colour_t colour);
WXD_EXPORTED void wxd_StyledTextCtrl_MarkerSetBackground(wxd_StyledTextCtrl_t* self, int markerNumber, wxd_Colour_t colour);

// Styling operations
WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetFont(wxd_StyledTextCtrl_t* self, int style, wxd_Font_t* font);
WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetForeground(wxd_StyledTextCtrl_t* self, int style, wxd_Colour_t colour);
WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetBackground(wxd_StyledTextCtrl_t* self, int style, wxd_Colour_t colour);
WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetBold(wxd_StyledTextCtrl_t* self, int style, bool bold);
WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetItalic(wxd_StyledTextCtrl_t* self, int style, bool italic);
WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetUnderline(wxd_StyledTextCtrl_t* self, int style, bool underline);
WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetSize(wxd_StyledTextCtrl_t* self, int style, int size);
WXD_EXPORTED void wxd_StyledTextCtrl_StyleClearAll(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_StartStyling(wxd_StyledTextCtrl_t* self, int start);
WXD_EXPORTED void wxd_StyledTextCtrl_SetStyling(wxd_StyledTextCtrl_t* self, int length, int style);

// Lexer and language support
WXD_EXPORTED void wxd_StyledTextCtrl_SetLexer(wxd_StyledTextCtrl_t* self, int lexer);
WXD_EXPORTED int wxd_StyledTextCtrl_GetLexer(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_SetLexerLanguage(wxd_StyledTextCtrl_t* self, const char* language);

// Margin operations
WXD_EXPORTED void wxd_StyledTextCtrl_SetMarginType(wxd_StyledTextCtrl_t* self, int margin, int margin_type);
WXD_EXPORTED void wxd_StyledTextCtrl_SetMarginWidth(wxd_StyledTextCtrl_t* self, int margin, int pixel_width);
WXD_EXPORTED void wxd_StyledTextCtrl_SetMarginLineNumbers(wxd_StyledTextCtrl_t* self, int margin, bool line_numbers);

// Zoom operations
WXD_EXPORTED void wxd_StyledTextCtrl_ZoomIn(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_ZoomOut(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_SetZoom(wxd_StyledTextCtrl_t* self, int zoom_level);
WXD_EXPORTED int wxd_StyledTextCtrl_GetZoom(wxd_StyledTextCtrl_t* self);

// Modified state
WXD_EXPORTED bool wxd_StyledTextCtrl_GetModify(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_SetSavePoint(wxd_StyledTextCtrl_t* self);

// Find and replace
WXD_EXPORTED void wxd_StyledTextCtrl_SearchAnchor(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED int wxd_StyledTextCtrl_FindText(wxd_StyledTextCtrl_t* self, int min_pos, int max_pos, const char* text, int flags);
WXD_EXPORTED int wxd_StyledTextCtrl_SearchNext(wxd_StyledTextCtrl_t* self, int search_flags, const char* text);
WXD_EXPORTED int wxd_StyledTextCtrl_SearchPrev(wxd_StyledTextCtrl_t* self, int search_flags, const char* text);
WXD_EXPORTED void wxd_StyledTextCtrl_ReplaceSelection(wxd_StyledTextCtrl_t* self, const char* text);
WXD_EXPORTED int wxd_StyledTextCtrl_ReplaceTarget(wxd_StyledTextCtrl_t* self, const char* text);
WXD_EXPORTED void wxd_StyledTextCtrl_SetTargetStart(wxd_StyledTextCtrl_t* self, int start);
WXD_EXPORTED void wxd_StyledTextCtrl_SetTargetEnd(wxd_StyledTextCtrl_t* self, int end);
WXD_EXPORTED int wxd_StyledTextCtrl_GetTargetStart(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED int wxd_StyledTextCtrl_GetTargetEnd(wxd_StyledTextCtrl_t* self);

// Navigation operations
WXD_EXPORTED int wxd_StyledTextCtrl_GetCurrentLine(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_GotoLine(wxd_StyledTextCtrl_t* self, int line);
WXD_EXPORTED void wxd_StyledTextCtrl_GotoPos(wxd_StyledTextCtrl_t* self, int pos);

// Tab and indentation
WXD_EXPORTED void wxd_StyledTextCtrl_SetTabWidth(wxd_StyledTextCtrl_t* self, int tab_width);
WXD_EXPORTED int wxd_StyledTextCtrl_GetTabWidth(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_SetIndent(wxd_StyledTextCtrl_t* self, int indent_size);
WXD_EXPORTED int wxd_StyledTextCtrl_GetIndent(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_SetUseTabs(wxd_StyledTextCtrl_t* self, bool use_tabs);
WXD_EXPORTED bool wxd_StyledTextCtrl_GetUseTabs(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_SetLineIndentation(wxd_StyledTextCtrl_t* self, int line, int indentation);
WXD_EXPORTED int wxd_StyledTextCtrl_GetLineIndentation(wxd_StyledTextCtrl_t* self, int line);

// View options
WXD_EXPORTED void wxd_StyledTextCtrl_SetViewEOL(wxd_StyledTextCtrl_t* self, bool visible);
WXD_EXPORTED bool wxd_StyledTextCtrl_GetViewEOL(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_SetViewWhiteSpace(wxd_StyledTextCtrl_t* self, int view_ws);
WXD_EXPORTED int wxd_StyledTextCtrl_GetViewWhiteSpace(wxd_StyledTextCtrl_t* self);

// Caret operations
WXD_EXPORTED void wxd_StyledTextCtrl_SetCaretPeriod(wxd_StyledTextCtrl_t* self, int period_ms);
WXD_EXPORTED int wxd_StyledTextCtrl_GetCaretPeriod(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_SetCaretWidth(wxd_StyledTextCtrl_t* self, int pixel_width);
WXD_EXPORTED int wxd_StyledTextCtrl_GetCaretWidth(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_SetCaretLineVisible(wxd_StyledTextCtrl_t* self, bool show);
WXD_EXPORTED bool wxd_StyledTextCtrl_GetCaretLineVisible(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_SetCaretLineBackground(wxd_StyledTextCtrl_t* self, wxd_Colour_t back);

// Undo/Redo operations
WXD_EXPORTED void wxd_StyledTextCtrl_Redo(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED bool wxd_StyledTextCtrl_CanUndo(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED bool wxd_StyledTextCtrl_CanRedo(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_EmptyUndoBuffer(wxd_StyledTextCtrl_t* self);

// Autocompletion
WXD_EXPORTED void wxd_StyledTextCtrl_AutoCompShow(wxd_StyledTextCtrl_t* self, int length_entered, const char* item_list);
WXD_EXPORTED void wxd_StyledTextCtrl_AutoCompCancel(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED bool wxd_StyledTextCtrl_AutoCompActive(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_AutoCompComplete(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_AutoCompSetSeparator(wxd_StyledTextCtrl_t* self, int separator_char);
WXD_EXPORTED void wxd_StyledTextCtrl_AutoCompSelect(wxd_StyledTextCtrl_t* self, const char* select);

// Bracket matching
WXD_EXPORTED void wxd_StyledTextCtrl_BraceHighlight(wxd_StyledTextCtrl_t* self, int pos_a, int pos_b);
WXD_EXPORTED void wxd_StyledTextCtrl_BraceBadLight(wxd_StyledTextCtrl_t* self, int pos);
WXD_EXPORTED int wxd_StyledTextCtrl_BraceMatch(wxd_StyledTextCtrl_t* self, int pos);

// Call tips
WXD_EXPORTED void wxd_StyledTextCtrl_CallTipShow(wxd_StyledTextCtrl_t* self, int pos, const char* definition);
WXD_EXPORTED void wxd_StyledTextCtrl_CallTipCancel(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED bool wxd_StyledTextCtrl_CallTipActive(wxd_StyledTextCtrl_t* self);
WXD_EXPORTED void wxd_StyledTextCtrl_CallTipSetHighlight(wxd_StyledTextCtrl_t* self, int highlight_start, int highlight_end);

// Folding operations
WXD_EXPORTED void wxd_StyledTextCtrl_SetFoldLevel(wxd_StyledTextCtrl_t* self, int line, int level);
WXD_EXPORTED int wxd_StyledTextCtrl_GetFoldLevel(wxd_StyledTextCtrl_t* self, int line);
WXD_EXPORTED void wxd_StyledTextCtrl_ToggleFold(wxd_StyledTextCtrl_t* self, int line);
WXD_EXPORTED void wxd_StyledTextCtrl_SetFoldExpanded(wxd_StyledTextCtrl_t* self, int line, bool expanded);
WXD_EXPORTED bool wxd_StyledTextCtrl_GetFoldExpanded(wxd_StyledTextCtrl_t* self, int line);

// Word operations
WXD_EXPORTED int wxd_StyledTextCtrl_WordStartPosition(wxd_StyledTextCtrl_t* self, int pos, bool only_word_chars);
WXD_EXPORTED int wxd_StyledTextCtrl_WordEndPosition(wxd_StyledTextCtrl_t* self, int pos, bool only_word_chars);

// Wrap mode operations
WXD_EXPORTED void wxd_StyledTextCtrl_SetWrapMode(wxd_StyledTextCtrl_t* self, int wrap_mode);
WXD_EXPORTED int wxd_StyledTextCtrl_GetWrapMode(wxd_StyledTextCtrl_t* self);

#endif // WXD_STYLEDTEXTCTRL_H 
