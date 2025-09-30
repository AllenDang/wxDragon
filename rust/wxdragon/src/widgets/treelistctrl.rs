//! wxTreeListCtrl wrapper
//!
//! The `TreeListCtrl` widget combines the functionality of a tree control with list control columns,
//! providing a powerful way to display hierarchical data with additional information in columns.
//! It supports checkboxes for easy selection/deselection of items.
//!
//! # Examples
//!
//! ```rust,no_run
//! use wxdragon::prelude::*;
//! use wxdragon::widgets::treelistctrl::{TreeListCtrl, TreeListCtrlStyle, CheckboxState, TreeListCtrlEventData};
//! use wxdragon::widgets::list_ctrl::ListColumnFormat;
//!
//! fn create_tree_list(parent: &dyn WxWidget) -> TreeListCtrl {
//!     // Create a tree list control with checkboxes
//!     let tree_list = TreeListCtrl::builder(parent)
//!         .with_style(TreeListCtrlStyle::Default | TreeListCtrlStyle::Checkbox)
//!         .build();
//!
//!     // Add columns
//!     tree_list.append_column("Name", 200, ListColumnFormat::Left);
//!     tree_list.append_column("Size", 100, ListColumnFormat::Right);
//!     tree_list.append_column("Modified", 150, ListColumnFormat::Left);
//!
//!     // Add root item
//!     let root = tree_list.append_item(&tree_list.get_root_item(), "Documents").unwrap();
//!     tree_list.set_item_text(&root, 1, "Folder");
//!     tree_list.set_item_text(&root, 2, "Today");
//!
//!     // Add child items with checkboxes
//!     let doc1 = tree_list.append_item(&root, "Report.pdf").unwrap();
//!     tree_list.set_item_text(&doc1, 1, "1.2 MB");
//!     tree_list.set_item_text(&doc1, 2, "Yesterday");
//!     tree_list.check_item(&doc1, CheckboxState::Checked);
//!
//!     // Set up event handlers using the generated methods
//!     tree_list.on_item_checked(|event: TreeListCtrlEventData| {
//!         if let Some(item) = event.get_item() {
//!             if let Some(is_checked) = event.is_checked() {
//!                 println!("Item {:?} was {}", item, if is_checked { "checked" } else { "unchecked" });
//!             }
//!         }
//!     });
//!
//!     tree_list.on_selection_changed(|event: TreeListCtrlEventData| {
//!         if let Some(item) = event.get_item() {
//!             println!("Selection changed to item {:?}", item);
//!         }
//!     });
//!
//!     tree_list.on_column_sorted(|event: TreeListCtrlEventData| {
//!         if let Some(column) = event.get_column() {
//!             println!("Column {} was clicked for sorting", column);
//!         }
//!     });
//!
//!     tree_list.on_item_checked(|event: TreeListCtrlEventData| {
//!         if let Some(item) = event.get_item() {
//!             if let Some(old_state) = event.get_old_checked_state() {
//!                 if let Some(is_checked) = event.is_checked() {
//!                     println!("Item {:?} changed from {:?} to {}",
//!                         item, old_state, if is_checked { "checked" } else { "unchecked" });
//!                 }
//!             }
//!         }
//!     });
//!
//!     tree_list
//! }
//! ```

use std::ffi::CString;
use std::os::raw::c_char;

use crate::event::{Event, EventType};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::widgets::list_ctrl::ListColumnFormat;
use crate::window::{Window, WxWidget};
use wxdragon_sys as ffi;

// --- TreeListCtrl Styles ---
widget_style_enum!(
    name: TreeListCtrlStyle,
    doc: "Style flags for TreeListCtrl widget.",
    variants: {
        Default: 0, "Default style.",
        Checkbox: 1, "Show checkboxes next to items.",
        Three_State: 2, "Use 3-state checkboxes (checked, unchecked, undetermined)."
    },
    default_variant: Default
);

/// Checkbox state for tree list items
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxState {
    Unchecked = 0,
    Checked = 1,
    Undetermined = 2,
}

impl From<CheckboxState> for i32 {
    fn from(val: CheckboxState) -> Self {
        val as i32
    }
}

impl From<i32> for CheckboxState {
    fn from(val: i32) -> Self {
        match val {
            0 => CheckboxState::Unchecked,
            1 => CheckboxState::Checked,
            2 => CheckboxState::Undetermined,
            _ => CheckboxState::Unchecked,
        }
    }
}

