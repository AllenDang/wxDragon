#ifndef WXD_APP_H
#define WXD_APP_H

#include "../wxd_types.h" // Adjust path as necessary if wxd_types.h is at the root of include/

// --- App Functions ---
WXD_EXPORTED wxd_App_t* wxd_GetApp();
WXD_EXPORTED int wxd_Main(int argc, char** argv, wxd_OnInitCallback on_init, void* userData);
WXD_EXPORTED void wxd_App_SetTopWindow(wxd_App_t* app, wxd_Window_t* window);

// Process callback queue
WXD_EXPORTED void wxd_App_ProcessCallbacks();

// Utility to free strings allocated by wxDragon C API
WXD_EXPORTED void wxd_free_string(char* str);

// New function to free an array of integers allocated by C++
WXD_EXPORTED void wxd_free_int_array(int* ptr);

// --- Appearance Support (wxWidgets 3.3.0+) ---

// Set the application appearance mode (requires wxWidgets 3.3.0+)
WXD_EXPORTED wxd_AppearanceResult wxd_App_SetAppearance(wxd_App_t* app, wxd_Appearance appearance);

// Get system appearance information
WXD_EXPORTED wxd_SystemAppearance_t* wxd_SystemSettings_GetAppearance();

// Check if the system is using dark mode
WXD_EXPORTED bool wxd_SystemAppearance_IsDark(wxd_SystemAppearance_t* appearance);

// Check if the system background is dark
WXD_EXPORTED bool wxd_SystemAppearance_IsUsingDarkBackground(wxd_SystemAppearance_t* appearance);

// Get the system appearance name (mainly for macOS)
WXD_EXPORTED char* wxd_SystemAppearance_GetName(wxd_SystemAppearance_t* appearance);

// Free system appearance object
WXD_EXPORTED void wxd_SystemAppearance_Destroy(wxd_SystemAppearance_t* appearance);

// --- End of Appearance Support ---

#endif // WXD_APP_H 