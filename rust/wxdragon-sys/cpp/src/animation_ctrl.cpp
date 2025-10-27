#include <wx/wxprec.h>
#include <wx/wx.h>
#include <wx/animate.h>
#include <wx/mstream.h>
#include "wxdragon.h"

// --- wxAnimationCtrl ---

WXD_EXPORTED wxd_AnimationCtrl_t*
wxd_AnimationCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* animation_file, int x, int y,
                         int width, int height, int64_t style, const char* name)
{
    if (!parent) {
        return nullptr;
    }

    wxWindow* wx_parent = (wxWindow*)parent;
    wxString wx_animation_file = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(animation_file);
    wxString wx_name = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(name);

    wxPoint wx_pos = wxPoint(x, y);
    wxSize wx_size = wxSize(width, height);

    // wxAnimationCtrl requires an animation to be loaded at creation or via SetAnimation
    // We'll try to load it here directly if a file is provided.
    // If animation_file is empty or loading fails, an inactive control is created.
    wxAnimation animation;
    if (!wx_animation_file.IsEmpty()) {
        // Note: wxAnimation::LoadFile can return false if loading fails.
        // The wxAnimationCtrl constructor itself doesn't seem to report this failure directly.
        animation.LoadFile(wx_animation_file);
    }

    wxAnimationCtrl* ctrl = new wxAnimationCtrl(wx_parent, id,
                                                animation, // This can be an invalid/empty animation
                                                wx_pos, wx_size, style, wx_name);

    // TODO: Consider if we need to check if `animation` was successfully loaded and if ctrl creation failed.
    // For now, following pattern of returning the pointer.
    return (wxd_AnimationCtrl_t*)ctrl;
}

WXD_EXPORTED bool
wxd_AnimationCtrl_Play(wxd_AnimationCtrl_t* self)
{
    if (!self)
        return false;
    return ((wxAnimationCtrl*)self)->Play();
}

WXD_EXPORTED void
wxd_AnimationCtrl_Stop(wxd_AnimationCtrl_t* self)
{
    if (!self)
        return;
    ((wxAnimationCtrl*)self)->Stop();
}

WXD_EXPORTED bool
wxd_AnimationCtrl_IsPlaying(wxd_AnimationCtrl_t* self)
{
    if (!self)
        return false;
    return ((wxAnimationCtrl*)self)->IsPlaying();
}

WXD_EXPORTED bool
wxd_AnimationCtrl_LoadFile(wxd_AnimationCtrl_t* self, const char* animation_file)
{
    if (!self)
        return false;
    wxString wx_animation_file = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(animation_file);
    if (wx_animation_file.IsEmpty())
        return false; // Cannot load an empty file path

    // wxAnimationCtrl::LoadFile returns bool
    return ((wxAnimationCtrl*)self)->LoadFile(wx_animation_file);
}

WXD_EXPORTED bool
wxd_AnimationCtrl_LoadFromBytes(wxd_AnimationCtrl_t* self, const unsigned char* data, size_t len)
{
    if (!self || !data || len == 0)
        return false;

    wxAnimationCtrl* ctrl = (wxAnimationCtrl*)self;
    wxMemoryInputStream stream(data, len);
    if (!stream.IsOk()) {
        return false; // Failed to create memory stream
    }

    wxAnimation animation;
    if (animation.Load(stream)) {
        ctrl->SetAnimation(animation); // SetAnimation returns void
        return true;                   // Return true if Load and SetAnimation were called
    }
    else {
        // wxAnimation::Load failed
        return false;
    }
}