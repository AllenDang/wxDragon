//! DataViewTreeCtrl implementation.

// use crate::event::EventType; // For event binding later
use crate::widgets::dataview::item::DataViewItem;
use crate::window::{Window, WxWidget}; // WxWidget needed for parent_type and Deref target
                                       // TODO: Add ImageList if/when it's implemented
                                       // use crate::widgets::imagelist::ImageList;
use crate::widgets::imagelist::ImageList; // USE THE NEW ImageList
use crate::{
    implement_widget_traits_with_target,
    widget_builder,
    widget_style_enum, // Corrected macro import and usage
    // WxEvtHandler, // For event binding later
    Id,
    Point,
    Size, // Colour removed (unused)
};
use std::ffi::CString;
// use std::rc::Rc; // Unused
use wxdragon_sys as ffi;
// Import necessary types for columns from parent dataview module
use super::column::DataViewColumn;
use super::enums::{DataViewAlign, DataViewCellMode, DataViewColumnFlags}; // Added DataViewCellMode and DataViewColumnFlags
use super::renderer::{DataViewIconTextRenderer, DataViewTextRenderer}; // Added DataViewIconTextRenderer
use super::variant::VariantType; // Added VariantType

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
        label: String = String::new() // Added label field as it's used in build_impl
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let label_c_str = CString::new(slf.label.clone()).unwrap_or_default();
        let handle = unsafe {
            ffi::wxd_DataViewTreeCtrl_new(
                parent_ptr,
                slf.id,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits(),
                std::ptr::null_mut(),
                label_c_str.as_ptr(),
            )
        };
        if handle.is_null() {
            panic!("Failed to create DataViewTreeCtrl");
        }
        // Construct Window directly from the handle
        let window = unsafe { Window::from_ptr(handle) };
        DataViewTreeCtrl { window }
    }
}

#[derive(Clone)]
pub struct DataViewTreeCtrl {
    window: Window,
}

// Implement WxWidget, Deref<Target=Window>, DerefMut, WxEvtHandler (if needed), Drop
// This should provide self.handle_ptr() and allow DataViewTreeCtrl to be used where Window is expected.
// Note: WxEvtHandler from crate::{...} might be needed if events are bound.
// For now, implement_widget_traits_with_target only includes WxWidget, Deref, DerefMut, Drop.
// If WxEvtHandler is needed by the macro, its import should be uncommented.
// The `macros.rs` shows WxEvtHandler is part of this macro.
// So, WxEvtHandler needs to be imported.
// use crate::WxEvtHandler; // This line will be uncommented just above if needed.
// For now, assuming the provided `implement_widget_traits_with_target` in `macros.rs` handles it.
// Actually, looking at the provided `macros.rs`, `implement_widget_traits_with_target` *does* include `WxEvtHandler`.
// So, `use crate::WxEvtHandler;` IS needed at the top of the file.

