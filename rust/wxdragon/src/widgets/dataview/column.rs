//! DataViewColumn implementation.

use super::enums::{DataViewAlign, DataViewColumnFlags};
use super::renderer::DataViewRenderer;
use std::ffi::CString;
use wxdragon_sys as ffi;

/// A column in a DataViewCtrl.
///
/// DataViewColumn associates a renderer with a model column and handles the display
/// of data in a specific column of the control.
pub struct DataViewColumn {
    handle: *mut ffi::wxd_DataViewColumn_t,
}

impl DataViewColumn {
    /// Creates a new column for a DataViewCtrl.
    ///
    /// # Parameters
    ///
    /// * `title` - The header text for this column
    /// * `renderer` - The renderer that will be used to display data in this
    ///   column. The column takes ownership of it, which is why this is taken
    ///   by value: `wxDataViewColumn` deletes its renderer, so one renderer
    ///   cannot be shared between two columns.
    /// * `model_column` - The column index in the data model
    /// * `width` - The column width (in pixels)
    /// * `align` - The alignment of the column content
    /// * `flags` - Column flags specifying behavior (e.g., resizable, sortable)
    pub fn new(
        title: &str,
        renderer: impl DataViewRenderer,
        model_column: usize,
        width: i32,
        align: DataViewAlign,
        flags: DataViewColumnFlags,
    ) -> Self {
        let title_cstr = CString::new(title).unwrap_or_default();
        let handle = unsafe {
            // FFI function now takes 6 arguments, including flags as int.
            ffi::wxd_DataViewColumn_Create(
                title_cstr.as_ptr(),
                renderer.as_raw(),
                model_column as i32, // FFI expects int
                width,
                align.bits() as i32, // FFI expects int (align.bits() is i64)
                flags.bits() as i32, // FFI expects int (flags.bits() is i64)
            )
        };
        Self { handle }
    }

    /// Creates a DataViewColumn from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid pointer to a wxDataViewColumn.
    pub unsafe fn from_ptr(ptr: *mut ffi::wxd_DataViewColumn_t) -> Self {
        Self { handle: ptr }
    }

    /// Gets the raw pointer to the native wxDataViewColumn.
    pub fn as_raw(&self) -> *mut ffi::wxd_DataViewColumn_t {
        self.handle
    }

    /// Sets the title of the column header.
    pub fn set_title(&self, title: &str) {
        let title_cstr = CString::new(title).unwrap_or_default();
        unsafe {
            ffi::wxd_DataViewColumn_SetTitle(self.handle, title_cstr.as_ptr());
        }
    }

    /// Sets whether the column can be resized by the user.
    pub fn set_resizeable(&self, resizeable: bool) {
        unsafe {
            ffi::wxd_DataViewColumn_SetResizeable(self.handle, resizeable);
        }
    }

    /// Checks if the column can be resized by the user.
    pub fn is_resizeable(&self) -> bool {
        unsafe { ffi::wxd_DataViewColumn_IsResizeable(self.handle) }
    }

    /// Sets whether the column can be sorted by clicking its header.
    pub fn set_sortable(&self, sortable: bool) {
        unsafe {
            ffi::wxd_DataViewColumn_SetSortable(self.handle, sortable);
        }
    }

    /// Checks if the column can be sorted by clicking its header.
    pub fn is_sortable(&self) -> bool {
        unsafe { ffi::wxd_DataViewColumn_IsSortable(self.handle) }
    }
}
