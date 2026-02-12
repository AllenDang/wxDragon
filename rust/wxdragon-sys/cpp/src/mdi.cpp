#include <wx/wxprec.h>
#include <wx/wx.h>
#include <wx/mdi.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"

extern "C" {

WXD_EXPORTED wxd_Frame_t*
wxd_MDIParentFrame_Create(wxd_Window_t* parent, wxd_Id id, const char* title, wxd_Point pos,
                          wxd_Size size, wxd_Style_t style, const char* name)
{
    wxMDIParentFrame* frame = new wxMDIParentFrame(
        reinterpret_cast<wxWindow*>(parent), id, 
        wxString::FromUTF8(title ? title : ""),
        wxd_cpp_utils::to_wx(pos), wxd_cpp_utils::to_wx(size),
        style, wxString::FromUTF8(name ? name : "")
    );
    return reinterpret_cast<wxd_Frame_t*>(frame);
}

WXD_EXPORTED wxd_Frame_t*
wxd_MDIChildFrame_Create(wxd_Frame_t* parent, wxd_Id id, const char* title, wxd_Point pos,
                         wxd_Size size, wxd_Style_t style, const char* name)
{
    wxMDIParentFrame* parentFrame = reinterpret_cast<wxMDIParentFrame*>(parent);
    wxMDIChildFrame* frame = new wxMDIChildFrame(
        parentFrame, id, 
        wxString::FromUTF8(title ? title : ""),
        wxd_cpp_utils::to_wx(pos), wxd_cpp_utils::to_wx(size),
        style, wxString::FromUTF8(name ? name : "")
    );
    return reinterpret_cast<wxd_Frame_t*>(frame);
}

WXD_EXPORTED wxd_Window_t*
wxd_MDIParentFrame_GetClientWindow(wxd_Frame_t* frame)
{
    wxMDIParentFrame* mdiParent = reinterpret_cast<wxMDIParentFrame*>(frame);
    if (mdiParent) {
        return reinterpret_cast<wxd_Window_t*>(mdiParent->GetClientWindow());
    }
    return nullptr;
}

}
