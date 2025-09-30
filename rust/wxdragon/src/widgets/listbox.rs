//! Safe wrapper for wxListBox.

use crate::event::event_data::CommandEventData;
use crate::event::{Event, EventType};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use crate::Menu;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
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
#[derive(Clone)]
pub struct ListBox {
    window: Window,
}

impl ListBox {
    /// Creates a new `ListBoxBuilder`.
    pub fn builder(parent: &dyn WxWidget) -> ListBoxBuilder<'_> {
        ListBoxBuilder::new(parent)
    }

    /// Appends an item to the list box.
    pub fn append(&self, item: &str) {
        let c_item = CString::new(item).expect("Invalid CString for ListBox item");
        unsafe {
            ffi::wxd_ListBox_Append(self.window.as_ptr() as *mut RawListBox, c_item.as_ptr());
        }
    }

    /// Removes all items from the list box.
    pub fn clear(&self) {
        unsafe {
            ffi::wxd_ListBox_Clear(self.window.as_ptr() as *mut RawListBox);
        }
    }

    /// Gets the index of the currently selected item.
    /// Returns `None` if no item is selected (matches `NOT_FOUND`).
    /// Note: For multi-selection list boxes, this returns the *first* selected item.
    pub fn get_selection(&self) -> Option<u32> {
        let selection =
            unsafe { ffi::wxd_ListBox_GetSelection(self.window.as_ptr() as *mut RawListBox) };
        if selection == NOT_FOUND {
            None
        } else {
            Some(selection as u32)
        }
    }

    /// Gets the string value of the currently selected item.
    /// Returns `None` if no item is selected.
    pub fn get_string_selection(&self) -> Option<String> {
        unsafe {
            // Allocate a buffer first, like in Event::get_string
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed = ffi::wxd_ListBox_GetStringSelection(
                self.window.as_ptr() as *mut RawListBox,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None; // Indicates error or no selection
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                // String fit in the initial buffer
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                // Buffer was too small, allocate exact size + null terminator
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_ListBox_GetStringSelection(
                    self.window.as_ptr() as *mut RawListBox,
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop(); // Remove null terminator
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None // Error on second call
                }
            }
        }
    }

    /// Selects or deselects an item at the given index.
    /// For single-selection list boxes, `select = true` selects the item.
    /// For multi-selection list boxes, `select = true` toggles the selection.
    pub fn set_selection(&self, index: u32, select: bool) {
        unsafe {
            ffi::wxd_ListBox_SetSelection(
                self.window.as_ptr() as *mut RawListBox,
                index as i32,
                select,
            );
        }
    }

    /// Selects an item by its string value.
    /// If the string is not found, no selection is made.
    pub fn set_string_selection(&self, item: &str, select: bool) {
        // Create a CString, handling null bytes gracefully
        let c_item = match CString::new(item) {
            Ok(s) => s,
            Err(_) => {
                // If text contains null bytes, create a copy without them
                let filtered: String = item.chars().filter(|&c| c != '\0').collect();
                CString::new(filtered).unwrap_or_else(|_| CString::new("").unwrap())
            }
        };
        unsafe {
            ffi::wxd_ListBox_SetStringSelection(
                self.window.as_ptr() as *mut RawListBox,
                c_item.as_ptr(),
                select,
            );
        }
    }

    /// Gets the string at the specified index.
    /// Returns `None` if the index is out of bounds.
    pub fn get_string(&self, index: u32) -> Option<String> {
        unsafe {
            // Allocate buffer first
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed = ffi::wxd_ListBox_GetString(
                self.window.as_ptr() as *mut RawListBox,
                index as i32,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None; // Indicates error or invalid index
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_ListBox_GetString(
                    self.window.as_ptr() as *mut RawListBox,
                    index as i32,
                    vec_buffer.as_mut_ptr() as *mut c_char,
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

    /// Gets the number of items in the list box.
    pub fn get_count(&self) -> u32 {
        unsafe { ffi::wxd_ListBox_GetCount(self.window.as_ptr() as *mut RawListBox) }
    }

    /// Deletes the item at the specified index.
    pub fn delete(&self, index: u32) {
        unsafe {
            ffi::wxd_ListBox_Delete(self.window.as_ptr() as *mut RawListBox, index as i32);
        }
    }

    /// Creates a ListBox from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_ListBox_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut RawListBox) -> Self {
        assert!(!ptr.is_null());
        ListBox {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Pops up a menu at the specified position.
    /// If `pos` is `None`, the menu is popped up at the current cursor position.
    /// # Returns
    /// `true` if the menu was popped up successfully, `false` otherwise.
    pub fn popup_menu(&self, menu: &Menu, pos: Option<Point>) -> bool {
        let pos = pos.unwrap_or_else(|| Point::new(-1, -1));
        unsafe {
            ffi::wxd_ListBox_PopupMenu(
                self.window.as_ptr() as *mut RawListBox,
                menu.as_ptr(),
                pos.into(),
            )
        }
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

// Apply common trait implementations for ListBox
implement_widget_traits_with_target!(ListBox, window, Window);

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
impl_xrc_support!(ListBox, { window });

// Widget casting support for ListBox
impl_widget_cast!(ListBox, "wxListBox", { window });
