#ifndef WXD_SOUND_H
#define WXD_SOUND_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Sound play flags
typedef enum {
    WXD_SOUND_SYNC = 0x0000,
    WXD_SOUND_ASYNC = 0x0001,
    WXD_SOUND_LOOP = 0x0002
} wxd_SoundFlags;

// --- Sound Functions ---

WXD_EXPORTED wxd_Sound_t*
wxd_Sound_Create(const char* fileName, bool isResource);

WXD_EXPORTED void
wxd_Sound_Destroy(wxd_Sound_t* self);

WXD_EXPORTED bool
wxd_Sound_IsOk(wxd_Sound_t* self);

WXD_EXPORTED bool
wxd_Sound_Play(wxd_Sound_t* self, unsigned int flags);

WXD_EXPORTED bool
wxd_Sound_PlayFile(const char* filename, unsigned int flags);

WXD_EXPORTED void
wxd_Sound_Stop(void);

#ifdef __cplusplus
}
#endif

#endif // WXD_SOUND_H
