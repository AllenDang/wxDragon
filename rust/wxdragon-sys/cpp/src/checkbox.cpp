#include "wx/wxprec.h"

#ifndef WX_PRECOMP
#include "wx/wx.h"
#endif

#include "wx/checkbox.h"
#include "wxdragon.h"

extern "C" {

// Create a new wxCheckBox
WXD_EXPORTED wxd_CheckBox_t*
wxd_CheckBox_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos,
                    wxd_Size size, wxd_Style_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    wxCheckBox* ctrl = new wxCheckBox(parentWin, id, wxString::FromUTF8(label),
                                      wxPoint(pos.x, pos.y), wxSize(size.width, size.height),
                                      style);
    return (wxd_CheckBox_t*)ctrl;
}

// Get the checked state of the wxCheckBox
WXD_EXPORTED bool
wxd_CheckBox_IsChecked(wxd_CheckBox_t* checkBox)
{
    wxCheckBox* ctrl = (wxCheckBox*)checkBox;
    if (!ctrl) {
        return false; // Or handle error differently
    }
    return ctrl->IsChecked();
}

// Set the checked state of the wxCheckBox
WXD_EXPORTED void
wxd_CheckBox_SetValue(wxd_CheckBox_t* checkBox, bool value)
{
    wxCheckBox* ctrl = (wxCheckBox*)checkBox;
    if (ctrl) {
        ctrl->SetValue(value);
    }
}

} // extern "C"