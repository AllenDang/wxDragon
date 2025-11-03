//! Common event types for DataView widgets.
//!
//! This module defines event types and data structures shared by
//! DataViewCtrl, DataViewListCtrl, and DataViewTreeCtrl.

use super::item::DataViewItem;
use crate::event::{Event, EventToken, EventType, WxEvtHandler};
use wxdragon_sys as ffi;

/// Events emitted by DataView widgets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataViewEventType {
    /// Emitted when an item is selected
    SelectionChanged,
    /// Emitted when an item is activated (e.g., double-clicked)
    ItemActivated,
    /// Emitted when an item editing begins
    ItemEditingStarted,
    /// Emitted when an item editing ends successfully
    ItemEditingDone,
    /// Emitted when an item editing is canceled
    ///
    /// This uses the same underlying wxWidgets event as ItemEditingDone.
    /// To check if editing was canceled in your handler, use:
    /// ```rust,no_run
    /// # use wxdragon::prelude::*;
    /// # let data_view: DataViewCtrl = todo!();
    /// data_view.on_item_editing_cancelled(|event| {
    ///     if event.is_edit_cancelled() {
    ///         // Handle cancellation
    ///     }
    /// });
    /// ```
    ItemEditingCancelled,
    /// Emitted when an item is expanded (tree views only)
    ItemExpanded,
    /// Emitted when an item is collapsed (tree views only)
    ItemCollapsed,
    /// Emitted when a column header is clicked
    ColumnHeaderClick,
    /// Emitted when a column header is right-clicked
    ColumnHeaderRightClick,
    /// Emitted before item expansion (tree views only)
    ItemExpanding,
    /// Emitted before item collapse (tree views only)
    ItemCollapsing,
    /// Emitted when a column is sorted
    ColumnSorted,
    /// Emitted when a column is reordered
    ColumnReordered,
    /// Emitted when a context menu is requested on an item
    ///
    /// This event provides the item and column information directly.
    /// Use this instead of the generic `on_context_menu` from MenuEvents trait
    /// for better DataView-specific context information.
    ItemContextMenu,
}

/// Event data for a DataView event
#[derive(Debug)]
pub struct DataViewEvent {
    /// The underlying event
    pub event: Event,
    /// The type of event
    pub event_type: DataViewEventType,
}

impl DataViewEvent {
    /// Create a new DataViewEvent from a generic Event
    pub fn new(event: Event, event_type: DataViewEventType) -> Self {
        Self { event, event_type }
    }

    /// Get the mouse position in window coordinates
    pub fn get_position(&self) -> Option<crate::Point> {
        if self.event.is_null() {
            return None;
        }
        // Prefer DataViewEventType-specific position, which is provided by wxWidgets for
        // certain DataView events (e.g., context menu events). For other events,
        // wxWidgets returns wxDefaultPosition (-1, -1).
        let p = unsafe { ffi::wxd_DataViewEvent_GetPosition(self.event.0) };
        if p.x == -1 && p.y == -1 {
            None
        } else {
            Some(crate::Point { x: p.x, y: p.y })
        }
    }

    /// Get the ID of the control that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Skip this event (allow it to be processed by the parent window)
    pub fn skip(&self, skip: bool) {
        self.event.skip(skip);
    }

    /// Get the row that was affected by this event
    pub fn get_row(&self) -> Option<i64> {
        if self.event.is_null() {
            return None;
        }
        let mut row: i64 = 0;
        if unsafe { ffi::wxd_DataViewEvent_GetRow(self.event.0, &mut row) } {
            Some(row)
        } else {
            None
        }
    }

    /// Get the item that was affected by this event (for tree views)
    pub fn get_item(&self) -> Option<DataViewItem> {
        if self.event.is_null() {
            return None;
        }

        unsafe {
            let item_ptr = ffi::wxd_DataViewEvent_GetItem(self.event.0);
            if item_ptr.is_null() {
                None
            } else {
                // The C++ function returns a newly-allocated wrapper pointer that Rust takes ownership of
                Some(DataViewItem::from(item_ptr))
            }
        }
    }

