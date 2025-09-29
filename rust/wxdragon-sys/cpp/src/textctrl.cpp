#include "wx/wxprec.h"

#ifndef WX_PRECOMP
    #include "wx/wx.h"
#endif

#include "wx/textctrl.h"
#include "wxdragon.h"
#include "wxd_utils.h"

extern "C" {

// Create a new wxTextCtrl
WXD_EXPORTED wxd_TextCtrl_t* wxd_TextCtrl_Create(
    wxd_Window_t* parent, 
    wxd_Id id, 
    const char* value, 
    wxd_Point pos, 
    wxd_Size size, 
    wxd_Style_t style
) {
    wxWindow* parentWin = (wxWindow*)parent;
    wxTextCtrl* ctrl = new wxTextCtrl(
        parentWin, 
        id, 
        wxString::FromUTF8(value ? value : ""),
        wxd_cpp_utils::to_wx(pos),
        wxd_cpp_utils::to_wx(size),
        style
    );
    return (wxd_TextCtrl_t*)ctrl;
}

// Set the value of the wxTextCtrl
WXD_EXPORTED void wxd_TextCtrl_SetValue(wxd_TextCtrl_t* textCtrl, const char* value) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetValue(wxString::FromUTF8(value ? value : ""));
    }
}

// Get the value of the wxTextCtrl
WXD_EXPORTED int wxd_TextCtrl_GetValue(wxd_TextCtrl_t* textCtrl, char* buffer, int buffer_len) {
    if (!textCtrl || !buffer || buffer_len <= 0) return -1;
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    wxString value = ctrl->GetValue();
    return wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, (size_t)buffer_len);
}

// Append text to the wxTextCtrl
WXD_EXPORTED void wxd_TextCtrl_AppendText(wxd_TextCtrl_t* textCtrl, const char* text) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl && text) {
        ctrl->AppendText(wxString::FromUTF8(text));
    }
}

// Clear the wxTextCtrl contents
WXD_EXPORTED void wxd_TextCtrl_Clear(wxd_TextCtrl_t* textCtrl) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->Clear();
    }
}

// Check if the wxTextCtrl has been modified
WXD_EXPORTED bool wxd_TextCtrl_IsModified(wxd_TextCtrl_t* textCtrl) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl) return false;
    return ctrl->IsModified();
}

// Set the modified state of the wxTextCtrl
WXD_EXPORTED void wxd_TextCtrl_SetModified(wxd_TextCtrl_t* textCtrl, bool modified) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetModified(modified);
    }
}

// Make the wxTextCtrl editable or read-only
WXD_EXPORTED void wxd_TextCtrl_SetEditable(wxd_TextCtrl_t* textCtrl, bool editable) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetEditable(editable);
    }
}

// Check if the wxTextCtrl is editable
WXD_EXPORTED bool wxd_TextCtrl_IsEditable(wxd_TextCtrl_t* textCtrl) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl) return false;
    return ctrl->IsEditable();
}

// Get the insertion point position
WXD_EXPORTED wxd_Long_t wxd_TextCtrl_GetInsertionPoint(wxd_TextCtrl_t* textCtrl) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl) return 0;
    return ctrl->GetInsertionPoint();
}

// Set the insertion point position
WXD_EXPORTED void wxd_TextCtrl_SetInsertionPoint(wxd_TextCtrl_t* textCtrl, wxd_Long_t pos) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetInsertionPoint(pos);
    }
}

// Set the maximum length of text that can be entered
WXD_EXPORTED void wxd_TextCtrl_SetMaxLength(wxd_TextCtrl_t* textCtrl, wxd_Long_t len) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetMaxLength(len);
    }
}

// Get the last position in the control (text length)
WXD_EXPORTED wxd_Long_t wxd_TextCtrl_GetLastPosition(wxd_TextCtrl_t* textCtrl) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl) return 0;
    return ctrl->GetLastPosition();
}

// Check if the control is a multiline text control
WXD_EXPORTED bool wxd_TextCtrl_IsMultiLine(wxd_TextCtrl_t* textCtrl) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl) return false;
    return ctrl->IsMultiLine();
}

// Check if the control is a single-line text control
WXD_EXPORTED bool wxd_TextCtrl_IsSingleLine(wxd_TextCtrl_t* textCtrl) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl) return false;
    return ctrl->IsSingleLine();
}

// Selection operations
WXD_EXPORTED void wxd_TextCtrl_SetSelection(wxd_TextCtrl_t* textCtrl, wxd_Long_t from, wxd_Long_t to) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetSelection(from, to);
    }
}

WXD_EXPORTED void wxd_TextCtrl_GetSelection(wxd_TextCtrl_t* textCtrl, wxd_Long_t* from, wxd_Long_t* to) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl && from && to) {
        long from_long, to_long;
        ctrl->GetSelection(&from_long, &to_long);
        *from = static_cast<wxd_Long_t>(from_long);
        *to = static_cast<wxd_Long_t>(to_long);
    }
}

WXD_EXPORTED void wxd_TextCtrl_SelectAll(wxd_TextCtrl_t* textCtrl) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SelectAll();
    }
}

WXD_EXPORTED int wxd_TextCtrl_GetStringSelection(wxd_TextCtrl_t* textCtrl, char* buffer, int buffer_len) {
    if (!textCtrl || !buffer || buffer_len <= 0) return -1;
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    wxString selection = ctrl->GetStringSelection();
    return wxd_cpp_utils::copy_wxstring_to_buffer(selection, buffer, (size_t)buffer_len);
}

} // extern "C" 