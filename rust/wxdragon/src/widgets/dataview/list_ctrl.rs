//! DataViewListCtrl implementation.

use crate::event::WxEvtHandler;
use crate::window::{WindowHandle, WxWidget};
use crate::{Id, Point, Size};
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

use super::enums::DataViewColumnFlags;
use super::item::DataViewItem;
use super::variant::Variant;
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
        let column = DataViewColumn::new(label, renderer, model_column, width, align, flags);
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
        let column = DataViewColumn::new(label, renderer, model_column, width, align, flags);
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
        let column = DataViewColumn::new(label, renderer, model_column, width, DataViewAlign::Center, flags);
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

    /// Selects all rows in the control.
    ///
    /// This is only meaningful when the control uses [`DataViewStyle::Multiple`].
    pub fn select_all(&self) {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_DataViewCtrl_SelectAll(ptr) }
    }

    // ==========================================================================
    // Item Count
    // ==========================================================================

    /// Gets the number of items (rows) in the control.
    ///
    /// # Returns
    ///
    /// The number of items in the list control.
    pub fn get_item_count(&self) -> usize {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_DataViewListCtrl_GetItemCount(ptr) as usize }
    }

    // ==========================================================================
    // Adding Items
    // ==========================================================================

    /// Appends a row with the specified values.
    ///
    /// # Parameters
    ///
    /// * `values` - The values for each column in the new row
    ///
    /// # Returns
    ///
    /// `true` if the row was successfully appended, `false` otherwise.
    pub fn append_item(&self, values: &[Variant]) -> bool {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return false;
        }
        let ptrs: Vec<*const ffi::wxd_Variant_t> = values.iter().map(|v| v.as_const_ptr()).collect();
        unsafe { ffi::wxd_DataViewListCtrl_AppendItem(ptr, ptrs.as_ptr(), ptrs.len() as u32, 0) }
    }

    /// Prepends a row with the specified values at the beginning of the list.
    ///
    /// # Parameters
    ///
    /// * `values` - The values for each column in the new row
    ///
    /// # Returns
    ///
    /// `true` if the row was successfully prepended, `false` otherwise.
    pub fn prepend_item(&self, values: &[Variant]) -> bool {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return false;
        }
        let ptrs: Vec<*const ffi::wxd_Variant_t> = values.iter().map(|v| v.as_const_ptr()).collect();
        unsafe { ffi::wxd_DataViewListCtrl_PrependItem(ptr, ptrs.as_ptr(), ptrs.len() as u32, 0) }
    }

    /// Inserts a row at the specified position.
    ///
    /// # Parameters
    ///
    /// * `row` - The position where the row should be inserted
    /// * `values` - The values for each column in the new row
    ///
    /// # Returns
    ///
    /// `true` if the row was successfully inserted, `false` otherwise.
    pub fn insert_item(&self, row: usize, values: &[Variant]) -> bool {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return false;
        }
        let ptrs: Vec<*const ffi::wxd_Variant_t> = values.iter().map(|v| v.as_const_ptr()).collect();
        unsafe { ffi::wxd_DataViewListCtrl_InsertItem(ptr, row as u32, ptrs.as_ptr(), ptrs.len() as u32, 0) }
    }

    // ==========================================================================
    // Removing Items
    // ==========================================================================

    /// Deletes the row at the specified index.
    ///
    /// # Parameters
    ///
    /// * `row` - The index of the row to delete
    ///
    /// # Returns
    ///
    /// `true` if the row was successfully deleted, `false` otherwise.
    pub fn delete_item(&self, row: usize) -> bool {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_DataViewListCtrl_DeleteItem(ptr, row as u32) }
    }

    /// Deletes all items from the list control.
    pub fn delete_all_items(&self) {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_DataViewListCtrl_DeleteAllItems(ptr) }
    }

    // ==========================================================================
    // Get/Set Values
    // ==========================================================================

    /// Sets a value at the specified row and column.
    ///
    /// # Parameters
    ///
    /// * `row` - The row index
    /// * `col` - The column index
    /// * `value` - The value to set
    pub fn set_value<T: Into<Variant>>(&self, row: usize, col: usize, value: T) {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return;
        }
        let v: Variant = value.into();
        unsafe { ffi::wxd_DataViewListCtrl_SetValue(ptr, row as u32, col as u32, v.as_const_ptr()) }
    }

    /// Gets a value from the specified row and column.
    ///
    /// # Parameters
    ///
    /// * `row` - The row index
    /// * `col` - The column index
    ///
    /// # Returns
    ///
    /// An `Option` containing the value, or `None` if the value could not be retrieved.
    pub fn get_value(&self, row: usize, col: usize) -> Option<Variant> {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return None;
        }
        let v_ptr = unsafe { ffi::wxd_DataViewListCtrl_GetValue(ptr, row as u32, col as u32) };
        if v_ptr.is_null() {
            None
        } else {
            Some(unsafe { Variant::from_raw(v_ptr) })
        }
    }

    // ==========================================================================
    // Text Convenience Methods
    // ==========================================================================

    /// Sets a text value at the specified row and column.
    ///
    /// This is a convenience method for setting string values.
    ///
    /// # Parameters
    ///
    /// * `row` - The row index
    /// * `col` - The column index
    /// * `value` - The text value to set
    pub fn set_text_value(&self, row: usize, col: usize, value: &str) {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return;
        }
        let c_str = CString::new(value).unwrap_or_default();
        unsafe { ffi::wxd_DataViewListCtrl_SetTextValue(ptr, row as u32, col as u32, c_str.as_ptr()) }
    }

    /// Gets a text value from the specified row and column.
    ///
    /// This is a convenience method for getting string values.
    ///
    /// # Parameters
    ///
    /// * `row` - The row index
    /// * `col` - The column index
    ///
    /// # Returns
    ///
    /// The text value at the specified position.
    pub fn get_text_value(&self, row: usize, col: usize) -> String {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_DataViewListCtrl_GetTextValue(ptr, row as u32, col as u32, std::ptr::null_mut(), 0) };
        if len < 0 {
            return String::new();
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_DataViewListCtrl_GetTextValue(ptr, row as u32, col as u32, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() }
    }

    // ==========================================================================
    // Toggle Convenience Methods
    // ==========================================================================

    /// Sets a toggle (boolean) value at the specified row and column.
    ///
    /// This is a convenience method for setting boolean values in toggle columns.
    ///
    /// # Parameters
    ///
    /// * `row` - The row index
    /// * `col` - The column index
    /// * `value` - The boolean value to set
    pub fn set_toggle_value(&self, row: usize, col: usize, value: bool) {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_DataViewListCtrl_SetToggleValue(ptr, row as u32, col as u32, value) }
    }

    /// Gets a toggle (boolean) value from the specified row and column.
    ///
    /// This is a convenience method for getting boolean values from toggle columns.
    ///
    /// # Parameters
    ///
    /// * `row` - The row index
    /// * `col` - The column index
    ///
    /// # Returns
    ///
    /// The boolean value at the specified position.
    pub fn get_toggle_value(&self, row: usize, col: usize) -> bool {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_DataViewListCtrl_GetToggleValue(ptr, row as u32, col as u32) }
    }

    // ==========================================================================
    // Row/Item Conversion
    // ==========================================================================

    /// Converts a DataViewItem to a row index.
    ///
    /// # Parameters
    ///
    /// * `item` - The DataViewItem to convert
    ///
    /// # Returns
    ///
    /// An `Option` containing the row index, or `None` if the item is invalid.
    pub fn item_to_row(&self, item: &DataViewItem) -> Option<usize> {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return None;
        }
        let row = unsafe { ffi::wxd_DataViewListCtrl_ItemToRow(ptr, **item) };
        if row >= 0 { Some(row as usize) } else { None }
    }

    /// Converts a row index to a DataViewItem.
    ///
    /// # Parameters
    ///
    /// * `row` - The row index to convert
    ///
    /// # Returns
    ///
    /// An `Option` containing the DataViewItem, or `None` if the row is invalid.
    pub fn row_to_item(&self, row: usize) -> Option<DataViewItem> {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return None;
        }
        let item_ptr = unsafe { ffi::wxd_DataViewListCtrl_RowToItem(ptr, row as i32) };
        if item_ptr.is_null() {
            None
        } else {
            Some(DataViewItem::from(item_ptr as *const _))
        }
    }

    /// Sets the item that has keyboard focus.
    ///
    /// The item can be obtained from a row index with [`Self::row_to_item`].
    pub fn set_current_item(&self, item: &DataViewItem) {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_DataViewCtrl_SetCurrentItem(ptr, **item) }
    }

    /// Ensures that the given item is visible, scrolling the control if necessary.
    ///
    /// The item can be obtained from a row index with [`Self::row_to_item`].
    pub fn ensure_visible(&self, item: &DataViewItem) {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_DataViewCtrl_EnsureVisible(ptr, **item) }
    }

    // ==========================================================================
    // Additional Selection Methods
    // ==========================================================================

    /// Unselects the specified row.
    ///
    /// # Parameters
    ///
    /// * `row` - The index of the row to unselect
    pub fn unselect_row(&self, row: usize) {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_DataViewListCtrl_UnselectRow(ptr, row as u32) }
    }

    /// Checks if a row is selected.
    ///
    /// # Parameters
    ///
    /// * `row` - The index of the row to check
    ///
    /// # Returns
    ///
    /// `true` if the row is selected, `false` otherwise.
    pub fn is_row_selected(&self, row: usize) -> bool {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_DataViewListCtrl_IsRowSelected(ptr, row as u32) }
    }

    // ==========================================================================
    // Item Data
    // ==========================================================================

    /// Sets custom data associated with an item.
    ///
    /// # Parameters
    ///
    /// * `item` - The DataViewItem to set data for
    /// * `data` - The custom data pointer
    pub fn set_item_data(&self, item: &DataViewItem, data: usize) {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_DataViewListCtrl_SetItemData(ptr, **item, data) }
    }

    /// Gets custom data associated with an item.
    ///
    /// # Parameters
    ///
    /// * `item` - The DataViewItem to get data from
    ///
    /// # Returns
    ///
    /// The custom data pointer associated with the item.
    pub fn get_item_data(&self, item: &DataViewItem) -> usize {
        let ptr = self.dvlc_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_DataViewListCtrl_GetItemData(ptr, **item) }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use crate::widgets::Frame;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const NOT_RUN: u8 = 0;
    const STARTED: u8 = 1;
    const FINISHED: u8 = 2;

    /// Runs `body` against a one-column list control inside a wx main loop that
    /// a one-shot timer exits again (see the `window.rs` and `combobox.rs`
    /// tests). Panics in the init callback are swallowed by `crate::main`, so
    /// the body's progress is tracked: started-but-unfinished fails the test,
    /// never-started (wx could not initialise) only logs. Skipped on macOS,
    /// where the test thread is not the OS main thread.
    fn with_list_ctrl(body: impl FnOnce(&DataViewListCtrl) + 'static) {
        let _gui_test = crate::app::GUI_TEST_LOCK.lock().unwrap();
        SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
        let progress = Rc::new(Cell::new(NOT_RUN));
        let progress_in_loop = progress.clone();
        let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
        let timer_store_clone = timer_store.clone();

        let res = crate::main(move |app| {
            let frame = Frame::builder().with_title("dataview list test").build();
            let list = DataViewListCtrl::builder(&frame).build();
            list.append_text_column("text", 0, DataViewAlign::Left, 200, DataViewColumnFlags::Resizable);
            list.append_item(&[Variant::from_string("")]);

            progress_in_loop.set(STARTED);
            body(&list);
            progress_in_loop.set(FINISHED);

            let timer = Timer::new(&frame);
            let app_clone = app;
            let timer_store_cleanup = timer_store_clone.clone();
            timer.on_tick(move |_evt| {
                timer_store_cleanup.borrow_mut().take();
                app_clone.exit_main_loop();
            });
            timer.start(100, true);
            timer_store_clone.borrow_mut().replace(timer);
        });
        if let Err(e) = res {
            log::warn!("Test failed with error: {e:?}");
        }
        assert_ne!(progress.get(), STARTED, "test body panicked inside the wx main loop");
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn text_values_round_trip() {
        with_list_ctrl(|list| {
            list.set_text_value(0, 0, "hello");
            assert_eq!(list.get_text_value(0, 0), "hello");

            // Multi-byte, so a byte length and a char count disagree.
            list.set_text_value(0, 0, "héllo wörld — ünïcøde");
            assert_eq!(list.get_text_value(0, 0), "héllo wörld — ünïcøde");

            list.set_text_value(0, 0, "");
            assert_eq!(list.get_text_value(0, 0), "");
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn a_long_text_value_is_not_truncated() {
        with_list_ctrl(|list| {
            // Longer than any fixed stack buffer a getter might be tempted to
            // use, and long enough that the UTF-8 conversion allocates.
            let long = "wxDragon ".repeat(500);
            list.set_text_value(0, 0, &long);
            assert_eq!(list.get_text_value(0, 0), long);
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn get_text_value_on_a_destroyed_control_is_empty() {
        with_list_ctrl(|list| {
            list.destroy();
            assert_eq!(list.get_text_value(0, 0), "");
        });
    }
}
