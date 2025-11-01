#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/image.h>  // For wxImage
#include <wx/bitmap.h> // For wxBitmap
#include <cstdlib>     // For malloc, free
#include <cstring>     // For memcpy

// Implementation for wxd_Bitmap_CreateFromRGBA
WXD_EXPORTED wxd_Bitmap_t*
wxd_Bitmap_CreateFromRGBA(const unsigned char* data, int width, int height)
{
    if (!data || width <= 0 || height <= 0) {
        return nullptr;
    }

    // wxImage::SetData requires a buffer allocated with malloc that it can free later.
    // We must copy the RGB data from the Rust-provided RGBA buffer into a new malloc'd buffer.
    size_t num_pixels = static_cast<size_t>(width) * static_cast<size_t>(height);
    size_t rgb_data_size = num_pixels * 3;
    unsigned char* rgb_data = static_cast<unsigned char*>(malloc(rgb_data_size));
    if (!rgb_data) {
        WXD_LOG_ERROR("Failed to allocate memory for bitmap RGB data.");
        return nullptr;
    }

    // Also need a separate buffer for Alpha data
    size_t alpha_data_size = num_pixels; // 1 byte per pixel
    unsigned char* alpha_data = static_cast<unsigned char*>(malloc(alpha_data_size));
    if (!alpha_data) {
        WXD_LOG_ERROR("Failed to allocate memory for bitmap Alpha data.");
        free(rgb_data); // Clean up already allocated buffer
        return nullptr;
    }

    // Copy data from input RGBA buffer to separate RGB and Alpha buffers
    for (size_t i = 0; i < num_pixels; ++i) {
        rgb_data[i * 3 + 0] = data[i * 4 + 0]; // R
        rgb_data[i * 3 + 1] = data[i * 4 + 1]; // G
        rgb_data[i * 3 + 2] = data[i * 4 + 2]; // B
        alpha_data[i] = data[i * 4 + 3];       // A
    }

    // Create wxImage. It takes ownership of rgb_data AND alpha_data.
    wxImage image(width, height, rgb_data, alpha_data); // Pass both buffers

    if (!image.IsOk()) {
        WXD_LOG_ERROR("Failed to create wxImage from RGBA data.");
        // If image creation failed, wxImage *should* have freed rgb_data and alpha_data, but double-check docs.
        // Assuming it did, we just return nullptr.
        // If it didn't free on failure (unlikely), we would need: free(rgb_data); free(alpha_data);
        return nullptr;
    }

    // Now create the wxBitmap from the wxImage.
    // Use depth -1 for default screen depth.
    // Use nothrow to avoid throwing across the FFI boundary.
    wxBitmap* bitmap = new (std::nothrow) wxBitmap(image, -1);

    if (!bitmap || !bitmap->IsOk()) {
        WXD_LOG_ERROR("Failed to create wxBitmap from wxImage.");
        delete bitmap; // Clean up partially created bitmap if possible
        return nullptr;
    }

    return reinterpret_cast<wxd_Bitmap_t*>(bitmap);
}

// Implementation for wxd_Bitmap_Destroy
WXD_EXPORTED void
wxd_Bitmap_Destroy(wxd_Bitmap_t* bitmap)
{
    wxBitmap* bmp = reinterpret_cast<wxBitmap*>(bitmap);
    if (!bmp)
        return;
    // Never delete the global wxNullBitmap.
    if (bmp == &wxNullBitmap)
        return;
    delete bmp;
}

// ADDED: Get bitmap dimensions and validity
WXD_EXPORTED int
wxd_Bitmap_GetWidth(const wxd_Bitmap_t* bitmap)
{
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    if (!bmp || !bmp->IsOk())
        return 0;
    return bmp->GetWidth();
}

WXD_EXPORTED int
wxd_Bitmap_GetHeight(const wxd_Bitmap_t* bitmap)
{
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    if (!bmp || !bmp->IsOk())
        return 0;
    return bmp->GetHeight();
}

WXD_EXPORTED bool
wxd_Bitmap_IsOk(const wxd_Bitmap_t* bitmap)
{
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    if (!bmp)
        return false;
    return bmp->IsOk();
}

WXD_EXPORTED wxd_Bitmap_t*
wxd_Bitmap_Clone(const wxd_Bitmap_t* bitmap)
{
    if (!bitmap) {
        return nullptr;
    }
    const wxBitmap* original_bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    if (!original_bmp->IsOk()) {
        return nullptr; // Don't clone an invalid bitmap
    }

    // Use nothrow to avoid throwing across the FFI boundary.
    wxBitmap* cloned_bmp = new (std::nothrow) wxBitmap(*original_bmp); // shallow copy (ref-counted)

    if (!cloned_bmp || !cloned_bmp->IsOk()) {
        delete cloned_bmp; // Safe to call delete on nullptr
        return nullptr;
    }
    return reinterpret_cast<wxd_Bitmap_t*>(cloned_bmp);
}

// Extract RGBA data from bitmap
WXD_EXPORTED unsigned char*
wxd_Bitmap_GetRGBAData(const wxd_Bitmap_t* bitmap, size_t* width, size_t* height)
{
    if (!bitmap) {
        return nullptr;
    }

    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);
    if (!bmp || !bmp->IsOk()) {
        return nullptr;
    }

    // Convert bitmap to image to access pixel data
    wxImage image = bmp->ConvertToImage();
    if (!image.IsOk()) {
        return nullptr;
    }

    int w = image.GetWidth();
    int h = image.GetHeight();
    size_t num_pixels = static_cast<size_t>(w) * static_cast<size_t>(h);
    size_t rgba_data_size = num_pixels * 4; // 4 bytes per pixel (RGBA)

    if (width) {
        *width = static_cast<size_t>(w);
    }

    if (height) {
        *height = static_cast<size_t>(h);
    }

    // Allocate memory for RGBA data
    unsigned char* rgba_data = static_cast<unsigned char*>(malloc(rgba_data_size));
    if (!rgba_data) {
        return nullptr;
    }

    // Get RGB and alpha data from image
    unsigned char* rgb_data = image.GetData();
    unsigned char* alpha_data = image.GetAlpha();

    // Combine RGB and alpha into RGBA format
    for (size_t i = 0; i < num_pixels; ++i) {
        rgba_data[i * 4 + 0] = rgb_data[i * 3 + 0]; // R
        rgba_data[i * 4 + 1] = rgb_data[i * 3 + 1]; // G
        rgba_data[i * 4 + 2] = rgb_data[i * 3 + 2]; // B

        // Set alpha channel (255 if no alpha data available)
        if (alpha_data) {
            rgba_data[i * 4 + 3] = alpha_data[i]; // A
        }
        else {
            rgba_data[i * 4 + 3] = 255; // Fully opaque
        }
    }

    return rgba_data;
}

// Free RGBA data allocated by wxd_Bitmap_GetRGBAData
WXD_EXPORTED void
wxd_Bitmap_FreeRGBAData(unsigned char* data)
{
    if (data) {
        free(data);
    }
}

// Get a pointer to wxNullBitmap
WXD_EXPORTED const wxd_Bitmap_t*
wxd_Bitmap_GetNull(void)
{
    return reinterpret_cast<const wxd_Bitmap_t*>(&wxNullBitmap);
}