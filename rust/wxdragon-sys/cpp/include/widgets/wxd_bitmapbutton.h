#ifndef WXD_BITMAPBUTTON_H
#define WXD_BITMAPBUTTON_H

#include "../wxd_types.h"

// --- BitmapButton Functions ---
WXD_EXPORTED wxd_BitmapButton_t*
wxd_BitmapButton_Create(wxd_Window_t* parent, wxd_Id id,
                        const wxd_Bitmap_t* bitmap, // Main bitmap (normal state)
                        wxd_Point pos, wxd_Size size, wxd_Style_t style, const char* name,
                        const wxd_Bitmap_t* bitmap_disabled, // Disabled state bitmap (can be NULL)
                        const wxd_Bitmap_t* bitmap_focus,    // Focus state bitmap (can be NULL)
                        const wxd_Bitmap_t* bitmap_hover     // Hover state bitmap (can be NULL)
);

// --- Setters for individual bitmaps after creation ---
WXD_EXPORTED void
wxd_BitmapButton_SetBitmapLabel(wxd_BitmapButton_t* self, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED void
wxd_BitmapButton_SetBitmapDisabled(wxd_BitmapButton_t* self, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED void
wxd_BitmapButton_SetBitmapFocus(wxd_BitmapButton_t* self, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED void
wxd_BitmapButton_SetBitmapHover(wxd_BitmapButton_t* self, const wxd_Bitmap_t* bitmap);

// --- Getters for individual bitmaps ---
WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapButton_GetBitmapLabel(wxd_BitmapButton_t* self);

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapButton_GetBitmapDisabled(wxd_BitmapButton_t* self);

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapButton_GetBitmapFocus(wxd_BitmapButton_t* self);

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapButton_GetBitmapHover(wxd_BitmapButton_t* self);

#endif // WXD_BITMAPBUTTON_H