#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/bmpbndl.h>
#include <wx/bitmap.h>
#include <wx/mstream.h>
#include <vector>

// Create an empty bitmap bundle
WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_Create()
{
    wxBitmapBundle* bundle = new wxBitmapBundle();
    return reinterpret_cast<wxd_BitmapBundle_t*>(bundle);
}

// Create a bitmap bundle from a single bitmap
WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_CreateFromBitmap(const wxd_Bitmap_t* bitmap)
{
    if (!bitmap) {
        return wxd_BitmapBundle_Create();
    }

    const wxBitmap* wx_bitmap = reinterpret_cast<const wxBitmap*>(bitmap);
    wxBitmapBundle* bundle = new wxBitmapBundle(*wx_bitmap);
    return reinterpret_cast<wxd_BitmapBundle_t*>(bundle);
}

// Create a bitmap bundle from multiple bitmaps of different sizes
WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_FromBitmaps(const wxd_Bitmap_t* const* bitmaps, size_t count)
{
    if (!bitmaps || count == 0) {
        return wxd_BitmapBundle_Create();
    }

    wxVector<wxBitmap> wx_bitmaps;
    wx_bitmaps.reserve(count);

    for (size_t i = 0; i < count; ++i) {
        if (bitmaps[i]) {
            const wxBitmap* wx_bitmap = reinterpret_cast<const wxBitmap*>(bitmaps[i]);
            wx_bitmaps.push_back(*wx_bitmap);
        }
    }

    if (wx_bitmaps.empty()) {
        return wxd_BitmapBundle_Create();
    }

    wxBitmapBundle* bundle = new wxBitmapBundle(wxBitmapBundle::FromBitmaps(wx_bitmaps));
    return reinterpret_cast<wxd_BitmapBundle_t*>(bundle);
}

// Clone a bitmap bundle
WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_Clone(const wxd_BitmapBundle_t* bundle)
{
    if (!bundle) {
        return wxd_BitmapBundle_Create();
    }

    const wxBitmapBundle* wx_bundle = reinterpret_cast<const wxBitmapBundle*>(bundle);
    wxBitmapBundle* new_bundle = new wxBitmapBundle(*wx_bundle);
    return reinterpret_cast<wxd_BitmapBundle_t*>(new_bundle);
}

// Destroy a bitmap bundle
WXD_EXPORTED void
wxd_BitmapBundle_Destroy(wxd_BitmapBundle_t* bundle)
{
    if (bundle) {
        wxBitmapBundle* wx_bundle = reinterpret_cast<wxBitmapBundle*>(bundle);
        delete wx_bundle;
    }
}

// Create a bitmap bundle from an SVG file
WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_FromSVGFile(const char* path, wxd_Size size)
{
    if (!path) {
        return wxd_BitmapBundle_Create();
    }

    wxString wx_path = wxString::FromUTF8(path);
    wxSize wx_size(size.width, size.height);
    wxBitmapBundle* bundle = new wxBitmapBundle(wxBitmapBundle::FromSVGFile(wx_path, wx_size));
    return reinterpret_cast<wxd_BitmapBundle_t*>(bundle);
}

// Create a bitmap bundle from SVG text
WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_FromSVGText(const char* svg_text, wxd_Size size)
{
    if (!svg_text) {
        return wxd_BitmapBundle_Create();
    }

    wxString wx_svg = wxString::FromUTF8(svg_text);
    wxSize wx_size(size.width, size.height);
    wxBitmapBundle* bundle = new wxBitmapBundle(wxBitmapBundle::FromSVG(wx_svg, wx_size));
    return reinterpret_cast<wxd_BitmapBundle_t*>(bundle);
}

// Create a bitmap bundle from SVG data
WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_FromSVGData(const unsigned char* data, size_t len, wxd_Size size)
{
    if (!data || len == 0) {
        return wxd_BitmapBundle_Create();
    }

    wxSize wx_size(size.width, size.height);
    wxBitmapBundle* bundle = new wxBitmapBundle(wxBitmapBundle::FromSVG(data, len, wx_size));
    return reinterpret_cast<wxd_BitmapBundle_t*>(bundle);
}

// Get a bitmap of the specified size from the bundle
WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapBundle_GetBitmap(const wxd_BitmapBundle_t* bundle, wxd_Size size)
{
    if (!bundle) {
        return nullptr;
    }

    const wxBitmapBundle* wx_bundle = reinterpret_cast<const wxBitmapBundle*>(bundle);
    wxSize wx_size(size.width, size.height);
    wxBitmap bitmap = wx_bundle->GetBitmap(wx_size);

    if (!bitmap.IsOk()) {
        return nullptr;
    }

    wxBitmap* new_bitmap = new wxBitmap(bitmap);
    return reinterpret_cast<wxd_Bitmap_t*>(new_bitmap);
}

// Get a bitmap for a specific window from the bundle
WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapBundle_GetBitmapFor(const wxd_BitmapBundle_t* bundle, wxd_Window_t* window)
{
    if (!bundle || !window) {
        return nullptr;
    }

    const wxBitmapBundle* wx_bundle = reinterpret_cast<const wxBitmapBundle*>(bundle);
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wxBitmap bitmap = wx_bundle->GetBitmapFor(wx_window);

    if (!bitmap.IsOk()) {
        return nullptr;
    }

    wxBitmap* new_bitmap = new wxBitmap(bitmap);
    return reinterpret_cast<wxd_Bitmap_t*>(new_bitmap);
}

// Get the default size of the bitmap bundle
WXD_EXPORTED wxd_Size
wxd_BitmapBundle_GetDefaultSize(const wxd_BitmapBundle_t* bundle)
{
    wxd_Size size = { 0, 0 };

    if (bundle) {
        const wxBitmapBundle* wx_bundle = reinterpret_cast<const wxBitmapBundle*>(bundle);
        wxSize wx_size = wx_bundle->GetDefaultSize();
        size.width = wx_size.GetWidth();
        size.height = wx_size.GetHeight();
    }

    return size;
}

// Get the preferred bitmap size at a specific scale
WXD_EXPORTED wxd_Size
wxd_BitmapBundle_GetPreferredBitmapSizeAtScale(const wxd_BitmapBundle_t* bundle, double scale)
{
    wxd_Size size = { 0, 0 };

    if (bundle) {
        const wxBitmapBundle* wx_bundle = reinterpret_cast<const wxBitmapBundle*>(bundle);
        wxSize wx_size = wx_bundle->GetPreferredBitmapSizeAtScale(scale);
        size.width = wx_size.GetWidth();
        size.height = wx_size.GetHeight();
    }

    return size;
}

// Get the preferred bitmap size for a specific window
WXD_EXPORTED wxd_Size
wxd_BitmapBundle_GetPreferredBitmapSizeFor(const wxd_BitmapBundle_t* bundle, wxd_Window_t* window)
{
    wxd_Size size = { 0, 0 };

    if (bundle && window) {
        const wxBitmapBundle* wx_bundle = reinterpret_cast<const wxBitmapBundle*>(bundle);
        wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
        wxSize wx_size = wx_bundle->GetPreferredBitmapSizeFor(wx_window);
        size.width = wx_size.GetWidth();
        size.height = wx_size.GetHeight();
    }

    return size;
}

// Check if the bitmap bundle is valid
WXD_EXPORTED bool
wxd_BitmapBundle_IsOk(const wxd_BitmapBundle_t* bundle)
{
    if (!bundle) {
        return false;
    }

    const wxBitmapBundle* wx_bundle = reinterpret_cast<const wxBitmapBundle*>(bundle);
    return wx_bundle->IsOk();
}