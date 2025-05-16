#include "wx/wx.h"
#include "wx/filedlg.h"
#include "wx/arrstr.h"
#include "../include/wxdragon.h"
#include "wxd_utils.h" // For WXD_STR_TO_WX_STRING_UTF8_NULL_OK and GET_WX_STRING_RESULT

// --- wxFileDialog ---

WXD_EXPORTED wxd_FileDialog_t* wxd_FileDialog_Create(
    wxd_Window_t* parent,
    const char* message,
    const char* defaultDir,
    const char* defaultFile,
    const char* wildcard,
    wxd_Style_t style,
    int x, int y, // Note: wxFileDialog doesn't directly take x,y,width,height in constructor
    int width, int height) { // These are for wxWindow, but FileDialog often uses default/platform sizing

    wxWindow* parent_wx = (wxWindow*)parent;
    wxFileDialog* dlg = new wxFileDialog(
        parent_wx,
        WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message),
        WXD_STR_TO_WX_STRING_UTF8_NULL_OK(defaultDir),
        WXD_STR_TO_WX_STRING_UTF8_NULL_OK(defaultFile),
        WXD_STR_TO_WX_STRING_UTF8_NULL_OK(wildcard),
        style,
        wxPoint(x,y) // Position can be passed
        // Size is typically not passed or is wxDefaultSize for FileDialog
    );
    // If width/height are not -1 (DEFAULT_SIZE), we could call SetSize, but usually not done for file dialogs.
    if (width != -1 && height != -1) {
        dlg->SetSize(width, height);
    }
    return (wxd_FileDialog_t*)dlg;
}

// ShowModal is inherited from wxd_Dialog_ShowModal

WXD_EXPORTED int wxd_FileDialog_GetPath(wxd_FileDialog_t* self, char* buffer, int bufLen) {
    if (!self) return -1;
    wxFileDialog* dlg = (wxFileDialog*)self;
    return GET_WX_STRING_RESULT(dlg->GetPath(), buffer, bufLen);
}

WXD_EXPORTED void wxd_FileDialog_GetPaths(wxd_FileDialog_t* self, wxd_ArrayString_t* paths_out) {
    if (!self || !paths_out || !paths_out->internal_data) return;
    wxFileDialog* dlg = (wxFileDialog*)self;
    wxArrayString wx_paths;
    dlg->GetPaths(wx_paths);
    wxd_ArrayString_AssignFromWxArrayString(paths_out, wx_paths);
}

WXD_EXPORTED int wxd_FileDialog_GetFilename(wxd_FileDialog_t* self, char* buffer, int bufLen) {
    if (!self) return -1;
    wxFileDialog* dlg = (wxFileDialog*)self;
    return GET_WX_STRING_RESULT(dlg->GetFilename(), buffer, bufLen);
}

WXD_EXPORTED void wxd_FileDialog_GetFilenames(wxd_FileDialog_t* self, wxd_ArrayString_t* filenames_out) {
    if (!self || !filenames_out || !filenames_out->internal_data) return;
    wxFileDialog* dlg = (wxFileDialog*)self;
    wxArrayString wx_filenames;
    dlg->GetFilenames(wx_filenames);
    wxd_ArrayString_AssignFromWxArrayString(filenames_out, wx_filenames);
}

WXD_EXPORTED int wxd_FileDialog_GetDirectory(wxd_FileDialog_t* self, char* buffer, int bufLen) {
    if (!self) return -1;
    wxFileDialog* dlg = (wxFileDialog*)self;
    return GET_WX_STRING_RESULT(dlg->GetDirectory(), buffer, bufLen);
}

WXD_EXPORTED int wxd_FileDialog_GetFilterIndex(wxd_FileDialog_t* self) {
    if (!self) return -1; // Or some other error indicator, wxNOT_FOUND maybe?
    wxFileDialog* dlg = (wxFileDialog*)self;
    return dlg->GetFilterIndex();
}

// --- Optional Setters ---
WXD_EXPORTED void wxd_FileDialog_SetMessage(wxd_FileDialog_t* self, const char* message) {
    if (!self) return;
    ((wxFileDialog*)self)->SetMessage(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message));
}

WXD_EXPORTED void wxd_FileDialog_SetPath(wxd_FileDialog_t* self, const char* path) {
    if (!self) return;
    ((wxFileDialog*)self)->SetPath(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(path));
}

WXD_EXPORTED void wxd_FileDialog_SetDirectory(wxd_FileDialog_t* self, const char* directory) {
    if (!self) return;
    ((wxFileDialog*)self)->SetDirectory(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(directory));
}

WXD_EXPORTED void wxd_FileDialog_SetFilename(wxd_FileDialog_t* self, const char* name) {
    if (!self) return;
    ((wxFileDialog*)self)->SetFilename(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(name));
}

WXD_EXPORTED void wxd_FileDialog_SetWildcard(wxd_FileDialog_t* self, const char* wildCard) {
    if (!self) return;
    ((wxFileDialog*)self)->SetWildcard(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(wildCard));
}

WXD_EXPORTED void wxd_FileDialog_SetFilterIndex(wxd_FileDialog_t* self, int filterIndex) {
    if (!self) return;
    ((wxFileDialog*)self)->SetFilterIndex(filterIndex);
}

// Add missing GetMessage, GetWildcard functions, and GetCurrentlySelectedFilterIndex
WXD_EXPORTED int wxd_FileDialog_GetMessage(wxd_FileDialog_t* self, char* buffer, int bufLen) {
    if (!self) return -1;
    wxFileDialog* dlg = (wxFileDialog*)self;
    return GET_WX_STRING_RESULT(dlg->GetMessage(), buffer, bufLen);
}

WXD_EXPORTED int wxd_FileDialog_GetWildcard(wxd_FileDialog_t* self, char* buffer, int bufLen) {
    if (!self) return -1;
    wxFileDialog* dlg = (wxFileDialog*)self;
    return GET_WX_STRING_RESULT(dlg->GetWildcard(), buffer, bufLen);
}

WXD_EXPORTED int wxd_FileDialog_GetCurrentlySelectedFilterIndex(wxd_FileDialog_t* self) {
    if (!self) return -1;
    wxFileDialog* dlg = (wxFileDialog*)self;
#ifdef __WXMSW__
    return dlg->GetCurrentlySelectedFilterIndex();
#else
    #ifdef __WXOSX__
        return dlg->GetCurrentlySelectedFilterIndex();
    #else
        return -1; // Not implemented on this platform
    #endif
#endif
} 