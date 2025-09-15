//! Source for drag operations.

use super::{DataObject, DragResult};
use crate::prelude::WxWidget;
use wxdragon_sys as ffi;

/// A drop source is where a drag operation originates from.
///
/// To start a drag operation, create a `DropSource`, set its data using `set_data()`,
/// and call `do_drag_drop()`.
pub struct DropSource {
    ptr: *mut ffi::wxd_DropSource_t,
}

impl DropSource {
    /// Creates a new DropSource associated with the given window.
    pub fn new<W: WxWidget>(window: &W) -> Self {
        let ptr = unsafe { ffi::wxd_DropSource_Create(window.handle_ptr()) };
        DropSource { ptr }
    }

    /// Sets the data object that will be dragged.
    pub fn set_data<D: DataObject>(&self, data: &D) {
        unsafe {
            ffi::wxd_DropSource_SetData(self.ptr, data.as_data_object_ptr());
        }
    }

    /// Starts the drag and drop operation.
    ///
    /// This method doesn't return until the operation is completed, either by
    /// dropping onto a valid target or by cancelling the operation.
    ///
    /// # Parameters
    ///
    /// * `allow_move` - If `true`, the operation can result in moving the data,
    ///   which means that the source may need to delete the data after a successful
    ///   operation. If `false`, the operation can only result in copying the data.
    ///
    /// # Returns
    ///
    /// The result of the drag and drop operation.
    pub fn do_drag_drop(&self, allow_move: bool) -> DragResult {
        let result: ffi::WXDEventTypeCEnum =
            unsafe { ffi::wxd_DropSource_DoDragDrop(self.ptr, allow_move) };
        DragResult::from(result)
    }
}

impl Drop for DropSource {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::wxd_DropSource_Destroy(self.ptr);
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}
