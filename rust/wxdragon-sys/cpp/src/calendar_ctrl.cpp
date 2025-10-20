#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wxdragon.h"
#include <wx/calctrl.h>
#include <wx/datetime.h> // For wxDateTime

// Helper to convert wxd_DateTime_t to wxDateTime
static wxDateTime wxd_to_wx_datetime(const wxd_DateTime_t* wxd_dt) {
    if (!wxd_dt) {
        // If no date is provided, wxCalendarCtrl defaults to today, so we can reflect that.
        // Or, if wxd_DateTime_t cannot represent an "unset" state distinctly from a valid date (e.g. all zeros)
        // then this needs careful handling based on how Rust side sends "no initial date".
        // For now, assuming NULL means "use wxWidgets default", which is wxDateTime::Today() for wxCalendarCtrl constructor.
        // For SetDate, a NULL might be an error or mean "no change", depending on API design.
        // Let's assume for Create, NULL means "default", for SetDate it means "error/invalid".
        return wxDateTime::Today(); 
    }
    // wxDateTime month is 0-11 (wxDateTime::Jan, wxDateTime::Feb, ...)
    // wxd_DateTime_t.month is 0-11 as per its definition in wxdragon.h (matching wxDateTime)
    return wxDateTime((unsigned short)wxd_dt->day, (wxDateTime::Month)wxd_dt->month, wxd_dt->year,
                      (unsigned short)wxd_dt->hour, (unsigned short)wxd_dt->minute, (unsigned short)wxd_dt->second);
}

// Helper to convert wxDateTime to wxd_DateTime_t
static wxd_DateTime_t wx_to_wxd_datetime(const wxDateTime& wx_dt) {
    wxd_DateTime_t wxd_dt;
    wxd_dt.day = wx_dt.GetDay(wxDateTime::Local);
    wxd_dt.month = wx_dt.GetMonth(wxDateTime::Local); // wxDateTime::Month is 0-indexed
    wxd_dt.year = wx_dt.GetYear(wxDateTime::Local);
    wxd_dt.hour = wx_dt.GetHour(wxDateTime::Local);
    wxd_dt.minute = wx_dt.GetMinute(wxDateTime::Local);
    wxd_dt.second = wx_dt.GetSecond(wxDateTime::Local);
    return wxd_dt;
}

extern "C" {

WXD_EXPORTED wxd_CalendarCtrl_t* wxd_CalendarCtrl_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    const wxd_DateTime_t* date, // Initial date, can be NULL for default (current date)
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
) {
    if (!parent) return NULL; // Parent is mandatory for a control
    wxWindow* wx_parent = (wxWindow*)parent;
    
    wxDateTime initialDate;
    if (date && date->year != 0) { // Check if date is somewhat initialized, year 0 could be a sentinel
        initialDate = wxd_to_wx_datetime(date);
    } else {
        initialDate = wxDateTime::Today(); // Default to today if date is NULL or uninitialized
    }

    wxCalendarCtrl* ctrl = new wxCalendarCtrl(
        wx_parent,
        id,
        initialDate,
        wxPoint(pos.x, pos.y),
        wxSize(size.width, size.height),
        style
        // Consider adding wxCAL_SHOW_HOLIDAYS or other defaults if desired, or leave to style
    );
    return (wxd_CalendarCtrl_t*)ctrl;
}

WXD_EXPORTED bool wxd_CalendarCtrl_SetDate(wxd_CalendarCtrl_t* self, const wxd_DateTime_t* date) {
    if (!self || !date || date->year == 0) return false; // Ensure valid pointers and a somewhat valid date
    wxCalendarCtrl* ctrl = (wxCalendarCtrl*)self;
    return ctrl->SetDate(wxd_to_wx_datetime(date));
}

WXD_EXPORTED wxd_DateTime_t* wxd_CalendarCtrl_GetDate(wxd_CalendarCtrl_t* self) {
    if (!self) {
        // Return nullptr for an invalid/uninitialized date pointer
        return nullptr; 
    }
    wxCalendarCtrl* ctrl = (wxCalendarCtrl*)self;
    const wxDateTime& dt = ctrl->GetDate();
    if (!dt.IsValid()) {
        return nullptr;
    }
    // Allocate on heap, Rust (DateTime::from_raw) will take ownership and free.
    return new wxd_DateTime_t{
        (short)dt.GetDay(wxDateTime::Local),
        (unsigned short)dt.GetMonth(wxDateTime::Local), // wxDateTime::Month is 0-indexed, wxd_DateTime_t.month is also 0-indexed
        dt.GetYear(wxDateTime::Local),
        (short)dt.GetHour(wxDateTime::Local),
        (short)dt.GetMinute(wxDateTime::Local),
        (short)dt.GetSecond(wxDateTime::Local)
    };
}

// Get the date from a calendar event
WXD_EXPORTED wxd_DateTime_t* wxd_CalendarEvent_GetDate(wxd_Event_t* event) {
    if (!event) return nullptr;
    
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    if (!wx_event->IsKindOf(wxCLASSINFO(wxCalendarEvent))) return nullptr;
    
    wxCalendarEvent* cal_event = static_cast<wxCalendarEvent*>(wx_event);
    const wxDateTime& dt = cal_event->GetDate();
    
    if (!dt.IsValid()) return nullptr;
    
    // Allocate on heap, Rust will take ownership and free
    return new wxd_DateTime_t{
        (short)dt.GetDay(wxDateTime::Local),
        (unsigned short)(dt.GetMonth(wxDateTime::Local)), // Keep 0-indexed for consistency with other functions
        dt.GetYear(wxDateTime::Local),
        (short)dt.GetHour(wxDateTime::Local),
        (short)dt.GetMinute(wxDateTime::Local),
        (short)dt.GetSecond(wxDateTime::Local)
    };
}

} // extern "C" 