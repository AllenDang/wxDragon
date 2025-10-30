#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wxdragon.h"
#include "wxd_utils.h" // For colour conversion helpers
#include <wx/hyperlink.h>
#include <wx/string.h>
#include <wx/gdicmn.h> // For wxPoint, wxSize
#include <wx/colour.h> // For wxColour

// Keep track of allocated C-strings for GetURL so they can be freed later by Rust if needed,
// or adopt a strategy where Rust always copies the string.
// For now, we'll return a pointer to wxString's internal buffer. This is generally unsafe
// if the wxString is temporary or modified. A safer approach is for Rust to provide a buffer.
// However, GetURL is typically followed by an immediate copy in Rust.
// Let's assume wxString returned by GetURL() lives as long as the control.
// A common pattern for C APIs is to return const char* that the caller must copy.
// wxString::ToUTF8() returns a temporary wxCharBuffer. We need to manage its lifetime.
// A simple way for now is to have static buffers, but this is not thread-safe or re-entrant.
// A better C API would involve the caller providing the buffer.
// Given the existing patterns (e.g. wxd_TextCtrl_GetValue), they expect the Rust side to manage this.
// For GetURL, wxHyperlinkCtrl::GetURL() returns a const wxString&.
// wxString::mb_str() or wxString::ToUTF8() can be used. ToUTF8() is preferred.
// The result of ToUTF8() is a wxCharBuffer, which owns the memory.
// To return a `const char*` that's valid after the function returns, we'd typically
// need to `strdup` it or use a static buffer (bad).
// For wxWidgets, `wxString::utf8_str()` returns `wxScopedCharBuffer` which has `data()` method.
// The `wxScopedCharBuffer` itself must not go out of scope.
// Let's follow the pattern potentially used in `wxd_Frame_GetTitle`
// which likely rely on wxString's internal buffer or a temporary buffer which Rust copies immediately.
// For now, we'll return `link->GetURL().ToUTF8().data()`. This is risky if not copied immediately.
// A robust solution would be for Rust to allocate and pass a buffer.

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

// This buffer is not thread-safe. For a real application, Rust should pass a buffer.
// Or the Rust side must immediately copy the returned const char*.
static wxCharBuffer s_get_url_buffer;

WXD_EXPORTED const char*
wxd_HyperlinkCtrl_GetURL(wxd_HyperlinkCtrl_t* self)
{
    wxHyperlinkCtrl* link = (wxHyperlinkCtrl*)self;
    if (!link)
        return NULL;
    s_get_url_buffer = link->GetURL().ToUTF8(); // ToUTF8() result assigned to static buffer
    return s_get_url_buffer.data();             // data() should now be valid
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