//! wxGrid wrapper - a powerful spreadsheet-like grid control

use crate::color::Colour;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;


// --- Grid Selection Modes ---

/// Selection modes for Grid
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum GridSelectionMode {
    /// Allow selecting individual cells (default)
    #[default]
    Cells = 0,
    /// Allow selecting only entire rows
    Rows = 1,
    /// Allow selecting only entire columns
    Columns = 2,
    /// Allow selecting rows or columns
    RowsOrColumns = 3,
    /// Disallow selecting anything
    None = 4,
}

// --- Grid Style ---

widget_style_enum!(
    name: GridStyle,
    doc: "Style flags for Grid widget.",
    variants: {
        Default: 0, "Default grid style."
    },
    default_variant: Default
);

// --- Grid Events ---

/// Events emitted by Grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridEvent {
    /// Cell was left-clicked
    CellLeftClick,
    /// Cell was right-clicked
    CellRightClick,
    /// Cell was double-left-clicked
    CellLeftDClick,
    /// Cell was double-right-clicked
    CellRightDClick,
    /// Label was left-clicked
    LabelLeftClick,
    /// Label was right-clicked
    LabelRightClick,
    /// Label was double-left-clicked
    LabelLeftDClick,
    /// Label was double-right-clicked
    LabelRightDClick,
    /// Cell value was changed
    CellChanged,
    /// A cell was selected
    SelectCell,
    /// Cell editor was shown
    EditorShown,
    /// Cell editor was hidden
    EditorHidden,
    /// Cell editor was created
    EditorCreated,
    /// Cell drag started
    CellBeginDrag,
    /// Row was resized
    RowSize,
    /// Column was resized
    ColSize,
    /// Range was selected
    RangeSelected,
}

/// Event data for Grid events
#[derive(Debug)]
pub struct GridEventData {
    event: Event,
}

impl GridEventData {
    /// Create a new GridEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the row of the cell that triggered the event
    pub fn get_row(&self) -> i32 {
        if self.event.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_GridEvent_GetRow(self.event.0) }
    }

    /// Get the column of the cell that triggered the event
    pub fn get_col(&self) -> i32 {
        if self.event.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_GridEvent_GetCol(self.event.0) }
    }

    /// Get the position where the event occurred
    pub fn get_position(&self) -> Point {
        if self.event.is_null() {
            return Point::new(0, 0);
        }
        let pos = unsafe { ffi::wxd_GridEvent_GetPosition(self.event.0) };
        Point::new(pos.x, pos.y)
    }

    /// Returns true if the user is selecting cells
    pub fn selecting(&self) -> bool {
        if self.event.is_null() {
            return false;
        }
        unsafe { ffi::wxd_GridEvent_Selecting(self.event.0) }
    }

    /// Returns true if Ctrl key was pressed during the event
    pub fn control_down(&self) -> bool {
        if self.event.is_null() {
            return false;
        }
        unsafe { ffi::wxd_GridEvent_ControlDown(self.event.0) }
    }

    /// Returns true if Shift key was pressed during the event
    pub fn shift_down(&self) -> bool {
        if self.event.is_null() {
            return false;
        }
        unsafe { ffi::wxd_GridEvent_ShiftDown(self.event.0) }
    }

    /// Returns true if Alt key was pressed during the event
    pub fn alt_down(&self) -> bool {
        if self.event.is_null() {
            return false;
        }
        unsafe { ffi::wxd_GridEvent_AltDown(self.event.0) }
    }

    /// Returns true if Meta/Cmd key was pressed during the event
    pub fn meta_down(&self) -> bool {
        if self.event.is_null() {
            return false;
        }
        unsafe { ffi::wxd_GridEvent_MetaDown(self.event.0) }
    }
}

/// A powerful spreadsheet-like grid control
///
/// Grid uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed, the handle becomes invalid
/// and all operations become safe no-ops.
///
/// # Example
/// ```ignore
/// let grid = Grid::builder(&frame)
///     .with_size(Size::new(400, 300))
///     .build();
///
/// grid.create_grid(10, 5, GridSelectionMode::Cells);
/// grid.set_col_label_value(0, "Name");
/// grid.set_col_label_value(1, "Value");
/// grid.set_cell_value(0, 0, "Hello");
/// grid.set_cell_value(0, 1, "World");
/// ```
#[derive(Clone, Copy)]
pub struct Grid {
    handle: WindowHandle,
}

