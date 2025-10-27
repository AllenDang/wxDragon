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

WXD_EXPORTED const char*
wxd_DirPickerCtrl_GetPath(wxd_DirPickerCtrl_t* self)
{
    if (!self)
        return NULL;
    wxString path_str = ((wxDirPickerCtrl*)self)->GetPath();
    wxScopedCharBuffer utf8_buf = path_str.ToUTF8();
    if (!utf8_buf.data() || strlen(utf8_buf.data()) == 0) { // Check for empty or null buffer
        // wxWidgets GetPath might return empty string, strdup("") is fine
        return strdup("");
    }
    return strdup(utf8_buf.data());
}

WXD_EXPORTED void
wxd_DirPickerCtrl_SetPath(wxd_DirPickerCtrl_t* self, const char* path)
{
    if (!self)
        return;
    ((wxDirPickerCtrl*)self)->SetPath(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(path));
}