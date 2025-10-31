#include <wx/wxprec.h>
#include <wx/wx.h>
#include <wx/cmdline.h>
#include "../include/wxdragon.h"
#include <wx/app.h>
#include <wx/image.h>
#include <cstdlib>
#include <wx/private/safecall.h>
#include <wx/scopeguard.h>
#include <vector>
#include <utility>

// --- Globals ---
// Store the C callback and user data provided to wxd_Main
static wxd_OnInitCallback g_OnInitCallback = nullptr;
static void* g_OnInitUserData = nullptr;

// Function to process Rust callbacks, implemented in Rust
extern "C" int
process_rust_callbacks();

// --- Internal C++ App Class ---

class WxdApp : public wxApp {
public:
    // Called by wxWidgets framework on application startup.
    virtual bool
    OnInit() override;

    // Accept any command line parameters without errors
    virtual void
    OnInitCmdLine(wxCmdLineParser& parser) override;

    virtual bool
    OnCmdLineParsed(wxCmdLineParser& parser) override;

    // Idle event handler to process callbacks
    void
    OnIdle(wxIdleEvent& event);

    // Optional: Override OnExit for cleanup if needed
    // virtual int OnExit() override;

#ifdef __WXOSX__
    // macOS-specific overrides
    virtual void
    MacOpenFiles(const wxArrayString& fileNames) override;
    virtual void
    MacOpenURL(const wxString& url) override;
    virtual void
    MacNewFile() override;
    virtual void
    MacReopenApp() override;
    virtual void
    MacPrintFiles(const wxArrayString& fileNames) override;

    // Store multiple callbacks for each event type
    struct MacCallbackList {
        std::vector<std::pair<wxd_MacOpenFilesCallback, void*>> openFiles;
        std::vector<std::pair<wxd_MacOpenURLCallback, void*>> openURL;
        std::vector<std::pair<wxd_MacNewFileCallback, void*>> newFile;
        std::vector<std::pair<wxd_MacReopenAppCallback, void*>> reopenApp;
        std::vector<std::pair<wxd_MacPrintFilesCallback, void*>> printFiles;
    } m_macCallbacks;
#endif
};

// Implementation of OnInit - this is where we call the C callback
bool
WxdApp::OnInit()
{
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
    }
    else {
        // Should not happen if wxd_Main is used correctly
        WXD_LOG_ERROR("wxDragon: No OnInit callback provided to wxd_Main.");
        return false;
    }
}

// Process callbacks on idle
void
WxdApp::OnIdle(wxIdleEvent& event)
{
    // Process any pending Rust callbacks
    int callbacks_processed = process_rust_callbacks();

    // Only request more idle events if there were callbacks to process
    // This prevents unnecessary CPU usage when the app is idle
    if (callbacks_processed > 0) {
        event.RequestMore();
    }
}

// Configure command line parser to accept any parameters (no options).
void
WxdApp::OnInitCmdLine(wxCmdLineParser& parser)
{
    // Call base to keep standard behaviour, then override as needed
    wxApp::OnInitCmdLine(parser);

    // Disable option parsing so tokens starting with '-' are not treated as options
    parser.EnableLongOptions(false);
    parser.SetSwitchChars("");

    // Accept any number of free string parameters
    static const wxCmdLineEntryDesc cmdLineDesc[] = {
        { wxCMD_LINE_PARAM, nullptr, nullptr, "arg", wxCMD_LINE_VAL_STRING,
          wxCMD_LINE_PARAM_MULTIPLE | wxCMD_LINE_PARAM_OPTIONAL },
        { wxCMD_LINE_NONE, nullptr, nullptr, nullptr, wxCMD_LINE_VAL_NONE, 0 }
    };
    parser.SetDesc(cmdLineDesc);
}

// Always accept the parsed command line
bool
WxdApp::OnCmdLineParsed(wxCmdLineParser& parser)
{
    // Don't enforce any checks here; accept everything
    return true;
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
int
wxd_Main(int argc, char** argv, wxd_OnInitCallback on_init_cb, void* userData)
{
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
            if (!wxTheApp->CallOnInit()) {
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
        });

    wxEntryCleanup();
    g_OnInitCallback = nullptr;
    g_OnInitUserData = nullptr;
    return result;
}

