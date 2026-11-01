//! Source for drag operations.

use super::{DataObject, DragResult};
use crate::prelude::WxWidget;
use wxdragon_sys as ffi;

/// A drop source is where a drag operation originates from.
///
/// To start a drag operation, create a `DropSource`, set its data using `set_data()`,
/// and call `do_drag_drop()`.
///
/// ```no_run
/// # use wxdragon::prelude::*;
/// # fn start_drag(panel: &Panel) {
/// let mut source = DropSource::new(panel);
/// source.set_data(TextDataObject::new("Dragged text"));
/// let result = source.do_drag_drop(true);
/// # let _ = result;
/// # }
/// ```
pub struct DropSource {
    ptr: *mut ffi::wxd_DropSource_t,
    // wxDropSource only keeps a pointer to its data object and never frees it,
    // so the data object is kept alive here for as long as the drop source.
    // Fields are dropped after `Drop::drop`, so the source goes first.
    data: Option<Box<dyn DataObject>>,
}

impl DropSource {
    /// Creates a new DropSource associated with the given window.
    pub fn new<W: WxWidget>(window: &W) -> Self {
        let ptr = unsafe { ffi::wxd_DropSource_Create(window.handle_ptr()) };
        DropSource { ptr, data: None }
    }

    /// Sets the data object that will be dragged.
    ///
    /// The drop source takes the data object and keeps it alive until the drop
    /// source is dropped or another data object is set, because wxWidgets only
    /// stores a pointer to it.
    pub fn set_data<D: DataObject + 'static>(&mut self, data: D) {
        let data: Box<dyn DataObject> = Box::new(data);
        unsafe {
            ffi::wxd_DropSource_SetData(self.ptr, data.as_data_object_ptr());
        }
        // Replace the previous data object only after wxDropSource points at
        // the new one, so it never holds a pointer to a freed object.
        self.data = Some(data);
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
        let result: ffi::wxd_DragResult = unsafe { ffi::wxd_DropSource_DoDragDrop(self.ptr, allow_move) };
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
