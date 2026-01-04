//! Safe wrapper for wxSpinButton.

use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
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
///
/// SpinButton uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let spin = SpinButton::builder(&frame).with_range(0, 100).build();
///
/// // SpinButton is Copy - no clone needed for closures!
/// spin.bind_spin(move |_| {
///     // Safe: if spin was destroyed, this is a no-op
///     let value = spin.value();
///     println!("Value: {}", value);
/// });
///
/// // After parent destruction, spin operations are safe no-ops
/// frame.destroy();
/// assert!(!spin.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct SpinButton {
    /// Safe handle to the underlying wxSpinButton - automatically invalidated on destroy
    handle: WindowHandle,
}

impl SpinButton {
    /// Creates a new SpinButton builder.
    pub fn builder(parent: &dyn WxWidget) -> SpinButtonBuilder<'_> {
        SpinButtonBuilder::new(parent)
    }

    /// Helper to get raw spin button pointer, returns null if widget has been destroyed
    #[inline]
    fn spinbutton_ptr(&self) -> *mut ffi::wxd_SpinButton_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_SpinButton_t)
            .unwrap_or(std::ptr::null_mut())
    }

    // --- Methods specific to SpinButton ---

    /// Gets the current value.
    /// Returns 0 if the widget has been destroyed.
    pub fn value(&self) -> i32 {
        let ptr = self.spinbutton_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_SpinButton_GetValue(ptr) }
    }

    /// Sets the value.
    /// No-op if the widget has been destroyed.
    pub fn set_value(&self, value: i32) {
        let ptr = self.spinbutton_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SpinButton_SetValue(ptr, value as c_int) };
    }

    /// Sets the allowed range.
    /// No-op if the widget has been destroyed.
    pub fn set_range(&self, min_value: i32, max_value: i32) {
        let ptr = self.spinbutton_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SpinButton_SetRange(ptr, min_value as c_int, max_value as c_int) };
    }

    /// Gets the minimum allowed value.
    /// Returns 0 if the widget has been destroyed.
    pub fn min(&self) -> i32 {
        let ptr = self.spinbutton_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_SpinButton_GetMin(ptr) }
    }

    /// Gets the maximum allowed value.
    /// Returns 0 if the widget has been destroyed.
    pub fn max(&self) -> i32 {
        let ptr = self.spinbutton_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_SpinButton_GetMax(ptr) }
    }

    /// Returns the underlying WindowHandle for this spin button.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for SpinButton (using WindowHandle)
impl WxWidget for SpinButton {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for SpinButton {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for SpinButton {}

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

        let spin_button = SpinButton {
            handle: WindowHandle::new(spin_button_ptr as *mut ffi::wxd_Window_t),
        };

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

// XRC Support - enables SpinButton to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for SpinButton {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        SpinButton {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for SpinButton
impl crate::window::FromWindowWithClassName for SpinButton {
    fn class_name() -> &'static str {
        "wxSpinButton"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        SpinButton {
            handle: WindowHandle::new(ptr),
        }
    }
}
