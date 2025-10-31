//! Event system for tree controls.

use crate::event::{Event, EventType};
use crate::widgets::treectrl::TreeItemId;
use std::ffi::CStr;
use wxdragon_sys as ffi;

/// Events specific to tree controls (TreeCtrl)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeEvent {
    /// Fired when the selection changes
    SelectionChanged,
    /// Fired when selection is about to change
    SelectionChanging,
    /// Fired when an item is activated (typically by double-click)
    ItemActivated,
    /// Fired when item label editing begins
    BeginLabelEdit,
    /// Fired when item label editing ends
    EndLabelEdit,
    /// Fired when an item is about to expand
    ItemExpanding,
    /// Fired when an item has expanded
    ItemExpanded,
    /// Fired when an item is about to collapse
    ItemCollapsing,
    /// Fired when an item has collapsed
    ItemCollapsed,
    /// Fired when an item is right-clicked
    ItemRightClick,
    /// Fired when mouse drag operation begins
    BeginDrag,
    /// Fired when drag operation ends
    EndDrag,
}

/// Event data for tree events
#[derive(Debug)]
pub struct TreeEventData {
    pub event: Event,
}

impl TreeEventData {
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the widget ID that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Get the TreeItemId of the affected item
    pub fn get_item(&self) -> Option<TreeItemId> {
        if self.event.is_null() {
            return None;
        }
        let item_ptr = unsafe { ffi::wxd_TreeEvent_GetItem(self.event.0) };
        unsafe { TreeItemId::from_ptr(item_ptr) }
    }

    /// Get the label text for label edit events
    pub fn get_label(&self) -> Option<String> {
        if self.event.is_null() {
            return None;
        }
        let len = unsafe { ffi::wxd_TreeEvent_GetLabel(self.event.0, std::ptr::null_mut(), 0) };
        if len < 0 {
            return None;
        }
        let mut buf = vec![0; len as usize + 1];
        if unsafe { ffi::wxd_TreeEvent_GetLabel(self.event.0, buf.as_mut_ptr(), buf.len()) } == len {
            Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
        } else {
            None
        }
    }

    /// Check if label editing was cancelled
    pub fn is_edit_cancelled(&self) -> Option<bool> {
        if self.event.is_null() {
            return None;
        }
        // Convert from C int boolean (0/1) to Rust bool
        Some(unsafe { ffi::wxd_TreeEvent_IsEditCancelled(self.event.0) != 0 })
    }
}

// Use the macro to implement the trait
crate::implement_category_event_handlers!(TreeEvents, TreeEvent, TreeEventData,
    SelectionChanged => selection_changed, EventType::TREE_SEL_CHANGED,
    SelectionChanging => selection_changing, EventType::TREE_SEL_CHANGING,
    ItemActivated => item_activated, EventType::TREE_ITEM_ACTIVATED,
    BeginLabelEdit => begin_label_edit, EventType::TREE_BEGIN_LABEL_EDIT,
    EndLabelEdit => end_label_edit, EventType::TREE_END_LABEL_EDIT,
    ItemExpanding => item_expanding, EventType::TREE_ITEM_EXPANDING,
    ItemExpanded => item_expanded, EventType::TREE_ITEM_EXPANDED,
    ItemCollapsing => item_collapsing, EventType::TREE_ITEM_COLLAPSING,
    ItemCollapsed => item_collapsed, EventType::TREE_ITEM_COLLAPSED,
    ItemRightClick => item_right_click, EventType::TREE_ITEM_RIGHT_CLICK,
    BeginDrag => begin_drag, EventType::TREE_BEGIN_DRAG,
    EndDrag => end_drag, EventType::TREE_END_DRAG
);
