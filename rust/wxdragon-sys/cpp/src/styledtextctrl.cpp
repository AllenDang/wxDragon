#include "wx/wxprec.h"

#ifndef WX_PRECOMP
    #include "wx/wx.h"
#endif

#include "wx/stc/stc.h"
#include "../include/wxdragon.h"
#include "wxd_utils.h"

extern "C" {

// Create a new wxStyledTextCtrl
WXD_EXPORTED wxd_StyledTextCtrl_t* wxd_StyledTextCtrl_Create(
    wxd_Window_t* parent, 
    wxd_Id id, 
    wxd_Point pos, 
    wxd_Size size, 
    wxd_Style_t style
) {
    wxWindow* parentWin = (wxWindow*)parent;
    wxStyledTextCtrl* ctrl = new wxStyledTextCtrl(
        parentWin, 
        id, 
        wxd_cpp_utils::to_wx(pos),
        wxd_cpp_utils::to_wx(size),
        style
    );
    return (wxd_StyledTextCtrl_t*)ctrl;
}

// Text content operations
WXD_EXPORTED void wxd_StyledTextCtrl_SetText(wxd_StyledTextCtrl_t* self, const char* text) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetText(wxString::FromUTF8(text ? text : ""));
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetText(wxd_StyledTextCtrl_t* self, char* buffer, int buffer_len) {
    if (!self || !buffer || buffer_len <= 0) return -1;
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    wxString text = ctrl->GetText();
    return wxd_cpp_utils::copy_wxstring_to_buffer(text, buffer, (size_t)buffer_len);
}

WXD_EXPORTED void wxd_StyledTextCtrl_AppendText(wxd_StyledTextCtrl_t* self, const char* text) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && text) {
        ctrl->AppendText(wxString::FromUTF8(text));
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_InsertText(wxd_StyledTextCtrl_t* self, int pos, const char* text) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && text) {
        ctrl->InsertText(pos, wxString::FromUTF8(text));
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_ClearAll(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->ClearAll();
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_DeleteRange(wxd_StyledTextCtrl_t* self, int start, int length) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->DeleteRange(start, length);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetLength(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->GetLength();
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetLineCount(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->GetLineCount();
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetCharAt(wxd_StyledTextCtrl_t* self, int pos) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->GetCharAt(pos);
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetStyleAt(wxd_StyledTextCtrl_t* self, int pos) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->GetStyleAt(pos);
}

// Clipboard operations
WXD_EXPORTED void wxd_StyledTextCtrl_Cut(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->Cut();
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_Copy(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->Copy();
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_Paste(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->Paste();
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_Undo(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->Undo();
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_SelectAll(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SelectAll();
    }
}

// Read-only state
WXD_EXPORTED void wxd_StyledTextCtrl_SetReadOnly(wxd_StyledTextCtrl_t* self, bool readOnly) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetReadOnly(readOnly);
    }
}

WXD_EXPORTED bool wxd_StyledTextCtrl_GetReadOnly(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return false;
    return ctrl->GetReadOnly();
}

// Position and selection operations
WXD_EXPORTED int wxd_StyledTextCtrl_GetCurrentPos(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->GetCurrentPos();
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetCurrentPos(wxd_StyledTextCtrl_t* self, int pos) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetCurrentPos(pos);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_GetSelection(wxd_StyledTextCtrl_t* self, int* start, int* end) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && start && end) {
        ctrl->GetSelection(start, end);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetSelection(wxd_StyledTextCtrl_t* self, int start, int end) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetSelection(start, end);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetSelectionStart(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->GetSelectionStart();
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetSelectionEnd(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->GetSelectionEnd();
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetSelectedText(wxd_StyledTextCtrl_t* self, char* buffer, int buffer_len) {
    if (!self || !buffer || buffer_len <= 0) return -1;
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    wxString text = ctrl->GetSelectedText();
    return wxd_cpp_utils::copy_wxstring_to_buffer(text, buffer, (size_t)buffer_len);
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetSelectionMode(wxd_StyledTextCtrl_t* self, int selectionMode) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetSelectionMode(selectionMode);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetSelectionMode(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->GetSelectionMode();
}

// Navigation and view operations
WXD_EXPORTED void wxd_StyledTextCtrl_EnsureCaretVisible(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->EnsureCaretVisible();
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_LineScroll(wxd_StyledTextCtrl_t* self, int columns, int lines) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->LineScroll(columns, lines);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_ScrollToLine(wxd_StyledTextCtrl_t* self, int line) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->ScrollToLine(line);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_ScrollToColumn(wxd_StyledTextCtrl_t* self, int column) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->ScrollToColumn(column);
    }
}

// Line operations
WXD_EXPORTED int wxd_StyledTextCtrl_LineFromPosition(wxd_StyledTextCtrl_t* self, int pos) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->LineFromPosition(pos);
}

WXD_EXPORTED int wxd_StyledTextCtrl_PositionFromLine(wxd_StyledTextCtrl_t* self, int line) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->PositionFromLine(line);
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetLineText(wxd_StyledTextCtrl_t* self, int line, char* buffer, int buffer_len) {
    if (!self || !buffer || buffer_len <= 0) return -1;
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    wxString text = ctrl->GetLine(line);
    return wxd_cpp_utils::copy_wxstring_to_buffer(text, buffer, (size_t)buffer_len);
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetLineLength(wxd_StyledTextCtrl_t* self, int line) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->LineLength(line);
}

// Marker operations
WXD_EXPORTED void wxd_StyledTextCtrl_MarkerDefine(wxd_StyledTextCtrl_t* self, int markerNumber, int markerSymbol, wxd_Colour_t foreground, wxd_Colour_t background) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        wxColour fgColor(foreground.r, foreground.g, foreground.b, foreground.a);
        wxColour bgColor(background.r, background.g, background.b, background.a);
        ctrl->MarkerDefine(markerNumber, markerSymbol, fgColor, bgColor);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_MarkerAdd(wxd_StyledTextCtrl_t* self, int line, int markerNumber) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return -1;
    return ctrl->MarkerAdd(line, markerNumber);
}

WXD_EXPORTED void wxd_StyledTextCtrl_MarkerDelete(wxd_StyledTextCtrl_t* self, int line, int markerNumber) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->MarkerDelete(line, markerNumber);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_MarkerDeleteAll(wxd_StyledTextCtrl_t* self, int markerNumber) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->MarkerDeleteAll(markerNumber);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_MarkerGet(wxd_StyledTextCtrl_t* self, int line) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->MarkerGet(line);
}

WXD_EXPORTED int wxd_StyledTextCtrl_MarkerNext(wxd_StyledTextCtrl_t* self, int lineStart, int markerMask) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return -1;
    return ctrl->MarkerNext(lineStart, markerMask);
}

WXD_EXPORTED int wxd_StyledTextCtrl_MarkerPrevious(wxd_StyledTextCtrl_t* self, int lineStart, int markerMask) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return -1;
    return ctrl->MarkerPrevious(lineStart, markerMask);
}

WXD_EXPORTED void wxd_StyledTextCtrl_MarkerSetForeground(wxd_StyledTextCtrl_t* self, int markerNumber, wxd_Colour_t colour) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        wxColour color(colour.r, colour.g, colour.b, colour.a);
        ctrl->MarkerSetForeground(markerNumber, color);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_MarkerSetBackground(wxd_StyledTextCtrl_t* self, int markerNumber, wxd_Colour_t colour) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        wxColour color(colour.r, colour.g, colour.b, colour.a);
        ctrl->MarkerSetBackground(markerNumber, color);
    }
}

