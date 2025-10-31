#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/button.h>
#include <wx/bitmap.h> // For wxBitmap

// --- Button Functions Implementation ---

extern "C" {

wxd_Button_t*
wxd_Button_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size,
                  wxd_Style_t style)
{
    wxWindow* wx_parent = reinterpret_cast<wxWindow*>(parent);

    wxButton* wx_button = new wxButton(wx_parent, id, wxString::FromUTF8(label ? label : ""),
                                       wxd_cpp_utils::to_wx(pos), wxd_cpp_utils::to_wx(size),
                                       style);

    return reinterpret_cast<wxd_Button_t*>(wx_button);
}

void
wxd_Button_Destroy(wxd_Button_t* button)
{
    if (!button)
        return;
    wxButton* wx_button = reinterpret_cast<wxButton*>(button);
    // Schedule for destruction. Cleanup happens via wxEVT_DESTROY.
    wx_button->Destroy();
}

void
wxd_Button_SetLabel(wxd_Button_t* button, const char* label)
{
    if (!button)
        return;
    wxButton* wx_button = reinterpret_cast<wxButton*>(button);
    wx_button->SetLabel(wxString::FromUTF8(label ? label : ""));
}

int
wxd_Button_GetLabel(wxd_Button_t* button, char* buffer, size_t buffer_len)
{
    if (!button)
        return -1;
    wxButton* wx_button = reinterpret_cast<wxButton*>(button);
    wxString label_str = wx_button->GetLabel();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(label_str, buffer, buffer_len);
}

// --- Bitmap related functions for wxButton ---

static int
map_to_wx_direction(wxd_ButtonBitmapPosition_t dir)
{
    switch (dir) {
    case WXD_BUTTON_BITMAP_LEFT:
        return wxLEFT;
    case WXD_BUTTON_BITMAP_RIGHT:
        return wxRIGHT;
    case WXD_BUTTON_BITMAP_TOP:
        return wxTOP;
    case WXD_BUTTON_BITMAP_BOTTOM:
        return wxBOTTOM;
    default:
        return wxLEFT; // Default to left if unspecified
    }
}

void
wxd_Button_SetBitmap(wxd_Button_t* self, const wxd_Bitmap_t* bitmap, wxd_ButtonBitmapPosition_t dir)
{
    if (!self)
        return;
    wxButton* btn = reinterpret_cast<wxButton*>(self);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    // wxBitmapBundle can be created from a single bitmap
    btn->SetBitmap(bmp ? wxBitmapBundle(*bmp) : wxBitmapBundle(),
                   static_cast<wxDirection>(map_to_wx_direction(dir)));
}

void
wxd_Button_SetBitmapDisabled(wxd_Button_t* self, const wxd_Bitmap_t* bitmap)
{
    if (!self)
        return;
    wxButton* btn = reinterpret_cast<wxButton*>(self);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    btn->SetBitmapDisabled(bmp ? wxBitmapBundle(*bmp) : wxBitmapBundle());
}

void
wxd_Button_SetBitmapFocus(wxd_Button_t* self, const wxd_Bitmap_t* bitmap)
{
    if (!self)
        return;
    wxButton* btn = reinterpret_cast<wxButton*>(self);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    btn->SetBitmapFocus(bmp ? wxBitmapBundle(*bmp) : wxBitmapBundle());
}

void
wxd_Button_SetBitmapCurrent(wxd_Button_t* self, const wxd_Bitmap_t* bitmap)
{
    if (!self)
        return;
    wxButton* btn = reinterpret_cast<wxButton*>(self);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    btn->SetBitmapCurrent(bmp ? wxBitmapBundle(*bmp) : wxBitmapBundle());
}

void
wxd_Button_SetBitmapPressed(wxd_Button_t* self, const wxd_Bitmap_t* bitmap)
{
    if (!self)
        return;
    wxButton* btn = reinterpret_cast<wxButton*>(self);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    btn->SetBitmapPressed(bmp ? wxBitmapBundle(*bmp) : wxBitmapBundle());
}

