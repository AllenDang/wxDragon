//!
//! Safe wrapper for wxBitmap.

use std::os::raw::{c_int, c_uchar};
use wxdragon_sys as ffi;

/// Represents a platform-dependent bitmap image.
#[derive(Debug)] // Keep Debug if useful, or remove if pointer isn't meaningful for debug
pub struct Bitmap {
    ptr: *mut ffi::wxd_Bitmap_t,
    is_owned: bool, // Tracks whether Rust owns this bitmap and should destroy it
}

impl AsRef<*mut ffi::wxd_Bitmap_t> for Bitmap {
    /// Returns a reference to the raw bitmap pointer.
    fn as_ref(&self) -> &*mut ffi::wxd_Bitmap_t {
        &self.ptr
    }
}

impl std::ops::Deref for Bitmap {
    type Target = *mut ffi::wxd_Bitmap_t;

    /// Dereferences to the raw bitmap pointer.
    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl Bitmap {
    /// Using wxNullBitmap to represent an invalid/empty bitmap.
    ///
    /// # Example
    /// ```rust
    /// # use wxdragon::prelude::*;
    /// // Get an empty bitmap (non-owning wrapper; Drop will not free it)
    /// let empty = Bitmap::null_bitmap();
    /// assert!(!empty.is_ok());
    /// ```
    /// Returns a non-owning wrapper around wxNullBitmap.
    /// This value will not free the underlying object on Drop.
    pub fn null_bitmap() -> Self {
        unsafe { Bitmap::from_ptr_unowned(ffi::wxd_Bitmap_GetNull() as *mut _) }
    }

    /// Checks if this bitmap is wxNullBitmap.
    pub fn is_null_bitmap(&self) -> bool {
        std::ptr::eq(self.ptr, unsafe { ffi::wxd_Bitmap_GetNull() })
    }

    /// Creates a new empty bitmap with the specified width and height.
    pub fn new(width: i32, height: i32) -> Option<Self> {
        if width <= 0 || height <= 0 {
            return None;
        }

        // Create RGBA data (4 bytes per pixel)
        let pixel_count = (width * height * 4) as usize;
        let data = vec![0; pixel_count]; // All zeros for a fully transparent bitmap

        Self::from_rgba(&data, width as u32, height as u32)
    }

    /// Creates a new bitmap from raw RGBA pixel data.
    ///
    /// # Arguments
    /// * `data` - A slice containing the raw RGBA pixel data (4 bytes per pixel).
    /// * `width` - The width of the image in pixels.
    /// * `height` - The height of the image in pixels.
    ///
    /// Returns `None` if the bitmap creation fails (e.g., invalid dimensions, memory allocation error).
    pub fn from_rgba(data: &[u8], width: u32, height: u32) -> Option<Self> {
        let expected_len = (width * height * 4) as usize;
        if data.len() != expected_len || width == 0 || height == 0 {
            log::error!(
                "Bitmap::from_rgba: Invalid data length or dimensions. Expected {}, got {}, w={}, h={}", 
                expected_len, data.len(), width, height
            );
            return None;
        }

        let data = data.as_ptr() as *const c_uchar;
        let ptr = unsafe { ffi::wxd_Bitmap_CreateFromRGBA(data, width as c_int, height as c_int) };

        if ptr.is_null() {
            None
        } else {
            Some(Bitmap {
                ptr,
                is_owned: true,
            }) // We own bitmaps created this way
        }
    }

    /// Creates a bitmap wrapper around an existing bitmap pointer, transferring ownership to Rust.
    /// The bitmap will be destroyed when the wrapper is dropped.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid wxBitmap pointer, and no other code should destroy it.
    pub(crate) fn from_ptr_owned(ptr: *mut ffi::wxd_Bitmap_t) -> Self {
        Bitmap {
            ptr,
            is_owned: true,
        }
    }

    /// Creates a bitmap wrapper around an existing bitmap pointer without taking ownership.
    /// Rust will NOT destroy the bitmap when the wrapper is dropped.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid `wxd_Bitmap_t` pointer that is managed (lifetime-wise)
    /// by other code (e.g., wxWidgets internals). The pointer must remain valid for the
    /// lifetime of this `Bitmap` object.
    pub(crate) unsafe fn from_ptr_unowned(ptr: *mut ffi::wxd_Bitmap_t) -> Self {
        Bitmap {
            ptr,
            is_owned: false,
        }
    }

    /// Consumes the Bitmap and returns the raw pointer, transferring ownership to the caller.
    #[allow(dead_code)]
    pub(crate) fn into_raw(mut self) -> *mut ffi::wxd_Bitmap_t {
        let ptr = self.ptr;
        // Prevent Drop from freeing the pointer we are returning.
        self.ptr = std::ptr::null_mut();
        self.is_owned = false;
        ptr
    }

    /// Returns the width of the bitmap in pixels.
    pub fn get_width(&self) -> i32 {
        if self.ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Bitmap_GetWidth(self.ptr) as i32 }
    }

