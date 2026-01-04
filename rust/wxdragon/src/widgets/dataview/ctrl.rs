//! DataViewCtrl implementation.

use crate::{Id, WxWidget};
// These macros are exported at the crate root
use wxdragon_sys as ffi;

use super::enums::DataViewColumnFlags;
use super::renderer::DataViewIconTextRenderer;
use super::{
    DataViewAlign, DataViewBitmapRenderer, DataViewCellMode, DataViewChoiceRenderer, DataViewColumn, DataViewDateRenderer,
    DataViewItem, DataViewModel, DataViewProgressRenderer, DataViewSpinRenderer, DataViewTextRenderer, DataViewToggleRenderer,
    VariantType,
};

use crate::color::Colour;
use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::window::WindowHandle;

// Define style enum for DataViewCtrl using the macro
widget_style_enum!(
    name: DataViewStyle,
    doc: "Style flags for DataViewCtrl widgets.",
    variants: {
        Single: ffi::WXD_DV_SINGLE, "Single-selection mode.",
        Multiple: ffi::WXD_DV_MULTIPLE, "Multiple-selection mode.",
        RowLines: ffi::WXD_DV_ROW_LINES, "Display row dividers.",
        HorizontalRules: ffi::WXD_DV_HORIZ_RULES, "Display horizontal rules.",
        VerticalRules: ffi::WXD_DV_VERT_RULES, "Display vertical rules.",
        VariableLineHeight: ffi::WXD_DV_VARIABLE_LINE_HEIGHT, "Enable variable line height.",
        NoHeader: ffi::WXD_DV_NO_HEADER, "Hide column headers."
    },
    default_variant: Single
);

/// Represents a wxWidgets DataViewCtrl in Rust.
///
/// DataViewCtrl is a control that displays data in a tabular or tree-like format,
/// with customizable renderers and a flexible data model.
///
/// DataViewCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Features
///
/// - Displays data in a customizable grid or tree format
/// - Supports multiple column types (text, toggle, progress, etc.)
/// - Configurable selection modes (single or multiple)
/// - Row/column highlighting and styling options
///
/// # Example
///
/// ```rust,no_run
/// use wxdragon::prelude::*;
/// # let frame = Frame::builder().build();
///
/// let panel = Panel::builder(&frame)
///     .build();
///
/// let data_view = DataViewCtrl::builder(&panel)
///     .with_id(100)
///     .with_style(DataViewStyle::RowLines | DataViewStyle::VerticalRules)
///     .build();
/// ```
#[derive(Clone, Copy)]
pub struct DataViewCtrl {
    /// Safe handle to the underlying wxDataViewCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

/// Configuration for appending a spin column
#[derive(Debug, Clone)]
pub struct SpinColumnConfig {
    pub label: String,
    pub model_column: usize,
    pub width: i32,
    pub align: DataViewAlign,
    pub min: i32,
    pub max: i32,
    pub inc: i32,
    pub flags: DataViewColumnFlags,
}

impl SpinColumnConfig {
    pub fn new(label: &str, model_column: usize, min: i32, max: i32) -> Self {
        Self {
            label: label.to_string(),
            model_column,
            width: 80,
            align: DataViewAlign::Left,
            min,
            max,
            inc: 1,
            flags: DataViewColumnFlags::Resizable,
        }
    }

    pub fn with_width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }

    pub fn with_align(mut self, align: DataViewAlign) -> Self {
        self.align = align;
        self
    }

    pub fn with_inc(mut self, inc: i32) -> Self {
        self.inc = inc;
        self
    }

    pub fn with_flags(mut self, flags: DataViewColumnFlags) -> Self {
        self.flags = flags;
        self
    }
}

