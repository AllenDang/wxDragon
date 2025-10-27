//!
//! Safe wrapper for wxBitmap.

use std::os::raw::{c_int, c_uchar};
use wxdragon_sys as ffi;

/// Represents a platform-dependent bitmap image.
#[derive(Debug)] // Keep Debug if useful, or remove if pointer isn't meaningful for debug
pub struct Bitmap {
    ptr: *const ffi::wxd_Bitmap_t,
    is_owned: bool, // Tracks whether Rust owns this bitmap and should destroy it
}

impl AsRef<*const ffi::wxd_Bitmap_t> for Bitmap {
    /// Returns a reference to the raw bitmap pointer.
    fn as_ref(&self) -> &*const ffi::wxd_Bitmap_t {
        &self.ptr
    }
}

impl std::ops::Deref for Bitmap {
    type Target = *const ffi::wxd_Bitmap_t;

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
        unsafe { Bitmap::from(ffi::wxd_Bitmap_GetNull()) }
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
                expected_len,
                data.len(),
                width,
                height
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

    pub fn is_owned(&self) -> bool {
        self.is_owned
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

        let (mut width, mut height) = (0_usize, 0_usize);
        let data_ptr = unsafe { ffi::wxd_Bitmap_GetRGBAData(self.ptr, &mut width, &mut height) };
        if data_ptr.is_null() {
            return None;
        }

        // let width = self.get_width() as usize;
        // let height = self.get_height() as usize;
        let data_len = width * height * 4; // 4 bytes per pixel (RGBA)

        // Copy the data from C++ allocated memory to Rust Vec
        let rgba_data = unsafe { std::slice::from_raw_parts(data_ptr, data_len).to_vec() };

        // Free the C++ allocated memory
        unsafe { ffi::wxd_Bitmap_FreeRGBAData(data_ptr) };

        Some(rgba_data)
    }
}

impl Clone for Bitmap {
    fn clone(&self) -> Self {
        let cloned_ptr = unsafe { ffi::wxd_Bitmap_Clone(self.ptr) };
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

impl From<*const ffi::wxd_Bitmap_t> for Bitmap {
    /// Creates a non-owning Bitmap wrapper from a raw pointer.
    /// The pointer must be valid for the lifetime of the Bitmap object.
    fn from(ptr: *const ffi::wxd_Bitmap_t) -> Self {
        Bitmap {
            ptr,
            is_owned: false,
        }
    }
}

impl From<*mut ffi::wxd_Bitmap_t> for Bitmap {
    /// Creates an owning Bitmap wrapper from a raw pointer.
    /// The pointer must be valid and Rust will take ownership of it.
    fn from(ptr: *mut ffi::wxd_Bitmap_t) -> Self {
        Bitmap {
            ptr,
            is_owned: true,
        }
    }
}

impl TryFrom<Bitmap> for *const ffi::wxd_Bitmap_t {
    type Error = std::io::Error;
    fn try_from(bitmap: Bitmap) -> Result<Self, Self::Error> {
        if bitmap.is_owned {
            Err(std::io::Error::other(
                "Cannot convert owned Bitmap to raw pointer without transferring ownership",
            ))
        } else {
            Ok(bitmap.ptr)
        }
    }
}

impl TryFrom<Bitmap> for *mut ffi::wxd_Bitmap_t {
    type Error = std::io::Error;
    fn try_from(mut bitmap: Bitmap) -> Result<Self, Self::Error> {
        if bitmap.is_owned {
            bitmap.is_owned = false; // Prevent Drop from freeing it
            Ok(bitmap.ptr as *mut ffi::wxd_Bitmap_t)
        } else {
            Err(std::io::Error::other(
                "Cannot convert unowned Bitmap to mutable raw pointer",
            ))
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
