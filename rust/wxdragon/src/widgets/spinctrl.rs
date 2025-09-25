//! Safe wrapper for wxSpinCtrl.

use crate::event::event_data::CommandEventData;
use crate::event::{Event, EventType, WindowEvents};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
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
#[derive(Clone)]
pub struct SpinCtrl {
    window: Window,
}

impl SpinCtrl {
    /// Creates a new SpinCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> SpinCtrlBuilder<'_> {
        SpinCtrlBuilder::new(parent)
    }

    // Internal constructor
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SpinCtrl_t) -> Self {
        SpinCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the raw underlying spin control pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_SpinCtrl_t {
        self.window.handle_ptr() as *mut ffi::wxd_SpinCtrl_t
    }

    // --- Methods specific to SpinCtrl ---

    /// Gets the current value.
    pub fn value(&self) -> i32 {
        unsafe { ffi::wxd_SpinCtrl_GetValue(self.as_ptr()) }
    }

    /// Sets the value.
    pub fn set_value(&self, value: i32) {
        unsafe { ffi::wxd_SpinCtrl_SetValue(self.as_ptr(), value as c_int) };
    }

    /// Sets the allowed range.
    pub fn set_range(&self, min_val: i32, max_val: i32) {
        unsafe { ffi::wxd_SpinCtrl_SetRange(self.as_ptr(), min_val as c_int, max_val as c_int) };
    }

    /// Gets the minimum allowed value.
    pub fn min(&self) -> i32 {
        unsafe { ffi::wxd_SpinCtrl_GetMin(self.as_ptr()) }
    }

    /// Gets the maximum allowed value.
    pub fn max(&self) -> i32 {
        unsafe { ffi::wxd_SpinCtrl_GetMax(self.as_ptr()) }
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

// Add WindowEvents implementation
impl WindowEvents for SpinCtrl {}

// Apply common trait implementations
implement_widget_traits_with_target!(SpinCtrl, window, Window);

// Use the widget_builder macro to generate the SpinCtrlBuilder implementation
widget_builder!(
    name: SpinCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: SpinCtrlStyle,
    fields: {
        min_value: i32 = 0,
        max_value: i32 = 100,
        initial_value: i32 = 0,
        value_str: String = "".to_string()
    },
    build_impl: |slf| {
        let initial_c_string =
            CString::new(slf.value_str.clone()).expect("CString::new failed for SpinCtrl initial value");

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
                slf.initial_value as c_int,
            )
        };

        if spin_ctrl_ptr.is_null() {
            panic!("Failed to create SpinCtrl");
        }

        let spin_ctrl = unsafe { SpinCtrl::from_ptr(spin_ctrl_ptr) };
        if slf.value_str.parse::<i32>().is_err() {
            let initial = slf.initial_value.clamp(slf.min_value, slf.max_value);
            spin_ctrl.set_value(initial);
        }
        spin_ctrl
    }
);

// Extension to SpinCtrlBuilder to add specialized methods
impl<'a> SpinCtrlBuilder<'a> {
    /// Sets the allowed range.
    pub fn with_range(mut self, min_val: i32, max_val: i32) -> Self {
        self.min_value = min_val;
        self.max_value = max_val;
        // Adjust initial value if it's outside the new range
        self.initial_value = self.initial_value.clamp(min_val, max_val);
        self.value_str = self.initial_value.to_string();
        self
    }
}

// Add XRC Support - enables SpinCtrl to be created from XRC-managed pointers
impl_xrc_support!(SpinCtrl, { window });

// Widget casting support for SpinCtrl
impl_widget_cast!(SpinCtrl, "wxSpinCtrl", { window });
