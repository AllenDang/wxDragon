//! wxGrid wrapper - a powerful spreadsheet-like grid control

use crate::color::Colour;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::font::Font;
use crate::geometry::{Point, Rect, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

/// Cell span type returned by `get_cell_size`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum CellSpan {
    /// This cell is inside a span covered by another cell.
    Inside = -1,
    /// This is a normal, non-spanning cell.
    None = 0,
    /// This cell spans several physical grid cells.
    Main = 1,
}

/// Tab behaviour when cursor reaches the end of a row.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum TabBehaviour {
    /// Do nothing (default).
    Stop = 0,
    /// Move to the beginning of the next (or end of previous) row.
    Wrap = 1,
    /// Move to the next (or previous) control after the grid.
    Leave = 2,
}

// --- Grid Coordinate Types ---

/// Represents a cell position in the grid (row, column).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GridCellCoords {
    pub row: i32,
    pub col: i32,
}

/// Represents a rectangular block of cells in the grid.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GridBlockCoords {
    pub top_row: i32,
    pub left_col: i32,
    pub bottom_row: i32,
    pub right_col: i32,
}

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

        let ptr = unsafe { ffi::wxd_Grid_Create(parent_ptr, id, pos.into(), size.into(), style) };

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
            CStr::from_ptr(buffer.as_ptr() as *const i8).to_string_lossy().into_owned()
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
            CStr::from_ptr(buffer.as_ptr() as *const i8).to_string_lossy().into_owned()
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
            CStr::from_ptr(buffer.as_ptr() as *const i8).to_string_lossy().into_owned()
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

    /// Auto-sizes the label of a specific row to fit its content.
    pub fn auto_size_row_label_size(&self, row: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_AutoSizeRowLabelSize(ptr, row) }
    }

    /// Auto-sizes the label of a specific column to fit its content.
    pub fn auto_size_col_label_size(&self, col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_AutoSizeColLabelSize(ptr, col) }
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

    /// Gets the individually selected cells (not part of block/row/col selections).
    pub fn get_selected_cells(&self) -> Vec<GridCellCoords> {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Vec::new();
        }
        unsafe {
            let count = ffi::wxd_Grid_GetSelectedCells(ptr, std::ptr::null_mut(), 0);
            if count <= 0 {
                return Vec::new();
            }
            let mut buffer = vec![ffi::wxd_GridCellCoords { row: 0, col: 0 }; count as usize];
            ffi::wxd_Grid_GetSelectedCells(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            buffer.iter().map(|c| GridCellCoords { row: c.row, col: c.col }).collect()
        }
    }

    /// Gets all selected blocks as a unified view of the selection.
    ///
    /// This merges all types of selection (cells, rows, columns, blocks) into
    /// a minimal set of non-overlapping rectangular blocks.
    pub fn get_selected_blocks(&self) -> Vec<GridBlockCoords> {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Vec::new();
        }
        unsafe {
            let count = ffi::wxd_Grid_GetSelectedBlocks(ptr, std::ptr::null_mut(), 0);
            if count <= 0 {
                return Vec::new();
            }
            let mut buffer = vec![
                ffi::wxd_GridBlockCoords {
                    top_row: 0,
                    left_col: 0,
                    bottom_row: 0,
                    right_col: 0,
                };
                count as usize
            ];
            ffi::wxd_Grid_GetSelectedBlocks(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            buffer
                .iter()
                .map(|b| GridBlockCoords {
                    top_row: b.top_row,
                    left_col: b.left_col,
                    bottom_row: b.bottom_row,
                    right_col: b.right_col,
                })
                .collect()
        }
    }

    /// Gets the selected row blocks.
    ///
    /// Returns blocks corresponding to contiguous ranges of selected rows.
    pub fn get_selected_row_blocks(&self) -> Vec<GridBlockCoords> {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Vec::new();
        }
        unsafe {
            let count = ffi::wxd_Grid_GetSelectedRowBlocks(ptr, std::ptr::null_mut(), 0);
            if count <= 0 {
                return Vec::new();
            }
            let mut buffer = vec![
                ffi::wxd_GridBlockCoords {
                    top_row: 0,
                    left_col: 0,
                    bottom_row: 0,
                    right_col: 0,
                };
                count as usize
            ];
            ffi::wxd_Grid_GetSelectedRowBlocks(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            buffer
                .iter()
                .map(|b| GridBlockCoords {
                    top_row: b.top_row,
                    left_col: b.left_col,
                    bottom_row: b.bottom_row,
                    right_col: b.right_col,
                })
                .collect()
        }
    }

    /// Gets the selected column blocks.
    ///
    /// Returns blocks corresponding to contiguous ranges of selected columns.
    pub fn get_selected_col_blocks(&self) -> Vec<GridBlockCoords> {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Vec::new();
        }
        unsafe {
            let count = ffi::wxd_Grid_GetSelectedColBlocks(ptr, std::ptr::null_mut(), 0);
            if count <= 0 {
                return Vec::new();
            }
            let mut buffer = vec![
                ffi::wxd_GridBlockCoords {
                    top_row: 0,
                    left_col: 0,
                    bottom_row: 0,
                    right_col: 0,
                };
                count as usize
            ];
            ffi::wxd_Grid_GetSelectedColBlocks(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            buffer
                .iter()
                .map(|b| GridBlockCoords {
                    top_row: b.top_row,
                    left_col: b.left_col,
                    bottom_row: b.bottom_row,
                    right_col: b.right_col,
                })
                .collect()
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

    // --- Cell Font ---

    /// Returns the font for the cell at the specified location.
    pub fn get_cell_font(&self, row: i32, col: i32) -> Option<Font> {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return None;
        }
        let font_ptr = unsafe { ffi::wxd_Grid_GetCellFont(ptr, row, col) };
        if font_ptr.is_null() {
            return None;
        }
        Some(unsafe { Font::from_ptr(font_ptr, true) })
    }

    /// Sets the font for the cell at the specified location.
    pub fn set_cell_font(&self, row: i32, col: i32, font: &Font) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetCellFont(ptr, row, col, font.as_ptr() as *const _) }
    }

    /// Returns the default font for grid cells.
    pub fn get_default_cell_font(&self) -> Option<Font> {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return None;
        }
        let font_ptr = unsafe { ffi::wxd_Grid_GetDefaultCellFont(ptr) };
        if font_ptr.is_null() {
            return None;
        }
        Some(unsafe { Font::from_ptr(font_ptr, true) })
    }

    /// Sets the default font for grid cells.
    pub fn set_default_cell_font(&self, font: &Font) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetDefaultCellFont(ptr, font.as_ptr() as *const _) }
    }

    // --- Label Font ---

    /// Returns the font used for row and column labels.
    pub fn get_label_font(&self) -> Option<Font> {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return None;
        }
        let font_ptr = unsafe { ffi::wxd_Grid_GetLabelFont(ptr) };
        if font_ptr.is_null() {
            return None;
        }
        Some(unsafe { Font::from_ptr(font_ptr, true) })
    }

    /// Sets the font used for row and column labels.
    pub fn set_label_font(&self, font: &Font) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetLabelFont(ptr, font.as_ptr() as *const _) }
    }

    // --- Label Alignment ---

    /// Returns the column label alignment as (horizontal, vertical).
    pub fn get_col_label_alignment(&self) -> (i32, i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return (0, 0);
        }
        let (mut h, mut v) = (0i32, 0i32);
        unsafe { ffi::wxd_Grid_GetColLabelAlignment(ptr, &mut h, &mut v) }
        (h, v)
    }

    /// Sets the column label alignment.
    pub fn set_col_label_alignment(&self, horiz: i32, vert: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetColLabelAlignment(ptr, horiz, vert) }
    }

    /// Returns the row label alignment as (horizontal, vertical).
    pub fn get_row_label_alignment(&self) -> (i32, i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return (0, 0);
        }
        let (mut h, mut v) = (0i32, 0i32);
        unsafe { ffi::wxd_Grid_GetRowLabelAlignment(ptr, &mut h, &mut v) }
        (h, v)
    }

    /// Sets the row label alignment.
    pub fn set_row_label_alignment(&self, horiz: i32, vert: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetRowLabelAlignment(ptr, horiz, vert) }
    }

    /// Returns the column label text orientation.
    pub fn get_col_label_text_orientation(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetColLabelTextOrientation(ptr) }
    }

    /// Sets the column label text orientation.
    pub fn set_col_label_text_orientation(&self, orientation: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetColLabelTextOrientation(ptr, orientation) }
    }

    // --- Corner Label ---

    /// Returns the corner label value.
    pub fn get_corner_label_value(&self) -> String {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let needed = unsafe { ffi::wxd_Grid_GetCornerLabelValue(ptr, std::ptr::null_mut(), 0) };
        if needed <= 0 {
            return String::new();
        }
        let mut buf = vec![0u8; (needed + 1) as usize];
        unsafe { ffi::wxd_Grid_GetCornerLabelValue(ptr, buf.as_mut_ptr() as *mut i8, buf.len() as i32) };
        let c_str = unsafe { CStr::from_ptr(buf.as_ptr() as *const i8) };
        c_str.to_string_lossy().into_owned()
    }

    /// Sets the corner label value.
    pub fn set_corner_label_value(&self, value: &str) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        let c_str = CString::new(value).unwrap_or_default();
        unsafe { ffi::wxd_Grid_SetCornerLabelValue(ptr, c_str.as_ptr()) }
    }

    /// Returns the corner label alignment as (horizontal, vertical).
    pub fn get_corner_label_alignment(&self) -> (i32, i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return (0, 0);
        }
        let (mut h, mut v) = (0i32, 0i32);
        unsafe { ffi::wxd_Grid_GetCornerLabelAlignment(ptr, &mut h, &mut v) }
        (h, v)
    }

    /// Sets the corner label alignment.
    pub fn set_corner_label_alignment(&self, horiz: i32, vert: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetCornerLabelAlignment(ptr, horiz, vert) }
    }

    /// Returns the corner label text orientation.
    pub fn get_corner_label_text_orientation(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetCornerLabelTextOrientation(ptr) }
    }

    /// Sets the corner label text orientation.
    pub fn set_corner_label_text_orientation(&self, orientation: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetCornerLabelTextOrientation(ptr, orientation) }
    }

    // --- Native Column Header ---

    /// Use native rendering for column labels.
    pub fn set_use_native_col_labels(&self, native_labels: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetUseNativeColLabels(ptr, native_labels) }
    }

    /// Enable native header control for column labels.
    pub fn use_native_col_header(&self, native_header: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_UseNativeColHeader(ptr, native_header) }
    }

    /// Returns true if native header control is being used.
    pub fn is_using_native_header(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_IsUsingNativeHeader(ptr) }
    }

    // --- Cell Spanning ---

    /// Sets the cell at (row, col) to span num_rows rows and num_cols columns.
    pub fn set_cell_size(&self, row: i32, col: i32, num_rows: i32, num_cols: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetCellSize(ptr, row, col, num_rows, num_cols) }
    }

    /// Gets the size of the cell in number of cells covered.
    /// Returns (span_type, num_rows, num_cols).
    pub fn get_cell_size(&self, row: i32, col: i32) -> (CellSpan, i32, i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return (CellSpan::None, 1, 1);
        }
        let (mut nr, mut nc) = (0i32, 0i32);
        let span = unsafe { ffi::wxd_Grid_GetCellSize(ptr, row, col, &mut nr, &mut nc) };
        let cell_span = match span {
            -1 => CellSpan::Inside,
            1 => CellSpan::Main,
            _ => CellSpan::None,
        };
        (cell_span, nr, nc)
    }

    // --- Cell Overflow ---

    /// Returns true if the cell value can overflow into adjacent cells.
    pub fn get_cell_overflow(&self, row: i32, col: i32) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return true;
        }
        unsafe { ffi::wxd_Grid_GetCellOverflow(ptr, row, col) }
    }

    /// Sets overflow permission for the cell.
    pub fn set_cell_overflow(&self, row: i32, col: i32, allow: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetCellOverflow(ptr, row, col, allow) }
    }

    /// Returns the default cell overflow setting.
    pub fn get_default_cell_overflow(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return true;
        }
        unsafe { ffi::wxd_Grid_GetDefaultCellOverflow(ptr) }
    }

    /// Sets the default cell overflow setting.
    pub fn set_default_cell_overflow(&self, allow: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetDefaultCellOverflow(ptr, allow) }
    }

    // --- Column Format ---

    /// Sets the column to display boolean values.
    pub fn set_col_format_bool(&self, col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetColFormatBool(ptr, col) }
    }

    /// Sets the column to display integer values.
    pub fn set_col_format_number(&self, col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetColFormatNumber(ptr, col) }
    }

    /// Sets the column to display float values with given width and precision.
    pub fn set_col_format_float(&self, col: i32, width: i32, precision: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetColFormatFloat(ptr, col, width, precision) }
    }

    /// Sets the column to display date values with optional format string.
    pub fn set_col_format_date(&self, col: i32, format: &str) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        let c_str = CString::new(format).unwrap_or_default();
        unsafe { ffi::wxd_Grid_SetColFormatDate(ptr, col, c_str.as_ptr()) }
    }

    /// Sets the column to display a custom type.
    pub fn set_col_format_custom(&self, col: i32, type_name: &str) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        let c_str = CString::new(type_name).unwrap_or_default();
        unsafe { ffi::wxd_Grid_SetColFormatCustom(ptr, col, c_str.as_ptr()) }
    }

    // --- Sorting ---

    /// Returns the column currently displaying the sorting indicator, or -1.
    pub fn get_sorting_column(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_GetSortingColumn(ptr) }
    }

    /// Returns true if this column is currently used for sorting.
    pub fn is_sorting_by(&self, col: i32) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_IsSortingBy(ptr, col) }
    }

    /// Returns true if the current sort order is ascending.
    pub fn is_sort_order_ascending(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return true;
        }
        unsafe { ffi::wxd_Grid_IsSortOrderAscending(ptr) }
    }

    /// Sets the column to display the sorting indicator.
    pub fn set_sorting_column(&self, col: i32, ascending: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetSortingColumn(ptr, col, ascending) }
    }

    /// Removes any currently shown sorting indicator.
    pub fn unset_sorting_column(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_UnsetSortingColumn(ptr) }
    }

    // --- Tab Behaviour ---

    /// Sets the grid's TAB key behaviour.
    pub fn set_tab_behaviour(&self, behaviour: TabBehaviour) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetTabBehaviour(ptr, behaviour as i32) }
    }

    // --- Frozen Rows/Cols ---

    /// Freezes the specified number of rows and columns.
    pub fn freeze_to(&self, row: i32, col: i32) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_FreezeTo(ptr, row, col) }
    }

    /// Returns the number of frozen rows.
    pub fn get_number_frozen_rows(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetNumberFrozenRows(ptr) }
    }

    /// Returns the number of frozen columns.
    pub fn get_number_frozen_cols(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetNumberFrozenCols(ptr) }
    }

    // --- Row/Col Minimal Sizes ---

    /// Returns the minimal acceptable column width.
    pub fn get_col_minimal_acceptable_width(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetColMinimalAcceptableWidth(ptr) }
    }

    /// Sets the minimal acceptable column width.
    pub fn set_col_minimal_acceptable_width(&self, width: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetColMinimalAcceptableWidth(ptr, width) }
    }

    /// Sets the minimal width for a specific column.
    pub fn set_col_minimal_width(&self, col: i32, width: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetColMinimalWidth(ptr, col, width) }
    }

    /// Returns the minimal acceptable row height.
    pub fn get_row_minimal_acceptable_height(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetRowMinimalAcceptableHeight(ptr) }
    }

    /// Sets the minimal acceptable row height.
    pub fn set_row_minimal_acceptable_height(&self, height: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetRowMinimalAcceptableHeight(ptr, height) }
    }

    /// Sets the minimal height for a specific row.
    pub fn set_row_minimal_height(&self, row: i32, height: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetRowMinimalHeight(ptr, row, height) }
    }

    // --- Default Label Sizes ---

    /// Returns the default width for row labels.
    pub fn get_default_row_label_size(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetDefaultRowLabelSize(ptr) }
    }

    /// Returns the default height for column labels.
    pub fn get_default_col_label_size(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetDefaultColLabelSize(ptr) }
    }

    // --- Cell Edit Control ---

    /// Returns true if the in-place edit control can be used for the current cell.
    pub fn can_enable_cell_control(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_CanEnableCellControl(ptr) }
    }

    /// Returns true if the in-place edit control is currently shown.
    pub fn is_cell_edit_control_shown(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_IsCellEditControlShown(ptr) }
    }

    /// Returns true if the current cell is read-only.
    pub fn is_current_cell_read_only(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_IsCurrentCellReadOnly(ptr) }
    }

    /// Hides the in-place cell edit control.
    pub fn hide_cell_edit_control(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_HideCellEditControl(ptr) }
    }

    /// Shows the in-place cell edit control.
    pub fn show_cell_edit_control(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_ShowCellEditControl(ptr) }
    }

    /// Saves the current in-place edit control value.
    pub fn save_edit_control_value(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SaveEditControlValue(ptr) }
    }

    // --- Cell Highlight ---

    /// Returns the colour used for the cell highlight.
    pub fn get_cell_highlight_colour(&self) -> Colour {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Colour::new(0, 0, 0, 255);
        }
        let c = unsafe { ffi::wxd_Grid_GetCellHighlightColour(ptr) };
        Colour::new(c.r, c.g, c.b, c.a)
    }

    /// Sets the colour used for the cell highlight.
    pub fn set_cell_highlight_colour(&self, colour: Colour) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetCellHighlightColour(ptr, colour.into()) }
    }

    /// Returns the pen width for the cell highlight.
    pub fn get_cell_highlight_pen_width(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetCellHighlightPenWidth(ptr) }
    }

    /// Sets the pen width for the cell highlight.
    pub fn set_cell_highlight_pen_width(&self, width: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetCellHighlightPenWidth(ptr, width) }
    }

    /// Returns the pen width for the read-only cell highlight.
    pub fn get_cell_highlight_ro_pen_width(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Grid_GetCellHighlightROPenWidth(ptr) }
    }

    /// Sets the pen width for the read-only cell highlight.
    pub fn set_cell_highlight_ro_pen_width(&self, width: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetCellHighlightROPenWidth(ptr, width) }
    }

    // --- Frozen Border ---

    /// Sets the colour for the frozen area border.
    pub fn set_grid_frozen_border_colour(&self, colour: Colour) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetGridFrozenBorderColour(ptr, colour.into()) }
    }

    /// Sets the pen width for the frozen area border.
    pub fn set_grid_frozen_border_pen_width(&self, width: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetGridFrozenBorderPenWidth(ptr, width) }
    }

    // --- Cursor Movement ---

    /// Moves the grid cursor up by one row.
    pub fn move_cursor_up(&self, expand_selection: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_MoveCursorUp(ptr, expand_selection) }
    }

    /// Moves the grid cursor down by one row.
    pub fn move_cursor_down(&self, expand_selection: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_MoveCursorDown(ptr, expand_selection) }
    }

    /// Moves the grid cursor left by one column.
    pub fn move_cursor_left(&self, expand_selection: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_MoveCursorLeft(ptr, expand_selection) }
    }

    /// Moves the grid cursor right by one column.
    pub fn move_cursor_right(&self, expand_selection: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_MoveCursorRight(ptr, expand_selection) }
    }

    /// Moves the grid cursor up to the next block boundary.
    pub fn move_cursor_up_block(&self, expand_selection: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_MoveCursorUpBlock(ptr, expand_selection) }
    }

    /// Moves the grid cursor down to the next block boundary.
    pub fn move_cursor_down_block(&self, expand_selection: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_MoveCursorDownBlock(ptr, expand_selection) }
    }

    /// Moves the grid cursor left to the next block boundary.
    pub fn move_cursor_left_block(&self, expand_selection: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_MoveCursorLeftBlock(ptr, expand_selection) }
    }

    /// Moves the grid cursor right to the next block boundary.
    pub fn move_cursor_right_block(&self, expand_selection: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_MoveCursorRightBlock(ptr, expand_selection) }
    }

    /// Moves the grid cursor up by one page.
    pub fn move_page_up(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_MovePageUp(ptr) }
    }

    /// Moves the grid cursor down by one page.
    pub fn move_page_down(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_MovePageDown(ptr) }
    }

    /// Returns the current grid cursor coordinates.
    pub fn get_grid_cursor_coords(&self) -> GridCellCoords {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return GridCellCoords { row: -1, col: -1 };
        }
        let c = unsafe { ffi::wxd_Grid_GetGridCursorCoords(ptr) };
        GridCellCoords { row: c.row, col: c.col }
    }

    // --- Scrolling ---

    /// Returns pixels per horizontal scroll increment.
    pub fn get_scroll_line_x(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 15;
        }
        unsafe { ffi::wxd_Grid_GetScrollLineX(ptr) }
    }

    /// Returns pixels per vertical scroll increment.
    pub fn get_scroll_line_y(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return 15;
        }
        unsafe { ffi::wxd_Grid_GetScrollLineY(ptr) }
    }

    /// Sets pixels per horizontal scroll increment.
    pub fn set_scroll_line_x(&self, x: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetScrollLineX(ptr, x) }
    }

    /// Sets pixels per vertical scroll increment.
    pub fn set_scroll_line_y(&self, y: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetScrollLineY(ptr, y) }
    }

    /// Returns the topmost fully visible row, or -1.
    pub fn get_first_fully_visible_row(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_GetFirstFullyVisibleRow(ptr) }
    }

    /// Returns the leftmost fully visible column, or -1.
    pub fn get_first_fully_visible_column(&self) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_GetFirstFullyVisibleColumn(ptr) }
    }

    // --- Coordinate Conversion ---

    /// Returns the column at the given pixel x position.
    pub fn x_to_col(&self, x: i32, clip_to_min_max: bool) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_XToCol(ptr, x, clip_to_min_max) }
    }

    /// Returns the row at the given pixel y position.
    pub fn y_to_row(&self, y: i32, clip_to_min_max: bool) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_YToRow(ptr, y, clip_to_min_max) }
    }

    /// Returns the column whose right edge is near the given x position.
    pub fn x_to_edge_of_col(&self, x: i32) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_XToEdgeOfCol(ptr, x) }
    }

    /// Returns the row whose bottom edge is near the given y position.
    pub fn y_to_edge_of_row(&self, y: i32) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_YToEdgeOfRow(ptr, y) }
    }

    /// Translates pixel coordinates to cell coordinates.
    pub fn xy_to_cell(&self, x: i32, y: i32) -> GridCellCoords {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return GridCellCoords { row: -1, col: -1 };
        }
        let c = unsafe { ffi::wxd_Grid_XYToCell(ptr, x, y) };
        GridCellCoords { row: c.row, col: c.col }
    }

    /// Returns the rectangle for the given cell in logical coordinates.
    pub fn cell_to_rect(&self, row: i32, col: i32) -> Rect {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Rect::new(0, 0, 0, 0);
        }
        let r = unsafe { ffi::wxd_Grid_CellToRect(ptr, row, col) };
        Rect::new(r.x, r.y, r.width, r.height)
    }

    /// Returns the device rect for a block of cells.
    pub fn block_to_device_rect(&self, top_row: i32, left_col: i32, bottom_row: i32, right_col: i32) -> Rect {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return Rect::new(0, 0, 0, 0);
        }
        let r = unsafe { ffi::wxd_Grid_BlockToDeviceRect(ptr, top_row, left_col, bottom_row, right_col) };
        Rect::new(r.x, r.y, r.width, r.height)
    }

    // --- Grid Clipping ---

    /// Returns true if horizontal grid lines are clipped at the last column.
    pub fn are_horz_grid_lines_clipped(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return true;
        }
        unsafe { ffi::wxd_Grid_AreHorzGridLinesClipped(ptr) }
    }

    /// Returns true if vertical grid lines are clipped at the last row.
    pub fn are_vert_grid_lines_clipped(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return true;
        }
        unsafe { ffi::wxd_Grid_AreVertGridLinesClipped(ptr) }
    }

    /// Sets whether horizontal grid lines are clipped at the last column.
    pub fn clip_horz_grid_lines(&self, clip: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_ClipHorzGridLines(ptr, clip) }
    }

    /// Sets whether vertical grid lines are clipped at the last row.
    pub fn clip_vert_grid_lines(&self, clip: bool) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_ClipVertGridLines(ptr, clip) }
    }

    // --- Extra Drag/Move Operations ---

    /// Returns true if cell dragging is enabled.
    pub fn can_drag_cell(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_CanDragCell(ptr) }
    }

    /// Returns true if columns can be moved by dragging.
    pub fn can_drag_col_move(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_CanDragColMove(ptr) }
    }

    /// Returns true if grid lines can be dragged to resize.
    pub fn can_drag_grid_size(&self) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_CanDragGridSize(ptr) }
    }

    /// Enables or disables column moving by dragging.
    pub fn enable_drag_col_move(&self, enable: bool) -> bool {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Grid_EnableDragColMove(ptr, enable) }
    }

    /// Disables column moving by dragging.
    pub fn disable_drag_col_move(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_DisableDragColMove(ptr) }
    }

    /// Disables column sizing by dragging.
    pub fn disable_drag_col_size(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_DisableDragColSize(ptr) }
    }

    /// Disables row sizing by dragging.
    pub fn disable_drag_row_size(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_DisableDragRowSize(ptr) }
    }

    /// Disables grid line dragging.
    pub fn disable_drag_grid_size(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_DisableDragGridSize(ptr) }
    }

    /// Disables interactive resizing of a specific column.
    pub fn disable_col_resize(&self, col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_DisableColResize(ptr, col) }
    }

    /// Disables interactive resizing of a specific row.
    pub fn disable_row_resize(&self, row: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_DisableRowResize(ptr, row) }
    }

    // --- Column Position/Move (pre-existing FFI) ---

    /// Returns the column ID at the specified display position.
    pub fn get_col_at(&self, pos: i32) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_GetColAt(ptr, pos) }
    }

    /// Returns the display position of the specified column.
    pub fn get_col_pos(&self, col: i32) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_GetColPos(ptr, col) }
    }

    /// Sets the display position of the specified column.
    pub fn set_col_pos(&self, col: i32, pos: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetColPos(ptr, col, pos) }
    }

    /// Resets column positions to default order.
    pub fn reset_col_pos(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_ResetColPos(ptr) }
    }

    // --- Row Position/Move ---

    /// Returns the row ID at the specified position.
    pub fn get_row_at(&self, pos: i32) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_GetRowAt(ptr, pos) }
    }

    /// Returns the position of the specified row.
    pub fn get_row_pos(&self, idx: i32) -> i32 {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Grid_GetRowPos(ptr, idx) }
    }

    /// Sets the position of the specified row.
    pub fn set_row_pos(&self, idx: i32, pos: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetRowPos(ptr, idx, pos) }
    }

    /// Resets row positions to default.
    pub fn reset_row_pos(&self) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_ResetRowPos(ptr) }
    }

    // --- Margins ---

    /// Sets the extra margins around the grid area.
    pub fn set_margins(&self, extra_width: i32, extra_height: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_SetMargins(ptr, extra_width, extra_height) }
    }

    // --- Refresh ---

    /// Invalidates the cached attribute for the given cell.
    pub fn refresh_attr(&self, row: i32, col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_RefreshAttr(ptr, row, col) }
    }

    /// Redraws the specified block of cells.
    pub fn refresh_block(&self, top_row: i32, left_col: i32, bottom_row: i32, right_col: i32) {
        let ptr = self.grid_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Grid_RefreshBlock(ptr, top_row, left_col, bottom_row, right_col) }
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
