#ifndef WXD_BITMAPCOMBOBOX_H
#define WXD_BITMAPCOMBOBOX_H

#include "../wxd_types.h"

// --- BitmapComboBox Functions ---
WXD_EXPORTED wxd_BitmapComboBox_t*
wxd_BitmapComboBox_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos,
                          wxd_Size size, wxd_Style_t style);

WXD_EXPORTED void
wxd_BitmapComboBox_Append(wxd_BitmapComboBox_t* self, const char* item, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED void
wxd_BitmapComboBox_Clear(wxd_BitmapComboBox_t* self);

WXD_EXPORTED int
wxd_BitmapComboBox_GetSelection(wxd_BitmapComboBox_t* self);

WXD_EXPORTED void
wxd_BitmapComboBox_SetSelection(wxd_BitmapComboBox_t* self, int index);

WXD_EXPORTED int
wxd_BitmapComboBox_GetString(wxd_BitmapComboBox_t* self, int index, char* buffer,
                             size_t buffer_len);

WXD_EXPORTED unsigned int
wxd_BitmapComboBox_GetCount(wxd_BitmapComboBox_t* self);

WXD_EXPORTED void
wxd_BitmapComboBox_SetValue(wxd_BitmapComboBox_t* self, const char* value);

WXD_EXPORTED int
wxd_BitmapComboBox_GetValue(wxd_BitmapComboBox_t* self, char* buffer, size_t buffer_len);

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapComboBox_GetItemBitmap(wxd_BitmapComboBox_t* self, unsigned int n);

WXD_EXPORTED void
wxd_BitmapComboBox_SetItemBitmap(wxd_BitmapComboBox_t* self, unsigned int n,
                                 const wxd_Bitmap_t* bitmap);

#endif // WXD_BITMAPCOMBOBOX_H