// Gets the handle to the global application instance.
wxd_App_t*
wxd_GetApp()
{
    // wxTheApp is the global pointer to the wxApp instance
    return reinterpret_cast<wxd_App_t*>(wxTheApp);
}

// Sets the top window (main frame) for the application.
void
wxd_App_SetTopWindow(wxd_App_t* app, wxd_Window_t* window)
{
    if (!app || !window)
        return;
    WxdApp* wx_app = reinterpret_cast<WxdApp*>(app);
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wx_app->SetTopWindow(wx_window);
}

// Manual callback processing for cases where we need to trigger it
void
wxd_App_ProcessCallbacks()
{
    process_rust_callbacks();
}

// Implementation for wxd_free_int_array
void
wxd_free_int_array(int* ptr)
{
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
wxd_AppearanceResult
wxd_App_SetAppearance(wxd_App_t* app, wxd_Appearance appearance)
{
    if (!app)
        return WXD_APPEARANCE_RESULT_FAILURE;

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
wxd_SystemAppearance_t*
wxd_SystemSettings_GetAppearance()
{
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
bool
wxd_SystemAppearance_IsDark(wxd_SystemAppearance_t* appearance)
{
    if (!appearance)
        return false;

#if wxCHECK_VERSION(3, 3, 0)
    wxSystemAppearance* wx_appearance = reinterpret_cast<wxSystemAppearance*>(appearance);
    return wx_appearance->IsDark();
#else
    return false;
#endif
}

// Check if the system background is dark
bool
wxd_SystemAppearance_IsUsingDarkBackground(wxd_SystemAppearance_t* appearance)
{
    if (!appearance)
        return false;

#if wxCHECK_VERSION(3, 3, 0)
    wxSystemAppearance* wx_appearance = reinterpret_cast<wxSystemAppearance*>(appearance);
    return wx_appearance->IsUsingDarkBackground();
#else
    return false;
#endif
}

// Get the system appearance name (mainly for macOS)
WXD_EXPORTED int
wxd_SystemAppearance_GetName(const wxd_SystemAppearance_t* appearance, char* out, size_t out_len)
{
    if (!appearance)
        return -1;

#if wxCHECK_VERSION(3, 3, 0)
    const wxSystemAppearance* a = reinterpret_cast<const wxSystemAppearance*>(appearance);
    wxString name = a->GetName();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(name, out, out_len);
#endif
    return -1;
}

// Free system appearance object
void
wxd_SystemAppearance_Destroy(wxd_SystemAppearance_t* appearance)
{
    if (!appearance)
        return;

#if wxCHECK_VERSION(3, 3, 0)
    wxSystemAppearance* wx_appearance = reinterpret_cast<wxSystemAppearance*>(appearance);
    delete wx_appearance;
#endif
}

// --- End of Appearance Support Implementation ---

// --- macOS-specific App Event Handlers Implementation ---

#ifdef __WXOSX__

// MacOpenFiles override - calls all registered handlers
void
WxdApp::MacOpenFiles(const wxArrayString& fileNames)
{
    if (m_macCallbacks.openFiles.empty()) {
        // No callbacks registered, use default behavior
        wxApp::MacOpenFiles(fileNames);
        return;
    }

    // Convert wxArrayString to C string array
    size_t count = fileNames.GetCount();
    std::vector<std::string> strings;
    std::vector<const char*> cStrings;
    strings.reserve(count);
    cStrings.reserve(count);

    for (size_t i = 0; i < count; i++) {
        strings.push_back(fileNames[i].ToStdString());
        cStrings.push_back(strings.back().c_str());
    }

    // Call ALL registered Rust callbacks
    for (const auto& pair : m_macCallbacks.openFiles) {
        if (pair.first) {
            pair.first(pair.second, cStrings.data(), static_cast<int>(count));
        }
    }
}

// MacOpenURL override - calls all registered handlers
void
WxdApp::MacOpenURL(const wxString& url)
{
    if (m_macCallbacks.openURL.empty()) {
        wxApp::MacOpenURL(url);
        return;
    }

    std::string urlStr = url.ToStdString();

    // Call ALL registered Rust callbacks
    for (const auto& pair : m_macCallbacks.openURL) {
        if (pair.first) {
            pair.first(pair.second, urlStr.c_str());
        }
    }
}

// MacNewFile override - calls all registered handlers
void
WxdApp::MacNewFile()
{
    if (m_macCallbacks.newFile.empty()) {
        wxApp::MacNewFile();
        return;
    }

    // Call ALL registered Rust callbacks
    for (const auto& pair : m_macCallbacks.newFile) {
        if (pair.first) {
            pair.first(pair.second);
        }
    }
}

// MacReopenApp override - calls all registered handlers
void
WxdApp::MacReopenApp()
{
    if (m_macCallbacks.reopenApp.empty()) {
        wxApp::MacReopenApp();
        return;
    }

    // Call ALL registered Rust callbacks
    for (const auto& pair : m_macCallbacks.reopenApp) {
        if (pair.first) {
            pair.first(pair.second);
        }
    }
}

// MacPrintFiles override - calls all registered handlers
void
WxdApp::MacPrintFiles(const wxArrayString& fileNames)
{
    if (m_macCallbacks.printFiles.empty()) {
        wxApp::MacPrintFiles(fileNames);
        return;
    }

    // Convert wxArrayString to C string array
    size_t count = fileNames.GetCount();
    std::vector<std::string> strings;
    std::vector<const char*> cStrings;
    strings.reserve(count);
    cStrings.reserve(count);

    for (size_t i = 0; i < count; i++) {
        strings.push_back(fileNames[i].ToStdString());
        cStrings.push_back(strings.back().c_str());
    }

    // Call ALL registered Rust callbacks
    for (const auto& pair : m_macCallbacks.printFiles) {
        if (pair.first) {
            pair.first(pair.second, cStrings.data(), static_cast<int>(count));
        }
    }
}

#endif // __WXOSX__

// Registration functions - add handlers to the callback lists
void
wxd_App_AddMacOpenFilesHandler(wxd_App_t* app, wxd_MacOpenFilesCallback callback, void* userData)
{
#ifdef __WXOSX__
    if (!app || !callback)
        return;
    WxdApp* wx_app = reinterpret_cast<WxdApp*>(app);
    wx_app->m_macCallbacks.openFiles.push_back(std::make_pair(callback, userData));
#endif
}

void
wxd_App_AddMacOpenURLHandler(wxd_App_t* app, wxd_MacOpenURLCallback callback, void* userData)
{
#ifdef __WXOSX__
    if (!app || !callback)
        return;
    WxdApp* wx_app = reinterpret_cast<WxdApp*>(app);
    wx_app->m_macCallbacks.openURL.push_back(std::make_pair(callback, userData));
#endif
}

void
wxd_App_AddMacNewFileHandler(wxd_App_t* app, wxd_MacNewFileCallback callback, void* userData)
{
#ifdef __WXOSX__
    if (!app || !callback)
        return;
    WxdApp* wx_app = reinterpret_cast<WxdApp*>(app);
    wx_app->m_macCallbacks.newFile.push_back(std::make_pair(callback, userData));
#endif
}

void
wxd_App_AddMacReopenAppHandler(wxd_App_t* app, wxd_MacReopenAppCallback callback, void* userData)
{
#ifdef __WXOSX__
    if (!app || !callback)
        return;
    WxdApp* wx_app = reinterpret_cast<WxdApp*>(app);
    wx_app->m_macCallbacks.reopenApp.push_back(std::make_pair(callback, userData));
#endif
}

void
wxd_App_AddMacPrintFilesHandler(wxd_App_t* app, wxd_MacPrintFilesCallback callback, void* userData)
{
#ifdef __WXOSX__
    if (!app || !callback)
        return;
    WxdApp* wx_app = reinterpret_cast<WxdApp*>(app);
    wx_app->m_macCallbacks.printFiles.push_back(std::make_pair(callback, userData));
#endif
}

// --- End of macOS-specific App Event Handlers Implementation ---
