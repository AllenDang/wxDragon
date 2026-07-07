//! Event system for notebook controls.

use crate::event::{Event, EventType, WxEvtHandler};

/// Events specific to notebook controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotebookEvent {
    /// A notebook page has been changed.
    PageChanged,
    /// A notebook page is about to change. This event can be vetoed.
    PageChanging,
}

/// Event data for notebook events.
#[derive(Debug)]
pub struct NotebookEventData {
    /// The base event.
    pub event: Event,
}

impl NotebookEventData {
    /// Creates a new `NotebookEventData` from a base `Event`.
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Gets the newly selected page index.
    pub fn get_selection(&self) -> Option<i32> {
        self.event.get_selection()
    }

    /// Gets the previously selected page index.
    pub fn get_old_selection(&self) -> Option<i32> {
        self.event.get_old_selection()
    }
}

/// Trait for controls that have notebook-like behavior.
pub trait NotebookEvents: WxEvtHandler {}

// Use the macro to implement the trait
crate::implement_category_event_handlers!(
    NotebookEvents, NotebookEvent, NotebookEventData,
    PageChanged => page_changed, EventType::NOTEBOOK_PAGE_CHANGED,
    PageChanging => page_changing, EventType::NOTEBOOK_PAGE_CHANGING
);