/// Represents a tree item ID in the TreeListCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeListItem {
    id: i64,
}

impl TreeListItem {
    pub fn new(id: i64) -> Self {
        Self { id }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn is_valid(&self) -> bool {
        self.id != 0
    }
}

/// Events emitted by TreeListCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeListCtrlEvent {
    /// Emitted when the selection changes
    SelectionChanged,
    /// Emitted when an item checkbox is checked/unchecked
    ItemChecked,
    /// Emitted when an item is activated (double-clicked)
    ItemActivated,
    /// Emitted when a column header is clicked for sorting
    ColumnSorted,
    /// Emitted when an item is expanding
    ItemExpanding,
    /// Emitted when an item has expanded
    ItemExpanded,
}

/// Event data for TreeListCtrl events
#[derive(Debug)]
pub struct TreeListCtrlEventData {
    event: Event,
}

impl TreeListCtrlEventData {
    /// Create a new TreeListCtrlEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the widget ID that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Get the TreeListItem of the affected item
    pub fn get_item(&self) -> Option<TreeListItem> {
        if self.event.is_null() {
            return None;
        }
        let id = unsafe { ffi::wxd_TreeListEvent_GetItem(self.event._as_ptr()) };
        if id != 0 {
            Some(TreeListItem::new(id))
        } else {
            None
        }
    }

    /// Get the column index for column-related events
    pub fn get_column(&self) -> Option<i32> {
        if self.event.is_null() {
            return None;
        }
        let col = unsafe { ffi::wxd_TreeListEvent_GetColumn(self.event._as_ptr()) };
        if col >= 0 {
            Some(col)
        } else {
            None
        }
    }

    /// Get the label text for label edit events (fallback to generic event string)
    pub fn get_string(&self) -> Option<String> {
        self.event.get_string()
    }

    /// Get the previous checkbox state for ItemChecked events
    pub fn get_old_checked_state(&self) -> Option<CheckboxState> {
        if self.event.is_null() {
            return None;
        }
        let state = unsafe { ffi::wxd_TreeListEvent_GetOldCheckedState(self.event._as_ptr()) };
        if state >= 0 {
            Some(CheckboxState::from(state))
        } else {
            None
        }
    }

    /// Get the checkbox state for ItemChecked events
    pub fn is_checked(&self) -> Option<bool> {
        self.event.is_checked()
    }

    /// Skip this event (allow it to be processed by the parent window)
    pub fn skip(&self, skip: bool) {
        self.event.skip(skip);
    }
}

/// Represents a wxTreeListCtrl widget.
///
/// TreeListCtrl combines tree functionality with list columns, allowing hierarchical data
/// to be displayed with additional information in columns. It supports checkboxes for
/// easy selection/deselection of items.
#[derive(Clone)]
pub struct TreeListCtrl {
    window: Window,
}

