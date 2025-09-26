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

WXD_EXPORTED wxd_ComboBox_t* wxd_ComboBox_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    const char* value, // Initial value for text field
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
) {
    wxWindow* parentWin = (wxWindow*)parent;
    if (!parentWin) return nullptr;

    wxString wxValue = wxString::FromUTF8(value ? value : "");
    wxComboBox* combo = new wxComboBox(parentWin, id, wxValue, 
                                     wxd_cpp_utils::to_wx(pos),
                                     wxd_cpp_utils::to_wx(size),
                                     0, nullptr, style);
    return (wxd_ComboBox_t*)combo;
}

WXD_EXPORTED void wxd_ComboBox_Append(wxd_ComboBox_t* combo, const char* item) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb && item) {
        cb->Append(wxString::FromUTF8(item));
    }
}

WXD_EXPORTED void wxd_ComboBox_Clear(wxd_ComboBox_t* combo) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->Clear(); // Clears list items
        // cb->SetValue(""); // Optionally clear text field too?
                          // Standard Clear() only clears the list.
    }
}

WXD_EXPORTED int wxd_ComboBox_GetSelection(wxd_ComboBox_t* combo) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb) return wxNOT_FOUND;
    return cb->GetSelection(); // Returns wxNOT_FOUND (-1) if nothing selected
}

WXD_EXPORTED int wxd_ComboBox_GetStringSelection(wxd_ComboBox_t* combo, char* buffer, int buffer_len) {
    if (!combo || !buffer || buffer_len <= 0) return -1;
    wxComboBox* cb = (wxComboBox*)combo;
    wxString selection = cb->GetStringSelection();
    return wxd_cpp_utils::copy_wxstring_to_buffer(selection, buffer, (size_t)buffer_len);
}

WXD_EXPORTED void wxd_ComboBox_SetSelection(wxd_ComboBox_t* combo, int index) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        // SetSelection also updates the text field to the selected string
        cb->SetSelection(index);
    }
}

WXD_EXPORTED int wxd_ComboBox_GetString(wxd_ComboBox_t* combo, int index, char* buffer, int buffer_len) {
    if (!combo || !buffer || buffer_len <= 0) return -1;
    wxComboBox* cb = (wxComboBox*)combo;
    if (index < 0 || (unsigned int)index >= cb->GetCount()) return -1;

    wxString item = cb->GetString((unsigned int)index);
    return wxd_cpp_utils::copy_wxstring_to_buffer(item, buffer, (size_t)buffer_len);
}

WXD_EXPORTED unsigned int wxd_ComboBox_GetCount(wxd_ComboBox_t* combo) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb) return 0;
    return cb->GetCount();
}

WXD_EXPORTED void wxd_ComboBox_SetValue(wxd_ComboBox_t* combo, const char* value) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->SetValue(wxString::FromUTF8(value ? value : ""));
    }
}

WXD_EXPORTED int wxd_ComboBox_GetValue(wxd_ComboBox_t* combo, char* buffer, int buffer_len) {
    if (!combo || !buffer || buffer_len <= 0) return -1;
    wxComboBox* cb = (wxComboBox*)combo;
    wxString value = cb->GetValue();
    return wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, (size_t)buffer_len);
}

// Text Selection Functions (inherited from wxTextEntry)
WXD_EXPORTED void wxd_ComboBox_GetTextSelection(wxd_ComboBox_t* combo, wxd_Long_t* from, wxd_Long_t* to) {
    if (!combo || !from || !to) return;
    wxComboBox* cb = (wxComboBox*)combo;
    long wx_from, wx_to;
    cb->GetSelection(&wx_from, &wx_to);
    *from = static_cast<wxd_Long_t>(wx_from);
    *to = static_cast<wxd_Long_t>(wx_to);
}

WXD_EXPORTED void wxd_ComboBox_SetTextSelection(wxd_ComboBox_t* combo, wxd_Long_t from, wxd_Long_t to) {
    if (!combo) return;
    wxComboBox* cb = (wxComboBox*)combo;
    cb->SetSelection(static_cast<long>(from), static_cast<long>(to));
}

WXD_EXPORTED wxd_Long_t wxd_ComboBox_GetInsertionPoint(wxd_ComboBox_t* combo) {
    if (!combo) return 0;
    wxComboBox* cb = (wxComboBox*)combo;
    return static_cast<wxd_Long_t>(cb->GetInsertionPoint());
}

WXD_EXPORTED void wxd_ComboBox_SetInsertionPoint(wxd_ComboBox_t* combo, wxd_Long_t pos) {
    if (!combo) return;
    wxComboBox* cb = (wxComboBox*)combo;
    cb->SetInsertionPoint(static_cast<long>(pos));
}

WXD_EXPORTED wxd_Long_t wxd_ComboBox_GetLastPosition(wxd_ComboBox_t* combo) {
    if (!combo) return 0;
    wxComboBox* cb = (wxComboBox*)combo;
    return static_cast<wxd_Long_t>(cb->GetLastPosition());
}

// Destroy handled by parent window

} // extern "C" 