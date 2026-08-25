#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wx/combobox.h"
#include "wx/window.h"
#include "wx/string.h"
#include "wx/arrstr.h"
#include "wx/defs.h" // For wxNOT_FOUND
#include "../include/wxdragon.h"
#include "wxd_utils.h"

extern "C" {

WXD_EXPORTED wxd_ComboBox_t*
wxd_ComboBox_Create(wxd_Window_t* parent, wxd_Id id,
                    const char* value, // Initial value for text field
                    wxd_Point pos, wxd_Size size, wxd_Style_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    if (!parentWin)
        return nullptr;

    wxString wxValue = wxString::FromUTF8(value ? value : "");
    wxComboBox* combo = new wxComboBox(parentWin, id, wxValue, wxd_cpp_utils::to_wx(pos),
                                       wxd_cpp_utils::to_wx(size), 0, nullptr, style);
    return (wxd_ComboBox_t*)combo;
}

WXD_EXPORTED void
wxd_ComboBox_Append(wxd_ComboBox_t* combo, const char* item)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb && item) {
        cb->Append(wxString::FromUTF8(item));
    }
}

WXD_EXPORTED void
wxd_ComboBox_Insert(wxd_ComboBox_t* combo, const char* item, unsigned int pos)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb && item) {
        cb->Insert(wxString::FromUTF8(item), pos);
    }
}

WXD_EXPORTED void
wxd_ComboBox_Clear(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        // wxComboBox::Clear also empties the text entry on MSW, where
        // wxTextEntry::Clear runs alongside the list clear and generates
        // a text event.
        cb->Clear();
    }
}

WXD_EXPORTED int
wxd_ComboBox_GetSelection(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return wxNOT_FOUND;
    return cb->GetSelection(); // Returns wxNOT_FOUND (-1) if nothing selected
}

WXD_EXPORTED int
wxd_ComboBox_GetStringSelection(wxd_ComboBox_t* combo, char* buffer, size_t buffer_len)
{
    if (!combo)
        return -1;
    wxComboBox* cb = (wxComboBox*)combo;
    wxString selection = cb->GetStringSelection();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(selection, buffer, buffer_len);
}

WXD_EXPORTED void
wxd_ComboBox_SetSelection(wxd_ComboBox_t* combo, int index)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        // SetSelection also updates the text field to the selected string
        cb->SetSelection(index);
    }
}

WXD_EXPORTED int
wxd_ComboBox_GetString(wxd_ComboBox_t* combo, int index, char* buffer, size_t buffer_len)
{
    if (!combo)
        return -1;
    wxComboBox* cb = (wxComboBox*)combo;
    if (index < 0 || (unsigned int)index >= cb->GetCount())
        return -1;

    wxString item = cb->GetString((unsigned int)index);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(item, buffer, buffer_len);
}

WXD_EXPORTED unsigned int
wxd_ComboBox_GetCount(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return 0;
    return cb->GetCount();
}

WXD_EXPORTED void
wxd_ComboBox_SetValue(wxd_ComboBox_t* combo, const char* value)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->SetValue(wxString::FromUTF8(value ? value : ""));
    }
}

WXD_EXPORTED int
wxd_ComboBox_GetValue(wxd_ComboBox_t* combo, char* buffer, size_t buffer_len)
{
    if (!combo)
        return -1;
    wxComboBox* cb = (wxComboBox*)combo;
    wxString value = cb->GetValue();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, buffer_len);
}

// Text Selection Functions (inherited from wxTextEntry)
WXD_EXPORTED void
wxd_ComboBox_GetTextSelection(wxd_ComboBox_t* combo, wxd_Long_t* from, wxd_Long_t* to)
{
    if (!combo || !from || !to)
        return;
    wxComboBox* cb = (wxComboBox*)combo;
    long wx_from, wx_to;
    cb->GetSelection(&wx_from, &wx_to);
    *from = static_cast<wxd_Long_t>(wx_from);
    *to = static_cast<wxd_Long_t>(wx_to);
}

WXD_EXPORTED void
wxd_ComboBox_SetTextSelection(wxd_ComboBox_t* combo, wxd_Long_t from, wxd_Long_t to)
{
    if (!combo)
        return;
    wxComboBox* cb = (wxComboBox*)combo;
    cb->SetSelection(static_cast<long>(from), static_cast<long>(to));
}

