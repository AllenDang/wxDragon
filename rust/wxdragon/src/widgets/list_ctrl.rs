//! wxListCtrl wrapper

use crate::event::{Event, EventType, WindowEvents};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::widgets::imagelist::ImageList;
use crate::widgets::item_data::{get_item_data, remove_item_data, store_item_data, HasItemData};
use crate::window::{Window, WxWidget};
use std::any::Any;
use std::ffi::CString;
use std::os::raw::{c_int, c_longlong};
use std::sync::Arc;
use wxdragon_sys as ffi;

// --- ListCtrl Styles ---
widget_style_enum!(
    name: ListCtrlStyle,
    doc: "Style flags for ListCtrl widget.",
    variants: {
        Default: 0, "Default list control style.",
        SingleSel: ffi::WXD_LC_SINGLE_SEL, "Single selection (default is multiple).",
        SortAscending: ffi::WXD_LC_SORT_ASCENDING, "Sort in ascending order.",
        SortDescending: ffi::WXD_LC_SORT_DESCENDING, "Sort in descending order.",
        Virtual: ffi::WXD_LC_VIRTUAL, "The application provides items text on demand.",
        EditLabels: ffi::WXD_LC_EDIT_LABELS, "Labels can be edited for in-place renaming.",

        // View styles
        Icon: ffi::WXD_LC_ICON, "Large icon view.",
        SmallIcon: ffi::WXD_LC_SMALL_ICON, "Small icon view.",
        List: ffi::WXD_LC_LIST, "List view showing items on a single line.",
        Report: ffi::WXD_LC_REPORT, "Multicolumn report view (detail view).",

        // Alignment styles
        AlignTop: ffi::WXD_LC_ALIGN_TOP, "Align icons with the top (default).",
        AlignLeft: ffi::WXD_LC_ALIGN_LEFT, "Align icons with the left.",

        // Behavior styles
        AutoArrange: ffi::WXD_LC_AUTOARRANGE, "Icons arrange themselves.",
        HRules: ffi::WXD_LC_HRULES, "Horizontal rules in report mode.",
        VRules: ffi::WXD_LC_VRULES, "Vertical rules in report mode.",
        NoHeader: ffi::WXD_LC_NO_HEADER, "No header in report mode.",
        NoSort: ffi::WXD_LC_NO_SORT_HEADER, "No sorting when clicking on headers."
    },
    default_variant: Default
);

// --- ListColumnFormat Enum (for LIST_FORMAT_... constants) ---
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
#[derive(Default)]
pub enum ListColumnFormat {
    /// Align column content to the left
    #[default]
    Left = ffi::WXD_LIST_FORMAT_LEFT as i32,
    /// Align column content to the right
    Right = ffi::WXD_LIST_FORMAT_RIGHT as i32,
    /// Align column content to the center
    Centre = ffi::WXD_LIST_FORMAT_CENTRE as i32,
}