// Styling operations
WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetFont(wxd_StyledTextCtrl_t* self, int style, wxd_Font_t* font) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && font) {
        wxFont* font_ptr = (wxFont*)font;
        ctrl->StyleSetFont(style, *font_ptr);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetForeground(wxd_StyledTextCtrl_t* self, int style, wxd_Colour_t colour) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        wxColour wxCol(colour.r, colour.g, colour.b, colour.a);
        ctrl->StyleSetForeground(style, wxCol);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetBackground(wxd_StyledTextCtrl_t* self, int style, wxd_Colour_t colour) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        wxColour wxCol(colour.r, colour.g, colour.b, colour.a);
        ctrl->StyleSetBackground(style, wxCol);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetBold(wxd_StyledTextCtrl_t* self, int style, bool bold) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->StyleSetBold(style, bold);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetItalic(wxd_StyledTextCtrl_t* self, int style, bool italic) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->StyleSetItalic(style, italic);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetUnderline(wxd_StyledTextCtrl_t* self, int style, bool underline) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->StyleSetUnderline(style, underline);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_StyleSetSize(wxd_StyledTextCtrl_t* self, int style, int size) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->StyleSetSize(style, size);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_StyleClearAll(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->StyleClearAll();
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_StartStyling(wxd_StyledTextCtrl_t* self, int start) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->StartStyling(start);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetStyling(wxd_StyledTextCtrl_t* self, int length, int style) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetStyling(length, style);
    }
}

