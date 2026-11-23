#ifndef WXD_SLIDER_H
#define WXD_SLIDER_H

#include "../wxd_types.h"

// The other wxSlider functions are generated from cpp/spec/slider.wxd.

// Makes Up, Right and Page Up raise the value and Down, Left and Page Down lower it, on every
// platform. The native Windows trackbar lowers it for Up and Page Up.
WXD_EXPORTED void
wxd_Slider_UseNaturalKeys(wxd_Slider_t* self);

#endif // WXD_SLIDER_H
