//! wxTreeCtrl wrapper
//!
//! The `TreeCtrl` widget provides a tree control for displaying hierarchical data.
//! This module uses the `HasItemData` trait for associating custom data with tree items.
//!
//! # Examples
//!
//! ```rust,no_run
//! use wxdragon::prelude::*;
//! use wxdragon::widgets::treectrl::{TreeCtrl, TreeCtrlStyle};
//! use wxdragon::HasItemData;
//!
//! // Create custom data to associate with tree items
//! #[derive(Clone)]
//! struct PersonData {
//!     name: String,
//!     age: u32,
//!     role: String,
//! }
//!
//! fn create_tree_with_data(parent: &dyn WxWidget) -> TreeCtrl {
//!     // Create a tree control
//!     let tree = TreeCtrl::builder(parent)
//!         .with_style(TreeCtrlStyle::HasButtons | TreeCtrlStyle::LinesAtRoot)
//!         .build();
//!
//!     // Add root with associated data
//!     let ceo = PersonData {
//!         name: "John Smith".to_string(),
//!         age: 52,
//!         role: "CEO".to_string(),
//!     };
//!     let root = tree.add_root_with_data("Company", ceo, None, None).unwrap();
//!
//!     // Add child with different data type
//!     tree.append_item_with_data(&root, "Budget", 1000000, None, None).unwrap();
//!
//!     // Add another child with string data
//!     tree.append_item_with_data(
//!         &root,
//!         "Mission",
//!         "To create amazing products".to_string(),
//!         None,
//!         None,
//!     ).unwrap();
//!
//!     // Later, when handling selection events:
//!     // if let Some(item_id) = tree.get_selection() {
//!     //     if let Some(data) = tree.get_custom_data(&item_id) {
//!     //         if let Some(person) = data.downcast_ref::<PersonData>() {
//!     //             println!("Selected person: {}", person.name);
//!     //         } else if let Some(budget) = data.downcast_ref::<i32>() {
//!     //             println!("Selected budget: ${}", budget);
//!     //         } else if let Some(text) = data.downcast_ref::<String>() {
//!     //             println!("Selected text: {}", text);
//!     //         }
//!     //     }
//!     // }
//!
//!     tree
//! }
//! ```

use std::any::Any;
use std::ffi::CString;
use std::ptr;
use std::sync::Arc;

use crate::event::{TreeEvents, WxEvtHandler};
// Base for some events
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::widgets::imagelist::ImageList;
use crate::widgets::item_data::{HasItemData, get_item_data, remove_item_data, store_item_data};
use crate::window::{WindowHandle, WxWidget};
use wxdragon_sys as ffi;

// --- TreeCtrl Styles ---
widget_style_enum!(
    name: TreeCtrlStyle,
    doc: "Style flags for TreeCtrl widget.",
    variants: {
        Default: ffi::WXD_TR_DEFAULT_STYLE, "Default style. Combines `HasButtons` and `LinesAtRoot`.",
        HasButtons: ffi::WXD_TR_HAS_BUTTONS, "Use buttons to show expand/collapse state.",
        LinesAtRoot: ffi::WXD_TR_LINES_AT_ROOT, "Use lines to show hierarchy at the root level.",
        NoLines: ffi::WXD_TR_NO_LINES, "Don't show any lines.",
        Single: ffi::WXD_TR_SINGLE, "Only allow a single item to be selected.",
        HideRoot: ffi::WXD_TR_HIDE_ROOT, "Hide the root item, making its children appear as top-level items.",
        EditLabels: ffi::WXD_TR_EDIT_LABELS, "Allow editing of item labels."
        // Add other TR_ styles as needed, e.g., TR_FULL_ROW_HIGHLIGHT, TR_MULTIPLE, etc.
        // TR_NO_BUTTONS = ffi::WXD_TR_NO_BUTTONS, (if available)
        // TR_ROW_LINES = ffi::WXD_TR_ROW_LINES, (if available)
        // TR_TWIST_BUTTONS = ffi::WXD_TR_TWIST_BUTTONS, (if available)
    },
    default_variant: Default
);

