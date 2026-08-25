//! Safe wrapper for wxComboBox.

use crate::event::event_data::CommandEventData;
use crate::event::{Event, EventType, TextEvents, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::utils::{ArrayString, read_ffi_string};
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

    /// Appends an item to the combobox.
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

    /// Inserts an item into the combobox at the specified position.
    /// No-op if the combobox has been destroyed.
    pub fn insert(&self, item: &str, pos: usize) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let c_item = CString::new(item).expect("Invalid CString for ComboBox item");
        unsafe {
            ffi::wxd_ComboBox_Insert(ptr, c_item.as_ptr(), pos as u32);
        }
    }

    /// Clears all items from the combobox.
    /// On MSW this also empties the text entry field and generates a text event.
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

    // --- Text entry (wxTextEntry) ---

    /// Sets the text entry field value without generating a text event.
    /// No-op if the combobox has been destroyed.
    pub fn change_value(&self, value: &str) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let c_value = CString::new(value).expect("Invalid CString for ComboBox value");
        unsafe { ffi::wxd_ComboBox_ChangeValue(ptr, c_value.as_ptr()) };
    }

    /// Inserts text at the current insertion point of the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn write_text(&self, text: &str) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let c_text = CString::new(text).expect("Invalid CString for ComboBox text");
        unsafe { ffi::wxd_ComboBox_WriteText(ptr, c_text.as_ptr()) };
    }

    /// Appends text to the end of the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn append_text(&self, text: &str) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let c_text = CString::new(text).expect("Invalid CString for ComboBox text");
        unsafe { ffi::wxd_ComboBox_AppendText(ptr, c_text.as_ptr()) };
    }

    /// Removes the text in the given range from the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn remove(&self, from: i64, to: i64) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_Remove(ptr, from, to) };
    }

    /// Replaces the text in the given range of the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn replace(&self, from: i64, to: i64, value: &str) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let c_value = CString::new(value).expect("Invalid CString for ComboBox value");
        unsafe { ffi::wxd_ComboBox_Replace(ptr, from, to, c_value.as_ptr()) };
    }

    /// Gets the text in the given range of the text entry field.
    /// Returns an empty string if the combobox has been destroyed.
    pub fn get_range(&self, from: i64, to: i64) -> String {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return String::new();
        }
        read_ffi_string(|buf, len| unsafe { ffi::wxd_ComboBox_GetRange(ptr, from, to, buf, len) }).unwrap_or_default()
    }

    /// Selects all text in the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn select_all(&self) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_SelectAll(ptr) };
    }

    /// Deselects all text in the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn select_none(&self) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_SelectNone(ptr) };
    }

    /// Moves the insertion point to the end of the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn set_insertion_point_end(&self) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_SetInsertionPointEnd(ptr) };
    }

    /// Makes the text entry field editable or read-only.
    /// No-op if the combobox has been destroyed.
    pub fn set_editable(&self, editable: bool) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_SetEditable(ptr, editable) };
    }

    /// Returns whether the text entry field is editable.
    /// Returns false if the combobox has been destroyed.
    pub fn is_editable(&self) -> bool {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ComboBox_IsEditable(ptr) }
    }

    /// Limits the number of characters the user can type into the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn set_max_length(&self, len: i64) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_SetMaxLength(ptr, len) };
    }

    /// Sets the hint shown in the text entry field while it is empty.
    /// Returns false if hints are not supported or the combobox has been destroyed.
    pub fn set_hint(&self, hint: &str) -> bool {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_hint = CString::new(hint).expect("Invalid CString for ComboBox hint");
        unsafe { ffi::wxd_ComboBox_SetHint(ptr, c_hint.as_ptr()) }
    }

    /// Gets the hint shown in the text entry field while it is empty.
    /// Returns an empty string if the combobox has been destroyed.
    pub fn get_hint(&self) -> String {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return String::new();
        }
        read_ffi_string(|buf, len| unsafe { ffi::wxd_ComboBox_GetHint(ptr, buf, len) }).unwrap_or_default()
    }

    /// Returns whether the text entry field is empty.
    /// Returns true if the combobox has been destroyed.
    pub fn is_text_empty(&self) -> bool {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return true;
        }
        unsafe { ffi::wxd_ComboBox_IsTextEmpty(ptr) }
    }

    /// Copies the selected text to the clipboard.
    /// No-op if the combobox has been destroyed.
    pub fn copy(&self) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_Copy(ptr) };
    }

    /// Cuts the selected text to the clipboard.
    /// No-op if the combobox has been destroyed.
    pub fn cut(&self) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_Cut(ptr) };
    }

    /// Pastes the clipboard text at the insertion point.
    /// No-op if the combobox has been destroyed.
    pub fn paste(&self) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_Paste(ptr) };
    }

    /// Undoes the last edit of the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn undo(&self) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_Undo(ptr) };
    }

    /// Redoes the last undone edit of the text entry field.
    /// No-op if the combobox has been destroyed.
    pub fn redo(&self) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_Redo(ptr) };
    }

    /// Returns whether there is selected text to copy.
    /// Returns false if the combobox has been destroyed.
    pub fn can_copy(&self) -> bool {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ComboBox_CanCopy(ptr) }
    }

    /// Returns whether there is selected text to cut.
    /// Returns false if the combobox has been destroyed.
    pub fn can_cut(&self) -> bool {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ComboBox_CanCut(ptr) }
    }

    /// Returns whether the clipboard holds text that can be pasted.
    /// Returns false if the combobox has been destroyed.
    pub fn can_paste(&self) -> bool {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ComboBox_CanPaste(ptr) }
    }

    /// Returns whether there is an edit to undo.
    /// Returns false if the combobox has been destroyed.
    pub fn can_undo(&self) -> bool {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ComboBox_CanUndo(ptr) }
    }

    /// Returns whether there is an undone edit to redo.
    /// Returns false if the combobox has been destroyed.
    pub fn can_redo(&self) -> bool {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ComboBox_CanRedo(ptr) }
    }

    /// Enables native auto-completion of the text entry field from the given choices.
    /// Returns false if auto-completion is not supported or the combobox has been destroyed.
    pub fn auto_complete(&self, choices: &[&str]) -> bool {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return false;
        }
        let array = ArrayString::from(choices);
        unsafe { ffi::wxd_ComboBox_AutoComplete(ptr, array.as_const_ptr()) }
    }

    // --- Items (wxItemContainer) ---

    /// Deletes the item at the specified index.
    /// No-op if the index is out of bounds or the combobox has been destroyed.
    pub fn delete(&self, index: u32) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_Delete(ptr, index) };
    }

    /// Sets the text of the item at the given index.
    /// No-op if the index is out of bounds or the combobox has been destroyed.
    pub fn set_string(&self, index: u32, text: &str) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let c_text = CString::new(text).expect("Invalid CString for ComboBox item");
        unsafe { ffi::wxd_ComboBox_SetString(ptr, index, c_text.as_ptr()) };
    }

    /// Selects the item with the given text, which also shows it in the text entry field.
    /// Returns false if no item matches or the combobox has been destroyed.
    pub fn set_string_selection(&self, text: &str) -> bool {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_text = CString::new(text).expect("Invalid CString for ComboBox item");
        unsafe { ffi::wxd_ComboBox_SetStringSelection(ptr, c_text.as_ptr()) }
    }

    /// Finds the index of the item with the given text.
    /// Returns `None` if no item matches or the combobox has been destroyed.
    pub fn find_string(&self, text: &str, case_sensitive: bool) -> Option<u32> {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return None;
        }
        let c_text = CString::new(text).expect("Invalid CString for ComboBox item");
        let index = unsafe { ffi::wxd_ComboBox_FindString(ptr, c_text.as_ptr(), case_sensitive) };
        if index == NOT_FOUND { None } else { Some(index as u32) }
    }

    /// Gets all items in the combobox list.
    /// Returns an empty vector if the combobox has been destroyed.
    pub fn get_strings(&self) -> Vec<String> {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return Vec::new();
        }
        let array_ptr = unsafe { ffi::wxd_ComboBox_GetStrings(ptr) };
        if array_ptr.is_null() {
            return Vec::new();
        }
        ArrayString::from(array_ptr).get_strings()
    }

    /// Returns whether the combobox list has no items.
    /// Returns true if the combobox has been destroyed.
    pub fn is_list_empty(&self) -> bool {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return true;
        }
        unsafe { ffi::wxd_ComboBox_IsListEmpty(ptr) }
    }

    /// Returns whether the combobox keeps its items sorted (`ComboBoxStyle::Sort`).
    /// Returns false if the combobox has been destroyed.
    pub fn is_sorted(&self) -> bool {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ComboBox_IsSorted(ptr) }
    }

    /// Appends several items to the combobox list.
    /// No-op if the combobox has been destroyed.
    pub fn append_items(&self, items: &[&str]) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let array = ArrayString::from(items);
        unsafe { ffi::wxd_ComboBox_AppendItems(ptr, array.as_const_ptr()) };
    }

    /// Inserts several items into the combobox list before the given position.
    /// No-op if the position is out of bounds or the combobox has been destroyed.
    pub fn insert_items(&self, items: &[&str], pos: u32) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let array = ArrayString::from(items);
        unsafe { ffi::wxd_ComboBox_InsertItems(ptr, array.as_const_ptr(), pos) };
    }

    /// Replaces all items in the combobox list.
    /// On MSW this also empties the text entry field and generates a text event.
    /// No-op if the combobox has been destroyed.
    pub fn set_items(&self, items: &[&str]) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let array = ArrayString::from(items);
        unsafe { ffi::wxd_ComboBox_SetItems(ptr, array.as_const_ptr()) };
    }

    // --- Dropdown ---

    /// Shows the dropdown list.
    /// No-op if the combobox has been destroyed.
    pub fn popup(&self) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_Popup(ptr) };
    }

    /// Hides the dropdown list.
    /// No-op if the combobox has been destroyed.
    pub fn dismiss(&self) {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ComboBox_Dismiss(ptr) };
    }

    /// Gets the index of the item highlighted in the dropdown list while it is
    /// shown; the same as `get_selection` while it is hidden.
    /// Returns `None` if nothing is highlighted or the combobox has been destroyed.
    pub fn get_current_selection(&self) -> Option<u32> {
        let ptr = self.combobox_ptr();
        if ptr.is_null() {
            return None;
        }
        let selection = unsafe { ffi::wxd_ComboBox_GetCurrentSelection(ptr) };
        if selection == NOT_FOUND {
            None
        } else {
            Some(selection as u32)
        }
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
    /// Fired when the dropdown list is shown
    DropDown,
    /// Fired when the dropdown list is hidden
    CloseUp,
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
    Selected => selection_changed, EventType::COMMAND_COMBOBOX_SELECTED,
    DropDown => dropdown, EventType::COMMAND_COMBOBOX_DROPDOWN,
    CloseUp => closeup, EventType::COMMAND_COMBOBOX_CLOSEUP
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const NOT_RUN: u8 = 0;
    const STARTED: u8 = 1;
    const FINISHED: u8 = 2;

    /// Runs `body` against a fresh combo box inside a wx main loop that a
    /// one-shot timer exits again (see the `window.rs` tests). Panics in the
    /// init callback are swallowed by `crate::main`, so the body's progress is
    /// tracked: started-but-unfinished fails the test, never-started (wx could
    /// not initialise) only logs. Skipped on macOS, where the test thread is
    /// not the OS main thread.
    fn with_combo(body: impl FnOnce(&ComboBox) + 'static) {
        let _gui_test = crate::app::GUI_TEST_LOCK.lock().unwrap();
        SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
        let progress = Rc::new(Cell::new(NOT_RUN));
        let progress_in_loop = progress.clone();
        let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
        let timer_store_clone = timer_store.clone();

        let res = crate::main(move |app| {
            let frame = Frame::builder().with_title("combobox test").build();
            let panel = Panel::builder(&frame).build();
            let combo = ComboBox::builder(&panel)
                .with_style(ComboBoxStyle::Default | ComboBoxStyle::ProcessEnter)
                .build();

            progress_in_loop.set(STARTED);
            body(&combo);
            progress_in_loop.set(FINISHED);

            let timer = Timer::new(&frame);
            let app_clone = app;
            let timer_store_cleanup = timer_store_clone.clone();
            timer.on_tick(move |_evt| {
                timer_store_cleanup.borrow_mut().take();
                app_clone.exit_main_loop();
            });
            timer.start(100, true);
            timer_store_clone.borrow_mut().replace(timer);
        });
        if let Err(e) = res {
            log::warn!("Test failed with error: {:?}", e);
        }
        assert_ne!(progress.get(), STARTED, "test body panicked inside the wx main loop");
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn change_value_does_not_generate_a_text_event() {
        with_combo(|combo| {
            let events = Rc::new(Cell::new(0));
            let counter = events.clone();
            combo.on_text_updated(move |_| counter.set(counter.get() + 1));

            combo.change_value("silent");
            assert_eq!(combo.get_value(), "silent");
            assert_eq!(events.get(), 0);

            combo.set_value("loud");
            assert!(events.get() >= 1);
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn write_text_inserts_at_the_insertion_point() {
        with_combo(|combo| {
            combo.set_value("ab");
            combo.set_insertion_point_end();
            combo.write_text("z");
            assert_eq!(combo.get_value(), "abz");

            combo.set_insertion_point(1);
            combo.write_text("-");
            assert_eq!(combo.get_value(), "a-bz");
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn text_ranges_can_be_read_removed_and_replaced() {
        with_combo(|combo| {
            combo.set_value("abcd");
            assert_eq!(combo.get_range(1, 3), "bc");

            combo.remove(0, 1);
            assert_eq!(combo.get_value(), "bcd");

            combo.replace(0, 2, "X");
            assert_eq!(combo.get_value(), "Xd");

            combo.append_text("!");
            assert_eq!(combo.get_value(), "Xd!");
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn select_all_and_select_none_drive_the_text_selection() {
        with_combo(|combo| {
            combo.set_value("abc");
            combo.select_all();
            assert_eq!(combo.get_text_selection(), Some((0, 3)));
            assert!(combo.can_copy());

            combo.select_none();
            let (from, to) = combo.get_text_selection().unwrap();
            assert_eq!(from, to);
            assert!(!combo.can_copy());
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn is_text_empty_tracks_the_text() {
        with_combo(|combo| {
            assert!(combo.is_text_empty());
            combo.change_value("x");
            assert!(!combo.is_text_empty());
            combo.change_value("");
            assert!(combo.is_text_empty());
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn editable_can_be_toggled() {
        with_combo(|combo| {
            assert!(combo.is_editable());
            combo.set_editable(false);
            assert!(!combo.is_editable());
            combo.set_editable(true);
            assert!(combo.is_editable());
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn hint_round_trips_where_the_platform_supports_it() {
        with_combo(|combo| {
            if combo.set_hint("type here") {
                assert_eq!(combo.get_hint(), "type here");
            }
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn remaining_text_entry_calls_are_safe() {
        with_combo(|combo| {
            combo.set_max_length(8);
            combo.auto_complete(&["alpha", "beta"]);
            combo.change_value("abc");
            combo.undo();
            combo.redo();
            let _ = (combo.can_undo(), combo.can_redo(), combo.can_cut(), combo.can_paste());
            combo.dismiss();
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn set_items_replaces_the_list() {
        with_combo(|combo| {
            combo.append("old");
            combo.set_items(&["a", "b"]);
            assert_eq!(combo.get_strings(), vec!["a", "b"]);
            assert_eq!(combo.get_count(), 2);
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn append_items_and_insert_items_extend_the_list() {
        with_combo(|combo| {
            combo.append_items(&["a", "b"]);
            combo.insert_items(&["x", "y"], 1);
            assert_eq!(combo.get_strings(), vec!["a", "x", "y", "b"]);
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn delete_removes_one_item_and_ignores_bad_indices() {
        with_combo(|combo| {
            combo.set_items(&["a", "b", "c"]);
            combo.delete(1);
            assert_eq!(combo.get_strings(), vec!["a", "c"]);

            combo.delete(5);
            assert_eq!(combo.get_count(), 2);
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn set_string_renames_an_item() {
        with_combo(|combo| {
            combo.set_items(&["a", "b"]);
            combo.set_string(1, "B");
            assert_eq!(combo.get_string(1).as_deref(), Some("B"));
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn find_string_respects_the_case_flag() {
        with_combo(|combo| {
            combo.set_items(&["Alpha", "beta"]);
            assert_eq!(combo.find_string("alpha", false), Some(0));
            assert_eq!(combo.find_string("alpha", true), None);
            assert_eq!(combo.find_string("gamma", false), None);
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn set_string_selection_selects_and_shows_the_item() {
        with_combo(|combo| {
            combo.set_items(&["a", "b"]);
            assert!(combo.set_string_selection("b"));
            assert_eq!(combo.get_selection(), Some(1));
            assert_eq!(combo.get_current_selection(), Some(1));
            assert_eq!(combo.get_value(), "b");

            assert!(!combo.set_string_selection("zzz"));
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn is_list_empty_tracks_the_items() {
        with_combo(|combo| {
            assert!(combo.is_list_empty());
            combo.append("a");
            assert!(!combo.is_list_empty());
            assert!(!combo.is_sorted());
        });
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn dropdown_and_closeup_events_can_be_bound() {
        with_combo(|combo| {
            combo.on_dropdown(|_| {});
            combo.on_closeup(|_| {});
        });
    }
}
