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

} // extern "C"