// --- TreeItemIcon Enum ---
/// Specifies which icon of a tree item is being referred to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)] // Matches wxd_TreeItemIconType_t which is an enum, typically int/u32
pub enum TreeItemIcon {
    Normal = ffi::wxd_TreeItemIconType_t_WXD_TreeItemIcon_Normal,
    Selected = ffi::wxd_TreeItemIconType_t_WXD_TreeItemIcon_Selected,
    Expanded = ffi::wxd_TreeItemIconType_t_WXD_TreeItemIcon_Expanded,
    SelectedExpanded = ffi::wxd_TreeItemIconType_t_WXD_TreeItemIcon_SelectedExpanded,
}

impl From<TreeItemIcon> for ffi::wxd_TreeItemIconType_t {
    fn from(icon: TreeItemIcon) -> Self {
        icon as ffi::wxd_TreeItemIconType_t
    }
}

// Represents the opaque wxTreeItemId used by wxWidgets.
// This struct owns the pointer returned by the C++ FFI functions
// and is responsible for freeing it via wxd_TreeItemId_Free.
#[derive(Debug)]
pub struct TreeItemId {
    ptr: *mut ffi::wxd_TreeItemId_t,
}

impl TreeItemId {
    // Creates a new TreeItemId from a raw pointer.
    // Assumes ownership of the pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_TreeItemId_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            // Add validation to ensure the C++ side returned a valid pointer
            let ptr_value = ptr as usize;

            // Basic sanity check on the pointer before accepting it
            if ptr_value.is_multiple_of(std::mem::align_of::<*mut std::ffi::c_void>())  // Properly aligned
                && ptr_value > 0x1000  // Not in null/low memory range
                && ptr_value < (usize::MAX / 2)
            // Not in kernel space
            {
                // Additional check: try to verify the TreeItemId is valid
                if unsafe { ffi::wxd_TreeItemId_IsOk(ptr) } {
                    Some(TreeItemId { ptr })
                } else {
                    log::warn!("Warning: C++ returned invalid TreeItemId pointer {ptr:p}, rejecting");
                    // Free the invalid pointer since we were supposed to take ownership
                    unsafe { ffi::wxd_TreeItemId_Free(ptr) };
                    None
                }
            } else {
                log::warn!("Warning: C++ returned corrupted TreeItemId pointer {ptr:p}, rejecting");
                None
            }
        }
    }

    // Checks if the underlying wxTreeItemId is valid.
    pub fn is_ok(&self) -> bool {
        unsafe { ffi::wxd_TreeItemId_IsOk(self.ptr) }
    }

    // Returns the raw pointer - use with caution.
    pub(crate) fn as_ptr(&self) -> *mut ffi::wxd_TreeItemId_t {
        self.ptr
    }
}

impl Clone for TreeItemId {
    fn clone(&self) -> Self {
        let clone_ptr = unsafe { ffi::wxd_TreeItemId_Clone(self.ptr) };
        TreeItemId { ptr: clone_ptr }
    }
}

// Implement conversion to u64 for TreeItemId
impl From<&TreeItemId> for u64 {
    fn from(tree_item: &TreeItemId) -> Self {
        // We use the address of the TreeItemId itself as our value
        tree_item as *const _ as usize as u64
    }
}

impl Drop for TreeItemId {
    fn drop(&mut self) {
        // Only free if the pointer is not null.
        if !self.ptr.is_null() {
            unsafe {
                // In release mode, we're seeing crashes when calling C++ functions on TreeItemId pointers.
                // Let's be more defensive and add bounds checking.

                // Check if the pointer looks reasonable (not obviously corrupted)
                let ptr_value = self.ptr as usize;

                // Basic sanity check: pointer should be aligned and in a reasonable memory range
                // On macOS ARM64, user space addresses are typically in a specific range
                if ptr_value.is_multiple_of(std::mem::align_of::<*mut std::ffi::c_void>())  // Properly aligned
                    && ptr_value > 0x1000  // Not in null/low memory range
                    && ptr_value < (usize::MAX / 2)
                // Not in kernel space
                {
                    // Additional validation: check if the TreeItemId is valid before freeing
                    if ffi::wxd_TreeItemId_IsOk(self.ptr) {
                        // Tell the C++ side to free the WXD_TreeItemId_t struct.
                        ffi::wxd_TreeItemId_Free(self.ptr);
                    } else {
                        // TreeItemId is not valid, but still try to free the memory
                        // This might be a TreeItemId that was already invalidated
                        ffi::wxd_TreeItemId_Free(self.ptr);
                    }
                } else {
                    // Pointer looks corrupted, don't try to free it to avoid crashes
                    log::warn!(
                        "Warning: Dropping TreeItemId with corrupted pointer {:p}, not freeing to avoid crash",
                        self.ptr
                    );
                }
            }
            self.ptr = ptr::null_mut();
        }
    }
}

