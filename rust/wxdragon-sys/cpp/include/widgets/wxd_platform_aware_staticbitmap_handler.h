#ifndef WXD_PLATFORM_AWARE_STATICBITMAP_HANDLER_H
#define WXD_PLATFORM_AWARE_STATICBITMAP_HANDLER_H

#include "../wxd_types.h"

// Function to register the platform-aware StaticBitmap XRC handler
// Only available when XRC feature is enabled
WXD_EXPORTED void
wxd_XmlResource_InitPlatformAwareStaticBitmapHandler(wxd_XmlResource_t* resource);

#endif // WXD_PLATFORM_AWARE_STATICBITMAP_HANDLER_H