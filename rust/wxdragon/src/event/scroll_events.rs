//! Event system for scrollable controls.

use crate::event::{Event, EventType};
use wxdragon_sys as ffi;

/// Events specific to scrollable controls (ScrollBar, Slider, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollEventType {
    /// Fired when scrolling to the top
    Top,
    /// Fired when scrolling to the bottom
    Bottom,
    /// Fired when scrolling up one line
    LineUp,
    /// Fired when scrolling down one line
    LineDown,
    /// Fired when scrolling up one page
    PageUp,
    /// Fired when scrolling down one page
    PageDown,
    /// Fired while thumb is being dragged
    ThumbTrack,
    /// Fired when thumb is released
    ThumbRelease,
    /// Fired when the scrollbar position has changed and tracking is complete
    Changed,
}

/// Event data for scroll events
#[derive(Debug)]
pub struct ScrollEvent {
    pub event: Event,
}

impl ScrollEvent {
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the widget ID that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Get the current position of the scrollbar
    pub fn get_position(&self) -> Option<i32> {
        if self.event.is_null() {
            return None;
        }
        let pos = unsafe { ffi::wxd_ScrollEvent_GetPosition(self.event.0) };
        if pos == -1 { None } else { Some(pos) }
    }

    /// Get the orientation of the scrollbar (horizontal or vertical)
    pub fn get_orientation(&self) -> Option<i32> {
        if self.event.is_null() {
            return None;
        }
        let orient = unsafe { ffi::wxd_ScrollEvent_GetOrientation(self.event.0) };
        if orient == -1 { None } else { Some(orient) }
    }
}

// Use the macro to implement the trait
crate::implement_category_event_handlers!(ScrollEvents, ScrollEventType, ScrollEvent,
    Top => scroll_top, EventType::SCROLL_TOP,
    Bottom => scroll_bottom, EventType::SCROLL_BOTTOM,
    LineUp => scroll_lineup, EventType::SCROLL_LINEUP,
    LineDown => scroll_linedown, EventType::SCROLL_LINEDOWN,
    PageUp => scroll_pageup, EventType::SCROLL_PAGEUP,
    PageDown => scroll_pagedown, EventType::SCROLL_PAGEDOWN,
    ThumbTrack => thumb_track, EventType::SCROLL_THUMBTRACK,
    ThumbRelease => thumb_release, EventType::SCROLL_THUMBRELEASE,
    Changed => scroll_changed, EventType::SCROLL_CHANGED
);
