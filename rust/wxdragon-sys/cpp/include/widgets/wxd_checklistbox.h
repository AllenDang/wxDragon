#ifndef WXD_CHECKLISTBOX_H
#define WXD_CHECKLISTBOX_H

#include "../wxd_types.h"

// --- CheckListBox Functions ---
WXD_EXPORTED wxd_CheckListBox_t*
wxd_CheckListBox_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                        wxd_Style_t style);
WXD_EXPORTED void
wxd_CheckListBox_Append(wxd_CheckListBox_t* clbox, const char* item);
WXD_EXPORTED void
wxd_CheckListBox_Clear(wxd_CheckListBox_t* clbox);
WXD_EXPORTED int
wxd_CheckListBox_GetSelection(wxd_CheckListBox_t* clbox);
WXD_EXPORTED int
wxd_CheckListBox_GetStringSelection(wxd_CheckListBox_t* clbox, char* buffer, int buffer_len);
WXD_EXPORTED void
wxd_CheckListBox_SetSelection(wxd_CheckListBox_t* clbox, int index, bool select);
WXD_EXPORTED int
wxd_CheckListBox_GetString(wxd_CheckListBox_t* clbox, int index, char* buffer, int buffer_len);
WXD_EXPORTED unsigned int
wxd_CheckListBox_GetCount(wxd_CheckListBox_t* clbox);
WXD_EXPORTED bool
wxd_CheckListBox_IsChecked(wxd_CheckListBox_t* clbox, unsigned int index);
WXD_EXPORTED void
wxd_CheckListBox_Check(wxd_CheckListBox_t* clbox, unsigned int index, bool check);

#endif // WXD_CHECKLISTBOX_H