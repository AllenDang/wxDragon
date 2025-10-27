#ifndef WXD_GAUGE_H
#define WXD_GAUGE_H

#include "../wxd_types.h"

// --- Gauge Functions ---
WXD_EXPORTED wxd_Gauge_t*
wxd_Gauge_Create(wxd_Window_t* parent, wxd_Id id, int range, int x, int y, int w, int h,
                 wxd_Style_t style);
WXD_EXPORTED void
wxd_Gauge_SetRange(wxd_Gauge_t* self, int range);
WXD_EXPORTED void
wxd_Gauge_SetValue(wxd_Gauge_t* self, int value);
WXD_EXPORTED int
wxd_Gauge_GetValue(const wxd_Gauge_t* self);

#endif // WXD_GAUGE_H