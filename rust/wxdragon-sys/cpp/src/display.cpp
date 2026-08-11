#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"

#include <wx/display.h>

namespace {

// wxDisplay's constructor asserts (and, worse, leaves the object in a corrupt state on
// some platforms) if given an index that isn't in [0, GetCount()). wxDisplay::IsOk() can't
// help here since the assert fires inside the constructor, before IsOk() would even run, so
// every wrapper below must check this itself before ever constructing a wxDisplay.
bool
is_valid_index(unsigned int index)
{
    return index < wxDisplay::GetCount();
}

} // namespace

extern "C" {

WXD_EXPORTED unsigned int
wxd_Display_GetCount(void)
{
    return wxDisplay::GetCount();
}

WXD_EXPORTED int
wxd_Display_GetFromPoint(wxd_Point pt)
{
    return wxDisplay::GetFromPoint(wxPoint(pt.x, pt.y));
}

WXD_EXPORTED int
wxd_Display_GetFromRect(wxd_Rect rect)
{
    return wxDisplay::GetFromRect(wxRect(rect.x, rect.y, rect.width, rect.height));
}

WXD_EXPORTED int
wxd_Display_GetFromWindow(wxd_Window_t* window)
{
    return wxDisplay::GetFromWindow(reinterpret_cast<const wxWindow*>(window));
}

WXD_EXPORTED bool
wxd_Display_IsOk(unsigned int index)
{
    return is_valid_index(index) && wxDisplay(index).IsOk();
}

WXD_EXPORTED wxd_Rect
wxd_Display_GetGeometry(unsigned int index)
{
    if (!is_valid_index(index))
        return { 0, 0, 0, 0 };
    wxRect rect = wxDisplay(index).GetGeometry();
    return { rect.x, rect.y, rect.width, rect.height };
}

WXD_EXPORTED wxd_Rect
wxd_Display_GetClientArea(unsigned int index)
{
    if (!is_valid_index(index))
        return { 0, 0, 0, 0 };
    wxRect rect = wxDisplay(index).GetClientArea();
    return { rect.x, rect.y, rect.width, rect.height };
}

WXD_EXPORTED int
wxd_Display_GetDepth(unsigned int index)
{
    if (!is_valid_index(index))
        return 0;
    return wxDisplay(index).GetDepth();
}

WXD_EXPORTED wxd_Size
wxd_Display_GetPPI(unsigned int index)
{
    if (!is_valid_index(index))
        return { 0, 0 };
    wxSize ppi = wxDisplay(index).GetPPI();
    return { ppi.GetWidth(), ppi.GetHeight() };
}

WXD_EXPORTED double
wxd_Display_GetScaleFactor(unsigned int index)
{
    if (!is_valid_index(index))
        return 1.0;
    return wxDisplay(index).GetScaleFactor();
}

WXD_EXPORTED int
wxd_Display_GetName(unsigned int index, char* buffer, size_t buffer_len)
{
    if (!is_valid_index(index))
        return (int)wxd_cpp_utils::copy_wxstring_to_buffer(wxString(), buffer, buffer_len);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(wxDisplay(index).GetName(), buffer, buffer_len);
}

WXD_EXPORTED bool
wxd_Display_IsPrimary(unsigned int index)
{
    if (!is_valid_index(index))
        return false;
    return wxDisplay(index).IsPrimary();
}

WXD_EXPORTED void
wxd_Display_InvalidateCache(void)
{
    wxDisplay::InvalidateCache();
}

} // extern "C"
