#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/aui/framemanager.h>
#include <wx/aui/auibook.h>
#include <wx/aui/aui.h>

// Helper macro for string conversion, WXD_STR_TO_WX_STRING_UTF8_NULL_OK is available via wxd_utils.h in wxdragon.h
// but can be defined locally if preferred or for standalone testing.
// #define WXD_STR_TO_WX_STRING_UTF8_NULL_OK(str) (str ? wxString::FromUTF8(str) : wxString())

extern "C" {

wxd_AuiMDIParentFrame_t*
wxd_AuiMDIParentFrame_Create(wxd_Window_t* parent, int id, const char* title, wxd_Point pos,
                             wxd_Size size, int64_t style, const char* name)
{
    wxWindow* parentPtr = (wxWindow*)parent;
    wxPoint wxPos = wxPoint(pos.x, pos.y);
    wxSize wxSizeInstance = wxSize(size.width, size.height);
    // Use the macro from wxd_utils.h (included via wxdragon.h) for robust null handling and UTF-8 conversion.
    wxString wxTitle = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title);
    wxString wxName = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(name);

    wxAuiMDIParentFrame* frame =
        new wxAuiMDIParentFrame(parentPtr, id, wxTitle, wxPos, wxSizeInstance, style, wxName);
    return (wxd_AuiMDIParentFrame_t*)frame;
}

// Implementations for other wxAuiMDIParentFrame specific functions will go here.
}