#ifndef WXD_BITMAPBUNDLE_H
#define WXD_BITMAPBUNDLE_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Opaque type for wxBitmapBundle
typedef struct wxd_BitmapBundle_t wxd_BitmapBundle_t;

// Construction functions
WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_Create();

WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_CreateFromBitmap(const wxd_Bitmap_t* bitmap);

WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_Clone(const wxd_BitmapBundle_t* bundle);

WXD_EXPORTED void
wxd_BitmapBundle_Destroy(wxd_BitmapBundle_t* bundle);

// Create from multiple bitmaps
WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_FromBitmaps(const wxd_Bitmap_t* const* bitmaps, size_t count);

// SVG-related functions
WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_FromSVGFile(const char* path, wxd_Size size);

WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_FromSVGText(const char* svg_text, wxd_Size size);

WXD_EXPORTED wxd_BitmapBundle_t*
wxd_BitmapBundle_FromSVGData(const unsigned char* data, size_t len, wxd_Size size);

// Bitmap retrieval functions
WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapBundle_GetBitmap(const wxd_BitmapBundle_t* bundle, wxd_Size size);

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapBundle_GetBitmapFor(const wxd_BitmapBundle_t* bundle, wxd_Window_t* window);

// Information functions
WXD_EXPORTED wxd_Size
wxd_BitmapBundle_GetDefaultSize(const wxd_BitmapBundle_t* bundle);

WXD_EXPORTED wxd_Size
wxd_BitmapBundle_GetPreferredBitmapSizeAtScale(const wxd_BitmapBundle_t* bundle, double scale);

WXD_EXPORTED wxd_Size
wxd_BitmapBundle_GetPreferredBitmapSizeFor(const wxd_BitmapBundle_t* bundle, wxd_Window_t* window);

WXD_EXPORTED bool
wxd_BitmapBundle_IsOk(const wxd_BitmapBundle_t* bundle);

#ifdef __cplusplus
}
#endif

#endif // WXD_BITMAPBUNDLE_H