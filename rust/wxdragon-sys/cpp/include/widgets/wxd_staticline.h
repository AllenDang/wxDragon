#ifndef WXD_STATICLINE_H
#define WXD_STATICLINE_H

#include "../wxd_types.h"

// --- StaticLine Functions ---
WXD_EXPORTED wxd_StaticLine_t*
wxd_StaticLine_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                      wxd_Style_t style, const char* name);

#endif // WXD_STATICLINE_H