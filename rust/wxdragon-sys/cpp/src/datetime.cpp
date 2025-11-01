#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/datetime.h>

extern "C" {

// Create a new wxDateTime from components and return an opaque pointer
wxd_DateTime_t*
wxd_DateTime_FromComponents(int year,
                            unsigned short month, // Already 0-based, adjusted in Rust wrapper
                            short day, short hour, short minute, short second)
{
    // Validate parameters according to wxDateTime::Set requirements
    if (year <= 0 || month >= 12 || day <= 0 || day > 31 || hour < 0 || hour >= 24 || minute < 0 ||
        minute >= 60 || second < 0 || second >= 60) {
        return nullptr;
    }

    wxDateTime* dt = new (std::nothrow) wxDateTime(static_cast<wxDateTime::wxDateTime_t>(day),
                                                   static_cast<wxDateTime::Month>(month), year,
                                                   static_cast<wxDateTime::wxDateTime_t>(hour),
                                                   static_cast<wxDateTime::wxDateTime_t>(minute),
                                                   static_cast<wxDateTime::wxDateTime_t>(second));
    if (!dt || !dt->IsValid()) {
        delete dt;
        return nullptr;
    }
    return reinterpret_cast<wxd_DateTime_t*>(dt);
}

wxd_DateTime_t*
wxd_DateTime_Clone(const wxd_DateTime_t* dt)
{
    if (!dt)
        return nullptr;
    const wxDateTime* wdt = reinterpret_cast<const wxDateTime*>(dt);
    wxDateTime* clone = new (std::nothrow) wxDateTime(*wdt);
    if (!clone || !clone->IsValid()) {
        delete clone;
        return nullptr;
    }
    return reinterpret_cast<wxd_DateTime_t*>(clone);
}

wxd_DateTime_t*
wxd_DateTime_Now()
{
    wxDateTime* now = new (std::nothrow) wxDateTime(wxDateTime::Now());
    if (!now || !now->IsValid()) {
        delete now;
        return nullptr;
    }
    return reinterpret_cast<wxd_DateTime_t*>(now);
}

// Returns a default-constructed (invalid) wxDateTime pointer
wxd_DateTime_t*
wxd_DateTime_Default()
{
    wxDateTime* dt = new (std::nothrow) wxDateTime();
    return reinterpret_cast<wxd_DateTime_t*>(dt);
}

bool
wxd_DateTime_IsValid(const wxd_DateTime_t* dt)
{
    if (!dt)
        return false;
    const wxDateTime* wdt = reinterpret_cast<const wxDateTime*>(dt);
    return wdt->IsValid();
}

WXD_EXPORTED void
wxd_DateTime_Destroy(wxd_DateTime_t* dt)
{
    if (!dt)
        return;
    wxDateTime* wdt = reinterpret_cast<wxDateTime*>(dt);
    delete wdt;
}

WXD_EXPORTED int
wxd_DateTime_GetYear(const wxd_DateTime_t* dt)
{
    if (!dt)
        return 0;
    const wxDateTime* wdt = reinterpret_cast<const wxDateTime*>(dt);
    return wdt->GetYear();
}

WXD_EXPORTED unsigned short
wxd_DateTime_GetMonth(const wxd_DateTime_t* dt)
{
    if (!dt)
        return 0;
    const wxDateTime* wdt = reinterpret_cast<const wxDateTime*>(dt);
    return static_cast<unsigned short>(wdt->GetMonth());
}

WXD_EXPORTED short
wxd_DateTime_GetDay(const wxd_DateTime_t* dt)
{
    if (!dt)
        return 0;
    const wxDateTime* wdt = reinterpret_cast<const wxDateTime*>(dt);
    return static_cast<short>(wdt->GetDay());
}

WXD_EXPORTED short
wxd_DateTime_GetHour(const wxd_DateTime_t* dt)
{
    if (!dt)
        return 0;
    const wxDateTime* wdt = reinterpret_cast<const wxDateTime*>(dt);
    return static_cast<short>(wdt->GetHour());
}

WXD_EXPORTED short
wxd_DateTime_GetMinute(const wxd_DateTime_t* dt)
{
    if (!dt)
        return 0;
    const wxDateTime* wdt = reinterpret_cast<const wxDateTime*>(dt);
    return static_cast<short>(wdt->GetMinute());
}

WXD_EXPORTED short
wxd_DateTime_GetSecond(const wxd_DateTime_t* dt)
{
    if (!dt)
        return 0;
    const wxDateTime* wdt = reinterpret_cast<const wxDateTime*>(dt);
    return static_cast<short>(wdt->GetSecond());
}

} // extern "C"