#ifndef WXD_EDITABLELISTBOX_H
#define WXD_EDITABLELISTBOX_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Create a new wxEditableListBox
 * 
 * @param parent Parent window
 * @param id Window identifier
 * @param label Label shown at the top of the listbox
 * @param pos Initial position
 * @param size Initial size
 * @param style The window style, see wxEditableListBox
 * @return Pointer to the new wxEditableListBox
 */
wxd_Window_t*
wxd_EditableListBox_New(wxd_Window_t* parent, int id, const char* label, int x, int y, int width,
                        int height, int64_t style);

/**
 * @brief Set the string items in the listbox
 * 
 * @param self Pointer to wxEditableListBox
 * @param strings Array of strings
 * @param count Number of strings
 */
void
wxd_EditableListBox_SetStrings(wxd_Window_t* self, const char** strings, int count);

/**
 * @brief Add a string item to the listbox
 * 
 * @param self Pointer to wxEditableListBox
 * @param string String to add
 */
void
wxd_EditableListBox_AddString(wxd_Window_t* self, const char* string);

/**
 * @brief Get the underlying wxListCtrl
 * 
 * @param self Pointer to wxEditableListBox
 * @return Pointer to the internal wxListBox 
 */
wxd_Window_t*
wxd_EditableListBox_GetListCtrl(wxd_Window_t* self);

/**
 * @brief Copy strings from the listbox to an array of wxd_ArrayString_t
 * 
 * @param self Pointer to wxEditableListBox
 * @return Pointer to the array of strings
 */
wxd_ArrayString_t*
wxd_EditableListBox_CopyStringsToArrayString(wxd_Window_t* self_ptr);

#ifdef __cplusplus
}
#endif

#endif // WXD_EDITABLELISTBOX_H