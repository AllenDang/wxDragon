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

    // Default position/size if not specified
    wxPoint pos = (x == -1 && y == -1) ? wxDefaultPosition : wxPoint(x, y);

    wxSingleChoiceDialog* dialog =
        new wxSingleChoiceDialog(parent_wx, WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message),
                                 WXD_STR_TO_WX_STRING_UTF8_NULL_OK(caption), *wxChoices,
                                 nullptr, // Client data
                                 style);

    // Set position and size if provided
    if (x != -1 && y != -1) {
        dialog->SetPosition(pos);
    }
    if (width != -1 && height != -1) {
        dialog->SetSize(width, height);
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