//! DataViewListCtrl implementation.

use crate::event::WxEvtHandler;
use crate::window::{WindowHandle, WxWidget};
use crate::{Id, Point, Size};
use wxdragon_sys as ffi;

use super::enums::DataViewColumnFlags;
use super::{
    DataViewAlign, DataViewCellMode, DataViewColumn, DataViewProgressRenderer, DataViewStyle, DataViewTextRenderer,
    DataViewToggleRenderer, VariantType,
};

/// A simplified DataViewCtrl that displays data in a list format.
///
/// DataViewListCtrl is a convenience wrapper around DataViewCtrl that simplifies
/// the display of tabular data without requiring a custom model.
///
/// DataViewListCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct DataViewListCtrl {
    /// Safe handle to the underlying wxDataViewListCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl DataViewListCtrl {
    /// Creates a builder for configuring and constructing a DataViewListCtrl.
    pub fn builder(parent: &dyn WxWidget) -> DataViewListCtrlBuilder<'_> {
        DataViewListCtrlBuilder::new(parent)
    }

    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: i32, pos: Point, size: Size, style: i64) -> Self {
        let ptr = unsafe {
            ffi::wxd_DataViewListCtrl_Create(
                parent_ptr,
                id as i64,
                &pos as *const Point as *const ffi::wxd_Point,
                &size as *const Size as *const ffi::wxd_Size,
                style,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create DataViewListCtrl widget");
        }

        DataViewListCtrl {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Helper to get raw window pointer, returns null if widget has been destroyed
    #[inline]
    fn dvlc_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    /// Returns the underlying WindowHandle for this DataViewListCtrl.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    /// Appends a text column to this list control.
    ///
    /// # Parameters
    ///
    /// * `label` - The header label for the column
    /// * `model_column` - The column index in the data model
    /// * `align` - The text alignment
    /// * `width` - The column width (in pixels)
    /// * `flags` - Column flags (e.g., resizable, sortable)
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_text_column(
        &self,
        label: &str,
        model_column: usize,
        align: DataViewAlign,
        width: i32,
        flags: DataViewColumnFlags,
    ) -> bool {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return false;
        }
        let renderer = DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, align);
        let column = DataViewColumn::new(label, &renderer, model_column, width, align, flags);
        unsafe { ffi::wxd_DataViewCtrl_AppendColumn(ptr, column.as_raw()) }
    }

    /// Appends a toggle column to this list control.
    ///
    /// # Parameters
    ///
    /// * `label` - The header label for the column
    /// * `model_column` - The column index in the data model
    /// * `align` - The alignment of the checkbox
    /// * `width` - The column width (in pixels)
    /// * `flags` - Column flags
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_toggle_column(
        &self,
        label: &str,
        model_column: usize,
        align: DataViewAlign,
        width: i32,
        flags: DataViewColumnFlags,
    ) -> bool {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return false;
        }
        let renderer = DataViewToggleRenderer::new(VariantType::Bool, DataViewCellMode::Activatable, align);
        let column = DataViewColumn::new(label, &renderer, model_column, width, align, flags);
        unsafe { ffi::wxd_DataViewCtrl_AppendColumn(ptr, column.as_raw()) }
    }

    /// Appends a progress column to this list control.
    ///
    /// # Parameters
    ///
    /// * `label` - The header label for the column
    /// * `model_column` - The column index in the data model
    /// * `width` - The column width (in pixels)
    /// * `flags` - Column flags
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_progress_column(&self, label: &str, model_column: usize, width: i32, flags: DataViewColumnFlags) -> bool {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return false;
        }
        let renderer = DataViewProgressRenderer::new(VariantType::Int32, DataViewCellMode::Inert);
        let column = DataViewColumn::new(label, &renderer, model_column, width, DataViewAlign::Center, flags);
        unsafe { ffi::wxd_DataViewCtrl_AppendColumn(ptr, column.as_raw()) }
    }

    /// Selects the specified row.
    ///
    /// # Parameters
    ///
    /// * `row` - The row index to select
    ///
    /// # Returns
    ///
    /// `true` if the row was successfully selected, `false` otherwise.
    pub fn select_row(&self, row: usize) -> bool {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_DataViewCtrl_SelectRow(ptr, row as i64) }
    }

    /// Gets the currently selected row.
    ///
    /// # Returns
    ///
    /// An `Option` containing the index of the selected row, or `None` if no row is selected.
    pub fn get_selected_row(&self) -> Option<usize> {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return None;
        }
        let row = unsafe { ffi::wxd_DataViewCtrl_GetSelectedRow(ptr) };
        if row >= 0 { Some(row as usize) } else { None }
    }

    /// Deselects all currently selected items.
    pub fn unselect_all(&self) {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_DataViewCtrl_UnselectAll(ptr) }
    }
}

// Manual WxWidget implementation for DataViewListCtrl (using WindowHandle)
impl WxWidget for DataViewListCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for DataViewListCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for DataViewListCtrl {}

widget_builder!(
    name: DataViewListCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: DataViewStyle,
    fields: {},
    build_impl: |slf| {
        DataViewListCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
);

// Implement DataViewEventHandler for DataViewListCtrl
impl crate::widgets::dataview::DataViewEventHandler for DataViewListCtrl {}
