#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/tglbtn.h> // For wxBitmapToggleButton
#include <wx/bitmap.h>

// Helper functions for point/size conversion
inline wxPoint
wxd_to_wx_point(const wxd_Point& p)
{
    if (p.x == -1 && p.y == -1)
        return wxDefaultPosition;
    return wxPoint(p.x, p.y);
}

inline wxSize
wxd_to_wx_size(const wxd_Size& s)
{
    if (s.width == -1 && s.height == -1)
        return wxDefaultSize;
    return wxSize(s.width, s.height);
}

extern "C" {

WXD_EXPORTED wxd_BitmapToggleButton_t*
wxd_BitmapToggleButton_Create(wxd_Window_t* parent, wxd_Id id,
                               const wxd_Bitmap_t* bitmap,
                               wxd_Point pos, wxd_Size size, wxd_Style_t style,
                               const char* name_str,
                               const wxd_Bitmap_t* bitmap_disabled_wxd,
                               const wxd_Bitmap_t* bitmap_focus_wxd,
                               const wxd_Bitmap_t* bitmap_pressed_wxd)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    const wxBitmap* bmp_main = reinterpret_cast<const wxBitmap*>(bitmap);

    if (!parentWin) {
        return nullptr;
    }

    wxBitmapToggleButton* btn = nullptr;
    try {
        btn = new wxBitmapToggleButton(parentWin, id,
                                       bmp_main ? *bmp_main : wxNullBitmap,
                                       wxd_to_wx_point(pos), wxd_to_wx_size(size), style,
                                       wxDefaultValidator,
                                       WXD_STR_TO_WX_STRING_UTF8_NULL_OK(name_str));
    }
    catch (const std::exception& e) {
        WXD_LOG_ERRORF("Exception creating wxBitmapToggleButton: %s", e.what());
        return nullptr;
    }
    catch (...) {
        WXD_LOG_ERROR("Unknown exception creating wxBitmapToggleButton");
        return nullptr;
    }

    if (!btn) {
        WXD_LOG_ERROR("wxBitmapToggleButton creation returned null pointer unexpectedly.");
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
    if (bitmap_pressed_wxd) {
        const wxBitmap* bmp_pressed = reinterpret_cast<const wxBitmap*>(bitmap_pressed_wxd);
        if (bmp_pressed && bmp_pressed->IsOk()) {
            btn->SetBitmapPressed(*bmp_pressed);
        }
    }

    return reinterpret_cast<wxd_BitmapToggleButton_t*>(btn);
}

WXD_EXPORTED bool
wxd_BitmapToggleButton_GetValue(wxd_BitmapToggleButton_t* btn)
{
    wxBitmapToggleButton* wxBtn = reinterpret_cast<wxBitmapToggleButton*>(btn);
    if (wxBtn) {
        return wxBtn->GetValue();
    }
    return false;
}

WXD_EXPORTED void
wxd_BitmapToggleButton_SetValue(wxd_BitmapToggleButton_t* btn, bool state)
{
    wxBitmapToggleButton* wxBtn = reinterpret_cast<wxBitmapToggleButton*>(btn);
    if (wxBtn) {
        wxBtn->SetValue(state);
    }
}

// --- Setters for individual bitmaps after creation ---

WXD_EXPORTED void
wxd_BitmapToggleButton_SetBitmapLabel(wxd_BitmapToggleButton_t* self, const wxd_Bitmap_t* bitmap)
{
    if (!self)
        return;
    wxBitmapToggleButton* btn = reinterpret_cast<wxBitmapToggleButton*>(self);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    btn->SetBitmapLabel(bmp ? *bmp : wxNullBitmap);
}

WXD_EXPORTED void
wxd_BitmapToggleButton_SetBitmapDisabled(wxd_BitmapToggleButton_t* self, const wxd_Bitmap_t* bitmap)
{
    if (!self)
        return;
    wxBitmapToggleButton* btn = reinterpret_cast<wxBitmapToggleButton*>(self);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    btn->SetBitmapDisabled(bmp ? *bmp : wxNullBitmap);
}

WXD_EXPORTED void
wxd_BitmapToggleButton_SetBitmapFocus(wxd_BitmapToggleButton_t* self, const wxd_Bitmap_t* bitmap)
{
    if (!self)
        return;
    wxBitmapToggleButton* btn = reinterpret_cast<wxBitmapToggleButton*>(self);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    btn->SetBitmapFocus(bmp ? *bmp : wxNullBitmap);
}

WXD_EXPORTED void
wxd_BitmapToggleButton_SetBitmapPressed(wxd_BitmapToggleButton_t* self, const wxd_Bitmap_t* bitmap)
{
    if (!self)
        return;
    wxBitmapToggleButton* btn = reinterpret_cast<wxBitmapToggleButton*>(self);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    btn->SetBitmapPressed(bmp ? *bmp : wxNullBitmap);
}

// --- Getters for individual bitmaps ---
// Return heap-allocated copies - Rust must free via Bitmap::drop

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapToggleButton_GetBitmapLabel(wxd_BitmapToggleButton_t* self)
{
    if (!self)
        return nullptr;
    wxBitmapToggleButton* btn = reinterpret_cast<wxBitmapToggleButton*>(self);
    const wxBitmap& bmp = btn->GetBitmapLabel();
    if (!bmp.IsOk())
        return nullptr;
    return (wxd_Bitmap_t*)new wxBitmap(bmp);
}

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapToggleButton_GetBitmapDisabled(wxd_BitmapToggleButton_t* self)
{
    if (!self)
        return nullptr;
    wxBitmapToggleButton* btn = reinterpret_cast<wxBitmapToggleButton*>(self);
    const wxBitmap& bmp = btn->GetBitmapDisabled();
    if (!bmp.IsOk())
        return nullptr;
    return (wxd_Bitmap_t*)new wxBitmap(bmp);
}

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapToggleButton_GetBitmapFocus(wxd_BitmapToggleButton_t* self)
{
    if (!self)
        return nullptr;
    wxBitmapToggleButton* btn = reinterpret_cast<wxBitmapToggleButton*>(self);
    const wxBitmap& bmp = btn->GetBitmapFocus();
    if (!bmp.IsOk())
        return nullptr;
    return (wxd_Bitmap_t*)new wxBitmap(bmp);
}

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapToggleButton_GetBitmapPressed(wxd_BitmapToggleButton_t* self)
{
    if (!self)
        return nullptr;
    wxBitmapToggleButton* btn = reinterpret_cast<wxBitmapToggleButton*>(self);
    const wxBitmap& bmp = btn->GetBitmapPressed();
    if (!bmp.IsOk())
        return nullptr;
    return (wxd_Bitmap_t*)new wxBitmap(bmp);
}

} // extern "C"
