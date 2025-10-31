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

/**
 * Retrieves the currently selected string from the wxCheckListBox.
 * Returns the length of the string copied into the buffer, excluding the null terminator,
 * if any error, -1 returned.
 * If the buffer is too small, it copies as much as fits and returns the required size.
 * If no selection is made, returns 0 and does not modify the buffer.
 */
WXD_EXPORTED int
wxd_CheckListBox_GetStringSelection(const wxd_CheckListBox_t* clbox, char* buffer,
                                    size_t buffer_len);

WXD_EXPORTED void
wxd_CheckListBox_SetSelection(wxd_CheckListBox_t* clbox, int index, bool select);

/**
 * Retrieves the string at the specified index from the wxCheckListBox.
 * Returns the length of the string copied into the buffer, excluding the null terminator,
 * if any error, -1 returned.
 * If the buffer is too small, it copies as much as fits and returns the required size.
 * If the index is out of bounds, returns 0 and does not modify the buffer.
 */
WXD_EXPORTED int
wxd_CheckListBox_GetString(const wxd_CheckListBox_t* clbox, size_t index, char* buffer,
                           size_t buffer_len);

WXD_EXPORTED unsigned int
wxd_CheckListBox_GetCount(wxd_CheckListBox_t* clbox);
WXD_EXPORTED bool
wxd_CheckListBox_IsChecked(wxd_CheckListBox_t* clbox, unsigned int index);
WXD_EXPORTED void
wxd_CheckListBox_Check(wxd_CheckListBox_t* clbox, unsigned int index, bool check);

#endif // WXD_CHECKLISTBOX_H