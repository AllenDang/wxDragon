#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wx/radiobut.h" // Include wxRadioButton header
#include "wx/window.h"
#include "wx/string.h" // For wxString
#include "wxdragon.h"  // Include our C API header

extern "C" {

WXD_EXPORTED wxd_RadioButton_t*
wxd_RadioButton_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos,
                       wxd_Size size, wxd_Style_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    if (!parentWin)
        return nullptr;

    wxString wxLabel = wxString::FromUTF8(label ? label : "");
    wxPoint wxPos(pos.x, pos.y);
    wxSize wxSize(size.width, size.height);

    // Create the wxRadioButton instance
    wxRadioButton* radio = new wxRadioButton(parentWin, id, wxLabel, wxPos, wxSize, style);

    return (wxd_RadioButton_t*)radio;
}

WXD_EXPORTED bool
wxd_RadioButton_GetValue(wxd_RadioButton_t* radio)
{
    wxRadioButton* rb = (wxRadioButton*)radio;
    if (!rb)
        return false;
    return rb->GetValue();
}

WXD_EXPORTED void
wxd_RadioButton_SetValue(wxd_RadioButton_t* radio, bool value)
{
    wxRadioButton* rb = (wxRadioButton*)radio;
    if (rb) {
        rb->SetValue(value);
    }
}

// Note: No explicit Destroy function needed. Radio buttons are child controls,
// destroyed when their parent window is destroyed.

} // extern "C"