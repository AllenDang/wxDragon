#include "wx/wxprec.h"

#ifndef WX_PRECOMP
#include "wx/wx.h"
#endif

#include "wx/gauge.h"
#include "../include/wxdragon.h"

extern "C" {

WXD_EXPORTED wxd_Gauge_t*
wxd_Gauge_Create(wxd_Window_t* parent, wxd_Id id, int range, int x, int y, int w, int h,
                 wxd_Style_t style)
{
    // Cast opaque pointers back to wxWidgets types
    wxWindow* wx_parent = reinterpret_cast<wxWindow*>(parent);
    wxPoint pos = wxPoint(x, y);
    wxSize size = wxSize(w, h);
    // wxGA_HORIZONTAL is the default if no orientation flag is given
    wxGauge* gauge = new wxGauge(wx_parent, id, range, pos, size, style | wxGA_HORIZONTAL);
    // Cast the result back to the opaque pointer type
    return reinterpret_cast<wxd_Gauge_t*>(gauge);
    // Note: wxWidgets automatically adds it to the parent's children
}

WXD_EXPORTED void
wxd_Gauge_SetRange(wxd_Gauge_t* self, int range)
{
    wxGauge* gauge = reinterpret_cast<wxGauge*>(self);
    if (!gauge)
        return;
    gauge->SetRange(range);
}

WXD_EXPORTED void
wxd_Gauge_SetValue(wxd_Gauge_t* self, int value)
{
    wxGauge* gauge = reinterpret_cast<wxGauge*>(self);
    if (!gauge)
        return;
    gauge->SetValue(value);
}

WXD_EXPORTED int
wxd_Gauge_GetValue(const wxd_Gauge_t* self)
{
    const wxGauge* gauge = reinterpret_cast<const wxGauge*>(self);
    if (!gauge)
        return 0; // Or some error indication? Returning 0 for now.
    return gauge->GetValue();
}

// Note: wxGauge is a wxWindow, so wxd_Window_Destroy should work for cleanup.
// No specific wxd_Gauge_Destroy needed unless extra cleanup is required.

} // extern "C"