impl ListColumnFormat {
    /// Returns the raw integer value of the format
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

// --- ListItemState (for LIST_STATE_... constants) ---
widget_style_enum!(
    name: ListItemState,
    doc: "Item state flags for ListCtrl items.",
    variants: {
        None: 0, "No state (used for clearing states).",
        Selected: ffi::WXD_LIST_STATE_SELECTED, "Item is selected.",
        Focused: ffi::WXD_LIST_STATE_FOCUSED, "Item has focus.",
        Disabled: ffi::WXD_LIST_STATE_DISABLED, "Item is disabled.",
        DropHilited: ffi::WXD_LIST_STATE_DROPHILITED, "Item is highlighted as a drop target."
    },
    default_variant: None
);

// --- ListNextItemFlag Enum (for LIST_NEXT_... constants) ---
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
#[derive(Default)]
pub enum ListNextItemFlag {
    /// All items, no geometric restriction
    #[default]
    All = ffi::WXD_LIST_NEXT_ALL as i32,
    /// Item above current one
    Above = ffi::WXD_LIST_NEXT_ABOVE as i32,
    /// Item below current one
    Below = ffi::WXD_LIST_NEXT_BELOW as i32,
    /// Item to the left of current one
    Left = ffi::WXD_LIST_NEXT_LEFT as i32,
    /// Item to the right of current one
    Right = ffi::WXD_LIST_NEXT_RIGHT as i32,
}

impl ListNextItemFlag {
    /// Returns the raw integer value of the flag
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

/// Events emitted by ListCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListCtrlEvent {
    /// Emitted when an item is selected
    ItemSelected,
    /// Emitted when an item is deselected
    ItemDeselected,
    /// Emitted when an item is activated (double-clicked or Enter)
    ItemActivated,
    /// Emitted when an item is focused
    ItemFocused,
    /// Emitted when a column header is clicked
    ColumnClick,
    /// Emitted when a column header is right-clicked
    ColumnRightClick,
    /// Emitted when a column begins to be dragged
    ColumnBeginDrag,
    /// Emitted when beginning in-place editing of an item's label
    BeginLabelEdit,
    /// Emitted when ending in-place editing of an item's label
    EndLabelEdit,
    /// Emitted when beginning to drag an item
    BeginDrag,
    /// Emitted when beginning to right drag an item
    BeginRDrag,
    /// Emitted when an item is deleted
    DeleteItem,
    /// Emitted when all items are deleted
    DeleteAllItems,
    /// Emitted when the key is pressed with focus on the list
    KeyDown,
    /// Emitted when an item is inserted
    InsertItem,
    /// Emitted when an item is right-clicked
    ItemRightClick,
    /// Emitted when an item is middle-clicked
    ItemMiddleClick,
}

/// Event data for ListCtrl events
#[derive(Debug)]
pub struct ListCtrlEventData {
    event: Event,
}

impl ListCtrlEventData {
    /// Create a new ListCtrlEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the item index affected by the event
    pub fn get_item_index(&self) -> i32 {
        if self.event.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_ListEvent_GetItemIndex(self.event.0) }
    }

    /// Get the column index affected by the event (for column-related events)
    pub fn get_column(&self) -> Option<i32> {
        if self.event.is_null() {
            return None;
        }
        let col = unsafe { ffi::wxd_ListEvent_GetColumn(self.event.0) };
        if col == -1 {
            None
        } else {
            Some(col)
        }
    }

    /// Get the item label (for label edit events)
    pub fn get_label(&self) -> Option<String> {
        if self.event.is_null() {
            return None;
        }
        unsafe {
            let mut buffer: [std::os::raw::c_char; 1024] = [0; 1024];
            let len_needed =
                ffi::wxd_ListEvent_GetLabel(self.event.0, buffer.as_mut_ptr(), buffer.len() as i32);
            if len_needed < 0 {
                return None;
            }
            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                Some(
                    std::ffi::CStr::from_ptr(buffer.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                )
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_ListEvent_GetLabel(
                    self.event.0,
                    vec_buffer.as_mut_ptr() as *mut std::os::raw::c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop();
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None
                }
            }
        }
    }

    /// Check if editing was cancelled (for end edit events)
    pub fn is_edit_cancelled(&self) -> Option<bool> {
        if self.event.is_null() {
            return None;
        }
        // Boolean functions from C++ return int (0/1), explicitly convert to Rust bool
        Some(unsafe { ffi::wxd_ListEvent_IsEditCancelled(self.event.0) })
    }

    /// Get the point where the event occurred (for click events)
    pub fn get_position(&self) -> Option<Point> {
        self.event.get_position()
    }

    /// Get the key code (for key events)
    pub fn get_key_code(&self) -> Option<i32> {
        self.event.get_key_code()
    }
}

// --- ImageList Type Constants ---
/// Constants for image list types
pub mod image_list_type {
    /// Normal sized images (typically for Icon view)
    pub const NORMAL: i32 = 0;
    /// Small sized images (typically for Report/List view)
    pub const SMALL: i32 = 1;
    /// State images (for checkboxes)
    pub const STATE: i32 = 2;
}

/// A control for displaying and manipulating multiple items
///
/// The ListCtrl can display items in various formats including:
/// - List view (one column)
/// - Report view (multiple columns with headers)
/// - Icon view (large or small icons)
#[derive(Clone)]
pub struct ListCtrl {
    window: Window,
}

