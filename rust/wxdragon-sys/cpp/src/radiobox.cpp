#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"
#include <wx/radiobox.h>
#include <wx/arrstr.h>

extern "C" {

WXD_EXPORTED wxd_RadioBox_t*
wxd_RadioBox_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos,
                    wxd_Size size, int n, const char* const* choices, int majorDimension,
                    wxd_Style_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    if (!parentWin)
        return nullptr;

    wxArrayString wxChoices;
    for (int i = 0; i < n; ++i) {
        if (choices[i]) {
            wxChoices.Add(wxString::FromUTF8(choices[i]));
        }
        else {
            wxChoices.Add(wxEmptyString); // Add empty string if null encountered
        }
    }

    wxRadioBox* rbox = new wxRadioBox(parentWin, id, wxString::FromUTF8(label ? label : ""),
                                      wxd_cpp_utils::to_wx(pos), wxd_cpp_utils::to_wx(size),
                                      wxChoices, majorDimension, style);

    return (wxd_RadioBox_t*)rbox;
}

WXD_EXPORTED int
wxd_RadioBox_GetSelection(wxd_RadioBox_t* self)
{
    wxRadioBox* rbox = (wxRadioBox*)self;
    if (!rbox)
        return wxNOT_FOUND;
    return rbox->GetSelection();
}

WXD_EXPORTED void
wxd_RadioBox_SetSelection(wxd_RadioBox_t* self, int n)
{
    wxRadioBox* rbox = (wxRadioBox*)self;
    if (rbox) {
        rbox->SetSelection(n);
    }
}

WXD_EXPORTED int
wxd_RadioBox_GetString(wxd_RadioBox_t* self, int n, char* buffer, size_t buffer_len)
{
    if (!self)
        return -1;
    wxRadioBox* rbox = (wxRadioBox*)self;
    if (n < 0 || (unsigned int)n >= rbox->GetCount())
        return -1;

    wxString item = rbox->GetString((unsigned int)n);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(item, buffer, buffer_len);
}

WXD_EXPORTED unsigned int
wxd_RadioBox_GetCount(wxd_RadioBox_t* self)
{
    wxRadioBox* rbox = (wxRadioBox*)self;
    if (!rbox)
        return 0;
    return rbox->GetCount();
}

WXD_EXPORTED bool
wxd_RadioBox_EnableItem(wxd_RadioBox_t* self, int n, bool enable)
{
    wxRadioBox* rbox = (wxRadioBox*)self;
    if (!rbox)
        return false;
    return rbox->Enable(n, enable);
}

WXD_EXPORTED bool
wxd_RadioBox_IsItemEnabled(wxd_RadioBox_t* self, int n)
{
    wxRadioBox* rbox = (wxRadioBox*)self;
    if (!rbox)
        return false;
    return rbox->IsItemEnabled(n);
}

WXD_EXPORTED bool
wxd_RadioBox_ShowItem(wxd_RadioBox_t* self, int n, bool show)
{
    wxRadioBox* rbox = (wxRadioBox*)self;
    if (!rbox)
        return false;
    return rbox->Show(n, show);
}

WXD_EXPORTED bool
wxd_RadioBox_IsItemShown(wxd_RadioBox_t* self, int n)
{
    wxRadioBox* rbox = (wxRadioBox*)self;
    if (!rbox)
        return false;
    return rbox->IsItemShown(n);
}

// Destroy handled by parent window

} // extern "C"