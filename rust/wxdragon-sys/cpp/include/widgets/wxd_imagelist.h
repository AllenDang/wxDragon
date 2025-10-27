#ifndef WXD_IMAGELIST_H
#define WXD_IMAGELIST_H

#include "../wxd_types.h" // For wxd_ImageList_t, wxd_Bitmap_t, WXD_EXPORTED

#ifdef __cplusplus
extern "C" {
#endif

// Creates a new wxImageList.
// Returns a pointer to the new image list, or NULL on failure.
// The caller (Rust) owns the returned wxImageList and is responsible for destroying it
// via wxd_ImageList_Destroy.
WXD_EXPORTED wxd_ImageList_t*
wxd_ImageList_Create(int width, int height, bool mask, int initialCount);

// Destroys a wxImageList previously created with wxd_ImageList_Create.
WXD_EXPORTED void
wxd_ImageList_Destroy(wxd_ImageList_t* self);

// Adds a bitmap to the image list. wxImageList makes an internal copy of the bitmap.
// Returns the index of the new image in the list, or -1 on failure.
WXD_EXPORTED int
wxd_ImageList_Add(wxd_ImageList_t* self, const wxd_Bitmap_t* bitmap);

// Adds a bitmap and a mask to the image list. wxImageList makes internal copies.
// Returns the index of the new image in the list, or -1 on failure.
WXD_EXPORTED int
wxd_ImageList_AddWithMask(wxd_ImageList_t* self, const wxd_Bitmap_t* bitmap,
                          const wxd_Bitmap_t* mask);

// Returns the number of images in the list.
WXD_EXPORTED int
wxd_ImageList_GetImageCount(wxd_ImageList_t* self);

// Removes all images from the list.
// Returns true if successful, false otherwise.
WXD_EXPORTED bool
wxd_ImageList_RemoveAll(wxd_ImageList_t* self);

#ifdef __cplusplus
}
#endif

#endif // WXD_IMAGELIST_H