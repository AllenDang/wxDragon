#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../../include/wxdragon.h"
#include "../../include/core/wxd_timer.h"
#include <wx/timer.h>

extern "C" {

// Create a new wxTimer associated with an event handler
WXD_EXPORTED wxd_Timer_t*
wxd_Timer_Create(wxd_EvtHandler_t* owner)
{
    wxEvtHandler* wx_owner = reinterpret_cast<wxEvtHandler*>(owner);
    if (!wx_owner) {
        return nullptr;
    }
    wxTimer* timer = new wxTimer(wx_owner);
    return reinterpret_cast<wxd_Timer_t*>(timer);
}

// Destroy/delete a wxTimer
WXD_EXPORTED void
wxd_Timer_Destroy(wxd_Timer_t* self)
{
    if (!self)
        return;
    wxTimer* timer = reinterpret_cast<wxTimer*>(self);
    if (timer->IsRunning()) {
        timer->Stop();
    }
    delete timer;
}

// Start the timer
WXD_EXPORTED bool
wxd_Timer_Start(wxd_Timer_t* self, int milliseconds, bool oneShot)
{
    if (!self)
        return false;
    wxTimer* timer = reinterpret_cast<wxTimer*>(self);
    return timer->Start(milliseconds, oneShot);
}

// Stop the timer
WXD_EXPORTED void
wxd_Timer_Stop(wxd_Timer_t* self)
{
    if (!self)
        return;
    wxTimer* timer = reinterpret_cast<wxTimer*>(self);
    timer->Stop();
}

// Check if the timer is running
WXD_EXPORTED bool
wxd_Timer_IsRunning(wxd_Timer_t* self)
{
    if (!self)
        return false;
    wxTimer* timer = reinterpret_cast<wxTimer*>(self);
    return timer->IsRunning();
}

// Get the timer interval in milliseconds
WXD_EXPORTED int
wxd_Timer_GetInterval(wxd_Timer_t* self)
{
    if (!self)
        return 0;
    wxTimer* timer = reinterpret_cast<wxTimer*>(self);
    return timer->GetInterval();
}

// Set the timer interval in milliseconds (restart with new interval if running)
WXD_EXPORTED void
wxd_Timer_SetInterval(wxd_Timer_t* self, int milliseconds)
{
    if (!self)
        return;
    wxTimer* timer = reinterpret_cast<wxTimer*>(self);
    bool wasRunning = timer->IsRunning();
    bool oneShot = timer->IsOneShot();

    if (wasRunning) {
        timer->Stop();
        timer->Start(milliseconds, oneShot);
    }
    // If not running, the interval will be used on the next Start() call
}

} // extern "C"