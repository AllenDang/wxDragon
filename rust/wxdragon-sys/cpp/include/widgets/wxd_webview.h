#ifndef WXD_WEBVIEW_H
#define WXD_WEBVIEW_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Opaque type for wxWebView
typedef struct wxd_WebView wxd_WebView_t;

// Creation
WXD_EXPORTED wxd_WebView_t* wxd_WebView_Create(wxd_Window_t* parent, wxd_Id id, const char* url,
                                               wxd_Point pos, wxd_Size size, long style,
                                               const char* name, const char* backend);

// Navigation
WXD_EXPORTED void wxd_WebView_LoadURL(wxd_WebView_t* self, const char* url);
WXD_EXPORTED void wxd_WebView_Reload(wxd_WebView_t* self, int flags);
WXD_EXPORTED void wxd_WebView_Stop(wxd_WebView_t* self);
WXD_EXPORTED bool wxd_WebView_CanGoBack(wxd_WebView_t* self);
WXD_EXPORTED bool wxd_WebView_CanGoForward(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_GoBack(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_GoForward(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_ClearHistory(wxd_WebView_t* self);

// State
WXD_EXPORTED bool wxd_WebView_IsBusy(wxd_WebView_t* self);
WXD_EXPORTED int wxd_WebView_GetCurrentURL(wxd_WebView_t* self, char* buffer, int len);
WXD_EXPORTED int wxd_WebView_GetCurrentTitle(wxd_WebView_t* self, char* buffer, int len);
WXD_EXPORTED int wxd_WebView_GetPageSource(wxd_WebView_t* self, char* buffer, int len);
WXD_EXPORTED int wxd_WebView_GetPageText(wxd_WebView_t* self, char* buffer, int len);

// Zoom
WXD_EXPORTED bool wxd_WebView_CanSetZoomType(wxd_WebView_t* self, int type);
WXD_EXPORTED int wxd_WebView_GetZoom(wxd_WebView_t* self);
WXD_EXPORTED int wxd_WebView_GetZoomType(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_SetZoom(wxd_WebView_t* self, int zoom);
WXD_EXPORTED void wxd_WebView_SetZoomType(wxd_WebView_t* self, int zoomType);

// Scripting
WXD_EXPORTED int wxd_WebView_RunScript(wxd_WebView_t* self, const char* javascript, char* output, int output_len);

// Clipboard
WXD_EXPORTED bool wxd_WebView_CanCut(wxd_WebView_t* self);
WXD_EXPORTED bool wxd_WebView_CanCopy(wxd_WebView_t* self);
WXD_EXPORTED bool wxd_WebView_CanPaste(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_Cut(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_Copy(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_Paste(wxd_WebView_t* self);
WXD_EXPORTED bool wxd_WebView_CanUndo(wxd_WebView_t* self);
WXD_EXPORTED bool wxd_WebView_CanRedo(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_Undo(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_Redo(wxd_WebView_t* self);

// Selection
WXD_EXPORTED void wxd_WebView_SelectAll(wxd_WebView_t* self);
WXD_EXPORTED bool wxd_WebView_HasSelection(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_DeleteSelection(wxd_WebView_t* self);
WXD_EXPORTED int wxd_WebView_GetSelectedText(wxd_WebView_t* self, char* buffer, int len);
WXD_EXPORTED int wxd_WebView_GetSelectedSource(wxd_WebView_t* self, char* buffer, int len);
WXD_EXPORTED void wxd_WebView_ClearSelection(wxd_WebView_t* self);

// Editing
WXD_EXPORTED bool wxd_WebView_IsEditable(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_SetEditable(wxd_WebView_t* self, bool enable);

// Printing
WXD_EXPORTED void wxd_WebView_Print(wxd_WebView_t* self);

// Context Menu & Dev Tools
WXD_EXPORTED void wxd_WebView_EnableContextMenu(wxd_WebView_t* self, bool enable);
WXD_EXPORTED bool wxd_WebView_IsContextMenuEnabled(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_EnableAccessToDevTools(wxd_WebView_t* self, bool enable);
WXD_EXPORTED bool wxd_WebView_IsAccessToDevToolsEnabled(wxd_WebView_t* self);
WXD_EXPORTED bool wxd_WebView_ShowDevTools(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_EnableBrowserAcceleratorKeys(wxd_WebView_t* self, bool enable);
WXD_EXPORTED bool wxd_WebView_AreBrowserAcceleratorKeysEnabled(wxd_WebView_t* self);

// Zoom Factor
WXD_EXPORTED float wxd_WebView_GetZoomFactor(wxd_WebView_t* self);
WXD_EXPORTED void wxd_WebView_SetZoomFactor(wxd_WebView_t* self, float zoom);

// Page Loading
WXD_EXPORTED void wxd_WebView_SetPage(wxd_WebView_t* self, const char* html, const char* baseUrl);
WXD_EXPORTED long wxd_WebView_Find(wxd_WebView_t* self, const char* text, int flags);

// History
WXD_EXPORTED void wxd_WebView_EnableHistory(wxd_WebView_t* self, bool enable);

// Configuration
WXD_EXPORTED bool wxd_WebView_SetUserAgent(wxd_WebView_t* self, const char* userAgent);
WXD_EXPORTED int wxd_WebView_GetUserAgent(wxd_WebView_t* self, char* buffer, int len);
WXD_EXPORTED bool wxd_WebView_SetProxy(wxd_WebView_t* self, const char* proxy);

// Advanced Scripting
WXD_EXPORTED bool wxd_WebView_AddScriptMessageHandler(wxd_WebView_t* self, const char* name);
WXD_EXPORTED bool wxd_WebView_RemoveScriptMessageHandler(wxd_WebView_t* self, const char* name);
WXD_EXPORTED bool wxd_WebView_AddUserScript(wxd_WebView_t* self, const char* javascript, int injectionTime);
WXD_EXPORTED void wxd_WebView_RemoveAllUserScripts(wxd_WebView_t* self);

// Native Backend
WXD_EXPORTED void* wxd_WebView_GetNativeBackend(wxd_WebView_t* self);
WXD_EXPORTED int wxd_WebView_GetBackend(wxd_WebView_t* self, char* buffer, int len);

// Static utility functions
WXD_EXPORTED bool wxd_WebView_IsBackendAvailable(const char* backend);

#ifdef __cplusplus
}
#endif

#endif // WXD_WEBVIEW_H
