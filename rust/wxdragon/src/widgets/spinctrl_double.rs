use crate::event::{Event, EventType};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
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

#[derive(Clone)]
pub struct SpinCtrlDouble {
    window: Window,
}

impl SpinCtrlDouble {
    pub fn builder(parent: &dyn WxWidget) -> SpinCtrlDoubleBuilder<'_> {
        SpinCtrlDoubleBuilder::new(parent)
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SpinCtrlDouble_t) -> Self {
        SpinCtrlDouble {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Get the raw underlying spin control double pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_SpinCtrlDouble_t {
        self.window.handle_ptr() as *mut ffi::wxd_SpinCtrlDouble_t
    }

    pub fn get_value(&self) -> f64 {
        unsafe { ffi::wxd_SpinCtrlDouble_GetValue(self.as_ptr()) }
    }

    pub fn set_value(&self, value: f64) {
        unsafe { ffi::wxd_SpinCtrlDouble_SetValue(self.as_ptr(), value) }
    }

    pub fn set_range(&self, min_val: f64, max_val: f64) {
        unsafe { ffi::wxd_SpinCtrlDouble_SetRange(self.as_ptr(), min_val, max_val) }
    }

    pub fn get_min(&self) -> f64 {
        unsafe { ffi::wxd_SpinCtrlDouble_GetMin(self.as_ptr()) }
    }

    pub fn get_max(&self) -> f64 {
        unsafe { ffi::wxd_SpinCtrlDouble_GetMax(self.as_ptr()) }
    }

    pub fn set_increment(&self, inc: f64) {
        unsafe { ffi::wxd_SpinCtrlDouble_SetIncrements(self.as_ptr(), inc) }
    }

    pub fn get_increment(&self) -> f64 {
        unsafe { ffi::wxd_SpinCtrlDouble_GetIncrement(self.as_ptr()) }
    }

    pub fn set_digits(&self, digits: u32) {
        unsafe { ffi::wxd_SpinCtrlDouble_SetDigits(self.as_ptr(), digits) }
    }

    pub fn get_digits(&self) -> u32 {
        unsafe { ffi::wxd_SpinCtrlDouble_GetDigits(self.as_ptr()) }
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

// Apply common trait implementations
implement_widget_traits_with_target!(SpinCtrlDouble, window, Window);

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

// Add XRC Support - enables SpinCtrlDouble to be created from XRC-managed pointers
impl_xrc_support!(SpinCtrlDouble, { window });

// Widget casting support for SpinCtrlDouble
impl_widget_cast!(SpinCtrlDouble, "wxSpinCtrlDouble", { window });
