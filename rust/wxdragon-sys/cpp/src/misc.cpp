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

} // extern "C"
