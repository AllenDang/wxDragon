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

// Implementations for other wxd_FileCtrl_XXX functions will go here
// Example:
/*
const char* wxd_FileCtrl_GetPath(wxd_FileCtrl_t* self) {
    wxFileCtrl* ctrl = (wxFileCtrl*)self;
    if (!ctrl) return nullptr;
    return wxd_wx_string_to_c_str(ctrl->GetPath());
}

void wxd_FileCtrl_SetPath(wxd_FileCtrl_t* self, const char* path) {
    wxFileCtrl* ctrl = (wxFileCtrl*)self;
    if (!ctrl) return;
    ctrl->SetPath(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(path));
}
*/

} // extern "C"