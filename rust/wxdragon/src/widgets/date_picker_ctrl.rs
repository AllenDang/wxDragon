use std::ptr;

use wxdragon_sys as ffi;

use crate::datetime::DateTime;
use crate::event::{Event, EventType};
use crate::implement_widget_traits_with_target;
use crate::prelude::*;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
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
        Some(unsafe { DateTime::from_raw(*date_ptr) })
    }
}

// --- wxDatePickerCtrl ---
#[derive(Clone)]
pub struct DatePickerCtrl {
    window: Window, // Embed Window for common wxWindow functionality
}

impl DatePickerCtrl {
    /// Creates a new DatePickerCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> DatePickerCtrlBuilder<'_> {
        DatePickerCtrlBuilder::new(parent)
    }

    /// Gets the currently selected date.
    pub fn get_value(&self) -> DateTime {
        let ffi_dt = unsafe {
            ffi::wxd_DatePickerCtrl_GetValue(self.window.as_ptr() as *mut ffi::wxd_DatePickerCtrl_t)
        };
        DateTime::from_raw(ffi_dt)
    }

    /// Sets the currently selected date.
    pub fn set_value(&self, dt: &DateTime) {
        unsafe {
            ffi::wxd_DatePickerCtrl_SetValue(
                self.window.as_ptr() as *mut ffi::wxd_DatePickerCtrl_t,
                dt.as_ptr(),
            );
        }
    }

    /// Gets the valid range for dates on the control.
    /// Returns `Ok((Option<DateTime>, Option<DateTime>))` if successful.
    /// The DateTimes in the tuple will be None if the corresponding bound is not set or if the bounds are invalid.
    pub fn get_range(&self) -> Result<(Option<DateTime>, Option<DateTime>), String> {
        let mut ffi_dt1 = unsafe { ffi::wxd_DateTime_Default() };
        let mut ffi_dt2 = unsafe { ffi::wxd_DateTime_Default() };

        // GetRange C FFI returns true if a range is set, false otherwise.
        // It populates dt1 and dt2 regardless; they will be invalid if no range or only one side of range is set.
        let _has_range = unsafe {
            ffi::wxd_DatePickerCtrl_GetRange(
                self.window.as_ptr() as *mut ffi::wxd_DatePickerCtrl_t,
                &mut ffi_dt1,
                &mut ffi_dt2,
            )
        };

        let dt1 = DateTime::from_raw(ffi_dt1);
        let dt2 = DateTime::from_raw(ffi_dt2);

        let opt_dt1 = if dt1.is_valid() { Some(dt1) } else { None };
        let opt_dt2 = if dt2.is_valid() { Some(dt2) } else { None };

        Ok((opt_dt1, opt_dt2))
    }

    /// Sets the valid range for dates on the control.
    /// Pass `None` for `dt_start` or `dt_end` to remove the lower or upper bound, respectively.
    pub fn set_range(&self, dt_start: Option<&DateTime>, dt_end: Option<&DateTime>) {
        let ptr_dt1 = dt_start.map_or(ptr::null(), |dt| dt.as_ptr());
        let ptr_dt2 = dt_end.map_or(ptr::null(), |dt| dt.as_ptr());

        unsafe {
            ffi::wxd_DatePickerCtrl_SetRange(
                self.window.as_ptr() as *mut ffi::wxd_DatePickerCtrl_t,
                ptr_dt1,
                ptr_dt2,
            );
        }
    }

    /// Creates a DatePickerCtrl from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_DatePickerCtrl_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_DatePickerCtrl_t) -> Self {
        DatePickerCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

// Implement event handlers for DatePickerCtrl
crate::implement_widget_local_event_handlers!(
    DatePickerCtrl,
    DatePickerCtrlEvent,
    DatePickerCtrlEventData,
    DateChanged => date_changed, EventType::DATE_CHANGED
);

// Add XRC Support - enables DatePickerCtrl to be created from XRC-managed pointers
impl_xrc_support!(DatePickerCtrl, { window });

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

        let ffi_dt_ptr = slf.value.as_ref().map_or(ptr::null(), |dt_val| dt_val.as_ptr());

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
        } else {
            unsafe { DatePickerCtrl::from_ptr(ptr) }
        }
    }
);

// Use the implement_widget_traits_with_target macro to implement traits
implement_widget_traits_with_target!(DatePickerCtrl, window, Window);

// Widget casting support for DatePickerCtrl
impl_widget_cast!(DatePickerCtrl, "wxDatePickerCtrl", { window });