impl Grid {
    /// Creates a new Grid builder.
    pub fn builder(parent: &dyn WxWidget) -> GridBuilder<'_> {
        GridBuilder::new(parent)
    }

    /// Internal implementation used by the builder.
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, pos: Point, size: Size, style: i64) -> Self {
        assert!(!parent_ptr.is_null(), "Grid requires a parent");

        let ptr = unsafe {
            ffi::wxd_Grid_Create(parent_ptr, id, pos.into(), size.into(), style)
        };

        if ptr.is_null() {
            panic!("Failed to create Grid: FFI returned null pointer.");
        }

        Grid {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw grid pointer
    #[inline]
    fn grid_ptr(&self) -> *mut ffi::wxd_Grid_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_Grid_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Returns the underlying WindowHandle for this grid control.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    // --- Grid Initialization ---

    /// Creates the grid with the specified number of rows and columns.
    /// Must be called after construction before the grid can be used.
    pub fn create_grid(&self, num_rows: i32, num_cols: i32, selection_mode: GridSelectionMode) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_CreateGrid(ptr, num_rows, num_cols, selection_mode as i32) }
    }

    // --- Grid Dimensions ---

    /// Gets the number of rows in the grid.
    pub fn get_number_rows(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetNumberRows(ptr) }
    }

    /// Gets the number of columns in the grid.
    pub fn get_number_cols(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetNumberCols(ptr) }
    }

    // --- Row and Column Management ---

    /// Inserts rows at the specified position.
    pub fn insert_rows(&self, pos: i32, num_rows: i32, update_labels: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_InsertRows(ptr, pos, num_rows, update_labels) }
    }

    /// Appends rows to the end of the grid.
    pub fn append_rows(&self, num_rows: i32, update_labels: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_AppendRows(ptr, num_rows, update_labels) }
    }

    /// Deletes rows starting at the specified position.
    pub fn delete_rows(&self, pos: i32, num_rows: i32, update_labels: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_DeleteRows(ptr, pos, num_rows, update_labels) }
    }

    /// Inserts columns at the specified position.
    pub fn insert_cols(&self, pos: i32, num_cols: i32, update_labels: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_InsertCols(ptr, pos, num_cols, update_labels) }
    }

    /// Appends columns to the end of the grid.
    pub fn append_cols(&self, num_cols: i32, update_labels: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_AppendCols(ptr, num_cols, update_labels) }
    }

    /// Deletes columns starting at the specified position.
    pub fn delete_cols(&self, pos: i32, num_cols: i32, update_labels: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_DeleteCols(ptr, pos, num_cols, update_labels) }
    }

    // --- Cell Value Accessors ---

    /// Gets the value of a cell.
    pub fn get_cell_value(&self, row: i32, col: i32) -> String {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let len = ffi::wxd_Grid_GetCellValue(ptr, row, col, std::ptr::null_mut(), 0);
            if len <= 0 {
                return String::new();
            }
            let mut buffer = vec![0u8; len as usize + 1];
            ffi::wxd_Grid_GetCellValue(ptr, row, col, buffer.as_mut_ptr() as *mut i8, buffer.len() as i32);
            CStr::from_ptr(buffer.as_ptr() as *const i8)
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Sets the value of a cell.
    pub fn set_cell_value(&self, row: i32, col: i32, value: &str) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        let c_value = CString::new(value).unwrap_or_default();
        unsafe { ffi::wxd_Grid_SetCellValue(ptr, row, col, c_value.as_ptr()) }
    }

    // --- Label Functions ---

    /// Gets the row label value.
    pub fn get_row_label_value(&self, row: i32) -> String {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let len = ffi::wxd_Grid_GetRowLabelValue(ptr, row, std::ptr::null_mut(), 0);
            if len <= 0 {
                return String::new();
            }
            let mut buffer = vec![0u8; len as usize + 1];
            ffi::wxd_Grid_GetRowLabelValue(ptr, row, buffer.as_mut_ptr() as *mut i8, buffer.len() as i32);
            CStr::from_ptr(buffer.as_ptr() as *const i8)
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Sets the row label value.
    pub fn set_row_label_value(&self, row: i32, value: &str) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        let c_value = CString::new(value).unwrap_or_default();
        unsafe { ffi::wxd_Grid_SetRowLabelValue(ptr, row, c_value.as_ptr()) }
    }

    /// Gets the column label value.
    pub fn get_col_label_value(&self, col: i32) -> String {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let len = ffi::wxd_Grid_GetColLabelValue(ptr, col, std::ptr::null_mut(), 0);
            if len <= 0 {
                return String::new();
            }
            let mut buffer = vec![0u8; len as usize + 1];
            ffi::wxd_Grid_GetColLabelValue(ptr, col, buffer.as_mut_ptr() as *mut i8, buffer.len() as i32);
            CStr::from_ptr(buffer.as_ptr() as *const i8)
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Sets the column label value.
    pub fn set_col_label_value(&self, col: i32, value: &str) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        let c_value = CString::new(value).unwrap_or_default();
        unsafe { ffi::wxd_Grid_SetColLabelValue(ptr, col, c_value.as_ptr()) }
    }

    /// Gets the row label size (width).
    pub fn get_row_label_size(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetRowLabelSize(ptr) }
    }

    /// Sets the row label size (width).
    pub fn set_row_label_size(&self, width: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetRowLabelSize(ptr, width) }
    }

    /// Gets the column label size (height).
    pub fn get_col_label_size(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetColLabelSize(ptr) }
    }

    /// Sets the column label size (height).
    pub fn set_col_label_size(&self, height: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetColLabelSize(ptr, height) }
    }

    /// Hides the row labels.
    pub fn hide_row_labels(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_HideRowLabels(ptr) }
    }

    /// Hides the column labels.
    pub fn hide_col_labels(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_HideColLabels(ptr) }
    }

    // --- Row and Column Sizes ---

    /// Gets the default row size.
    pub fn get_default_row_size(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetDefaultRowSize(ptr) }
    }

    /// Gets the size of a specific row.
    pub fn get_row_size(&self, row: i32) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetRowSize(ptr, row) }
    }

    /// Sets the default row size.
    pub fn set_default_row_size(&self, height: i32, resize_existing: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetDefaultRowSize(ptr, height, resize_existing) }
    }

    /// Sets the size of a specific row.
    pub fn set_row_size(&self, row: i32, height: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetRowSize(ptr, row, height) }
    }

    /// Gets the default column size.
    pub fn get_default_col_size(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetDefaultColSize(ptr) }
    }

    /// Gets the size of a specific column.
    pub fn get_col_size(&self, col: i32) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetColSize(ptr, col) }
    }

    /// Sets the default column size.
    pub fn set_default_col_size(&self, width: i32, resize_existing: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetDefaultColSize(ptr, width, resize_existing) }
    }

    /// Sets the size of a specific column.
    pub fn set_col_size(&self, col: i32, width: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetColSize(ptr, col, width) }
    }

    /// Auto-sizes a column to fit its contents.
    pub fn auto_size_column(&self, col: i32, set_as_min: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_AutoSizeColumn(ptr, col, set_as_min) }
    }

    /// Auto-sizes a row to fit its contents.
    pub fn auto_size_row(&self, row: i32, set_as_min: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_AutoSizeRow(ptr, row, set_as_min) }
    }

    /// Auto-sizes all columns to fit their contents.
    pub fn auto_size_columns(&self, set_as_min: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_AutoSizeColumns(ptr, set_as_min) }
    }

    /// Auto-sizes all rows to fit their contents.
    pub fn auto_size_rows(&self, set_as_min: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_AutoSizeRows(ptr, set_as_min) }
    }

    /// Auto-sizes the grid to fit its contents.
    pub fn auto_size(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_AutoSize(ptr) }
    }

    // --- Cell Formatting ---

    /// Gets the background colour of a cell.
    pub fn get_cell_background_colour(&self, row: i32, col: i32) -> Colour {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Colour::new(255, 255, 255, 255);
        }
        unsafe {
            let c = ffi::wxd_Grid_GetCellBackgroundColour(ptr, row, col);
            Colour::new(c.r, c.g, c.b, c.a)
        }
    }

    /// Sets the background colour of a cell.
    pub fn set_cell_background_colour(&self, row: i32, col: i32, colour: &Colour) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetCellBackgroundColour(ptr, row, col, (*colour).into()) }
    }

    /// Gets the text colour of a cell.
    pub fn get_cell_text_colour(&self, row: i32, col: i32) -> Colour {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Colour::new(0, 0, 0, 255);
        }
        unsafe {
            let c = ffi::wxd_Grid_GetCellTextColour(ptr, row, col);
            Colour::new(c.r, c.g, c.b, c.a)
        }
    }

    /// Sets the text colour of a cell.
    pub fn set_cell_text_colour(&self, row: i32, col: i32, colour: &Colour) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetCellTextColour(ptr, row, col, (*colour).into()) }
    }

    /// Gets the alignment of a cell. Returns (horizontal, vertical).
    pub fn get_cell_alignment(&self, row: i32, col: i32) -> (i32, i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return (0, 0);
        }
        let mut horiz = 0;
        let mut vert = 0;
        unsafe { ffi::wxd_Grid_GetCellAlignment(ptr, row, col, &mut horiz, &mut vert) }
        (horiz, vert)
    }

    /// Sets the alignment of a cell.
    pub fn set_cell_alignment(&self, row: i32, col: i32, horiz: i32, vert: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetCellAlignment(ptr, row, col, horiz, vert) }
    }

    // --- Default Cell Formatting ---

    /// Gets the default cell background colour.
    pub fn get_default_cell_background_colour(&self) -> Colour {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Colour::new(255, 255, 255, 255);
        }
        unsafe {
            let c = ffi::wxd_Grid_GetDefaultCellBackgroundColour(ptr);
            Colour::new(c.r, c.g, c.b, c.a)
        }
    }

    /// Sets the default cell background colour.
    pub fn set_default_cell_background_colour(&self, colour: &Colour) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetDefaultCellBackgroundColour(ptr, (*colour).into()) }
    }

    /// Gets the default cell text colour.
    pub fn get_default_cell_text_colour(&self) -> Colour {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Colour::new(0, 0, 0, 255);
        }
        unsafe {
            let c = ffi::wxd_Grid_GetDefaultCellTextColour(ptr);
            Colour::new(c.r, c.g, c.b, c.a)
        }
    }

    /// Sets the default cell text colour.
    pub fn set_default_cell_text_colour(&self, colour: &Colour) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetDefaultCellTextColour(ptr, (*colour).into()) }
    }

    /// Gets the default cell alignment. Returns (horizontal, vertical).
    pub fn get_default_cell_alignment(&self) -> (i32, i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return (0, 0);
        }
        let mut horiz = 0;
        let mut vert = 0;
        unsafe { ffi::wxd_Grid_GetDefaultCellAlignment(ptr, &mut horiz, &mut vert) }
        (horiz, vert)
    }

    /// Sets the default cell alignment.
    pub fn set_default_cell_alignment(&self, horiz: i32, vert: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetDefaultCellAlignment(ptr, horiz, vert) }
    }

    // --- Read-Only Cells ---

    /// Returns true if the cell is read-only.
    pub fn is_read_only(&self, row: i32, col: i32) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_IsReadOnly(ptr, row, col) }
    }

    /// Sets whether a cell is read-only.
    pub fn set_read_only(&self, row: i32, col: i32, is_read_only: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetReadOnly(ptr, row, col, is_read_only) }
    }

    // --- Selection ---

    /// Selects a row.
    pub fn select_row(&self, row: i32, add_to_selected: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SelectRow(ptr, row, add_to_selected) }
    }

    /// Selects a column.
    pub fn select_col(&self, col: i32, add_to_selected: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SelectCol(ptr, col, add_to_selected) }
    }

    /// Selects a block of cells.
    pub fn select_block(&self, top_row: i32, left_col: i32, bottom_row: i32, right_col: i32, add_to_selected: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SelectBlock(ptr, top_row, left_col, bottom_row, right_col, add_to_selected) }
    }

    /// Selects all cells in the grid.
    pub fn select_all(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SelectAll(ptr) }
    }

    /// Returns true if there is a selection.
    pub fn is_selection(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_IsSelection(ptr) }
    }

    /// Deselects a row.
    pub fn deselect_row(&self, row: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_DeselectRow(ptr, row) }
    }

    /// Deselects a column.
    pub fn deselect_col(&self, col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_DeselectCol(ptr, col) }
    }

    /// Deselects a cell.
    pub fn deselect_cell(&self, row: i32, col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_DeselectCell(ptr, row, col) }
    }

    /// Clears the selection.
    pub fn clear_selection(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_ClearSelection(ptr) }
    }

    /// Returns true if the cell is in the selection.
    pub fn is_in_selection(&self, row: i32, col: i32) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_IsInSelection(ptr, row, col) }
    }

    /// Gets the selected rows.
    pub fn get_selected_rows(&self) -> Vec<i32> {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Vec::new();
        }
        unsafe {
            let count = ffi::wxd_Grid_GetSelectedRows(ptr, std::ptr::null_mut(), 0);
            if count <= 0 {
                return Vec::new();
            }
            let mut buffer = vec![0i32; count as usize];
            ffi::wxd_Grid_GetSelectedRows(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            buffer
        }
    }

    /// Gets the selected columns.
    pub fn get_selected_cols(&self) -> Vec<i32> {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Vec::new();
        }
        unsafe {
            let count = ffi::wxd_Grid_GetSelectedCols(ptr, std::ptr::null_mut(), 0);
            if count <= 0 {
                return Vec::new();
            }
            let mut buffer = vec![0i32; count as usize];
            ffi::wxd_Grid_GetSelectedCols(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            buffer
        }
    }

    // --- Grid Cursor ---

    /// Gets the current cursor row.
    pub fn get_grid_cursor_row(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_GetGridCursorRow(ptr) }
    }

    /// Gets the current cursor column.
    pub fn get_grid_cursor_col(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_GetGridCursorCol(ptr) }
    }

    /// Sets the grid cursor position.
    pub fn set_grid_cursor(&self, row: i32, col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetGridCursor(ptr, row, col) }
    }

    /// Moves the cursor to the specified cell and makes it visible.
    pub fn go_to_cell(&self, row: i32, col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_GoToCell(ptr, row, col) }
    }

    // --- Cell Visibility ---

    /// Returns true if the cell is visible.
    pub fn is_visible(&self, row: i32, col: i32, whole_cell_visible: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_IsVisible(ptr, row, col, whole_cell_visible) }
    }

    /// Makes the cell visible by scrolling.
    pub fn make_cell_visible(&self, row: i32, col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_MakeCellVisible(ptr, row, col) }
    }

    // --- Editing ---

    /// Returns true if the grid is editable.
    pub fn is_editable(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_IsEditable(ptr) }
    }

    /// Enables or disables editing for the grid.
    pub fn enable_editing(&self, edit: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_EnableEditing(ptr, edit) }
    }

    /// Enables or disables the cell edit control.
    pub fn enable_cell_edit_control(&self, enable: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_EnableCellEditControl(ptr, enable) }
    }

    /// Disables the cell edit control.
    pub fn disable_cell_edit_control(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_DisableCellEditControl(ptr) }
    }

    /// Returns true if the cell edit control is enabled.
    pub fn is_cell_edit_control_enabled(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_IsCellEditControlEnabled(ptr) }
    }

    // --- Grid Lines ---

    /// Enables or disables grid lines.
    pub fn enable_grid_lines(&self, enable: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_EnableGridLines(ptr, enable) }
    }

    /// Returns true if grid lines are enabled.
    pub fn grid_lines_enabled(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_GridLinesEnabled(ptr) }
    }

    /// Gets the grid line colour.
    pub fn get_grid_line_colour(&self) -> Colour {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Colour::new(0, 0, 0, 255);
        }
        unsafe {
            let c = ffi::wxd_Grid_GetGridLineColour(ptr);
            Colour::new(c.r, c.g, c.b, c.a)
        }
    }

    /// Sets the grid line colour.
    pub fn set_grid_line_colour(&self, colour: &Colour) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetGridLineColour(ptr, (*colour).into()) }
    }

    // --- Label Appearance ---

    /// Gets the label background colour.
    pub fn get_label_background_colour(&self) -> Colour {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Colour::new(192, 192, 192, 255);
        }
        unsafe {
            let c = ffi::wxd_Grid_GetLabelBackgroundColour(ptr);
            Colour::new(c.r, c.g, c.b, c.a)
        }
    }

    /// Sets the label background colour.
    pub fn set_label_background_colour(&self, colour: &Colour) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetLabelBackgroundColour(ptr, (*colour).into()) }
    }

    /// Gets the label text colour.
    pub fn get_label_text_colour(&self) -> Colour {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Colour::new(0, 0, 0, 255);
        }
        unsafe {
            let c = ffi::wxd_Grid_GetLabelTextColour(ptr);
            Colour::new(c.r, c.g, c.b, c.a)
        }
    }

    /// Sets the label text colour.
    pub fn set_label_text_colour(&self, colour: &Colour) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetLabelTextColour(ptr, (*colour).into()) }
    }

    // --- Batch Updates ---

    /// Begins a batch update (prevents screen updates until EndBatch).
    pub fn begin_batch(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_BeginBatch(ptr) }
    }

    /// Ends a batch update.
    pub fn end_batch(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_EndBatch(ptr) }
    }

    /// Gets the current batch count.
    pub fn get_batch_count(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetBatchCount(ptr) }
    }

    /// Forces an immediate refresh of the grid.
    pub fn force_refresh(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_ForceRefresh(ptr) }
    }

    /// Clears all cell values in the grid.
    pub fn clear_grid(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_ClearGrid(ptr) }
    }

    // --- Drag Operations ---

    /// Enables or disables row resizing by dragging.
    pub fn enable_drag_row_size(&self, enable: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_EnableDragRowSize(ptr, enable) }
    }

    /// Enables or disables column resizing by dragging.
    pub fn enable_drag_col_size(&self, enable: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_EnableDragColSize(ptr, enable) }
    }

    /// Enables or disables cell dragging.
    pub fn enable_drag_cell(&self, enable: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_EnableDragCell(ptr, enable) }
    }

    // --- Selection Mode ---

    /// Sets the selection mode.
    pub fn set_selection_mode(&self, mode: GridSelectionMode) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetSelectionMode(ptr, mode as i32) }
    }

    /// Gets the selection mode.
    pub fn get_selection_mode(&self) -> GridSelectionMode {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return GridSelectionMode::Cells;
        }
        let mode = unsafe { ffi::wxd_Grid_GetSelectionMode(ptr) };
        match mode {
            1 => GridSelectionMode::Rows,
            2 => GridSelectionMode::Columns,
            3 => GridSelectionMode::RowsOrColumns,
            4 => GridSelectionMode::None,
            _ => GridSelectionMode::Cells,
        }
    }

    // --- Selection Colors ---

    /// Gets the selection background colour.
    pub fn get_selection_background(&self) -> Colour {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Colour::new(0, 0, 128, 255);
        }
        unsafe {
            let c = ffi::wxd_Grid_GetSelectionBackground(ptr);
            Colour::new(c.r, c.g, c.b, c.a)
        }
    }

    /// Sets the selection background colour.
    pub fn set_selection_background(&self, colour: &Colour) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetSelectionBackground(ptr, (*colour).into()) }
    }

    /// Gets the selection foreground colour.
    pub fn get_selection_foreground(&self) -> Colour {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Colour::new(255, 255, 255, 255);
        }
        unsafe {
            let c = ffi::wxd_Grid_GetSelectionForeground(ptr);
            Colour::new(c.r, c.g, c.b, c.a)
        }
    }

    /// Sets the selection foreground colour.
    pub fn set_selection_foreground(&self, colour: &Colour) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetSelectionForeground(ptr, (*colour).into()) }
    }

    // --- Row/Column Hiding ---

    /// Hides a row.
    pub fn hide_row(&self, row: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_HideRow(ptr, row) }
    }

    /// Shows a hidden row.
    pub fn show_row(&self, row: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_ShowRow(ptr, row) }
    }

    /// Returns true if the row is shown.
    pub fn is_row_shown(&self, row: i32) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_IsRowShown(ptr, row) }
    }

    /// Hides a column.
    pub fn hide_col(&self, col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_HideCol(ptr, col) }
    }

    /// Shows a hidden column.
    pub fn show_col(&self, col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_ShowCol(ptr, col) }
    }

    /// Returns true if the column is shown.
    pub fn is_col_shown(&self, col: i32) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_IsColShown(ptr, col) }
    }
}

