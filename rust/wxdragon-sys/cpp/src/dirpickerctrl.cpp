#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h" // Main header for WXD_EXPORTED, types, and wxd_pickers.h
#include "wxd_utils.h"           // For WXD_STR_TO_WX_STRING_UTF8_NULL_OK

#include <wx/filepicker.h> // For wxDirPickerCtrl (it's in this header with wxFilePickerCtrl)
#include <cstring>         // For strdup

// --- DirPickerCtrl ---
WXD_EXPORTED wxd_DirPickerCtrl_t*
wxd_DirPickerCtrl_Create(wxd_Window_t* parent, wxd_Id id,
                         const char* message, // Label for the dialog invoke button
                         const char* path,    // Initial path
                         wxd_Point pos, wxd_Size size, wxd_Style_t style)
{
    wxString wx_path = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(path);
    wxString wx_message;
    if (message) {
        wx_message = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message);
    }
    else {
        wx_message = wxDirSelectorPromptStr;
    }

    return (wxd_DirPickerCtrl_t*)new wxDirPickerCtrl((wxWindow*)parent, id,
                                                     wx_path,    // path
                                                     wx_message, // message for dialog
                                                     wxPoint(pos.x, pos.y),
                                                     wxSize(size.width, size.height), style,
                                                     wxDefaultValidator, wxDirPickerCtrlNameStr);
}

WXD_EXPORTED int
wxd_DirPickerCtrl_GetPath(const wxd_DirPickerCtrl_t* self, char* buffer, size_t buffer_len)
{
    if (!self)
        return -1;
    wxString path_str = ((wxDirPickerCtrl*)self)->GetPath();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(path_str, buffer, buffer_len);
}

WXD_EXPORTED void
wxd_DirPickerCtrl_SetPath(wxd_DirPickerCtrl_t* self, const char* path)
{
    if (!self)
        return;
    ((wxDirPickerCtrl*)self)->SetPath(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(path));
}