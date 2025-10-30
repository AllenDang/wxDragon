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

WXD_EXPORTED size_t
wxd_DirPickerCtrl_GetPath(const wxd_DirPickerCtrl_t* self, char* buffer, size_t buffer_len)
{
    if (!self)
        return 0;
    wxString path_str = ((wxDirPickerCtrl*)self)->GetPath();
    wxScopedCharBuffer utf8_buf = path_str.ToUTF8();

    if (buffer && buffer_len > 0) {
        size_t copy_len = std::min(buffer_len - 1, utf8_buf.length());
        memcpy(buffer, utf8_buf.data(), copy_len);
        buffer[copy_len] = '\0';
    }

    return utf8_buf.length();
}

WXD_EXPORTED void
wxd_DirPickerCtrl_SetPath(wxd_DirPickerCtrl_t* self, const char* path)
{
    if (!self)
        return;
    ((wxDirPickerCtrl*)self)->SetPath(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(path));
}