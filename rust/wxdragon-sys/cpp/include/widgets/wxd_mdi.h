#ifndef WXD_MDI_H
#define WXD_MDI_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// --- wxMDIParentFrame ---
WXD_EXPORTED wxd_Frame_t*
wxd_MDIParentFrame_Create(wxd_Window_t* parent, wxd_Id id, const char* title, wxd_Point pos,
                          wxd_Size size, wxd_Style_t style, const char* name);

// --- wxMDIChildFrame ---
WXD_EXPORTED wxd_Frame_t*
wxd_MDIChildFrame_Create(wxd_Frame_t* parent, wxd_Id id, const char* title, wxd_Point pos,
                         wxd_Size size, wxd_Style_t style, const char* name);

// --- wxMDIClientWindow ---
WXD_EXPORTED wxd_Window_t*
wxd_MDIParentFrame_GetClientWindow(wxd_Frame_t* frame);

#ifdef __cplusplus
}
#endif

#endif // WXD_MDI_H