impl ListCtrl {
    /// Creates a new ListCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> ListCtrlBuilder<'_> {
        ListCtrlBuilder::new(parent)
    }

    /// Internal implementation used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        assert!(!parent_ptr.is_null(), "ListCtrl requires a parent");

        let ptr =
            unsafe { ffi::wxd_ListCtrl_Create(parent_ptr, id, pos.into(), size.into(), style) };

        if ptr.is_null() {
            panic!("Failed to create ListCtrl: FFI returned null pointer.");
        }

        unsafe { Self::from_ptr(ptr) }
    }

    /// Creates a ListCtrl from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be a valid wxd_ListCtrl_t pointer.
    unsafe fn from_ptr(ptr: *mut ffi::wxd_ListCtrl_t) -> Self {
        ListCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the ListCtrl pointer from the window pointer.
    #[inline]
    fn as_list_ctrl_ptr(&self) -> *mut ffi::wxd_ListCtrl_t {
        self.handle_ptr() as *mut ffi::wxd_ListCtrl_t
    }

    /// Inserts a column at the specified position.
    pub fn insert_column(
        &self,
        col: i64,
        heading: &str,
        format: ListColumnFormat,
        width: i32,
    ) -> i32 {
        let c_heading = CString::new(heading).unwrap_or_default();
        unsafe {
            ffi::wxd_ListCtrl_InsertColumn(
                self.as_list_ctrl_ptr(),
                col as c_longlong,
                c_heading.as_ptr(),
                format as c_int,
                width,
            )
        }
    }

    /// Sets the width of the specified column.
    pub fn set_column_width(&self, col: i64, width: i32) -> bool {
        unsafe {
            ffi::wxd_ListCtrl_SetColumnWidth(self.as_list_ctrl_ptr(), col as c_longlong, width)
        }
    }

    /// Gets the width of the specified column.
    pub fn get_column_width(&self, col: i64) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetColumnWidth(self.as_list_ctrl_ptr(), col as c_longlong) }
    }

    /// Gets the number of columns in the list control.
    pub fn get_column_count(&self) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetColumnCount(self.as_list_ctrl_ptr()) }
    }

    /// Inserts a simple item (label only) at the specified index.
    pub fn insert_item(&self, index: i64, label: &str, image_index: Option<i32>) -> i32 {
        let c_label = CString::new(label).unwrap_or_default();
        let img_idx = image_index.unwrap_or(-1); // Use -1 if no image is specified
        unsafe {
            ffi::wxd_ListCtrl_InsertItemWithImage(
                self.as_list_ctrl_ptr(),
                index as c_longlong,
                c_label.as_ptr(),
                img_idx,
            )
        }
    }

    /// Sets the text of an item (label in column 0).
    pub fn set_item_text(&self, index: i64, text: &str) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_ListCtrl_SetItemText(
                self.as_list_ctrl_ptr(),
                index as c_longlong,
                c_text.as_ptr(),
            )
        }
    }

    /// Sets the text of an item in the specified column.
    ///
    /// # Arguments
    /// * `index` - The index of the item.
    /// * `col` - The column index (0-based).
    /// * `text` - The text to set.
    ///
    /// # Example
    /// ```
    /// list_ctrl.set_item_text_by_column(0, 1, "Column 1 text");
    /// ```
    pub fn set_item_text_by_column(&self, index: i64, col: i32, text: &str) {
        if col == 0 {
            // Use the standard method for column 0
            self.set_item_text(index, text);
            return;
        }

        // Use SetItem to set column text
        let c_text = CString::new(text).unwrap_or_default();
        let mask = ffi::WXD_LIST_MASK_TEXT as i64;
        let state = 0;
        let state_mask = 0;
        let image = -1;
        let data = 0;
        let item_fmt = 0;

        unsafe {
            ffi::wxd_ListCtrl_SetItem(
                self.as_list_ctrl_ptr(),
                index as c_longlong,
                col as c_int,
                c_text.as_ptr(),
                image,
                item_fmt,
                state,
                state_mask,
                data,
                mask,
            );
        }
    }

    /// Gets the text of an item in the specified column.
    pub fn get_item_text(&self, index: i64, col: i32) -> String {
        unsafe {
            let needed_len = ffi::wxd_ListCtrl_GetItemText(
                self.as_list_ctrl_ptr(),
                index as c_longlong,
                col,
                std::ptr::null_mut(),
                0,
            );
            if needed_len <= 0 {
                return String::new();
            }
            let mut buffer: Vec<u8> = Vec::with_capacity(needed_len as usize);
            let actual_len = ffi::wxd_ListCtrl_GetItemText(
                self.as_list_ctrl_ptr(),
                index as c_longlong,
                col,
                buffer.as_mut_ptr() as *mut i8,
                needed_len as i32,
            );
            if actual_len <= 0 {
                return String::new();
            }
            buffer.set_len(actual_len as usize);
            String::from_utf8_lossy(&buffer).into_owned()
        }
    }

    /// Gets the number of items in the list control.
    pub fn get_item_count(&self) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetItemCount(self.as_list_ctrl_ptr()) }
    }

    /// Sets the state of an item using the ListItemState enum.
    ///
    /// # Arguments
    /// * `item` - The index of the item.
    /// * `state` - The state flags to set or clear.
    /// * `state_mask` - The state flags to modify (only bits set in mask will be changed).
    ///
    /// # Example
    /// ```
    /// // To select an item:
    /// list_ctrl.set_item_state(0, ListItemState::Selected, ListItemState::Selected);
    /// // To deselect an item:
    /// list_ctrl.set_item_state(0, ListItemState::default(), ListItemState::Selected);
    /// ```
    pub fn set_item_state(
        &self,
        item: i64,
        state: ListItemState,
        state_mask: ListItemState,
    ) -> bool {
        unsafe {
            ffi::wxd_ListCtrl_SetItemState(
                self.as_list_ctrl_ptr(),
                item as c_longlong,
                state.bits() as c_longlong,
                state_mask.bits() as c_longlong,
            )
        }
    }

    /// Gets the state of an item using the ListItemState enum.
    ///
    /// # Arguments
    /// * `item` - The index of the item.
    /// * `state_mask` - The specific state flag to check.
    ///
    /// # Returns
    /// Returns true if the state specified by state_mask is set, false otherwise.
    ///
    /// # Example
    /// ```
    /// // Check if an item is selected:
    /// let is_selected = list_ctrl.get_item_state(0, ListItemState::Selected);
    /// ```
    pub fn get_item_state(&self, item: i64, state_mask: ListItemState) -> bool {
        let state = unsafe {
            ffi::wxd_ListCtrl_GetItemState(
                self.as_list_ctrl_ptr(),
                item as c_longlong,
                state_mask.bits() as c_longlong,
            )
        };
        state != 0
    }

    /// Gets the next item based on geometry and state.
    pub fn get_next_item(
        &self,
        item: i64,
        geometry: ListNextItemFlag,
        state: ListItemState,
    ) -> i32 {
        unsafe {
            ffi::wxd_ListCtrl_GetNextItem(
                self.as_list_ctrl_ptr(),
                item as c_longlong,
                geometry as c_int,
                state.bits() as c_int,
            )
        }
    }

    /// Gets the first selected item in the list control.
    pub fn get_first_selected_item(&self) -> i32 {
        self.get_next_item(-1, ListNextItemFlag::All, ListItemState::Selected)
    }

    /// Sets the image for a specific item.
    ///
    /// # Arguments
    /// * `item_index` - The 0-based index of the item.
    /// * `image_index` - The index of the image in the image list.
    ///
    /// # Returns
    /// `true` if successful, `false` otherwise.
    pub fn set_item_image(&self, item_index: i64, image_index: i32) -> bool {
        unsafe {
            ffi::wxd_ListCtrl_SetItemImageIndex(
                self.as_list_ctrl_ptr(),
                item_index as c_longlong,
                image_index,
            )
        }
    }

    /// Deletes the specified item.
    pub fn delete_item(&self, item: i64) -> bool {
        unsafe { ffi::wxd_ListCtrl_DeleteItem(self.as_list_ctrl_ptr(), item as c_longlong) }
    }

    /// Deletes all items from the list control.
    pub fn delete_all_items(&self) -> bool {
        unsafe { ffi::wxd_ListCtrl_DeleteAllItems(self.as_list_ctrl_ptr()) }
    }

    /// Deletes all items and columns from the list control.
    pub fn clear_all(&self) -> bool {
        unsafe { ffi::wxd_ListCtrl_ClearAll(self.as_list_ctrl_ptr()) }
    }

    /// Gets the number of selected items.
    pub fn get_selected_item_count(&self) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetSelectedItemCount(self.as_list_ctrl_ptr()) }
    }

    /// Ensures that the specified item is visible.
    pub fn ensure_visible(&self, item: i64) -> bool {
        unsafe { ffi::wxd_ListCtrl_EnsureVisible(self.as_list_ctrl_ptr(), item as c_longlong) }
    }

    /// Determines which item, if any, is at the specified point.
    /// Returns a tuple (item_index, flags, subitem_index).
    pub fn hit_test(&self, point: Point) -> (i32, i32, i32) {
        let mut flags: i32 = 0;
        let mut subitem: c_longlong = 0;
        let item = unsafe {
            ffi::wxd_ListCtrl_HitTest(
                self.as_list_ctrl_ptr(),
                point.into(),
                &mut flags as *mut i32,
                &mut subitem as *mut c_longlong,
            )
        };
        (item, flags, subitem as i32)
    }

    /// Starts editing the label of the specified item.
    ///
    /// # Returns
    /// Returns the TextCtrl that will be used to edit the label.
    /// The caller does not own this TextCtrl; it will be deleted automatically
    /// when editing is finished.
    pub fn edit_label(&self, item: i64) -> crate::widgets::textctrl::TextCtrl {
        let ptr =
            unsafe { ffi::wxd_ListCtrl_EditLabel(self.as_list_ctrl_ptr(), item as c_longlong) };

        if ptr.is_null() {
            panic!("Failed to start editing item label: FFI returned null pointer.");
        }

        unsafe { crate::widgets::textctrl::TextCtrl::from_ptr(ptr) }
    }

    // --- Item Appearance Methods ---

    /// Sets the background color of an item.
    pub fn set_item_background_colour(&self, item: i64, colour: &crate::color::Colour) {
        unsafe {
            ffi::wxd_ListCtrl_SetItemBackgroundColour(
                self.as_list_ctrl_ptr(),
                item as c_longlong,
                (*colour).into(),
            )
        }
    }

    /// Sets the text color of an item.
    pub fn set_item_text_colour(&self, item: i64, colour: &crate::color::Colour) {
        unsafe {
            ffi::wxd_ListCtrl_SetItemTextColour(
                self.as_list_ctrl_ptr(),
                item as c_longlong,
                (*colour).into(),
            )
        }
    }

    /// Gets the background color of an item.
    pub fn get_item_background_colour(&self, item: i64) -> crate::color::Colour {
        unsafe {
            let c_colour = ffi::wxd_ListCtrl_GetItemBackgroundColour(
                self.as_list_ctrl_ptr(),
                item as c_longlong,
            );
            crate::color::Colour::from(c_colour)
        }
    }

    /// Gets the text color of an item.
    pub fn get_item_text_colour(&self, item: i64) -> crate::color::Colour {
        unsafe {
            let c_colour =
                ffi::wxd_ListCtrl_GetItemTextColour(self.as_list_ctrl_ptr(), item as c_longlong);
            crate::color::Colour::from(c_colour)
        }
    }

    // --- Column Management Methods ---

    /// Sets the custom order of columns.
    ///
    /// By default, the columns in a list control appear in order of their indices (0, 1, 2, ...).
    /// This method allows you to set a custom visual order for the columns.
    pub fn set_columns_order(&self, orders: &[i32]) -> bool {
        unsafe {
            ffi::wxd_ListCtrl_SetColumnsOrder(
                self.as_list_ctrl_ptr(),
                orders.len() as c_int,
                orders.as_ptr() as *mut c_int,
            )
        }
    }

    /// Gets the custom order of all columns.
    ///
    /// Returns a vector of column indices in their current display order.
    pub fn get_columns_order(&self) -> Vec<i32> {
        unsafe {
            let mut count: c_int = 0;
            let ptr = ffi::wxd_ListCtrl_GetColumnsOrder(
                self.as_list_ctrl_ptr(),
                &mut count as *mut c_int,
            );

            if ptr.is_null() || count <= 0 {
                return Vec::new();
            }

            let mut result = Vec::with_capacity(count as usize);
            for i in 0..count {
                result.push(*ptr.offset(i as isize));
            }

            // Free the memory allocated by the C function
            ffi::wxd_free_int_array(ptr);

            result
        }
    }

    /// Gets the position in which the given column is currently displayed.
    ///
    /// Returns the position where the column is currently shown, or -1 if an error occurred.
    pub fn get_column_order(&self, col: i32) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetColumnOrder(self.as_list_ctrl_ptr(), col) }
    }

    /// Gets the column index at the given display position.
    ///
    /// Returns the index of the column which is shown at the specified position, or -1 if an error occurred.
    pub fn get_column_index_from_order(&self, pos: i32) -> i32 {
        unsafe { ffi::wxd_ListCtrl_GetColumnIndexFromOrder(self.as_list_ctrl_ptr(), pos) }
    }

    // --- Virtual List Support Methods ---

    /// Sets the number of items in a virtual list control.
    ///
    /// Must be used with a list control created with the `ListCtrlStyle::Virtual` style.
    pub fn set_item_count(&self, count: i64) {
        unsafe { ffi::wxd_ListCtrl_SetItemCount(self.as_list_ctrl_ptr(), count as c_longlong) }
    }

    /// Refreshes a single item in a virtual list control.
    pub fn refresh_item(&self, item: i64) {
        unsafe { ffi::wxd_ListCtrl_RefreshItem(self.as_list_ctrl_ptr(), item as c_longlong) }
    }

    /// Refreshes a range of items in a virtual list control.
    pub fn refresh_items(&self, item_from: i64, item_to: i64) {
        unsafe {
            ffi::wxd_ListCtrl_RefreshItems(
                self.as_list_ctrl_ptr(),
                item_from as c_longlong,
                item_to as c_longlong,
            )
        }
    }

    // --- ImageList Methods ---

    /// Sets the image list for the control.
    /// The ListCtrl takes ownership of the ImageList.
    ///
    /// # Arguments
    /// * `image_list` - The ImageList to set.
    /// * `list_type` - Which image list to set (e.g., `image_list_type::NORMAL`, `image_list_type::SMALL`).
    pub fn set_image_list(&self, image_list: ImageList, list_type: i32) {
        unsafe {
            ffi::wxd_ListCtrl_SetImageList(
                self.as_list_ctrl_ptr(),
                image_list.as_ptr(), // Pass the raw pointer
                list_type,
            );
        }
        // wxWidgets takes ownership of the image list
        std::mem::forget(image_list);
    }

    /// Gets the image list associated with the control.
    /// The ListCtrl owns the ImageList, so the caller should not delete it.
    ///
    /// # Arguments
    /// * `list_type` - Which image list to get (e.g., `image_list_type::NORMAL`, `image_list_type::SMALL`).
    ///
    /// # Returns
    /// An Option containing the ImageList if it exists, otherwise None.
    pub fn get_image_list(&self, list_type: i32) -> Option<ImageList> {
        let ptr = unsafe { ffi::wxd_ListCtrl_GetImageList(self.as_list_ctrl_ptr(), list_type) };
        if ptr.is_null() {
            None
        } else {
            // The ImageList is owned by wxWidgets, so create an unowned wrapper
            Some(unsafe { ImageList::from_ptr_unowned(ptr) })
        }
    }
}

