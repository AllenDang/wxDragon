//! Safe wrapper for wxSpinCtrl.

use crate::event::event_data::CommandEventData;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by impl_xrc_support for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// Re-export constants from wxdragon-sys

// --- Style enum using macro ---
widget_style_enum!(
    name: SpinCtrlStyle,
    doc: "Style flags for SpinCtrl.",
    variants: {
        Default: ffi::WXD_SP_VERTICAL | ffi::WXD_SP_ARROW_KEYS, "Default style (vertical, arrow keys enabled).",
        Horizontal: ffi::WXD_SP_HORIZONTAL, "Horizontal spin control.",
        Vertical: ffi::WXD_SP_VERTICAL, "Vertical spin control.",
        ArrowKeys: ffi::WXD_SP_ARROW_KEYS, "Allow using arrow keys to change the value.",
        Wrap: ffi::WXD_SP_WRAP, "The value wraps around when incrementing/decrementing past max/min.",
        ProcessEnter: ffi::WXD_TE_PROCESS_ENTER, "Process the Enter key press event (generates a command event)."
    },
    default_variant: Default
);

/// Represents a wxSpinCtrl widget.
///
/// SpinCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let spinctrl = SpinCtrl::builder(&frame).with_range(0, 100).build();
///
/// // SpinCtrl is Copy - no clone needed for closures!
/// spinctrl.bind_value_changed(move |_| {
///     // Safe: if spinctrl was destroyed, this is a no-op
///     let value = spinctrl.value();
///     println!("Value: {}", value);
/// });
///
/// // After parent destruction, spinctrl operations are safe no-ops
/// frame.destroy();
/// assert!(!spinctrl.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct SpinCtrl {
    /// Safe handle to the underlying wxSpinCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl SpinCtrl {
    /// Creates a new SpinCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> SpinCtrlBuilder<'_> {
        SpinCtrlBuilder::new(parent)
    }

    /// Creates a new SpinCtrl from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Helper to get raw spinctrl pointer, returns null if widget has been destroyed
    #[inline]
    fn spinctrl_ptr(&self) -> *mut ffi::wxd_SpinCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_SpinCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    // --- Methods specific to SpinCtrl ---

    /// Gets the current value.
    /// Returns 0 if the spinctrl has been destroyed.
    pub fn value(&self) -> i32 {
        let ptr = self.spinctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_SpinCtrl_GetValue(ptr) }
    }

    /// Sets the value.
    /// No-op if the spinctrl has been destroyed.
    pub fn set_value(&self, value: i32) {
        let ptr = self.spinctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SpinCtrl_SetValue(ptr, value as c_int) };
    }

    /// Sets the allowed range.
    /// No-op if the spinctrl has been destroyed.
    pub fn set_range(&self, min_val: i32, max_val: i32) {
        let ptr = self.spinctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SpinCtrl_SetRange(ptr, min_val as c_int, max_val as c_int) };
    }

    /// Gets the minimum allowed value.
    /// Returns 0 if the spinctrl has been destroyed.
    pub fn min(&self) -> i32 {
        let ptr = self.spinctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_SpinCtrl_GetMin(ptr) }
    }

    /// Gets the maximum allowed value.
    /// Returns 0 if the spinctrl has been destroyed.
    pub fn max(&self) -> i32 {
        let ptr = self.spinctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_SpinCtrl_GetMax(ptr) }
    }

    /// Returns the underlying WindowHandle for this spinctrl.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

/// Events that can be emitted by a `SpinCtrl`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpinCtrlEvent {
    /// The SpinCtrl's value has changed.
    ValueChanged,
}

/// Event data for a `SpinCtrl::ValueChanged` event.
#[derive(Debug)]
pub struct SpinCtrlEventData {
    /// The base command event data.
    pub base: CommandEventData,
}

impl SpinCtrlEventData {
    /// Creates a new `SpinCtrlEventData`.
    pub fn new(event: Event) -> Self {
        Self {
            base: CommandEventData::new(event),
        }
    }

    /// Gets the current value of the SpinCtrl from the event.
    pub fn get_value(&self) -> i32 {
        // For wxSpinCtrl, the event's GetInt() method returns the current value.
        self.base.get_int().unwrap_or(0)
    }

    // get_position() is an alias for get_value() for SpinCtrl
    pub fn get_position(&self) -> i32 {
        self.get_value()
    }
}

// Use the implement_widget_local_event_handlers macro
crate::implement_widget_local_event_handlers!(
    SpinCtrl, SpinCtrlEvent, SpinCtrlEventData,
    ValueChanged => value_changed, EventType::SPINCTRL
);

// Manual WxWidget implementation for SpinCtrl (using WindowHandle)
impl WxWidget for SpinCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for SpinCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for SpinCtrl {}

// Use the widget_builder macro to generate the SpinCtrlBuilder implementation
widget_builder!(
    name: SpinCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: SpinCtrlStyle,
    fields: {
        min_value: i32 = 0,
        max_value: i32 = 100,
        initial_value: i32 = 0,
        value_str: String = String::new()
    },
    build_impl: |slf| {
        let initial_value = slf.initial_value.clamp(slf.min_value, slf.max_value);
        let value_str =  match slf.value_str.parse::<i32>() {
            Err(_) => initial_value.to_string(),
            Ok(v) => v.clamp(slf.min_value, slf.max_value).to_string(),
        };
        let initial_c_string = CString::new(value_str).expect("CString::new failed for SpinCtrl initial value");

        let spin_ctrl_ptr = unsafe {
            ffi::wxd_SpinCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                initial_c_string.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
                slf.min_value as c_int,
                slf.max_value as c_int,
                initial_value as c_int,
            )
        };

        if spin_ctrl_ptr.is_null() {
            panic!("Failed to create SpinCtrl");
        }

        // Create a WindowHandle which automatically registers for destroy events
        SpinCtrl {
            handle: WindowHandle::new(spin_ctrl_ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// Extension to SpinCtrlBuilder to add specialized methods
impl<'a> SpinCtrlBuilder<'a> {
    /// Sets the allowed range.
    pub fn with_range(mut self, min_val: i32, max_val: i32) -> Self {
        self.min_value = min_val;
        self.max_value = max_val;
        self
    }
}

// XRC Support - enables SpinCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for SpinCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        SpinCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for SpinCtrl
impl crate::window::FromWindowWithClassName for SpinCtrl {
    fn class_name() -> &'static str {
        "wxSpinCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        SpinCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
