#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wx/listbox.h"
#include "wx/window.h"
#include "wx/string.h"
#include "wx/arrstr.h"
#include "wx/defs.h" // For wxNOT_FOUND
#include "../include/wxdragon.h"
#include "wxd_utils.h"

extern "C" {

WXD_EXPORTED wxd_ListBox_t* wxd_ListBox_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
) {
    wxWindow* parentWin = (wxWindow*)parent;
    if (!parentWin) return nullptr;

    wxListBox* listbox = new wxListBox(parentWin, id, 
                                     wxd_cpp_utils::to_wx(pos),
                                     wxd_cpp_utils::to_wx(size),
                                     0, nullptr, style);
    return (wxd_ListBox_t*)listbox;
}

WXD_EXPORTED void wxd_ListBox_Append(wxd_ListBox_t* listbox, const char* item) {
    wxListBox* lb = (wxListBox*)listbox;
    if (lb && item) {
        lb->Append(wxString::FromUTF8(item));
    }
}

WXD_EXPORTED void wxd_ListBox_Clear(wxd_ListBox_t* listbox) {
    wxListBox* lb = (wxListBox*)listbox;
    if (lb) {
        lb->Clear();
    }
}

WXD_EXPORTED int wxd_ListBox_GetSelection(wxd_ListBox_t* listbox) {
    wxListBox* lb = (wxListBox*)listbox;
    if (!lb) return wxNOT_FOUND;
    return lb->GetSelection();
}

WXD_EXPORTED int wxd_ListBox_GetStringSelection(wxd_ListBox_t* listbox, char* buffer, int buffer_len) {
    if (!listbox || !buffer || buffer_len <= 0) return -1;
    wxListBox* lb = (wxListBox*)listbox;
    wxString selection = lb->GetStringSelection();
    return wxd_cpp_utils::copy_wxstring_to_buffer(selection, buffer, (size_t)buffer_len);
}

WXD_EXPORTED void wxd_ListBox_SetSelection(wxd_ListBox_t* listbox, int index, bool select) {
    wxListBox* lb = (wxListBox*)listbox;
    if (lb) {
        lb->SetSelection(index, select);
    }
}

WXD_EXPORTED int wxd_ListBox_GetString(wxd_ListBox_t* listbox, int index, char* buffer, int buffer_len) {
    if (!listbox || !buffer || buffer_len <= 0) return -1;
    wxListBox* lb = (wxListBox*)listbox;
    if (index < 0 || (unsigned int)index >= lb->GetCount()) return -1;

    wxString item = lb->GetString((unsigned int)index);
    return wxd_cpp_utils::copy_wxstring_to_buffer(item, buffer, (size_t)buffer_len);
}

WXD_EXPORTED unsigned int wxd_ListBox_GetCount(wxd_ListBox_t* listbox) {
    wxListBox* lb = (wxListBox*)listbox;
    if (!lb) return 0;
    return lb->GetCount();
}

} // extern "C" 