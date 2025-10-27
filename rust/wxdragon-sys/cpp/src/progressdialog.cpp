#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wxdragon.h"
#include <wx/progdlg.h>

extern "C" {

wxd_ProgressDialog_t*
wxd_ProgressDialog_Create(wxd_Window_t* parent, const char* title, const char* message, int maximum,
                          wxd_Style_t style)
{
    wxWindow* wxParent = reinterpret_cast<wxWindow*>(parent);
    wxString wxTitle = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title);
    wxString wxMessage = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message);

    wxProgressDialog* dialog = new wxProgressDialog(wxTitle, wxMessage, maximum, wxParent, style);

    return reinterpret_cast<wxd_ProgressDialog_t*>(dialog);
}

bool
wxd_ProgressDialog_Update(wxd_ProgressDialog_t* self, int value, const char* newmsg, bool* skip)
{
    wxProgressDialog* dialog = reinterpret_cast<wxProgressDialog*>(self);
    wxString wxNewMsg;
    if (newmsg) {
        wxNewMsg = wxString::FromUTF8(newmsg);
    }

    bool skipResult = false;
    bool result = dialog->Update(value, wxNewMsg, &skipResult);

    if (skip) {
        *skip = skipResult;
    }

    return result;
}

bool
wxd_ProgressDialog_Pulse(wxd_ProgressDialog_t* self, const char* newmsg, bool* skip)
{
    wxProgressDialog* dialog = reinterpret_cast<wxProgressDialog*>(self);
    wxString wxNewMsg;
    if (newmsg) {
        wxNewMsg = wxString::FromUTF8(newmsg);
    }

    bool skipResult = false;
    bool result = dialog->Pulse(wxNewMsg, &skipResult);

    if (skip) {
        *skip = skipResult;
    }

    return result;
}

void
wxd_ProgressDialog_Resume(wxd_ProgressDialog_t* self)
{
    wxProgressDialog* dialog = reinterpret_cast<wxProgressDialog*>(self);
    dialog->Resume();
}

int
wxd_ProgressDialog_GetValue(wxd_ProgressDialog_t* self)
{
    wxProgressDialog* dialog = reinterpret_cast<wxProgressDialog*>(self);
    return dialog->GetValue();
}

int
wxd_ProgressDialog_GetRange(wxd_ProgressDialog_t* self)
{
    wxProgressDialog* dialog = reinterpret_cast<wxProgressDialog*>(self);
    return dialog->GetRange();
}

bool
wxd_ProgressDialog_WasCancelled(wxd_ProgressDialog_t* self)
{
    wxProgressDialog* dialog = reinterpret_cast<wxProgressDialog*>(self);
    return dialog->WasCancelled();
}

bool
wxd_ProgressDialog_WasSkipped(wxd_ProgressDialog_t* self)
{
    wxProgressDialog* dialog = reinterpret_cast<wxProgressDialog*>(self);
    return dialog->WasSkipped();
}

} // extern "C"