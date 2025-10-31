#ifndef WXD_DATAOBJECT_H
#define WXD_DATAOBJECT_H

#include "../wxd_types.h"

// --- DataObject Functions ---
WXD_EXPORTED void
wxd_DataObject_Destroy(wxd_DataObject_t* data_object);

// --- TextDataObject Functions ---
WXD_EXPORTED wxd_TextDataObject_t*
wxd_TextDataObject_Create(const char* text);

/**
 * Gets the text from the data object.
 * Returns the length of the text retrieved (not including the null terminator).
 * If buffer is not null and buffer_len is non-zero, copies up to buffer_len-1 characters into buffer,
 * null-terminating it.
 */
WXD_EXPORTED int
wxd_TextDataObject_GetText(const wxd_TextDataObject_t* data_object, char* buffer,
                           size_t buffer_len);

WXD_EXPORTED void
wxd_TextDataObject_SetText(wxd_TextDataObject_t* data_object, const char* text);

// --- FileDataObject Functions ---
WXD_EXPORTED wxd_FileDataObject_t*
wxd_FileDataObject_Create();
WXD_EXPORTED void
wxd_FileDataObject_Destroy(wxd_FileDataObject_t* obj);
WXD_EXPORTED void
wxd_FileDataObject_AddFile(wxd_FileDataObject_t* data_object, const char* file);
WXD_EXPORTED int
wxd_FileDataObject_GetFileCount(wxd_FileDataObject_t* data_object);

WXD_EXPORTED int
wxd_FileDataObject_GetFile(wxd_FileDataObject_t* data_object, int index, char* buffer,
                           size_t buffer_len);
WXD_EXPORTED int
wxd_FileDataObject_GetFilenames(wxd_FileDataObject_t* obj, wxd_ArrayString_t* filenames);

// --- BitmapDataObject Functions ---
WXD_EXPORTED wxd_BitmapDataObject_t*
wxd_BitmapDataObject_Create(const wxd_Bitmap_t* bitmap);

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapDataObject_GetBitmap(wxd_BitmapDataObject_t* data_object);

#endif // WXD_DATAOBJECT_H