WXD_EXPORTED wxd_Long_t
wxd_ComboBox_GetInsertionPoint(wxd_ComboBox_t* combo)
{
    if (!combo)
        return 0;
    wxComboBox* cb = (wxComboBox*)combo;
    return static_cast<wxd_Long_t>(cb->GetInsertionPoint());
}

WXD_EXPORTED void
wxd_ComboBox_SetInsertionPoint(wxd_ComboBox_t* combo, wxd_Long_t pos)
{
    if (!combo)
        return;
    wxComboBox* cb = (wxComboBox*)combo;
    cb->SetInsertionPoint(static_cast<long>(pos));
}

WXD_EXPORTED wxd_Long_t
wxd_ComboBox_GetLastPosition(wxd_ComboBox_t* combo)
{
    if (!combo)
        return 0;
    wxComboBox* cb = (wxComboBox*)combo;
    return static_cast<wxd_Long_t>(cb->GetLastPosition());
}

// Text entry functions (inherited from wxTextEntry)
WXD_EXPORTED void
wxd_ComboBox_ChangeValue(wxd_ComboBox_t* combo, const char* value)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->ChangeValue(wxString::FromUTF8(value ? value : ""));
    }
}

WXD_EXPORTED void
wxd_ComboBox_WriteText(wxd_ComboBox_t* combo, const char* text)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb && text) {
        cb->WriteText(wxString::FromUTF8(text));
    }
}

WXD_EXPORTED void
wxd_ComboBox_AppendText(wxd_ComboBox_t* combo, const char* text)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb && text) {
        cb->AppendText(wxString::FromUTF8(text));
    }
}

WXD_EXPORTED void
wxd_ComboBox_Remove(wxd_ComboBox_t* combo, wxd_Long_t from, wxd_Long_t to)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->Remove(static_cast<long>(from), static_cast<long>(to));
    }
}

WXD_EXPORTED void
wxd_ComboBox_Replace(wxd_ComboBox_t* combo, wxd_Long_t from, wxd_Long_t to, const char* value)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb && value) {
        cb->Replace(static_cast<long>(from), static_cast<long>(to), wxString::FromUTF8(value));
    }
}

WXD_EXPORTED int
wxd_ComboBox_GetRange(wxd_ComboBox_t* combo, wxd_Long_t from, wxd_Long_t to, char* buffer,
                      size_t buffer_len)
{
    if (!combo)
        return -1;
    wxComboBox* cb = (wxComboBox*)combo;
    wxString range = cb->GetRange(static_cast<long>(from), static_cast<long>(to));
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(range, buffer, buffer_len);
}

WXD_EXPORTED void
wxd_ComboBox_SelectAll(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->SelectAll();
    }
}

WXD_EXPORTED void
wxd_ComboBox_SelectNone(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->SelectNone();
    }
}

WXD_EXPORTED void
wxd_ComboBox_SetInsertionPointEnd(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->SetInsertionPointEnd();
    }
}

WXD_EXPORTED void
wxd_ComboBox_SetEditable(wxd_ComboBox_t* combo, bool editable)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->SetEditable(editable);
    }
}

WXD_EXPORTED bool
wxd_ComboBox_IsEditable(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return false;
    return cb->IsEditable();
}

WXD_EXPORTED void
wxd_ComboBox_SetMaxLength(wxd_ComboBox_t* combo, wxd_Long_t len)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->SetMaxLength(static_cast<unsigned long>(len));
    }
}

WXD_EXPORTED bool
wxd_ComboBox_SetHint(wxd_ComboBox_t* combo, const char* hint)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return false;
    return cb->SetHint(wxString::FromUTF8(hint ? hint : ""));
}

WXD_EXPORTED int
wxd_ComboBox_GetHint(wxd_ComboBox_t* combo, char* buffer, size_t buffer_len)
{
    if (!combo)
        return -1;
    wxComboBox* cb = (wxComboBox*)combo;
    wxString hint = cb->GetHint();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(hint, buffer, buffer_len);
}

WXD_EXPORTED bool
wxd_ComboBox_IsTextEmpty(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return true;
    return cb->IsTextEmpty();
}

WXD_EXPORTED void
wxd_ComboBox_Copy(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->Copy();
    }
}

WXD_EXPORTED void
wxd_ComboBox_Cut(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->Cut();
    }
}

WXD_EXPORTED void
wxd_ComboBox_Paste(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->Paste();
    }
}

