#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/notifmsg.h>         // Required for wxNotificationMessage
#include <wx/generic/notifmsg.h> // Required for wxGenericNotificationMessage

// --- wxNotificationMessage ---

WXD_EXPORTED wxd_NotificationMessage_t*
wxd_NotificationMessage_Create(const char* title, const char* message, wxd_Window_t* parent,
                               int flags)
{
    wxString wxTitle = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title);
    wxString wxMessage = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message);
    wxWindow* wxParent = parent ? reinterpret_cast<wxWindow*>(parent) : nullptr;

#ifdef __WXMSW__
    // Use generic notifications on Windows to avoid taskbar icon issues
    // that cause app hanging and persistent icons
    wxGenericNotificationMessage* instance =
        new wxGenericNotificationMessage(wxTitle, wxMessage, wxParent, flags);
#else
    // Use native notifications on other platforms
    wxNotificationMessage* instance =
        new wxNotificationMessage(wxTitle, wxMessage, wxParent, flags);
#endif

    return reinterpret_cast<wxd_NotificationMessage_t*>(instance);
}

WXD_EXPORTED void
wxd_NotificationMessage_Destroy(wxd_NotificationMessage_t* self)
{
    if (self) {
        // wxNotificationMessage is not a wxWindow, so it needs to be deleted directly.
        // It does not have a Destroy() method that wxWindow objects use for deferred deletion.
        delete reinterpret_cast<wxNotificationMessage*>(self);
    }
}

WXD_EXPORTED bool
wxd_NotificationMessage_Show(wxd_NotificationMessage_t* self, int timeout)
{
    if (!self)
        return false;
    // wxNotificationMessage::Show can take wxNotificationMessage::Timeout_Auto or wxNotificationMessage::Timeout_Never
    // or a duration in seconds.
    return reinterpret_cast<wxNotificationMessage*>(self)->Show(timeout);
}

WXD_EXPORTED bool
wxd_NotificationMessage_Close(wxd_NotificationMessage_t* self)
{
    if (!self)
        return false;
    return reinterpret_cast<wxNotificationMessage*>(self)->Close();
}

WXD_EXPORTED void
wxd_NotificationMessage_SetTitle(wxd_NotificationMessage_t* self, const char* title)
{
    if (!self)
        return;
    wxString wxTitle = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title);
    reinterpret_cast<wxNotificationMessage*>(self)->SetTitle(wxTitle);
}

WXD_EXPORTED void
wxd_NotificationMessage_SetMessage(wxd_NotificationMessage_t* self, const char* message)
{
    if (!self)
        return;
    wxString wxMessage = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message);
    reinterpret_cast<wxNotificationMessage*>(self)->SetMessage(wxMessage);
}

WXD_EXPORTED void
wxd_NotificationMessage_SetFlags(wxd_NotificationMessage_t* self, int flags)
{
    if (!self)
        return;
    reinterpret_cast<wxNotificationMessage*>(self)->SetFlags(flags);
}

WXD_EXPORTED void
wxd_NotificationMessage_SetParent(wxd_NotificationMessage_t* self, wxd_Window_t* parent)
{
    if (!self)
        return;
    wxWindow* wxParent = parent ? reinterpret_cast<wxWindow*>(parent) : nullptr;
    reinterpret_cast<wxNotificationMessage*>(self)->SetParent(wxParent);
}

WXD_EXPORTED bool
wxd_NotificationMessage_AddAction(wxd_NotificationMessage_t* self, wxd_Id actionid,
                                  const char* label)
{
    if (!self)
        return false;
    wxString wxLabel = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(label);
    return reinterpret_cast<wxNotificationMessage*>(self)->AddAction(actionid, wxLabel);
}

// Event binding for wxNotificationMessage: Events like wxEVT_NOTIFICATION_MESSAGE_CLICK,
// wxEVT_NOTIFICATION_MESSAGE_DISMISSED, and wxEVT_NOTIFICATION_MESSAGE_ACTION are command events.
// They are typically handled by a wxEvtHandler (e.g., a parent window) that binds to these event types.
// The wxd_EvtHandler_Bind function in event.cpp, along with the corresponding WXDEventTypeCEnum values,
// is used for this purpose. No specific binding functions on wxNotificationMessage itself are needed.
// The AddAction C API function has been implemented above.