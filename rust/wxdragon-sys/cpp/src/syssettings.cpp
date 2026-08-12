#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/settings.h>

namespace {

wxSystemColour
map_system_colour(wxd_SystemColour index)
{
    switch (index) {
    case WXD_SYS_COLOUR_SCROLLBAR:
        return wxSYS_COLOUR_SCROLLBAR;
    case WXD_SYS_COLOUR_DESKTOP:
        return wxSYS_COLOUR_DESKTOP;
    case WXD_SYS_COLOUR_ACTIVECAPTION:
        return wxSYS_COLOUR_ACTIVECAPTION;
    case WXD_SYS_COLOUR_INACTIVECAPTION:
        return wxSYS_COLOUR_INACTIVECAPTION;
    case WXD_SYS_COLOUR_MENU:
        return wxSYS_COLOUR_MENU;
    case WXD_SYS_COLOUR_WINDOW:
        return wxSYS_COLOUR_WINDOW;
    case WXD_SYS_COLOUR_WINDOWFRAME:
        return wxSYS_COLOUR_WINDOWFRAME;
    case WXD_SYS_COLOUR_MENUTEXT:
        return wxSYS_COLOUR_MENUTEXT;
    case WXD_SYS_COLOUR_WINDOWTEXT:
        return wxSYS_COLOUR_WINDOWTEXT;
    case WXD_SYS_COLOUR_CAPTIONTEXT:
        return wxSYS_COLOUR_CAPTIONTEXT;
    case WXD_SYS_COLOUR_ACTIVEBORDER:
        return wxSYS_COLOUR_ACTIVEBORDER;
    case WXD_SYS_COLOUR_INACTIVEBORDER:
        return wxSYS_COLOUR_INACTIVEBORDER;
    case WXD_SYS_COLOUR_APPWORKSPACE:
        return wxSYS_COLOUR_APPWORKSPACE;
    case WXD_SYS_COLOUR_HIGHLIGHT:
        return wxSYS_COLOUR_HIGHLIGHT;
    case WXD_SYS_COLOUR_HIGHLIGHTTEXT:
        return wxSYS_COLOUR_HIGHLIGHTTEXT;
    case WXD_SYS_COLOUR_BTNFACE:
        return wxSYS_COLOUR_BTNFACE;
    case WXD_SYS_COLOUR_BTNSHADOW:
        return wxSYS_COLOUR_BTNSHADOW;
    case WXD_SYS_COLOUR_GRAYTEXT:
        return wxSYS_COLOUR_GRAYTEXT;
    case WXD_SYS_COLOUR_BTNTEXT:
        return wxSYS_COLOUR_BTNTEXT;
    case WXD_SYS_COLOUR_INACTIVECAPTIONTEXT:
        return wxSYS_COLOUR_INACTIVECAPTIONTEXT;
    case WXD_SYS_COLOUR_BTNHIGHLIGHT:
        return wxSYS_COLOUR_BTNHIGHLIGHT;
    case WXD_SYS_COLOUR_3DDKSHADOW:
        return wxSYS_COLOUR_3DDKSHADOW;
    case WXD_SYS_COLOUR_3DLIGHT:
        return wxSYS_COLOUR_3DLIGHT;
    case WXD_SYS_COLOUR_INFOTEXT:
        return wxSYS_COLOUR_INFOTEXT;
    case WXD_SYS_COLOUR_INFOBK:
        return wxSYS_COLOUR_INFOBK;
    case WXD_SYS_COLOUR_LISTBOX:
        return wxSYS_COLOUR_LISTBOX;
    case WXD_SYS_COLOUR_HOTLIGHT:
        return wxSYS_COLOUR_HOTLIGHT;
    case WXD_SYS_COLOUR_GRADIENTACTIVECAPTION:
        return wxSYS_COLOUR_GRADIENTACTIVECAPTION;
    case WXD_SYS_COLOUR_GRADIENTINACTIVECAPTION:
        return wxSYS_COLOUR_GRADIENTINACTIVECAPTION;
    case WXD_SYS_COLOUR_MENUHILIGHT:
        return wxSYS_COLOUR_MENUHILIGHT;
    case WXD_SYS_COLOUR_MENUBAR:
        return wxSYS_COLOUR_MENUBAR;
    case WXD_SYS_COLOUR_LISTBOXTEXT:
        return wxSYS_COLOUR_LISTBOXTEXT;
    case WXD_SYS_COLOUR_LISTBOXHIGHLIGHTTEXT:
        return wxSYS_COLOUR_LISTBOXHIGHLIGHTTEXT;
    case WXD_SYS_COLOUR_GRIDLINES:
        return wxSYS_COLOUR_GRIDLINES;
    case WXD_SYS_COLOUR_LISTBOXHIGHLIGHT:
        return wxSYS_COLOUR_LISTBOXHIGHLIGHT;
    }
    return wxSYS_COLOUR_WINDOW;
}

wxSystemFont
map_system_font(wxd_SystemFont index)
{
    switch (index) {
    case WXD_SYS_OEM_FIXED_FONT:
        return wxSYS_OEM_FIXED_FONT;
    case WXD_SYS_ANSI_FIXED_FONT:
        return wxSYS_ANSI_FIXED_FONT;
    case WXD_SYS_ANSI_VAR_FONT:
        return wxSYS_ANSI_VAR_FONT;
    case WXD_SYS_SYSTEM_FONT:
        return wxSYS_SYSTEM_FONT;
    case WXD_SYS_DEVICE_DEFAULT_FONT:
        return wxSYS_DEVICE_DEFAULT_FONT;
    case WXD_SYS_SYSTEM_FIXED_FONT:
        return wxSYS_SYSTEM_FIXED_FONT;
    case WXD_SYS_DEFAULT_GUI_FONT:
        return wxSYS_DEFAULT_GUI_FONT;
    }
    return wxSYS_DEFAULT_GUI_FONT;
}

wxSystemMetric
map_system_metric(wxd_SystemMetric index)
{
    switch (index) {
    case WXD_SYS_MOUSE_BUTTONS:
        return wxSYS_MOUSE_BUTTONS;
    case WXD_SYS_BORDER_X:
        return wxSYS_BORDER_X;
    case WXD_SYS_BORDER_Y:
        return wxSYS_BORDER_Y;
    case WXD_SYS_CURSOR_SIZE:
        return wxSYS_CURSOR_SIZE;
    case WXD_SYS_DCLICK_X:
        return wxSYS_DCLICK_X;
    case WXD_SYS_DCLICK_Y:
        return wxSYS_DCLICK_Y;
    case WXD_SYS_DRAG_X:
        return wxSYS_DRAG_X;
    case WXD_SYS_DRAG_Y:
        return wxSYS_DRAG_Y;
    case WXD_SYS_EDGE_X:
        return wxSYS_EDGE_X;
    case WXD_SYS_EDGE_Y:
        return wxSYS_EDGE_Y;
    case WXD_SYS_HSCROLL_ARROW_X:
        return wxSYS_HSCROLL_ARROW_X;
    case WXD_SYS_HSCROLL_ARROW_Y:
        return wxSYS_HSCROLL_ARROW_Y;
    case WXD_SYS_HTHUMB_X:
        return wxSYS_HTHUMB_X;
    case WXD_SYS_ICON_X:
        return wxSYS_ICON_X;
    case WXD_SYS_ICON_Y:
        return wxSYS_ICON_Y;
    case WXD_SYS_ICONSPACING_X:
        return wxSYS_ICONSPACING_X;
    case WXD_SYS_ICONSPACING_Y:
        return wxSYS_ICONSPACING_Y;
    case WXD_SYS_WINDOWMIN_X:
        return wxSYS_WINDOWMIN_X;
    case WXD_SYS_WINDOWMIN_Y:
        return wxSYS_WINDOWMIN_Y;
    case WXD_SYS_SCREEN_X:
        return wxSYS_SCREEN_X;
    case WXD_SYS_SCREEN_Y:
        return wxSYS_SCREEN_Y;
    case WXD_SYS_FRAMESIZE_X:
        return wxSYS_FRAMESIZE_X;
    case WXD_SYS_FRAMESIZE_Y:
        return wxSYS_FRAMESIZE_Y;
    case WXD_SYS_SMALLICON_X:
        return wxSYS_SMALLICON_X;
    case WXD_SYS_SMALLICON_Y:
        return wxSYS_SMALLICON_Y;
    case WXD_SYS_HSCROLL_Y:
        return wxSYS_HSCROLL_Y;
    case WXD_SYS_VSCROLL_X:
        return wxSYS_VSCROLL_X;
    case WXD_SYS_VSCROLL_ARROW_X:
        return wxSYS_VSCROLL_ARROW_X;
    case WXD_SYS_VSCROLL_ARROW_Y:
        return wxSYS_VSCROLL_ARROW_Y;
    case WXD_SYS_VTHUMB_Y:
        return wxSYS_VTHUMB_Y;
    case WXD_SYS_CAPTION_Y:
        return wxSYS_CAPTION_Y;
    case WXD_SYS_MENU_Y:
        return wxSYS_MENU_Y;
    case WXD_SYS_NETWORK_PRESENT:
        return wxSYS_NETWORK_PRESENT;
    case WXD_SYS_PENWINDOWS_PRESENT:
        return wxSYS_PENWINDOWS_PRESENT;
    case WXD_SYS_SHOW_SOUNDS:
        return wxSYS_SHOW_SOUNDS;
    case WXD_SYS_SWAP_BUTTONS:
        return wxSYS_SWAP_BUTTONS;
    case WXD_SYS_DCLICK_MSEC:
        return wxSYS_DCLICK_MSEC;
    case WXD_SYS_CARET_ON_MSEC:
        return wxSYS_CARET_ON_MSEC;
    case WXD_SYS_CARET_OFF_MSEC:
        return wxSYS_CARET_OFF_MSEC;
    case WXD_SYS_CARET_TIMEOUT_MSEC:
        return wxSYS_CARET_TIMEOUT_MSEC;
    }
    return wxSYS_SCREEN_X;
}

} // namespace

extern "C" {

WXD_EXPORTED wxd_Colour_t
wxd_SystemSettings_GetColour(wxd_SystemColour index)
{
    wxColour colour = wxSystemSettings::GetColour(map_system_colour(index));
    return { colour.Red(), colour.Green(), colour.Blue(), colour.Alpha() };
}

WXD_EXPORTED wxd_Font_t*
wxd_SystemSettings_GetFont(wxd_SystemFont index)
{
    wxFont font = wxSystemSettings::GetFont(map_system_font(index));
    if (!font.IsOk())
        return nullptr;
    return reinterpret_cast<wxd_Font_t*>(new wxFont(font));
}

WXD_EXPORTED int
wxd_SystemSettings_GetMetric(wxd_SystemMetric index, wxd_Window_t* win)
{
    const wxWindow* window = reinterpret_cast<const wxWindow*>(win);
    return wxSystemSettings::GetMetric(map_system_metric(index), window);
}

} // extern "C"
