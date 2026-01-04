//! DataViewTreeCtrl implementation.

use crate::event::WxEvtHandler;
use crate::widgets::dataview::item::DataViewItem;
use crate::widgets::imagelist::ImageList;
use crate::window::{WindowHandle, WxWidget};
use crate::{Id, Point, Size};
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;
// Import necessary types for columns from parent dataview module
use super::DataViewModel;
use super::column::DataViewColumn;
use super::enums::{DataViewAlign, DataViewCellMode, DataViewColumnFlags};
use super::renderer::{DataViewIconTextRenderer, DataViewTextRenderer};
use super::variant::VariantType;

// Styles for DataViewTreeCtrl (currently uses general DataViewCtrl styles)
// If specific styles are needed, they can be added here.
// For now, we'll use a placeholder style enum or rely on DataViewCtrl's styles.
widget_style_enum! {
    name: DataViewTreeCtrlStyle,
    doc: "Style flags for DataViewTreeCtrl widget.",
    variants: {
        Default: 0, "Default style.",
        DvMultiple: ffi::WXD_DV_MULTIPLE, "Allow multiple selections.",
        DvRowLines: ffi::WXD_DV_ROW_LINES, "Show row lines.",
        DvHorizRules: ffi::WXD_DV_HORIZ_RULES, "Show horizontal rules (same as DvRowLines).",
        DvVariableLineHeight: ffi::WXD_DV_VARIABLE_LINE_HEIGHT, "Allow variable line height."
    },
    default_variant: Default
}

// REMOVE UNUSED DOC COMMENT THAT WAS FOR THE DELETED ImageListPtr
// /// Opaque pointer for ImageList. For now, using a raw pointer.
// pub type ImageListPtr = *mut ffi::wxd_ImageList_t; // REMOVE THIS

