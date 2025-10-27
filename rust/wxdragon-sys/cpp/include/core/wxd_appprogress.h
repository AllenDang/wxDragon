#ifndef WXD_APPPROGRESS_H
#define WXD_APPPROGRESS_H

#include "../wxd_types.h"

// Create a new wxAppProgressIndicator
WXD_EXPORTED wxd_AppProgressIndicator_t*
wxd_AppProgressIndicator_Create(wxd_Window_t* parent);

// Destroy/delete a wxAppProgressIndicator
WXD_EXPORTED void
wxd_AppProgressIndicator_Destroy(wxd_AppProgressIndicator_t* self);

// Check if the application progress display is available.
WXD_EXPORTED bool
wxd_AppProgressIndicator_IsAvailable(wxd_AppProgressIndicator_t* self);

// Set the progress value in taskbar button of parent window.
WXD_EXPORTED void
wxd_AppProgressIndicator_SetValue(wxd_AppProgressIndicator_t* self, int value);

// Set the progress range in taskbar button of parent window.
WXD_EXPORTED void
wxd_AppProgressIndicator_SetRange(wxd_AppProgressIndicator_t* self, int range);

// Makes the progress bar run in indeterminate mode.
WXD_EXPORTED void
wxd_AppProgressIndicator_Pulse(wxd_AppProgressIndicator_t* self);

#endif
