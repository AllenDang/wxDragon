#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/utils.h>

extern "C" {

// Produces an audible beep sound
WXD_EXPORTED void
wxd_Bell(void)
{
    wxBell();
}

// Opens the given URL in the default browser
WXD_EXPORTED bool
wxd_LaunchDefaultBrowser(const char* url, int flags)
{
    if (!url)
        return false;

    wxString wxUrl = wxString::FromUTF8(url);
    return wxLaunchDefaultBrowser(wxUrl, flags);
}

// Opens the given file/document in its default application
WXD_EXPORTED bool
wxd_LaunchDefaultApplication(const char* path, int flags)
{
    if (!path)
        return false;

    wxString wxPath = wxString::FromUTF8(path);
    return wxLaunchDefaultApplication(wxPath, flags);
}

// Gets the current global mouse position in screen coordinates
WXD_EXPORTED wxd_Point
wxd_GetMousePosition(void)
{
    int x = 0, y = 0;
    wxGetMousePosition(&x, &y);
    return { x, y };
}

// Returns true if the given key is currently pressed down
WXD_EXPORTED bool
wxd_GetKeyState(int keycode)
{
    return wxGetKeyState(static_cast<wxKeyCode>(keycode));
}

} // extern "C"
