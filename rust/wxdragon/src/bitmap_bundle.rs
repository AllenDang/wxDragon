//!
//! Safe wrapper for wxBitmapBundle.

use std::ffi::CString;
use std::path::Path;
use wxdragon_sys as ffi;

use crate::bitmap::Bitmap;
use crate::geometry::Size;
use crate::window::WxWidget;

/// Represents a collection of bitmaps of the same image in different sizes/resolutions.
///
/// This class allows the application to provide images in different resolutions so
/// that they can be displayed optimally at different DPI levels. It's especially useful
/// for toolbar icons and other UI elements that need to scale well at high DPI.
///
/// BitmapBundle can be created from:
/// - A single bitmap (for backwards compatibility)
/// - Multiple bitmaps of different sizes (for optimal display at different DPI levels)
/// - An SVG file (which can be rendered at any size)
/// - SVG data in memory
/// - SVG string content
#[derive(Debug)]
pub struct BitmapBundle {
    ptr: *mut ffi::wxd_BitmapBundle_t,
    is_owned: bool, // Tracks whether Rust owns this bundle and should destroy it
}

impl BitmapBundle {
    /// Creates a new empty bitmap bundle.
    ///
    /// While valid, the empty bundle is not particularly useful as it doesn't
    /// contain any bitmaps to be shown.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_BitmapBundle_Create() };
        BitmapBundle { ptr, is_owned: true }
    }

    /// Creates a BitmapBundle from an existing pointer, taking ownership of it.
    ///
    /// This is primarily for internal use when receiving a bundle from wxWidgets.
    ///
    /// # Safety
    /// The pointer must be a valid wxBitmapBundle pointer that can be owned
    /// and eventually destroyed by Rust.
    pub fn from_ptr_owned(ptr: *mut ffi::wxd_BitmapBundle_t) -> Self {
        BitmapBundle { ptr, is_owned: true }
    }

    /// Creates a bitmap bundle from a single bitmap.
    ///
    /// This is primarily for backward compatibility with APIs that used to take a wxBitmap.
    pub fn from_bitmap(bitmap: &Bitmap) -> Self {
        let ptr = unsafe { ffi::wxd_BitmapBundle_CreateFromBitmap(bitmap.as_const_ptr()) };
        BitmapBundle { ptr, is_owned: true }
    }

    /// Creates a bitmap bundle from multiple bitmaps of different sizes.
    ///
    /// This is the preferred way to create a bitmap bundle for applications supporting
    /// multiple DPIs. Provide the same image in different resolutions to allow wxWidgets
    /// to choose the most appropriate one for the current display.
    ///
    /// # Arguments
    /// * `bitmaps` - A vector of bitmaps to include in the bundle. For best results,
    ///   include versions at 100%, 150%, and 200% of the base size (e.g., 16x16, 24x24, 32x32).
    ///
    /// # Returns
    /// A new bitmap bundle containing all the provided bitmaps.
    /// If the vector is empty, returns an empty bundle.
    pub fn from_bitmaps(bitmaps: &[Bitmap]) -> Self {
        if bitmaps.is_empty() {
            return Self::new();
        }

        // Create an array of pointers to the bitmaps
        let bitmap_ptrs: Vec<*const ffi::wxd_Bitmap_t> = bitmaps.iter().map(|bmp| bmp.as_const_ptr()).collect();

        let ptr = unsafe { ffi::wxd_BitmapBundle_FromBitmaps(bitmap_ptrs.as_ptr(), bitmaps.len()) };

        BitmapBundle { ptr, is_owned: true }
    }

    /// Creates a bitmap bundle from an SVG file.
    ///
    /// # Arguments
    /// * `path` - Path to the SVG file.
    /// * `default_size` - The size to use when the exact size needed is not known.
    ///
    /// # Returns
    /// None if the file could not be loaded or if the SVG is invalid.
    pub fn from_svg_file<P: AsRef<Path>>(path: P, default_size: Size) -> Option<Self> {
        let c_path = match path.as_ref().to_str().map(CString::new) {
            Some(Ok(s)) => s,
            _ => return None,
        };

        let size: ffi::wxd_Size = default_size.into();

        let ptr = unsafe { ffi::wxd_BitmapBundle_FromSVGFile(c_path.as_ptr(), size) };
        if ptr.is_null() {
            None
        } else {
            Some(BitmapBundle { ptr, is_owned: true })
        }
    }

    /// Creates a bitmap bundle from SVG text.
    ///
    /// # Arguments
    /// * `svg_text` - String containing SVG content.
    /// * `default_size` - The size to use when the exact size needed is not known.
    ///
    /// # Returns
    /// None if the SVG content is invalid.
    pub fn from_svg_text(svg_text: &str, default_size: Size) -> Option<Self> {
        let c_svg = match CString::new(svg_text) {
            Ok(s) => s,
            Err(_) => return None,
        };

        let size: ffi::wxd_Size = default_size.into();

        let ptr = unsafe { ffi::wxd_BitmapBundle_FromSVGText(c_svg.as_ptr(), size) };
        if ptr.is_null() {
            None
        } else {
            Some(BitmapBundle { ptr, is_owned: true })
        }
    }

    /// Creates a bitmap bundle from raw SVG data.
    ///
    /// # Arguments
    /// * `data` - Byte slice containing SVG data.
    /// * `default_size` - The size to use when the exact size needed is not known.
    ///
    /// # Returns
    /// None if the SVG data is invalid.
    pub fn from_svg_data(data: &[u8], default_size: Size) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let size: ffi::wxd_Size = default_size.into();

        let ptr = unsafe { ffi::wxd_BitmapBundle_FromSVGData(data.as_ptr(), data.len(), size) };

        if ptr.is_null() {
            None
        } else {
            Some(BitmapBundle { ptr, is_owned: true })
        }
    }

    /// Retrieves a bitmap of the specified size from the bundle.
    ///
    /// The returned bitmap is the best match for the requested size, possibly scaled
    /// from one of the bitmaps in the bundle or rendered from SVG if the bundle was
    /// created from SVG content.
    ///
    /// # Arguments
    /// * `size` - Desired size in pixels.
    ///
    /// # Returns
    /// None if the bundle is invalid or couldn't create a bitmap.
    pub fn get_bitmap(&self, size: Size) -> Option<Bitmap> {
        let wxd_size: ffi::wxd_Size = size.into();

        let bitmap_ptr = unsafe { ffi::wxd_BitmapBundle_GetBitmap(self.ptr, wxd_size) };

        if bitmap_ptr.is_null() {
            None
        } else {
            // The C++ side returns a new bitmap that we own
            Some(Bitmap::from(bitmap_ptr))
        }
    }

    /// Retrieves a bitmap sized appropriately for the DPI scaling of the given window.
    ///
    /// This is the recommended way to get a bitmap from the bundle when you know
    /// which window it will be displayed in, as it ensures the bitmap is sized
    /// correctly for the window's DPI.
    ///
    /// # Arguments
    /// * `window` - The window where the bitmap will be displayed.
    ///
    /// # Returns
    /// None if the bundle is invalid, the window is invalid, or a bitmap couldn't be created.
    pub fn get_bitmap_for(&self, window: &dyn WxWidget) -> Option<Bitmap> {
        let bitmap_ptr = unsafe { ffi::wxd_BitmapBundle_GetBitmapFor(self.ptr, window.handle_ptr()) };

        if bitmap_ptr.is_null() {
            None
        } else {
            // The C++ side returns a new bitmap that we own
            Some(Bitmap::from(bitmap_ptr))
        }
    }

    /// Gets the default size of the bundle.
    ///
    /// This is the size that the bundle considers its "standard" size,
    /// typically the size of the original bitmap or the default size specified
    /// when creating from SVG.
    pub fn get_default_size(&self) -> Size {
        let size = unsafe { ffi::wxd_BitmapBundle_GetDefaultSize(self.ptr) };
        Size::from(size)
    }

    /// Gets the preferred bitmap size for a specific scale factor.
    ///
    /// This returns the size of the bitmap that would be best to use at
    /// the given scale factor (e.g., 1.0 for standard DPI, 2.0 for 200% scaling).
    ///
    /// # Arguments
    /// * `scale` - The scale factor (1.0 = 100%, 2.0 = 200%, etc.)
    pub fn get_preferred_bitmap_size_at_scale(&self, scale: f64) -> Size {
        let size = unsafe { ffi::wxd_BitmapBundle_GetPreferredBitmapSizeAtScale(self.ptr, scale) };
        Size::from(size)
    }

    /// Gets the preferred bitmap size for a specific window.
    ///
    /// This returns the size of the bitmap that would be best to use for
    /// the DPI scaling of the given window.
    ///
    /// # Arguments
    /// * `window` - The window where the bitmap will be displayed.
    pub fn get_preferred_bitmap_size_for(&self, window: &dyn WxWidget) -> Size {
        let size = unsafe { ffi::wxd_BitmapBundle_GetPreferredBitmapSizeFor(self.ptr, window.handle_ptr()) };
        Size::from(size)
    }

    /// Checks if the bitmap bundle is valid and contains at least one bitmap.
    pub fn is_ok(&self) -> bool {
        !self.ptr.is_null() && unsafe { ffi::wxd_BitmapBundle_IsOk(self.ptr) }
    }

    /// Returns the raw bundle pointer.
    /// Use with caution, primarily for internal FFI calls.
    pub fn as_ptr(&self) -> *mut ffi::wxd_BitmapBundle_t {
        self.ptr
    }
}

impl Clone for BitmapBundle {
    fn clone(&self) -> Self {
        let cloned_ptr = unsafe { ffi::wxd_BitmapBundle_Clone(self.ptr) };
        if cloned_ptr.is_null() {
            panic!(
                "Failed to clone wxBitmapBundle: wxd_BitmapBundle_Clone returned null. Original: {:?}",
                self.ptr
            );
        }
        // A cloned bundle is always owned by Rust
        BitmapBundle {
            ptr: cloned_ptr,
            is_owned: true,
        }
    }
}

impl Drop for BitmapBundle {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.is_owned {
            unsafe { ffi::wxd_BitmapBundle_Destroy(self.ptr) };
        }
    }
}

impl Default for BitmapBundle {
    fn default() -> Self {
        Self::new()
    }
}
