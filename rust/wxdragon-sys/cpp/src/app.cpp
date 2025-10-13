#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/app.h>
#include <wx/image.h>
#include <cstdlib>
#include <wx/private/safecall.h>
#include <wx/scopeguard.h>

// --- Globals --- 
// Store the C callback and user data provided to wxd_Main
static wxd_OnInitCallback g_OnInitCallback = nullptr;
static void* g_OnInitUserData = nullptr;

// Function to process Rust callbacks, implemented in Rust
extern "C" int process_rust_callbacks();

// --- Internal C++ App Class --- 

class WxdApp : public wxApp {
public:
    // Called by wxWidgets framework on application startup.
    virtual bool OnInit() override;

    // Idle event handler to process callbacks
    void OnIdle(wxIdleEvent& event);
    
    // Optional: Override OnExit for cleanup if needed
    // virtual int OnExit() override;
};

// Implementation of OnInit - this is where we call the C callback
bool WxdApp::OnInit() {
    // Call base class OnInit (important)
    if (!wxApp::OnInit()) {
        return false;
    }

#ifdef __WXMSW__
    // Use best available visual (important for proper rendering, especially with transparency)
    SetUseBestVisual(true);
#endif

    // Initialize all stock items (standard icons, etc.)
    wxInitializeStockLists();

    // Bind idle event to process callbacks
    Bind(wxEVT_IDLE, &WxdApp::OnIdle, this);

    // Call the stored C callback function
    if (g_OnInitCallback) {
        // The callback is responsible for creating the main window
        // and calling wxd_App_SetTopWindow.
        bool success = g_OnInitCallback(g_OnInitUserData);
        return success;
    } else {
        // Should not happen if wxd_Main is used correctly
        wxLogError("wxDragon: No OnInit callback provided to wxd_Main.");
        return false;
    }
}

// Process callbacks on idle
void WxdApp::OnIdle(wxIdleEvent& event) {
    // Process any pending Rust callbacks
    int callbacks_processed = process_rust_callbacks();
    
    // Only request more idle events if there were callbacks to process
    // This prevents unnecessary CPU usage when the app is idle
    if (callbacks_processed > 0) {
        event.RequestMore();
    }
}

// --- C API Implementation --- 

// This macro creates the necessary wxWidgets entry points (like main or WinMain)
// and instantiates our WxdApp class when wxEntry is called.
// It effectively hides the platform-specific entry point boilerplate.
// However, it means our C API user doesn't write main(), they write a function
// that calls wxd_Main(), and we need a way to trigger wxEntry.

// Let's use DECLARE/IMPLEMENT_APP_NO_MAIN. This requires us to provide
// the actual main() function or equivalent, allowing our wxd_Main to control
// the startup sequence.
wxDECLARE_APP(WxdApp);
wxIMPLEMENT_APP_NO_MAIN(WxdApp);

// Main entry point implementation
int wxd_Main(int argc, char** argv, wxd_OnInitCallback on_init_cb, void* userData) {
    if (!on_init_cb) {
        fprintf(stderr, "wxDragon Error: No OnInit callback provided to wxd_Main.\n");
        return 1;
    }

    g_OnInitCallback = on_init_cb;
    g_OnInitUserData = userData;

    if (!wxEntryStart(argc, argv)) {
        fprintf(stderr, "wxDragon Error: Failed to initialize wxWidgets (wxEntryStart failed).\n");
        return 1;
    }

    // Initialize all available image handlers (PNG, JPEG, etc.)
    // This must be done after wxEntryStart and before any image loading (e.g., in app OnInit).
    wxInitAllImageHandlers();

    int result = wxSafeCall<int>(
        []() {
            // wxTheApp should now be a WxdApp instance.
            if (wxTheApp == nullptr) {
                fprintf(stderr, "wxDragon Error: wxTheApp is null after wxEntryStart.\n");
                return wxApp::GetFatalErrorExitCode();
            }
            // CallOnInit will execute WxdApp::OnInit, which calls the Rust g_OnInitCallback.
            if ( !wxTheApp->CallOnInit() ) {
                // don't call OnExit() if OnInit() failed
                return wxTheApp->GetErrorExitCode();
            }

            // ensure that OnExit() is called if OnInit() had succeeded
            wxON_BLOCK_EXIT_OBJ0(*wxTheApp, wxApp::OnExit);

            // Rust initialization was successful (returned true),
            // then app execution, start the main event loop
            return wxTheApp->OnRun();
        },
        []() {
            wxApp::CallOnUnhandledException();
            return wxApp::GetFatalErrorExitCode();
        }
    );

    wxEntryCleanup();
    g_OnInitCallback = nullptr;
    g_OnInitUserData = nullptr;
    return result;
}

// Gets the handle to the global application instance.
wxd_App_t* wxd_GetApp() {
    // wxTheApp is the global pointer to the wxApp instance
    return reinterpret_cast<wxd_App_t*>(wxTheApp);
}

