#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"   // For wxd_cpp_utils::to_wx
#include <wx/statline.h> // Include wxStaticLine header

extern "C" {

WXDRAGON_API wxd_StaticLine_t*
wxd_StaticLine_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                      wxd_Style_t style, const char* name)
{
    if (!parent) {
        return nullptr;
    }
    wxWindow* wx_parent = reinterpret_cast<wxWindow*>(parent);

    wxStaticLine* sline = new wxStaticLine(
        wx_parent, id, wxd_cpp_utils::to_wx(pos), wxd_cpp_utils::to_wx(size), style,
        wxString::FromUTF8(name ? name : wxStaticLineNameStr) // Use default name if NULL
    );

    return reinterpret_cast<wxd_StaticLine_t*>(sline);
}

} // extern "C"