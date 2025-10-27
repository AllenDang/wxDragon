#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/aui/framemanager.h> // For wxAuiManager
#include <wx/aui/auibook.h>      // For wxAuiNotebook
#include <wx/aui/aui.h>          // For wxAuiPaneInfo etc. and wxAuiMDIChildFrame

// wxAuiMDIChildFrame is part of <wx/aui/aui.h>

extern "C" {

WXD_EXPORTED wxd_AuiMDIChildFrame_t*
wxd_AuiMDIChildFrame_Create(wxd_AuiMDIParentFrame_t* parent, int id, const char* title,
                            wxd_Point pos, wxd_Size size, int64_t style, const char* name)
{
    wxAuiMDIParentFrame* parentPtr = (wxAuiMDIParentFrame*)parent;
    if (!parentPtr) {
        // It's crucial that the parent is a valid wxAuiMDIParentFrame
        return nullptr;
    }

    wxPoint wxPos = wxPoint(pos.x, pos.y);
    wxSize wxSizeInstance = wxSize(size.width, size.height);
    wxString wxTitle = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title);
    wxString wxName = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(name);

    wxAuiMDIChildFrame* frame =
        new wxAuiMDIChildFrame(parentPtr, id, wxTitle, wxPos, wxSizeInstance, style, wxName);
    return (wxd_AuiMDIChildFrame_t*)frame;
}

// Implementations for other wxAuiMDIChildFrame specific functions will go here.
}