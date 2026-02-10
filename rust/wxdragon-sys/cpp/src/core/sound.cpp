#include <wx/sound.h>
#include "../../include/core/wxd_sound.h"
#include "../wxd_utils.h"

extern "C" {

wxd_Sound_t*
wxd_Sound_Create(const char* fileName, bool isResource) {
    wxSound* sound = new wxSound(wxString::FromUTF8(fileName), isResource);
    return (wxd_Sound_t*)sound;
}

void
wxd_Sound_Destroy(wxd_Sound_t* self) {
    if (self) {
        delete (wxSound*)self;
    }
}

bool
wxd_Sound_IsOk(wxd_Sound_t* self) {
    if (self) {
        return ((wxSound*)self)->IsOk();
    }
    return false;
}

bool
wxd_Sound_Play(wxd_Sound_t* self, unsigned int flags) {
    if (self) {
        return ((wxSound*)self)->Play(flags);
    }
    return false;
}

bool
wxd_Sound_PlayFile(const char* filename, unsigned int flags) {
    return wxSound::Play(wxString::FromUTF8(filename), flags);
}

void
wxd_Sound_Stop(void) {
    wxSound::Stop();
}

} // extern "C"
