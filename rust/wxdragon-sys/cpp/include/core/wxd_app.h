#ifndef WXD_APP_H
#define WXD_APP_H

#include "../wxd_types.h" // Adjust path as necessary if wxd_types.h is at the root of include/

// --- App Functions ---
WXD_EXPORTED wxd_App_t*
wxd_GetApp();
WXD_EXPORTED int
wxd_Main(int argc, char** argv, wxd_OnInitCallback on_init, void* userData);
WXD_EXPORTED void
wxd_App_SetTopWindow(wxd_App_t* app, wxd_Window_t* window);

// Process callback queue
WXD_EXPORTED void
wxd_App_ProcessCallbacks();

// New function to free an array of integers allocated by C++
WXD_EXPORTED void
wxd_free_int_array(int* ptr);

// --- Appearance Support (wxWidgets 3.3.0+) ---

// Set the application appearance mode (requires wxWidgets 3.3.0+)
WXD_EXPORTED wxd_AppearanceResult
wxd_App_SetAppearance(wxd_App_t* app, wxd_Appearance appearance);

// Get system appearance information
WXD_EXPORTED wxd_SystemAppearance_t*
wxd_SystemSettings_GetAppearance();

// Check if the system is using dark mode
WXD_EXPORTED bool
wxd_SystemAppearance_IsDark(wxd_SystemAppearance_t* appearance);

// Check if the system background is dark
WXD_EXPORTED bool
wxd_SystemAppearance_IsUsingDarkBackground(wxd_SystemAppearance_t* appearance);

/**
 * @brief Get the system appearance name (mainly for macOS)
 * Returns the required UTF-8 byte length (excluding null terminator).
 * If out is not null and out_len > 0, copies up to out_len - 1 bytes and null-terminates.
 * If out is null or out_len == 0, nothing is written.
 * @return Required UTF-8 byte length (excluding null terminator)
 */
WXD_EXPORTED int
wxd_SystemAppearance_GetName(const wxd_SystemAppearance_t* appearance, char* out, size_t out_len);

// Free system appearance object
WXD_EXPORTED void
wxd_SystemAppearance_Destroy(wxd_SystemAppearance_t* appearance);

// --- End of Appearance Support ---

// --- macOS-specific App Event Handlers ---

// Register handlers for macOS application events (supports multiple handlers per event)
WXD_EXPORTED void
wxd_App_AddMacOpenFilesHandler(wxd_App_t* app, wxd_MacOpenFilesCallback callback, void* userData);
WXD_EXPORTED void
wxd_App_AddMacOpenURLHandler(wxd_App_t* app, wxd_MacOpenURLCallback callback, void* userData);
WXD_EXPORTED void
wxd_App_AddMacNewFileHandler(wxd_App_t* app, wxd_MacNewFileCallback callback, void* userData);
WXD_EXPORTED void
wxd_App_AddMacReopenAppHandler(wxd_App_t* app, wxd_MacReopenAppCallback callback, void* userData);
WXD_EXPORTED void
wxd_App_AddMacPrintFilesHandler(wxd_App_t* app, wxd_MacPrintFilesCallback callback, void* userData);

// --- End of macOS-specific App Event Handlers ---

#endif // WXD_APP_H