widget_builder! {
    name: DataViewTreeCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: DataViewTreeCtrlStyle,
    fields: {
        label: String = String::new()
    },
    build_impl: |slf| {
        DataViewTreeCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            &slf.label,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
}

/// Represents a wxDataViewTreeCtrl.
///
/// DataViewTreeCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct DataViewTreeCtrl {
    /// Safe handle to the underlying wxDataViewTreeCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl DataViewTreeCtrl {
    /// Creates a new builder for a DataViewTreeCtrl.
    pub fn builder<'a>(parent: &'a dyn WxWidget) -> DataViewTreeCtrlBuilder<'a> {
        DataViewTreeCtrlBuilder::new(parent)
    }

    /// Internal implementation used by the builder.
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, label: &str, pos: Point, size: Size, style: i64) -> Self {
        let label_c_str = CString::new(label).unwrap_or_default();
        let ptr = unsafe {
            ffi::wxd_DataViewTreeCtrl_new(
                parent_ptr,
                id,
                pos.into(),
                size.into(),
                style,
                std::ptr::null_mut(),
                label_c_str.as_ptr(),
            )
        };
        if ptr.is_null() {
            panic!("Failed to create DataViewTreeCtrl");
        }
        DataViewTreeCtrl {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Helper to get raw window pointer, returns null if widget has been destroyed
    #[inline]
    fn dvtc_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    /// Returns the underlying WindowHandle for this DataViewTreeCtrl.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    // --- Column Management (inherited from DataViewCtrl conceptually) ---
    /// Appends a pre-created column to the control.
    pub fn append_column(&self, column: &DataViewColumn) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_AppendColumn(self.dvtc_ptr(), column.as_raw()) }
    }

    /// Prepends a column to the control.
    pub fn prepend_column(&self, column: &DataViewColumn) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_PrependColumn(self.dvtc_ptr(), column.as_raw()) }
    }

    /// Inserts a column at the specified position.
    pub fn insert_column(&self, pos: usize, column: &DataViewColumn) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_InsertColumn(self.dvtc_ptr(), pos as i64, column.as_raw()) }
    }

    /// Remove all columns
    pub fn clear_columns(&self) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_ClearColumns(self.dvtc_ptr()) }
    }

    /// Gets the column that currently displays the expander buttons.
    pub fn get_expander_column(&self) -> Option<DataViewColumn> {
        unsafe {
            let col_ptr = ffi::wxd_DataViewCtrl_GetExpanderColumn(self.dvtc_ptr());
            if col_ptr.is_null() {
                None
            } else {
                // DataViewColumn::from_ptr takes ownership if the C++ side allocated it and passed it.
                // If GetExpanderColumn returns a pointer to an existing column owned by wxWidgets,
                // then from_ptr (which calls wxd_DataViewColumn_Release on drop) might be incorrect.
                // This needs careful FFI contract consideration.
                // For now, assume from_ptr is the intended way to wrap an existing C++ object if it
                // effectively means taking over a reference or if the C++ side expects release.
                Some(DataViewColumn::from_ptr(col_ptr))
            }
        }
    }

    /// Sets the column that will display the expander buttons.
    pub fn set_expander_column(&self, column: &DataViewColumn) {
        unsafe { ffi::wxd_DataViewCtrl_SetExpanderColumn(self.dvtc_ptr(), column.as_raw()) }
    }

    /// Gets the column at the given position (0-indexed).
    pub fn get_column(&self, pos: usize) -> Option<DataViewColumn> {
        unsafe {
            let col_ptr = ffi::wxd_DataViewCtrl_GetColumn(self.dvtc_ptr(), pos as u32);
            if col_ptr.is_null() {
                None
            } else {
                // See notes in get_expander_column about DataViewColumn::from_ptr ownership
                Some(DataViewColumn::from_ptr(col_ptr))
            }
        }
    }

    /// Creates and appends a text column to this control.
    pub fn append_text_column(
        &self,
        label: &str,
        model_column: u32,
        width: i32,
        align: DataViewAlign,
        flags: DataViewColumnFlags,
    ) -> bool {
        let renderer = DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, align);
        let column = DataViewColumn::new(label, &renderer, model_column as usize, width, align, flags);
        self.append_column(&column)
    }

    /// Creates and appends an icon+text column to this control.
    pub fn append_icon_text_column(
        &self,
        label: &str,
        model_column: i32,
        width: i32,
        align: DataViewAlign,
        flags: DataViewColumnFlags,
    ) -> bool {
        let renderer = DataViewIconTextRenderer::new(VariantType::IconText, DataViewCellMode::Inert, align);
        let column = DataViewColumn::new(label, &renderer, model_column as usize, width, align, flags);
        self.append_column(&column)
    }

    /// Associates a data model with this DataViewTreeCtrl.
    ///
    /// Mirrors DataViewCtrl::associate_model. The model must outlive the control.
    pub fn associate_model<M: DataViewModel>(&self, model: &M) -> bool {
        let model_ptr = model.handle_ptr();
        unsafe { ffi::wxd_DataViewCtrl_AssociateModel(self.dvtc_ptr(), model_ptr) }
    }

    // --- Item Management ---
    pub fn append_item(&self, parent: &DataViewItem, text: &str, icon: i32) -> DataViewItem {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            let raw_item = ffi::wxd_DataViewTreeCtrl_AppendItem(self.dvtc_ptr(), **parent, text_c_str.as_ptr(), icon);
            DataViewItem::from(raw_item)
        }
    }

    pub fn append_container(&self, parent: &DataViewItem, text: &str, icon: i32, expanded_icon: i32) -> DataViewItem {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            let raw_item =
                ffi::wxd_DataViewTreeCtrl_AppendContainer(self.dvtc_ptr(), **parent, text_c_str.as_ptr(), icon, expanded_icon);
            DataViewItem::from(raw_item)
        }
    }

    pub fn prepend_item(&self, parent: &DataViewItem, text: &str, icon: i32) -> DataViewItem {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            let raw_item = ffi::wxd_DataViewTreeCtrl_PrependItem(self.dvtc_ptr(), **parent, text_c_str.as_ptr(), icon);
            DataViewItem::from(raw_item)
        }
    }

    pub fn prepend_container(&self, parent: &DataViewItem, text: &str, icon: i32, expanded_icon: i32) -> DataViewItem {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            let raw_item =
                ffi::wxd_DataViewTreeCtrl_PrependContainer(self.dvtc_ptr(), **parent, text_c_str.as_ptr(), icon, expanded_icon);
            DataViewItem::from(raw_item)
        }
    }

    pub fn insert_item(&self, parent: &DataViewItem, previous: &DataViewItem, text: &str, icon: i32) -> DataViewItem {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            let raw_item = ffi::wxd_DataViewTreeCtrl_InsertItem(self.dvtc_ptr(), **parent, **previous, text_c_str.as_ptr(), icon);
            DataViewItem::from(raw_item)
        }
    }

    pub fn insert_container(
        &self,
        parent: &DataViewItem,
        previous: &DataViewItem,
        text: &str,
        icon: i32,
        expanded_icon: i32,
    ) -> DataViewItem {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            let raw_item = ffi::wxd_DataViewTreeCtrl_InsertContainer(
                self.dvtc_ptr(),
                **parent,
                **previous,
                text_c_str.as_ptr(),
                icon,
                expanded_icon,
            );
            DataViewItem::from(raw_item)
        }
    }

    pub fn delete_item(&self, item: &DataViewItem) {
        unsafe { ffi::wxd_DataViewTreeCtrl_DeleteItem(self.dvtc_ptr(), **item) };

        // Note: C++ ffi::wxd_DataViewItem_Release is called by DataViewItem's Drop trait
        // when the Rust DataViewItem object goes out of scope IF it owned the item.
        // If `item` here is just a borrow, its owner will handle the drop.
        // `DeleteItem` in C++ side does not delete the wxDataViewItem memory itself,
        // only removes it from the tree. The Rust `DataViewItem`'s Drop is responsible.
    }

    pub fn delete_children(&self, item: &DataViewItem) {
        unsafe { ffi::wxd_DataViewTreeCtrl_DeleteChildren(self.dvtc_ptr(), **item) };
    }

    pub fn delete_all_items(&self) {
        unsafe { ffi::wxd_DataViewTreeCtrl_DeleteAllItems(self.dvtc_ptr()) };
    }

    // --- Item Attributes ---
    pub fn get_item_text(&self, item: &DataViewItem) -> String {
        let ptr = self.dvtc_ptr();
        let len = unsafe { ffi::wxd_DataViewTreeCtrl_GetItemText(ptr, **item, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut b = vec![0; len as usize + 1]; // +1 for null terminator
        unsafe { ffi::wxd_DataViewTreeCtrl_GetItemText(ptr, **item, b.as_mut_ptr(), b.len()) };
        unsafe { CStr::from_ptr(b.as_ptr()).to_string_lossy().to_string() }
    }

    pub fn set_item_text(&self, item: &DataViewItem, text: &str) {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_DataViewTreeCtrl_SetItemText(self.dvtc_ptr(), **item, text_c_str.as_ptr());
        }
    }

    pub fn set_item_icon(&self, item: &DataViewItem, icon_idx: i32) {
        unsafe {
            ffi::wxd_DataViewTreeCtrl_SetItemIcon(self.dvtc_ptr(), **item, icon_idx);
        }
    }

    pub fn set_item_expanded_icon(&self, item: &DataViewItem, icon_idx: i32) {
        unsafe {
            ffi::wxd_DataViewTreeCtrl_SetItemExpandedIcon(self.dvtc_ptr(), **item, icon_idx);
        }
    }

    // --- Item Relationships ---
    pub fn get_item_parent(&self, item: &DataViewItem) -> DataViewItem {
        unsafe {
            let raw_item = ffi::wxd_DataViewTreeCtrl_GetItemParent(self.dvtc_ptr(), **item);
            DataViewItem::from(raw_item)
        }
    }

    pub fn get_child_count(&self, parent: &DataViewItem) -> u32 {
        unsafe { ffi::wxd_DataViewTreeCtrl_GetChildCount(self.dvtc_ptr(), **parent) }
    }

    pub fn get_nth_child(&self, parent: &DataViewItem, pos: u32) -> DataViewItem {
        unsafe {
            let raw_item = ffi::wxd_DataViewTreeCtrl_GetNthChild(self.dvtc_ptr(), **parent, pos);
            DataViewItem::from(raw_item)
        }
    }

    pub fn is_container(&self, item: &DataViewItem) -> bool {
        unsafe { ffi::wxd_DataViewTreeCtrl_IsContainer(self.dvtc_ptr(), **item) }
    }

    // --- Tree State ---
    pub fn expand(&self, item: &DataViewItem) {
        unsafe { ffi::wxd_DataViewTreeCtrl_Expand(self.dvtc_ptr(), **item) };
    }

    pub fn collapse(&self, item: &DataViewItem) {
        unsafe { ffi::wxd_DataViewTreeCtrl_Collapse(self.dvtc_ptr(), **item) };
    }

    pub fn is_expanded(&self, item: &DataViewItem) -> bool {
        unsafe { ffi::wxd_DataViewTreeCtrl_IsExpanded(self.dvtc_ptr(), **item) }
    }

    // --- Image List ---
    /// Sets the image list for the control.
    /// The control takes ownership of the image list and will delete it when the control is destroyed.
    pub fn set_image_list(&self, image_list: ImageList) {
        // Takes ownership of image_list
        unsafe {
            ffi::wxd_DataViewTreeCtrl_SetImageList(self.dvtc_ptr(), image_list.as_ptr() as *mut ffi::wxd_ImageList_t);
            // Prevent Rust from dropping the ImageList as wxWidgets now owns it.
            std::mem::forget(image_list);
        }
    }

    pub fn get_image_list(&self) -> Option<ImageList> {
        unsafe {
            let raw_ptr = ffi::wxd_DataViewTreeCtrl_GetImageList(self.dvtc_ptr()) as *mut ffi::wxd_ImageList_t;
            if raw_ptr.is_null() {
                None
            } else {
                Some(ImageList::from_ptr_unowned(raw_ptr)) // Use unowned constructor
            }
        }
    }

    // --- Selection Methods ---
    /// Gets the currently selected item.
    ///
    /// # Returns
    ///
    /// An `Option` containing the selected item, or `None` if no item is selected.
    pub fn get_selection(&self) -> Option<DataViewItem> {
        // For DataViewTreeCtrl, use GetSelections and take the first item
        // This works around issues with GetSelection on tree controls
        let selections = self.get_selections();
        selections.into_iter().next()
    }

    /// Gets all selected items.
    ///
    /// # Returns
    ///
    /// A vector of selected items.
    pub fn get_selections(&self) -> Vec<DataViewItem> {
        // Use the inherited method through deref - this is more efficient than reimplementing
        let count = unsafe { ffi::wxd_DataViewCtrl_GetSelectedItemsCount(self.dvtc_ptr()) };
        if count == 0 {
            return Vec::new();
        }

        let mut items = Vec::with_capacity(count as usize);
        let mut items_raw: Vec<*const ffi::wxd_DataViewItem_t> = vec![std::ptr::null(); count as usize];

        let items_raw_ptr = items_raw.as_mut_ptr();
        unsafe { ffi::wxd_DataViewCtrl_GetSelections(self.dvtc_ptr(), items_raw_ptr, count) };

        for raw_ptr in items_raw {
            if !raw_ptr.is_null() {
                items.push(DataViewItem::from(raw_ptr));
            }
        }

        items
    }

    // TODO: Add event binding. This would re-introduce EventType, WxEvtHandler, and potentially CommandEvent.
    // Example:
    // pub fn bind_selection_changed<F>(&self, closure: F)
    // where
    //     F: Fn(crate::CommandEvent) + 'static, // Assuming CommandEvent is at crate level
    // {
    //     self.bind_event_handler(crate::event::EventType::DataViewSelectionChanged, closure);
    // }
}

