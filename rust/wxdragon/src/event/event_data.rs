use crate::event::Event;
use crate::geometry::Point;

/// Common data for command events (button clicks, menu selections, etc.)
#[derive(Debug)]
pub struct CommandEventData {
    pub event: Event,
}

impl CommandEventData {
    pub fn new(event: Event) -> Self {
        CommandEventData { event }
    }

    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    pub fn get_string(&self) -> Option<String> {
        self.event.get_string()
    }

    pub fn is_checked(&self) -> Option<bool> {
        self.event.is_checked()
    }

    pub fn get_int(&self) -> Option<i32> {
        self.event.get_int()
    }

    pub fn skip(&self, skip: bool) {
        self.event.skip(skip);
    }
}

/// Data for mouse events
#[derive(Debug)]
pub struct MouseEventData {
    pub event: Event,
}

impl MouseEventData {
    pub fn new(event: Event) -> Self {
        MouseEventData { event }
    }

    pub fn get_position(&self) -> Option<Point> {
        self.event.get_position()
    }

    /// Gets the wheel rotation value for mouse wheel events.
    /// Returns the wheel rotation amount in multiples of wheel delta.
    /// Positive values indicate forward/up scrolling, negative values indicate backward/down scrolling.
    pub fn get_wheel_rotation(&self) -> i32 {
        self.event.get_wheel_rotation()
    }

    /// Gets the wheel delta value for mouse wheel events.
    /// This is the basic unit of wheel rotation, typically 120 on most systems.
    /// The actual rotation can be calculated as get_wheel_rotation() / get_wheel_delta().
    pub fn get_wheel_delta(&self) -> i32 {
        self.event.get_wheel_delta()
    }

    pub fn skip(&self, skip: bool) {
        self.event.skip(skip);
    }
}

/// Data for keyboard events
#[derive(Debug)]
pub struct KeyEventData {
    pub event: Event,
}

impl KeyEventData {
    pub fn new(event: Event) -> Self {
        KeyEventData { event }
    }

    pub fn get_key_code(&self) -> Option<i32> {
        self.event.get_key_code()
    }

    pub fn get_unicode_key(&self) -> Option<i32> {
        self.event.get_unicode_key()
    }

    /// Check if the Control key is pressed during this key event
    pub fn control_down(&self) -> bool {
        self.event.control_down()
    }

    /// Check if the Shift key is pressed during this key event
    pub fn shift_down(&self) -> bool {
        self.event.shift_down()
    }

    /// Check if the Alt key is pressed during this key event
    pub fn alt_down(&self) -> bool {
        self.event.alt_down()
    }

    /// Check if the Meta key is pressed during this key event (Cmd on macOS, Windows key on Windows)
    pub fn meta_down(&self) -> bool {
        self.event.meta_down()
    }

    /// Check if the platform-specific command key is pressed (Cmd on macOS, Ctrl on Windows/Linux)
    pub fn cmd_down(&self) -> bool {
        self.event.cmd_down()
    }

    pub fn skip(&self, skip: bool) {
        self.event.skip(skip);
    }
}
