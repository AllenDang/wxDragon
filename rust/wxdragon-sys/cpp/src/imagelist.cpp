#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/imaglist.h>
#include <wx/bitmap.h>

// Helper to cast to wxImageList*
static inline wxImageList*
ToWxImageList(wxd_ImageList_t* self)
{
    return reinterpret_cast<wxImageList*>(self);
}

// Helper to cast to wxBitmap*
static inline const wxBitmap*
ToWxBitmap(const wxd_Bitmap_t* self)
{
    return reinterpret_cast<const wxBitmap*>(self);
}

WXD_EXPORTED wxd_ImageList_t*
wxd_ImageList_Create(int width, int height, bool mask, int initialCount)
{
    if (width <= 0 || height <= 0) {
        // wxImageList asserts if width/height is 0, better to return null early.
        return nullptr;
    }
    wxImageList* list = new wxImageList(width, height, mask, initialCount);
    return reinterpret_cast<wxd_ImageList_t*>(list);
}

WXD_EXPORTED void
wxd_ImageList_Destroy(wxd_ImageList_t* self)
{
    delete ToWxImageList(self);
}

WXD_EXPORTED int
wxd_ImageList_Add(wxd_ImageList_t* self, const wxd_Bitmap_t* bitmap_ptr)
{
    wxImageList* list = ToWxImageList(self);
    const wxBitmap* bitmap = ToWxBitmap(bitmap_ptr);
    if (!list || !bitmap || !bitmap->IsOk()) {
        return -1;
    }
    // wxImageList::Add copies the bitmap
    return list->Add(*bitmap);
}

WXD_EXPORTED int
wxd_ImageList_AddWithMask(wxd_ImageList_t* self, const wxd_Bitmap_t* bitmap_ptr,
                          const wxd_Bitmap_t* mask_ptr)
{
    wxImageList* list = ToWxImageList(self);
    const wxBitmap* bitmap = ToWxBitmap(bitmap_ptr);
    const wxBitmap* mask = ToWxBitmap(mask_ptr);

    if (!list || !bitmap || !bitmap->IsOk()) {
        return -1;
    }
    if (!mask || !mask->IsOk()) { // Mask must also be valid
        return -1;
    }
    // wxImageList::Add copies both bitmaps
    return list->Add(*bitmap, *mask);
}

WXD_EXPORTED int
wxd_ImageList_GetImageCount(wxd_ImageList_t* self)
{
    wxImageList* list = ToWxImageList(self);
    if (!list) {
        return 0; // Or -1 to indicate error, but 0 is typical for empty/null
    }
    return list->GetImageCount();
}

WXD_EXPORTED bool
wxd_ImageList_RemoveAll(wxd_ImageList_t* self)
{
    wxImageList* list = ToWxImageList(self);
    if (!list) {
        return false;
    }
    return list->RemoveAll();
}