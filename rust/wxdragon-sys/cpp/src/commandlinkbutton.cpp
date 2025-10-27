#include <wx/wxprec.h>
#include <wx/wx.h>
#include <wx/commandlinkbutton.h>
#include "wxdragon.h"

extern "C" {

// --- wxCommandLinkButton ---
WXD_EXPORTED wxd_CommandLinkButton_t*
wxd_CommandLinkButton_Create(wxd_Window_t* parent, wxd_Id id, const char* mainLabel,
                             const char* note, wxd_Point pos, wxd_Size size, wxd_Style_t style)
{
    wxWindow* wx_parent = (wxWindow*)parent;
    wxCommandLinkButton* wx_button =
        new wxCommandLinkButton(wx_parent, id, wxString::FromUTF8(mainLabel ? mainLabel : ""),
                                wxString::FromUTF8(note ? note : ""), wxPoint(pos.x, pos.y),
                                wxSize(size.width, size.height), style);
    return (wxd_CommandLinkButton_t*)wx_button;
}

WXD_EXPORTED void
wxd_CommandLinkButton_SetNote(wxd_CommandLinkButton_t* self, const char* note)
{
    wxCommandLinkButton* wx_button = (wxCommandLinkButton*)self;
    if (wx_button) {
        wx_button->SetNote(wxString::FromUTF8(note ? note : ""));
    }
}

} // extern "C"