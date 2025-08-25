//! Window-level events that are common to all widgets.

use crate::event::event_data::{KeyEventData, MouseEventData};
use crate::event::{Event, EventType};
use crate::geometry::Size;
use std::fmt::Debug;

/// Base window events that are common to all widgets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowEvent {
    // Mouse events
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
    MiddleDown,
    MiddleUp,
    Motion,
    MouseWheel,
    EnterWindow,
    LeaveWindow,

    // Keyboard events
    KeyDown,
    KeyUp,
    Char,

    // Window events
    Size,
    Move, // Now implemented in C++ layer
    Paint,
    Erase,     // Now implemented in C++ layer
    SetFocus,  // Now implemented in C++ layer
    KillFocus, // Now implemented in C++ layer

    // Misc events
    Idle,
    Close,
    Destroy,
}

/// Data for window events that can be converted to appropriate specific event type
#[derive(Debug)]
pub enum WindowEventData {
    MouseButton(MouseButtonEvent),
    MouseMotion(MouseMotionEvent),
    MouseEnter(MouseEnterEvent),
    MouseLeave(MouseLeaveEvent),
    Keyboard(KeyboardEvent),
    Size(WindowSizeEvent),
    Idle(IdleEventData),
    General(Event),
}

impl WindowEventData {
    /// Create a new WindowEventData from an Event, based on the event type
    pub fn new(event: Event) -> Self {
        // Replace the unsafe block with a direct call
        let win_event = event._as_ptr();
        if win_event.is_null() {
            return WindowEventData::General(event);
        }

        // Check for mouse button events
        if unsafe { wxdragon_sys::wxd_IsMouseButtonEvent(win_event) > 0 } {
            return WindowEventData::MouseButton(MouseButtonEvent::new(event));
        }

        // Check for mouse motion events
        if unsafe { wxdragon_sys::wxd_IsMouseMotionEvent(win_event) > 0 } {
            return WindowEventData::MouseMotion(MouseMotionEvent::new(event));
        }

        // Check for keyboard events
        if unsafe { wxdragon_sys::wxd_IsKeyboardEvent(win_event) > 0 } {
            return WindowEventData::Keyboard(KeyboardEvent::new(event));
        }

        // Check for size events and mouse enter/leave events
        if let Some(event_type) = event.get_event_type() {
            if event_type == EventType::SIZE {
                return WindowEventData::Size(WindowSizeEvent::new(event));
            } else if event_type == EventType::ENTER_WINDOW {
                return WindowEventData::MouseEnter(MouseEnterEvent::new(event));
            } else if event_type == EventType::LEAVE_WINDOW {
                return WindowEventData::MouseLeave(MouseLeaveEvent::new(event));
            } else if event_type == EventType::IDLE {
                return WindowEventData::Idle(IdleEventData::new(event));
            }
        }

        // Default to general event
        WindowEventData::General(event)
    }

    /// Skip this event, allowing it to propagate to parent handlers
    pub fn skip(&self, skip: bool) {
        match self {
            WindowEventData::MouseButton(event) => event.event.skip(skip),
            WindowEventData::MouseMotion(event) => event.event.skip(skip),
            WindowEventData::MouseEnter(event) => event.event.skip(skip),
            WindowEventData::MouseLeave(event) => event.event.skip(skip),
            WindowEventData::Keyboard(event) => event.event.skip(skip),
            WindowEventData::Size(event) => event.event.skip(skip),
            WindowEventData::Idle(event) => event.event.skip(skip),
            WindowEventData::General(event) => event.skip(skip),
        }
    }
}

/// Mouse button events (left/right/middle click, up/down)
#[derive(Debug)]
pub struct MouseButtonEvent {
    pub event: MouseEventData,
}

impl MouseButtonEvent {
    pub fn new(event: Event) -> Self {
        Self {
            event: MouseEventData::new(event),
        }
    }

    pub fn get_position(&self) -> Option<crate::geometry::Point> {
        self.event.get_position()
    }
}