// Sets the top window (main frame) for the application.
void wxd_App_SetTopWindow(wxd_App_t* app, wxd_Window_t* window) {
    if (!app || !window) return;
    WxdApp* wx_app = reinterpret_cast<WxdApp*>(app);
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wx_app->SetTopWindow(wx_window);
}

// Manual callback processing for cases where we need to trigger it
void wxd_App_ProcessCallbacks() {
    process_rust_callbacks();
}

// Implementation for wxd_free_string
void wxd_free_string(char* str) {
    if (str) {
        free(str); // free is from <cstdlib> or <stdlib.h>
    }
}

// Implementation for wxd_free_int_array
void wxd_free_int_array(int* ptr) {
    if (ptr) {
        free(ptr);
    }
}

// --- Appearance Support Implementation (wxWidgets 3.3.0+) ---

#if wxCHECK_VERSION(3, 3, 0)
#include <wx/settings.h>
#include <wx/systhemectrl.h>
#endif

// Set the application appearance mode
wxd_AppearanceResult wxd_App_SetAppearance(wxd_App_t* app, wxd_Appearance appearance) {
    if (!app) return WXD_APPEARANCE_RESULT_FAILURE;
    
#if wxCHECK_VERSION(3, 3, 0)
    wxApp* wx_app = reinterpret_cast<wxApp*>(app);
    
    wxApp::Appearance wx_appearance;
    switch (appearance) {
        case WXD_APPEARANCE_LIGHT:
            wx_appearance = wxApp::Appearance::Light;
            break;
        case WXD_APPEARANCE_DARK:
            wx_appearance = wxApp::Appearance::Dark;
            break;
        case WXD_APPEARANCE_SYSTEM:
        default:
            wx_appearance = wxApp::Appearance::System;
            break;
    }
    
    wxApp::AppearanceResult result = wx_app->SetAppearance(wx_appearance);
    
    switch (result) {
        case wxApp::AppearanceResult::Ok:
            return WXD_APPEARANCE_RESULT_OK;
        case wxApp::AppearanceResult::Failure:
            return WXD_APPEARANCE_RESULT_FAILURE;
        case wxApp::AppearanceResult::CannotChange:
            return WXD_APPEARANCE_RESULT_CANNOT_CHANGE;
    }
    
    return WXD_APPEARANCE_RESULT_FAILURE;
#else
    // For older wxWidgets versions, appearance is not supported
    return WXD_APPEARANCE_RESULT_FAILURE;
#endif
}

// Get system appearance information
wxd_SystemAppearance_t* wxd_SystemSettings_GetAppearance() {
#if wxCHECK_VERSION(3, 3, 0)
    wxSystemAppearance appearance = wxSystemSettings::GetAppearance();
    // Create a copy on the heap to return to Rust
    wxSystemAppearance* heap_appearance = new wxSystemAppearance(appearance);
    return reinterpret_cast<wxd_SystemAppearance_t*>(heap_appearance);
#else
    // For older wxWidgets versions, return null
    return nullptr;
#endif
}

// Check if the system is using dark mode
bool wxd_SystemAppearance_IsDark(wxd_SystemAppearance_t* appearance) {
    if (!appearance) return false;
    
#if wxCHECK_VERSION(3, 3, 0)
    wxSystemAppearance* wx_appearance = reinterpret_cast<wxSystemAppearance*>(appearance);
    return wx_appearance->IsDark();
#else
    return false;
#endif
}

// Check if the system background is dark
bool wxd_SystemAppearance_IsUsingDarkBackground(wxd_SystemAppearance_t* appearance) {
    if (!appearance) return false;
    
#if wxCHECK_VERSION(3, 3, 0)
    wxSystemAppearance* wx_appearance = reinterpret_cast<wxSystemAppearance*>(appearance);
    return wx_appearance->IsUsingDarkBackground();
#else
    return false;
#endif
}

// Get the system appearance name (mainly for macOS)
char* wxd_SystemAppearance_GetName(wxd_SystemAppearance_t* appearance) {
    if (!appearance) return strdup("");
    
#if wxCHECK_VERSION(3, 3, 0)
    wxSystemAppearance* wx_appearance = reinterpret_cast<wxSystemAppearance*>(appearance);
    wxString name = wx_appearance->GetName();
    const wxScopedCharBuffer utf8_buf = name.ToUTF8();
    if (utf8_buf.data()) {
        return strdup(utf8_buf.data());
    }
#endif
    return strdup("");
}

// Free system appearance object
void wxd_SystemAppearance_Destroy(wxd_SystemAppearance_t* appearance) {
    if (!appearance) return;
    
#if wxCHECK_VERSION(3, 3, 0)
    wxSystemAppearance* wx_appearance = reinterpret_cast<wxSystemAppearance*>(appearance);
    delete wx_appearance;
#endif
}

// --- End of Appearance Support Implementation ---
