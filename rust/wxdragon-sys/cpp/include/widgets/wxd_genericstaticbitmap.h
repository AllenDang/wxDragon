#ifndef WXD_GENERICSTATICBITMAP_H
#define WXD_GENERICSTATICBITMAP_H

#include "../wxd_types.h"

// --- GenericStaticBitmap Functions ---
WXD_EXPORTED wxd_GenericStaticBitmap_t*
wxd_GenericStaticBitmap_CreateWithBitmap(wxd_Window_t* parent, wxd_Id id,
                                         const wxd_Bitmap_t* bitmap, wxd_Point pos, wxd_Size size,
                                         wxd_Style_t style, const char* name);

WXD_EXPORTED void
wxd_GenericStaticBitmap_SetBitmap(wxd_GenericStaticBitmap_t* self, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED wxd_Bitmap_t*
wxd_GenericStaticBitmap_GetBitmap(wxd_GenericStaticBitmap_t* self);

// BitmapBundle support
WXD_EXPORTED wxd_GenericStaticBitmap_t*
wxd_GenericStaticBitmap_CreateWithBitmapBundle(wxd_Window_t* parent, wxd_Id id,
                                               wxd_BitmapBundle_t* bundle);

WXD_EXPORTED void
wxd_GenericStaticBitmap_SetBitmapBundle(wxd_GenericStaticBitmap_t* genericStaticBitmap,
                                        wxd_BitmapBundle_t* bundle);

// Scale Mode functions
WXD_EXPORTED void
wxd_GenericStaticBitmap_SetScaleMode(wxd_GenericStaticBitmap_t* self, int scaleMode);

WXD_EXPORTED int
wxd_GenericStaticBitmap_GetScaleMode(wxd_GenericStaticBitmap_t* self);

#endif // WXD_GENERICSTATICBITMAP_H