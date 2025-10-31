#ifndef WXD_RADIOBOX_H
#define WXD_RADIOBOX_H

#include "../wxd_types.h"

// --- RadioBox Functions ---
WXD_EXPORTED wxd_RadioBox_t*
wxd_RadioBox_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos,
                    wxd_Size size, int n, const char* const* choices, int majorDimension,
                    wxd_Style_t style);
WXD_EXPORTED int
wxd_RadioBox_GetSelection(wxd_RadioBox_t* self);
WXD_EXPORTED void
wxd_RadioBox_SetSelection(wxd_RadioBox_t* self, int n);

WXD_EXPORTED int
wxd_RadioBox_GetString(wxd_RadioBox_t* self, int n, char* buffer, size_t buffer_len);

WXD_EXPORTED unsigned int
wxd_RadioBox_GetCount(wxd_RadioBox_t* self);
WXD_EXPORTED bool
wxd_RadioBox_EnableItem(wxd_RadioBox_t* self, int n, bool enable);
WXD_EXPORTED bool
wxd_RadioBox_IsItemEnabled(wxd_RadioBox_t* self, int n);
WXD_EXPORTED bool
wxd_RadioBox_ShowItem(wxd_RadioBox_t* self, int n, bool show);
WXD_EXPORTED bool
wxd_RadioBox_IsItemShown(wxd_RadioBox_t* self, int n);

#endif // WXD_RADIOBOX_H