#include "wx/wxprec.h"

#ifndef WX_PRECOMP
#include "wx/wx.h"
#endif

#include "../include/wxdragon.h"
#include "wxd_utils.h"

#if wxdUSE_WEBVIEW

#include "wx/webview.h"

extern "C" {

WXD_EXPORTED wxd_WebView_t*
wxd_WebView_Create(wxd_Window_t* parent, wxd_Id id, const char* url, wxd_Point pos, wxd_Size size,
                   long style, const char* name, const char* backend)
{
    wxWindow* parentWin = (wxWindow*)parent;
    wxString urlStr = url ? wxString::FromUTF8(url) : wxString();
    wxString nameStr = name ? wxString::FromUTF8(name) : wxWebViewNameStr;
    // Use wxWebViewBackendDefault if backend is null or empty string
    // On Windows, wxWebViewBackendDefault is "" which triggers Edge/IE auto-selection
    // On macOS/Linux, wxWebViewBackendDefault is "wxWebViewWebKit"
    wxString backendStr = (!backend || backend[0] == '\0') ? wxWebViewBackendDefault : wxString::FromUTF8(backend);

    wxWebView* webview = wxWebView::New(parentWin, id, urlStr, wxd_cpp_utils::to_wx(pos),
                                        wxd_cpp_utils::to_wx(size), backendStr, style,
                                        nameStr);

    return (wxd_WebView_t*)webview;
}

WXD_EXPORTED void
wxd_WebView_LoadURL(wxd_WebView_t* self, const char* url)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview && url) {
        webview->LoadURL(wxString::FromUTF8(url));
    }
}

WXD_EXPORTED void
wxd_WebView_Reload(wxd_WebView_t* self, int flags)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview) {
        webview->Reload((wxWebViewReloadFlags)flags);
    }
}

WXD_EXPORTED void
wxd_WebView_Stop(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview) {
        webview->Stop();
    }
}

WXD_EXPORTED bool
wxd_WebView_CanGoBack(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->CanGoBack() : false;
}

WXD_EXPORTED bool
wxd_WebView_CanGoForward(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->CanGoForward() : false;
}

WXD_EXPORTED void
wxd_WebView_GoBack(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview) {
        webview->GoBack();
    }
}

WXD_EXPORTED void
wxd_WebView_GoForward(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview) {
        webview->GoForward();
    }
}

WXD_EXPORTED void
wxd_WebView_ClearHistory(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview) {
        webview->ClearHistory();
    }
}

WXD_EXPORTED bool
wxd_WebView_IsBusy(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->IsBusy() : false;
}

WXD_EXPORTED int
wxd_WebView_GetCurrentURL(wxd_WebView_t* self, char* buffer, int len)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return 0;
    return wxd_cpp_utils::copy_wxstring_to_buffer(webview->GetCurrentURL(), buffer, len);
}

WXD_EXPORTED int
wxd_WebView_GetCurrentTitle(wxd_WebView_t* self, char* buffer, int len)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return 0;
    return wxd_cpp_utils::copy_wxstring_to_buffer(webview->GetCurrentTitle(), buffer, len);
}

WXD_EXPORTED int
wxd_WebView_GetPageSource(wxd_WebView_t* self, char* buffer, int len)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return 0;
    return wxd_cpp_utils::copy_wxstring_to_buffer(webview->GetPageSource(), buffer, len);
}

WXD_EXPORTED int
wxd_WebView_GetPageText(wxd_WebView_t* self, char* buffer, int len)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return 0;
    return wxd_cpp_utils::copy_wxstring_to_buffer(webview->GetPageText(), buffer, len);
}

WXD_EXPORTED bool
wxd_WebView_CanSetZoomType(wxd_WebView_t* self, int type)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->CanSetZoomType((wxWebViewZoomType)type) : false;
}

WXD_EXPORTED int
wxd_WebView_GetZoom(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return 0;

#ifdef __WXMSW__
    // On Windows with IE backend, GetZoom() internally calls GetZoomFactor()
    // which can trigger an assertion failure with the optical zoom API.
    // Detect IE backend and return a safe default instead of crashing.
    wxString backendName = webview->GetClassInfo()->GetClassName();
    if (backendName.Contains("IE")) {
        // IE backend - GetZoom is unreliable, return medium as default
        // The zoom type should have been set to Layout at creation time
        // to make SetZoom work, but GetZoom may still fail
        return (int)wxWEBVIEW_ZOOM_MEDIUM;
    }
#endif

    return (int)webview->GetZoom();
}

WXD_EXPORTED int
wxd_WebView_GetZoomType(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? (int)webview->GetZoomType() : 0;
}

WXD_EXPORTED void
wxd_WebView_SetZoom(wxd_WebView_t* self, int zoom)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return;

