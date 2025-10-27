#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h" // For wxd_FilePickerCtrl_t and API declarations
#include "wxd_utils.h"           // For WXD_STR_TO_WX_STRING_UTF8_NULL_OK

#include <wx/filepicker.h> // For wxFilePickerCtrl
#include <cstring>         // For strdup

// --- FilePickerCtrl ---
WXD_EXPORTED wxd_FilePickerCtrl_t*
wxd_FilePickerCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* message,
                          const char* wildcard, const char* path, wxd_Point pos, wxd_Size size,
                          wxd_Style_t style)
{
    return (wxd_FilePickerCtrl_t*)new wxFilePickerCtrl(
        (wxWindow*)parent, id, WXD_STR_TO_WX_STRING_UTF8_NULL_OK(path),
        WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message), WXD_STR_TO_WX_STRING_UTF8_NULL_OK(wildcard),
        wxPoint(pos.x, pos.y), wxSize(size.width, size.height), style, wxDefaultValidator,
        wxFilePickerCtrlNameStr // Default name, can be overridden by parent class methods if exposed
    );
}

WXD_EXPORTED const char*
wxd_FilePickerCtrl_GetPath(wxd_FilePickerCtrl_t* self)
{
    if (!self)
        return NULL;
    wxString path = ((wxFilePickerCtrl*)self)->GetPath();
    wxScopedCharBuffer utf8_buf = path.ToUTF8();
    if (!utf8_buf.data()) {
        return strdup("");
    }
    return strdup(utf8_buf.data());
}

WXD_EXPORTED void
wxd_FilePickerCtrl_SetPath(wxd_FilePickerCtrl_t* self, const char* path)
{
    if (!self)
        return;
    ((wxFilePickerCtrl*)self)->SetPath(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(path));
}