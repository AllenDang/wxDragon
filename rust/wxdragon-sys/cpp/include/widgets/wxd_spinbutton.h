#ifndef WXD_SPINBUTTON_H
#define WXD_SPINBUTTON_H

#include "../wxd_types.h"

// --- SpinButton Functions ---
WXD_EXPORTED wxd_SpinButton_t*
wxd_SpinButton_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                      wxd_Style_t style);

WXD_EXPORTED int
wxd_SpinButton_GetValue(wxd_SpinButton_t* self);

WXD_EXPORTED void
wxd_SpinButton_SetValue(wxd_SpinButton_t* self, int value);

WXD_EXPORTED void
wxd_SpinButton_SetRange(wxd_SpinButton_t* self, int minVal, int maxVal);

WXD_EXPORTED int
wxd_SpinButton_GetMin(wxd_SpinButton_t* self);

WXD_EXPORTED int
wxd_SpinButton_GetMax(wxd_SpinButton_t* self);

#endif // WXD_SPINBUTTON_H