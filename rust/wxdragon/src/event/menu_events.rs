//! Menu-specific events and event data.

use crate::event::{Event, EventType};
use crate::geometry::Point;
use wxdragon_sys as ffi;

/// Menu event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuEvent {
    /// Menu item selected (wxEVT_MENU)
    Selected,
    /// Menu opened (wxEVT_MENU_OPEN)
    Opened,
    /// Menu closed (wxEVT_MENU_CLOSE)
    Closed,
    /// Menu item highlighted (wxEVT_MENU_HIGHLIGHT)
    Highlighted,
    /// Context menu requested (wxEVT_CONTEXT_MENU)
    ContextRequested,
}

/// Event data for menu events
#[derive(Debug)]
pub struct MenuEventData {
    event: Event,
}

impl MenuEventData {
    /// Creates a new MenuEventData from a raw event
    pub(crate) fn new(event: Event) -> Self {
        Self { event }
    }

    /// Gets the underlying event
    pub fn event(&self) -> &Event {
        &self.event
    }

    /// Gets the menu ID (for wxEVT_MENU_OPEN, wxEVT_MENU_CLOSE, wxEVT_MENU_HIGHLIGHT)
    pub fn get_menu_id(&self) -> Option<i32> {
        if self.event.is_null() {
            return None;
        }
        let menu_id = unsafe { ffi::wxd_MenuEvent_GetMenuId(self.event._as_ptr()) };
        if menu_id == -1 {
            None
        } else {
            Some(menu_id)
        }
    }

    /// Checks if this is a popup menu event
    pub fn is_popup(&self) -> bool {
        if self.event.is_null() {
            return false;
        }
        unsafe { ffi::wxd_MenuEvent_IsPopup(self.event._as_ptr()) }
    }

    /// Gets the context menu position (for wxEVT_CONTEXT_MENU)
    pub fn get_context_position(&self) -> Option<Point> {
        if self.event.is_null() {
            return None;
        }
        let c_point = unsafe { ffi::wxd_ContextMenuEvent_GetPosition(self.event._as_ptr()) };
        if c_point.x == -1 && c_point.y == -1 {
            None
        } else {
            Some(Point {
                x: c_point.x,
                y: c_point.y,
            })
        }
    }

    /// Gets the event ID (widget/menu item ID)
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Skips the event to allow further processing
    pub fn skip(&self, skip: bool) {
        self.event.skip(skip);
    }

    /// Checks if the event can be vetoed
    pub fn can_veto(&self) -> bool {
        self.event.can_veto()
    }

    /// Vetos the event if possible
    pub fn veto(&self) {
        self.event.veto();
    }

    /// Checks if the event has been vetoed
    pub fn is_vetoed(&self) -> bool {
        self.event.is_vetoed()
    }
}

/// Trait for widgets that can handle menu events
pub trait MenuEvents: crate::event::WxEvtHandler {
    /// Binds a handler for menu selection events (wxEVT_MENU)
    fn on_menu_selected<F>(&self, callback: F)
    where
        F: FnMut(MenuEventData) + 'static,
    {
        self.bind_menu_event(MenuEvent::Selected, callback);
    }

    /// Binds a handler for menu open events (wxEVT_MENU_OPEN)
    fn on_menu_opened<F>(&self, callback: F)
    where
        F: FnMut(MenuEventData) + 'static,
    {
        self.bind_menu_event(MenuEvent::Opened, callback);
    }

    /// Binds a handler for menu close events (wxEVT_MENU_CLOSE)
    fn on_menu_closed<F>(&self, callback: F)
    where
        F: FnMut(MenuEventData) + 'static,
    {
        self.bind_menu_event(MenuEvent::Closed, callback);
    }

    /// Binds a handler for menu highlight events (wxEVT_MENU_HIGHLIGHT)
    fn on_menu_highlighted<F>(&self, callback: F)
    where
        F: FnMut(MenuEventData) + 'static,
    {
        self.bind_menu_event(MenuEvent::Highlighted, callback);
    }

    /// Binds a handler for context menu events (wxEVT_CONTEXT_MENU)
    fn on_context_menu<F>(&self, callback: F)
    where
        F: FnMut(MenuEventData) + 'static,
    {
        self.bind_menu_event(MenuEvent::ContextRequested, callback);
    }

    /// Internal binding method
    #[doc(hidden)]
    fn bind_menu_event<F>(&self, event: MenuEvent, mut callback: F)
    where
        F: FnMut(MenuEventData) + 'static,
    {
        let event_type = match event {
            MenuEvent::Selected => EventType::MENU,
            MenuEvent::Opened => EventType::MENU_OPEN,
            MenuEvent::Closed => EventType::MENU_CLOSE,
            MenuEvent::Highlighted => EventType::MENU_HIGHLIGHT,
            MenuEvent::ContextRequested => EventType::CONTEXT_MENU,
        };

        let wrapper = move |event: Event| {
            let menu_event_data = MenuEventData::new(event);
            callback(menu_event_data);
        };

        crate::event::WxEvtHandler::bind_internal(self, event_type, wrapper);
    }
}

/// Convenience methods for menu event handling
impl MenuEventData {
    /// Gets a string description of the menu event type
    pub fn get_event_type_name(&self) -> &'static str {
        if let Some(event_type) = self.event.get_event_type() {
            match event_type {
                EventType::MENU => "Menu Selected",
                EventType::MENU_OPEN => "Menu Opened",
                EventType::MENU_CLOSE => "Menu Closed",
                EventType::MENU_HIGHLIGHT => "Menu Highlighted",
                EventType::CONTEXT_MENU => "Context Menu Requested",
                _ => "Unknown Menu Event",
            }
        } else {
            "Invalid Event"
        }
    }

    /// Formats the menu event data for logging
    pub fn format_for_logging(&self) -> String {
        let event_name = self.get_event_type_name();
        let id = self.get_id();

        match self.event.get_event_type() {
            Some(EventType::CONTEXT_MENU) => {
                if let Some(pos) = self.get_context_position() {
                    format!(
                        "{} - ID: {}, Position: ({}, {})",
                        event_name, id, pos.x, pos.y
                    )
                } else {
                    format!("{} - ID: {}", event_name, id)
                }
            }
            Some(EventType::MENU_OPEN | EventType::MENU_CLOSE | EventType::MENU_HIGHLIGHT) => {
                let popup_info = if self.is_popup() {
                    " (Popup)"
                } else {
                    " (Menu Bar)"
                };
                if let Some(menu_id) = self.get_menu_id() {
                    format!(
                        "{} - ID: {}, Menu ID: {}{}",
                        event_name, id, menu_id, popup_info
                    )
                } else {
                    format!("{} - ID: {}{}", event_name, id, popup_info)
                }
            }
            _ => {
                format!("{} - ID: {}", event_name, id)
            }
        }
    }
}
