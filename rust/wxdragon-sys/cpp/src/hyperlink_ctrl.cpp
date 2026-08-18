#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wxdragon.h"
#include "wxd_utils.h" // For colour conversion helpers
#include <wx/hyperlink.h>
#include <wx/string.h>
#include <wx/gdicmn.h> // For wxPoint, wxSize
#include <wx/colour.h> // For wxColour

WXD_EXPORTED wxd_HyperlinkCtrl_t*
wxd_HyperlinkCtrl_Create(wxd_Window_t* parent, int id, const char* label, const char* url, int x,
                         int y, int w, int h, int64_t style)
{
    wxWindow* p = (wxWindow*)parent;
    wxString wxLabel = wxString::FromUTF8(label);
    wxString wxUrl = wxString::FromUTF8(url);
    wxPoint pos = (x == -1 && y == -1) ? wxDefaultPosition : wxPoint(x, y);
    wxSize size = (w == -1 && h == -1) ? wxDefaultSize : wxSize(w, h);

    wxHyperlinkCtrl* link = new wxHyperlinkCtrl(p, id, wxLabel, wxUrl, pos, size, style);
    return (wxd_HyperlinkCtrl_t*)link;
}

WXD_EXPORTED int
wxd_HyperlinkCtrl_GetURL(const wxd_HyperlinkCtrl_t* self, char* buf, size_t buf_len)
{
    wxHyperlinkCtrl* link = (wxHyperlinkCtrl*)self;
    if (!link)
        return -1;
    wxString url = link->GetURL();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(url, buf, buf_len);
}

WXD_EXPORTED void
wxd_HyperlinkCtrl_SetURL(wxd_HyperlinkCtrl_t* self, const char* url)
{
    wxHyperlinkCtrl* link = (wxHyperlinkCtrl*)self;
    if (!link)
        return;
    link->SetURL(wxString::FromUTF8(url));
}

WXD_EXPORTED bool
wxd_HyperlinkCtrl_GetVisited(wxd_HyperlinkCtrl_t* self)
{
    wxHyperlinkCtrl* link = (wxHyperlinkCtrl*)self;
    if (!link)
        return false;
    return link->GetVisited();
}

WXD_EXPORTED void
wxd_HyperlinkCtrl_SetVisited(wxd_HyperlinkCtrl_t* self, bool visited)
{
    wxHyperlinkCtrl* link = (wxHyperlinkCtrl*)self;
    if (!link)
        return;
    link->SetVisited(visited);
}

WXD_EXPORTED unsigned long
wxd_HyperlinkCtrl_GetHoverColour(wxd_HyperlinkCtrl_t* self)
{
    wxHyperlinkCtrl* link = (wxHyperlinkCtrl*)self;
    if (!link)
        return 0;
    return wxColourToWxdColour(link->GetHoverColour());
}

WXD_EXPORTED void
wxd_HyperlinkCtrl_SetHoverColour(wxd_HyperlinkCtrl_t* self, unsigned long colour)
{
    wxHyperlinkCtrl* link = (wxHyperlinkCtrl*)self;
    if (!link)
        return;
    link->SetHoverColour(wxdColourToWxColour(colour));
}

WXD_EXPORTED unsigned long
wxd_HyperlinkCtrl_GetNormalColour(wxd_HyperlinkCtrl_t* self)
{
    wxHyperlinkCtrl* link = (wxHyperlinkCtrl*)self;
    if (!link)
        return 0;
    return wxColourToWxdColour(link->GetNormalColour());
}

WXD_EXPORTED void
wxd_HyperlinkCtrl_SetNormalColour(wxd_HyperlinkCtrl_t* self, unsigned long colour)
{
    wxHyperlinkCtrl* link = (wxHyperlinkCtrl*)self;
    if (!link)
        return;
    link->SetNormalColour(wxdColourToWxColour(colour));
}

WXD_EXPORTED unsigned long
wxd_HyperlinkCtrl_GetVisitedColour(wxd_HyperlinkCtrl_t* self)
{
    wxHyperlinkCtrl* link = (wxHyperlinkCtrl*)self;
    if (!link)
        return 0;
    return wxColourToWxdColour(link->GetVisitedColour());
}

WXD_EXPORTED void
wxd_HyperlinkCtrl_SetVisitedColour(wxd_HyperlinkCtrl_t* self, unsigned long colour)
{
    wxHyperlinkCtrl* link = (wxHyperlinkCtrl*)self;
    if (!link)
        return;
    link->SetVisitedColour(wxdColourToWxColour(colour));
}