#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/bmpbuttn.h>
#include <wx/bitmap.h>
#include <cstdio> // For printf

// Implementation for wxd_BitmapButton_Create
WXD_EXPORTED wxd_BitmapButton_t*
wxd_BitmapButton_Create(
    wxd_Window_t* parent, wxd_Id id,
    const wxd_Bitmap_t* bitmap, // Main bitmap (normal state)
    wxd_Point pos, wxd_Size size, wxd_Style_t style, const char* name_str,
    const wxd_Bitmap_t* bitmap_disabled_wxd, // Disabled state bitmap (can be NULL)
    const wxd_Bitmap_t* bitmap_focus_wxd,    // Focus state bitmap (can be NULL)
    const wxd_Bitmap_t* bitmap_hover_wxd     // Hover state bitmap (can be NULL)
)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    const wxBitmap* bmp_main = reinterpret_cast<const wxBitmap*>(bitmap);

    if (!parentWin) {
        return nullptr;
    }
    // Main bitmap validity is handled by wxBitmapButton constructor if bmp_main is null or not Ok

    wxBitmapButton* btn = nullptr;
    try {
        btn = new wxBitmapButton(parentWin, id,
                                 bmp_main ? *bmp_main : wxNullBitmap, // Main bitmap
                                 wxd_cpp_utils::to_wx(pos), wxd_cpp_utils::to_wx(size), style,
                                 wxDefaultValidator, WXD_STR_TO_WX_STRING_UTF8_NULL_OK(name_str));
    }
    catch (const std::exception& e) {
        WXD_LOG_ERRORF("Exception creating wxBitmapButton: %s", e.what());
        return nullptr;
    }
    catch (...) {
        WXD_LOG_ERROR("Unknown exception creating wxBitmapButton");
        return nullptr;
    }

    if (!btn) {
        WXD_LOG_ERROR("wxBitmapButton creation returned null pointer unexpectedly.");
        return nullptr;
    }

    // Set other state bitmaps if provided
    if (bitmap_disabled_wxd) {
        const wxBitmap* bmp_disabled = reinterpret_cast<const wxBitmap*>(bitmap_disabled_wxd);
        if (bmp_disabled && bmp_disabled->IsOk()) {
            btn->SetBitmapDisabled(*bmp_disabled);
        }
    }
    if (bitmap_focus_wxd) {
        const wxBitmap* bmp_focus = reinterpret_cast<const wxBitmap*>(bitmap_focus_wxd);
        if (bmp_focus && bmp_focus->IsOk()) {
            btn->SetBitmapFocus(*bmp_focus);
        }
    }
    if (bitmap_hover_wxd) {
        const wxBitmap* bmp_hover = reinterpret_cast<const wxBitmap*>(bitmap_hover_wxd);
        if (bmp_hover && bmp_hover->IsOk()) {
            btn->SetBitmapHover(*bmp_hover);
        }
    }

    return reinterpret_cast<wxd_BitmapButton_t*>(btn);
}