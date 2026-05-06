#ifndef WXD_TEXTCTRL_H
#define WXD_TEXTCTRL_H

#include "../wxd_types.h"

// --- TextCtrl Functions ---
WXD_EXPORTED wxd_TextCtrl_t*
wxd_TextCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos,
                    wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void
wxd_TextCtrl_SetValue(wxd_TextCtrl_t* textCtrl, const char* value);
WXD_EXPORTED int
wxd_TextCtrl_GetValue(wxd_TextCtrl_t* textCtrl, char* buffer, int buffer_len);
WXD_EXPORTED void
wxd_TextCtrl_AppendText(wxd_TextCtrl_t* textCtrl, const char* text);
WXD_EXPORTED void
wxd_TextCtrl_Clear(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED void
wxd_TextCtrl_WriteText(wxd_TextCtrl_t* textCtrl, const char* text);
WXD_EXPORTED void
wxd_TextCtrl_ChangeValue(wxd_TextCtrl_t* textCtrl, const char* value);
WXD_EXPORTED void
wxd_TextCtrl_Remove(wxd_TextCtrl_t* textCtrl, wxd_Long_t from, wxd_Long_t to);
WXD_EXPORTED void
wxd_TextCtrl_Replace(wxd_TextCtrl_t* textCtrl, wxd_Long_t from, wxd_Long_t to, const char* value);
WXD_EXPORTED int
wxd_TextCtrl_GetNumberOfLines(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED int
wxd_TextCtrl_GetLineText(wxd_TextCtrl_t* textCtrl, wxd_Long_t lineNo, char* buffer, int buffer_len);
WXD_EXPORTED bool
wxd_TextCtrl_IsModified(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED void
wxd_TextCtrl_SetModified(wxd_TextCtrl_t* textCtrl, bool modified);
WXD_EXPORTED void
wxd_TextCtrl_SetEditable(wxd_TextCtrl_t* textCtrl, bool editable);
WXD_EXPORTED bool
wxd_TextCtrl_IsEditable(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED wxd_Long_t
wxd_TextCtrl_GetInsertionPoint(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED void
wxd_TextCtrl_SetInsertionPoint(wxd_TextCtrl_t* textCtrl, wxd_Long_t pos);
WXD_EXPORTED void
wxd_TextCtrl_SetMaxLength(wxd_TextCtrl_t* textCtrl, wxd_Long_t len);
WXD_EXPORTED wxd_Long_t
wxd_TextCtrl_GetLastPosition(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED bool
wxd_TextCtrl_IsMultiLine(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED bool
wxd_TextCtrl_IsSingleLine(wxd_TextCtrl_t* textCtrl);

// Selection operations
WXD_EXPORTED void
wxd_TextCtrl_SetSelection(wxd_TextCtrl_t* textCtrl, wxd_Long_t from, wxd_Long_t to);
WXD_EXPORTED void
wxd_TextCtrl_GetSelection(wxd_TextCtrl_t* textCtrl, wxd_Long_t* from, wxd_Long_t* to);
WXD_EXPORTED void
wxd_TextCtrl_SelectAll(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED int
wxd_TextCtrl_GetStringSelection(wxd_TextCtrl_t* textCtrl, char* buffer, int buffer_len);

WXD_EXPORTED void
wxd_TextCtrl_SetInsertionPointEnd(wxd_TextCtrl_t* textCtrl);

// --- TextAttr Functions ---
WXD_EXPORTED wxd_TextAttr_t*
wxd_TextAttr_Create();

WXD_EXPORTED void
wxd_TextAttr_Delete(wxd_TextAttr_t* attr);

WXD_EXPORTED void
wxd_TextAttr_SetTextColour(wxd_TextAttr_t* attr, wxd_Colour_t colText);

WXD_EXPORTED void
wxd_TextAttr_SetBackgroundColour(wxd_TextAttr_t* attr, wxd_Colour_t colBack);

WXD_EXPORTED void
wxd_TextAttr_SetFont(wxd_TextAttr_t* attr, const wxd_Font_t* font);

WXD_EXPORTED void
wxd_TextAttr_SetAlignment(wxd_TextAttr_t* attr, int alignment);

WXD_EXPORTED void
wxd_TextAttr_SetLeftIndent(wxd_TextAttr_t* attr, int indent, int subIndent);

WXD_EXPORTED void
wxd_TextAttr_SetRightIndent(wxd_TextAttr_t* attr, int indent);

WXD_EXPORTED void
wxd_TextAttr_SetLineSpacing(wxd_TextAttr_t* attr, int spacing);

WXD_EXPORTED void
wxd_TextAttr_SetParagraphSpacingAfter(wxd_TextAttr_t* attr, int spacing);

WXD_EXPORTED void
wxd_TextAttr_SetParagraphSpacingBefore(wxd_TextAttr_t* attr, int spacing);

WXD_EXPORTED void
wxd_TextAttr_SetBulletStyle(wxd_TextAttr_t* attr, int style);

// --- TextCtrl Style Functions ---
WXD_EXPORTED void
wxd_TextCtrl_SetStyle(wxd_TextCtrl_t* textCtrl, wxd_Long_t start, wxd_Long_t end, const wxd_TextAttr_t* style);

WXD_EXPORTED wxd_TextAttr_t*
wxd_TextCtrl_GetDefaultStyle(wxd_TextCtrl_t* textCtrl);

WXD_EXPORTED void
wxd_TextCtrl_SetDefaultStyle(wxd_TextCtrl_t* textCtrl, const wxd_TextAttr_t* style);

WXD_EXPORTED bool
wxd_TextCtrl_PositionToXY(wxd_TextCtrl_t* textCtrl, wxd_Long_t pos, wxd_Long_t* x, wxd_Long_t* y);

WXD_EXPORTED wxd_Long_t
wxd_TextCtrl_XYToPosition(wxd_TextCtrl_t* textCtrl, wxd_Long_t x, wxd_Long_t y);

WXD_EXPORTED int
wxd_TextCtrl_GetLineLength(wxd_TextCtrl_t* textCtrl, wxd_Long_t lineNo);

#endif // WXD_TEXTCTRL_H