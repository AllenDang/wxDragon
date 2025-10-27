#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wxdragon.h"
#include <wx/timectrl.h>
#include <wx/datetime.h>

// Use the same helpers from datepickerctrl.cpp

// Helper to convert wxd_DateTime_t* (opaque) to wxDateTime
static wxDateTime
wxd_to_wx_datetime(const wxd_DateTime_t* wxd_dt)
{
    if (!wxd_dt) {
        return wxDefaultDateTime;
    }
    const wxDateTime* dt = reinterpret_cast<const wxDateTime*>(wxd_dt);
    return *dt;
}

// --- wxTimePickerCtrl Functions ---

WXD_EXPORTED wxd_TimePickerCtrl_t*
wxd_TimePickerCtrl_Create(wxd_Window_t* parent, int id, const wxd_DateTime_t* dt,
                          wxd_Point pos, // Pass by value
                          wxd_Size size, // Pass by value
                          int64_t style)
{
    wxWindow* wx_parent = (wxWindow*)parent;
    wxPoint wx_pos = wxPoint(pos.x, pos.y);           // Use directly
    wxSize wx_size = wxSize(size.width, size.height); // Use directly
    wxDateTime wx_dt_val = dt ? wxd_to_wx_datetime(dt) : wxDefaultDateTime;

    // Ensure default style if none provided
    if (style == 0) {
        style = wxTP_DEFAULT; // Default style for time picker
    }

    wxTimePickerCtrl* wx_picker =
        new wxTimePickerCtrl(wx_parent, id, wx_dt_val, wx_pos, wx_size, style);
    return (wxd_TimePickerCtrl_t*)wx_picker;
}

WXD_EXPORTED wxd_DateTime_t*
wxd_TimePickerCtrl_GetValue(wxd_TimePickerCtrl_t* self)
{
    wxTimePickerCtrl* wx_picker = (wxTimePickerCtrl*)self;
    if (!wx_picker) {
        return nullptr;
    }
    wxDateTime val = wx_picker->GetValue();
    if (!val.IsValid())
        return nullptr;
    return reinterpret_cast<wxd_DateTime_t*>(new (std::nothrow) wxDateTime(val));
}

WXD_EXPORTED void
wxd_TimePickerCtrl_SetValue(wxd_TimePickerCtrl_t* self, const wxd_DateTime_t* dt)
{
    wxTimePickerCtrl* wx_picker = (wxTimePickerCtrl*)self;
    if (!wx_picker)
        return;

    wxDateTime wx_dt_val = dt ? wxd_to_wx_datetime(dt) : wxDefaultDateTime;
    wx_picker->SetValue(wx_dt_val);
}

// Event type constant for TIME_CHANGED
// This should be defined in wxdragon.h in WXDEventTypeCEnum,
// and its value extracted by const_extractor.
// For completeness, its C++ equivalent is wxEVT_TIME_CHANGED.
// The Rust side will use ffi::WXD_EVENT_TYPE_TIME_CHANGED.