    /// Returns the height of the bitmap in pixels.
    pub fn get_height(&self) -> i32 {
        if self.ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Bitmap_GetHeight(self.ptr) as i32 }
    }

    /// Checks if the bitmap is valid.
    pub fn is_ok(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Bitmap_IsOk(self.ptr) }
    }

    /// Extracts the raw RGBA pixel data from the bitmap.
    ///
    /// Returns a vector containing RGBA pixel data where each pixel is represented
    /// by 4 consecutive bytes: R, G, B, A. The data is ordered row by row from
    /// top to bottom, left to right within each row.
    ///
    /// # Returns
    /// - `Some(Vec<u8>)` containing RGBA data if extraction succeeds
    /// - `None` if the bitmap is invalid or extraction fails
    ///
    /// # Example
    /// ```rust
    /// # use wxdragon::prelude::*;
    /// # fn example() -> Option<()> {
    /// let bitmap = Bitmap::new(100, 100)?;
    /// let rgba_data = bitmap.get_rgba_data()?;
    ///
    /// // Each pixel takes 4 bytes (RGBA)
    /// assert_eq!(rgba_data.len(), 100 * 100 * 4);
    ///
    /// // Use with image crate:
    /// // let img = image::RgbaImage::from_raw(100, 100, rgba_data)?;
    /// # Some(())
    /// # }
    /// ```
    pub fn get_rgba_data(&self) -> Option<Vec<u8>> {
        if self.ptr.is_null() || !self.is_ok() {
            return None;
        }

        unsafe {
            let data_ptr = ffi::wxd_Bitmap_GetRGBAData(self.ptr);
            if data_ptr.is_null() {
                return None;
            }

            let width = self.get_width() as usize;
            let height = self.get_height() as usize;
            let data_len = width * height * 4; // 4 bytes per pixel (RGBA)

            // Copy the data from C++ allocated memory to Rust Vec
            let rgba_data = std::slice::from_raw_parts(data_ptr, data_len).to_vec();

            // Free the C++ allocated memory
            ffi::wxd_Bitmap_FreeRGBAData(data_ptr);

            Some(rgba_data)
        }
    }
}

impl Clone for Bitmap {
    fn clone(&self) -> Self {
        unsafe {
            let cloned_ptr = ffi::wxd_Bitmap_Clone(self.ptr);
            if cloned_ptr.is_null() {
                panic!(
                    "Failed to clone wxBitmap: wxd_Bitmap_Clone returned null. Original: {:?}",
                    self.ptr
                );
            }
            // A cloned bitmap is always owned by Rust
            Bitmap {
                ptr: cloned_ptr,
                is_owned: true,
            }
        }
    }
}

impl Drop for Bitmap {
    /// Destroys the associated C++ wxBitmap object if Rust owns the bitmap.
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.is_owned {
            unsafe { ffi::wxd_Bitmap_Destroy(self.ptr) };
        }
    }
}
