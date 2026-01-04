use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use wxdragon_sys as ffi;

use std::ffi::CString;
use std::os::raw::c_longlong;

// --- Style enum using macro ---
widget_style_enum!(
    name: SpinCtrlDoubleStyle,
    doc: "Style flags for SpinCtrlDouble.",
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

/// Events emitted by SpinCtrlDouble
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpinCtrlDoubleEvent {
    /// Emitted when the value is changed
    ValueChanged,
    /// Emitted when the user presses Enter
    Enter,
}

/// Event data for SpinCtrlDouble events
#[derive(Debug)]
pub struct SpinCtrlDoubleEventData {
    event: Event,
}

impl SpinCtrlDoubleEventData {
    /// Create a new SpinCtrlDoubleEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the ID of the control that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }
}

// --- SpinCtrlDouble --- //

/// Represents a wxSpinCtrlDouble.
///
/// SpinCtrlDouble uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct SpinCtrlDouble {
    /// Safe handle to the underlying wxSpinCtrlDouble - automatically invalidated on destroy
    handle: WindowHandle,
}

impl SpinCtrlDouble {
    pub fn builder(parent: &dyn WxWidget) -> SpinCtrlDoubleBuilder<'_> {
        SpinCtrlDoubleBuilder::new(parent)
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SpinCtrlDouble_t) -> Self {
        SpinCtrlDouble {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw spin control double pointer, returns null if widget has been destroyed
    #[inline]
    fn spin_ctrl_double_ptr(&self) -> *mut ffi::wxd_SpinCtrlDouble_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_SpinCtrlDouble_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Get the raw underlying spin control double pointer.
    /// Returns null if the widget has been destroyed.
    pub fn as_ptr(&self) -> *mut ffi::wxd_SpinCtrlDouble_t {
        self.spin_ctrl_double_ptr()
    }

    /// Gets the current value.
    /// Returns 0.0 if the widget has been destroyed.
    pub fn get_value(&self) -> f64 {
        let ptr = self.spin_ctrl_double_ptr();
        if ptr.is_null() {
            return 0.0;
        }
        unsafe { ffi::wxd_SpinCtrlDouble_GetValue(ptr) }
    }

    /// Sets the current value.
    /// No-op if the widget has been destroyed.
    pub fn set_value(&self, value: f64) {
        let ptr = self.spin_ctrl_double_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SpinCtrlDouble_SetValue(ptr, value) }
    }

    /// Sets the allowed range.
    /// No-op if the widget has been destroyed.
    pub fn set_range(&self, min_val: f64, max_val: f64) {
        let ptr = self.spin_ctrl_double_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SpinCtrlDouble_SetRange(ptr, min_val, max_val) }
    }

    /// Gets the minimum allowed value.
    /// Returns 0.0 if the widget has been destroyed.
    pub fn get_min(&self) -> f64 {
        let ptr = self.spin_ctrl_double_ptr();
        if ptr.is_null() {
            return 0.0;
        }
        unsafe { ffi::wxd_SpinCtrlDouble_GetMin(ptr) }
    }

    /// Gets the maximum allowed value.
    /// Returns 0.0 if the widget has been destroyed.
    pub fn get_max(&self) -> f64 {
        let ptr = self.spin_ctrl_double_ptr();
        if ptr.is_null() {
            return 0.0;
        }
        unsafe { ffi::wxd_SpinCtrlDouble_GetMax(ptr) }
    }

    /// Sets the increment value.
    /// No-op if the widget has been destroyed.
    pub fn set_increment(&self, inc: f64) {
        let ptr = self.spin_ctrl_double_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SpinCtrlDouble_SetIncrements(ptr, inc) }
    }

    /// Gets the increment value.
    /// Returns 0.0 if the widget has been destroyed.
    pub fn get_increment(&self) -> f64 {
        let ptr = self.spin_ctrl_double_ptr();
        if ptr.is_null() {
            return 0.0;
        }
        unsafe { ffi::wxd_SpinCtrlDouble_GetIncrement(ptr) }
    }

    /// Sets the number of digits after the decimal point.
    /// No-op if the widget has been destroyed.
    pub fn set_digits(&self, digits: u32) {
        let ptr = self.spin_ctrl_double_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SpinCtrlDouble_SetDigits(ptr, digits) }
    }

    /// Gets the number of digits after the decimal point.
    /// Returns 0 if the widget has been destroyed.
    pub fn get_digits(&self) -> u32 {
        let ptr = self.spin_ctrl_double_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_SpinCtrlDouble_GetDigits(ptr) }
    }

    /// Returns the underlying WindowHandle for this spin control.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Implement event handlers for SpinCtrlDouble
crate::implement_widget_local_event_handlers!(
    SpinCtrlDouble,
    SpinCtrlDoubleEvent,
    SpinCtrlDoubleEventData,
    ValueChanged => value_changed, EventType::SPINCTRLDOUBLE,
    Enter => enter, EventType::TEXT_ENTER
);

// Manual WxWidget implementation for SpinCtrlDouble (using WindowHandle)
impl WxWidget for SpinCtrlDouble {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for SpinCtrlDouble {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for SpinCtrlDouble {}

// Use the widget_builder macro to generate the SpinCtrlDoubleBuilder implementation
widget_builder!(
    name: SpinCtrlDouble,
    parent_type: &'a dyn WxWidget,
    style_type: SpinCtrlDoubleStyle,
    fields: {
        value_str: String = String::new(),
        min_value: f64 = 0.0,
        max_value: f64 = 100.0,
        initial_value: f64 = 0.0,
        increment: f64 = 1.0
    },
    build_impl: |slf| {
       let initial_value = slf.initial_value.clamp(slf.min_value, slf.max_value);
       let value_str =  match slf.value_str.parse::<f64>() {
            Err(_) => initial_value.to_string(),
            Ok(v) => v.clamp(slf.min_value, slf.max_value).to_string(),
        };
        let c_value_str = CString::new(value_str).expect("CString::new failed for value_str");
        let raw_ptr = unsafe {
            ffi::wxd_SpinCtrlDouble_Create(
                slf.parent.handle_ptr(),
                slf.id,
                c_value_str.as_ptr(),
                slf.pos.x,
                slf.pos.y,
                slf.size.width,
                slf.size.height,
                slf.style.bits() as c_longlong,
                slf.min_value,
                slf.max_value,
                initial_value,
                slf.increment,
            )
        };
        if raw_ptr.is_null() {
            panic!("Failed to create wxSpinCtrlDouble");
        }
        unsafe { SpinCtrlDouble::from_ptr(raw_ptr) }
    }
);

// Extension to SpinCtrlBuilder to add specialized methods
impl<'a> SpinCtrlDoubleBuilder<'a> {
    /// Sets the allowed range.
    pub fn with_range(mut self, min_value: f64, max_value: f64) -> Self {
        self.min_value = min_value;
        self.max_value = max_value;
        self
    }
}

// XRC Support - enables SpinCtrlDouble to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for SpinCtrlDouble {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        SpinCtrlDouble {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for SpinCtrlDouble
impl crate::window::FromWindowWithClassName for SpinCtrlDouble {
    fn class_name() -> &'static str {
        "wxSpinCtrlDouble"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        SpinCtrlDouble {
            handle: WindowHandle::new(ptr),
        }
    }
}