// Lexer and language support
WXD_EXPORTED void wxd_StyledTextCtrl_SetLexer(wxd_StyledTextCtrl_t* self, int lexer) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetLexer(lexer);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetLexer(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->GetLexer();
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetLexerLanguage(wxd_StyledTextCtrl_t* self, const char* language) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && language) {
        ctrl->SetLexerLanguage(wxString::FromUTF8(language));
    }
}

// Margin operations
WXD_EXPORTED void wxd_StyledTextCtrl_SetMarginType(wxd_StyledTextCtrl_t* self, int margin, int margin_type) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetMarginType(margin, margin_type);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetMarginWidth(wxd_StyledTextCtrl_t* self, int margin, int pixel_width) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetMarginWidth(margin, pixel_width);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetMarginLineNumbers(wxd_StyledTextCtrl_t* self, int margin, bool line_numbers) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        if (line_numbers) {
            ctrl->SetMarginType(margin, 1); // 1 = wxSTC_MARGIN_NUMBER
            ctrl->SetMarginMask(margin, 0);
        } else {
            ctrl->SetMarginType(margin, 0); // 0 = wxSTC_MARGIN_SYMBOL
        }
    }
}

// Zoom operations
WXD_EXPORTED void wxd_StyledTextCtrl_ZoomIn(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->ZoomIn();
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_ZoomOut(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->ZoomOut();
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetZoom(wxd_StyledTextCtrl_t* self, int zoom_level) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetZoom(zoom_level);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetZoom(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->GetZoom();
}

// Modified state
WXD_EXPORTED bool wxd_StyledTextCtrl_GetModify(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (!ctrl) return false;
    return ctrl->GetModify();
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetSavePoint(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetSavePoint();
    }
}

// Find and replace
WXD_EXPORTED void wxd_StyledTextCtrl_SearchAnchor(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->SearchAnchor();
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_FindText(wxd_StyledTextCtrl_t* self, int min_pos, int max_pos, const char* text, int flags) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && text) {
        return ctrl->FindText(min_pos, max_pos, wxString::FromUTF8(text), flags);
    }
    return -1;
}

// Additional Find and Replace functions
WXD_EXPORTED int wxd_StyledTextCtrl_SearchNext(wxd_StyledTextCtrl_t* self, int search_flags, const char* text) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && text) {
        return ctrl->SearchNext(search_flags, wxString::FromUTF8(text));
    }
    return -1;
}

WXD_EXPORTED int wxd_StyledTextCtrl_SearchPrev(wxd_StyledTextCtrl_t* self, int search_flags, const char* text) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && text) {
        return ctrl->SearchPrev(search_flags, wxString::FromUTF8(text));
    }
    return -1;
}

