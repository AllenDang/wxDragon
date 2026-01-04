// ! Safe wrapper for wxCheckListBox.

use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

// Create a style enum for CheckListBox, reusing the values from ListBoxStyle
widget_style_enum!(
    name: CheckListBoxStyle,
    doc: "Style flags for the CheckListBox widget.",
    variants: {
        Default: 0, "Default style.",
        Single: ffi::WXD_LB_SINGLE, "Single-selection list.",
        Multiple: ffi::WXD_LB_MULTIPLE, "Multiple-selection list.",
        Extended: ffi::WXD_LB_EXTENDED, "Extended-selection list.",
        HScroll: ffi::WXD_LB_HSCROLL, "Create horizontal scrollbar if contents are too wide.",
        AlwaysSB: ffi::WXD_LB_ALWAYS_SB, "Always show a vertical scrollbar.",
        Sort: ffi::WXD_LB_SORT, "Sort strings in the list alphabetically."
    },
    default_variant: Default
);

/// Events emitted by CheckListBox
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckListBoxEvent {
    /// Emitted when an item is selected
    Selected,
    /// Emitted when a checkbox is toggled
    Toggled,
    /// Emitted when an item is double-clicked
    DoubleClicked,
}

/// Event data for CheckListBox events
#[derive(Debug)]
pub struct CheckListBoxEventData {
    event: Event,
}

impl CheckListBoxEventData {
    /// Create a new CheckListBoxEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the index of the item that was selected or toggled
    pub fn get_selection(&self) -> Option<u32> {
        // For CheckListBox events, GetInt() returns the selection index
        self.event.get_int().map(|i| i as u32)
    }

    /// Get the text of the item that was selected or toggled
    pub fn get_string(&self) -> Option<String> {
        self.event.get_string()
    }

    /// Get whether the checkbox was checked or unchecked (for Toggled events)
    pub fn is_checked(&self) -> Option<bool> {
        self.event.is_checked()
    }
}

/// Represents a wxCheckListBox control, which combines a ListBox with checkboxes.
///
/// CheckListBox uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let checklist = CheckListBox::builder(&frame).build();
///
/// // CheckListBox is Copy - no clone needed for closures!
/// checklist.bind_toggled(move |_| {
///     // Safe: if checklist was destroyed, this is a no-op
///     checklist.append("New item");
/// });
///
/// // After parent destruction, checklist operations are safe no-ops
/// frame.destroy();
/// assert!(!checklist.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct CheckListBox {
    /// Safe handle to the underlying wxCheckListBox - automatically invalidated on destroy
    handle: WindowHandle,
}

