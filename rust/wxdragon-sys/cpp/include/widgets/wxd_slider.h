#ifndef WXD_SLIDER_H
#define WXD_SLIDER_H

#include "../wxd_types.h"

// --- Slider Functions ---
WXD_EXPORTED wxd_Slider_t*
wxd_Slider_Create(wxd_Window_t* parent, wxd_Id id, int value, int minValue, int maxValue,
                  wxd_Point pos, wxd_Size size, wxd_Style_t style);

WXD_EXPORTED int
wxd_Slider_GetValue(wxd_Slider_t* self);

WXD_EXPORTED void
wxd_Slider_SetValue(wxd_Slider_t* self, int value);

WXD_EXPORTED void
wxd_Slider_SetRange(wxd_Slider_t* self, int minValue, int maxValue);

WXD_EXPORTED int
wxd_Slider_GetMin(wxd_Slider_t* self);

WXD_EXPORTED int
wxd_Slider_GetMax(wxd_Slider_t* self);

#endif // WXD_SLIDER_H