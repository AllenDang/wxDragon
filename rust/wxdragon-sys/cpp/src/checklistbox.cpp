#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/checklst.h> // Include the correct header
#include <wx/window.h>
#include <wx/defs.h>   // For wxID_ANY, wxNOT_FOUND, wxDefaultPosition, wxDefaultSize
#include "wxd_utils.h" // For wxd_cpp_utils::copy_wxstring_to_buffer

// Helper function (already defined in event.cpp, consider moving to a common utils file later)
// REMOVED: static int copy_wxstring_to_buffer(...) - Now using wxd_cpp_utils version

// --- wxCheckListBox C Functions ---

extern "C" {

WXD_EXPORTED wxd_CheckListBox_t*
wxd_CheckListBox_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                        wxd_Style_t style)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxPoint wxPos(pos.x, pos.y);
    wxSize wxSize(size.width, size.height);

    // For simplicity, we start with an empty choices array.
    // The wxCheckListBox constructor requires wxArrayString, but it can be empty.
    wxArrayString emptyChoices;

    wxCheckListBox* clbox = new wxCheckListBox(parentWin, id, wxPos, wxSize, emptyChoices, style);

    return reinterpret_cast<wxd_CheckListBox_t*>(clbox);
}

WXD_EXPORTED void
wxd_CheckListBox_Append(wxd_CheckListBox_t* clbox, const char* item)
{
    wxCheckListBox* wxClbox = reinterpret_cast<wxCheckListBox*>(clbox);
    if (wxClbox && item) {
        wxClbox->Append(wxString::FromUTF8(item));
    }
}

WXD_EXPORTED void
wxd_CheckListBox_Clear(wxd_CheckListBox_t* clbox)
{
    wxCheckListBox* wxClbox = reinterpret_cast<wxCheckListBox*>(clbox);
    if (wxClbox) {
        wxClbox->Clear();
    }
}

WXD_EXPORTED int
wxd_CheckListBox_GetSelection(wxd_CheckListBox_t* clbox)
{
    wxCheckListBox* wxClbox = reinterpret_cast<wxCheckListBox*>(clbox);
    if (wxClbox) {
        return wxClbox->GetSelection(); // Returns index or wxNOT_FOUND
    }
    return wxNOT_FOUND;
}

WXD_EXPORTED size_t
wxd_CheckListBox_GetStringSelection(const wxd_CheckListBox_t* clbox, char* buffer,
                                    size_t buffer_len)
{
    if (!clbox)
        return 0;
    const wxString sel = reinterpret_cast<const wxCheckListBox*>(clbox)->GetStringSelection();
    wxScopedCharBuffer strBuf = sel.ToUTF8();
    // Clear buffer first in case selection is empty
    if (buffer && buffer_len > 0) {
        size_t copyLen = wxMin(static_cast<size_t>(strBuf.length()), buffer_len - 1);
        memcpy(buffer, strBuf.data(), copyLen);
        buffer[copyLen] = '\0'; // Null-terminate the buffer
    }

    return strBuf.length();
}

WXD_EXPORTED void
wxd_CheckListBox_SetSelection(wxd_CheckListBox_t* clbox, int index, bool select)
{
    wxCheckListBox* wxClbox = reinterpret_cast<wxCheckListBox*>(clbox);
    if (wxClbox) {
        // Note: wxCheckListBox is single-selection by default. This behaves like wxListBox.
        wxClbox->SetSelection(index, select);
    }
}

WXD_EXPORTED size_t
wxd_CheckListBox_GetString(const wxd_CheckListBox_t* clbox, size_t index, char* buffer,
                           size_t buffer_len)
{
    if (!clbox)
        return 0;
    const wxCheckListBox* wxClbox = reinterpret_cast<const wxCheckListBox*>(clbox);
    if (index >= wxClbox->GetCount())
        return 0;
    const wxString str = wxClbox->GetString(index);
    wxScopedCharBuffer strBuf = str.ToUTF8();
    if (buffer && buffer_len > 0) {
        size_t copyLen = wxMin(static_cast<size_t>(strBuf.length()), buffer_len - 1);
        memcpy(buffer, strBuf.data(), copyLen);
        buffer[copyLen] = '\0'; // Null-terminate the buffer
    }
    return strBuf.length();
}

WXD_EXPORTED unsigned int
wxd_CheckListBox_GetCount(wxd_CheckListBox_t* clbox)
{
    wxCheckListBox* wxClbox = reinterpret_cast<wxCheckListBox*>(clbox);
    if (wxClbox) {
        return wxClbox->GetCount();
    }
    return 0;
}

WXD_EXPORTED bool
wxd_CheckListBox_IsChecked(wxd_CheckListBox_t* clbox, unsigned int index)
{
    wxCheckListBox* wxClbox = reinterpret_cast<wxCheckListBox*>(clbox);
    if (wxClbox) {
        // Check index bounds? wxWidgets might assert or behave unexpectedly.
        if (index < wxClbox->GetCount()) {
            return wxClbox->IsChecked(index);
        }
    }
    return false; // Return false if index out of bounds or control invalid
}

WXD_EXPORTED void
wxd_CheckListBox_Check(wxd_CheckListBox_t* clbox, unsigned int index, bool check)
{
    wxCheckListBox* wxClbox = reinterpret_cast<wxCheckListBox*>(clbox);
    if (wxClbox) {
        if (index < wxClbox->GetCount()) {
            wxClbox->Check(index, check);
        }
    }
}

} // extern "C"