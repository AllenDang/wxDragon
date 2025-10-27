#ifndef WXD_PANEL_H
#define WXD_PANEL_H

#include "../wxd_types.h"

// --- Panel Functions ---
WXD_EXPORTED wxd_Panel_t*
wxd_Panel_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);

#endif // WXD_PANEL_H