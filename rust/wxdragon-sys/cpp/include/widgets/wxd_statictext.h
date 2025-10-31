#ifndef WXD_STATICTEXT_H
#define WXD_STATICTEXT_H

#include "../wxd_types.h"

// --- StaticText Functions ---
WXD_EXPORTED wxd_StaticText_t*
wxd_StaticText_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos,
                      wxd_Size size, wxd_Style_t style);

WXD_EXPORTED void
wxd_StaticText_Destroy(wxd_StaticText_t* stext); // Generic might suffice

WXD_EXPORTED void
wxd_StaticText_SetLabel(wxd_StaticText_t* stext, const char* label);

/**
 * Gets the label of the static text control.
 * Returns the length of the label string (not including the null terminator) or -1 if stext is null.
 * If buffer is not null and buffer_len is non-zero, copies up to buffer_len-1 characters into buffer,
 * null-terminating it.
 */
WXD_EXPORTED int
wxd_StaticText_GetLabel(const wxd_StaticText_t* stext, char* buffer, size_t buffer_len);

WXD_EXPORTED void
wxd_StaticText_Wrap(wxd_StaticText_t* stext, int width);

#endif // WXD_STATICTEXT_H