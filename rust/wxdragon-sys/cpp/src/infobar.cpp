#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#if wxUSE_INFOBAR
#include <wx/infobar.h>

extern "C" {

WXD_EXPORTED wxd_InfoBar_t*
wxd_InfoBar_Create(wxd_Window_t* parent, wxd_Id id)
{
	wxWindow* wxParent = parent ? reinterpret_cast<wxWindow*>(parent) : nullptr;
	wxInfoBar* bar = new wxInfoBar(wxParent, id);
	return reinterpret_cast<wxd_InfoBar_t*>(bar);
}

WXD_EXPORTED void
wxd_InfoBar_ShowMessage(wxd_InfoBar_t* self, const char* msg, int flags)
{
	if (!self)
		return;
	wxString wxMsg = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(msg);
	reinterpret_cast<wxInfoBar*>(self)->ShowMessage(wxMsg, flags);
}

WXD_EXPORTED void
wxd_InfoBar_Dismiss(wxd_InfoBar_t* self)
{
	if (!self)
		return;
	reinterpret_cast<wxInfoBar*>(self)->Dismiss();
}

WXD_EXPORTED void
wxd_InfoBar_AddButton(wxd_InfoBar_t* self, wxd_Id btnid, const char* label)
{
	if (!self)
		return;
	wxString wxLabel = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(label);
	reinterpret_cast<wxInfoBar*>(self)->AddButton(btnid, wxLabel);
}

WXD_EXPORTED void
wxd_InfoBar_RemoveButton(wxd_InfoBar_t* self, wxd_Id btnid)
{
	if (!self)
		return;
	reinterpret_cast<wxInfoBar*>(self)->RemoveButton(btnid);
}

WXD_EXPORTED void
wxd_InfoBar_SetShowHideEffects(wxd_InfoBar_t* self, int showEffect, int hideEffect)
{
	if (!self)
		return;
	reinterpret_cast<wxInfoBar*>(self)->SetShowHideEffects(
		static_cast<wxShowEffect>(showEffect),
		static_cast<wxShowEffect>(hideEffect)
	);
}

WXD_EXPORTED int
wxd_InfoBar_GetShowEffect(wxd_InfoBar_t* self)
{
	if (!self)
		return 0;
	return static_cast<int>(reinterpret_cast<wxInfoBar*>(self)->GetShowEffect());
}

WXD_EXPORTED int
wxd_InfoBar_GetHideEffect(wxd_InfoBar_t* self)
{
	if (!self)
		return 0;
	return static_cast<int>(reinterpret_cast<wxInfoBar*>(self)->GetHideEffect());
}

WXD_EXPORTED void
wxd_InfoBar_SetEffectDuration(wxd_InfoBar_t* self, int duration)
{
	if (!self)
		return;
	reinterpret_cast<wxInfoBar*>(self)->SetEffectDuration(duration);
}

WXD_EXPORTED int
wxd_InfoBar_GetEffectDuration(wxd_InfoBar_t* self)
{
	if (!self)
		return 0;
	return reinterpret_cast<wxInfoBar*>(self)->GetEffectDuration();
}

WXD_EXPORTED size_t
wxd_InfoBar_GetButtonCount(wxd_InfoBar_t* self)
{
	if (!self)
		return 0;
	return reinterpret_cast<wxInfoBar*>(self)->GetButtonCount();
}

WXD_EXPORTED bool
wxd_InfoBar_HasButtonId(wxd_InfoBar_t* self, wxd_Id btnid)
{
	if (!self)
		return false;
	return reinterpret_cast<wxInfoBar*>(self)->HasButtonId(btnid);
}

}

#endif // wxUSE_INFOBAR