/// TreeIterationCookie is used to keep track of the state while iterating through children
pub struct TreeIterationCookie {
    cookie_ptr: *mut std::ffi::c_void,
}

impl TreeIterationCookie {
    /// Creates a new cookie
    fn new(cookie_ptr: *mut std::ffi::c_void) -> Self {
        Self { cookie_ptr }
    }

    /// Gets the raw pointer to the cookie
    fn as_ptr_mut(&mut self) -> *mut *mut std::ffi::c_void {
        &mut self.cookie_ptr as *mut *mut std::ffi::c_void
    }
}

impl Drop for TreeIterationCookie {
    fn drop(&mut self) {
        // NOTE: The cookie is automatically freed by the C++ side when iteration
        // completes (GetNextChild returns null), so we don't need to free it here.
        // Attempting to free it manually was causing memory safety issues because
        // the cookie is allocated with C++ 'new' but we were trying to free it
        // with Rust's Box allocator.

        // Just set to null for safety, but don't free
        self.cookie_ptr = ptr::null_mut();
    }
}

/// Represents the wxTreeCtrl widget.
///
/// TreeCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct TreeCtrl {
    handle: WindowHandle,
}

/// TreeCtrl implementation with tree traversal capabilities.
/// The following FFI functions are available for tree traversal:
/// - `wxd_TreeCtrl_GetRootItem`: Get the root item
/// - `wxd_TreeCtrl_GetFirstChild`: Get the first child of an item
/// - `wxd_TreeCtrl_GetNextChild`: Get the next child of an item using the same cookie
/// - `wxd_TreeCtrl_GetNextSibling`: Get the next sibling of an item
/// - `wxd_TreeCtrl_GetChildrenCount`: Get the number of children of an item
impl TreeCtrl {
    /// Creates a new TreeCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> TreeCtrlBuilder<'_> {
        TreeCtrlBuilder::new(parent)
    }

    /// Internal implementation used by the builder.
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, pos: Point, size: Size, style: i64) -> Self {
        assert!(!parent_ptr.is_null(), "TreeCtrl parent cannot be null");

        let ctrl_ptr = unsafe { ffi::wxd_TreeCtrl_Create(parent_ptr, id, pos.into(), size.into(), style as ffi::wxd_Style_t) };

        if ctrl_ptr.is_null() {
            panic!("Failed to create wxTreeCtrl");
        }

        let tree_ctrl = TreeCtrl {
            handle: WindowHandle::new(ctrl_ptr as *mut ffi::wxd_Window_t),
        };

        // Set up cleanup for custom data
        tree_ctrl.setup_cleanup();

        tree_ctrl
    }

    /// Helper to get raw TreeCtrl pointer, returns null if widget has been destroyed
    #[inline]
    fn treectrl_ptr(&self) -> *mut ffi::wxd_TreeCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_TreeCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Adds the root item to the tree control.
    ///
    /// # Arguments
    /// * `text` - The text label for the root item.
    /// * `image` - Optional index of the image for the item (normal state).
    /// * `selected_image` - Optional index of the image for the item when selected.
    ///
    /// Returns the new item ID, or None if creation failed.
    pub fn add_root(&self, text: &str, image: Option<i32>, selected_image: Option<i32>) -> Option<TreeItemId> {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return None;
        }
        let c_text = CString::new(text).unwrap_or_default();
        let img = image.unwrap_or(-1);
        let sel_img = selected_image.unwrap_or(-1);
        let item_ptr = unsafe { ffi::wxd_TreeCtrl_AddRoot(ptr, c_text.as_ptr(), img, sel_img, ptr::null_mut()) };
        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    /// Adds the root item to the tree control with associated data.
    ///
    /// # Arguments
    /// * `text` - The text label for the root item.
    /// * `data` - Custom data to associate with the item.
    /// * `image` - Optional index of the image for the item (normal state).
    /// * `selected_image` - Optional index of the image for the item when selected.
    ///
    /// Returns the new item ID, or None if creation failed.
    pub fn add_root_with_data<T: Any + Send + Sync + 'static>(
        &self,
        text: &str,
        data: T,
        image: Option<i32>,
        selected_image: Option<i32>,
    ) -> Option<TreeItemId> {
        let root_item = self.add_root(text, image, selected_image)?;
        self.set_custom_data_direct(&root_item, data);
        Some(root_item)
    }

    /// Appends an item to the given parent item.
    ///
    /// # Arguments
    /// * `parent` - The parent item.
    /// * `text` - The text label for the new item.
    /// * `image` - Optional index of the image for the item (normal state).
    /// * `selected_image` - Optional index of the image for the item when selected.
    ///
    /// Returns the new item ID, or None if creation failed.
    pub fn append_item(
        &self,
        parent: &TreeItemId,
        text: &str,
        image: Option<i32>,
        selected_image: Option<i32>,
    ) -> Option<TreeItemId> {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return None;
        }
        let t = CString::new(text).unwrap_or_default();
        let img = image.unwrap_or(-1);
        let sel_img = selected_image.unwrap_or(-1);
        let item_ptr = unsafe { ffi::wxd_TreeCtrl_AppendItem(ptr, parent.as_ptr(), t.as_ptr(), img, sel_img, ptr::null_mut()) };
        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    /// Appends an item to the given parent item with associated data.
    ///
    /// # Arguments
    /// * `parent` - The parent item.
    /// * `text` - The text label for the new item.
    /// * `data` - Custom data to associate with the item.
    /// * `image` - Optional index of the image for the item (normal state).
    /// * `selected_image` - Optional index of the image for the item when selected.
    ///
    /// Returns the new item ID, or None if creation failed.
    pub fn append_item_with_data<T: Any + Send + Sync + 'static>(
        &self,
        parent: &TreeItemId,
        text: &str,
        data: T,
        image: Option<i32>,
        selected_image: Option<i32>,
    ) -> Option<TreeItemId> {
        let item = self.append_item(parent, text, image, selected_image)?;
        self.set_custom_data_direct(&item, data);
        Some(item)
    }

    /// Deletes the specified item and all its children.
    /// Note: The passed TreeItemId becomes invalid after this call.
    pub fn delete(&self, item: &TreeItemId) {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return;
        }
        // Clean up any attached data before deleting the item
        let _ = self.clear_custom_data(item);

        unsafe { ffi::wxd_TreeCtrl_Delete(ptr, item.as_ptr()) };
    }

    /// Gets the currently selected item.
    /// Returns None if no item is selected or on error.
    pub fn get_selection(&self) -> Option<TreeItemId> {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return None;
        }
        let item_ptr = unsafe { ffi::wxd_TreeCtrl_GetSelection(ptr) };
        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    /// Selects the given item.
    pub fn select_item(&self, item: &TreeItemId) {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_TreeCtrl_SelectItem(ptr, item.as_ptr());
        }
    }

    /// Expands the given item to show its children.
    pub fn expand(&self, item: &TreeItemId) {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TreeCtrl_Expand(ptr, item.as_ptr()) };
    }

    /// Sets up the TreeCtrl to clean up all custom data when it's destroyed.
    /// This is automatically called during construction.
    fn setup_cleanup(&self) {
        use crate::event::EventType;

        // Create a clone for the closure
        let tree_ctrl_clone = *self;

        // Bind to the DESTROY event for proper cleanup when the window is destroyed
        self.bind_internal(EventType::DESTROY, move |_event| {
            // Clean up all custom data when the control is destroyed
            tree_ctrl_clone.cleanup_all_custom_data();
        });
    }

    /// Manually clean up all custom data associated with this TreeCtrl.
    /// This can be called explicitly when needed, but is automatically
    /// called when the TreeCtrl is destroyed.
    pub fn cleanup_custom_data(&self) {
        self.cleanup_all_custom_data();
    }

    /// Gets the root item of the tree
    pub fn get_root_item(&self) -> Option<TreeItemId> {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return None;
        }
        let item_ptr = unsafe { ffi::wxd_TreeCtrl_GetRootItem(ptr) };
        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    /// Gets the first child of the specified item.
    /// Returns None if the item has no children.
    ///
    /// This also returns a cookie that must be used for subsequent calls to get_next_child.
    pub fn get_first_child(&self, item: &TreeItemId) -> Option<(TreeItemId, TreeIterationCookie)> {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return None;
        }
        let mut cookie_ptr = ptr::null_mut();
        let child_ptr =
            unsafe { ffi::wxd_TreeCtrl_GetFirstChild(ptr, item.as_ptr(), &mut cookie_ptr as *mut *mut std::ffi::c_void) };

        let child = unsafe { TreeItemId::from_ptr(child_ptr) }?;
        let cookie = TreeIterationCookie::new(cookie_ptr);

        Some((child, cookie))
    }

    /// Gets the next child of an item using a cookie from a previous call to get_first_child
    /// or get_next_child.
    ///
    /// Returns None when there are no more children.
    pub fn get_next_child(&self, item: &TreeItemId, cookie: &mut TreeIterationCookie) -> Option<TreeItemId> {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return None;
        }
        let child_ptr = unsafe { ffi::wxd_TreeCtrl_GetNextChild(ptr, item.as_ptr(), cookie.as_ptr_mut()) };

        unsafe { TreeItemId::from_ptr(child_ptr) }
    }

    /// Gets the next sibling of the specified item.
    /// Returns None if the item has no next sibling.
    pub fn get_next_sibling(&self, item: &TreeItemId) -> Option<TreeItemId> {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return None;
        }
        let sibling_ptr = unsafe { ffi::wxd_TreeCtrl_GetNextSibling(ptr, item.as_ptr()) };

        unsafe { TreeItemId::from_ptr(sibling_ptr) }
    }

    /// Gets the number of children of the specified item.
    ///
    /// # Parameters
    ///
    /// * `item` - The item to check
    /// * `recursively` - If true, count all descendants, not just immediate children
    ///
    /// # Returns
    ///
    /// The number of children (or descendants if recursively is true)
    pub fn get_children_count(&self, item: &TreeItemId, recursively: bool) -> usize {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_TreeCtrl_GetChildrenCount(ptr, item.as_ptr(), recursively) as usize }
    }

    // --- ImageList and Item Image Methods ---

    /// Sets the image list for the tree control.
    /// The tree control takes ownership of the image list.
    pub fn set_image_list(&self, image_list: ImageList) {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_TreeCtrl_SetImageList(ptr, image_list.as_ptr());
        }
        // wxTreeCtrl takes ownership of the ImageList
        std::mem::forget(image_list);
    }

    /// Gets the image list associated with the tree control.
    /// The tree control owns the image list, so the caller should not delete it.
    pub fn get_image_list(&self) -> Option<ImageList> {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return None;
        }
        let img_ptr = unsafe { ffi::wxd_TreeCtrl_GetImageList(ptr) };
        if img_ptr.is_null() {
            None
        } else {
            Some(unsafe { ImageList::from_ptr_unowned(img_ptr) })
        }
    }

    /// Sets the image for the given item.
    pub fn set_item_image(&self, item: &TreeItemId, image_index: i32, icon_type: TreeItemIcon) {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TreeCtrl_SetItemImage(ptr, item.as_ptr(), image_index, icon_type.into()) };
    }

    /// Gets the image for the given item.
    /// Returns -1 if no image is associated with the item for the given type.
    pub fn get_item_image(&self, item: &TreeItemId, icon_type: TreeItemIcon) -> i32 {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_TreeCtrl_GetItemImage(ptr, item.as_ptr(), icon_type.into()) }
    }
}

