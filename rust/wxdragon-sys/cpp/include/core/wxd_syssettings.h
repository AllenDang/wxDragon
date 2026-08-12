#ifndef WXD_SYSSETTINGS_H
#define WXD_SYSSETTINGS_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Gets a standard system colour (e.g. the native window background or highlight colour).
WXD_EXPORTED wxd_Colour_t
wxd_SystemSettings_GetColour(wxd_SystemColour index);

// Gets a standard system font. Returns a newly allocated wxd_Font_t* owned by the caller,
// or NULL if the returned font is not valid.
WXD_EXPORTED wxd_Font_t*
wxd_SystemSettings_GetFont(wxd_SystemFont index);

// Gets a system-dependent metric, optionally scaled for the display that `win` is on.
// Pass NULL for `win` to use the default display.
WXD_EXPORTED int
wxd_SystemSettings_GetMetric(wxd_SystemMetric index, wxd_Window_t* win);

#ifdef __cplusplus
}
#endif

#endif // WXD_SYSSETTINGS_H
