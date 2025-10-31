#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"
#include <wx/collpane.h>

extern "C" {

WXD_EXPORTED wxd_CollapsiblePane_t*
wxd_CollapsiblePane_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos,
                           wxd_Size size, wxd_Style_t style, const char* name)
{
    wxWindow* parentWindow = reinterpret_cast<wxWindow*>(parent);
    if (!parentWindow)
        return nullptr;

    wxString wxLabel = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(label);
    wxString wxName = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(name);

    wxCollapsiblePane* pane =
        new wxCollapsiblePane(parentWindow, id, wxLabel, wxPoint(pos.x, pos.y),
                              wxSize(size.width, size.height), style, wxDefaultValidator, wxName);

    return reinterpret_cast<wxd_CollapsiblePane_t*>(pane);
}

WXD_EXPORTED bool
wxd_CollapsiblePane_IsExpanded(wxd_CollapsiblePane_t* self)
{
    wxCollapsiblePane* pane = reinterpret_cast<wxCollapsiblePane*>(self);
    if (!pane)
        return false;
    return pane->IsExpanded();
}

WXD_EXPORTED bool
wxd_CollapsiblePane_IsCollapsed(wxd_CollapsiblePane_t* self)
{
    wxCollapsiblePane* pane = reinterpret_cast<wxCollapsiblePane*>(self);
    if (!pane)
        return true;
    return pane->IsCollapsed();
}

WXD_EXPORTED void
wxd_CollapsiblePane_Expand(wxd_CollapsiblePane_t* self, bool expand)
{
    wxCollapsiblePane* pane = reinterpret_cast<wxCollapsiblePane*>(self);
    if (!pane)
        return;
    if (expand) {
        pane->Expand();
    }
    else {
        pane->Collapse();
    }
}

WXD_EXPORTED void
wxd_CollapsiblePane_Collapse(wxd_CollapsiblePane_t* self, bool collapse)
{
    wxCollapsiblePane* pane = reinterpret_cast<wxCollapsiblePane*>(self);
    if (!pane)
        return;
    if (collapse) {
        pane->Collapse();
    }
    else {
        pane->Expand();
    }
}

WXD_EXPORTED wxd_Window_t*
wxd_CollapsiblePane_GetPane(wxd_CollapsiblePane_t* self)
{
    wxCollapsiblePane* pane = reinterpret_cast<wxCollapsiblePane*>(self);
    if (!pane)
        return nullptr;
    wxWindow* paneWindow = pane->GetPane();
    return reinterpret_cast<wxd_Window_t*>(paneWindow);
}

WXD_EXPORTED void
wxd_CollapsiblePane_SetLabel(wxd_CollapsiblePane_t* self, const char* label)
{
    wxCollapsiblePane* pane = reinterpret_cast<wxCollapsiblePane*>(self);
    if (!pane || !label)
        return;
    wxString wxLabel = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(label);
    pane->SetLabel(wxLabel);
}

WXD_EXPORTED int
wxd_CollapsiblePane_GetLabel(const wxd_CollapsiblePane_t* self, char* out, size_t out_len)
{
    const wxCollapsiblePane* pane = reinterpret_cast<const wxCollapsiblePane*>(self);
    if (!pane)
        return -1;

    wxString label = pane->GetLabel();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(label, out, out_len);
}

} // extern "C"