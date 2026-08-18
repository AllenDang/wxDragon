#ifndef WXD_INFOBAR_H
#define WXD_INFOBAR_H

#include "../wxdragon.h"

#ifdef __cplusplus
extern "C" {
#endif

WXD_EXPORTED wxd_InfoBar_t*
wxd_InfoBar_Create(wxd_Window_t* parent, wxd_Id id);

WXD_EXPORTED void
wxd_InfoBar_ShowMessage(wxd_InfoBar_t* self, const char* msg, int flags);

WXD_EXPORTED void
wxd_InfoBar_Dismiss(wxd_InfoBar_t* self);

WXD_EXPORTED void
wxd_InfoBar_AddButton(wxd_InfoBar_t* self, wxd_Id btnid, const char* label);

WXD_EXPORTED void
wxd_InfoBar_RemoveButton(wxd_InfoBar_t* self, wxd_Id btnid);

WXD_EXPORTED void
wxd_InfoBar_SetShowHideEffects(wxd_InfoBar_t* self, int showEffect, int hideEffect);

WXD_EXPORTED int
wxd_InfoBar_GetShowEffect(wxd_InfoBar_t* self);

WXD_EXPORTED int
wxd_InfoBar_GetHideEffect(wxd_InfoBar_t* self);

WXD_EXPORTED void
wxd_InfoBar_SetEffectDuration(wxd_InfoBar_t* self, int duration);

WXD_EXPORTED int
wxd_InfoBar_GetEffectDuration(wxd_InfoBar_t* self);

WXD_EXPORTED size_t
wxd_InfoBar_GetButtonCount(wxd_InfoBar_t* self);

WXD_EXPORTED bool
wxd_InfoBar_HasButtonId(wxd_InfoBar_t* self, wxd_Id btnid);

#ifdef __cplusplus
}
#endif

#endif // WXD_INFOBAR_H
