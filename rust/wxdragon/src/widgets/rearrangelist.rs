//! Safe wrapper for wxRearrangeList.

use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by XRC support for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// --- Style enum using macro ---
widget_style_enum!(
    name: RearrangeListStyle,
    doc: "Style flags for RearrangeList widget.",
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

/// Events emitted by RearrangeList
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RearrangeListEvent {
    /// Emitted when an item is selected
    Selected,
    /// Emitted when an item is checked/unchecked
    Toggled,
    /// Emitted when items are rearranged
    Rearranged,
}

/// Event data for RearrangeList events
#[derive(Debug)]
pub struct RearrangeListEventData {
    event: Event,
}

impl RearrangeListEventData {
    /// Create a new RearrangeListEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the index of the item that was selected or toggled
    pub fn get_selection(&self) -> Option<u32> {
        self.event.get_int().map(|i| i as u32)
    }

    /// Get the text of the item that was selected or toggled
    pub fn get_string(&self) -> Option<String> {
        self.event.get_string()
    }

    /// Get whether the item was checked or unchecked (for Toggled events)
    pub fn is_checked(&self) -> Option<bool> {
        self.event.is_checked()
    }
}

/// Represents a wxRearrangeList control, which allows reordering and checking/unchecking items.
///
/// RearrangeList uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let list = RearrangeList::builder(&frame).items(vec!["Item 1".to_string()]).build();
///
/// // RearrangeList is Copy - no clone needed for closures!
/// list.bind_selected(move |_| {
///     // Safe: if list was destroyed, this is a no-op
///     if let Some(sel) = list.get_selection() {
///         println!("Selected: {}", sel);
///     }
/// });
///
/// // After parent destruction, list operations are safe no-ops
/// frame.destroy();
/// assert!(!list.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct RearrangeList {
    /// Safe handle to the underlying wxRearrangeList - automatically invalidated on destroy
    handle: WindowHandle,
}

impl RearrangeList {
    /// Creates a new `RearrangeListBuilder` for constructing a rearrange list control.
    pub fn builder(parent: &dyn WxWidget) -> RearrangeListBuilder<'_> {
        RearrangeListBuilder::new(parent)
    }

    /// Helper to get raw rearrangelist pointer, returns null if widget has been destroyed
    #[inline]
    fn rearrangelist_ptr(&self) -> *mut ffi::wxd_RearrangeList_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_RearrangeList_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Gets the current order of items in the list.
    /// Returns empty vector if the widget has been destroyed.
    ///
    /// The returned vector contains values that represent both the order and checked state of items:
    /// - Positive values (n) represent checked items at the original position n.
    /// - Negative values (~n) represent unchecked items at the original position n.
    pub fn get_current_order(&self) -> Vec<i32> {
        let ptr = self.rearrangelist_ptr();
        if ptr.is_null() {
            return Vec::new();
        }
        unsafe {
            let count = self.get_count() as usize;

            // Create a buffer to receive the order array
            let mut buffer: Vec<i32> = vec![0; count];

            // Call the C API to fill the buffer
            ffi::wxd_RearrangeList_GetCurrentOrder(ptr, buffer.as_mut_ptr(), count as i32);

            buffer
        }
    }

    /// Move the currently selected item one position up.
    /// Returns false if the widget has been destroyed.
    ///
    /// Returns true if the item was moved, false if it couldn't be moved
    /// (e.g., if it's already at the top).
    pub fn move_current_up(&self) -> bool {
        let ptr = self.rearrangelist_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RearrangeList_MoveCurrentUp(ptr) }
    }

    /// Move the currently selected item one position down.
    /// Returns false if the widget has been destroyed.
    ///
    /// Returns true if the item was moved, false if it couldn't be moved
    /// (e.g., if it's already at the bottom).
    pub fn move_current_down(&self) -> bool {
        let ptr = self.rearrangelist_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RearrangeList_MoveCurrentDown(ptr) }
    }

    /// Check if the currently selected item can be moved up.
    /// Returns false if the widget has been destroyed.
    pub fn can_move_current_up(&self) -> bool {
        let ptr = self.rearrangelist_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RearrangeList_CanMoveCurrentUp(ptr) }
    }

    /// Check if the currently selected item can be moved down.
    /// Returns false if the widget has been destroyed.
    pub fn can_move_current_down(&self) -> bool {
        let ptr = self.rearrangelist_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RearrangeList_CanMoveCurrentDown(ptr) }
    }

    /// Gets the index of the currently selected item.
    /// Returns `None` if no item is selected or if the widget has been destroyed.
    pub fn get_selection(&self) -> Option<u32> {
        let ptr = self.rearrangelist_ptr();
        if ptr.is_null() {
            return None;
        }
        let selection = unsafe { ffi::wxd_RearrangeList_GetSelection(ptr) };
        if selection == -1 { None } else { Some(selection as u32) }
    }

    /// Sets the selection to the item at the given index.
    /// No-op if the widget has been destroyed.
    pub fn set_selection(&self, index: u32, select: bool) {
        let ptr = self.rearrangelist_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RearrangeList_SetSelection(ptr, index as i32, select) };
    }

    /// Gets the string at the specified index.
    /// Returns `None` if the index is out of bounds or if the widget has been destroyed.
    pub fn get_string(&self, index: u32) -> Option<String> {
        let ptr = self.rearrangelist_ptr();
        if ptr.is_null() {
            return None;
        }
        let mut buffer = [0; 1024];
        let len_needed = unsafe { ffi::wxd_RearrangeList_GetString(ptr, index as i32, buffer.as_mut_ptr(), buffer.len()) };

        if len_needed < 0 {
            return None; // Error or invalid index
        }

        if len_needed < buffer.len() as i32 {
            let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
            Some(c_str.to_string_lossy().into_owned())
        } else {
            let mut buf = vec![0; len_needed as usize + 1];
            let len_copied = unsafe { ffi::wxd_RearrangeList_GetString(ptr, index as i32, buf.as_mut_ptr(), buf.len()) };
            if len_copied == len_needed {
                Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
            } else {
                None
            }
        }
    }

    /// Gets the number of items in the list.
    /// Returns 0 if the widget has been destroyed.
    pub fn get_count(&self) -> u32 {
        let ptr = self.rearrangelist_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_RearrangeList_GetCount(ptr) }
    }

    /// Checks or unchecks an item at the given index.
    /// No-op if the widget has been destroyed.
    pub fn check(&self, index: u32, check: bool) {
        let ptr = self.rearrangelist_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_RearrangeList_Check(
                ptr, index, // FFI function now takes u32 (unsigned int in C++)
                check,
            );
        }
    }

    /// Checks if the item at the given index is currently checked.
    /// Returns false if the widget has been destroyed.
    pub fn is_checked(&self, index: u32) -> bool {
        let ptr = self.rearrangelist_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RearrangeList_IsChecked(ptr, index as i32) }
    }
}

