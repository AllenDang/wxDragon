#ifndef WXD_ARTPROVIDER_H
#define WXD_ARTPROVIDER_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// --- ArtProvider Functions ---
// Getting bitmaps and bundles
WXD_EXPORTED wxd_Bitmap_t*
wxd_ArtProvider_GetBitmap(const char* id, const char* client, wxd_Size size);

WXD_EXPORTED wxd_BitmapBundle_t*
wxd_ArtProvider_GetBitmapBundle(const char* id, const char* client, wxd_Size size);

// Size hint functions
WXD_EXPORTED wxd_Size
wxd_ArtProvider_GetDIPSizeHint(const char* client);

WXD_EXPORTED wxd_Size
wxd_ArtProvider_GetSizeHint(const char* client, wxd_Window_t* window);

WXD_EXPORTED wxd_Size
wxd_ArtProvider_GetNativeDIPSizeHint(const char* client);

WXD_EXPORTED wxd_Size
wxd_ArtProvider_GetNativeSizeHint(const char* client, wxd_Window_t* window);

// Other utility functions
WXD_EXPORTED bool
wxd_ArtProvider_HasNativeProvider(void);

#ifdef __cplusplus
}
#endif

#endif // WXD_ARTPROVIDER_H