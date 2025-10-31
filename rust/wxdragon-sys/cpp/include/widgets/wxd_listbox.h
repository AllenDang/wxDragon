#ifndef WXD_LISTBOX_H
#define WXD_LISTBOX_H

#include "../wxd_types.h"

// --- ListBox Functions ---
WXD_EXPORTED wxd_ListBox_t*
wxd_ListBox_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                   wxd_Style_t style);
WXD_EXPORTED void
wxd_ListBox_Append(wxd_ListBox_t* self, const char* item);
WXD_EXPORTED void
wxd_ListBox_Clear(wxd_ListBox_t* listbox);
WXD_EXPORTED int
wxd_ListBox_GetSelection(wxd_ListBox_t* listbox);

WXD_EXPORTED int
wxd_ListBox_GetStringSelection(wxd_ListBox_t* listbox, char* buffer, size_t buffer_len);

WXD_EXPORTED void
wxd_ListBox_SetSelection(wxd_ListBox_t* listbox, int index, bool select);
WXD_EXPORTED void
wxd_ListBox_SetStringSelection(wxd_ListBox_t* listbox, const char* item, bool select);

WXD_EXPORTED int
wxd_ListBox_GetString(wxd_ListBox_t* listbox, int index, char* buffer, size_t buffer_len);

WXD_EXPORTED unsigned int
wxd_ListBox_GetCount(wxd_ListBox_t* listbox);
WXD_EXPORTED void
wxd_ListBox_Delete(wxd_ListBox_t* listbox, int index);
WXD_EXPORTED bool
wxd_ListBox_PopupMenu(wxd_ListBox_t* listbox, wxd_Menu_t* menu, wxd_Point pos);

#endif // WXD_LISTBOX_H