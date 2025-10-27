#ifndef WXD_SCROLLBAR_H
#define WXD_SCROLLBAR_H

#include "../wxd_types.h"

// --- ScrollBar Functions ---
WXD_EXPORTED wxd_ScrollBar_t*
wxd_ScrollBar_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                     wxd_Style_t style, const char* name);
WXD_EXPORTED void
wxd_ScrollBar_SetScrollbar(wxd_ScrollBar_t* self, int position, int thumbSize, int range,
                           int pageSize, bool refresh);
WXD_EXPORTED int
wxd_ScrollBar_GetThumbPosition(wxd_ScrollBar_t* self);

#endif // WXD_SCROLLBAR_H