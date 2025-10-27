#ifndef WXD_CALENDAR_CTRL_H
#define WXD_CALENDAR_CTRL_H

#include "../wxd_types.h" // For wxd_Window_t, wxd_Id, wxd_DateTime_t, wxd_Point, wxd_Size, wxd_Style_t, wxd_CalendarCtrl_t

#ifdef __cplusplus
extern "C" {
#endif

WXD_EXPORTED wxd_CalendarCtrl_t*
wxd_CalendarCtrl_Create(wxd_Window_t* parent, wxd_Id id, const wxd_DateTime_t* date, wxd_Point pos,
                        wxd_Size size, wxd_Style_t style);

WXD_EXPORTED bool
wxd_CalendarCtrl_SetDate(wxd_CalendarCtrl_t* self, const wxd_DateTime_t* date);

WXD_EXPORTED wxd_DateTime_t*
wxd_CalendarCtrl_GetDate(
    wxd_CalendarCtrl_t*
        self); // Changed return to wxd_DateTime_t* to match C++ impl style (heap allocated)

#ifdef __cplusplus
}
#endif

#endif // WXD_CALENDAR_CTRL_H