impl TreeListCtrl {
    /// Creates a new TreeListCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> TreeListCtrlBuilder<'_> {
        TreeListCtrlBuilder::new(parent)
    }

    /// Creates a new TreeListCtrl wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_TreeListCtrl_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_TreeListCtrl_t) -> Self {
        TreeListCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Internal implementation used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        let ptr = unsafe {
            ffi::wxd_TreeListCtrl_Create(
                parent_ptr,
                id,
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create TreeListCtrl widget");
        }

        unsafe { TreeListCtrl::from_ptr(ptr) }
    }

    // --- Column Management ---

    /// Appends a new column to the control.
    ///
    /// # Arguments
    /// * `text` - The column header text
    /// * `width` - The column width in pixels
    /// * `align` - The column alignment
    ///
    /// Returns the column index.
    pub fn append_column(&self, text: &str, width: i32, align: ListColumnFormat) -> i32 {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_TreeListCtrl_AppendColumn(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                c_text.as_ptr(),
                width,
                align.as_i32(),
            )
        }
    }

    /// Gets the number of columns in the control.
    pub fn get_column_count(&self) -> i32 {
        unsafe {
            ffi::wxd_TreeListCtrl_GetColumnCount(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t
            )
        }
    }

    /// Sets the width of the specified column.
    pub fn set_column_width(&self, col: i32, width: i32) {
        unsafe {
            ffi::wxd_TreeListCtrl_SetColumnWidth(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                col,
                width,
            );
        }
    }

    /// Gets the width of the specified column.
    pub fn get_column_width(&self, col: i32) -> i32 {
        unsafe {
            ffi::wxd_TreeListCtrl_GetColumnWidth(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                col,
            )
        }
    }

    /// Deletes the column with the given index.
    pub fn delete_column(&self, col: u32) -> bool {
        unsafe {
            ffi::wxd_TreeListCtrl_DeleteColumn(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                col,
            )
        }
    }

    /// Deletes all columns.
    pub fn clear_columns(&self) {
        unsafe {
            ffi::wxd_TreeListCtrl_ClearColumns(self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t);
        }
    }

    /// Gets the width appropriate for showing the given text.
    pub fn width_for(&self, text: &str) -> i32 {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_TreeListCtrl_WidthFor(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                c_text.as_ptr(),
            )
        }
    }

    // --- Item Management ---

    /// Gets the root item of the tree.
    pub fn get_root_item(&self) -> TreeListItem {
        let id = unsafe {
            ffi::wxd_TreeListCtrl_GetRootItem(self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t)
        };
        TreeListItem::new(id)
    }

    /// Appends a new item to the tree.
    ///
    /// # Arguments
    /// * `parent` - The parent item
    /// * `text` - The item text
    ///
    /// Returns the new item, or None if the operation failed.
    pub fn append_item(&self, parent: &TreeListItem, text: &str) -> Option<TreeListItem> {
        let c_text = CString::new(text).unwrap_or_default();
        let id = unsafe {
            ffi::wxd_TreeListCtrl_AppendItem(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                parent.id(),
                c_text.as_ptr(),
            )
        };
        if id != 0 {
            Some(TreeListItem::new(id))
        } else {
            None
        }
    }

    /// Inserts a new item into the tree.
    ///
    /// # Arguments
    /// * `parent` - The parent item
    /// * `previous` - The item after which to insert the new item
    /// * `text` - The item text
    ///
    /// Returns the new item, or None if the operation failed.
    pub fn insert_item(
        &self,
        parent: &TreeListItem,
        previous: &TreeListItem,
        text: &str,
    ) -> Option<TreeListItem> {
        let c_text = CString::new(text).unwrap_or_default();
        let id = unsafe {
            ffi::wxd_TreeListCtrl_InsertItem(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                parent.id(),
                previous.id(),
                c_text.as_ptr(),
            )
        };
        if id != 0 {
            Some(TreeListItem::new(id))
        } else {
            None
        }
    }

    /// Prepends a new item to the tree (inserts at the beginning).
    ///
    /// # Arguments
    /// * `parent` - The parent item
    /// * `text` - The item text
    ///
    /// Returns the new item, or None if the operation failed.
    pub fn prepend_item(&self, parent: &TreeListItem, text: &str) -> Option<TreeListItem> {
        let c_text = CString::new(text).unwrap_or_default();
        let id = unsafe {
            ffi::wxd_TreeListCtrl_PrependItem(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                parent.id(),
                c_text.as_ptr(),
            )
        };
        if id != 0 {
            Some(TreeListItem::new(id))
        } else {
            None
        }
    }

    /// Deletes the specified item.
    pub fn delete_item(&self, item: &TreeListItem) {
        unsafe {
            ffi::wxd_TreeListCtrl_DeleteItem(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            );
        }
    }

    /// Deletes all items in the tree.
    pub fn delete_all_items(&self) {
        unsafe {
            ffi::wxd_TreeListCtrl_DeleteAllItems(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t
            );
        }
    }

    /// Sets the text for the specified item and column.
    pub fn set_item_text(&self, item: &TreeListItem, col: i32, text: &str) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_TreeListCtrl_SetItemText(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
                col,
                c_text.as_ptr(),
            );
        }
    }

    /// Gets the text for the specified item and column.
    pub fn get_item_text(&self, item: &TreeListItem, col: i32) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_TreeListCtrl_GetItemText(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
                col,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );
            if len >= 0 {
                let byte_slice =
                    std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    /// Expands the specified item.
    pub fn expand(&self, item: &TreeListItem) {
        unsafe {
            ffi::wxd_TreeListCtrl_Expand(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            );
        }
    }

    /// Collapses the specified item.
    pub fn collapse(&self, item: &TreeListItem) {
        unsafe {
            ffi::wxd_TreeListCtrl_Collapse(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            );
        }
    }

    /// Checks if the specified item is expanded.
    pub fn is_expanded(&self, item: &TreeListItem) -> bool {
        unsafe {
            ffi::wxd_TreeListCtrl_IsExpanded(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            )
        }
    }

    // --- Selection Management ---

    /// Gets the currently selected item.
    pub fn get_selection(&self) -> Option<TreeListItem> {
        let id = unsafe {
            ffi::wxd_TreeListCtrl_GetSelection(self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t)
        };
        if id != 0 {
            Some(TreeListItem::new(id))
        } else {
            None
        }
    }

    /// Selects the specified item.
    pub fn select_item(&self, item: &TreeListItem) {
        unsafe {
            ffi::wxd_TreeListCtrl_SelectItem(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            );
        }
    }

    /// Unselects all items.
    pub fn unselect_all(&self) {
        unsafe {
            ffi::wxd_TreeListCtrl_UnselectAll(self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t);
        }
    }

    // --- Checkbox Management ---

    /// Checks or unchecks the specified item.
    pub fn check_item(&self, item: &TreeListItem, state: CheckboxState) {
        unsafe {
            ffi::wxd_TreeListCtrl_CheckItem(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
                state.into(),
            );
        }
    }

    /// Gets the checkbox state of the specified item.
    pub fn get_checked_state(&self, item: &TreeListItem) -> CheckboxState {
        let state = unsafe {
            ffi::wxd_TreeListCtrl_GetCheckedState(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            )
        };
        CheckboxState::from(state)
    }

    /// Checks if the specified item is checked.
    pub fn is_checked(&self, item: &TreeListItem) -> bool {
        self.get_checked_state(item) == CheckboxState::Checked
    }

    /// Checks all items recursively starting from the specified item.
    pub fn check_item_recursively(&self, item: &TreeListItem, state: CheckboxState) {
        unsafe {
            ffi::wxd_TreeListCtrl_CheckItemRecursively(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
                state.into(),
            );
        }
    }

    /// Updates the parent's checkbox state based on children (for 3-state checkboxes).
    pub fn update_item_parent_state(&self, item: &TreeListItem) {
        unsafe {
            ffi::wxd_TreeListCtrl_UpdateItemParentState(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            );
        }
    }

    // --- Tree Navigation ---

    /// Gets the parent of the specified item.
    pub fn get_item_parent(&self, item: &TreeListItem) -> Option<TreeListItem> {
        let id = unsafe {
            ffi::wxd_TreeListCtrl_GetItemParent(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            )
        };
        if id != 0 {
            Some(TreeListItem::new(id))
        } else {
            None
        }
    }

    /// Gets the first child of the specified item.
    pub fn get_first_child(&self, item: &TreeListItem) -> Option<TreeListItem> {
        let id = unsafe {
            ffi::wxd_TreeListCtrl_GetFirstChild(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            )
        };
        if id != 0 {
            Some(TreeListItem::new(id))
        } else {
            None
        }
    }

    /// Gets the next sibling of the specified item.
    pub fn get_next_sibling(&self, item: &TreeListItem) -> Option<TreeListItem> {
        let id = unsafe {
            ffi::wxd_TreeListCtrl_GetNextSibling(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            )
        };
        if id != 0 {
            Some(TreeListItem::new(id))
        } else {
            None
        }
    }

    /// Gets the next item in depth-first tree traversal order.
    pub fn get_next_item(&self, item: &TreeListItem) -> Option<TreeListItem> {
        let id = unsafe {
            ffi::wxd_TreeListCtrl_GetNextItem(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            )
        };
        if id != 0 {
            Some(TreeListItem::new(id))
        } else {
            None
        }
    }

    /// Gets the first item in the tree (first child of root).
    pub fn get_first_item(&self) -> Option<TreeListItem> {
        let id = unsafe {
            ffi::wxd_TreeListCtrl_GetFirstItem(self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t)
        };
        if id != 0 {
            Some(TreeListItem::new(id))
        } else {
            None
        }
    }

    // --- Item Attributes ---

    /// Sets the image for the specified item.
    pub fn set_item_image(&self, item: &TreeListItem, closed: i32, opened: i32) {
        unsafe {
            ffi::wxd_TreeListCtrl_SetItemImage(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
                closed,
                opened,
            );
        }
    }

    // --- Multi-Selection Support ---

    /// Gets all selected items. Returns a vector of selected items.
    pub fn get_selections(&self) -> Vec<TreeListItem> {
        const MAX_SELECTIONS: usize = 1000;
        let mut selections: Vec<i64> = vec![0; MAX_SELECTIONS];
        let count = unsafe {
            ffi::wxd_TreeListCtrl_GetSelections(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                selections.as_mut_ptr(),
                MAX_SELECTIONS as u32,
            )
        };

        selections.truncate(count as usize);
        selections.into_iter().map(TreeListItem::new).collect()
    }

    /// Selects the specified item.
    pub fn select(&self, item: &TreeListItem) {
        unsafe {
            ffi::wxd_TreeListCtrl_Select(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            );
        }
    }

    /// Unselects the specified item.
    pub fn unselect(&self, item: &TreeListItem) {
        unsafe {
            ffi::wxd_TreeListCtrl_Unselect(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            );
        }
    }

    /// Checks if the specified item is selected.
    pub fn is_selected(&self, item: &TreeListItem) -> bool {
        unsafe {
            ffi::wxd_TreeListCtrl_IsSelected(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            )
        }
    }

    /// Selects all items (only valid in multiple selection mode).
    pub fn select_all(&self) {
        unsafe {
            ffi::wxd_TreeListCtrl_SelectAll(self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t);
        }
    }

    /// Ensures the specified item is visible.
    pub fn ensure_visible(&self, item: &TreeListItem) {
        unsafe {
            ffi::wxd_TreeListCtrl_EnsureVisible(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            );
        }
    }

    // --- Additional Checkbox Methods ---

    /// Unchecks the specified item.
    pub fn uncheck_item(&self, item: &TreeListItem) {
        unsafe {
            ffi::wxd_TreeListCtrl_UncheckItem(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
            );
        }
    }

    /// Checks if all children of the specified item are in the given state.
    pub fn are_all_children_in_state(&self, item: &TreeListItem, state: CheckboxState) -> bool {
        unsafe {
            ffi::wxd_TreeListCtrl_AreAllChildrenInState(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                item.id(),
                state as i32,
            )
        }
    }

    // --- Sorting ---

    /// Sets the column to sort by.
    pub fn set_sort_column(&self, col: u32, ascending: bool) {
        unsafe {
            ffi::wxd_TreeListCtrl_SetSortColumn(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                col,
                ascending,
            );
        }
    }

    /// Gets the current sort column and order.
    pub fn get_sort_column(&self) -> Option<(u32, bool)> {
        let mut col: u32 = 0;
        let mut ascending: bool = true;
        let has_sort = unsafe {
            ffi::wxd_TreeListCtrl_GetSortColumn(
                self.window.as_ptr() as *mut ffi::wxd_TreeListCtrl_t,
                &mut col,
                &mut ascending,
            )
        };
        if has_sort {
            Some((col, ascending))
        } else {
            None
        }
    }
}

