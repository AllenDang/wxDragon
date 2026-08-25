#ifndef WXD_COMBOBOX_H
#define WXD_COMBOBOX_H

#include "../wxd_types.h"

// --- ComboBox Functions ---
WXD_EXPORTED wxd_ComboBox_t*
wxd_ComboBox_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos,
                    wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void
wxd_ComboBox_Append(wxd_ComboBox_t* combo, const char* item);
WXD_EXPORTED void
wxd_ComboBox_Insert(wxd_ComboBox_t* combo, const char* item, unsigned int pos);
WXD_EXPORTED void
wxd_ComboBox_Clear(wxd_ComboBox_t* combo);
WXD_EXPORTED int
wxd_ComboBox_GetSelection(wxd_ComboBox_t* combo);

WXD_EXPORTED int
wxd_ComboBox_GetStringSelection(wxd_ComboBox_t* combo, char* buffer, size_t buffer_len);

WXD_EXPORTED void
wxd_ComboBox_SetSelection(wxd_ComboBox_t* combo, int index);

WXD_EXPORTED int
wxd_ComboBox_GetString(wxd_ComboBox_t* combo, int index, char* buffer, size_t buffer_len);

WXD_EXPORTED unsigned int
wxd_ComboBox_GetCount(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_SetValue(wxd_ComboBox_t* combo, const char* value);

WXD_EXPORTED int
wxd_ComboBox_GetValue(wxd_ComboBox_t* combo, char* buffer, size_t buffer_len);

// Text Selection Functions (inherited from wxTextEntry)
WXD_EXPORTED void
wxd_ComboBox_GetTextSelection(wxd_ComboBox_t* combo, wxd_Long_t* from, wxd_Long_t* to);
WXD_EXPORTED void
wxd_ComboBox_SetTextSelection(wxd_ComboBox_t* combo, wxd_Long_t from, wxd_Long_t to);
WXD_EXPORTED wxd_Long_t
wxd_ComboBox_GetInsertionPoint(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_SetInsertionPoint(wxd_ComboBox_t* combo, wxd_Long_t pos);
WXD_EXPORTED wxd_Long_t
wxd_ComboBox_GetLastPosition(wxd_ComboBox_t* combo);

// Text entry functions (inherited from wxTextEntry)
WXD_EXPORTED void
wxd_ComboBox_ChangeValue(wxd_ComboBox_t* combo, const char* value);
WXD_EXPORTED void
wxd_ComboBox_WriteText(wxd_ComboBox_t* combo, const char* text);
WXD_EXPORTED void
wxd_ComboBox_AppendText(wxd_ComboBox_t* combo, const char* text);
WXD_EXPORTED void
wxd_ComboBox_Remove(wxd_ComboBox_t* combo, wxd_Long_t from, wxd_Long_t to);
WXD_EXPORTED void
wxd_ComboBox_Replace(wxd_ComboBox_t* combo, wxd_Long_t from, wxd_Long_t to, const char* value);
WXD_EXPORTED int
wxd_ComboBox_GetRange(wxd_ComboBox_t* combo, wxd_Long_t from, wxd_Long_t to, char* buffer,
                      size_t buffer_len);
WXD_EXPORTED void
wxd_ComboBox_SelectAll(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_SelectNone(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_SetInsertionPointEnd(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_SetEditable(wxd_ComboBox_t* combo, bool editable);
WXD_EXPORTED bool
wxd_ComboBox_IsEditable(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_SetMaxLength(wxd_ComboBox_t* combo, wxd_Long_t len);
WXD_EXPORTED bool
wxd_ComboBox_SetHint(wxd_ComboBox_t* combo, const char* hint);
WXD_EXPORTED int
wxd_ComboBox_GetHint(wxd_ComboBox_t* combo, char* buffer, size_t buffer_len);
WXD_EXPORTED bool
wxd_ComboBox_IsTextEmpty(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_Copy(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_Cut(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_Paste(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_Undo(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_Redo(wxd_ComboBox_t* combo);
WXD_EXPORTED bool
wxd_ComboBox_CanCopy(wxd_ComboBox_t* combo);
WXD_EXPORTED bool
wxd_ComboBox_CanCut(wxd_ComboBox_t* combo);
WXD_EXPORTED bool
wxd_ComboBox_CanPaste(wxd_ComboBox_t* combo);
WXD_EXPORTED bool
wxd_ComboBox_CanUndo(wxd_ComboBox_t* combo);
WXD_EXPORTED bool
wxd_ComboBox_CanRedo(wxd_ComboBox_t* combo);
WXD_EXPORTED bool
wxd_ComboBox_AutoComplete(wxd_ComboBox_t* combo, const wxd_ArrayString_t* choices);

// Item functions (inherited from wxItemContainer)
WXD_EXPORTED void
wxd_ComboBox_Delete(wxd_ComboBox_t* combo, unsigned int index);
WXD_EXPORTED void
wxd_ComboBox_SetString(wxd_ComboBox_t* combo, unsigned int index, const char* text);
WXD_EXPORTED bool
wxd_ComboBox_SetStringSelection(wxd_ComboBox_t* combo, const char* text);
WXD_EXPORTED int
wxd_ComboBox_FindString(wxd_ComboBox_t* combo, const char* text, bool case_sensitive);
WXD_EXPORTED wxd_ArrayString_t*
wxd_ComboBox_GetStrings(wxd_ComboBox_t* combo);
WXD_EXPORTED bool
wxd_ComboBox_IsListEmpty(wxd_ComboBox_t* combo);
WXD_EXPORTED bool
wxd_ComboBox_IsSorted(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_AppendItems(wxd_ComboBox_t* combo, const wxd_ArrayString_t* items);
WXD_EXPORTED void
wxd_ComboBox_InsertItems(wxd_ComboBox_t* combo, const wxd_ArrayString_t* items, unsigned int pos);
WXD_EXPORTED void
wxd_ComboBox_SetItems(wxd_ComboBox_t* combo, const wxd_ArrayString_t* items);

// Dropdown functions
WXD_EXPORTED void
wxd_ComboBox_Popup(wxd_ComboBox_t* combo);
WXD_EXPORTED void
wxd_ComboBox_Dismiss(wxd_ComboBox_t* combo);
WXD_EXPORTED int
wxd_ComboBox_GetCurrentSelection(wxd_ComboBox_t* combo);

#endif // WXD_COMBOBOX_H