// Getters for wxButton bitmaps are a bit tricky as wxButton stores wxBitmapBundle.
// For simplicity, we might return the first bitmap from the bundle if it's not empty,
// or decide if this level of detail is needed for the FFI getters.
// wxWidgets documentation indicates GetBitmap() returns a const wxBitmapBundle&.
// To return a wxd_Bitmap_t*, we'd need to extract a specific bitmap or manage a copy.
// For now, returning nullptr for getters to indicate they are not straightforwardly implemented.
// A more complete solution might involve returning a specific state's bitmap if possible or managing ownership.

wxd_Bitmap_t*
wxd_Button_GetBitmap(wxd_Button_t* self)
{
    if (!self)
        return nullptr;
    wxButton* btn = reinterpret_cast<wxButton*>(self);
    const wxBitmapBundle& bundle = btn->GetBitmap();
    if (bundle.IsOk() &&
        !bundle.GetBitmap(wxDefaultSize).IsOk()) { // Check if bundle has any bitmap
        // This is imperfect. wxBitmapBundle can hold multiple sizes.
        // We are trying to get a raw wxd_Bitmap_t*, which assumes a single wxBitmap.
        // This might require creating a new wxBitmap from the bundle or managing a reference.
        // For now, returning null to avoid complex ownership or selection logic.
        // Consider if the Rust side actually needs to get these as owned Bitmap objects.
    }
    return nullptr; // Placeholder
}

wxd_Bitmap_t*
wxd_Button_GetBitmapDisabled(wxd_Button_t* self)
{
    if (!self)
        return nullptr;
    // Similar logic to GetBitmap
    return nullptr; // Placeholder
}

wxd_Bitmap_t*
wxd_Button_GetBitmapFocus(wxd_Button_t* self)
{
    if (!self)
        return nullptr;
    return nullptr; // Placeholder
}

wxd_Bitmap_t*
wxd_Button_GetBitmapCurrent(wxd_Button_t* self)
{
    if (!self)
        return nullptr;
    return nullptr; // Placeholder
}

wxd_Bitmap_t*
wxd_Button_GetBitmapPressed(wxd_Button_t* self)
{
    if (!self)
        return nullptr;
    return nullptr; // Placeholder
}

} // extern "C"

// BitmapBundle support implementations
WXD_EXPORTED void
wxd_Button_SetBitmapBundle(wxd_Button_t* button, wxd_BitmapBundle_t* bundle, wxd_Direction_t dir)
{
    if (!button)
        return;
    wxButton* btn = reinterpret_cast<wxButton*>(button);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bundle);
    btn->SetBitmap(bundle ? *bundlePtr : wxBitmapBundle(), static_cast<wxDirection>(dir));
}

WXD_EXPORTED void
wxd_Button_SetBitmapBundleDisabled(wxd_Button_t* button, wxd_BitmapBundle_t* bundle)
{
    if (!button)
        return;
    wxButton* btn = reinterpret_cast<wxButton*>(button);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bundle);
    btn->SetBitmapDisabled(bundle ? *bundlePtr : wxBitmapBundle());
}

WXD_EXPORTED void
wxd_Button_SetBitmapBundleFocus(wxd_Button_t* button, wxd_BitmapBundle_t* bundle)
{
    if (!button)
        return;
    wxButton* btn = reinterpret_cast<wxButton*>(button);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bundle);
    btn->SetBitmapFocus(bundle ? *bundlePtr : wxBitmapBundle());
}

WXD_EXPORTED void
wxd_Button_SetBitmapBundlePressed(wxd_Button_t* button, wxd_BitmapBundle_t* bundle)
{
    if (!button)
        return;
    wxButton* btn = reinterpret_cast<wxButton*>(button);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bundle);
    btn->SetBitmapPressed(bundle ? *bundlePtr : wxBitmapBundle());
}

WXD_EXPORTED void
wxd_Button_SetBitmapBundleHover(wxd_Button_t* button, wxd_BitmapBundle_t* bundle)
{
    if (!button)
        return;
    wxButton* btn = reinterpret_cast<wxButton*>(button);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bundle);
    btn->SetBitmapCurrent(bundle ? *bundlePtr : wxBitmapBundle());
}