impl CheckListBox {
    /// Creates a new `CheckListBoxBuilder` for constructing a check list box control.
    pub fn builder(parent: &dyn WxWidget) -> CheckListBoxBuilder<'_> {
        CheckListBoxBuilder::new(parent)
    }

    /// Helper to get raw checklistbox pointer, returns null if widget has been destroyed
    #[inline]
    fn checklistbox_ptr(&self) -> *mut ffi::wxd_CheckListBox_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_CheckListBox_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Appends an item to the list box.
    /// No-op if the checklistbox has been destroyed.
    pub fn append(&self, item: &str) {
        let ptr = self.checklistbox_ptr();
        if ptr.is_null() {
            return;
        }
        let c_item = CString::new(item).expect("Invalid CString for CheckListBox item");
        unsafe {
            ffi::wxd_CheckListBox_Append(ptr, c_item.as_ptr());
        }
    }

    /// Removes all items from the list box.
    /// No-op if the checklistbox has been destroyed.
    pub fn clear(&self) {
        let ptr = self.checklistbox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_CheckListBox_Clear(ptr);
        }
    }

    /// Gets the index of the currently selected item.
    /// Returns `None` if no item is selected (matches `NOT_FOUND`) or if destroyed.
    pub fn get_selection(&self) -> Option<u32> {
        let ptr = self.checklistbox_ptr();
        if ptr.is_null() {
            return None;
        }
        let selection = unsafe { ffi::wxd_CheckListBox_GetSelection(ptr) };
        if selection == -1 { None } else { Some(selection as u32) }
    }

    /// Gets the string value of the currently selected item.
    /// Returns `None` if no item is selected or if destroyed.
    pub fn get_string_selection(&self) -> Option<String> {
        let ptr = self.checklistbox_ptr();
        if ptr.is_null() {
            return None;
        }
        let len = unsafe { ffi::wxd_CheckListBox_GetStringSelection(ptr, std::ptr::null_mut(), 0) };

        if len < 0 {
            return None; // Error or no selection
        }

        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_CheckListBox_GetStringSelection(ptr, buf.as_mut_ptr(), buf.len()) };
        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
    }

    /// Selects or deselects an item at the given index.
    /// No-op if the checklistbox has been destroyed.
    pub fn set_selection(&self, index: u32, select: bool) {
        let ptr = self.checklistbox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_CheckListBox_SetSelection(ptr, index as i32, select) };
    }

    /// Gets the string at the specified index.
    /// Returns `None` if the index is out of bounds or if destroyed.
    pub fn get_string(&self, index: usize) -> Option<String> {
        let ptr = self.checklistbox_ptr();
        if ptr.is_null() {
            return None;
        }
        let len = unsafe { ffi::wxd_CheckListBox_GetString(ptr, index, std::ptr::null_mut(), 0) };

        if len < 0 {
            return None; // Index out of bounds
        }

        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_CheckListBox_GetString(ptr, index, buf.as_mut_ptr(), buf.len()) };
        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
    }

    /// Gets the number of items in the list box.
    /// Returns 0 if the checklistbox has been destroyed.
    pub fn get_count(&self) -> u32 {
        let ptr = self.checklistbox_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_CheckListBox_GetCount(ptr) }
    }

    /// Checks if the item at the given index is checked.
    /// Returns `false` if the index is out of bounds or if destroyed.
    pub fn is_checked(&self, index: u32) -> bool {
        let ptr = self.checklistbox_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_CheckListBox_IsChecked(ptr, index) }
    }

    /// Sets the checked state of the item at the given index.
    /// Does nothing if the index is out of bounds or if destroyed.
    pub fn check(&self, index: u32, check: bool) {
        let ptr = self.checklistbox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_CheckListBox_Check(ptr, index, check) }
    }

    /// Returns the underlying WindowHandle for this checklistbox.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for CheckListBox (using WindowHandle)
impl WxWidget for CheckListBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for CheckListBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for CheckListBox {}

// Implement event handlers for CheckListBox
crate::implement_widget_local_event_handlers!(
    CheckListBox,
    CheckListBoxEvent,
    CheckListBoxEventData,
    Selected => selected, EventType::COMMAND_LISTBOX_SELECTED,
    Toggled => toggled, EventType::COMMAND_CHECKLISTBOX_SELECTED,
    DoubleClicked => double_clicked, EventType::COMMAND_LISTBOX_DOUBLECLICKED
);

// XRC Support - enables CheckListBox to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for CheckListBox {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        CheckListBox {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for CheckListBox
impl crate::window::FromWindowWithClassName for CheckListBox {
    fn class_name() -> &'static str {
        "wxCheckListBox"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        CheckListBox {
            handle: WindowHandle::new(ptr),
        }
    }
}

widget_builder!(
    name: CheckListBox,
    parent_type: &'a dyn WxWidget,
    style_type: CheckListBoxStyle,
    fields: {
        choices: Vec<String> = Vec::new()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let pos = slf.pos.into();
        let size = slf.size.into();

        // Create the control
        let ctrl_ptr = unsafe {
            ffi::wxd_CheckListBox_Create(
                parent_ptr,
                slf.id,
                pos,
                size,
                slf.style.bits(),
            )
        };

        if ctrl_ptr.is_null() {
            panic!("Failed to create CheckListBox widget");
        }

        // Create a WindowHandle which automatically registers for destroy events
        let clbox = CheckListBox {
            handle: WindowHandle::new(ctrl_ptr as *mut ffi::wxd_Window_t),
        };

        // Append initial choices
        for choice_str in &slf.choices {
            clbox.append(choice_str);
        }

        clbox
    }
);
