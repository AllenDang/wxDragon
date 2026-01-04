//! Safe wrapper for wxListBox.

use crate::Menu;
use crate::event::event_data::CommandEventData;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

// --- Constants ---
// Special value returned by GetSelection when nothing is selected
pub const NOT_FOUND: i32 = -1; // wxNOT_FOUND is typically -1

// --- Style enum using macro ---
widget_style_enum!(
    name: ListBoxStyle,
    doc: "Style flags for ListBox.",
    variants: {
        Default: ffi::WXD_LB_SINGLE, "Default style (single selection).",
        Multiple: ffi::WXD_LB_MULTIPLE, "Multiple selection list: any number of items can be selected.",
        Extended: ffi::WXD_LB_EXTENDED, "Extended selection list: allows using Shift and Ctrl keys for selection.",
        Sort: ffi::WXD_LB_SORT, "The items in the listbox are kept sorted in alphabetical order.",
        AlwaysScrollbar: ffi::WXD_LB_ALWAYS_SB, "Always show a vertical scrollbar.",
        HorizontalScrollbar: ffi::WXD_LB_HSCROLL, "Create a horizontal scrollbar if contents are too wide (requires explicit sizing)."
    },
    default_variant: Default
);

// Opaque pointer type from FFI
pub type RawListBox = ffi::wxd_ListBox_t;

/// Represents a wxListBox control.
///
/// ListBox uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct ListBox {
    handle: WindowHandle,
}

impl ListBox {
    /// Creates a new `ListBoxBuilder`.
    pub fn builder(parent: &dyn WxWidget) -> ListBoxBuilder<'_> {
        ListBoxBuilder::new(parent)
    }

    /// Helper to get raw listbox pointer, returns null if widget has been destroyed
    #[inline]
    fn listbox_ptr(&self) -> *mut RawListBox {
        self.handle
            .get_ptr()
            .map(|p| p as *mut RawListBox)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Appends an item to the list box.
    /// No-op if the listbox has been destroyed.
    pub fn append(&self, item: &str) {
        let ptr = self.listbox_ptr();
        if ptr.is_null() {
            return;
        }
        let c_item = CString::new(item).expect("Invalid CString for ListBox item");
        unsafe {
            ffi::wxd_ListBox_Append(ptr, c_item.as_ptr());
        }
    }

    /// Removes all items from the list box.
    /// No-op if the listbox has been destroyed.
    pub fn clear(&self) {
        let ptr = self.listbox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_ListBox_Clear(ptr);
        }
    }

    /// Gets the index of the currently selected item.
    /// Returns `None` if no item is selected (matches `NOT_FOUND`) or if the listbox has been destroyed.
    /// Note: For multi-selection list boxes, this returns the *first* selected item.
    pub fn get_selection(&self) -> Option<u32> {
        let ptr = self.listbox_ptr();
        if ptr.is_null() {
            return None;
        }
        let selection = unsafe { ffi::wxd_ListBox_GetSelection(ptr) };
        if selection == NOT_FOUND {
            None
        } else {
            Some(selection as u32)
        }
    }

    /// Gets the string value of the currently selected item.
    /// Returns `None` if no item is selected or if the listbox has been destroyed.
    pub fn get_string_selection(&self) -> Option<String> {
        let ptr = self.listbox_ptr();
        if ptr.is_null() {
            return None;
        }
        // Allocate a buffer first, like in Event::get_string
        let mut buffer = [0; 1024];
        let len = unsafe { ffi::wxd_ListBox_GetStringSelection(ptr, buffer.as_mut_ptr(), buffer.len()) };

        if len < 0 {
            return None; // Indicates error or no selection
        }

        if len < buffer.len() as i32 {
            // String fit in the initial buffer
            return Some(unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned() });
        }
        // Buffer was too small, allocate exact size + null terminator
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_ListBox_GetStringSelection(ptr, buf.as_mut_ptr(), buf.len()) };
        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() })
    }

    /// Selects or deselects an item at the given index.
    /// For single-selection list boxes, `select = true` selects the item.
    /// For multi-selection list boxes, `select = true` toggles the selection.
    /// No-op if the listbox has been destroyed.
    pub fn set_selection(&self, index: u32, select: bool) {
        let ptr = self.listbox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ListBox_SetSelection(ptr, index as i32, select) };
    }

    /// Selects an item by its string value.
    /// If the string is not found, no selection is made.
    /// No-op if the listbox has been destroyed.
    pub fn set_string_selection(&self, item: &str, select: bool) {
        let ptr = self.listbox_ptr();
        if ptr.is_null() {
            return;
        }
        // Create a CString, handling null bytes gracefully
        let c_item = match CString::new(item) {
            Ok(s) => s,
            Err(_) => {
                // If text contains null bytes, create a copy without them
                let filtered: String = item.chars().filter(|&c| c != '\0').collect();
                CString::new(filtered).unwrap_or_else(|_| CString::new("").unwrap())
            }
        };
        unsafe { ffi::wxd_ListBox_SetStringSelection(ptr, c_item.as_ptr(), select) };
    }

    /// Gets the string at the specified index.
    /// Returns `None` if the index is out of bounds or if the listbox has been destroyed.
    pub fn get_string(&self, index: u32) -> Option<String> {
        let ptr = self.listbox_ptr();
        if ptr.is_null() {
            return None;
        }
        // Allocate buffer first
        let mut buffer = [0; 1024];
        let len = unsafe { ffi::wxd_ListBox_GetString(ptr, index as i32, buffer.as_mut_ptr(), buffer.len()) };

        if len < 0 {
            return None; // Indicates error or invalid index
        }

        if len < buffer.len() as i32 {
            return Some(unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned() });
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_ListBox_GetString(ptr, index as i32, buf.as_mut_ptr(), buf.len()) };
        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() })
    }

    /// Gets the number of items in the list box.
    /// Returns 0 if the listbox has been destroyed.
    pub fn get_count(&self) -> u32 {
        let ptr = self.listbox_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_ListBox_GetCount(ptr) }
    }

    /// Deletes the item at the specified index.
    /// No-op if the listbox has been destroyed.
    pub fn delete(&self, index: u32) {
        let ptr = self.listbox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_ListBox_Delete(ptr, index as i32);
        }
    }

    /// Creates a ListBox from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_ListBox_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut RawListBox) -> Self {
        assert!(!ptr.is_null());
        ListBox {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Pops up a menu at the specified position.
    /// If `pos` is `None`, the menu is popped up at the current cursor position.
    /// # Returns
    /// `true` if the menu was popped up successfully, `false` otherwise.
    /// Returns `false` if the listbox has been destroyed.
    pub fn popup_menu(&self, menu: &mut Menu, pos: Option<Point>) -> bool {
        let ptr = self.listbox_ptr();
        if ptr.is_null() {
            return false;
        }
        let pos = pos.unwrap_or_else(|| Point::new(-1, -1));
        unsafe { ffi::wxd_ListBox_PopupMenu(ptr, menu.as_mut_ptr(), pos.into()) }
    }
}

