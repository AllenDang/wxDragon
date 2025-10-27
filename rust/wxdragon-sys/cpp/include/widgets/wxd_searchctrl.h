#ifndef WXD_SEARCHCTRL_H
#define WXD_SEARCHCTRL_H

#include "../wxd_types.h"

// --- SearchCtrl Functions ---
WXD_EXPORTED wxd_SearchCtrl_t*
wxd_SearchCtrl_Create(wxd_Window_t* parent, int id, const char* value, int x, int y, int w, int h,
                      int64_t style);

WXD_EXPORTED void
wxd_SearchCtrl_ShowSearchButton(wxd_SearchCtrl_t* self, bool show);

WXD_EXPORTED bool
wxd_SearchCtrl_IsSearchButtonVisible(wxd_SearchCtrl_t* self);

WXD_EXPORTED void
wxd_SearchCtrl_ShowCancelButton(wxd_SearchCtrl_t* self, bool show);

WXD_EXPORTED bool
wxd_SearchCtrl_IsCancelButtonVisible(wxd_SearchCtrl_t* self);

WXD_EXPORTED wxd_Control_t*
wxd_SearchCtrl_GetCancelButton(wxd_SearchCtrl_t* self);

WXD_EXPORTED void
wxd_SearchCtrl_SetMenu(wxd_SearchCtrl_t* self, wxd_Menu_t* menu);

WXD_EXPORTED wxd_Menu_t*
wxd_SearchCtrl_GetMenu(wxd_SearchCtrl_t* self);

#endif // WXD_SEARCHCTRL_H