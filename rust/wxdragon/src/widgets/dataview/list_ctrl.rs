//! DataViewListCtrl implementation.

use crate::{implement_widget_traits_with_target, widget_builder};
use crate::{Id, Point, Size, Window, WxWidget};
use wxdragon_sys as ffi;

use super::enums::DataViewColumnFlags;
use super::{
    DataViewAlign, DataViewCellMode, DataViewColumn, DataViewProgressRenderer, DataViewStyle,
    DataViewTextRenderer, DataViewToggleRenderer, VariantType,
};

/// A simplified DataViewCtrl that displays data in a list format.
///
/// DataViewListCtrl is a convenience wrapper around DataViewCtrl that simplifies
/// the display of tabular data without requiring a custom model.
pub struct DataViewListCtrl {
    window: Window,
}

impl DataViewListCtrl {
    /// Creates a builder for configuring and constructing a DataViewListCtrl.
    pub fn builder(parent: &dyn WxWidget) -> DataViewListCtrlBuilder<'_> {
        DataViewListCtrlBuilder::new(parent)
    }

    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: i32,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        let handle = unsafe {
            ffi::wxd_DataViewListCtrl_Create(
                parent_ptr,
                id as i64,
                &pos as *const Point as *const ffi::wxd_Point,
                &size as *const Size as *const ffi::wxd_Size,
                style,
            )
        };

        let window = unsafe { Window::from_ptr(handle) };
        Self { window }
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
        let renderer =
            DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, align);
        let column = DataViewColumn::new(label, &renderer, model_column, width, align, flags);

        let ctrl_ptr = self.window.handle_ptr();
        unsafe { ffi::wxd_DataViewCtrl_AppendColumn(ctrl_ptr, column.as_raw()) }
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
        let renderer =
            DataViewToggleRenderer::new(VariantType::Bool, DataViewCellMode::Activatable, align);
        let column = DataViewColumn::new(label, &renderer, model_column, width, align, flags);

        let ctrl_ptr = self.window.handle_ptr();
        unsafe { ffi::wxd_DataViewCtrl_AppendColumn(ctrl_ptr, column.as_raw()) }
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
    pub fn append_progress_column(
        &self,
        label: &str,
        model_column: usize,
        width: i32,
        flags: DataViewColumnFlags,
    ) -> bool {
        let renderer = DataViewProgressRenderer::new(VariantType::Int32, DataViewCellMode::Inert);
        let column = DataViewColumn::new(
            label,
            &renderer,
            model_column,
            width,
            DataViewAlign::Center,
            flags,
        );

        let ctrl_ptr = self.window.handle_ptr();
        unsafe { ffi::wxd_DataViewCtrl_AppendColumn(ctrl_ptr, column.as_raw()) }
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
        unsafe { ffi::wxd_DataViewCtrl_SelectRow(self.window.handle_ptr(), row as i64) }
    }

    /// Gets the currently selected row.
    ///
    /// # Returns
    ///
    /// An `Option` containing the index of the selected row, or `None` if no row is selected.
    pub fn get_selected_row(&self) -> Option<usize> {
        let row = unsafe { ffi::wxd_DataViewCtrl_GetSelectedRow(self.window.handle_ptr()) };
        if row >= 0 {
            Some(row as usize)
        } else {
            None
        }
    }

    /// Deselects all currently selected items.
    pub fn unselect_all(&self) {
        unsafe { ffi::wxd_DataViewCtrl_UnselectAll(self.window.handle_ptr()) }
    }
}

implement_widget_traits_with_target!(DataViewListCtrl, window, Window);

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
