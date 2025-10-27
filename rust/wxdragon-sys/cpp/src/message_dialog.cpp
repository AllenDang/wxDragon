#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wxdragon.h"
#include "wx/msgdlg.h" // For wxMessageDialog

extern "C" {

wxd_MessageDialog*
wxd_MessageDialog_Create(wxd_Window_t* parent, const char* message, const char* caption,
                         int64_t style)
{
    wxWindow* wx_parent = (wxWindow*)parent;
    wxString wx_message = wxString::FromUTF8(message ? message : "");
    wxString wx_caption = wxString::FromUTF8(caption ? caption : "");

    // wxMessageDialog constructor: wxWindow *parent, const wxString &message,
    //                            const wxString &caption = wxMessageBoxCaptionStr,
    //                            long style = wxOK | wxCENTRE,
    //                            const wxPoint &pos = wxDefaultPosition
    // We don't expose pos for MessageDialog in C API, wxDefaultPosition is fine.
    wxMessageDialog* dlg = new wxMessageDialog(wx_parent, wx_message, wx_caption, style);
    return (wxd_MessageDialog*)dlg;
}

// ShowModal is handled by wxd_Dialog_ShowModal((wxd_Dialog*)dlg_ptr)
// Destroy is handled by wxd_Window_Destroy((wxd_Window_t*)dlg_ptr)

} // extern "C"