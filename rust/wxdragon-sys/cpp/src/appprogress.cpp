#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "../include/core/wxd_appprogress.h"
#include <wx/appprogress.h>

extern "C" {

// Create a new wxAppProgressIndicator
WXD_EXPORTED wxd_AppProgressIndicator_t*
wxd_AppProgressIndicator_Create(wxd_Window_t* parent)
{
    wxWindow* p = reinterpret_cast<wxWindow*>(parent);
    wxAppProgressIndicator* appprogress = new wxAppProgressIndicator(p);
    return reinterpret_cast<wxd_AppProgressIndicator_t*>(appprogress);
}

// Destroy/delete a wxAppProgressIndicator
WXD_EXPORTED void
wxd_AppProgressIndicator_Destroy(wxd_AppProgressIndicator_t* self)
{
    if (!self)
        return;
    wxAppProgressIndicator* appprogress = reinterpret_cast<wxAppProgressIndicator*>(self);
    delete appprogress;
}

// Check if the application progress display is available.
WXD_EXPORTED bool
wxd_AppProgressIndicator_IsAvailable(wxd_AppProgressIndicator_t* self)
{
    if (!self)
        return false;
    wxAppProgressIndicator* appprogress = reinterpret_cast<wxAppProgressIndicator*>(self);
    return appprogress->IsAvailable();
}

// Set the progress value in taskbar button of parent window.
WXD_EXPORTED void
wxd_AppProgressIndicator_SetValue(wxd_AppProgressIndicator_t* self, int value)
{
    if (!self)
        return;
    wxAppProgressIndicator* appprogress = reinterpret_cast<wxAppProgressIndicator*>(self);
    appprogress->SetValue(value);
}

// Set the progress range in taskbar button of parent window.
WXD_EXPORTED void
wxd_AppProgressIndicator_SetRange(wxd_AppProgressIndicator_t* self, int range)
{
    if (!self)
        return;
    wxAppProgressIndicator* appprogress = reinterpret_cast<wxAppProgressIndicator*>(self);
    appprogress->SetRange(range);
}

// Makes the progress bar run in indeterminate mode.
WXD_EXPORTED void
wxd_AppProgressIndicator_Pulse(wxd_AppProgressIndicator_t* self)
{
    if (!self)
        return;
    wxAppProgressIndicator* appprogress = reinterpret_cast<wxAppProgressIndicator*>(self);
    appprogress->Pulse();
}

} // extern "C"