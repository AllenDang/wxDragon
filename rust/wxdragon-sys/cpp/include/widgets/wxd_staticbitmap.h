#ifndef WXD_STATICBITMAP_H
#define WXD_STATICBITMAP_H

#include "../wxd_types.h"

// --- StaticBitmap Functions ---
WXD_EXPORTED wxd_StaticBitmap_t*
wxd_StaticBitmap_CreateWithBitmap(wxd_Window_t* parent, wxd_Id id, const wxd_Bitmap_t* bitmap,
                                  wxd_Point pos, wxd_Size size, wxd_Style_t style,
                                  const char* name);

WXD_EXPORTED void
wxd_StaticBitmap_SetBitmap(wxd_StaticBitmap_t* self, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED wxd_Bitmap_t*
wxd_StaticBitmap_GetBitmap(wxd_StaticBitmap_t* self);

// BitmapBundle support
WXD_EXPORTED wxd_StaticBitmap_t*
wxd_StaticBitmap_CreateWithBitmapBundle(wxd_Window_t* parent, wxd_Id id,
                                        wxd_BitmapBundle_t* bundle);

WXD_EXPORTED void
wxd_StaticBitmap_SetBitmapBundle(wxd_StaticBitmap_t* staticBitmap, wxd_BitmapBundle_t* bundle);

// Scale Mode functions
WXD_EXPORTED void
wxd_StaticBitmap_SetScaleMode(wxd_StaticBitmap_t* self, int scaleMode);

WXD_EXPORTED int
wxd_StaticBitmap_GetScaleMode(wxd_StaticBitmap_t* self);

#endif // WXD_STATICBITMAP_H