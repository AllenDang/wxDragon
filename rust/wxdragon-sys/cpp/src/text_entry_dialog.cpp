#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wx/textdlg.h"
#include "../include/wxdragon.h"
#include "wxd_utils.h" // For string helpers if needed, and wxd_OnWindowDestroy

WXD_EXPORTED wxd_TextEntryDialog_t*
wxd_TextEntryDialog_Create(wxd_Window_t* parent, const char* message, const char* caption,
                           const char* defaultValue, wxd_Style_t style, int x, int y, int width,
                           int height)
{
    wxWindow* parent_wx = (wxWindow*)parent;

    wxPoint pos = wxd_cpp_utils::to_wx(wxd_Point{x, y});
    // wxTextEntryDialog doesn't take size in constructor, handled by wxWidgets

    wxTextEntryDialog* dlg =
        new wxTextEntryDialog(parent_wx, WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message),
                              WXD_STR_TO_WX_STRING_UTF8_NULL_OK(caption),
                              WXD_STR_TO_WX_STRING_UTF8_NULL_OK(defaultValue), style, pos);

    // Optional: Set size if provided, though usually unnecessary for text entry dialogs.
    // Use OR (not AND) so that specifying just one axis still applies - only skip
    // when BOTH width and height are the -1 "unspecified" sentinel.
    if (width != -1 || height != -1) {
        wxSize size = wxd_cpp_utils::to_wx(wxd_Size{width, height});
        dlg->SetSize(size.GetWidth(), size.GetHeight());
    }

    // Don't bind wxd_OnWindowDestroy here; rely on Rust Drop calling wxd_Window_Destroy

    return (wxd_TextEntryDialog_t*)dlg;
}

// ShowModal is inherited from wxd_Dialog_ShowModal

WXD_EXPORTED int
wxd_TextEntryDialog_GetValue(wxd_TextEntryDialog_t* self, char* buffer, size_t bufLen)
{
    if (!self)
        return -1;
    wxTextEntryDialog* dlg = (wxTextEntryDialog*)self;
    wxString val = dlg->GetValue();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(val, buffer, bufLen);
}

// Optional: Setter implementation if uncommented in header
/*
WXD_EXPORTED void wxd_TextEntryDialog_SetValue(wxd_TextEntryDialog_t* self, const char* value) {
    if (!self) return;
    wxTextEntryDialog* dlg = (wxTextEntryDialog*)self;
    dlg->SetValue(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(value));
}
*/