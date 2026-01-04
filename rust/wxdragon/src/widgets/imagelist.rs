//! ImageList widget wrapper.

use crate::bitmap::Bitmap;
use wxdragon_sys as ffi; // Corrected path: crate::bitmap instead of crate::widgets::bitmap

/// Represents a wxImageList.
///
/// This struct is a wrapper around the wxWidgets ImageList object.
/// It provides owned and unowned variants - owned instances are destroyed on Drop,
/// while unowned instances (obtained from controls) are managed by wxWidgets.
#[derive(Debug)]
pub struct ImageList {
    pub(crate) ptr: *mut ffi::wxd_ImageList_t,
    /// Indicates whether Rust owns this ImageList and should destroy it on Drop.
    /// `false` if this ImageList is wrapping a pointer obtained from wxWidgets
    /// that wxWidgets still owns (e.g., via `get_image_list`).
    pub(crate) owned: bool,
}

impl ImageList {
    /// Creates a new, empty image list.
    ///
    /// # Arguments
    /// * `width` - The width of images in the list.
    /// * `height` - The height of images in the list.
    /// * `mask` - `true` to use a mask, `false` otherwise.
    /// * `initial_count` - The initial number of images the list can store (can be resized).
    ///
    /// Returns `ImageList` on success. Panics if creation failed.
    pub fn new(width: i32, height: i32, mask: bool, initial_count: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("ImageList::new: width and height must be positive.");
        }
        let ptr = unsafe { ffi::wxd_ImageList_Create(width, height, mask, initial_count) };
        if ptr.is_null() {
            panic!("ImageList::new: FFI call to wxd_ImageList_Create failed to create ImageList.");
        } else {
            ImageList { ptr, owned: true }
        }
    }

    /// Creates a new `ImageList` from a raw pointer, for an ImageList whose lifetime is managed by wxWidgets.
    /// Rust will not attempt to destroy this ImageList on Drop.
    /// This is typically used internally when retrieving an ImageList from a control.
    pub(crate) unsafe fn from_ptr_unowned(ptr: *mut ffi::wxd_ImageList_t) -> Self {
        ImageList { ptr, owned: false }
    }

    /// Returns the raw pointer to the underlying wxImageList object.
    ///
    /// This pointer can be used for FFI calls. The caller must ensure the
    /// `ImageList` object (and thus the pointer) remains valid.
    pub fn as_ptr(&self) -> *mut ffi::wxd_ImageList_t {
        self.ptr
    }

    /// Adds a bitmap to the image list.
    /// The `wxImageList` makes an internal copy of the bitmap.
    ///
    /// # Arguments
    /// * `bitmap` - The bitmap to add.
    ///
    /// Returns the index of the added image, or -1 if an error occurred.
    pub fn add_bitmap(&self, bitmap: &Bitmap) -> i32 {
        if self.ptr.is_null() || bitmap.as_const_ptr().is_null() {
            return -1;
        }
        unsafe { ffi::wxd_ImageList_Add(self.ptr, bitmap.as_const_ptr()) }
    }

    /// Adds a bitmap and its mask to the image list.
    /// The `wxImageList` makes internal copies of both the bitmap and the mask.
    ///
    /// # Arguments
    /// * `bitmap` - The bitmap to add.
    /// * `mask` - The mask for the bitmap.
    ///
    /// Returns the index of the added image, or -1 if an error occurred.
    pub fn add_bitmap_with_mask(&self, bitmap: &Bitmap, mask: &Bitmap) -> i32 {
        if self.ptr.is_null() || bitmap.as_const_ptr().is_null() || mask.as_const_ptr().is_null() {
            return -1;
        }
        unsafe { ffi::wxd_ImageList_AddWithMask(self.ptr, bitmap.as_const_ptr(), mask.as_const_ptr()) }
    }

    /// Returns the number of images in the list.
    pub fn get_image_count(&self) -> i32 {
        if self.ptr.is_null() {
            return 0; // Consistent with C++ returning 0 for null list
        }
        unsafe { ffi::wxd_ImageList_GetImageCount(self.ptr) }
    }

    /// Removes all images from the list.
    ///
    /// Returns `true` if successful, `false` otherwise (e.g., if the list pointer is null).
    pub fn remove_all(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ImageList_RemoveAll(self.ptr) }
    }
}

impl Drop for ImageList {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            unsafe {
                ffi::wxd_ImageList_Destroy(self.ptr);
            }
        }
    }
}

// Ensure ImageList is Send + Sync if it makes sense.
// wxWidgets objects are generally not Send/Sync unless explicitly designed for it.
// Since this wraps a raw pointer tied to the main UI thread, it's likely !Send and !Sync.
// For now, no explicit impl/negative impl. Default is !Send/!Sync due to raw pointer.
