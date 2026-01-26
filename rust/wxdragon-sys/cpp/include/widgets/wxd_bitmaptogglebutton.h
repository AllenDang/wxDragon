#ifndef WXD_BITMAPTOGGLEBUTTON_H
#define WXD_BITMAPTOGGLEBUTTON_H

#include "../wxd_types.h"

// --- BitmapToggleButton Functions ---

/// Creates a new wxBitmapToggleButton.
/// @param parent The parent window (required).
/// @param id The window identifier.
/// @param bitmap The main bitmap to display (can be NULL for empty bitmap).
/// @param pos The button position.
/// @param size The button size.
/// @param style The button style flags.
/// @param name The window name for lookup.
/// @param bitmap_disabled The bitmap for disabled state (can be NULL).
/// @param bitmap_focus The bitmap for focused state (can be NULL).
/// @param bitmap_pressed The bitmap for pressed/toggled state (can be NULL).
/// @return A pointer to the new button, or NULL on failure.
WXD_EXPORTED wxd_BitmapToggleButton_t*
wxd_BitmapToggleButton_Create(wxd_Window_t* parent, wxd_Id id,
                               const wxd_Bitmap_t* bitmap,
                               wxd_Point pos, wxd_Size size, wxd_Style_t style,
                               const char* name,
                               const wxd_Bitmap_t* bitmap_disabled,
                               const wxd_Bitmap_t* bitmap_focus,
                               const wxd_Bitmap_t* bitmap_pressed);

/// Gets the current toggle state of the button.
/// @param btn The button to query.
/// @return true if the button is pressed/toggled, false otherwise.
WXD_EXPORTED bool
wxd_BitmapToggleButton_GetValue(wxd_BitmapToggleButton_t* btn);

/// Sets the toggle state of the button.
/// @param btn The button to modify.
/// @param state The new toggle state.
WXD_EXPORTED void
wxd_BitmapToggleButton_SetValue(wxd_BitmapToggleButton_t* btn, bool state);

// --- Setters for individual bitmaps after creation ---

WXD_EXPORTED void
wxd_BitmapToggleButton_SetBitmapLabel(wxd_BitmapToggleButton_t* self, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED void
wxd_BitmapToggleButton_SetBitmapDisabled(wxd_BitmapToggleButton_t* self, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED void
wxd_BitmapToggleButton_SetBitmapFocus(wxd_BitmapToggleButton_t* self, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED void
wxd_BitmapToggleButton_SetBitmapPressed(wxd_BitmapToggleButton_t* self, const wxd_Bitmap_t* bitmap);

// --- Getters for individual bitmaps ---

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapToggleButton_GetBitmapLabel(wxd_BitmapToggleButton_t* self);

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapToggleButton_GetBitmapDisabled(wxd_BitmapToggleButton_t* self);

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapToggleButton_GetBitmapFocus(wxd_BitmapToggleButton_t* self);

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapToggleButton_GetBitmapPressed(wxd_BitmapToggleButton_t* self);

#endif // WXD_BITMAPTOGGLEBUTTON_H
