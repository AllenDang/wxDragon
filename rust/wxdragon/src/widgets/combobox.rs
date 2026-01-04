//! Safe wrapper for wxComboBox.

use crate::event::event_data::CommandEventData;
use crate::event::{Event, EventType, TextEvents, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

// Value for GetSelection when nothing selected
pub const NOT_FOUND: i32 = -1;

// Opaque pointer type from FFI
pub type RawComboBox = ffi::wxd_ComboBox_t;

/// Represents a wxComboBox control (dropdown list + text entry).
///
/// ComboBox uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct ComboBox {
    handle: WindowHandle,
}

impl ComboBox {
    /// Creates a new `ComboBoxBuilder`.
    pub fn builder(parent: &dyn WxWidget) -> ComboBoxBuilder<'_> {
        ComboBoxBuilder::new(parent)
    }

    /// Helper to get raw combobox pointer, returns null if widget has been destroyed
    #[inline]
    fn combobox_ptr(&self) -> *mut RawComboBox {
        self.handle
            .get_ptr()
            .map(|p| p as *mut RawComboBox)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Appends an item to the combobox list.
    /// No-op if the combobox has been destroyed.
    pub fn append(&self, item: &str) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let c_item = CString::new(item).expect("Invalid CString for ComboBox item");
        unsafe {
            ffi::wxd_ComboBox_Append(ptr, c_item.as_ptr());
        }
    }

    /// Removes all items from the combobox list.
    /// Does not clear the text entry field value.
    /// No-op if the combobox has been destroyed.
    pub fn clear(&self) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_ComboBox_Clear(ptr);
        }
    }

    /// Gets the index of the selected item in the list.
    /// Returns `None` if no item is selected or if the text doesn't match an item,
    /// or if the combobox has been destroyed.
    pub fn get_selection(&self) -> Option<u32> {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return None;
        }
        let selection = unsafe { ffi::wxd_ComboBox_GetSelection(ptr) };
        if selection == NOT_FOUND {
            None
        } else {
            Some(selection as u32)
        }
    }

    /// Gets the string selection from the combo box.
    /// Returns `None` if the combobox has been destroyed or there is no selection.
    pub fn get_string_selection(&self) -> Option<String> {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return None;
        }
        unsafe {
            let len = ffi::wxd_ComboBox_GetStringSelection(ptr, std::ptr::null_mut(), 0);

            if len < 0 {
                // Indicates an error or no selection
                return None;
            }

            let mut buf = vec![0; len as usize + 1];
            ffi::wxd_ComboBox_GetStringSelection(ptr, buf.as_mut_ptr(), buf.len());
            Some(CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string())
        }
    }

    /// Selects the item at the given index in the list.
    /// This also updates the text entry field to the selected string.
    /// No-op if the combobox has been destroyed.
    pub fn set_selection(&self, index: u32) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_ComboBox_SetSelection(ptr, index as i32);
        }
    }

    /// Gets the string at the specified index in the list.
    /// Returns `None` if the index is out of bounds or if the combobox has been destroyed.
    pub fn get_string(&self, index: u32) -> Option<String> {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return None;
        }
        unsafe {
            let len = ffi::wxd_ComboBox_GetString(ptr, index as i32, std::ptr::null_mut(), 0);
            if len < 0 {
                return None; // Error or invalid index
            }
            let mut buf = vec![0; len as usize + 1];
            ffi::wxd_ComboBox_GetString(ptr, index as i32, buf.as_mut_ptr(), buf.len());
            Some(CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned())
        }
    }

    /// Gets the number of items in the combobox list.
    /// Returns 0 if the combobox has been destroyed.
    pub fn get_count(&self) -> u32 {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_ComboBox_GetCount(ptr) }
    }

    /// Gets the current text value from the text entry field.
    /// Returns empty string if the combobox has been destroyed.
    pub fn get_value(&self) -> String {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let mut buffer = [0; 256]; // Reasonable buffer size
            let len = ffi::wxd_ComboBox_GetValue(ptr, buffer.as_mut_ptr(), buffer.len());

            if len <= 0 {
                return String::new(); // Return empty string for errors
            }

            if len < buffer.len() as i32 {
                CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned()
            } else {
                // Buffer too small, try again with required size
                let mut buf = vec![0; len as usize + 1];
                let len2 = ffi::wxd_ComboBox_GetValue(ptr, buf.as_mut_ptr(), buf.len());
                if len2 == len {
                    CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
                } else {
                    // Something went wrong
                    String::new()
                }
            }
        }
    }

    /// Sets the text value in the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn set_value(&self, value: &str) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let c_value = CString::new(value).expect("Invalid CString for ComboBox value");
        unsafe {
            ffi::wxd_ComboBox_SetValue(ptr, c_value.as_ptr());
        }
    }

    /// Gets the text selection range in the text entry field.
    /// Returns (from, to) positions, or None if there's an error or the combobox has been destroyed.
    pub fn get_text_selection(&self) -> Option<(i64, i64)> {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return None;
        }
        let mut from: i64 = 0;
        let mut to: i64 = 0;
        unsafe {
            ffi::wxd_ComboBox_GetTextSelection(ptr, &mut from, &mut to);
        }
        Some((from, to))
    }

    /// Sets the text selection range in the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn set_text_selection(&self, from: i64, to: i64) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_ComboBox_SetTextSelection(ptr, from, to);
        }
    }

    /// Gets the current insertion point (cursor position) in the text entry field.
    /// Returns 0 if the combobox has been destroyed.
    pub fn get_insertion_point(&self) -> i64 {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_ComboBox_GetInsertionPoint(ptr) }
    }

    /// Sets the insertion point (cursor position) in the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn set_insertion_point(&self, pos: i64) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_ComboBox_SetInsertionPoint(ptr, pos);
        }
    }

    /// Gets the last position in the text entry field.
    /// Returns 0 if the combobox has been destroyed.
    pub fn get_last_position(&self) -> i64 {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_ComboBox_GetLastPosition(ptr) }
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
            }

            let combo = ComboBox {
                handle: WindowHandle::new(ctrl_ptr as *mut ffi::wxd_Window_t)
            };

            // Append initial choices
            for item in &slf.choices {
                combo.append(item);
            }

            combo
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

// Manual WxWidget implementation for ComboBox (using WindowHandle)
impl WxWidget for ComboBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for ComboBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for ComboBox {}

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

// Add XRC Support - enables ComboBox to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for ComboBox {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ComboBox {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Widget casting support for ComboBox
impl crate::window::FromWindowWithClassName for ComboBox {
    fn class_name() -> &'static str {
        "wxComboBox"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ComboBox {
            handle: WindowHandle::new(ptr),
        }
    }
}
