#ifndef WXD_CHECKBOX_H
#define WXD_CHECKBOX_H

#include "../wxd_types.h"

// --- CheckBox Functions ---
WXD_EXPORTED wxd_CheckBox_t*
wxd_CheckBox_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos,
                    wxd_Size size, wxd_Style_t style);
WXD_EXPORTED bool
wxd_CheckBox_IsChecked(wxd_CheckBox_t* checkBox);
WXD_EXPORTED void
wxd_CheckBox_SetValue(wxd_CheckBox_t* checkBox, bool value);

#endif // WXD_CHECKBOX_H