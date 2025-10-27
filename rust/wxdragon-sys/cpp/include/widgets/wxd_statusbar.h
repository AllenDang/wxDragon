#ifndef WXD_STATUSBAR_H
#define WXD_STATUSBAR_H

#include "../wxd_types.h"

// --- StatusBar Functions ---
WXD_EXPORTED wxd_StatusBar_t*
wxd_StatusBar_Create(wxd_Window_t* parent, wxd_Id id, wxd_Style_t style);

WXD_EXPORTED void
wxd_StatusBar_SetFieldsCount(wxd_StatusBar_t* self, int count);

WXD_EXPORTED void
wxd_StatusBar_SetStatusText(wxd_StatusBar_t* self, const char* text, int fieldIndex);

WXD_EXPORTED void
wxd_StatusBar_SetStatusWidths(wxd_StatusBar_t* self, int count, const int* widths);

WXD_EXPORTED void
wxd_StatusBar_PushStatusText(wxd_StatusBar_t* self, const char* text, int fieldIndex);

WXD_EXPORTED void
wxd_StatusBar_PopStatusText(wxd_StatusBar_t* self, int fieldIndex);

#endif // WXD_STATUSBAR_H