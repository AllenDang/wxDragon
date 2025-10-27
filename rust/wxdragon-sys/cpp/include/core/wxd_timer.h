#ifndef WXD_TIMER_H
#define WXD_TIMER_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Timer notification styles
typedef enum {
    WXD_TIMER_NOTIFY_DEFAULT = 0, // Default notification behavior
    WXD_TIMER_NOTIFY_ONESHOT = 1  // One-shot timer (stops after first notification)
} WXDTimerNotify;

// Timer status values
typedef enum {
    WXD_TIMER_STATUS_IDLE = 0,   // Timer is idle
    WXD_TIMER_STATUS_RUNNING = 1 // Timer is running
} WXDTimerStatus;

// Create a new wxTimer
WXD_EXPORTED wxd_Timer_t*
wxd_Timer_Create(wxd_EvtHandler_t* owner);

// Destroy/delete a wxTimer
WXD_EXPORTED void
wxd_Timer_Destroy(wxd_Timer_t* self);

// Start the timer
WXD_EXPORTED bool
wxd_Timer_Start(wxd_Timer_t* self, int milliseconds, bool oneShot);

// Stop the timer
WXD_EXPORTED void
wxd_Timer_Stop(wxd_Timer_t* self);

// Check if the timer is running
WXD_EXPORTED bool
wxd_Timer_IsRunning(wxd_Timer_t* self);

// Get the timer interval in milliseconds
WXD_EXPORTED int
wxd_Timer_GetInterval(wxd_Timer_t* self);

// Set the timer interval in milliseconds
WXD_EXPORTED void
wxd_Timer_SetInterval(wxd_Timer_t* self, int milliseconds);

#ifdef __cplusplus
}
#endif

#endif // WXD_TIMER_H