#ifdef __WXMSW__
    // On Windows with IE backend, SetZoom internally calls SetIEOpticalZoomFactor
    // which can trigger assertion failures. Skip zoom operations on IE backend
    // as they are unreliable.
    wxString backendName = webview->GetClassInfo()->GetClassName();
    if (backendName.Contains("IE")) {
        // IE backend - SetZoom is unreliable, skip the operation
        return;
    }
#endif

    webview->SetZoom((wxWebViewZoom)zoom);
}

WXD_EXPORTED void
wxd_WebView_SetZoomType(wxd_WebView_t* self, int zoomType)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return;

#ifdef __WXMSW__
    // On Windows with IE backend, zoom operations are unreliable and can
    // trigger assertion failures. Skip all zoom type changes on IE.
    wxString backendName = webview->GetClassInfo()->GetClassName();
    if (backendName.Contains("IE")) {
        // IE backend - skip zoom type changes
        return;
    }
#endif

    webview->SetZoomType((wxWebViewZoomType)zoomType);
}

WXD_EXPORTED int
wxd_WebView_RunScript(wxd_WebView_t* self, const char* javascript, char* output, int output_len)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview || !javascript)
        return -1;

    wxString script = wxString::FromUTF8(javascript);
    wxString result;
    bool success = webview->RunScript(script, &result);

    if (!success)
        return -1;

    if (output && output_len > 0) {
        return (int)wxd_cpp_utils::copy_wxstring_to_buffer(result, output, output_len);
    }

    return 0;
}

WXD_EXPORTED bool
wxd_WebView_CanCut(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->CanCut() : false;
}

WXD_EXPORTED bool
wxd_WebView_CanCopy(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->CanCopy() : false;
}

WXD_EXPORTED bool
wxd_WebView_CanPaste(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->CanPaste() : false;
}

WXD_EXPORTED void
wxd_WebView_Cut(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->Cut();
}

WXD_EXPORTED void
wxd_WebView_Copy(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->Copy();
}

WXD_EXPORTED void
wxd_WebView_Paste(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->Paste();
}

WXD_EXPORTED bool
wxd_WebView_CanUndo(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->CanUndo() : false;
}

WXD_EXPORTED bool
wxd_WebView_CanRedo(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->CanRedo() : false;
}

WXD_EXPORTED void
wxd_WebView_Undo(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->Undo();
}

WXD_EXPORTED void
wxd_WebView_Redo(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->Redo();
}

WXD_EXPORTED void
wxd_WebView_SelectAll(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->SelectAll();
}

WXD_EXPORTED bool
wxd_WebView_HasSelection(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->HasSelection() : false;
}

WXD_EXPORTED void
wxd_WebView_DeleteSelection(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->DeleteSelection();
}

WXD_EXPORTED int
wxd_WebView_GetSelectedText(wxd_WebView_t* self, char* buffer, int len)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return 0;
    return wxd_cpp_utils::copy_wxstring_to_buffer(webview->GetSelectedText(), buffer, len);
}

WXD_EXPORTED int
wxd_WebView_GetSelectedSource(wxd_WebView_t* self, char* buffer, int len)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return 0;
    return wxd_cpp_utils::copy_wxstring_to_buffer(webview->GetSelectedSource(), buffer, len);
}

WXD_EXPORTED void
wxd_WebView_ClearSelection(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->ClearSelection();
}

WXD_EXPORTED bool
wxd_WebView_IsEditable(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->IsEditable() : false;
}

WXD_EXPORTED void
wxd_WebView_SetEditable(wxd_WebView_t* self, bool enable)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->SetEditable(enable);
}

WXD_EXPORTED void
wxd_WebView_Print(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->Print();
}

// Context Menu & Dev Tools
WXD_EXPORTED void
wxd_WebView_EnableContextMenu(wxd_WebView_t* self, bool enable)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->EnableContextMenu(enable);
}

WXD_EXPORTED bool
wxd_WebView_IsContextMenuEnabled(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->IsContextMenuEnabled() : false;
}

WXD_EXPORTED void
wxd_WebView_EnableAccessToDevTools(wxd_WebView_t* self, bool enable)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->EnableAccessToDevTools(enable);
}

WXD_EXPORTED bool
wxd_WebView_IsAccessToDevToolsEnabled(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->IsAccessToDevToolsEnabled() : false;
}

WXD_EXPORTED bool
wxd_WebView_ShowDevTools(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->ShowDevTools() : false;
}

WXD_EXPORTED void
wxd_WebView_EnableBrowserAcceleratorKeys(wxd_WebView_t* self, bool enable)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->EnableBrowserAcceleratorKeys(enable);
}

WXD_EXPORTED bool
wxd_WebView_AreBrowserAcceleratorKeysEnabled(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->AreBrowserAcceleratorKeysEnabled() : false;
}

