#ifndef WXD_MEDIACTRL_H
#define WXD_MEDIACTRL_H

#include "../wxd_types.h"

// Media state enum (will be exposed through Rust enums)
typedef enum {
    WXD_MEDIASTATE_STOPPED = 0,
    WXD_MEDIASTATE_PAUSED = 1,
    WXD_MEDIASTATE_PLAYING = 2
} wxd_MediaState;

// --- MediaCtrl Functions ---
WXD_EXPORTED wxd_MediaCtrl_t*
wxd_MediaCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* fileName, int x, int y, int width,
                     int height, int64_t style, const char* backend_name);

WXD_EXPORTED bool
wxd_MediaCtrl_Play(wxd_MediaCtrl_t* self);
WXD_EXPORTED bool
wxd_MediaCtrl_Pause(wxd_MediaCtrl_t* self);
WXD_EXPORTED bool
wxd_MediaCtrl_Stop(wxd_MediaCtrl_t* self);

WXD_EXPORTED bool
wxd_MediaCtrl_Load(wxd_MediaCtrl_t* self, const char* fileName);
WXD_EXPORTED bool
wxd_MediaCtrl_LoadURI(wxd_MediaCtrl_t* self, const char* uri);
WXD_EXPORTED bool
wxd_MediaCtrl_LoadURIWithProxy(wxd_MediaCtrl_t* self, const char* uri, const char* proxy);

WXD_EXPORTED wxd_MediaState
wxd_MediaCtrl_GetState(wxd_MediaCtrl_t* self);

WXD_EXPORTED wxd_Long_t
wxd_MediaCtrl_Seek(wxd_MediaCtrl_t* self, wxd_Long_t where, int mode);
WXD_EXPORTED wxd_Long_t
wxd_MediaCtrl_Tell(wxd_MediaCtrl_t* self);
WXD_EXPORTED wxd_Long_t
wxd_MediaCtrl_Length(wxd_MediaCtrl_t* self);

WXD_EXPORTED double
wxd_MediaCtrl_GetPlaybackRate(wxd_MediaCtrl_t* self);
WXD_EXPORTED bool
wxd_MediaCtrl_SetPlaybackRate(wxd_MediaCtrl_t* self, double dRate);

WXD_EXPORTED wxd_Long_t
wxd_MediaCtrl_GetDownloadProgress(wxd_MediaCtrl_t* self);
WXD_EXPORTED wxd_Long_t
wxd_MediaCtrl_GetDownloadTotal(wxd_MediaCtrl_t* self);

WXD_EXPORTED double
wxd_MediaCtrl_GetVolume(wxd_MediaCtrl_t* self);
WXD_EXPORTED bool
wxd_MediaCtrl_SetVolume(wxd_MediaCtrl_t* self, double volume);

WXD_EXPORTED bool
wxd_MediaCtrl_ShowPlayerControls(wxd_MediaCtrl_t* self, int flags);

#endif // WXD_MEDIACTRL_H