WXD_EXPORTED void wxd_StyledTextCtrl_ReplaceSelection(wxd_StyledTextCtrl_t* self, const char* text) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && text) {
        ctrl->ReplaceSelection(wxString::FromUTF8(text));
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_ReplaceTarget(wxd_StyledTextCtrl_t* self, const char* text) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && text) {
        return ctrl->ReplaceTarget(wxString::FromUTF8(text));
    }
    return -1;
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetTargetStart(wxd_StyledTextCtrl_t* self, int start) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetTargetStart(start);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetTargetEnd(wxd_StyledTextCtrl_t* self, int end) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetTargetEnd(end);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetTargetStart(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetTargetStart();
    }
    return -1;
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetTargetEnd(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetTargetEnd();
    }
    return -1;
}

// Navigation operations
WXD_EXPORTED int wxd_StyledTextCtrl_GetCurrentLine(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetCurrentLine();
    }
    return -1;
}

WXD_EXPORTED void wxd_StyledTextCtrl_GotoLine(wxd_StyledTextCtrl_t* self, int line) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->GotoLine(line);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_GotoPos(wxd_StyledTextCtrl_t* self, int pos) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->GotoPos(pos);
    }
}

// Tab and indentation
WXD_EXPORTED void wxd_StyledTextCtrl_SetTabWidth(wxd_StyledTextCtrl_t* self, int tab_width) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetTabWidth(tab_width);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetTabWidth(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetTabWidth();
    }
    return -1;
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetIndent(wxd_StyledTextCtrl_t* self, int indent_size) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetIndent(indent_size);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetIndent(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetIndent();
    }
    return -1;
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetUseTabs(wxd_StyledTextCtrl_t* self, bool use_tabs) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetUseTabs(use_tabs);
    }
}

WXD_EXPORTED bool wxd_StyledTextCtrl_GetUseTabs(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetUseTabs();
    }
    return false;
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetLineIndentation(wxd_StyledTextCtrl_t* self, int line, int indentation) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetLineIndentation(line, indentation);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetLineIndentation(wxd_StyledTextCtrl_t* self, int line) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetLineIndentation(line);
    }
    return -1;
}

// View options
WXD_EXPORTED void wxd_StyledTextCtrl_SetViewEOL(wxd_StyledTextCtrl_t* self, bool visible) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetViewEOL(visible);
    }
}

WXD_EXPORTED bool wxd_StyledTextCtrl_GetViewEOL(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetViewEOL();
    }
    return false;
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetViewWhiteSpace(wxd_StyledTextCtrl_t* self, int view_ws) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetViewWhiteSpace(view_ws);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetViewWhiteSpace(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetViewWhiteSpace();
    }
    return -1;
}

// Caret operations
WXD_EXPORTED void wxd_StyledTextCtrl_SetCaretPeriod(wxd_StyledTextCtrl_t* self, int period_ms) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetCaretPeriod(period_ms);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetCaretPeriod(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetCaretPeriod();
    }
    return -1;
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetCaretWidth(wxd_StyledTextCtrl_t* self, int pixel_width) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetCaretWidth(pixel_width);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetCaretWidth(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetCaretWidth();
    }
    return -1;
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetCaretLineVisible(wxd_StyledTextCtrl_t* self, bool show) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetCaretLineVisible(show);
    }
}

WXD_EXPORTED bool wxd_StyledTextCtrl_GetCaretLineVisible(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetCaretLineVisible();
    }
    return false;
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetCaretLineBackground(wxd_StyledTextCtrl_t* self, wxd_Colour_t back) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        wxColour colour(back.r, back.g, back.b, back.a);
        ctrl->SetCaretLineBackground(colour);
    }
}

// Undo/Redo operations
WXD_EXPORTED void wxd_StyledTextCtrl_Redo(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->Redo();
    }
}

WXD_EXPORTED bool wxd_StyledTextCtrl_CanUndo(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->CanUndo();
    }
    return false;
}

WXD_EXPORTED bool wxd_StyledTextCtrl_CanRedo(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->CanRedo();
    }
    return false;
}

WXD_EXPORTED void wxd_StyledTextCtrl_EmptyUndoBuffer(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->EmptyUndoBuffer();
    }
}