// Zoom Factor
WXD_EXPORTED float
wxd_WebView_GetZoomFactor(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return 1.0f;

#ifdef __WXMSW__
    // On Windows with IE backend, GetZoomFactor() calls GetIEOpticalZoomFactor()
    // which triggers an assertion failure. Return 1.0 as safe default.
    wxString backendName = webview->GetClassInfo()->GetClassName();
    if (backendName.Contains("IE")) {
        // IE backend - GetZoomFactor is unreliable, return 1.0 (100%)
        return 1.0f;
    }
#endif

    return webview->GetZoomFactor();
}

WXD_EXPORTED void
wxd_WebView_SetZoomFactor(wxd_WebView_t* self, float zoom)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return;

#ifdef __WXMSW__
    // On Windows with IE backend, SetZoomFactor uses optical zoom which
    // may not work reliably. Skip to avoid potential issues.
    // Use SetZoom() with discrete zoom levels instead for IE.
    wxString backendName = webview->GetClassInfo()->GetClassName();
    if (backendName.Contains("IE")) {
        // IE backend - SetZoomFactor is unreliable, ignore the call
        return;
    }
#endif

    webview->SetZoomFactor(zoom);
}

// Page Loading
WXD_EXPORTED void
wxd_WebView_SetPage(wxd_WebView_t* self, const char* html, const char* baseUrl)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview && html) {
        wxString htmlStr = wxString::FromUTF8(html);
        wxString baseUrlStr = baseUrl ? wxString::FromUTF8(baseUrl) : wxString();
        webview->SetPage(htmlStr, baseUrlStr);
    }
}

WXD_EXPORTED long
wxd_WebView_Find(wxd_WebView_t* self, const char* text, int flags)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview || !text)
        return -1;

    wxString textStr = wxString::FromUTF8(text);
    return webview->Find(textStr, flags);
}

// History
WXD_EXPORTED void
wxd_WebView_EnableHistory(wxd_WebView_t* self, bool enable)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->EnableHistory(enable);
}

// Configuration
WXD_EXPORTED bool
wxd_WebView_SetUserAgent(wxd_WebView_t* self, const char* userAgent)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview || !userAgent)
        return false;

    wxString userAgentStr = wxString::FromUTF8(userAgent);
    return webview->SetUserAgent(userAgentStr);
}

WXD_EXPORTED int
wxd_WebView_GetUserAgent(wxd_WebView_t* self, char* buffer, int len)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return 0;

    return wxd_cpp_utils::copy_wxstring_to_buffer(webview->GetUserAgent(), buffer, len);
}

WXD_EXPORTED bool
wxd_WebView_SetProxy(wxd_WebView_t* self, const char* proxy)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview || !proxy)
        return false;

    wxString proxyStr = wxString::FromUTF8(proxy);
    return webview->SetProxy(proxyStr);
}

// Advanced Scripting
WXD_EXPORTED bool
wxd_WebView_AddScriptMessageHandler(wxd_WebView_t* self, const char* name)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview || !name)
        return false;

    wxString nameStr = wxString::FromUTF8(name);
    return webview->AddScriptMessageHandler(nameStr);
}

WXD_EXPORTED bool
wxd_WebView_RemoveScriptMessageHandler(wxd_WebView_t* self, const char* name)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview || !name)
        return false;

    wxString nameStr = wxString::FromUTF8(name);
    return webview->RemoveScriptMessageHandler(nameStr);
}

WXD_EXPORTED bool
wxd_WebView_AddUserScript(wxd_WebView_t* self, const char* javascript, int injectionTime)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview || !javascript)
        return false;

    wxString scriptStr = wxString::FromUTF8(javascript);
    return webview->AddUserScript(scriptStr, (wxWebViewUserScriptInjectionTime)injectionTime);
}

WXD_EXPORTED void
wxd_WebView_RemoveAllUserScripts(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    if (webview)
        webview->RemoveAllUserScripts();
}

// Native Backend
WXD_EXPORTED void*
wxd_WebView_GetNativeBackend(wxd_WebView_t* self)
{
    wxWebView* webview = (wxWebView*)self;
    return webview ? webview->GetNativeBackend() : nullptr;
}

WXD_EXPORTED int
wxd_WebView_GetBackend(wxd_WebView_t* self, char* buffer, int len)
{
    wxWebView* webview = (wxWebView*)self;
    if (!webview)
        return 0;

    // Get the class name to determine the backend
    wxString backendName = webview->GetClassInfo()->GetClassName();
    return wxd_cpp_utils::copy_wxstring_to_buffer(backendName, buffer, len);
}

WXD_EXPORTED bool
wxd_WebView_IsBackendAvailable(const char* backend)
{
    wxString backendStr = backend ? wxString::FromUTF8(backend) : wxWebViewBackendDefault;
    return wxWebView::IsBackendAvailable(backendStr);
}

} // extern "C"

#endif // wxdUSE_WEBVIEW