impl DataViewCtrl {
    /// Creates a builder for configuring and constructing a DataViewCtrl.
    pub fn builder(parent: &dyn WxWidget) -> DataViewCtrlBuilder<'_> {
        DataViewCtrlBuilder::new(parent)
    }

    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: i32, pos: Point, size: Size, style: i64) -> Self {
        let ptr = unsafe {
            ffi::wxd_DataViewCtrl_Create(
                parent_ptr,
                id as i64,
                &pos as *const Point as *const ffi::wxd_Point,
                &size as *const Size as *const ffi::wxd_Size,
                style,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create DataViewCtrl widget");
        }

        DataViewCtrl {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Helper to get raw window pointer, returns null if widget has been destroyed
    #[inline]
    fn dvc_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    /// Returns the underlying WindowHandle for this DataViewCtrl.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    /// Associates a data model with this DataViewCtrl.
    ///
    /// The model provides the data that will be displayed in the control.
    ///
    /// # Returns
    ///
    /// `true` if the model was successfully associated, `false` otherwise.
    ///
    /// # Important
    ///
    /// This method doesn't take ownership of the model. When you associate a model
    /// with a DataViewCtrl, wxWidgets creates an internal copy of the model, which
    /// results in reference counting being managed by wxWidgets. You must ensure
    /// that:
    ///
    /// 1. The model lives at least as long as the control
    /// 2. The model's callbacks remain valid for its entire lifetime
    /// 3. You don't prematurely call any "release" methods on the model
    ///
    /// For CustomDataViewVirtualListModel, the Rust implementation already handles
    /// these requirements correctly in the Drop trait.
    pub fn associate_model<M: DataViewModel>(&self, model: &M) -> bool {
        // IMPORTANT: The model needs to remain valid for the lifetime of this control
        // wxWidgets doesn't fully manage the lifetime of custom models.
        let model_ptr = model.handle_ptr();
        unsafe { ffi::wxd_DataViewCtrl_AssociateModel(self.dvc_ptr(), model_ptr) }
    }

    /// Appends a column to the control.
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_column(&self, column: &DataViewColumn) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_AppendColumn(self.dvc_ptr(), column.as_raw()) }
    }

    /// Prepends a column to the control.
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully prepended, `false` otherwise.
    pub fn prepend_column(&self, column: &DataViewColumn) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_PrependColumn(self.dvc_ptr(), column.as_raw()) }
    }

    /// Inserts a column at the specified position.
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully inserted, `false` otherwise.
    pub fn insert_column(&self, pos: usize, column: &DataViewColumn) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_InsertColumn(self.dvc_ptr(), pos as i64, column.as_raw()) }
    }

    /// Selects the specified row.
    ///
    /// # Returns
    ///
    /// `true` if the row was successfully selected, `false` otherwise.
    pub fn select_row(&self, row: usize) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_SelectRow(self.dvc_ptr(), row as i64) }
    }

    /// Gets the currently selected row.
    ///
    /// # Returns
    ///
    /// An `Option` containing the index of the selected row, or `None` if no row is selected.
    pub fn get_selected_row(&self) -> Option<usize> {
        let row = unsafe { ffi::wxd_DataViewCtrl_GetSelectedRow(self.dvc_ptr()) };
        if row >= 0 { Some(row as usize) } else { None }
    }

    /// Deselects all currently selected items.
    pub fn unselect_all(&self) {
        unsafe { ffi::wxd_DataViewCtrl_UnselectAll(self.dvc_ptr()) }
    }

    /// Creates and appends a text column to this control.
    ///
    /// This is a convenience method for creating a text renderer column and appending it.
    ///
    /// # Parameters
    ///
    /// * `label` - The header label for the column
    /// * `model_column` - The column index in the data model
    /// * `width` - The column width (in pixels)
    /// * `align` - The text alignment
    /// * `flags` - Column flags (e.g., resizable, sortable)
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_text_column(
        &self,
        label: &str,
        model_column: usize,
        width: i32,
        align: DataViewAlign,
        flags: DataViewColumnFlags,
    ) -> bool {
        let renderer = DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, align);
        let column = DataViewColumn::new(label, &renderer, model_column, width, align, flags);
        self.append_column(&column)
    }

    /// Creates and appends a toggle (checkbox) column to this control.
    ///
    /// This is a convenience method for creating a toggle renderer column and appending it.
    ///
    /// # Parameters
    ///
    /// * `label` - The header label for the column
    /// * `model_column` - The column index in the data model
    /// * `width` - The column width (in pixels)
    /// * `align` - The alignment
    /// * `flags` - Column flags
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_toggle_column(
        &self,
        label: &str,
        model_column: usize,
        width: i32,
        align: DataViewAlign,
        flags: DataViewColumnFlags,
    ) -> bool {
        let renderer = DataViewToggleRenderer::new(VariantType::Bool, DataViewCellMode::Activatable, align);
        let column = DataViewColumn::new(label, &renderer, model_column, width, align, flags);
        self.append_column(&column)
    }

    /// Creates and appends a progress bar column to this control.
    ///
    /// This is a convenience method for creating a progress renderer column and appending it.
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
        let renderer = DataViewProgressRenderer::new(VariantType::Int32, DataViewCellMode::Inert);
        let column = DataViewColumn::new(label, &renderer, model_column, width, DataViewAlign::Center, flags);
        self.append_column(&column)
    }

    /// Creates and appends a bitmap column to this control.
    ///
    /// This is a convenience method for creating a bitmap renderer column and appending it.
    ///
    /// # Parameters
    ///
    /// * `label` - The header label for the column
    /// * `model_column` - The column index in the data model
    /// * `width` - The column width (in pixels)
    /// * `align` - The alignment
    /// * `flags` - Column flags
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_bitmap_column(
        &self,
        label: &str,
        model_column: usize,
        width: i32,
        align: DataViewAlign,
        flags: DataViewColumnFlags,
    ) -> bool {
        let renderer = DataViewBitmapRenderer::new(DataViewCellMode::Inert, align);
        let column = DataViewColumn::new(label, &renderer, model_column, width, align, flags);
        self.append_column(&column)
    }

    /// Creates and appends a date column to this control.
    ///
    /// This is a convenience method for creating a date renderer column and appending it.
    ///
    /// # Parameters
    ///
    /// * `label` - The header label for the column
    /// * `model_column` - The column index in the data model
    /// * `width` - The column width (in pixels)
    /// * `align` - The alignment
    /// * `flags` - Column flags
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_date_column(
        &self,
        label: &str,
        model_column: usize,
        width: i32,
        align: DataViewAlign,
        flags: DataViewColumnFlags,
    ) -> bool {
        let renderer = DataViewDateRenderer::new(VariantType::DateTime, DataViewCellMode::Activatable, align);
        let column = DataViewColumn::new(label, &renderer, model_column, width, align, flags);
        self.append_column(&column)
    }

    /// Creates and appends a choice column to this control.
    ///
    /// This is a convenience method for creating a choice renderer column and appending it.
    ///
    /// # Parameters
    ///
    /// * `label` - The header label for the column
    /// * `model_column` - The column index in the data model
    /// * `width` - The column width (in pixels)
    /// * `align` - The alignment
    /// * `choices` - A slice of string choices for the dropdown
    /// * `flags` - Column flags
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_choice_column(
        &self,
        label: &str,
        model_column: usize,
        width: i32,
        align: DataViewAlign,
        choices: &[&str],
        flags: DataViewColumnFlags,
    ) -> bool {
        let renderer = DataViewChoiceRenderer::new(VariantType::String, choices, DataViewCellMode::Editable, align);
        let column = DataViewColumn::new(label, &renderer, model_column, width, align, flags);
        self.append_column(&column)
    }

    /// Creates and appends a spin column to this control.
    /// This is a convenience method for creating a spin renderer and appending it.
    ///
    /// # Parameters
    ///
    /// * `config` - Configuration for the spin column
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_spin_column(&self, config: SpinColumnConfig) -> bool {
        let renderer = DataViewSpinRenderer::new(
            VariantType::Int64,
            DataViewCellMode::Editable,
            config.align,
            config.min,
            config.max,
            config.inc,
        );
        let column = DataViewColumn::new(
            &config.label,
            &renderer,
            config.model_column,
            config.width,
            config.align,
            config.flags,
        );
        self.append_column(&column)
    }

    /// Creates and appends an icon and text column to this control.
    /// This is a convenience method for creating a text renderer with icon support and appending it.
    ///
    /// # Parameters
    ///
    /// * `label` - The header label for the column.
    /// * `model_column` - The column index in the data model that provides the text and icon.
    /// * `width` - The desired width of the column in pixels.
    /// * `align` - The alignment of the content within the column.
    /// * `flags` - Column behavior flags (e.g., resizable, sortable).
    ///
    /// # Returns
    ///
    /// `true` if the column was successfully appended, `false` otherwise.
    pub fn append_icon_text_column(
        &self,
        label: &str,
        model_column: usize,
        width: i32,
        align: DataViewAlign,
        flags: DataViewColumnFlags,
    ) -> bool {
        let renderer = DataViewIconTextRenderer::new(VariantType::String, DataViewCellMode::Inert, align);
        let column = DataViewColumn::new(label, &renderer, model_column, width, align, flags);
        self.append_column(&column)
    }

    /// Gets the number of columns in the control.
    ///
    /// # Returns
    ///
    /// The number of columns.
    pub fn get_column_count(&self) -> usize {
        unsafe { ffi::wxd_DataViewCtrl_GetColumnCount(self.dvc_ptr()) as usize }
    }

    /// Gets a column by index.
    ///
    /// # Parameters
    ///
    /// * `pos` - The position of the column to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing the column, or `None` if `pos` is out of bounds.
    pub fn get_column(&self, pos: usize) -> Option<DataViewColumn> {
        if pos >= self.get_column_count() {
            return None; // Prevent out-of-bounds access
        }
        let raw_col = unsafe { ffi::wxd_DataViewCtrl_GetColumn(self.dvc_ptr(), pos as u32) };
        if raw_col.is_null() {
            None
        } else {
            // DataViewColumn::from_ptr is unsafe
            unsafe { Some(DataViewColumn::from_ptr(raw_col)) }
        }
    }

    /// Gets the position of a column.
    ///
    /// # Parameters
    ///
    /// * `column` - The column to find.
    ///
    /// # Returns
    ///
    /// The position of the column, or -1 if not found.
    pub fn get_column_position(&self, column: &DataViewColumn) -> i32 {
        unsafe { ffi::wxd_DataViewCtrl_GetColumnPosition(self.dvc_ptr(), column.as_raw()) }
    }

    /// Removes all columns from the control.
    ///
    /// # Returns
    ///
    /// `true` if successful, `false` otherwise.
    pub fn clear_columns(&self) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_ClearColumns(self.dvc_ptr()) }
    }

    /// Selects a specific item.
    ///
    /// # Parameters
    ///
    /// * `item` - The item to select.
    pub fn select(&self, item: &DataViewItem) {
        unsafe { ffi::wxd_DataViewCtrl_Select(self.dvc_ptr(), **item) };
    }

    /// Unselects a specific item.
    ///
    /// # Parameters
    ///
    /// * `item` - The item to unselect.
    pub fn unselect(&self, item: &DataViewItem) {
        unsafe { ffi::wxd_DataViewCtrl_Unselect(self.dvc_ptr(), **item) };
    }

    /// Selects all items in the control.
    pub fn select_all(&self) {
        unsafe { ffi::wxd_DataViewCtrl_SelectAll(self.dvc_ptr()) }
    }

    /// Checks if an item is selected.
    ///
    /// # Parameters
    ///
    /// * `item` - The item to check.
    ///
    /// # Returns
    ///
    /// `true` if the item is selected, `false` otherwise.
    pub fn is_selected(&self, item: &DataViewItem) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_IsSelected(self.dvc_ptr(), **item) }
    }

    /// Gets the number of selected items.
    ///
    /// # Returns
    ///
    /// The number of selected items.
    pub fn get_selected_items_count(&self) -> usize {
        unsafe { ffi::wxd_DataViewCtrl_GetSelectedItemsCount(self.dvc_ptr()) as usize }
    }

    /// Checks if any items are selected.
    ///
    /// # Returns
    ///
    /// `true` if any items are selected, `false` otherwise.
    pub fn has_selection(&self) -> bool {
        self.get_selected_items_count() > 0
    }

    /// Gets all selected items.
    ///
    /// # Returns
    ///
    /// A vector of selected items.
    pub fn get_selections(&self) -> Vec<DataViewItem> {
        let count = self.get_selected_items_count();
        if count == 0 {
            return Vec::new();
        }

        let mut items = Vec::with_capacity(count);
        // Create an array of const pointers that the C++ side will fill with newly-created wrappers
        let mut items_raw: Vec<*const ffi::wxd_DataViewItem_t> = vec![std::ptr::null(); count];

        let ptr = items_raw.as_mut_ptr();
        unsafe { ffi::wxd_DataViewCtrl_GetSelections(self.dvc_ptr(), ptr, count as u32) };

        for raw_ptr in items_raw {
            if !raw_ptr.is_null() {
                items.push(DataViewItem::from(raw_ptr));
            }
        }

        items
    }

    /// Sets multiple item selections.
    ///
    /// # Parameters
    ///
    /// * `items` - The items to select.
    pub fn set_selections(&self, items: &[DataViewItem]) {
        let items_raw: Vec<_> = items.iter().map(|item| **item).collect();
        unsafe { ffi::wxd_DataViewCtrl_SetSelections(self.dvc_ptr(), items_raw.as_ptr(), items_raw.len() as u32) };
    }

    /// Gets the currently focused item.
    ///
    /// # Returns
    ///
    /// An `Option` containing the current item, or `None` if no item is focused.
    pub fn get_current_item(&self) -> Option<DataViewItem> {
        let item_ptr = unsafe { ffi::wxd_DataViewCtrl_GetCurrentItem(self.dvc_ptr()) };
        if item_ptr.is_null() {
            None
        } else {
            Some(DataViewItem::from(item_ptr))
        }
    }

    /// Gets the nth child of a parent item (works for tree models attached to a DataViewCtrl)
    pub fn get_nth_child(&self, parent: &DataViewItem, pos: u32) -> DataViewItem {
        let item = unsafe { ffi::wxd_DataViewCtrl_GetNthChild(self.dvc_ptr(), **parent, pos) };
        DataViewItem::from(item)
    }

    /// Expand the given item (works for tree models attached to a DataViewCtrl)
    pub fn expand(&self, item: &DataViewItem) {
        unsafe { ffi::wxd_DataViewCtrl_Expand(self.dvc_ptr(), **item) };
    }

    /// Ensure the given item is visible (scroll into view)
    pub fn ensure_visible(&self, item: &DataViewItem) {
        unsafe { ffi::wxd_DataViewCtrl_EnsureVisible(self.dvc_ptr(), **item) };
    }

    /// Gets the currently selected item.
    ///
    /// # Returns
    ///
    /// An `Option` containing the selected item, or `None` if no item is selected.
    pub fn get_selection(&self) -> Option<DataViewItem> {
        let item_ptr = unsafe { ffi::wxd_DataViewCtrl_GetSelection(self.dvc_ptr()) };
        if item_ptr.is_null() {
            None
        } else {
            Some(DataViewItem::from(item_ptr))
        }
    }

    /// Sets the currently focused item.
    ///
    /// # Parameters
    ///
    /// * `item` - The item to set as current.
    pub fn set_current_item(&self, item: &DataViewItem) {
        unsafe { ffi::wxd_DataViewCtrl_SetCurrentItem(self.dvc_ptr(), **item) }
    }

    /// Gets the currently used indentation.
    ///
    /// # Returns
    ///
    /// The current indentation in pixels.
    pub fn get_indent(&self) -> i32 {
        unsafe { ffi::wxd_DataViewCtrl_GetIndent(self.dvc_ptr()) }
    }

    /// Sets the indentation for hierarchical items.
    ///
    /// # Parameters
    ///
    /// * `indent` - The indentation in pixels to use.
    pub fn set_indent(&self, indent: i32) {
        unsafe { ffi::wxd_DataViewCtrl_SetIndent(self.dvc_ptr(), indent) }
    }

    /// Gets the column used as the expander column in tree mode.
    ///
    /// # Returns
    ///
    /// An `Option` containing the expander column, or `None` if not set.
    pub fn get_expander_column(&self) -> Option<DataViewColumn> {
        let col_ptr = unsafe { ffi::wxd_DataViewCtrl_GetExpanderColumn(self.dvc_ptr()) };
        if col_ptr.is_null() {
            None
        } else {
            Some(unsafe { DataViewColumn::from_ptr(col_ptr) })
        }
    }

    /// Sets which column shall contain the tree-like expanders.
    ///
    /// # Parameters
    ///
    /// * `column` - The column to use as the expander column.
    pub fn set_expander_column(&self, column: &DataViewColumn) {
        unsafe { ffi::wxd_DataViewCtrl_SetExpanderColumn(self.dvc_ptr(), column.as_raw()) }
    }

    /// Sets the height of each row.
    ///
    /// # Parameters
    ///
    /// * `height` - The height in pixels for each row.
    ///
    /// # Returns
    ///
    /// `true` if row height was changed, `false` otherwise.
    ///
    /// # Note
    ///
    /// This cannot be used when the `VariableLineHeight` style is enabled.
    pub fn set_row_height(&self, height: i32) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_SetRowHeight(self.dvc_ptr(), height) }
    }

    /// Sets alternate row colors for the control.
    ///
    /// # Parameters
    ///
    /// * `colour` - The color to use for alternate rows
    ///
    /// # Returns
    ///
    /// `true` if the operation was successful, `false` otherwise.
    pub fn set_alternate_row_colour(&self, colour: &Colour) -> bool {
        let colour_raw = colour.to_raw();
        unsafe { ffi::wxd_DataViewCtrl_SetAlternateRowColour(self.dvc_ptr(), &colour_raw) }
    }

    /// Clears any current sorting on the control.
    ///
    /// Removes any active sorting from the `DataViewCtrl`, restoring the default (unsorted) order of items as provided by the model.
    ///
    /// # Behavior
    ///
    /// - The UI is updated immediately to reflect the removal of sorting; items will be displayed in their original order.
    /// - If a sort indicator was shown in the column header, it will be cleared.
    /// - This may trigger a `EVT_DATAVIEW_COLUMN_SORTED` or similar event, depending on the platform and model implementation.
    ///
    /// # Platform-specific notes
    ///
    /// - On all supported platforms (Windows, macOS, Linux/GTK), this method behaves consistently and clears sorting as expected.
    /// - If the control is not currently sorted, calling this method has no effect.
    pub fn clear_sorting(&self) {
        unsafe { ffi::wxd_DataViewCtrl_ClearSorting(self.dvc_ptr()) }
    }

    /// Programmatically set the sorting column and order.
    ///
    /// # Parameters
    ///
    /// * `column_index` - The **model column index** to sort by (not the display position). This refers to the index as used in your data model, regardless of column reordering or visibility in the UI.
    /// * `ascending` - If `true`, sort in ascending order; if `false`, sort in descending order.
    ///
    /// # Behavior
    ///
    /// Calling this method will immediately trigger a resort of the data in the control according to the specified column and order.
    ///
    /// # Events
    ///
    /// This operation may emit sorting-related events, such as [`EVT_DATAVIEW_COLUMN_SORTED`](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html), depending on the platform and model implementation.
    ///
    /// # Returns
    ///
    /// Returns `true` if the column index was valid and sorting was applied, `false` otherwise.
    pub fn set_sorting_column(&self, column_index: usize, ascending: bool) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_SetSortingColumn(self.dvc_ptr(), column_index as i32, ascending) }
    }

    /// Returns the current sorting state if any: (model_column_index, ascending).
    /// If no sorting is active, returns None.
    pub fn sorting_state(&self) -> Option<(usize, bool)> {
        let mut col: i32 = -1;
        let mut asc: bool = true;
        let ok = unsafe { ffi::wxd_DataViewCtrl_GetSortingState(self.dvc_ptr(), &mut col, &mut asc) };
        if ok && col >= 0 { Some((col as usize, asc)) } else { None }
    }
}

// Manual WxWidget implementation for DataViewCtrl (using WindowHandle)
impl WxWidget for DataViewCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for DataViewCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for DataViewCtrl {}

widget_builder!(
    name: DataViewCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: DataViewStyle,
    fields: {},
    build_impl: |slf| {
        DataViewCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
);

// Implement DataViewEventHandler for DataViewCtrl
impl crate::widgets::dataview::DataViewEventHandler for DataViewCtrl {}

// Implement DataViewTreeEventHandler for DataViewCtrl since it supports tree functionality
impl crate::widgets::dataview::DataViewTreeEventHandler for DataViewCtrl {}