    /// Get the column index involved in this event
    pub fn get_column(&self) -> Option<i32> {
        if self.event.is_null() {
            return None;
        }
        let mut column: i32 = 0;
        if unsafe { ffi::wxd_DataViewEvent_GetColumn(self.event.0, &mut column) } {
            Some(column)
        } else {
            None
        }
    }

    /// Get the model column involved in this event
    pub fn get_model_column(&self) -> Option<i32> {
        self.event.get_int()
    }

    /// Get whether editing was cancelled for editing events
    pub fn is_edit_cancelled(&self) -> bool {
        if self.event.is_null() {
            return false;
        }
        unsafe { ffi::wxd_DataViewEvent_IsEditCancelled(self.event.0) }
    }

    /// Get the value for editing events
    pub fn get_value(&self) -> Option<super::Variant> {
        if self.event.is_null() {
            return None;
        }
        let p = unsafe { ffi::wxd_DataViewEvent_GetValue(self.event.0) };
        if p.is_null() {
            return None;
        }
        // Wrap the returned pointer in a Variant; Rust takes ownership
        Some(super::Variant::from(p))
    }

    /// Set the value for editing events
    pub fn set_value(&self, value: &super::Variant) -> bool {
        if self.event.is_null() {
            return false;
        }

        unsafe { ffi::wxd_DataViewEvent_SetValue(self.event.0, value.as_const_ptr()) }
    }

    /// Returns whether the sort order is ascending for column-sorted events.
    ///
    /// This method is only meaningful for [`DataViewEventType::ColumnSorted`] events
    /// (i.e., when handling `wxEVT_DATAVIEW_COLUMN_SORTED`). For all other event types,
    /// this method will return `None`.
    ///
    /// # Example
    /// Typical usage within an `on_column_sorted` handler:
    /// ```no_run
    /// # use wxdragon::prelude::*;
    /// # let data_view: DataViewCtrl = todo!();
    /// data_view.on_column_sorted(|event| {
    ///     if let Some(ascending) = event.get_sort_order() {
    ///         if ascending {
    ///             // Handle ascending sort
    ///         } else {
    ///             // Handle descending sort
    ///         }
    ///     }
    /// });
    /// ```
    pub fn get_sort_order(&self) -> Option<bool> {
        if self.event.is_null() {
            return None;
        }
        let mut ascending = false;
        if unsafe { ffi::wxd_DataViewEvent_GetSortOrder(self.event.0, &mut ascending) } {
            Some(ascending)
        } else {
            None
        }
    }
}

/// Trait for DataView event handling
pub trait DataViewEventHandler: WxEvtHandler {
    /// Bind an event handler for DataView events.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn bind_dataview_event<F>(&self, event: DataViewEventType, mut callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        // Map enum variant to EventType
        let event_type = match event {
            DataViewEventType::SelectionChanged => EventType::DATAVIEW_SELECTION_CHANGED,
            DataViewEventType::ItemActivated => EventType::DATAVIEW_ITEM_ACTIVATED,
            DataViewEventType::ItemEditingStarted => EventType::DATAVIEW_ITEM_EDITING_STARTED,
            DataViewEventType::ItemEditingDone => EventType::DATAVIEW_ITEM_EDITING_DONE,
            DataViewEventType::ItemEditingCancelled => EventType::DATAVIEW_ITEM_EDITING_DONE, // Same underlying event as ItemEditingDone
            DataViewEventType::ItemExpanded => EventType::DATAVIEW_ITEM_EXPANDED,
            DataViewEventType::ItemCollapsed => EventType::DATAVIEW_ITEM_COLLAPSED,
            DataViewEventType::ColumnHeaderClick => EventType::DATAVIEW_COLUMN_HEADER_CLICK,
            DataViewEventType::ColumnHeaderRightClick => EventType::DATAVIEW_COLUMN_HEADER_RIGHT_CLICK,
            DataViewEventType::ItemExpanding => EventType::DATAVIEW_ITEM_EXPANDING,
            DataViewEventType::ItemCollapsing => EventType::DATAVIEW_ITEM_COLLAPSING,
            DataViewEventType::ColumnSorted => EventType::DATAVIEW_COLUMN_SORTED,
            DataViewEventType::ColumnReordered => EventType::DATAVIEW_COLUMN_REORDERED,
            DataViewEventType::ItemContextMenu => EventType::DATAVIEW_ITEM_CONTEXT_MENU,
        };

