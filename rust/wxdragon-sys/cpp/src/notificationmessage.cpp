#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/notifmsg.h>
#include <wx/generic/notifmsg.h>
#include <wx/bmpbndl.h>
#if wxUSE_TASKBARICON
#include <wx/taskbar.h>
#endif

extern "C" {

WXD_EXPORTED wxd_NotificationMessage_t*
wxd_NotificationMessage_Create(const char* title, const char* message, wxd_Window_t* parent,
                               int flags)
{
	wxString wxTitle = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title);
	wxString wxMessage = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message);
	wxWindow* wxParent = parent ? reinterpret_cast<wxWindow*>(parent) : nullptr;

	wxNotificationMessage* instance =
		new wxNotificationMessage(wxTitle, wxMessage, wxParent, flags);

	return reinterpret_cast<wxd_NotificationMessage_t*>(instance);
}

WXD_EXPORTED wxd_NotificationMessage_t*
wxd_NotificationMessage_CreateGeneric(const char* title, const char* message, wxd_Window_t* parent,
                                      int flags)
{
	wxString wxTitle = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title);
	wxString wxMessage = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message);
	wxWindow* wxParent = parent ? reinterpret_cast<wxWindow*>(parent) : nullptr;

	wxGenericNotificationMessage* instance =
		new wxGenericNotificationMessage(wxTitle, wxMessage, wxParent, flags);

	return reinterpret_cast<wxd_NotificationMessage_t*>(instance);
}

WXD_EXPORTED void
wxd_NotificationMessage_Destroy(wxd_NotificationMessage_t* self)
{
	if (self) {
		delete reinterpret_cast<wxNotificationMessageBase*>(self);
	}
}

WXD_EXPORTED bool
wxd_NotificationMessage_Show(wxd_NotificationMessage_t* self, int timeout)
{
	if (!self)
		return false;
	return reinterpret_cast<wxNotificationMessageBase*>(self)->Show(timeout);
}

WXD_EXPORTED bool
wxd_NotificationMessage_Close(wxd_NotificationMessage_t* self)
{
	if (!self)
		return false;
	return reinterpret_cast<wxNotificationMessageBase*>(self)->Close();
}

WXD_EXPORTED void
wxd_NotificationMessage_SetTitle(wxd_NotificationMessage_t* self, const char* title)
{
	if (!self)
		return;
	wxString wxTitle = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title);
	reinterpret_cast<wxNotificationMessageBase*>(self)->SetTitle(wxTitle);
}

WXD_EXPORTED void
wxd_NotificationMessage_SetMessage(wxd_NotificationMessage_t* self, const char* message)
{
	if (!self)
		return;
	wxString wxMessage = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message);
	reinterpret_cast<wxNotificationMessageBase*>(self)->SetMessage(wxMessage);
}

WXD_EXPORTED void
wxd_NotificationMessage_SetFlags(wxd_NotificationMessage_t* self, int flags)
{
	if (!self)
		return;
	reinterpret_cast<wxNotificationMessageBase*>(self)->SetFlags(flags);
}

WXD_EXPORTED void
wxd_NotificationMessage_SetParent(wxd_NotificationMessage_t* self, wxd_Window_t* parent)
{
	if (!self)
		return;
	wxWindow* wxParent = parent ? reinterpret_cast<wxWindow*>(parent) : nullptr;
	reinterpret_cast<wxNotificationMessageBase*>(self)->SetParent(wxParent);
}

WXD_EXPORTED bool
wxd_NotificationMessage_AddAction(wxd_NotificationMessage_t* self, wxd_Id actionid,
                                  const char* label)
{
	if (!self)
		return false;
	wxString wxLabel = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(label);
	return reinterpret_cast<wxNotificationMessageBase*>(self)->AddAction(actionid, wxLabel);
}

WXD_EXPORTED bool
wxd_NotificationMessage_MSWUseToasts(const char* shortcutPath, const char* appId)
{
#ifdef __WXMSW__
	wxString path = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(shortcutPath);
	wxString id = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(appId);
	return wxNotificationMessage::MSWUseToasts(path, id);
#else
	(void)shortcutPath;
	(void)appId;
	return false;
#endif
}

WXD_EXPORTED void
wxd_NotificationMessage_UseTaskBarIcon(wxd_NotificationMessage_t* self, wxd_TaskBarIcon_t* icon)
{
	if (!self)
		return;
#ifdef __WXMSW__
	// UseTaskBarIcon is only present on the native wxNotificationMessage, not on
	// wxGenericNotificationMessage (an unrelated sibling class with a different object
	// layout). Use dynamic_cast rather than reinterpret_cast so calling this on a
	// generic-backed notification (created via wxd_NotificationMessage_CreateGeneric)
	// is a safe no-op instead of undefined behavior.
	wxTaskBarIcon* tb = icon ? reinterpret_cast<wxTaskBarIcon*>(icon) : nullptr;
	wxNotificationMessage* native =
		dynamic_cast<wxNotificationMessage*>(reinterpret_cast<wxNotificationMessageBase*>(self));
	if (native) {
		native->UseTaskBarIcon(tb);
	}
#else
	(void)icon;
#endif
}

WXD_EXPORTED void
wxd_NotificationMessage_SetIcon(wxd_NotificationMessage_t* self, const wxd_BitmapBundle_t* icon)
{
	if (!self)
		return;
	if (icon) {
		const wxBitmapBundle* bundle = reinterpret_cast<const wxBitmapBundle*>(icon);
		wxIcon icon_obj = bundle->GetIcon(wxDefaultSize);
		reinterpret_cast<wxNotificationMessageBase*>(self)->SetIcon(icon_obj);
	}
}

}