#ifndef WXD_STATICBOX_H
#define WXD_STATICBOX_H

#include "../wxd_types.h"

// --- StaticBox Functions ---
WXD_EXPORTED wxd_StaticBox_t*
wxd_StaticBox_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos,
                     wxd_Size size, wxd_Style_t style);

#endif // WXD_STATICBOX_H