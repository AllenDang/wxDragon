#include "../include/wxdragon.h"
#include <wx/tglbtn.h> // Include toggle button header
#include <wx/window.h>
#include <wx/string.h>
#include <wx/defs.h> // For wxID_ANY, wxDefaultPosition, wxDefaultSize
// #include <string.h> // No longer needed for local strncpy
#include "wxd_utils.h" // For wxd_cpp_utils::copy_wxstring_to_buffer

// Helper function (duplicate from other files, consider common utils)
// REMOVED: static int copy_wxstring_to_buffer(...) - Now using wxd_cpp_utils version

// --- wxToggleButton C Functions ---

extern "C" {

WXD_EXPORTED wxd_ToggleButton_t* wxd_ToggleButton_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    const char* label,
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxPoint wxPos(pos.x, pos.y);
    wxSize wxSize(size.width, size.height);
    wxString wxLabel = wxString::FromUTF8(label ? label : "");

    wxToggleButton* tglbtn = new wxToggleButton(parentWin, id, wxLabel, wxPos, wxSize, style);

    wxd_Window_AttachCleanupNotifier(reinterpret_cast<wxd_Window_t*>(tglbtn));
    return reinterpret_cast<wxd_ToggleButton_t*>(tglbtn);
}

WXD_EXPORTED bool wxd_ToggleButton_GetValue(wxd_ToggleButton_t* tglbtn) {
    wxToggleButton* wxTglBtn = reinterpret_cast<wxToggleButton*>(tglbtn);
    if (wxTglBtn) {
        return wxTglBtn->GetValue();
    }
    return false;
}

WXD_EXPORTED void wxd_ToggleButton_SetValue(wxd_ToggleButton_t* tglbtn, bool state) {
    wxToggleButton* wxTglBtn = reinterpret_cast<wxToggleButton*>(tglbtn);
    if (wxTglBtn) {
        wxTglBtn->SetValue(state);
    }
}

WXD_EXPORTED void wxd_ToggleButton_SetLabel(wxd_ToggleButton_t* tglbtn, const char* label) {
    wxToggleButton* wxTglBtn = reinterpret_cast<wxToggleButton*>(tglbtn);
    if (wxTglBtn && label) {
        wxTglBtn->SetLabel(wxString::FromUTF8(label));
    }
}

WXD_EXPORTED int wxd_ToggleButton_GetLabel(wxd_ToggleButton_t* tglbtn, char* buffer, int buffer_len) {
    wxToggleButton* wxTglBtn = reinterpret_cast<wxToggleButton*>(tglbtn);
    if (wxTglBtn) {
        return static_cast<int>(wxd_cpp_utils::copy_wxstring_to_buffer(wxTglBtn->GetLabel(), buffer, static_cast<size_t>(buffer_len)));
    }
    if (buffer && buffer_len > 0) buffer[0] = '\0';
    return 0;
}

} // extern "C" 