        // Create wrapper with special handling for editing cancelled events
        let wrapper = move |base_event: Event| {
            // For ItemEditingCancelled events, only trigger callback if editing was actually cancelled
            if event == DataViewEventType::ItemEditingCancelled {
                let data = DataViewEvent::new(base_event, event);
                if data.is_edit_cancelled() {
                    callback(data);
                }
            } else if event == DataViewEventType::ItemEditingDone {
                // For ItemEditingDone events, only trigger callback if editing was NOT cancelled
                let data = DataViewEvent::new(base_event, event);
                if !data.is_edit_cancelled() {
                    callback(data);
                }
            } else {
                // For all other events, pass through normally
                let data = DataViewEvent::new(base_event, event);
                callback(data);
            }
        };

        // Use internal bind method and return the token
        WxEvtHandler::bind_internal(self, event_type, wrapper)
    }

    /// Binds a handler to the selection changed event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_selection_changed<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::SelectionChanged, callback)
    }

    /// Binds a handler to the item activated event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_item_activated<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ItemActivated, callback)
    }

    /// Binds a handler to the item editing started event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_item_editing_started<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ItemEditingStarted, callback)
    }

    /// Binds a handler to the item editing done event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_item_editing_done<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ItemEditingDone, callback)
    }

    /// Binds a handler to the item editing cancelled event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_item_editing_cancelled<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ItemEditingCancelled, callback)
    }

    /// Binds a handler to the column header click event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_column_header_click<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ColumnHeaderClick, callback)
    }

    /// Binds a handler to the column header right click event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_column_header_right_click<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ColumnHeaderRightClick, callback)
    }

    /// Binds a handler to the column sorted event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_column_sorted<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ColumnSorted, callback)
    }

    /// Binds a handler to the column reordered event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_column_reordered<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ColumnReordered, callback)
    }

    /// Binds a handler to the item context menu event.
    /// Returns an EventToken that can be used to unbind the handler later.
    ///
    /// This event is emitted when a context menu is requested on an item
    /// (e.g., via right-click or keyboard). The event provides direct access
    /// to the item and column information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use wxdragon::{DataViewCtrl, DataViewEventHandler};
    ///
    /// # let data_view: DataViewCtrl = todo!();
    /// data_view.on_item_context_menu(|event| {
    ///     if let Some(item) = event.get_item() {
    ///         if let Some(col) = event.get_column() {
    ///             println!("Context menu requested on item at column {}", col);
    ///             // Show a popup menu here
    ///         }
    ///     }
    /// });
    /// ```
    fn on_item_context_menu<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ItemContextMenu, callback)
    }
}

/// Extension trait for TreeView-specific events
pub trait DataViewTreeEventHandler: DataViewEventHandler {
    /// Binds a handler to the item expanded event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_item_expanded<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ItemExpanded, callback)
    }

    /// Binds a handler to the item collapsed event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_item_collapsed<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ItemCollapsed, callback)
    }

    /// Binds a handler to the item expanding event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_item_expanding<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ItemExpanding, callback)
    }

    /// Binds a handler to the item collapsing event.
    /// Returns an EventToken that can be used to unbind the handler later.
    fn on_item_collapsing<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(DataViewEvent) + 'static,
    {
        self.bind_dataview_event(DataViewEventType::ItemCollapsing, callback)
    }
}