// Implement HasItemData trait for TreeCtrl
impl HasItemData for TreeCtrl {
    fn set_custom_data<T: Any + Send + Sync + 'static>(&self, item_id: impl Into<u64>, data: T) -> u64 {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return 0;
        }

        // Convert from the generic item_id
        let item_id = item_id.into();

        // For TreeCtrl, we need the actual TreeItemId, not just a u64 representation
        // Get the concrete TreeItemId if that's what was passed
        if let Some(tree_item) = self.get_concrete_tree_item_id(item_id) {
            // First check if there's already data associated with this item
            let existing_data_id = unsafe { ffi::wxd_TreeCtrl_GetItemData(ptr, tree_item.as_ptr()) as u64 };

            // If we have existing data, remove it from the registry
            if existing_data_id != 0 {
                let _ = remove_item_data(existing_data_id);
            }

            // Store the new data in the registry
            let data_id = store_item_data(data);

            // Store the data_id in wxWidgets via the C++ FFI
            unsafe {
                ffi::wxd_TreeCtrl_SetItemData(ptr, tree_item.as_ptr(), data_id as i64);
            }

            return data_id;
        }

        // If we couldn't get a valid TreeItemId, return 0 (failure)
        0
    }

    fn get_custom_data(&self, item_id: impl Into<u64>) -> Option<Arc<dyn Any + Send + Sync>> {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return None;
        }

        // Convert from the generic item_id
        let item_id = item_id.into();

        // Get the concrete TreeItemId if that's what was passed
        let tree_item = self.get_concrete_tree_item_id(item_id)?;

        // Get the data ID from wxWidgets
        let data_id = unsafe { ffi::wxd_TreeCtrl_GetItemData(ptr, tree_item.as_ptr()) as u64 };

        if data_id == 0 {
            return None;
        }

        // Look up the data in the registry
        get_item_data(data_id)
    }

    fn has_custom_data(&self, item_id: impl Into<u64>) -> bool {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return false;
        }

        // Convert from the generic item_id
        let item_id = item_id.into();

        // Get the concrete TreeItemId if that's what was passed
        if let Some(tree_item) = self.get_concrete_tree_item_id(item_id) {
            // Check if this item has data via wxWidgets
            let data_id = unsafe { ffi::wxd_TreeCtrl_GetItemData(ptr, tree_item.as_ptr()) as u64 };

            return data_id != 0 && get_item_data(data_id).is_some();
        }

        false
    }

    fn clear_custom_data(&self, item_id: impl Into<u64>) -> bool {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return false;
        }

        // Convert from the generic item_id
        let item_id = item_id.into();

        // Get the concrete TreeItemId if that's what was passed
        if let Some(tree_item) = self.get_concrete_tree_item_id(item_id) {
            // Get the data ID from wxWidgets
            let data_id = unsafe { ffi::wxd_TreeCtrl_GetItemData(ptr, tree_item.as_ptr()) as u64 };

            if data_id != 0 {
                // Remove the data from the registry
                let _ = remove_item_data(data_id);

                // Clear the association in wxWidgets
                unsafe {
                    ffi::wxd_TreeCtrl_SetItemData(ptr, tree_item.as_ptr(), 0);
                }

                return true;
            }
        }

        false
    }

    fn cleanup_all_custom_data(&self) {
        // Get the root item
        let root = match self.get_root_item() {
            Some(root) => root,
            None => {
                return;
            }
        };

        // Recursively clean up the root and all its children
        self.clean_item_and_children(&root);
    }
}