// Implement the HasItemData trait for ListCtrl
impl HasItemData for ListCtrl {
    fn set_custom_data<T: Any + Send + Sync + 'static>(
        &self,
        item_id: impl Into<u64>,
        data: T,
    ) -> u64 {
        let item_index = item_id.into() as i64;

        // First check if there's already data associated with this item
        let existing_data_id = unsafe {
            ffi::wxd_ListCtrl_GetItemData(self.as_list_ctrl_ptr(), item_index as c_longlong) as u64
        };

        // If we have existing data, remove it from the registry
        if existing_data_id != 0 {
            let _ = remove_item_data(existing_data_id);
        }

        // Store the new data in the registry and get a unique ID
        let data_id = store_item_data(data);

        // Store the ID as an integer in the list item using the native set_item_data
        let result = unsafe {
            ffi::wxd_ListCtrl_SetItemData(
                self.as_list_ctrl_ptr(),
                item_index as c_longlong,
                data_id as c_longlong,
            )
        };

        // If setting failed, remove the data from the registry and return 0
        if !result {
            let _ = remove_item_data(data_id);
            return 0;
        }

        data_id
    }

    fn get_custom_data(&self, item_id: impl Into<u64>) -> Option<Arc<dyn Any + Send + Sync>> {
        let item_index = item_id.into() as i64;

        // Get the data ID using the native get_item_data
        let data_id = unsafe {
            ffi::wxd_ListCtrl_GetItemData(self.as_list_ctrl_ptr(), item_index as c_longlong) as u64
        };

        if data_id == 0 {
            return None;
        }

        // Look up the data in the registry
        get_item_data(data_id)
    }

    fn has_custom_data(&self, item_id: impl Into<u64>) -> bool {
        let item_index = item_id.into() as i64;

        // Get the data ID using the native get_item_data
        let data_id = unsafe {
            ffi::wxd_ListCtrl_GetItemData(self.as_list_ctrl_ptr(), item_index as c_longlong) as u64
        };

        // If the ID is non-zero and exists in the registry, there is custom data
        data_id != 0 && get_item_data(data_id).is_some()
    }

    fn clear_custom_data(&self, item_id: impl Into<u64>) -> bool {
        let item_index = item_id.into() as i64;

        // Get the data ID using the native get_item_data
        let data_id = unsafe {
            ffi::wxd_ListCtrl_GetItemData(self.as_list_ctrl_ptr(), item_index as c_longlong) as u64
        };

        // Only attempt to remove data if there's actually data to remove
        if data_id != 0 {
            // Remove the data from the registry
            let _ = remove_item_data(data_id);
        }

        // Clear the data in the list item by setting it to 0
        unsafe {
            ffi::wxd_ListCtrl_SetItemData(self.as_list_ctrl_ptr(), item_index as c_longlong, 0)
        }
    }

    fn cleanup_all_custom_data(&self) {
        // Get the total number of items in the list control
        let item_count = self.get_item_count();

        // Iterate through all items and clear their custom data
        for i in 0..item_count {
            self.clear_custom_data(i as u64);
        }
    }
}