// Use the widget_builder macro to generate the ListBoxBuilder implementation
widget_builder!(
    name: ListBox,
    parent_type: &'a dyn WxWidget,
    style_type: ListBoxStyle,
    fields: {
        choices: Vec<String> = Vec::new()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();

        // Call FFI to create the ListBox
        let ctrl_ptr = unsafe {
            ffi::wxd_ListBox_Create(
                parent_ptr,
                slf.id,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            )
        };

        if ctrl_ptr.is_null() {
            panic!("Failed to create ListBox: FFI returned null pointer.");
        }

        let list_box = unsafe { ListBox::from_ptr(ctrl_ptr) };

        // Append initial choices if any
        for choice_str in &slf.choices {
            list_box.append(choice_str);
        }

        list_box
    }
);

// Manual WxWidget implementation for ListBox (using WindowHandle)
impl WxWidget for ListBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for ListBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for ListBox {}

// --- ListBox specific event enum ---
/// Events specific to ListBox controls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListBoxEvent {
    /// Fired when an item is selected
    Selected,
    /// Fired when an item is double-clicked
    DoubleClicked,
}

/// Event data for ListBox events
#[derive(Debug)]
pub struct ListBoxEventData {
    pub event: CommandEventData,
}

impl ListBoxEventData {
    pub fn new(event: Event) -> Self {
        Self {
            event: CommandEventData::new(event),
        }
    }

    /// Get the widget ID that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Get the selected item's index
    pub fn get_selection(&self) -> Option<i32> {
        self.event.get_int()
    }

    /// Get the selected item's text (if available)
    pub fn get_string(&self) -> Option<String> {
        self.event.get_string()
    }
}

// At the bottom of the file, use the local macro
crate::implement_widget_local_event_handlers!(
    ListBox,
    ListBoxEvent,
    ListBoxEventData,
    Selected => selection_changed, EventType::COMMAND_LISTBOX_SELECTED,
    DoubleClicked => item_double_clicked, EventType::COMMAND_LISTBOX_DOUBLECLICKED
);

// Add XRC Support - enables ListBox to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for ListBox {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ListBox {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Widget casting support for ListBox
impl crate::window::FromWindowWithClassName for ListBox {
    fn class_name() -> &'static str {
        "wxListBox"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ListBox {
            handle: WindowHandle::new(ptr),
        }
    }
}
