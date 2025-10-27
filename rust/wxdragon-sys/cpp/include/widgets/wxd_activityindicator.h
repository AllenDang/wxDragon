#ifndef WXD_ACTIVITYINDICATOR_H
#define WXD_ACTIVITYINDICATOR_H

#include "../wxd_types.h"

// --- ActivityIndicator Functions ---
WXD_EXPORTED wxd_ActivityIndicator_t*
wxd_ActivityIndicator_Create(wxd_Window_t* parent, int id, int x, int y, int w, int h,
                             int64_t style);
WXD_EXPORTED void
wxd_ActivityIndicator_Start(wxd_ActivityIndicator_t* self);
WXD_EXPORTED void
wxd_ActivityIndicator_Stop(wxd_ActivityIndicator_t* self);
WXD_EXPORTED bool
wxd_ActivityIndicator_IsRunning(wxd_ActivityIndicator_t* self);

#endif // WXD_ACTIVITYINDICATOR_H