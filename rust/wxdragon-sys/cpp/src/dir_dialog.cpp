#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"

#include <wx/dirdlg.h>

WXD_EXPORTED wxd_DirDialog_t*
wxd_DirDialog_Create(wxd_Window_t* parent, const char* message, const char* defaultPath,
                     wxd_Style_t style, int x, int y, int width, int height)
{
    wxWindow* parent_wx = (wxWindow*)parent;

    // Default position/size if not specified
    wxPoint pos = (x == -1 && y == -1) ? wxDefaultPosition : wxPoint(x, y);
    wxSize size = (width == -1 && height == -1) ? wxDefaultSize : wxSize(width, height);

    wxDirDialog* dialog = new wxDirDialog(parent_wx, WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message),
                                          WXD_STR_TO_WX_STRING_UTF8_NULL_OK(defaultPath), style,
                                          pos, size);

    return reinterpret_cast<wxd_DirDialog_t*>(dialog);
}

WXD_EXPORTED size_t
wxd_DirDialog_GetPath(const wxd_DirDialog_t* self, char* buffer, size_t bufLen)
{
    if (!self)
        return 0;

    const wxDirDialog* dialog = reinterpret_cast<const wxDirDialog*>(self);
    wxString path = dialog->GetPath();
    wxScopedCharBuffer pathBuf = path.ToUTF8();
    if (buffer && bufLen > 0) {
        size_t copyLen = std::min(static_cast<size_t>(pathBuf.length()), bufLen - 1);
        std::memcpy(buffer, pathBuf.data(), copyLen);
        buffer[copyLen] = '\0';
    }

    return pathBuf.length();
}

WXD_EXPORTED void
wxd_DirDialog_SetPath(wxd_DirDialog_t* self, const char* path)
{
    if (!self || !path)
        return;

    wxDirDialog* dialog = reinterpret_cast<wxDirDialog*>(self);
    dialog->SetPath(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(path));
}

WXD_EXPORTED int
wxd_DirDialog_GetMessage(wxd_DirDialog_t* self, char* buffer, int bufLen)
{
    if (!self || !buffer || bufLen <= 0)
        return 0;

    wxDirDialog* dialog = reinterpret_cast<wxDirDialog*>(self);
    wxString message = dialog->GetMessage();

    return wxd_cpp_utils::copy_wxstring_to_buffer(message, buffer, bufLen);
}

WXD_EXPORTED void
wxd_DirDialog_SetMessage(wxd_DirDialog_t* self, const char* message)
{
    if (!self || !message)
        return;

    wxDirDialog* dialog = reinterpret_cast<wxDirDialog*>(self);
    dialog->SetMessage(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message));
}