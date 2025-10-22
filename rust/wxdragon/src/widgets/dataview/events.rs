//! Common event types for DataView widgets.
//!
//! This module defines event types and data structures shared by
//! DataViewCtrl, DataViewListCtrl, and DataViewTreeCtrl.

use super::item::DataViewItem;
use crate::event::{Event, EventBinding, EventType, WxEvtHandler};
use wxdragon_sys as ffi;

/// Events emitted by DataView widgets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataViewEvent {
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
    /// ```
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
pub struct DataViewEventData {
    /// The underlying event
    pub event: Event,
    /// The type of event
    pub event_type: DataViewEvent,
}

impl DataViewEventData {
    /// Create a new DataViewEventData from a generic Event
    pub fn new(event: Event, event_type: DataViewEvent) -> Self {
        Self { event, event_type }
    }

    /// Get the mouse position in window coordinates
    pub fn get_position(&self) -> Option<crate::Point> {
        self.event.get_position()
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

        // Create a temporary variant struct to hold the returned data
        let variant_raw = Box::new(unsafe { std::mem::zeroed::<ffi::wxd_Variant_t>() });
        let variant_ptr = Box::into_raw(variant_raw);

        if unsafe { ffi::wxd_DataViewEvent_GetValue(self.event.0, variant_ptr) } {
            // Convert the C++ variant to a Rust Variant, taking ownership and freeing the C resources
            unsafe { super::Variant::from_raw(variant_ptr) }
        } else {
            // Free the memory if the call failed
            unsafe { ffi::wxd_Variant_Free(variant_ptr) };
            None
        }
    }

    /// Set the value for editing events
    pub fn set_value(&self, value: &super::Variant) -> bool {
        if self.event.is_null() {
            return false;
        }

        // Clone the variant since we need to transfer ownership to C++
        let variant_clone = value.clone();

        // Use into_raw to properly transfer ownership to C++
        // C++ side will free the memory using wxd_Variant_Free
        unsafe { ffi::wxd_DataViewEvent_SetValue(self.event.0, variant_clone.into_raw()) }
    }
}

/// Trait for DataView event handling
pub trait DataViewEventHandler: WxEvtHandler {
    /// Bind an event handler for DataView events
    fn bind_dataview_event<F>(&self, event: DataViewEvent, mut callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        // Map enum variant to EventType
        let event_type = match event {
            DataViewEvent::SelectionChanged => EventType::DATAVIEW_SELECTION_CHANGED,
            DataViewEvent::ItemActivated => EventType::DATAVIEW_ITEM_ACTIVATED,
            DataViewEvent::ItemEditingStarted => EventType::DATAVIEW_ITEM_EDITING_STARTED,
            DataViewEvent::ItemEditingDone => EventType::DATAVIEW_ITEM_EDITING_DONE,
            DataViewEvent::ItemEditingCancelled => EventType::DATAVIEW_ITEM_EDITING_DONE, // Same underlying event as ItemEditingDone
            DataViewEvent::ItemExpanded => EventType::DATAVIEW_ITEM_EXPANDED,
            DataViewEvent::ItemCollapsed => EventType::DATAVIEW_ITEM_COLLAPSED,
            DataViewEvent::ColumnHeaderClick => EventType::DATAVIEW_COLUMN_HEADER_CLICK,
            DataViewEvent::ColumnHeaderRightClick => EventType::DATAVIEW_COLUMN_HEADER_RIGHT_CLICK,
            DataViewEvent::ItemExpanding => EventType::DATAVIEW_ITEM_EXPANDING,
            DataViewEvent::ItemCollapsing => EventType::DATAVIEW_ITEM_COLLAPSING,
            DataViewEvent::ColumnSorted => EventType::DATAVIEW_COLUMN_SORTED,
            DataViewEvent::ColumnReordered => EventType::DATAVIEW_COLUMN_REORDERED,
            DataViewEvent::ItemContextMenu => EventType::DATAVIEW_ITEM_CONTEXT_MENU,
        };

        // Create wrapper with special handling for editing cancelled events
        let wrapper = move |base_event: Event| {
            // For ItemEditingCancelled events, only trigger callback if editing was actually cancelled
            if event == DataViewEvent::ItemEditingCancelled {
                let data = DataViewEventData::new(base_event, event);
                if data.is_edit_cancelled() {
                    callback(data);
                }
            } else if event == DataViewEvent::ItemEditingDone {
                // For ItemEditingDone events, only trigger callback if editing was NOT cancelled
                let data = DataViewEventData::new(base_event, event);
                if !data.is_edit_cancelled() {
                    callback(data);
                }
            } else {
                // For all other events, pass through normally
                let data = DataViewEventData::new(base_event, event);
                callback(data);
            }
        };

        // Use internal bind method
        WxEvtHandler::bind_internal(self, event_type, wrapper);
    }

