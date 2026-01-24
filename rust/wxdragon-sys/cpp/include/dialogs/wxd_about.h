#ifndef WXD_ABOUT_H
#define WXD_ABOUT_H

#include "../wxd_types.h"

// --- AboutDialogInfo Functions ---

// Creates a new wxAboutDialogInfo object
WXD_EXPORTED wxd_AboutDialogInfo_t*
wxd_AboutDialogInfo_Create(void);

// Destroys a wxAboutDialogInfo object
WXD_EXPORTED void
wxd_AboutDialogInfo_Destroy(wxd_AboutDialogInfo_t* info);

// Sets the name of the application
WXD_EXPORTED void
wxd_AboutDialogInfo_SetName(wxd_AboutDialogInfo_t* info, const char* name);

// Sets the version string
WXD_EXPORTED void
wxd_AboutDialogInfo_SetVersion(wxd_AboutDialogInfo_t* info, const char* version);

// Sets the version with both short and long version strings
WXD_EXPORTED void
wxd_AboutDialogInfo_SetVersionEx(wxd_AboutDialogInfo_t* info, const char* version,
                                  const char* longVersion);

// Sets the description of the program
WXD_EXPORTED void
wxd_AboutDialogInfo_SetDescription(wxd_AboutDialogInfo_t* info, const char* desc);

// Sets the copyright string
WXD_EXPORTED void
wxd_AboutDialogInfo_SetCopyright(wxd_AboutDialogInfo_t* info, const char* copyright);

// Sets the license text
WXD_EXPORTED void
wxd_AboutDialogInfo_SetLicence(wxd_AboutDialogInfo_t* info, const char* licence);

// Sets the icon from a bitmap
WXD_EXPORTED void
wxd_AboutDialogInfo_SetIcon(wxd_AboutDialogInfo_t* info, const wxd_Bitmap_t* bitmap);

// Sets the website URL
WXD_EXPORTED void
wxd_AboutDialogInfo_SetWebSite(wxd_AboutDialogInfo_t* info, const char* url);

// Sets the website URL with custom description
WXD_EXPORTED void
wxd_AboutDialogInfo_SetWebSiteEx(wxd_AboutDialogInfo_t* info, const char* url,
                                  const char* desc);

// Adds a developer name to the list
WXD_EXPORTED void
wxd_AboutDialogInfo_AddDeveloper(wxd_AboutDialogInfo_t* info, const char* developer);

// Adds a documentation writer name to the list
WXD_EXPORTED void
wxd_AboutDialogInfo_AddDocWriter(wxd_AboutDialogInfo_t* info, const char* docwriter);

// Adds an artist name to the list
WXD_EXPORTED void
wxd_AboutDialogInfo_AddArtist(wxd_AboutDialogInfo_t* info, const char* artist);

// Adds a translator name to the list
WXD_EXPORTED void
wxd_AboutDialogInfo_AddTranslator(wxd_AboutDialogInfo_t* info, const char* translator);

// --- AboutBox Function ---

// Shows the about dialog
WXD_EXPORTED void
wxd_AboutBox(const wxd_AboutDialogInfo_t* info, wxd_Window_t* parent);

#endif // WXD_ABOUT_H
