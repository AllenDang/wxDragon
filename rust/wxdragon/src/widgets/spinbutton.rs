//! Safe wrapper for wxSpinButton.

use crate::event::{Event, EventType};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// --- Style enum using macro ---
widget_style_enum!(
    name: SpinButtonStyle,
    doc: "Style flags for SpinButton",
    variants: {
        Default: ffi::WXD_SP_VERTICAL, "Default style (vertical spin button).",
        Horizontal: ffi::WXD_SP_HORIZONTAL, "Horizontal spin button.",
        ArrowKeys: ffi::WXD_SP_ARROW_KEYS, "Allow using arrow keys to change the value.",
        Wrap: ffi::WXD_SP_WRAP, "The value wraps around when incrementing/decrementing past max/min."
    },
    default_variant: Default
);

/// Events emitted by SpinButton
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpinButtonEvent {
    /// Emitted when the up arrow is clicked
    SpinUp,
    /// Emitted when the down arrow is clicked
    SpinDown,
    /// Emitted when the value changes via either arrow
    Spin,
}

/// Event data for a SpinButton event
#[derive(Debug)]
pub struct SpinButtonEventData {
    event: Event,
}

impl SpinButtonEventData {
    /// Create a new SpinButtonEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the ID of the control that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Skip this event (allow it to be processed by the parent window)
    pub fn skip(&self, skip: bool) {
        self.event.skip(skip);
    }

    /// Get the integer value associated with this event
    pub fn get_int(&self) -> Option<i32> {
        self.event.get_int()
    }
}

/// Represents a wxSpinButton widget.
#[derive(Clone)]
pub struct SpinButton {
    window: Window,
}

impl SpinButton {
    /// Creates a new SpinButton builder.
    pub fn builder(parent: &dyn WxWidget) -> SpinButtonBuilder<'_> {
        SpinButtonBuilder::new(parent)
    }

    // Internal constructor
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SpinButton_t) -> Self {
        SpinButton {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the raw underlying spin button pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_SpinButton_t {
        self.window.handle_ptr() as *mut ffi::wxd_SpinButton_t
    }

    // --- Methods specific to SpinButton ---

    /// Gets the current value.
    pub fn value(&self) -> i32 {
        unsafe { ffi::wxd_SpinButton_GetValue(self.as_ptr()) }
    }

    /// Sets the value.
    pub fn set_value(&self, value: i32) {
        unsafe { ffi::wxd_SpinButton_SetValue(self.as_ptr(), value as c_int) };
    }

    /// Sets the allowed range.
    pub fn set_range(&self, min_value: i32, max_value: i32) {
        unsafe {
            ffi::wxd_SpinButton_SetRange(self.as_ptr(), min_value as c_int, max_value as c_int)
        };
    }

    /// Gets the minimum allowed value.
    pub fn min(&self) -> i32 {
        unsafe { ffi::wxd_SpinButton_GetMin(self.as_ptr()) }
    }

    /// Gets the maximum allowed value.
    pub fn max(&self) -> i32 {
        unsafe { ffi::wxd_SpinButton_GetMax(self.as_ptr()) }
    }
}

// Apply common trait implementations
implement_widget_traits_with_target!(SpinButton, window, Window);

// Extension to SpinButtonBuilder to add range and initial value handling
impl<'a> SpinButtonBuilder<'a> {
    /// Sets the allowed range.
    pub fn with_range(mut self, min_value: i32, max_value: i32) -> Self {
        self.min_value = min_value;
        self.max_value = max_value;
        self
    }
}

// Use the widget_builder macro to generate the SpinButtonBuilder implementation
widget_builder!(
    name: SpinButton,
    parent_type: &'a dyn WxWidget,
    style_type: SpinButtonStyle,
    fields: {
        min_value: i32 = 0,
        max_value: i32 = 100,
        initial_value: i32 = 0
    },
    build_impl: |slf| {
        let spin_button_ptr = unsafe {
            ffi::wxd_SpinButton_Create(
                slf.parent.handle_ptr(),
                slf.id,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            )
        };

        if spin_button_ptr.is_null() {
            panic!("Failed to create SpinButton");
        }

        let spin_button = unsafe { SpinButton::from_ptr(spin_button_ptr) };


        spin_button.set_range(slf.min_value, slf.max_value);

        // Clamp initial value to range
        spin_button.set_value(slf.initial_value.clamp(slf.min_value, slf.max_value));

        spin_button
    }
);

// Use the implement_widget_local_event_handlers macro to implement event handling
crate::implement_widget_local_event_handlers!(
    SpinButton,
    SpinButtonEvent,
    SpinButtonEventData,
    SpinUp => spin_up, EventType::SPIN_UP,
    SpinDown => spin_down, EventType::SPIN_DOWN,
    Spin => spin, EventType::SPIN
);

// Add XRC Support - enables SpinButton to be created from XRC-managed pointers
impl_xrc_support!(SpinButton, { window });

// Widget casting support for SpinButton
impl_widget_cast!(SpinButton, "wxSpinButton", { window });