// Helper methods for TreeCtrl
impl TreeCtrl {
    // This is a special method to handle the case of getting a TreeItemId from something
    // that implements Into<u64>. It handles different ways the item might be passed.
    fn get_concrete_tree_item_id(&self, _id: u64) -> Option<TreeItemId> {
        // Handle the case where we're given a reference to an existing TreeItemId
        // The id value will be the memory address of the TreeItemId reference
        if _id > u32::MAX as u64 {
            // Try to interpret it as a reference to an existing TreeItemId
            let ptr = _id as usize as *const TreeItemId;

            // Add extensive safety checks
            if !ptr.is_null() {
                // Check if the pointer looks reasonable (aligned and in valid memory range)
                let ptr_value = ptr as usize;
                if ptr_value.is_multiple_of(std::mem::align_of::<TreeItemId>())  // Properly aligned
                    && ptr_value > 0x1000  // Not in null/low memory range
                    && ptr_value < (usize::MAX / 2)
                // Not in kernel space (macOS ARM64)
                {
                    unsafe {
                        // Try to dereference the pointer carefully
                        let possible_tree_item = &*ptr;

                        // Validate that the TreeItemId's internal pointer looks reasonable
                        let internal_ptr = possible_tree_item.ptr as usize;
                        if !possible_tree_item.ptr.is_null()
                            && internal_ptr.is_multiple_of(std::mem::align_of::<*mut std::ffi::c_void>())
                            && internal_ptr > 0x1000
                            && internal_ptr < (usize::MAX / 2)
                        {
                            // Final validation: check if the TreeItemId is actually valid
                            if ffi::wxd_TreeItemId_IsOk(possible_tree_item.ptr) {
                                // Clone it to avoid lifetime issues
                                let clone_ptr = ffi::wxd_TreeItemId_Clone(possible_tree_item.ptr);
                                if !clone_ptr.is_null() {
                                    return Some(TreeItemId { ptr: clone_ptr });
                                }
                            }
                        }
                    }
                }
            }
        }

        // For smaller values, handle special cases
        match _id {
            // Special case for getting root item
            1 => self.get_root_item(),
            // Special case for getting selection
            2 => self.get_selection(),
            _ => None,
        }
    }