// Autocompletion
WXD_EXPORTED void wxd_StyledTextCtrl_AutoCompShow(wxd_StyledTextCtrl_t* self, int length_entered, const char* item_list) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && item_list) {
        ctrl->AutoCompShow(length_entered, wxString::FromUTF8(item_list));
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_AutoCompCancel(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->AutoCompCancel();
    }
}

WXD_EXPORTED bool wxd_StyledTextCtrl_AutoCompActive(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->AutoCompActive();
    }
    return false;
}

WXD_EXPORTED void wxd_StyledTextCtrl_AutoCompComplete(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->AutoCompComplete();
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_AutoCompSetSeparator(wxd_StyledTextCtrl_t* self, int separator_char) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->AutoCompSetSeparator(separator_char);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_AutoCompSelect(wxd_StyledTextCtrl_t* self, const char* select) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && select) {
        ctrl->AutoCompSelect(wxString::FromUTF8(select));
    }
}

// Bracket matching
WXD_EXPORTED void wxd_StyledTextCtrl_BraceHighlight(wxd_StyledTextCtrl_t* self, int pos_a, int pos_b) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->BraceHighlight(pos_a, pos_b);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_BraceBadLight(wxd_StyledTextCtrl_t* self, int pos) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->BraceBadLight(pos);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_BraceMatch(wxd_StyledTextCtrl_t* self, int pos) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->BraceMatch(pos);
    }
    return -1;
}

// Call tips
WXD_EXPORTED void wxd_StyledTextCtrl_CallTipShow(wxd_StyledTextCtrl_t* self, int pos, const char* definition) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl && definition) {
        ctrl->CallTipShow(pos, wxString::FromUTF8(definition));
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_CallTipCancel(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->CallTipCancel();
    }
}

WXD_EXPORTED bool wxd_StyledTextCtrl_CallTipActive(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->CallTipActive();
    }
    return false;
}

WXD_EXPORTED void wxd_StyledTextCtrl_CallTipSetHighlight(wxd_StyledTextCtrl_t* self, int highlight_start, int highlight_end) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->CallTipSetHighlight(highlight_start, highlight_end);
    }
}

// Folding operations
WXD_EXPORTED void wxd_StyledTextCtrl_SetFoldLevel(wxd_StyledTextCtrl_t* self, int line, int level) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetFoldLevel(line, level);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetFoldLevel(wxd_StyledTextCtrl_t* self, int line) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetFoldLevel(line);
    }
    return -1;
}

WXD_EXPORTED void wxd_StyledTextCtrl_ToggleFold(wxd_StyledTextCtrl_t* self, int line) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->ToggleFold(line);
    }
}

WXD_EXPORTED void wxd_StyledTextCtrl_SetFoldExpanded(wxd_StyledTextCtrl_t* self, int line, bool expanded) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetFoldExpanded(line, expanded);
    }
}

WXD_EXPORTED bool wxd_StyledTextCtrl_GetFoldExpanded(wxd_StyledTextCtrl_t* self, int line) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetFoldExpanded(line);
    }
    return false;
}

// Word operations
WXD_EXPORTED int wxd_StyledTextCtrl_WordStartPosition(wxd_StyledTextCtrl_t* self, int pos, bool only_word_chars) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->WordStartPosition(pos, only_word_chars);
    }
    return -1;
}

WXD_EXPORTED int wxd_StyledTextCtrl_WordEndPosition(wxd_StyledTextCtrl_t* self, int pos, bool only_word_chars) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->WordEndPosition(pos, only_word_chars);
    }
    return -1;
}

// Wrap mode operations
WXD_EXPORTED void wxd_StyledTextCtrl_SetWrapMode(wxd_StyledTextCtrl_t* self, int wrap_mode) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        ctrl->SetWrapMode(wrap_mode);
    }
}

WXD_EXPORTED int wxd_StyledTextCtrl_GetWrapMode(wxd_StyledTextCtrl_t* self) {
    wxStyledTextCtrl* ctrl = (wxStyledTextCtrl*)self;
    if (ctrl) {
        return ctrl->GetWrapMode();
    }
    return 0;
}

} // extern "C" 
