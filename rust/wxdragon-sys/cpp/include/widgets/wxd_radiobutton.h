#ifndef WXD_RADIOBUTTON_H
#define WXD_RADIOBUTTON_H

#include "../wxd_types.h"

// --- RadioButton Functions ---
WXD_EXPORTED wxd_RadioButton_t*
wxd_RadioButton_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos,
                       wxd_Size size, wxd_Style_t style);

WXD_EXPORTED bool
wxd_RadioButton_GetValue(wxd_RadioButton_t* radio);

WXD_EXPORTED void
wxd_RadioButton_SetValue(wxd_RadioButton_t* radio, bool value);

#endif // WXD_RADIOBUTTON_H