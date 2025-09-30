//! Safe wrapper for wxRearrangeList.

use crate::event::{Event, EventType};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
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
#[derive(Clone)]
pub struct RearrangeList {
    window: Window,
}

impl RearrangeList {
    /// Creates a new `RearrangeListBuilder` for constructing a rearrange list control.
    pub fn builder(parent: &dyn WxWidget) -> RearrangeListBuilder<'_> {
        RearrangeListBuilder::new(parent)
    }

    /// Gets the current order of items in the list.
    ///
    /// The returned vector contains values that represent both the order and checked state of items:
    /// - Positive values (n) represent checked items at the original position n.
    /// - Negative values (~n) represent unchecked items at the original position n.
    pub fn get_current_order(&self) -> Vec<i32> {
        unsafe {
            let count = self.get_count() as usize;

            // Create a buffer to receive the order array
            let mut buffer: Vec<i32> = vec![0; count];

            // Call the C API to fill the buffer
            ffi::wxd_RearrangeList_GetCurrentOrder(
                self.window.as_ptr() as *mut _,
                buffer.as_mut_ptr(),
                count as i32,
            );

            buffer
        }
    }

    /// Move the currently selected item one position up.
    ///
    /// Returns true if the item was moved, false if it couldn't be moved
    /// (e.g., if it's already at the top).
    pub fn move_current_up(&self) -> bool {
        unsafe { ffi::wxd_RearrangeList_MoveCurrentUp(self.window.as_ptr() as *mut _) }
    }

    /// Move the currently selected item one position down.
    ///
    /// Returns true if the item was moved, false if it couldn't be moved
    /// (e.g., if it's already at the bottom).
    pub fn move_current_down(&self) -> bool {
        unsafe { ffi::wxd_RearrangeList_MoveCurrentDown(self.window.as_ptr() as *mut _) }
    }

    /// Check if the currently selected item can be moved up.
    pub fn can_move_current_up(&self) -> bool {
        unsafe { ffi::wxd_RearrangeList_CanMoveCurrentUp(self.window.as_ptr() as *mut _) }
    }

    /// Check if the currently selected item can be moved down.
    pub fn can_move_current_down(&self) -> bool {
        unsafe { ffi::wxd_RearrangeList_CanMoveCurrentDown(self.window.as_ptr() as *mut _) }
    }

    /// Gets the index of the currently selected item.
    /// Returns `None` if no item is selected.
    pub fn get_selection(&self) -> Option<u32> {
        let selection =
            unsafe { ffi::wxd_RearrangeList_GetSelection(self.window.as_ptr() as *mut _) };
        if selection == -1 {
            None
        } else {
            Some(selection as u32)
        }
    }

    /// Sets the selection to the item at the given index.
    pub fn set_selection(&self, index: u32, select: bool) {
        unsafe {
            ffi::wxd_RearrangeList_SetSelection(
                self.window.as_ptr() as *mut _,
                index as i32,
                select,
            );
        }
    }

    /// Gets the string at the specified index.
    /// Returns `None` if the index is out of bounds.
    pub fn get_string(&self, index: u32) -> Option<String> {
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed = ffi::wxd_RearrangeList_GetString(
                self.window.as_ptr() as *mut _,
                index as i32,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None; // Error or invalid index
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_RearrangeList_GetString(
                    self.window.as_ptr() as *mut _,
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

    /// Gets the number of items in the list.
    pub fn get_count(&self) -> u32 {
        unsafe { ffi::wxd_RearrangeList_GetCount(self.window.as_ptr() as *mut _) }
    }

    /// Checks or unchecks an item at the given index.
    pub fn check(&self, index: u32, check: bool) {
        unsafe {
            // Cast the *mut wxd_Window_t to *mut wxd_RearrangeList_t for the FFI call
            let list_ptr = self.window.0 as *mut ffi::wxd_RearrangeList_t;
            ffi::wxd_RearrangeList_Check(
                list_ptr, index, // FFI function now takes u32 (unsigned int in C++)
                check,
            );
            // No pointer update needed as the control is not recreated.
        }
    }

    /// Checks if the item at the given index is currently checked.
    pub fn is_checked(&self, index: u32) -> bool {
        unsafe { ffi::wxd_RearrangeList_IsChecked(self.window.as_ptr() as *mut _, index as i32) }
    }
}

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
            window: unsafe { Window::from_ptr(ctrl_ptr as *mut ffi::wxd_Window_t) },
        }
    }
);

implement_widget_traits_with_target!(RearrangeList, window, Window);

// Add XRC Support - enables RearrangeList to be created from XRC-managed pointers
impl_xrc_support!(RearrangeList, { window });