/// Mouse motion events
#[derive(Debug)]
pub struct MouseMotionEvent {
    pub event: MouseEventData,
}

impl MouseMotionEvent {
    pub fn new(event: Event) -> Self {
        Self {
            event: MouseEventData::new(event),
        }
    }

    pub fn get_position(&self) -> Option<crate::geometry::Point> {
        self.event.get_position()
    }
}

/// Mouse enter events
#[derive(Debug)]
pub struct MouseEnterEvent {
    pub event: MouseEventData,
}

impl MouseEnterEvent {
    pub fn new(event: Event) -> Self {
        Self {
            event: MouseEventData::new(event),
        }
    }

    pub fn get_position(&self) -> Option<crate::geometry::Point> {
        self.event.get_position()
    }
}

/// Mouse leave events
#[derive(Debug)]
pub struct MouseLeaveEvent {
    pub event: MouseEventData,
}

impl MouseLeaveEvent {
    pub fn new(event: Event) -> Self {
        Self {
            event: MouseEventData::new(event),
        }
    }

    pub fn get_position(&self) -> Option<crate::geometry::Point> {
        self.event.get_position()
    }
}

/// Keyboard events
#[derive(Debug)]
pub struct KeyboardEvent {
    pub event: KeyEventData,
}

impl KeyboardEvent {
    pub fn new(event: Event) -> Self {
        Self {
            event: KeyEventData::new(event),
        }
    }

    pub fn get_key_code(&self) -> Option<i32> {
        self.event.get_key_code()
    }

    pub fn get_unicode_key(&self) -> Option<i32> {
        self.event.get_unicode_key()
    }

}

/// Window size events
#[derive(Debug)]
pub struct WindowSizeEvent {
    pub event: Event,
}

impl WindowSizeEvent {
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    pub fn get_size(&self) -> Option<Size> {
        // For now, we'll need to implement this in the Event struct
        // or re-implement the C API call here
        None
    }
}

/// Idle events
#[derive(Debug)]
pub struct IdleEventData {
    pub event: Event,
}

impl IdleEventData {
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Request more idle events to be sent.
    /// When `need_more` is true, the system will continue sending idle events.
    /// When false, idle events will stop until triggered by other activity.
    pub fn request_more(&self, need_more: bool) {
        self.event.request_more(need_more);
    }

    /// Returns true if more idle events have been requested.
    pub fn more_requested(&self) -> bool {
        self.event.more_requested()
    }
}

// Use the macro to implement the trait
crate::implement_category_event_handlers!(
    WindowEvents, WindowEvent, WindowEventData,
    LeftDown => mouse_left_down, EventType::LEFT_DOWN,
    LeftUp => mouse_left_up, EventType::LEFT_UP,
    RightDown => mouse_right_down, EventType::RIGHT_DOWN,
    RightUp => mouse_right_up, EventType::RIGHT_UP,
    MiddleDown => mouse_middle_down, EventType::MIDDLE_DOWN,
    MiddleUp => mouse_middle_up, EventType::MIDDLE_UP,
    Motion => mouse_motion, EventType::MOTION,
    MouseWheel => mouse_wheel, EventType::MOUSEWHEEL,
    EnterWindow => mouse_enter, EventType::ENTER_WINDOW,
    LeaveWindow => mouse_leave, EventType::LEAVE_WINDOW,
    KeyDown => key_down, EventType::KEY_DOWN,
    KeyUp => key_up, EventType::KEY_UP,
    Char => char, EventType::CHAR,
    Size => size, EventType::SIZE,
    Move => move_event, EventType::MOVE,
    Paint => paint, EventType::PAINT,
    Erase => erase_background, EventType::ERASE,
    SetFocus => set_focus, EventType::SET_FOCUS,
    KillFocus => kill_focus, EventType::KILL_FOCUS,
    Idle => idle, EventType::IDLE,
    Close => close, EventType::CLOSE_WINDOW,
    Destroy => destroy, EventType::DESTROY
);
