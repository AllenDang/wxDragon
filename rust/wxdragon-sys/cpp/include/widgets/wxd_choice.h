#ifndef WXD_CHOICE_H
#define WXD_CHOICE_H

#include "../wxd_types.h"

// --- Choice Functions ---
WXD_EXPORTED wxd_Choice_t*
wxd_Choice_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void
wxd_Choice_Append(wxd_Choice_t* self, const char* item);
WXD_EXPORTED void
wxd_Choice_Clear(wxd_Choice_t* choice);
WXD_EXPORTED int
wxd_Choice_GetSelection(wxd_Choice_t* choice);

WXD_EXPORTED int
wxd_Choice_GetStringSelection(wxd_Choice_t* choice, char* buffer, size_t buffer_len);

WXD_EXPORTED void
wxd_Choice_SetSelection(wxd_Choice_t* choice, int index);

WXD_EXPORTED int
wxd_Choice_GetString(wxd_Choice_t* choice, int index, char* buffer, size_t buffer_len);

WXD_EXPORTED unsigned int
wxd_Choice_GetCount(wxd_Choice_t* choice);

#endif // WXD_CHOICE_H