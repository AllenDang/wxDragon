#ifndef WXD_CURSOR_H
#define WXD_CURSOR_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// --- Cursor Creation Functions ---

/// Creates a cursor from a stock cursor type
WXD_EXPORTED wxd_Cursor_t*
wxd_Cursor_CreateStock(wxd_StockCursor cursor_id);

/// Creates a cursor from a file
WXD_EXPORTED wxd_Cursor_t*
wxd_Cursor_CreateFromFile(const char* filename, wxd_BitmapType type, int hotspot_x, int hotspot_y);

/// Creates a cursor from bitmap data
WXD_EXPORTED wxd_Cursor_t*
wxd_Cursor_CreateFromData(const unsigned char* bits, int width, int height, int hotspot_x,
                          int hotspot_y, const unsigned char* mask_bits);

/// Creates a cursor from an image
WXD_EXPORTED wxd_Cursor_t*
wxd_Cursor_CreateFromImage(const wxd_Bitmap_t* image);

/// Creates a copy of a cursor
WXD_EXPORTED wxd_Cursor_t*
wxd_Cursor_Copy(wxd_Cursor_t* cursor);

// --- Cursor Destruction ---

/// Destroys a cursor and frees its memory
WXD_EXPORTED void
wxd_Cursor_Destroy(wxd_Cursor_t* cursor);

// --- Cursor Properties ---

/// Returns true if the cursor is valid
WXD_EXPORTED bool
wxd_Cursor_IsOk(wxd_Cursor_t* cursor);

/// Gets the hotspot coordinates of the cursor
WXD_EXPORTED wxd_Point
wxd_Cursor_GetHotSpot(wxd_Cursor_t* cursor);

/// Sets the hotspot coordinates of the cursor (if supported)
WXD_EXPORTED void
wxd_Cursor_SetHotSpot(wxd_Cursor_t* cursor, int x, int y);

// --- Platform-specific Functions ---

/// Gets the native handle of the cursor (platform-specific)
WXD_EXPORTED void*
wxd_Cursor_GetHandle(wxd_Cursor_t* cursor);

/// Sets the native handle of the cursor (platform-specific)
WXD_EXPORTED void
wxd_Cursor_SetHandle(wxd_Cursor_t* cursor, void* handle);

// --- Global Cursor Functions ---

/// Sets the global cursor for the application
WXD_EXPORTED void
wxd_SetCursor(wxd_Cursor_t* cursor);

/// Gets the current global cursor
WXD_EXPORTED wxd_Cursor_t*
wxd_GetCursor();

/// Begins busy cursor (shows wait cursor)
WXD_EXPORTED void
wxd_BeginBusyCursor(wxd_Cursor_t* cursor);

/// Ends busy cursor (restores previous cursor)
WXD_EXPORTED void
wxd_EndBusyCursor();

/// Returns true if busy cursor is active
WXD_EXPORTED bool
wxd_IsBusy();

#ifdef __cplusplus
}
#endif

#endif // WXD_CURSOR_H