// Manual WxWidget implementation for DataViewTreeCtrl (using WindowHandle)
impl WxWidget for DataViewTreeCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for DataViewTreeCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for DataViewTreeCtrl {}

// Implement DataViewEventHandler for DataViewTreeCtrl
impl crate::widgets::dataview::DataViewEventHandler for DataViewTreeCtrl {}

// Implement DataViewTreeEventHandler for DataViewTreeCtrl since it supports tree functionality
impl crate::widgets::dataview::DataViewTreeEventHandler for DataViewTreeCtrl {}

// Missing wxd_DataViewTreeCtrl_new
// This needs to be added to rust/wxdragon-sys/cpp/include/widgets/wxd_dataviewtreectrl.h
// and implemented in rust/wxdragon-sys/cpp/src/dataviewtreectrl.cpp

/*
Example FFI declaration for wxd_DataViewTreeCtrl_new (in wxd_dataviewtreectrl.h):
WXD_EXPORTED wxd_Window_t* wxd_DataViewTreeCtrl_new(
    wxd_Window_t* parent,
    int id,
    wxd_Point pos,
    wxd_Size size,
    long style,
    wxd_Window_t* validator, // Typically NULL for DataViewCtrl
    const char* name
);

Example FFI implementation (in dataviewtreectrl.cpp):
WXD_EXPORTED wxd_Window_t* wxd_DataViewTreeCtrl_new(
    wxd_Window_t* parent_ptr,
    int id,
    wxd_Point pos,
    wxd_Size size,
    long style,
    wxd_Window_t* validator_ptr, // unused, wxValidator not directly mapped for DVTC creation
    const char* name)
{
    wxWindow* parent = reinterpret_cast<wxWindow*>(parent_ptr);
    wxValidator* validator = nullptr; // wxDataViewCtrl usually doesn't use validator in this way

    wxDataViewTreeCtrl* ctrl = new wxDataViewTreeCtrl(
        parent,
        static_cast<wxWindowID>(id),
        to_wx(pos),
        to_wx(size),
        style,
        *wxDefaultValidator, // wxWidgets uses default validator if none provided. How to pass NULL or default?
                             // For controls, wxDefaultValidator is fine.
        wxString::FromUTF8(name ? name : "")
    );
    return reinterpret_cast<wxd_Window_t*>(static_cast<wxWindow*>(ctrl));
}
*/

// Note on DataViewItem:
// - When Rust receives a DataViewItem from C++ (e.g. AppendItem), it's a new C++ heap allocation
//   (see FromWxDVI in dataviewtreectrl.cpp). Rust's DataViewItem takes ownership and its Drop
//   impl calls wxd_DataViewItem_Release.
// - When Rust passes a &DataViewItem to C++ (e.g. parent in AppendItem), C++ uses the
//   pointer via ToWxDVI, but does not take ownership or delete the wxDataViewItem.
// - For parent items (like the root), DataViewItem::default() should be used, which creates
//   an item with invalid state. ToWxDVI handles this by creating an invalid wxDataViewItem.
// - The icon parameters are integer indices into the ImageList associated with the control.
//   A value of -1 typically means no icon.