    /// Bind an event handler and return an RAII EventBinding handle that will unbind on drop
    fn bind_dataview_event_with_handle<F>(
        &self,
        event: DataViewEvent,
        mut callback: F,
    ) -> EventBinding
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        // Map enum variant to EventType
        let event_type = match event {
            DataViewEvent::SelectionChanged => EventType::DATAVIEW_SELECTION_CHANGED,
            DataViewEvent::ItemActivated => EventType::DATAVIEW_ITEM_ACTIVATED,
            DataViewEvent::ItemEditingStarted => EventType::DATAVIEW_ITEM_EDITING_STARTED,
            DataViewEvent::ItemEditingDone => EventType::DATAVIEW_ITEM_EDITING_DONE,
            DataViewEvent::ItemEditingCancelled => EventType::DATAVIEW_ITEM_EDITING_DONE,
            DataViewEvent::ItemExpanded => EventType::DATAVIEW_ITEM_EXPANDED,
            DataViewEvent::ItemCollapsed => EventType::DATAVIEW_ITEM_COLLAPSED,
            DataViewEvent::ColumnHeaderClick => EventType::DATAVIEW_COLUMN_HEADER_CLICK,
            DataViewEvent::ColumnHeaderRightClick => EventType::DATAVIEW_COLUMN_HEADER_RIGHT_CLICK,
            DataViewEvent::ItemExpanding => EventType::DATAVIEW_ITEM_EXPANDING,
            DataViewEvent::ItemCollapsing => EventType::DATAVIEW_ITEM_COLLAPSING,
            DataViewEvent::ColumnSorted => EventType::DATAVIEW_COLUMN_SORTED,
            DataViewEvent::ColumnReordered => EventType::DATAVIEW_COLUMN_REORDERED,
            DataViewEvent::ItemContextMenu => EventType::DATAVIEW_ITEM_CONTEXT_MENU,
        };

        // Create wrapper with special handling for editing cancelled events
        let wrapper = move |base_event: Event| {
            if event == DataViewEvent::ItemEditingCancelled {
                let data = DataViewEventData::new(base_event, event);
                if data.is_edit_cancelled() {
                    callback(data);
                }
            } else if event == DataViewEvent::ItemEditingDone {
                let data = DataViewEventData::new(base_event, event);
                if !data.is_edit_cancelled() {
                    callback(data);
                }
            } else {
                let data = DataViewEventData::new(base_event, event);
                callback(data);
            }
        };

        WxEvtHandler::bind_internal_return_handle(self, event_type, wrapper)
    }

    /// Binds a handler to the selection changed event
    fn on_selection_changed<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::SelectionChanged, callback)
    }

    /// Binds a handler to the item activated event
    fn on_item_activated<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ItemActivated, callback)
    }

    /// Binds a handler to the item editing started event
    fn on_item_editing_started<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ItemEditingStarted, callback)
    }

    /// Binds a handler to the item editing done event
    fn on_item_editing_done<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ItemEditingDone, callback)
    }

    /// Binds a handler to the item editing cancelled event
    fn on_item_editing_cancelled<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ItemEditingCancelled, callback)
    }

    /// Binds a handler to the column header click event
    fn on_column_header_click<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ColumnHeaderClick, callback)
    }

    /// Binds a handler to the column header right click event
    fn on_column_header_right_click<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ColumnHeaderRightClick, callback)
    }

    /// Binds a handler to the column sorted event
    fn on_column_sorted<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ColumnSorted, callback)
    }

    /// Binds a handler to the column reordered event
    fn on_column_reordered<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ColumnReordered, callback)
    }

    /// Binds a handler to the item context menu event
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
    fn on_item_context_menu<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ItemContextMenu, callback)
    }

    /// Binds a handler to the item context menu event and returns an EventBinding handle
    fn on_item_context_menu_handle<F>(&self, callback: F) -> EventBinding
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event_with_handle(DataViewEvent::ItemContextMenu, callback)
    }
}

/// Extension trait for TreeView-specific events
pub trait TreeViewEventHandler: DataViewEventHandler {
    /// Binds a handler to the item expanded event
    fn on_item_expanded<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ItemExpanded, callback)
    }

    /// Binds a handler to the item collapsed event
    fn on_item_collapsed<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ItemCollapsed, callback)
    }

    /// Binds a handler to the item expanding event
    fn on_item_expanding<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ItemExpanding, callback)
    }

    /// Binds a handler to the item collapsing event
    fn on_item_collapsing<F>(&self, callback: F)
    where
        F: FnMut(DataViewEventData) + 'static,
    {
        self.bind_dataview_event(DataViewEvent::ItemCollapsing, callback)
    }
}
