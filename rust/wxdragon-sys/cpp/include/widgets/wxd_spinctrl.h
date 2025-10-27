#ifndef WXD_SPINCTRL_H
#define WXD_SPINCTRL_H

#include "../wxd_types.h"

// --- SpinCtrl Functions ---
WXD_EXPORTED wxd_SpinCtrl_t*
wxd_SpinCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos,
                    wxd_Size size, wxd_Style_t style, int min_val, int max_val, int initial_val);

WXD_EXPORTED int
wxd_SpinCtrl_GetValue(wxd_SpinCtrl_t* self);

WXD_EXPORTED void
wxd_SpinCtrl_SetValue(wxd_SpinCtrl_t* self, int value);

WXD_EXPORTED void
wxd_SpinCtrl_SetRange(wxd_SpinCtrl_t* self, int minVal, int maxVal);

WXD_EXPORTED int
wxd_SpinCtrl_GetMin(wxd_SpinCtrl_t* self);

WXD_EXPORTED int
wxd_SpinCtrl_GetMax(wxd_SpinCtrl_t* self);

// --- SpinCtrlDouble Functions ---
WXD_EXPORTED wxd_SpinCtrlDouble_t*
wxd_SpinCtrlDouble_Create(wxd_Window_t* parent, int id, const char* value, int x, int y, int w,
                          int h, int64_t style, double min_val, double max_val, double initial_val,
                          double inc);

WXD_EXPORTED double
wxd_SpinCtrlDouble_GetValue(wxd_SpinCtrlDouble_t* self);

WXD_EXPORTED void
wxd_SpinCtrlDouble_SetValue(wxd_SpinCtrlDouble_t* self, double value);

WXD_EXPORTED void
wxd_SpinCtrlDouble_SetRange(wxd_SpinCtrlDouble_t* self, double min_val, double max_val);

WXD_EXPORTED double
wxd_SpinCtrlDouble_GetMin(wxd_SpinCtrlDouble_t* self);

WXD_EXPORTED double
wxd_SpinCtrlDouble_GetMax(wxd_SpinCtrlDouble_t* self);

WXD_EXPORTED void
wxd_SpinCtrlDouble_SetIncrements(wxd_SpinCtrlDouble_t* self, double inc);

WXD_EXPORTED double
wxd_SpinCtrlDouble_GetIncrement(wxd_SpinCtrlDouble_t* self);

WXD_EXPORTED void
wxd_SpinCtrlDouble_SetDigits(wxd_SpinCtrlDouble_t* self, unsigned int digits);

WXD_EXPORTED unsigned int
wxd_SpinCtrlDouble_GetDigits(wxd_SpinCtrlDouble_t* self);

#endif // WXD_SPINCTRL_H