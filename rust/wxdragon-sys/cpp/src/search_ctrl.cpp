#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wx/srchctrl.h"
#include "../include/wxdragon.h"
#include "wx/string.h" // Ensure wxString is available for FromUTF8

extern "C" {

// wxSearchCtrl
WXD_EXPORTED wxd_SearchCtrl_t*
wxd_SearchCtrl_Create(wxd_Window_t* parent, int id, const char* value, int x, int y, int width,
                      int height, int64_t style)
{
    wxWindow* wx_parent = (wxWindow*)parent;
    wxSearchCtrl* ctrl = new wxSearchCtrl(wx_parent, id, wxString::FromUTF8(value ? value : ""),
                                          wxPoint(x, y), wxSize(width, height), style);
    return (wxd_SearchCtrl_t*)ctrl;
}

WXD_EXPORTED void
wxd_SearchCtrl_ShowSearchButton(wxd_SearchCtrl_t* searchCtrl, bool show)
{
    wxSearchCtrl* ctrl = (wxSearchCtrl*)searchCtrl;
    if (ctrl) {
        ctrl->ShowSearchButton(show);
    }
}

WXD_EXPORTED bool
wxd_SearchCtrl_IsSearchButtonVisible(wxd_SearchCtrl_t* searchCtrl)
{
    wxSearchCtrl* ctrl = (wxSearchCtrl*)searchCtrl;
    if (ctrl) {
        return ctrl->IsSearchButtonVisible();
    }
    return false; // Or some other appropriate default for null ctrl
}

WXD_EXPORTED void
wxd_SearchCtrl_ShowCancelButton(wxd_SearchCtrl_t* searchCtrl, bool show)
{
    wxSearchCtrl* ctrl = (wxSearchCtrl*)searchCtrl;
    if (ctrl) {
        ctrl->ShowCancelButton(show);
    }
}

WXD_EXPORTED bool
wxd_SearchCtrl_IsCancelButtonVisible(wxd_SearchCtrl_t* searchCtrl)
{
    wxSearchCtrl* ctrl = (wxSearchCtrl*)searchCtrl;
    if (ctrl) {
        return ctrl->IsCancelButtonVisible();
    }
    return false; // Or some other appropriate default for null ctrl
}

// Set the value via wxSearchCtrl directly (avoid relying on wxTextCtrl casting)
WXD_EXPORTED void
wxd_SearchCtrl_SetValue(const wxd_SearchCtrl_t* searchCtrl, const char* value)
{
    wxSearchCtrl* ctrl =
        const_cast<wxSearchCtrl*>(reinterpret_cast<const wxSearchCtrl*>(searchCtrl));
    if (ctrl) {
        ctrl->SetValue(wxString::FromUTF8(value ? value : ""));
    }
}

// Get the value via wxSearchCtrl directly (avoid relying on wxTextCtrl casting)
// Always return the actual UTF-8 byte length of the current value (excluding the null terminator),
// regardless of whether a buffer was provided.
WXD_EXPORTED size_t
wxd_SearchCtrl_GetValue(const wxd_SearchCtrl_t* searchCtrl, char* buffer, size_t buffer_len)
{
    const wxSearchCtrl* ctrl = reinterpret_cast<const wxSearchCtrl*>(searchCtrl);
    if (!ctrl) {
        return 0;
    }

    wxString value = ctrl->GetValue();
    wxScopedCharBuffer utf8 = value.ToUTF8();
    const size_t actual_len = utf8.length(); // excludes null terminator

    // If the caller provided a buffer, copy into it (truncating if necessary),
    // but always return the full actual length.
    if (buffer && buffer_len > 0) {
        // copy_wxstring_to_buffer ensures null-termination if buffer_len > 0
        (void)wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, buffer_len);
    }

    return actual_len;
}

} // extern "C"