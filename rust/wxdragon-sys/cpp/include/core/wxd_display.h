#ifndef WXD_DISPLAY_H
#define WXD_DISPLAY_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Displays are numbered from 0 to wxd_Display_GetCount() - 1. All queries below
// are safe to call with an out-of-range index; they behave as documented for an
// invalid/disconnected display (false / zeroed rect / empty name, etc).

// Returns the number of connected displays.
WXD_EXPORTED unsigned int
wxd_Display_GetCount(void);

// Finds the display containing the given point, or -1 if it doesn't belong to any display.
WXD_EXPORTED int
wxd_Display_GetFromPoint(wxd_Point pt);

// Finds the display with the biggest intersection with the given rectangle,
// or -1 if the rectangle doesn't intersect any display.
WXD_EXPORTED int
wxd_Display_GetFromRect(wxd_Rect rect);

// Finds the display containing the given window, or -1 if the window isn't shown.
WXD_EXPORTED int
wxd_Display_GetFromWindow(wxd_Window_t* window);

// Returns true if `index` refers to a valid, currently connected display.
WXD_EXPORTED bool
wxd_Display_IsOk(unsigned int index);

// Returns the full geometry (position and size) of the display in screen coordinates.
WXD_EXPORTED wxd_Rect
wxd_Display_GetGeometry(unsigned int index);

// Returns the usable client area of the display, excluding taskbars/docks and similar chrome.
WXD_EXPORTED wxd_Rect
wxd_Display_GetClientArea(unsigned int index);

// Returns the colour depth in bits per pixel, or 0 if unknown.
WXD_EXPORTED int
wxd_Display_GetDepth(unsigned int index);

// Returns the display's resolution in pixels per inch.
WXD_EXPORTED wxd_Size
wxd_Display_GetPPI(unsigned int index);

// Returns the display's content scale factor (e.g. 2.0 for a 200% scaled display).
WXD_EXPORTED double
wxd_Display_GetScaleFactor(unsigned int index);

// Gets the display's name into `buffer` (may be empty). Returns the required buffer
// length in bytes (excluding the null terminator), following the same call-twice
// pattern as the other wxd string getters: call once with a null/zero-length buffer
// to get the required size, then again with a large-enough buffer.
WXD_EXPORTED int
wxd_Display_GetName(unsigned int index, char* buffer, size_t buffer_len);

// Returns true if `index` is the primary display (this is usually, but not always, display 0).
WXD_EXPORTED bool
wxd_Display_IsPrimary(unsigned int index);

// Clears any cached display information. Call this after receiving a display
// connected/disconnected notification from the OS.
WXD_EXPORTED void
wxd_Display_InvalidateCache(void);

#ifdef __cplusplus
}
#endif

#endif // WXD_DISPLAY_H
