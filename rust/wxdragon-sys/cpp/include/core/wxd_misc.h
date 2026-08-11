#ifndef WXD_MISC_H
#define WXD_MISC_H

#include "../wxd_types.h"

// --- Miscellaneous System Functions ---

// Produces an audible beep sound
WXD_EXPORTED void
wxd_Bell(void);

// Opens the given URL in the default browser
// Returns true if the browser was successfully launched, false otherwise
WXD_EXPORTED bool
wxd_LaunchDefaultBrowser(const char* url, int flags);

// Opens the given file/document in its default application (e.g. a PDF in the system's
// PDF viewer). Returns true if the application was successfully launched, false otherwise.
WXD_EXPORTED bool
wxd_LaunchDefaultApplication(const char* path, int flags);

// Gets the current global position of the mouse pointer in screen coordinates.
WXD_EXPORTED wxd_Point
wxd_GetMousePosition(void);

// Returns true if the given key (a WXK_* constant or ASCII/Unicode code point) is
// currently pressed down, independently of the event system (e.g. outside of an
// event handler, or for keys that aren't the source of the current event).
WXD_EXPORTED bool
wxd_GetKeyState(int keycode);

#endif // WXD_MISC_H
