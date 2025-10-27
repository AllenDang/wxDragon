#include <wx/wxprec.h>
#include <wx/wx.h>
#include <wx/mediactrl.h>
#include <wx/uri.h>
#include "../include/wxdragon.h"

extern "C" {

WXD_EXPORTED wxd_MediaCtrl_t*
wxd_MediaCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* fileName, int x, int y, int width,
                     int height, int64_t style, const char* backend_name)
{
    if (!parent) {
        return nullptr;
    }

    wxWindow* wx_parent = (wxWindow*)parent;
    wxString wx_fileName = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(fileName);
    wxString wx_backend_name = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(backend_name);

    wxPoint wx_pos = wxPoint(x, y);
    wxSize wx_size = wxSize(width, height);

    wxMediaCtrl* ctrl =
        new wxMediaCtrl(wx_parent, id, wx_fileName, wx_pos, wx_size, style, wx_backend_name);

    return (wxd_MediaCtrl_t*)ctrl;
}

WXD_EXPORTED bool
wxd_MediaCtrl_Play(wxd_MediaCtrl_t* self)
{
    if (!self)
        return false;
    return ((wxMediaCtrl*)self)->Play();
}

WXD_EXPORTED bool
wxd_MediaCtrl_Pause(wxd_MediaCtrl_t* self)
{
    if (!self)
        return false;
    return ((wxMediaCtrl*)self)->Pause();
}

WXD_EXPORTED bool
wxd_MediaCtrl_Stop(wxd_MediaCtrl_t* self)
{
    if (!self)
        return false;
    return ((wxMediaCtrl*)self)->Stop();
}

WXD_EXPORTED bool
wxd_MediaCtrl_Load(wxd_MediaCtrl_t* self, const char* fileName)
{
    if (!self)
        return false;
    wxString wx_fileName = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(fileName);
    return ((wxMediaCtrl*)self)->Load(wx_fileName);
}

WXD_EXPORTED bool
wxd_MediaCtrl_LoadURI(wxd_MediaCtrl_t* self, const char* uri)
{
    if (!self)
        return false;
    wxString wx_uri = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(uri);
    wxURI wxuri(wx_uri);
    return ((wxMediaCtrl*)self)->Load(wxuri);
}

WXD_EXPORTED bool
wxd_MediaCtrl_LoadURIWithProxy(wxd_MediaCtrl_t* self, const char* uri, const char* proxy)
{
    if (!self)
        return false;
    wxString wx_uri = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(uri);
    wxString wx_proxy = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(proxy);
    wxURI wxuri(wx_uri);
    wxURI wxproxy(wx_proxy);
    return ((wxMediaCtrl*)self)->Load(wxuri, wxproxy);
}

WXD_EXPORTED wxd_MediaState
wxd_MediaCtrl_GetState(wxd_MediaCtrl_t* self)
{
    if (!self)
        return WXD_MEDIASTATE_STOPPED;

    wxMediaState state = ((wxMediaCtrl*)self)->GetState();
    switch (state) {
    case wxMEDIASTATE_PLAYING:
        return WXD_MEDIASTATE_PLAYING;
    case wxMEDIASTATE_PAUSED:
        return WXD_MEDIASTATE_PAUSED;
    case wxMEDIASTATE_STOPPED:
    default:
        return WXD_MEDIASTATE_STOPPED;
    }
}

WXD_EXPORTED wxd_Long_t
wxd_MediaCtrl_Seek(wxd_MediaCtrl_t* self, wxd_Long_t where, int mode)
{
    if (!self)
        return 0;
    return ((wxMediaCtrl*)self)->Seek(where, (wxSeekMode)mode);
}

WXD_EXPORTED wxd_Long_t
wxd_MediaCtrl_Tell(wxd_MediaCtrl_t* self)
{
    if (!self)
        return 0;
    return ((wxMediaCtrl*)self)->Tell();
}

WXD_EXPORTED wxd_Long_t
wxd_MediaCtrl_Length(wxd_MediaCtrl_t* self)
{
    if (!self)
        return 0;
    return ((wxMediaCtrl*)self)->Length();
}

WXD_EXPORTED double
wxd_MediaCtrl_GetPlaybackRate(wxd_MediaCtrl_t* self)
{
    if (!self)
        return 1.0;
    return ((wxMediaCtrl*)self)->GetPlaybackRate();
}

WXD_EXPORTED bool
wxd_MediaCtrl_SetPlaybackRate(wxd_MediaCtrl_t* self, double dRate)
{
    if (!self)
        return false;
    return ((wxMediaCtrl*)self)->SetPlaybackRate(dRate);
}

WXD_EXPORTED wxd_Long_t
wxd_MediaCtrl_GetDownloadProgress(wxd_MediaCtrl_t* self)
{
    if (!self)
        return 0;
    return ((wxMediaCtrl*)self)->GetDownloadProgress();
}

WXD_EXPORTED wxd_Long_t
wxd_MediaCtrl_GetDownloadTotal(wxd_MediaCtrl_t* self)
{
    if (!self)
        return 0;
    return ((wxMediaCtrl*)self)->GetDownloadTotal();
}

WXD_EXPORTED double
wxd_MediaCtrl_GetVolume(wxd_MediaCtrl_t* self)
{
    if (!self)
        return 0.0;
    return ((wxMediaCtrl*)self)->GetVolume();
}

WXD_EXPORTED bool
wxd_MediaCtrl_SetVolume(wxd_MediaCtrl_t* self, double volume)
{
    if (!self)
        return false;
    return ((wxMediaCtrl*)self)->SetVolume(volume);
}

WXD_EXPORTED bool
wxd_MediaCtrl_ShowPlayerControls(wxd_MediaCtrl_t* self, int flags)
{
    if (!self)
        return false;
    return ((wxMediaCtrl*)self)->ShowPlayerControls((wxMediaCtrlPlayerControls)flags);
}

} // extern "C"