impl DataViewTreeCtrl {
    /// Creates a new builder for a DataViewTreeCtrl.
    pub fn builder<'a>(parent: &'a dyn WxWidget) -> DataViewTreeCtrlBuilder<'a> {
        DataViewTreeCtrlBuilder::new(parent)
    }

    // Methods will use self.handle_ptr() directly, relying on Deref to Window and Window's WxWidget impl.

    // --- Column Management (inherited from DataViewCtrl conceptually) ---
    /// Appends a pre-created column to the control.
    pub fn append_column(&self, column: &DataViewColumn) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_AppendColumn(self.handle_ptr(), column.as_raw()) }
    }

    /// Prepends a column to the control.
    pub fn prepend_column(&self, column: &DataViewColumn) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_PrependColumn(self.handle_ptr(), column.as_raw()) }
    }

    /// Inserts a column at the specified position.
    pub fn insert_column(&self, pos: usize, column: &DataViewColumn) -> bool {
        unsafe {
            ffi::wxd_DataViewCtrl_InsertColumn(self.handle_ptr(), pos as i64, column.as_raw())
        }
    }

    /// Remove all columns
    pub fn clear_columns(&self) -> bool {
        unsafe { ffi::wxd_DataViewCtrl_ClearColumns(self.handle_ptr()) }
    }

    /// Gets the column that currently displays the expander buttons.
    pub fn get_expander_column(&self) -> Option<DataViewColumn> {
        unsafe {
            let col_ptr = ffi::wxd_DataViewCtrl_GetExpanderColumn(self.handle_ptr());
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
        unsafe { ffi::wxd_DataViewCtrl_SetExpanderColumn(self.handle_ptr(), column.as_raw()) }
    }

    /// Gets the column at the given position (0-indexed).
    pub fn get_column(&self, pos: usize) -> Option<DataViewColumn> {
        unsafe {
            let col_ptr = ffi::wxd_DataViewCtrl_GetColumn(self.handle_ptr(), pos as u32);
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
        let renderer =
            DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, align);
        let column =
            DataViewColumn::new(label, &renderer, model_column as usize, width, align, flags);
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
        let renderer =
            DataViewIconTextRenderer::new(VariantType::IconText, DataViewCellMode::Inert, align);
        let column =
            DataViewColumn::new(label, &renderer, model_column as usize, width, align, flags);
        self.append_column(&column)
    }

    // --- Item Management ---
    pub fn append_item(&self, parent: &DataViewItem, text: &str, icon: i32) -> DataViewItem {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            let raw_item = ffi::wxd_DataViewTreeCtrl_AppendItem(
                self.handle_ptr(),
                parent.as_raw(),
                text_c_str.as_ptr(),
                icon,
            );
            DataViewItem::from_raw(raw_item)
        }
    }

    pub fn append_container(
        &self,
        parent: &DataViewItem,
        text: &str,
        icon: i32,
        expanded_icon: i32,
    ) -> DataViewItem {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            let raw_item = ffi::wxd_DataViewTreeCtrl_AppendContainer(
                self.handle_ptr(),
                parent.as_raw(),
                text_c_str.as_ptr(),
                icon,
                expanded_icon,
            );
            DataViewItem::from_raw(raw_item)
        }
    }

    pub fn prepend_item(&self, parent: &DataViewItem, text: &str, icon: i32) -> DataViewItem {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            let raw_item = ffi::wxd_DataViewTreeCtrl_PrependItem(
                self.handle_ptr(),
                parent.as_raw(),
                text_c_str.as_ptr(),
                icon,
            );
            DataViewItem::from_raw(raw_item)
        }
    }

    pub fn prepend_container(
        &self,
        parent: &DataViewItem,
        text: &str,
        icon: i32,
        expanded_icon: i32,
    ) -> DataViewItem {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            let raw_item = ffi::wxd_DataViewTreeCtrl_PrependContainer(
                self.handle_ptr(),
                parent.as_raw(),
                text_c_str.as_ptr(),
                icon,
                expanded_icon,
            );
            DataViewItem::from_raw(raw_item)
        }
    }

    pub fn insert_item(
        &self,
        parent: &DataViewItem,
        previous: &DataViewItem,
        text: &str,
        icon: i32,
    ) -> DataViewItem {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            let raw_item = ffi::wxd_DataViewTreeCtrl_InsertItem(
                self.handle_ptr(),
                parent.as_raw(),
                previous.as_raw(),
                text_c_str.as_ptr(),
                icon,
            );
            DataViewItem::from_raw(raw_item)
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
                self.handle_ptr(),
                parent.as_raw(),
                previous.as_raw(),
                text_c_str.as_ptr(),
                icon,
                expanded_icon,
            );
            DataViewItem::from_raw(raw_item)
        }
    }

    pub fn delete_item(&self, item: &DataViewItem) {
        unsafe {
            ffi::wxd_DataViewTreeCtrl_DeleteItem(self.handle_ptr(), item.as_raw());
        }
        // Note: C++ ffi::wxd_DataViewItem_Release is called by DataViewItem's Drop trait
        // when the Rust DataViewItem object goes out of scope IF it owned the item.
        // If `item` here is just a borrow, its owner will handle the drop.
        // `DeleteItem` in C++ side does not delete the wxDataViewItem memory itself,
        // only removes it from the tree. The Rust `DataViewItem`'s Drop is responsible.
    }

    pub fn delete_children(&self, item: &DataViewItem) {
        unsafe {
            ffi::wxd_DataViewTreeCtrl_DeleteChildren(self.handle_ptr(), item.as_raw());
        }
    }

    pub fn delete_all_items(&self) {
        unsafe {
            ffi::wxd_DataViewTreeCtrl_DeleteAllItems(self.handle_ptr());
        }
    }

    // --- Item Attributes ---
    pub fn get_item_text(&self, item: &DataViewItem) -> String {
        unsafe {
            let c_str = ffi::wxd_DataViewTreeCtrl_GetItemText(self.handle_ptr(), item.as_raw());
            if c_str.is_null() {
                String::new()
            } else {
                // Use CStr::from_ptr to borrow the string, then convert to owned String
                let c_str_borrowed = std::ffi::CStr::from_ptr(c_str);
                let rust_string = c_str_borrowed.to_string_lossy().into_owned();
                // Now free the C string using the proper FFI function
                ffi::wxd_free_string(c_str as *mut i8);
                rust_string
            }
        }
    }

    pub fn set_item_text(&self, item: &DataViewItem, text: &str) {
        let text_c_str = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_DataViewTreeCtrl_SetItemText(
                self.handle_ptr(),
                item.as_raw(),
                text_c_str.as_ptr(),
            );
        }
    }

    pub fn set_item_icon(&self, item: &DataViewItem, icon_idx: i32) {
        unsafe {
            ffi::wxd_DataViewTreeCtrl_SetItemIcon(self.handle_ptr(), item.as_raw(), icon_idx);
        }
    }

    pub fn set_item_expanded_icon(&self, item: &DataViewItem, icon_idx: i32) {
        unsafe {
            ffi::wxd_DataViewTreeCtrl_SetItemExpandedIcon(
                self.handle_ptr(),
                item.as_raw(),
                icon_idx,
            );
        }
    }

    // --- Item Relationships ---
    pub fn get_item_parent(&self, item: &DataViewItem) -> DataViewItem {
        unsafe {
            let raw_item =
                ffi::wxd_DataViewTreeCtrl_GetItemParent(self.handle_ptr(), item.as_raw());
            DataViewItem::from_raw(raw_item)
        }
    }

    pub fn get_child_count(&self, parent: &DataViewItem) -> u32 {
        unsafe { ffi::wxd_DataViewTreeCtrl_GetChildCount(self.handle_ptr(), parent.as_raw()) }
    }

    pub fn get_nth_child(&self, parent: &DataViewItem, pos: u32) -> DataViewItem {
        unsafe {
            let raw_item =
                ffi::wxd_DataViewTreeCtrl_GetNthChild(self.handle_ptr(), parent.as_raw(), pos);
            DataViewItem::from_raw(raw_item)
        }
    }

    pub fn is_container(&self, item: &DataViewItem) -> bool {
        unsafe { ffi::wxd_DataViewTreeCtrl_IsContainer(self.handle_ptr(), item.as_raw()) }
    }

    // --- Tree State ---
    pub fn expand(&self, item: &DataViewItem) {
        unsafe {
            ffi::wxd_DataViewTreeCtrl_Expand(self.handle_ptr(), item.as_raw());
        }
    }

    pub fn collapse(&self, item: &DataViewItem) {
        unsafe {
            ffi::wxd_DataViewTreeCtrl_Collapse(self.handle_ptr(), item.as_raw());
        }
    }

    pub fn is_expanded(&self, item: &DataViewItem) -> bool {
        unsafe { ffi::wxd_DataViewTreeCtrl_IsExpanded(self.handle_ptr(), item.as_raw()) }
    }

    // --- Image List ---
    /// Sets the image list for the control.
    /// The control takes ownership of the image list and will delete it when the control is destroyed.
    pub fn set_image_list(&self, image_list: ImageList) {
        // Takes ownership of image_list
        unsafe {
            ffi::wxd_DataViewTreeCtrl_SetImageList(
                self.handle_ptr(),
                image_list.as_ptr() as *mut ffi::wxd_ImageList_t,
            );
            // Prevent Rust from dropping the ImageList as wxWidgets now owns it.
            std::mem::forget(image_list);
        }
    }

    pub fn get_image_list(&self) -> Option<ImageList> {
        unsafe {
            let raw_ptr = ffi::wxd_DataViewTreeCtrl_GetImageList(self.handle_ptr())
                as *mut ffi::wxd_ImageList_t;
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
        let count = unsafe { ffi::wxd_DataViewCtrl_GetSelectedItemsCount(self.handle_ptr()) };
        if count == 0 {
            return Vec::new();
        }

        let mut items = Vec::with_capacity(count as usize);
        let mut items_raw = Vec::with_capacity(count as usize);
        items_raw.resize(
            count as usize,
            ffi::wxd_DataViewItem_t {
                id: std::ptr::null_mut(),
            },
        );

        unsafe {
            ffi::wxd_DataViewCtrl_GetSelections(self.handle_ptr(), items_raw.as_mut_ptr(), count);

            for raw_item in items_raw {
                if !raw_item.id.is_null() {
                    items.push(DataViewItem::from_raw(raw_item));
                }
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

implement_widget_traits_with_target!(DataViewTreeCtrl, window, Window);

// Implement DataViewEventHandler for DataViewTreeCtrl
impl crate::widgets::dataview::DataViewEventHandler for DataViewTreeCtrl {}

// Implement TreeViewEventHandler for DataViewTreeCtrl since it supports tree functionality
impl crate::widgets::dataview::TreeViewEventHandler for DataViewTreeCtrl {}

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
// - For parent items (like the root), DataViewItem::new_invalid() should be used, which creates
//   an item with a null `id`. ToWxDVI handles this by creating an invalid wxDataViewItem.
// - The icon parameters are integer indices into the ImageList associated with the control.
//   A value of -1 typically means no icon.
