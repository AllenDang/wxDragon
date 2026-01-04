use std::ptr;

use wxdragon_sys as ffi;

use crate::datetime::DateTime;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::prelude::*;
use crate::window::{WindowHandle, WxWidget};
use std::default::Default;

// --- Style enum using macro ---
widget_style_enum!(
    name: DatePickerCtrlStyle,
    doc: "Style flags for DatePickerCtrl widgets.",
    variants: {
        Default: ffi::WXD_DP_DEFAULT, "Default style.",
        Spin: ffi::WXD_DP_SPIN, "Uses a spin control to change the date.",
        Dropdown: ffi::WXD_DP_DROPDOWN, "Uses a dropdown control to select the date.",
        AllowNone: ffi::WXD_DP_ALLOWNONE, "Allow the user to select 'None' (no date).",
        ShowCentury: ffi::WXD_DP_SHOWCENTURY, "Shows the century in the default date format."
    },
    default_variant: Default
);

/// Events emitted by DatePickerCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatePickerCtrlEvent {
    /// Emitted when the date is changed
    DateChanged,
}

/// Event data for DatePickerCtrl events
#[derive(Debug)]
pub struct DatePickerCtrlEventData {
    event: Event,
}

impl DatePickerCtrlEventData {
    /// Create a new DatePickerCtrlEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the selected date
    pub fn get_date(&self) -> Option<DateTime> {
        if self.event.is_null() {
            return None;
        }
        let date_ptr = unsafe { ffi::wxd_CalendarEvent_GetDate(self.event.0) };
        if date_ptr.is_null() {
            return None;
        }
        Some(DateTime::from(date_ptr))
    }
}

// --- wxDatePickerCtrl ---
/// Represents a wxDatePickerCtrl.
///
/// DatePickerCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let date_picker = DatePickerCtrl::builder(&frame).build();
///
/// // DatePickerCtrl is Copy - no clone needed for closures!
/// date_picker.bind_date_changed(move |_| {
///     // Safe: if date_picker was destroyed, this is a no-op
///     let date = date_picker.get_value();
/// });
///
/// // After parent destruction, date_picker operations are safe no-ops
/// frame.destroy();
/// assert!(!date_picker.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct DatePickerCtrl {
    /// Safe handle to the underlying wxDatePickerCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl DatePickerCtrl {
    /// Creates a new DatePickerCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> DatePickerCtrlBuilder<'_> {
        DatePickerCtrlBuilder::new(parent)
    }

    /// Helper to get raw date picker pointer, returns null if widget has been destroyed
    #[inline]
    fn date_picker_ptr(&self) -> *mut ffi::wxd_DatePickerCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_DatePickerCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Gets the currently selected date.
    /// Returns an invalid DateTime if the widget has been destroyed.
    pub fn get_value(&self) -> DateTime {
        let ptr = self.date_picker_ptr();
        if ptr.is_null() {
            return DateTime::default();
        }
        let ffi_dt = unsafe { ffi::wxd_DatePickerCtrl_GetValue(ptr) };
        DateTime::from(ffi_dt)
    }

    /// Sets the currently selected date.
    /// No-op if the widget has been destroyed.
    pub fn set_value(&self, dt: &DateTime) {
        let ptr = self.date_picker_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_DatePickerCtrl_SetValue(ptr, dt.as_const_ptr()) };
    }

    /// Gets the valid range for dates on the control.
    /// Returns `Ok((Option<DateTime>, Option<DateTime>))` if successful.
    /// The DateTimes in the tuple will be None if the corresponding bound is not set or if the bounds are invalid.
    /// Returns an error if the widget has been destroyed.
    pub fn get_range(&self) -> Result<(Option<DateTime>, Option<DateTime>), String> {
        let ptr = self.date_picker_ptr();
        if ptr.is_null() {
            return Err("DatePickerCtrl has been destroyed".to_string());
        }

        let mut p1: *mut ffi::wxd_DateTime_t = std::ptr::null_mut();
        let mut p2: *mut ffi::wxd_DateTime_t = std::ptr::null_mut();

        let _has_range = unsafe { ffi::wxd_DatePickerCtrl_GetRange(ptr, &mut p1, &mut p2) };

        let opt_dt1 = if p1.is_null() { None } else { Some(DateTime::from(p1)) };
        let opt_dt2 = if p2.is_null() { None } else { Some(DateTime::from(p2)) };

        Ok((opt_dt1, opt_dt2))
    }

    /// Sets the valid range for dates on the control.
    /// Pass `None` for `dt_start` or `dt_end` to remove the lower or upper bound, respectively.
    /// No-op if the widget has been destroyed.
    pub fn set_range(&self, dt_start: Option<&DateTime>, dt_end: Option<&DateTime>) {
        let ptr = self.date_picker_ptr();
        if ptr.is_null() {
            return;
        }

        let ptr_dt1 = dt_start.map_or(ptr::null(), |dt| dt.as_const_ptr());
        let ptr_dt2 = dt_end.map_or(ptr::null(), |dt| dt.as_const_ptr());

        unsafe { ffi::wxd_DatePickerCtrl_SetRange(ptr, ptr_dt1, ptr_dt2) };
    }

    /// Creates a DatePickerCtrl from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_DatePickerCtrl_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the underlying WindowHandle for this date picker.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for DatePickerCtrl (using WindowHandle)
impl WxWidget for DatePickerCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for DatePickerCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for DatePickerCtrl {}

// Implement event handlers for DatePickerCtrl
crate::implement_widget_local_event_handlers!(
    DatePickerCtrl,
    DatePickerCtrlEvent,
    DatePickerCtrlEventData,
    DateChanged => date_changed, EventType::DATE_CHANGED
);

// Use the widget_builder macro to generate the DatePickerCtrlBuilder implementation
widget_builder!(
    name: DatePickerCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: DatePickerCtrlStyle,
    fields: {
        value: Option<DateTime> = None
    },
    build_impl: |slf| {
        assert!(!slf.parent.handle_ptr().is_null(), "DatePickerCtrl requires a parent");

        let ffi_dt_ptr = slf.value.as_ref().map_or(ptr::null(), |dt_val| dt_val.as_const_ptr());

        let ptr = unsafe {
            ffi::wxd_DatePickerCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                ffi_dt_ptr,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits(),
            )
        };
        if ptr.is_null() {
            panic!("Failed to create DatePickerCtrl: FFI returned null pointer.");
        }

        // Create a WindowHandle which automatically registers for destroy events
        DatePickerCtrl {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// XRC Support - enables DatePickerCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for DatePickerCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        DatePickerCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for DatePickerCtrl
impl crate::window::FromWindowWithClassName for DatePickerCtrl {
    fn class_name() -> &'static str {
        "wxDatePickerCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        DatePickerCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