// Implement common widget traits
implement_widget_traits_with_target!(ListCtrl, window, Window);

// Implement scrolling functionality for ListCtrl
impl crate::scrollable::WxScrollable for ListCtrl {}

// Use the widget_builder macro for ListCtrl
widget_builder!(
    name: ListCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: ListCtrlStyle,
    fields: {},
    build_impl: |slf| {
        let list_ctrl = ListCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits()
        );

        // Set up cleanup for custom data
        list_ctrl.setup_cleanup();

        list_ctrl
    }
);

// Register for destroy event to clean up custom data
impl ListCtrl {
    /// Sets up the ListCtrl to clean up all custom data when it's destroyed.
    /// This is automatically called during construction.
    fn setup_cleanup(&self) {
        // Create a clone for the closure
        let list_ctrl_clone = self.clone();

        // Bind to the DESTROY event for proper cleanup when the window is destroyed
        self.on_destroy(move |_event| {
            // Clean up all custom data when the control is destroyed
            list_ctrl_clone.cleanup_all_custom_data();
        });
    }

    /// Manually clean up all custom data associated with this ListCtrl.
    /// This can be called explicitly when needed.
    pub fn cleanup_custom_data(&self) {
        self.cleanup_all_custom_data();
    }
}

// Implement event handlers for ListCtrl
crate::implement_widget_local_event_handlers!(
    ListCtrl,
    ListCtrlEvent,
    ListCtrlEventData,
    ItemSelected => item_selected, EventType::LIST_ITEM_SELECTED,
    ItemDeselected => item_deselected, EventType::LIST_ITEM_DESELECTED,
    ItemActivated => item_activated, EventType::LIST_ITEM_ACTIVATED,
    ItemFocused => item_focused, EventType::LIST_ITEM_FOCUSED,
    ColumnClick => column_click, EventType::LIST_COL_CLICK,
    ColumnRightClick => column_right_click, EventType::LIST_COL_RIGHT_CLICK,
    ColumnBeginDrag => column_begin_drag, EventType::LIST_COL_BEGIN_DRAG,
    BeginLabelEdit => begin_label_edit, EventType::LIST_BEGIN_LABEL_EDIT,
    EndLabelEdit => end_label_edit, EventType::LIST_END_LABEL_EDIT,
    BeginDrag => begin_drag, EventType::LIST_BEGIN_DRAG,
    BeginRDrag => begin_right_drag, EventType::LIST_BEGIN_RDRAG,
    DeleteItem => delete_item_event, EventType::LIST_DELETE_ITEM,
    DeleteAllItems => delete_all_items_event, EventType::LIST_DELETE_ALL_ITEMS,
    KeyDown => key_down, EventType::LIST_KEY_DOWN,
    InsertItem => insert_item_event, EventType::LIST_INSERT_ITEM,
    ItemRightClick => item_right_click, EventType::LIST_ITEM_RIGHT_CLICK,
    ItemMiddleClick => item_middle_click, EventType::LIST_ITEM_MIDDLE_CLICK
);

// Add XRC Support - enables ListCtrl to be created from XRC-managed pointers
impl_xrc_support!(ListCtrl, { window });

// Widget casting support for ListCtrl
impl_widget_cast!(ListCtrl, "wxListCtrl", { window });
