#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/filectrl.h> // For wxFileCtrl

extern "C" {

wxd_FileCtrl_t*
wxd_FileCtrl_Create(wxd_Window_t* parent, int id, const char* default_directory,
                    const char* default_filename, const char* wild_card, int64_t style, int pos_x,
                    int pos_y, int size_w, int size_h, const char* name)
{
    wxWindow* parent_ptr = (wxWindow*)parent;
    wxString wx_default_directory = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(default_directory);
    wxString wx_default_filename = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(default_filename);
    // wxFileCtrl uses wxALL_FILES_PATTERN ("*.*") if wild_card is empty,
    // so WXD_STR_TO_WX_STRING_UTF8_NULL_OK handles this correctly if wild_card is NULL or empty.
    wxString wx_wild_card = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(wild_card);
    wxString wx_name = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(name);

    wxFileCtrl* ctrl = new wxFileCtrl(parent_ptr, id, wx_default_directory, wx_default_filename,
                                      wx_wild_card, style, wxPoint(pos_x, pos_y),
                                      wxSize(size_w, size_h), wx_name);
    return (wxd_FileCtrl_t*)ctrl;
}

WXD_EXPORTED size_t
wxd_FileCtrl_GetPath(const wxd_FileCtrl_t* self, char* buffer, size_t buffer_len)
{
    wxFileCtrl* ctrl = (wxFileCtrl*)self;
    if (!ctrl)
        return 0;

    wxString path = ctrl->GetPath();
    wxScopedCharBuffer pathBuf = path.ToUTF8();
    if (buffer && buffer_len > 0) {
        size_t copyLen = wxMin(static_cast<size_t>(pathBuf.length()), buffer_len - 1);
        memcpy(buffer, pathBuf.data(), copyLen);
        buffer[copyLen] = '\0'; // Null-terminate the buffer
    }
    return pathBuf.length();
}

// Implementations for other wxd_FileCtrl_XXX functions will go here
// Example:
/*

void wxd_FileCtrl_SetPath(wxd_FileCtrl_t* self, const char* path) {
    wxFileCtrl* ctrl = (wxFileCtrl*)self;
    if (!ctrl) return;
    ctrl->SetPath(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(path));
}
*/

} // extern "C"