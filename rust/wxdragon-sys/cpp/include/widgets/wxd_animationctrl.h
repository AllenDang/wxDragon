#ifndef WXD_ANIMATIONCTRL_H
#define WXD_ANIMATIONCTRL_H

#include "../wxd_types.h"

// --- AnimationCtrl Functions ---
WXD_EXPORTED wxd_AnimationCtrl_t*
wxd_AnimationCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* animation_file, int x, int y,
                         int width, int height, int64_t style, const char* name);
WXD_EXPORTED bool
wxd_AnimationCtrl_Play(wxd_AnimationCtrl_t* self);
WXD_EXPORTED void
wxd_AnimationCtrl_Stop(wxd_AnimationCtrl_t* self);
WXD_EXPORTED bool
wxd_AnimationCtrl_IsPlaying(wxd_AnimationCtrl_t* self);
WXD_EXPORTED bool
wxd_AnimationCtrl_LoadFile(wxd_AnimationCtrl_t* self, const char* animation_file);
WXD_EXPORTED bool
wxd_AnimationCtrl_LoadFromBytes(wxd_AnimationCtrl_t* self, const unsigned char* data, size_t len);

#endif // WXD_ANIMATIONCTRL_H