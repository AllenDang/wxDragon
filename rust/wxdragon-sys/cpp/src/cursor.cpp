#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/cursor.h>
#include <wx/bitmap.h>
#include <wx/utils.h>
#include <cstring>

extern "C" {

// Mapping function to convert our enum to wxWidgets stock cursor IDs
static wxStockCursor
map_stock_cursor(wxd_StockCursor cursor_id)
{
    switch (cursor_id) {
    case WXD_CURSOR_ARROW:
        return wxCURSOR_ARROW;
    case WXD_CURSOR_RIGHT_ARROW:
        return wxCURSOR_RIGHT_ARROW;
    case WXD_CURSOR_BULLSEYE:
        return wxCURSOR_BULLSEYE;
    case WXD_CURSOR_CHAR:
        return wxCURSOR_CHAR;
    case WXD_CURSOR_CROSS:
        return wxCURSOR_CROSS;
    case WXD_CURSOR_HAND:
        return wxCURSOR_HAND;
    case WXD_CURSOR_IBEAM:
        return wxCURSOR_IBEAM;
    case WXD_CURSOR_LEFT_BUTTON:
        return wxCURSOR_LEFT_BUTTON;
    case WXD_CURSOR_MAGNIFIER:
        return wxCURSOR_MAGNIFIER;
    case WXD_CURSOR_MIDDLE_BUTTON:
        return wxCURSOR_MIDDLE_BUTTON;
    case WXD_CURSOR_NO_ENTRY:
        return wxCURSOR_NO_ENTRY;
    case WXD_CURSOR_PAINT_BRUSH:
        return wxCURSOR_PAINT_BRUSH;
    case WXD_CURSOR_PENCIL:
        return wxCURSOR_PENCIL;
    case WXD_CURSOR_POINT_LEFT:
        return wxCURSOR_POINT_LEFT;
    case WXD_CURSOR_POINT_RIGHT:
        return wxCURSOR_POINT_RIGHT;
    case WXD_CURSOR_QUESTION_ARROW:
        return wxCURSOR_QUESTION_ARROW;
    case WXD_CURSOR_RIGHT_BUTTON:
        return wxCURSOR_RIGHT_BUTTON;
    case WXD_CURSOR_SIZENESW:
        return wxCURSOR_SIZENESW;
    case WXD_CURSOR_SIZENS:
        return wxCURSOR_SIZENS;
    case WXD_CURSOR_SIZENWSE:
        return wxCURSOR_SIZENWSE;
    case WXD_CURSOR_SIZEWE:
        return wxCURSOR_SIZEWE;
    case WXD_CURSOR_SIZING:
        return wxCURSOR_SIZING;
    case WXD_CURSOR_SPRAYCAN:
        return wxCURSOR_SPRAYCAN;
    case WXD_CURSOR_WAIT:
        return wxCURSOR_WAIT;
    case WXD_CURSOR_WATCH:
        return wxCURSOR_WATCH;
    case WXD_CURSOR_BLANK:
        return wxCURSOR_BLANK;
    case WXD_CURSOR_DEFAULT:
        return wxCURSOR_DEFAULT;
    case WXD_CURSOR_ARROWWAIT:
        return wxCURSOR_ARROWWAIT;
    default:
        return wxCURSOR_ARROW;
    }
}

static wxBitmapType
map_bitmap_type(wxd_BitmapType type)
{
    switch (type) {
    case WXD_BITMAP_TYPE_BMP:
        return wxBITMAP_TYPE_BMP;
    case WXD_BITMAP_TYPE_ICO:
        return wxBITMAP_TYPE_ICO;
    case WXD_BITMAP_TYPE_CUR:
        return wxBITMAP_TYPE_CUR;
    case WXD_BITMAP_TYPE_XBM:
        return wxBITMAP_TYPE_XBM;
    case WXD_BITMAP_TYPE_XPM:
        return wxBITMAP_TYPE_XPM;
    case WXD_BITMAP_TYPE_PNG:
        return wxBITMAP_TYPE_PNG;
    case WXD_BITMAP_TYPE_JPEG:
        return wxBITMAP_TYPE_JPEG;
    case WXD_BITMAP_TYPE_GIF:
        return wxBITMAP_TYPE_GIF;
    case WXD_BITMAP_TYPE_ANI:
        return wxBITMAP_TYPE_ANI;
    case WXD_BITMAP_TYPE_ANY:
        return wxBITMAP_TYPE_ANY;
    default:
        return wxBITMAP_TYPE_ANY;
    }
}

// --- Cursor Creation Functions ---

WXD_EXPORTED wxd_Cursor_t*
wxd_Cursor_CreateStock(wxd_StockCursor cursor_id)
{
    if (cursor_id == WXD_CURSOR_NONE) {
        return nullptr;
    }

    try {
        wxStockCursor stock_id = map_stock_cursor(cursor_id);
        wxCursor* cursor = new wxCursor(stock_id);
        return reinterpret_cast<wxd_Cursor_t*>(cursor);
    }
    catch (...) {
        return nullptr;
    }
}

WXD_EXPORTED wxd_Cursor_t*
wxd_Cursor_CreateFromFile(const char* filename, wxd_BitmapType type, int hotspot_x, int hotspot_y)
{
    if (!filename) {
        return nullptr;
    }

    try {
        wxString wx_filename = wxString::FromUTF8(filename);
        wxBitmapType wx_type = map_bitmap_type(type);
        wxCursor* cursor = new wxCursor(wx_filename, wx_type, hotspot_x, hotspot_y);

        if (cursor && cursor->IsOk()) {
            return reinterpret_cast<wxd_Cursor_t*>(cursor);
        }
        else {
            delete cursor;
            return nullptr;
        }
    }
    catch (...) {
        return nullptr;
    }
}

WXD_EXPORTED wxd_Cursor_t*
wxd_Cursor_CreateFromData(const unsigned char* bits, int width, int height, int hotspot_x,
                          int hotspot_y, const unsigned char* mask_bits)
{
    if (!bits || width <= 0 || height <= 0) {
        return nullptr;
    }

    try {
#ifdef __WXMSW__
        // On Windows, we need to create a cursor from a wxImage
        // The raw bits constructor takes different parameters

        // Create bitmap from bits
        wxBitmap bitmap = wxBitmap(reinterpret_cast<const char*>(bits), width, height);

        if (mask_bits) {
            wxBitmap mask_bitmap =
                wxBitmap(reinterpret_cast<const char*>(mask_bits), width, height);
            bitmap.SetMask(new wxMask(mask_bitmap));
        }

        // Convert to image and set hotspot
        wxImage image = bitmap.ConvertToImage();
        image.SetOption(wxIMAGE_OPTION_CUR_HOTSPOT_X, hotspot_x);
        image.SetOption(wxIMAGE_OPTION_CUR_HOTSPOT_Y, hotspot_y);

        wxCursor* cursor = new wxCursor(image);

        if (cursor && cursor->IsOk()) {
            return reinterpret_cast<wxd_Cursor_t*>(cursor);
        }
        else {
            delete cursor;
            return nullptr;
        }
#elif defined(__WXGTK__)
        // On GTK, we can use the raw bits constructor with foreground/background colors
        wxCursor* cursor;
        if (mask_bits) {
            cursor = new wxCursor(reinterpret_cast<const char*>(bits), width, height, hotspot_x,
                                  hotspot_y, reinterpret_cast<const char*>(mask_bits), wxWHITE,
                                  wxBLACK);
        }
        else {
            // For GTK without mask, we need to create a simple bitmap
            wxBitmap bitmap = wxBitmap(reinterpret_cast<const char*>(bits), width, height);
            wxImage image = bitmap.ConvertToImage();
            image.SetOption(wxIMAGE_OPTION_CUR_HOTSPOT_X, hotspot_x);
            image.SetOption(wxIMAGE_OPTION_CUR_HOTSPOT_Y, hotspot_y);
            cursor = new wxCursor(image);
        }

        if (cursor && cursor->IsOk()) {
            return reinterpret_cast<wxd_Cursor_t*>(cursor);
        }
        else {
            delete cursor;
            return nullptr;
        }
#else
        // On other platforms (macOS, etc.), try to use the image-based approach
        wxBitmap bitmap = wxBitmap(reinterpret_cast<const char*>(bits), width, height);

        if (mask_bits) {
            wxBitmap mask_bitmap =
                wxBitmap(reinterpret_cast<const char*>(mask_bits), width, height);
            bitmap.SetMask(new wxMask(mask_bitmap));
        }

        wxImage image = bitmap.ConvertToImage();
        image.SetOption(wxIMAGE_OPTION_CUR_HOTSPOT_X, hotspot_x);
        image.SetOption(wxIMAGE_OPTION_CUR_HOTSPOT_Y, hotspot_y);

        wxCursor* cursor = new wxCursor(image);

        if (cursor && cursor->IsOk()) {
            return reinterpret_cast<wxd_Cursor_t*>(cursor);
        }
        else {
            delete cursor;
            return nullptr;
        }
#endif
    }
    catch (...) {
        return nullptr;
    }
}

WXD_EXPORTED wxd_Cursor_t*
wxd_Cursor_CreateFromImage(const wxd_Bitmap_t* image)
{
    if (!image) {
        return nullptr;
    }

    try {
        const wxBitmap* wx_bitmap = reinterpret_cast<const wxBitmap*>(image);
        wxImage wx_image = wx_bitmap->ConvertToImage();
        wxCursor* cursor = new wxCursor(wx_image);

        if (cursor && cursor->IsOk()) {
            return reinterpret_cast<wxd_Cursor_t*>(cursor);
        }
        else {
            delete cursor;
            return nullptr;
        }
    }
    catch (...) {
        return nullptr;
    }
}

WXD_EXPORTED wxd_Cursor_t*
wxd_Cursor_Copy(wxd_Cursor_t* cursor)
{
    if (!cursor) {
        return nullptr;
    }

    try {
        wxCursor* wx_cursor = reinterpret_cast<wxCursor*>(cursor);
        wxCursor* new_cursor = new wxCursor(*wx_cursor);
        return reinterpret_cast<wxd_Cursor_t*>(new_cursor);
    }
    catch (...) {
        return nullptr;
    }
}

// --- Cursor Destruction ---

WXD_EXPORTED void
wxd_Cursor_Destroy(wxd_Cursor_t* cursor)
{
    if (cursor) {
        wxCursor* wx_cursor = reinterpret_cast<wxCursor*>(cursor);
        delete wx_cursor;
    }
}

// --- Cursor Properties ---

WXD_EXPORTED bool
wxd_Cursor_IsOk(wxd_Cursor_t* cursor)
{
    if (!cursor) {
        return false;
    }

    wxCursor* wx_cursor = reinterpret_cast<wxCursor*>(cursor);
    return wx_cursor->IsOk();
}

WXD_EXPORTED wxd_Point
wxd_Cursor_GetHotSpot(wxd_Cursor_t* cursor)
{
    wxd_Point result = { -1, -1 };

    if (!cursor) {
        return result;
    }

    wxCursor* wx_cursor = reinterpret_cast<wxCursor*>(cursor);
    wxPoint hotspot = wx_cursor->GetHotSpot();

    result.x = hotspot.x;
    result.y = hotspot.y;
    return result;
}

WXD_EXPORTED void
wxd_Cursor_SetHotSpot(wxd_Cursor_t* cursor, int x, int y)
{
    // Note: wxWidgets doesn't provide a SetHotSpot method for cursors
    // This is included for completeness but may not be implemented
    // on all platforms
    (void)cursor;
    (void)x;
    (void)y;
}

// --- Platform-specific Functions ---

WXD_EXPORTED void*
wxd_Cursor_GetHandle(wxd_Cursor_t* cursor)
{
    if (!cursor) {
        return nullptr;
    }

    wxCursor* wx_cursor = reinterpret_cast<wxCursor*>(cursor);
#ifdef __WXMSW__
    return reinterpret_cast<void*>(wx_cursor->GetHandle());
#else
    // On non-Windows platforms, handle may not be available
    return nullptr;
#endif
}

WXD_EXPORTED void
wxd_Cursor_SetHandle(wxd_Cursor_t* cursor, void* handle)
{
    if (!cursor || !handle) {
        return;
    }

#ifdef __WXMSW__
    wxCursor* wx_cursor = reinterpret_cast<wxCursor*>(cursor);
    wx_cursor->SetHandle(reinterpret_cast<WXHANDLE>(handle));
#else
    // On non-Windows platforms, this may not be supported
    (void)cursor;
    (void)handle;
#endif
}

// --- Global Cursor Functions ---

WXD_EXPORTED void
wxd_SetCursor(wxd_Cursor_t* cursor)
{
    if (cursor) {
        wxCursor* wx_cursor = reinterpret_cast<wxCursor*>(cursor);
        wxSetCursor(*wx_cursor);
    }
    else {
        wxSetCursor(wxNullCursor);
    }
}

WXD_EXPORTED wxd_Cursor_t*
wxd_GetCursor()
{
    // Note: wxWidgets doesn't have a global wxGetCursor function
    // This would need to be implemented by tracking the current cursor
    return nullptr;
}

WXD_EXPORTED void
wxd_BeginBusyCursor(wxd_Cursor_t* cursor)
{
    if (cursor) {
        wxCursor* wx_cursor = reinterpret_cast<wxCursor*>(cursor);
        wxBeginBusyCursor(wx_cursor);
    }
    else {
        wxBeginBusyCursor();
    }
}

WXD_EXPORTED void
wxd_EndBusyCursor()
{
    wxEndBusyCursor();
}

WXD_EXPORTED bool
wxd_IsBusy()
{
    return wxIsBusy();
}

} // extern "C"