WXD_EXPORTED void
wxd_ComboBox_Undo(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->Undo();
    }
}

WXD_EXPORTED void
wxd_ComboBox_Redo(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->Redo();
    }
}

WXD_EXPORTED bool
wxd_ComboBox_CanCopy(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return false;
    return cb->CanCopy();
}

WXD_EXPORTED bool
wxd_ComboBox_CanCut(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return false;
    return cb->CanCut();
}

WXD_EXPORTED bool
wxd_ComboBox_CanPaste(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return false;
    return cb->CanPaste();
}

WXD_EXPORTED bool
wxd_ComboBox_CanUndo(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return false;
    return cb->CanUndo();
}

WXD_EXPORTED bool
wxd_ComboBox_CanRedo(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return false;
    return cb->CanRedo();
}

WXD_EXPORTED bool
wxd_ComboBox_AutoComplete(wxd_ComboBox_t* combo, const wxd_ArrayString_t* choices)
{
    wxComboBox* cb = (wxComboBox*)combo;
    const wxArrayString* arr = reinterpret_cast<const wxArrayString*>(choices);
    if (!cb || !arr)
        return false;
    return cb->AutoComplete(*arr);
}

// Item functions (inherited from wxItemContainer)
WXD_EXPORTED void
wxd_ComboBox_Delete(wxd_ComboBox_t* combo, unsigned int index)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb && index < cb->GetCount()) {
        cb->Delete(index);
    }
}

WXD_EXPORTED void
wxd_ComboBox_SetString(wxd_ComboBox_t* combo, unsigned int index, const char* text)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb && text && index < cb->GetCount()) {
        cb->SetString(index, wxString::FromUTF8(text));
    }
}

WXD_EXPORTED bool
wxd_ComboBox_SetStringSelection(wxd_ComboBox_t* combo, const char* text)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb || !text)
        return false;
    return cb->SetStringSelection(wxString::FromUTF8(text));
}

WXD_EXPORTED int
wxd_ComboBox_FindString(wxd_ComboBox_t* combo, const char* text, bool case_sensitive)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb || !text)
        return wxNOT_FOUND;
    return cb->FindString(wxString::FromUTF8(text), case_sensitive);
}

WXD_EXPORTED wxd_ArrayString_t*
wxd_ComboBox_GetStrings(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return nullptr;
    return reinterpret_cast<wxd_ArrayString_t*>(new (std::nothrow) wxArrayString(cb->GetStrings()));
}

WXD_EXPORTED bool
wxd_ComboBox_IsListEmpty(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return true;
    return cb->IsListEmpty();
}

WXD_EXPORTED bool
wxd_ComboBox_IsSorted(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return false;
    return cb->IsSorted();
}

WXD_EXPORTED void
wxd_ComboBox_AppendItems(wxd_ComboBox_t* combo, const wxd_ArrayString_t* items)
{
    wxComboBox* cb = (wxComboBox*)combo;
    const wxArrayString* arr = reinterpret_cast<const wxArrayString*>(items);
    if (cb && arr) {
        cb->Append(*arr);
    }
}

WXD_EXPORTED void
wxd_ComboBox_InsertItems(wxd_ComboBox_t* combo, const wxd_ArrayString_t* items, unsigned int pos)
{
    wxComboBox* cb = (wxComboBox*)combo;
    const wxArrayString* arr = reinterpret_cast<const wxArrayString*>(items);
    if (cb && arr && pos <= cb->GetCount()) {
        cb->Insert(*arr, pos);
    }
}

WXD_EXPORTED void
wxd_ComboBox_SetItems(wxd_ComboBox_t* combo, const wxd_ArrayString_t* items)
{
    wxComboBox* cb = (wxComboBox*)combo;
    const wxArrayString* arr = reinterpret_cast<const wxArrayString*>(items);
    if (cb && arr) {
        cb->Set(*arr);
    }
}

// Dropdown functions
WXD_EXPORTED void
wxd_ComboBox_Popup(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->Popup();
    }
}

WXD_EXPORTED void
wxd_ComboBox_Dismiss(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->Dismiss();
    }
}

WXD_EXPORTED int
wxd_ComboBox_GetCurrentSelection(wxd_ComboBox_t* combo)
{
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb)
        return wxNOT_FOUND;
    return cb->GetCurrentSelection();
}

// Destroy handled by parent window

} // extern "C"