// --- Trait Implementations ---

impl WxWidget for Grid {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

impl WxEvtHandler for Grid {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// --- Builder ---

widget_builder!(
    name: Grid,
    parent_type: &'a dyn WxWidget,
    style_type: GridStyle,
    fields: {
        num_rows: i32 = 0,
        num_cols: i32 = 0,
        selection_mode: GridSelectionMode = GridSelectionMode::Cells
    },
    build_impl: |slf| {
        let grid = Grid::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits()
        );

        // If rows and cols are specified, create the grid
        if slf.num_rows > 0 && slf.num_cols > 0 {
            grid.create_grid(slf.num_rows, slf.num_cols, slf.selection_mode);
        }

        grid
    }
);

impl<'a> GridBuilder<'a> {
    /// Sets the number of rows for the grid.
    /// Alias for with_num_rows
    pub fn with_rows(mut self, rows: i32) -> Self {
        self.num_rows = rows;
        self
    }

    /// Sets the number of columns for the grid.
    /// Alias for with_num_cols
    pub fn with_cols(mut self, cols: i32) -> Self {
        self.num_cols = cols;
        self
    }
}

// --- Event Handlers ---

crate::implement_widget_local_event_handlers!(
    Grid,
    GridEvent,
    GridEventData,
    CellLeftClick => cell_left_click, EventType::GRID_CELL_LEFT_CLICK,
    CellRightClick => cell_right_click, EventType::GRID_CELL_RIGHT_CLICK,
    CellLeftDClick => cell_left_dclick, EventType::GRID_CELL_LEFT_DCLICK,
    CellRightDClick => cell_right_dclick, EventType::GRID_CELL_RIGHT_DCLICK,
    LabelLeftClick => label_left_click, EventType::GRID_LABEL_LEFT_CLICK,
    LabelRightClick => label_right_click, EventType::GRID_LABEL_RIGHT_CLICK,
    LabelLeftDClick => label_left_dclick, EventType::GRID_LABEL_LEFT_DCLICK,
    LabelRightDClick => label_right_dclick, EventType::GRID_LABEL_RIGHT_DCLICK,
    CellChanged => cell_changed, EventType::GRID_CELL_CHANGED,
    SelectCell => select_cell, EventType::GRID_SELECT_CELL,
    EditorShown => editor_shown, EventType::GRID_EDITOR_SHOWN,
    EditorHidden => editor_hidden, EventType::GRID_EDITOR_HIDDEN,
    EditorCreated => editor_created, EventType::GRID_EDITOR_CREATED,
    CellBeginDrag => cell_begin_drag, EventType::GRID_CELL_BEGIN_DRAG,
    RowSize => row_size, EventType::GRID_ROW_SIZE,
    ColSize => col_size, EventType::GRID_COL_SIZE,
    RangeSelected => range_selected, EventType::GRID_RANGE_SELECTED
);

// Widget casting support for Grid
impl crate::window::FromWindowWithClassName for Grid {
    fn class_name() -> &'static str {
        "wxGrid"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Grid {
            handle: WindowHandle::new(ptr),
        }
    }
}

// XRC Support
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for Grid {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Grid {
            handle: WindowHandle::new(ptr),
        }
    }
}
