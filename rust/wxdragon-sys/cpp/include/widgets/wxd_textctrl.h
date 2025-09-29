#ifndef WXD_TEXTCTRL_H
#define WXD_TEXTCTRL_H

#include "../wxd_types.h"

// --- TextCtrl Functions ---
WXD_EXPORTED wxd_TextCtrl_t* wxd_TextCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_TextCtrl_SetValue(wxd_TextCtrl_t* textCtrl, const char* value);
WXD_EXPORTED int wxd_TextCtrl_GetValue(wxd_TextCtrl_t* textCtrl, char* buffer, int buffer_len);
WXD_EXPORTED void wxd_TextCtrl_AppendText(wxd_TextCtrl_t* textCtrl, const char* text);
WXD_EXPORTED void wxd_TextCtrl_Clear(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED bool wxd_TextCtrl_IsModified(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED void wxd_TextCtrl_SetModified(wxd_TextCtrl_t* textCtrl, bool modified);
WXD_EXPORTED void wxd_TextCtrl_SetEditable(wxd_TextCtrl_t* textCtrl, bool editable);
WXD_EXPORTED bool wxd_TextCtrl_IsEditable(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED wxd_Long_t wxd_TextCtrl_GetInsertionPoint(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED void wxd_TextCtrl_SetInsertionPoint(wxd_TextCtrl_t* textCtrl, wxd_Long_t pos);
WXD_EXPORTED void wxd_TextCtrl_SetMaxLength(wxd_TextCtrl_t* textCtrl, wxd_Long_t len);
WXD_EXPORTED wxd_Long_t wxd_TextCtrl_GetLastPosition(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED bool wxd_TextCtrl_IsMultiLine(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED bool wxd_TextCtrl_IsSingleLine(wxd_TextCtrl_t* textCtrl);

// Selection operations
WXD_EXPORTED void wxd_TextCtrl_SetSelection(wxd_TextCtrl_t* textCtrl, wxd_Long_t from, wxd_Long_t to);
WXD_EXPORTED void wxd_TextCtrl_GetSelection(wxd_TextCtrl_t* textCtrl, wxd_Long_t* from, wxd_Long_t* to);
WXD_EXPORTED void wxd_TextCtrl_SelectAll(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED int wxd_TextCtrl_GetStringSelection(wxd_TextCtrl_t* textCtrl, char* buffer, int buffer_len);

#endif // WXD_TEXTCTRL_H 