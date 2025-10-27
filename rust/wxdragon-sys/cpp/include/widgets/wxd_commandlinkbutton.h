#ifndef WXD_COMMANDLINKBUTTON_H
#define WXD_COMMANDLINKBUTTON_H

#include "../wxd_types.h"

// --- CommandLinkButton Functions ---
WXD_EXPORTED wxd_CommandLinkButton_t*
wxd_CommandLinkButton_Create(wxd_Window_t* parent, wxd_Id id, const char* mainLabel,
                             const char* note, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void
wxd_CommandLinkButton_SetNote(wxd_CommandLinkButton_t* self, const char* note);

#endif // WXD_COMMANDLINKBUTTON_H