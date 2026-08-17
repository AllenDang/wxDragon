#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#if wxdUSE_AUI && wxUSE_AUI
#include <wx/aui/framemanager.h>
#include <wx/aui/auibook.h>
#include <wx/aui/aui.h>

extern "C" {

wxd_AuiMDIParentFrame_t*
wxd_AuiMDIParentFrame_Create(wxd_Window_t* parent, int id, const char* title, wxd_Point pos,
                             wxd_Size size, int64_t style, const char* name)
{
    wxWindow* parentPtr = (wxWindow*)parent;
    wxPoint wxPos = wxPoint(pos.x, pos.y);
    wxSize wxSizeInstance = wxSize(size.width, size.height);
    wxString wxTitle = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title);
    wxString wxName = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(name);

    wxAuiMDIParentFrame* frame =
        new wxAuiMDIParentFrame(parentPtr, id, wxTitle, wxPos, wxSizeInstance, style, wxName);
    return (wxd_AuiMDIParentFrame_t*)frame;
}

}

#endif