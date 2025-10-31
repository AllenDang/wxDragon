#ifndef WXD_TOGGLEBUTTON_H
#define WXD_TOGGLEBUTTON_H

#include "../wxd_types.h"

// --- ToggleButton Functions ---
WXD_EXPORTED wxd_ToggleButton_t*
wxd_ToggleButton_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos,
                        wxd_Size size, wxd_Style_t style);

WXD_EXPORTED bool
wxd_ToggleButton_GetValue(wxd_ToggleButton_t* tglbtn);

WXD_EXPORTED void
wxd_ToggleButton_SetValue(wxd_ToggleButton_t* tglbtn, bool state);

WXD_EXPORTED void
wxd_ToggleButton_SetLabel(wxd_ToggleButton_t* tglbtn, const char* label);

WXD_EXPORTED int
wxd_ToggleButton_GetLabel(const wxd_ToggleButton_t* tglbtn, char* buffer, size_t buffer_len);

#endif // WXD_TOGGLEBUTTON_H