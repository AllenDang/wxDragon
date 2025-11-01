//!
//! Safe wrapper for wxBitmap.

use std::marker::PhantomData;
use std::os::raw::{c_int, c_uchar};
use std::rc::Rc;
use wxdragon_sys as ffi;

/// Represents a platform-dependent bitmap image.
#[derive(Debug)] // Keep Debug if useful, or remove if pointer isn't meaningful for debug
pub struct Bitmap {
    ptr: *mut ffi::wxd_Bitmap_t,
    owned: bool, // Tracks whether Rust owns this bitmap and should destroy it
    // Prevent Send/Sync: wxWidgets objects are not thread-safe and must stay on UI thread.
    _nosend_nosync: PhantomData<Rc<()>>,
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
            return None;
        }
        Some(Bitmap {
            ptr,
            owned: true,
            _nosend_nosync: PhantomData,
        }) // We own bitmaps created this way
    }

    /// Returns `true` if this bitmap is owned by Rust and will be automatically destroyed when dropped.
    ///
    /// Returns `false` if the bitmap is managed elsewhere (e.g., by wxWidgets or another owner)
    /// and should not be destroyed by Rust code. This is important for avoiding double-free or
    /// use-after-free bugs when working with bitmaps obtained from external sources.
    ///
    /// Typically, bitmaps created via Rust constructors (such as [`Bitmap::from_rgba`]) are owned,
    /// while those wrapping external pointers are not.
    pub fn is_owned(&self) -> bool {
        self.owned
    }

    /// Returns the width of the bitmap in pixels.
    pub fn get_width(&self) -> i32 {
        if self.as_const_ptr().is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Bitmap_GetWidth(self.as_const_ptr()) as i32 }
    }

    /// Returns the height of the bitmap in pixels.
    pub fn get_height(&self) -> i32 {
        if self.as_const_ptr().is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Bitmap_GetHeight(self.as_const_ptr()) as i32 }
    }

    /// Checks if the bitmap is valid.
    pub fn is_ok(&self) -> bool {
        if self.as_const_ptr().is_null() {
            return false;
        }
        unsafe { ffi::wxd_Bitmap_IsOk(self.as_const_ptr()) }
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
        if self.as_const_ptr().is_null() || !self.is_ok() {
            return None;
        }

        let (mut width, mut height) = (0_usize, 0_usize);
        let data_ptr = unsafe { ffi::wxd_Bitmap_GetRGBAData(self.as_const_ptr(), &mut width, &mut height) };
        if data_ptr.is_null() {
            return None;
        }

        let data_len = width * height * 4; // 4 bytes per pixel (RGBA)

        // Copy the data from C++ allocated memory to Rust Vec
        let rgba_data = unsafe { std::slice::from_raw_parts(data_ptr, data_len).to_vec() };

        // Free the C++ allocated memory
        unsafe { ffi::wxd_Bitmap_FreeRGBAData(data_ptr) };

        Some(rgba_data)
    }

    /// Returns a const raw pointer to the underlying wxd_Bitmap_t.
    ///
    /// Ownership notes:
    /// - This does not transfer ownership; the pointer is only valid while this `Bitmap` (or another owner) keeps it alive.
    /// - Do not destroy this pointer yourself; the owner (or a caller who used `into_raw_mut`) is responsible.
    pub fn as_const_ptr(&self) -> *const ffi::wxd_Bitmap_t {
        self.ptr as *const _
    }

    /// Returns a mutable raw pointer to the underlying wxd_Bitmap_t.
    ///
    /// Use with care; prefer safe methods when possible.
    ///
    /// Ownership notes:
    /// - This does not transfer ownership. If you mutate through this pointer, ensure exclusive access and maintain invariants.
    /// - Do not destroy the returned pointer here; use `into_raw_mut` if you must assume ownership and destroy manually.
    pub fn as_mut_ptr(&mut self) -> *mut ffi::wxd_Bitmap_t {
        self.ptr
    }

    /// Consumes self and returns a raw mutable pointer, transferring ownership to the caller.
    ///
    /// After calling this, you must NOT use the original `Bitmap` again.
    ///
    /// Caller responsibilities:
    /// - You now own the pointer and must destroy it exactly once with `wxd_Bitmap_Destroy`.
    /// - Ensure the pointer is not used after destruction.
    pub fn into_raw_mut(self) -> *mut ffi::wxd_Bitmap_t {
        self.try_into()
            .expect("into_raw_mut can only be called on owning Bitmap instances")
    }

    /// Consumes a borrowed (non-owning) wrapper and returns a raw const pointer without taking ownership.
    ///
    /// Panics if called on an owning wrapper to avoid leaking the owned resource.
    ///
    /// Caller responsibilities:
    /// - This does NOT transfer ownership; do not destroy the returned pointer.
    /// - The pointer remains valid only while the original owner keeps it alive.
    pub fn into_raw_const(self) -> *const ffi::wxd_Bitmap_t {
        self.try_into()
            .expect("into_raw_const must only be used on non-owning (borrowed) wrappers")
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
            owned: true,
            _nosend_nosync: PhantomData,
        }
    }
}

