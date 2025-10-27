/* This is a new file */
#ifndef WXD_ADV_UI_H
#define WXD_ADV_UI_H

#include "../wxdragon.h" // For WXD_EXPORTED, wxd_Window_t, etc.

// --- NotificationMessage ---
WXD_EXPORTED wxd_NotificationMessage_t*
wxd_NotificationMessage_Create(const char* title, const char* message,
                               wxd_Window_t* parent, // Can be NULL
                               int flags // wxICON_INFORMATION, wxICON_WARNING, wxICON_ERROR, or 0
);

WXD_EXPORTED void
wxd_NotificationMessage_Destroy(wxd_NotificationMessage_t* self);

WXD_EXPORTED bool
wxd_NotificationMessage_Show(
    wxd_NotificationMessage_t* self,
    int timeout); // timeout: wxNOTIFY_TIMEOUT_AUTO, wxNOTIFY_TIMEOUT_NEVER, or ms
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

// AddAction function
WXD_EXPORTED bool
wxd_NotificationMessage_AddAction(wxd_NotificationMessage_t* self, wxd_Id actionid,
                                  const char* label);

// Event binding for wxNotificationMessage events (wxEVT_NOTIFICATION_MESSAGE_CLICK, etc.)
// is handled by binding to a parent wxEvtHandler (e.g., the parent window or frame)
// using the standard wxd_EvtHandler_Bind and the WXDEventTypeCEnum values for these events.
// No specific wxd_NotificationMessage_BindClick etc. functions are needed on the notification object itself.

#endif // WXD_ADV_UI_H