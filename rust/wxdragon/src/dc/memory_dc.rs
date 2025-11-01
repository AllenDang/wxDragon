use crate::bitmap::Bitmap;
use crate::dc::DeviceContext;

/// A device context for drawing to an off-screen bitmap.
///
/// MemoryDC can be used to draw to a bitmap, which can then be drawn
/// to another device context or saved to a file.
pub struct MemoryDC {
    dc_ptr: *mut wxdragon_sys::wxd_MemoryDC_t,
}

impl MemoryDC {
    /// Create a new memory device context
    pub fn new() -> Self {
        let dc_ptr = unsafe { wxdragon_sys::wxd_MemoryDC_Create() };
        Self { dc_ptr }
    }

    /// Select a bitmap to draw on
    ///
    /// # Arguments
    /// * `bitmap` - The bitmap to select into this DC
    pub fn select_object(&mut self, bitmap: &mut Bitmap) {
        unsafe { wxdragon_sys::wxd_MemoryDC_SelectObject(self.dc_ptr, bitmap.as_const_ptr()) };
    }

    /// Select a bitmap as a source for drawing operations
    ///
    /// # Arguments
    /// * `bitmap` - The bitmap to use as source
    pub fn select_object_as_source(&mut self, bitmap: &Bitmap) {
        unsafe { wxdragon_sys::wxd_MemoryDC_SelectObjectAsSource(self.dc_ptr, bitmap.as_const_ptr()) };
    }
}

impl DeviceContext for MemoryDC {
    fn dc_ptr(&self) -> *mut wxdragon_sys::wxd_DC_t {
        unsafe { wxdragon_sys::wxd_MemoryDC_AsDC(self.dc_ptr) }
    }
}

impl Drop for MemoryDC {
    fn drop(&mut self) {
        unsafe {
            wxdragon_sys::wxd_MemoryDC_Destroy(self.dc_ptr);
        }
    }
}

impl Default for MemoryDC {
    fn default() -> Self {
        Self::new()
    }
}