// Manual WxWidget implementation for RearrangeList (using WindowHandle)
impl WxWidget for RearrangeList {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for RearrangeList {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for RearrangeList {}

// Implement event handlers for RearrangeList
crate::implement_widget_local_event_handlers!(
    RearrangeList,
    RearrangeListEvent,
    RearrangeListEventData,
    Selected => selected, EventType::COMMAND_LISTBOX_SELECTED,
    Toggled => toggled, EventType::COMMAND_CHECKLISTBOX_SELECTED,
    Rearranged => rearranged, EventType::COMMAND_REARRANGE_LIST
);

widget_builder!(
    name: RearrangeList,
    parent_type: &'a dyn WxWidget,
    style_type: RearrangeListStyle,
    fields: {
        items: Vec<String> = Vec::new(),
        order: Vec<i32> = Vec::new()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let pos = slf.pos.into();
        let size = slf.size.into();

        // Prepare items for FFI
        let items_count = slf.items.len();
        let c_items: Vec<CString> = slf.items.iter()
            .map(|s| CString::new(s.as_str()).expect("Invalid CString for RearrangeList item"))
            .collect();
        let c_items_ptrs: Vec<*const c_char> = c_items.iter()
            .map(|cs| cs.as_ptr())
            .collect();

        // Use the provided order or generate a default one
        let order = if !slf.order.is_empty() {
            slf.order.clone()
        } else {
            // Default order: all items are checked and in original order
            (0..items_count as i32).collect()
        };

        // Create the control
        let ctrl_ptr = unsafe {
            ffi::wxd_RearrangeList_Create(
                parent_ptr,
                slf.id,
                pos,
                size,
                order.as_ptr(),
                order.len() as i32,
                c_items_ptrs.as_ptr() as *mut *const c_char,
                items_count as i32,
                slf.style.bits(),
            )
        };

        if ctrl_ptr.is_null() {
            panic!("Failed to create RearrangeList widget");
        }

        RearrangeList {
            handle: WindowHandle::new(ctrl_ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// XRC Support - enables RearrangeList to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for RearrangeList {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        RearrangeList {
            handle: WindowHandle::new(ptr),
        }
    }
}