    /// Recursively clean up the item and all its children
    fn clean_item_and_children(&self, item: &TreeItemId) {
        // Check if this item has any children
        if self.get_children_count(item, false) == 0 {
            // No children, we're done with this branch
            return;
        }

        // Get the first child
        if let Some((first_child, mut cookie)) = self.get_first_child(item) {
            // Clean up the first child and its descendants
            self.clean_item_and_children(&first_child);

            // Process all remaining children
            while let Some(next_child) = self.get_next_child(item, &mut cookie) {
                self.clean_item_and_children(&next_child);
            }
        }
    }

    /// Direct method to set custom data on a TreeItemId without going through u64 conversion.
    /// This is safer than the trait method when you have a direct TreeItemId reference.
    pub fn set_custom_data_direct<T: Any + Send + Sync + 'static>(&self, item_id: &TreeItemId, data: T) -> u64 {
        let ptr = self.treectrl_ptr();
        if ptr.is_null() {
            return 0;
        }

        // First check if there's already data associated with this item
        let existing_data_id = unsafe { ffi::wxd_TreeCtrl_GetItemData(ptr, item_id.as_ptr()) as u64 };

        // If we have existing data, remove it from the registry
        if existing_data_id != 0 {
            let _ = remove_item_data(existing_data_id);
        }

        // Store the new data in the registry
        let data_id = store_item_data(data);

        // Store the data_id in wxWidgets via the C++ FFI
        unsafe {
            ffi::wxd_TreeCtrl_SetItemData(ptr, item_id.as_ptr(), data_id as i64);
        }

        data_id
    }
}

// Manual WxWidget implementation for TreeCtrl (using WindowHandle)
impl WxWidget for TreeCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for TreeCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for TreeCtrl {}

// Implement scrolling functionality for TreeCtrl
impl crate::scrollable::WxScrollable for TreeCtrl {}

// Use the widget_builder macro for TreeCtrl
widget_builder!(
    name: TreeCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: TreeCtrlStyle,
    fields: {},
    build_impl: |slf| {
        TreeCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
);

// At the bottom of the file, add the TreeEvents trait implementation
impl TreeEvents for TreeCtrl {}

// XRC Support - enables TreeCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for TreeCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        TreeCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for TreeCtrl
impl crate::window::FromWindowWithClassName for TreeCtrl {
    fn class_name() -> &'static str {
        "wxTreeCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        TreeCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