impl From<*const ffi::wxd_Bitmap_t> for Bitmap {
    /// Creates a non-owning Bitmap wrapper from a raw pointer.
    /// The pointer must be valid for the lifetime of the Bitmap object.
    fn from(ptr: *const ffi::wxd_Bitmap_t) -> Self {
        assert!(!ptr.is_null(), "invalid null pointer passed to Bitmap::from");
        Bitmap {
            ptr: ptr as *mut _,
            owned: false,
            _nosend_nosync: PhantomData,
        }
    }
}

impl From<*mut ffi::wxd_Bitmap_t> for Bitmap {
    /// Creates an owning Bitmap wrapper from a raw pointer.
    /// The pointer must be valid and Rust will take ownership of it.
    fn from(ptr: *mut ffi::wxd_Bitmap_t) -> Self {
        assert!(!ptr.is_null(), "invalid null pointer passed to Bitmap::from");
        Bitmap {
            ptr,
            owned: true,
            _nosend_nosync: PhantomData,
        }
    }
}

impl TryFrom<Bitmap> for *const ffi::wxd_Bitmap_t {
    type Error = std::io::Error;
    fn try_from(bitmap: Bitmap) -> Result<Self, Self::Error> {
        if bitmap.owned {
            Err(std::io::Error::other(
                "Cannot convert owned Bitmap to raw pointer without transferring ownership",
            ))
        } else {
            let ptr = bitmap.ptr as *const _;
            std::mem::forget(bitmap);
            Ok(ptr)
        }
    }
}

impl TryFrom<Bitmap> for *mut ffi::wxd_Bitmap_t {
    type Error = std::io::Error;
    fn try_from(bitmap: Bitmap) -> Result<Self, Self::Error> {
        if bitmap.owned {
            let ptr = bitmap.ptr;
            std::mem::forget(bitmap); // Prevent Drop from freeing it
            Ok(ptr)
        } else {
            Err(std::io::Error::other("Cannot convert unowned Bitmap to mutable raw pointer"))
        }
    }
}

impl Drop for Bitmap {
    /// Destroys the associated C++ wxBitmap object if Rust owns the bitmap.
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.owned {
            unsafe { ffi::wxd_Bitmap_Destroy(self.as_mut_ptr()) };
            self.ptr = std::ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Bitmap;
    use wxdragon_sys as ffi;

    #[test]
    fn bitmap_from_rgba_clone_into_raw_then_destroy() {
        let (w, h) = (2u32, 2u32);
        let rgba = vec![255u8; (w * h * 4) as usize];

        // Create an owned bitmap from RGBA data
        let bmp = Bitmap::from_rgba(&rgba, w, h).expect("failed to create bitmap from rgba");
        assert!(bmp.is_owned(), "bitmap from_rgba should be owned");
        assert!(bmp.is_ok(), "created bitmap should be ok");

        // Clone the bitmap; clone is also owned
        let clone = bmp.clone();
        assert!(clone.is_owned(), "cloned bitmap should be owned");
        assert!(clone.is_ok(), "cloned bitmap should be ok");

        // Transfer ownership out of the clone and destroy manually
        let raw = clone.into_raw_mut();
        assert!(!raw.is_null(), "raw pointer from into_raw_mut should be non-null");
        unsafe { ffi::wxd_Bitmap_Destroy(raw) };

        // When this test ends, `bmp` will be dropped and should destroy its own handle.
        // If ownership transfer or Drop were incorrect, this test would double-free or leak.
    }
}
