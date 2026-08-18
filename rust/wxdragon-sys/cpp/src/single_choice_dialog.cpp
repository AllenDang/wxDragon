#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"

#include <wx/choicdlg.h>

WXD_EXPORTED wxd_SingleChoiceDialog_t*
wxd_SingleChoiceDialog_Create(const wxd_Window_t* parent, const char* message, const char* caption,
                              const wxd_ArrayString_t* choices, wxd_Style_t style, int x, int y,
                              int width, int height)
{
    wxWindow* parent_wx = (wxWindow*)parent;
    const wxArrayString* wxChoices = reinterpret_cast<const wxArrayString*>(choices);

    wxPoint pos = wxd_cpp_utils::to_wx(wxd_Point{x, y});
    wxSize size = wxd_cpp_utils::to_wx(wxd_Size{width, height});

    wxSingleChoiceDialog* dialog =
        new wxSingleChoiceDialog(parent_wx, WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message),
                                 WXD_STR_TO_WX_STRING_UTF8_NULL_OK(caption), *wxChoices,
                                 nullptr, // Client data
                                 style);

    // Set position/size if provided. Use OR (not AND) so that specifying just
    // one axis (e.g. x=-1, y=100) still applies - only skip when BOTH axes of
    // a pair are the -1 "unspecified" sentinel, matching the sentinel checks above.
    if (x != -1 || y != -1) {
        dialog->SetPosition(pos);
    }
    if (width != -1 || height != -1) {
        dialog->SetSize(size.GetWidth(), size.GetHeight());
    }

    return reinterpret_cast<wxd_SingleChoiceDialog_t*>(dialog);
}

WXD_EXPORTED int
wxd_SingleChoiceDialog_GetSelection(const wxd_SingleChoiceDialog_t* self)
{
    if (!self)
        return -1;
    const wxSingleChoiceDialog* dialog = reinterpret_cast<const wxSingleChoiceDialog*>(self);
    return dialog->GetSelection();
}

WXD_EXPORTED void
wxd_SingleChoiceDialog_SetSelection(wxd_SingleChoiceDialog_t* self, int selection)
{
    if (!self)
        return;
    wxSingleChoiceDialog* dialog = reinterpret_cast<wxSingleChoiceDialog*>(self);
    dialog->SetSelection(selection);
}

WXD_EXPORTED int
wxd_SingleChoiceDialog_GetStringSelection(const wxd_SingleChoiceDialog_t* self, char* buffer,
                                          size_t bufLen)
{
    if (!self)
        return -1;
    const wxSingleChoiceDialog* dialog = reinterpret_cast<const wxSingleChoiceDialog*>(self);
    wxString val = dialog->GetStringSelection();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(val, buffer, bufLen);
}