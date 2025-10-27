#ifndef WXD_BITMAP_H
#define WXD_BITMAP_H

#include "../wxd_types.h"

// --- Bitmap Functions ---
WXD_EXPORTED wxd_Bitmap_t*
wxd_Bitmap_CreateFromRGBA(const unsigned char* data, int width, int height);

WXD_EXPORTED void
wxd_Bitmap_Destroy(wxd_Bitmap_t* bitmap);

WXD_EXPORTED int
wxd_Bitmap_GetWidth(wxd_Bitmap_t* bitmap);

WXD_EXPORTED int
wxd_Bitmap_GetHeight(wxd_Bitmap_t* bitmap);

WXD_EXPORTED bool
wxd_Bitmap_IsOk(wxd_Bitmap_t* bitmap);

WXD_EXPORTED wxd_Bitmap_t*
wxd_Bitmap_Clone(wxd_Bitmap_t* bitmap);

// Extract RGBA data from bitmap
WXD_EXPORTED unsigned char*
wxd_Bitmap_GetRGBAData(wxd_Bitmap_t* bitmap);

WXD_EXPORTED void
wxd_Bitmap_FreeRGBAData(unsigned char* data);

// Get a pointer to wxNullBitmap
WXD_EXPORTED const wxd_Bitmap_t*
wxd_Bitmap_GetNull(void);

#endif // WXD_BITMAP_H