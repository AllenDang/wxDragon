#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"   // For wxd_cpp_utils::to_wx
#include <wx/scrolbar.h> // Include wxScrollBar header

extern "C" {

// Implementation for wxd_ScrollBar_Create
WXDRAGON_API wxd_ScrollBar_t*
wxd_ScrollBar_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                     wxd_Style_t style, const char* name)
{
    if (!parent) {
        return nullptr;
    }
    wxWindow* wx_parent = reinterpret_cast<wxWindow*>(parent);

    wxScrollBar* scrollBar = new wxScrollBar(
        wx_parent, id, wxd_cpp_utils::to_wx(pos), wxd_cpp_utils::to_wx(size), style,
        wxDefaultValidator,                                  // Use default validator
        wxString::FromUTF8(name ? name : wxScrollBarNameStr) // Use default name if NULL
    );

    return reinterpret_cast<wxd_ScrollBar_t*>(scrollBar);
}

// Implementation for wxd_ScrollBar_SetScrollbar
WXDRAGON_API void
wxd_ScrollBar_SetScrollbar(wxd_ScrollBar_t* self, int position, int thumbSize, int range,
                           int pageSize, bool refresh)
{
    wxScrollBar* scrollBar = reinterpret_cast<wxScrollBar*>(self);
    if (scrollBar) {
        scrollBar->SetScrollbar(position, thumbSize, range, pageSize, refresh);
    }
}

// Implementation for wxd_ScrollBar_GetThumbPosition
WXD_EXPORTED int
wxd_ScrollBar_GetThumbPosition(wxd_ScrollBar_t* self)
{
    wxScrollBar* scrollBar = reinterpret_cast<wxScrollBar*>(self);
    if (!scrollBar) {
        return 0; // Or some error indicator, maybe -1?
    }
    return scrollBar->GetThumbPosition();
}

} // extern "C"