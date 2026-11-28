#ifndef WXD_PREFERENCES_H
#define WXD_PREFERENCES_H

#include "../wxd_types.h"

// Builds the window for a page. wxWidgets passes the parent the window must be created in.
typedef wxd_Window_t* (*wxd_PreferencesPage_CreateWindowCallback)(void* userData,
                                                                  wxd_Window_t* parent);

// Frees a page's user data. Called when the editor deletes the page.
typedef void (*wxd_PreferencesPage_DropCallback)(void* userData);

// Stock page kinds, matching wxStockPreferencesPage::Kind.
#define WXD_PREFERENCES_PAGE_GENERAL  0
#define WXD_PREFERENCES_PAGE_ADVANCED 1

WXD_EXPORTED wxd_PreferencesEditor_t*
wxd_PreferencesEditor_Create(const char* title);

WXD_EXPORTED void
wxd_PreferencesEditor_Destroy(wxd_PreferencesEditor_t* editor);

// Adds a page with its own name and an optional icon, which is copied. The editor owns the page.
WXD_EXPORTED void
wxd_PreferencesEditor_AddPage(wxd_PreferencesEditor_t* editor, const char* name,
                              const wxd_BitmapBundle_t* icon,
                              wxd_PreferencesPage_CreateWindowCallback create,
                              wxd_PreferencesPage_DropCallback drop, void* userData);

// Adds a page with the platform's stock name and icon for `kind`. The editor owns the page.
WXD_EXPORTED void
wxd_PreferencesEditor_AddStockPage(wxd_PreferencesEditor_t* editor, int kind,
                                   wxd_PreferencesPage_CreateWindowCallback create,
                                   wxd_PreferencesPage_DropCallback drop, void* userData);

WXD_EXPORTED void
wxd_PreferencesEditor_Show(wxd_PreferencesEditor_t* editor, wxd_Window_t* parent);

WXD_EXPORTED void
wxd_PreferencesEditor_Dismiss(wxd_PreferencesEditor_t* editor);

WXD_EXPORTED bool
wxd_PreferencesEditor_ShouldApplyChangesImmediately(void);

WXD_EXPORTED bool
wxd_PreferencesEditor_ShownModally(void);

#endif // WXD_PREFERENCES_H
