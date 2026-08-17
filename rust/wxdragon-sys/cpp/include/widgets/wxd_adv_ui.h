#ifndef WXD_ADV_UI_H
#define WXD_ADV_UI_H

#include "../wxdragon.h"

#ifdef __cplusplus
extern "C" {
#endif

WXD_EXPORTED wxd_NotificationMessage_t*
wxd_NotificationMessage_Create(const char* title, const char* message,
                               wxd_Window_t* parent,
                               int flags);

WXD_EXPORTED wxd_NotificationMessage_t*
wxd_NotificationMessage_CreateGeneric(const char* title, const char* message,
                                      wxd_Window_t* parent,
                                      int flags);

WXD_EXPORTED void
wxd_NotificationMessage_Destroy(wxd_NotificationMessage_t* self);

WXD_EXPORTED bool
wxd_NotificationMessage_Show(wxd_NotificationMessage_t* self, int timeout);

WXD_EXPORTED bool
wxd_NotificationMessage_Close(wxd_NotificationMessage_t* self);

WXD_EXPORTED void
wxd_NotificationMessage_SetTitle(wxd_NotificationMessage_t* self, const char* title);

WXD_EXPORTED void
wxd_NotificationMessage_SetMessage(wxd_NotificationMessage_t* self, const char* message);

WXD_EXPORTED void
wxd_NotificationMessage_SetFlags(wxd_NotificationMessage_t* self, int flags);

WXD_EXPORTED void
wxd_NotificationMessage_SetParent(wxd_NotificationMessage_t* self, wxd_Window_t* parent);

WXD_EXPORTED bool
wxd_NotificationMessage_AddAction(wxd_NotificationMessage_t* self, wxd_Id actionid,
                                  const char* label);

WXD_EXPORTED bool
wxd_NotificationMessage_MSWUseToasts(const char* shortcutPath, const char* appId);

WXD_EXPORTED void
wxd_NotificationMessage_UseTaskBarIcon(wxd_NotificationMessage_t* self, wxd_TaskBarIcon_t* icon);

WXD_EXPORTED void
wxd_NotificationMessage_SetIcon(wxd_NotificationMessage_t* self, const wxd_BitmapBundle_t* icon);

#ifdef __cplusplus
}
#endif

#endif // WXD_ADV_UI_H