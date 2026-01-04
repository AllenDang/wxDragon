use std::ptr;

use wxdragon_sys as ffi;

use crate::datetime::DateTime;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::prelude::*;
use crate::window::{WindowHandle, WxWidget};
// Window is used by XRC support for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::default::Default;

// --- Style enum using macro ---
widget_style_enum!(
    name: TimePickerCtrlStyle,
    doc: "Style flags for TimePickerCtrl widgets.",
    variants: {
        Default: 0, "Default style."
    },
    default_variant: Default
);

/// Events emitted by TimePickerCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimePickerEvent {
    /// Emitted when the time value is changed
    TimeChanged,
}

/// Event data for a TimePickerCtrl event
#[derive(Debug)]
pub struct TimePickerEventData {
    event: Event,
}

impl TimePickerEventData {
    /// Create a new TimePickerEventData from a generic Event
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
}

// --- wxTimePickerCtrl ---
/// Represents a wxTimePickerCtrl.
///
/// TimePickerCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct TimePickerCtrl {
    /// Safe handle to the underlying wxTimePickerCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl TimePickerCtrl {
    /// Creates a new TimePickerCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> TimePickerCtrlBuilder<'_> {
        TimePickerCtrlBuilder::new(parent)
    }

    /// Helper to get raw time picker pointer, returns null if widget has been destroyed
    #[inline]
    fn time_picker_ptr(&self) -> *mut ffi::wxd_TimePickerCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_TimePickerCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Gets the currently selected time.
    /// Returns an invalid DateTime if the widget has been destroyed.
    pub fn get_value(&self) -> DateTime {
        let ptr = self.time_picker_ptr();
        if ptr.is_null() {
            return DateTime::default();
        }
        let ffi_dt = unsafe { ffi::wxd_TimePickerCtrl_GetValue(ptr) };
        DateTime::from(ffi_dt)
    }

    /// Sets the currently selected time.
    /// No-op if the widget has been destroyed.
    pub fn set_value(&self, dt: &DateTime) {
        let ptr = self.time_picker_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_TimePickerCtrl_SetValue(ptr, dt.as_const_ptr()) };
    }

    /// Returns the underlying WindowHandle for this time picker.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Use the widget_builder macro to generate the TimePickerCtrlBuilder implementation
widget_builder!(
    name: TimePickerCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: TimePickerCtrlStyle,
    fields: {
        value: Option<DateTime> = None
    },
    build_impl: |slf| {
        assert!(!slf.parent.handle_ptr().is_null(), "TimePickerCtrl requires a parent");

        let ffi_dt_ptr = slf.value.as_ref().map_or(ptr::null(), |dt_val| dt_val.as_const_ptr());

        let ptr = unsafe {
            ffi::wxd_TimePickerCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                ffi_dt_ptr,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits(),
            )
        };
        if ptr.is_null() {
            panic!("Failed to create TimePickerCtrl: FFI returned null pointer.");
        }

        // Create a WindowHandle which automatically registers for destroy events
        TimePickerCtrl {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// Manual WxWidget implementation for TimePickerCtrl (using WindowHandle)
impl WxWidget for TimePickerCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for TimePickerCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for TimePickerCtrl {}

// Use the implement_widget_local_event_handlers macro to implement event handling
crate::implement_widget_local_event_handlers!(
    TimePickerCtrl,
    TimePickerEvent,
    TimePickerEventData,
    TimeChanged => time_changed, EventType::TIME_CHANGED
);

// XRC Support - enables TimePickerCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for TimePickerCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        TimePickerCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for TimePickerCtrl
impl crate::window::FromWindowWithClassName for TimePickerCtrl {
    fn class_name() -> &'static str {
        "wxTimePickerCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        TimePickerCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
