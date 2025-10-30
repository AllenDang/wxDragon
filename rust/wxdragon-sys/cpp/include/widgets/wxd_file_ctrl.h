#ifndef WXD_FILE_CTRL_H
#define WXD_FILE_CTRL_H

#include "../wxd_types.h" // For WXD_WINDOW, WXD_STRING, etc.
// #include "../generated/wxd_generated_enums.h" // REMOVING THIS - WXDObjType pattern not found

#ifdef __cplusplus
extern "C" {
#endif

wxd_FileCtrl_t*
wxd_FileCtrl_Create(wxd_Window_t* parent, int id, const char* default_directory,
                    const char* default_filename, const char* wild_card, int64_t style, int pos_x,
                    int pos_y, int size_w, int size_h, const char* name);

WXD_EXPORTED size_t
wxd_FileCtrl_GetPath(const wxd_FileCtrl_t* self, char* buffer, size_t buffer_len);

// Placeholder for other wxFileCtrl specific functions:
// void wxd_FileCtrl_SetPath(wxd_FileCtrl_t* self, const char* path);
// const char* wxd_FileCtrl_GetFilename(wxd_FileCtrl_t* self);
// void wxd_FileCtrl_SetFilename(wxd_FileCtrl_t* self, const char* filename);
// const char* wxd_FileCtrl_GetDirectory(wxd_FileCtrl_t* self);
// void wxd_FileCtrl_SetDirectory(wxd_FileCtrl_t* self, const char* directory);
// const char* wxd_FileCtrl_GetWildcard(wxd_FileCtrl_t* self);
// void wxd_FileCtrl_SetWildcard(wxd_FileCtrl_t* self, const char* wildcard);
// int wxd_FileCtrl_GetFilterIndex(wxd_FileCtrl_t* self);
// void wxd_FileCtrl_SetFilterIndex(wxd_FileCtrl_t* self, int filter_index);
// bool wxd_FileCtrl_ShowHidden(wxd_FileCtrl_t* self, bool show);

#ifdef __cplusplus
}
#endif

#endif // WXD_FILE_CTRL_H