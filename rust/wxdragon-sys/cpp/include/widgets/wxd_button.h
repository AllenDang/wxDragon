#ifndef WXD_BUTTON_H
#define WXD_BUTTON_H

#include <stdint.h>
#include "../wxd_types.h"

// Enum for bitmap position on a button
typedef enum : int32_t {
    WXD_BUTTON_BITMAP_LEFT = 0,  // wxLEFT
    WXD_BUTTON_BITMAP_RIGHT = 1, // wxRIGHT
    WXD_BUTTON_BITMAP_TOP = 2,   // wxTOP
    WXD_BUTTON_BITMAP_BOTTOM = 3 // wxBOTTOM
} wxd_ButtonBitmapPosition_t;

// --- Button Functions ---
WXD_EXPORTED wxd_Button_t*
wxd_Button_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size,
                  wxd_Style_t style);

WXD_EXPORTED void
wxd_Button_Destroy(
    wxd_Button_t* button); // Note: Consider if needed, generic wxd_Window_Destroy might suffice

WXD_EXPORTED void
wxd_Button_SetLabel(wxd_Button_t* button, const char* label);

WXD_EXPORTED int
wxd_Button_GetLabel(wxd_Button_t* button, char* buffer, size_t buffer_len);

// --- Bitmap related functions for wxButton ---
WXD_EXPORTED void
wxd_Button_SetBitmap(wxd_Button_t* self, const wxd_Bitmap_t* bitmap,
                     wxd_ButtonBitmapPosition_t dir);

WXD_EXPORTED void
wxd_Button_SetBitmapDisabled(wxd_Button_t* self, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED void
wxd_Button_SetBitmapFocus(wxd_Button_t* self, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED void
wxd_Button_SetBitmapCurrent(wxd_Button_t* self, const wxd_Bitmap_t* bitmap); // For hover state

WXD_EXPORTED void
wxd_Button_SetBitmapPressed(wxd_Button_t* self, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED wxd_Bitmap_t*
wxd_Button_GetBitmap(wxd_Button_t* self);

WXD_EXPORTED wxd_Bitmap_t*
wxd_Button_GetBitmapDisabled(wxd_Button_t* self);

WXD_EXPORTED wxd_Bitmap_t*
wxd_Button_GetBitmapFocus(wxd_Button_t* self);

WXD_EXPORTED wxd_Bitmap_t*
wxd_Button_GetBitmapCurrent(wxd_Button_t* self);

WXD_EXPORTED wxd_Bitmap_t*
wxd_Button_GetBitmapPressed(wxd_Button_t* self);

// BitmapBundle support
WXD_EXPORTED void
wxd_Button_SetBitmapBundle(wxd_Button_t* button, wxd_BitmapBundle_t* bundle, wxd_Direction_t dir);

WXD_EXPORTED void
wxd_Button_SetBitmapBundleDisabled(wxd_Button_t* button, wxd_BitmapBundle_t* bundle);

WXD_EXPORTED void
wxd_Button_SetBitmapBundleFocus(wxd_Button_t* button, wxd_BitmapBundle_t* bundle);

WXD_EXPORTED void
wxd_Button_SetBitmapBundlePressed(wxd_Button_t* button, wxd_BitmapBundle_t* bundle);

WXD_EXPORTED void
wxd_Button_SetBitmapBundleHover(wxd_Button_t* button, wxd_BitmapBundle_t* bundle);

#endif // WXD_BUTTON_H