// Apply common trait implementations for this widget
implement_widget_traits_with_target!(TreeListCtrl, window, Window);

// Use the widget_builder macro for TreeListCtrl
widget_builder!(
    name: TreeListCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: TreeListCtrlStyle,
    fields: {},
    build_impl: |slf| {
        TreeListCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
);

// Implement event handlers for TreeListCtrl
crate::implement_widget_local_event_handlers!(
    TreeListCtrl,
    TreeListCtrlEvent,
    TreeListCtrlEventData,
    SelectionChanged => selection_changed, EventType::TREELIST_SELECTION_CHANGED,
    ItemChecked => item_checked, EventType::TREELIST_ITEM_CHECKED,
    ItemActivated => item_activated, EventType::TREELIST_ITEM_ACTIVATED,
    ColumnSorted => column_sorted, EventType::TREELIST_COLUMN_SORTED,
    ItemExpanding => item_expanding, EventType::TREELIST_ITEM_EXPANDING,
    ItemExpanded => item_expanded, EventType::TREELIST_ITEM_EXPANDED
);

// Implement standard window events trait

// XRC Support - enables TreeListCtrl to be created from XRC-managed pointers
impl_xrc_support!(TreeListCtrl, { window });

// Widget casting support for TreeListCtrl
impl_widget_cast!(TreeListCtrl, "wxTreeListCtrl", { window });
