//! Safe wrapper for wxComboBox.

use crate::event::event_data::CommandEventData;
use crate::event::{Event, EventType, TextEvents, WindowEvents};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// Value for GetSelection when nothing selected
pub const NOT_FOUND: i32 = -1;

// Opaque pointer type from FFI
pub type RawComboBox = ffi::wxd_ComboBox_t;

/// Represents a wxComboBox control (dropdown list + text entry).
#[derive(Clone)]
pub struct ComboBox {
    window: Window,
}

impl ComboBox {
    /// Creates a new `ComboBoxBuilder`.
    pub fn builder(parent: &dyn WxWidget) -> ComboBoxBuilder<'_> {
        ComboBoxBuilder::new(parent)
    }

    /// Appends an item to the combobox list.
    pub fn append(&self, item: &str) {
        let c_item = CString::new(item).expect("Invalid CString for ComboBox item");
        unsafe {
            ffi::wxd_ComboBox_Append(self.window.as_ptr() as *mut _, c_item.as_ptr());
        }
    }

    /// Removes all items from the combobox list.
    /// Does not clear the text entry field value.
    pub fn clear(&self) {
        unsafe {
            ffi::wxd_ComboBox_Clear(self.window.as_ptr() as *mut _);
        }
    }

    /// Gets the index of the selected item in the list.
    /// Returns `None` if no item is selected or if the text doesn't match an item.
    pub fn get_selection(&self) -> Option<u32> {
        let selection = unsafe { ffi::wxd_ComboBox_GetSelection(self.window.as_ptr() as *mut _) };
        if selection == NOT_FOUND {
            None
        } else {
            Some(selection as u32)
        }
    }

    /// Gets the string selection from the combo box.
    pub fn get_string_selection(&self) -> Option<String> {
        const DEFAULT_BUF_SIZE: usize = 256;
        let mut buffer: Vec<u8> = Vec::with_capacity(DEFAULT_BUF_SIZE);
        let mut actual_len: i32;

        unsafe {
            actual_len = ffi::wxd_ComboBox_GetStringSelection(
                self.window.as_ptr() as *mut _,
                buffer.as_mut_ptr() as *mut std::os::raw::c_char,
                buffer.capacity() as i32,
            );

            if actual_len < 0 {
                // Indicates an error or no selection
                return None;
            }

            if actual_len as usize > buffer.capacity() {
                // Buffer was too small, resize and try again
                buffer.reserve_exact(actual_len as usize - buffer.capacity());
                actual_len = ffi::wxd_ComboBox_GetStringSelection(
                    self.window.as_ptr() as *mut _,
                    buffer.as_mut_ptr() as *mut std::os::raw::c_char,
                    buffer.capacity() as i32,
                );
            }

            if actual_len >= 0 {
                buffer.set_len(actual_len as usize); // Set actual length of string
                                                     // Create String from UTF-8 bytes, handling potential invalid UTF-8
                Some(String::from_utf8_lossy(&buffer).into_owned())
            } else {
                None // Error occurred
            }
        }
    }

    /// Selects the item at the given index in the list.
    /// This also updates the text entry field to the selected string.
    pub fn set_selection(&self, index: u32) {
        unsafe {
            ffi::wxd_ComboBox_SetSelection(self.window.as_ptr() as *mut _, index as i32);
        }
    }

    /// Gets the string at the specified index in the list.
    /// Returns `None` if the index is out of bounds.
    pub fn get_string(&self, index: u32) -> Option<String> {
        unsafe {
            let mut buffer: [c_char; 256] = [0; 256]; // Reasonable buffer size
            let len_needed = ffi::wxd_ComboBox_GetString(
                self.window.as_ptr() as *mut _,
                index as i32,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed <= 0 {
                return None; // Error or invalid index
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize <= buffer.len() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                // Buffer too small, try again with required size
                let mut vec_buffer: Vec<c_char> = vec![0; len_needed_usize];
                let len_needed_2 = ffi::wxd_ComboBox_GetString(
                    self.window.as_ptr() as *mut _,
                    index as i32,
                    vec_buffer.as_mut_ptr(),
                    vec_buffer.len() as i32,
                );
                if len_needed_2 == len_needed {
                    let c_str = CStr::from_ptr(vec_buffer.as_ptr());
                    Some(c_str.to_string_lossy().into_owned())
                } else {
                    // Something went wrong
                    None
                }
            }
        }
    }

    /// Gets the number of items in the combobox list.
    pub fn get_count(&self) -> u32 {
        unsafe { ffi::wxd_ComboBox_GetCount(self.window.as_ptr() as *mut _) }
    }

    /// Gets the current text value from the text entry field.
    pub fn get_value(&self) -> String {
        unsafe {
            let mut buffer: [c_char; 256] = [0; 256]; // Reasonable buffer size
            let len_needed = ffi::wxd_ComboBox_GetValue(
                self.window.as_ptr() as *mut _,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed <= 0 {
                return String::new(); // Return empty string for errors
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize <= buffer.len() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                c_str.to_string_lossy().into_owned()
            } else {
                // Buffer too small, try again with required size
                let mut vec_buffer: Vec<c_char> = vec![0; len_needed_usize];
                let len_needed_2 = ffi::wxd_ComboBox_GetValue(
                    self.window.as_ptr() as *mut _,
                    vec_buffer.as_mut_ptr(),
                    vec_buffer.len() as i32,
                );
                if len_needed_2 == len_needed {
                    let c_str = CStr::from_ptr(vec_buffer.as_ptr());
                    c_str.to_string_lossy().into_owned()
                } else {
                    // Something went wrong
                    String::new()
                }
            }
        }
    }

    /// Sets the text value in the text entry field.
    pub fn set_value(&self, value: &str) {
        let c_value = CString::new(value).expect("Invalid CString for ComboBox value");
        unsafe {
            ffi::wxd_ComboBox_SetValue(self.window.as_ptr() as *mut _, c_value.as_ptr());
        }
    }

    /// Gets the text selection range in the text entry field.
    /// Returns (from, to) positions, or None if there's an error.
    pub fn get_text_selection(&self) -> Option<(i64, i64)> {
        let mut from: i64 = 0;
        let mut to: i64 = 0;
        unsafe {
            ffi::wxd_ComboBox_GetTextSelection(self.window.as_ptr() as *mut _, &mut from, &mut to);
        }
        Some((from, to))
    }

    /// Sets the text selection range in the text entry field.
    pub fn set_text_selection(&self, from: i64, to: i64) {
        unsafe {
            ffi::wxd_ComboBox_SetTextSelection(self.window.as_ptr() as *mut _, from, to);
        }
    }

    /// Gets the current insertion point (cursor position) in the text entry field.
    pub fn get_insertion_point(&self) -> i64 {
        unsafe { ffi::wxd_ComboBox_GetInsertionPoint(self.window.as_ptr() as *mut _) }
    }

    /// Sets the insertion point (cursor position) in the text entry field.
    pub fn set_insertion_point(&self, pos: i64) {
        unsafe {
            ffi::wxd_ComboBox_SetInsertionPoint(self.window.as_ptr() as *mut _, pos);
        }
    }

    /// Gets the last position in the text entry field.
    pub fn get_last_position(&self) -> i64 {
        unsafe { ffi::wxd_ComboBox_GetLastPosition(self.window.as_ptr() as *mut _) }
    }
}

// --- Style enum using macro ---
widget_style_enum!(
    name: ComboBoxStyle,
    doc: "Style flags for ComboBox widget.",
    variants: {
        Default: ffi::WXD_CB_DROPDOWN, "Default style: a regular dropdown combo box.",
        Simple: ffi::WXD_CB_SIMPLE, "A simple combo box with a permanently displayed list.",
        Sort: ffi::WXD_CB_SORT, "The list of items is kept sorted alphabetically.",
        ReadOnly: ffi::WXD_CB_READONLY, "The text field is read-only (user can only select from the list).",
        ProcessEnter: ffi::WXD_TE_PROCESS_ENTER, "Process the Enter key, generating a TEXT_ENTER event."
    },
    default_variant: Default
);

// --- Builder pattern using macro ---
widget_builder!(
    name: ComboBox,
    parent_type: &'a dyn WxWidget,
    style_type: ComboBoxStyle,
    fields: {
        value: String = String::new(),
        choices: Vec<String> = Vec::new()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        assert!(!parent_ptr.is_null(), "ComboBox requires a parent");

        let c_value = CString::new(slf.value.as_str()).expect("Invalid CString for ComboBox value");

        unsafe {
            let ctrl_ptr = ffi::wxd_ComboBox_Create(
                parent_ptr,
                slf.id,
                c_value.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            );

            if ctrl_ptr.is_null() {
                panic!("Failed to create ComboBox widget");
            } else {
                let window = Window::from_ptr(ctrl_ptr as *mut ffi::wxd_Window_t);
                let combo = ComboBox { window };

                // Append initial choices
                for item in &slf.choices {
                    combo.append(item);
                }

                combo
            }
        }
    }
);

// Add a convenience method to handle &[&str] choices
impl<'a> ComboBoxBuilder<'a> {
    /// Sets the initial items in the dropdown list from string slices.
    pub fn with_string_choices(mut self, choices: &[&str]) -> Self {
        self.choices = choices.iter().map(|s| s.to_string()).collect();
        self
    }
}

// --- Widget traits implementation using macro ---
implement_widget_traits_with_target!(ComboBox, window, Window);

// --- ComboBox specific event enum ---
/// Events specific to ComboBox controls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComboBoxEvent {
    /// Fired when an item is selected from the dropdown
    Selected,
}

/// Event data for ComboBox events
#[derive(Debug)]
pub struct ComboBoxEventData {
    pub event: CommandEventData,
}

impl ComboBoxEventData {
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
    ComboBox,
    ComboBoxEvent,
    ComboBoxEventData,
    Selected => selection_changed, EventType::COMMAND_COMBOBOX_SELECTED
);

// We still implement TextEvents for text entry capabilities
impl TextEvents for ComboBox {}

impl WindowEvents for ComboBox {}

// Add XRC Support - enables ComboBox to be created from XRC-managed pointers
impl_xrc_support!(ComboBox, { window });

// Widget casting support for ComboBox
impl_widget_cast!(ComboBox, "wxComboBox", { window });
