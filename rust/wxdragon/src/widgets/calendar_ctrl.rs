use crate::datetime::DateTime;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ptr;
use wxdragon_sys as ffi;

// Define a proper style enum for CalendarCtrl
widget_style_enum!(
    name: CalendarCtrlStyle,
    doc: "Style flags for Calendar control.",
    variants: {
        Default: 0, "Default style.",
        SundayFirst: ffi::WXD_CAL_SUNDAY_FIRST, "Show Sunday as the first day in the week.",
        MondayFirst: ffi::WXD_CAL_MONDAY_FIRST, "Show Monday as the first day in the week.",
        ShowHolidays: ffi::WXD_CAL_SHOW_HOLIDAYS, "Highlight holidays in the calendar.",
        NoYearChange: ffi::WXD_CAL_NO_YEAR_CHANGE, "Disable year changing.",
        NoMonthChange: ffi::WXD_CAL_NO_MONTH_CHANGE, "Disable month changing.",
        SequentialMonthSelection: ffi::WXD_CAL_SEQUENTIAL_MONTH_SELECTION, "Use alternative, more compact, style for the month and year selection controls.",
        ShowSurroundingWeeks: ffi::WXD_CAL_SHOW_SURROUNDING_WEEKS, "Show the neighbouring weeks in the previous and next months."
    },
    default_variant: Default
);

/// Represents a `wxCalendarCtrl`.
///
/// CalendarCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct CalendarCtrl {
    /// Safe handle to the underlying wxCalendarCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl CalendarCtrl {
    /// Creates a new `CalendarCtrlBuilder` for constructing a calendar control.
    pub fn builder(parent: &dyn WxWidget) -> CalendarCtrlBuilder<'_> {
        CalendarCtrlBuilder::new(parent)
    }

    /// Low-level constructor used by the builder.
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, date: Option<&DateTime>, pos: Point, size: Size, style: i64) -> Self {
        assert!(!parent_ptr.is_null(), "CalendarCtrl requires a parent");

        // Convert Option<&DateTime> to *const ffi::wxd_DateTime_t
        let c_date_ptr = date.map_or(ptr::null(), |d| d.as_const_ptr());

        let ptr = unsafe {
            ffi::wxd_CalendarCtrl_Create(parent_ptr, id, c_date_ptr, pos.into(), size.into(), style as ffi::wxd_Style_t)
        };

        if ptr.is_null() {
            panic!("Failed to create CalendarCtrl widget");
        }

        // Create a WindowHandle which automatically registers for destroy events
        CalendarCtrl {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw calendar pointer, returns null if widget has been destroyed
    #[inline]
    fn calendar_ptr(&self) -> *mut ffi::wxd_CalendarCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_CalendarCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Sets the currently displayed date.
    /// Returns false if the widget has been destroyed.
    pub fn set_date(&self, date: &DateTime) -> bool {
        let ptr = self.calendar_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_CalendarCtrl_SetDate(ptr, date.as_const_ptr()) }
    }

    /// Gets the currently displayed date.
    /// Returns None if the widget has been destroyed.
    pub fn get_date(&self) -> Option<DateTime> {
        let ptr = self.calendar_ptr();
        if ptr.is_null() {
            return None;
        }
        unsafe {
            let raw_dt_ptr = ffi::wxd_CalendarCtrl_GetDate(ptr);
            if raw_dt_ptr.is_null() {
                None
            } else {
                Some(DateTime::from(raw_dt_ptr))
            }
        }
    }

    /// Returns the underlying WindowHandle for this calendar control.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// --- CalendarCtrl Event Handling ---

/// Event types specific to `CalendarCtrl`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarEvent {
    /// A day has been selected in the calendar.
    /// Corresponds to `EventType::CALENDAR_SEL_CHANGED`.
    SelectionChanged,
    /// A day has been double-clicked.
    /// Corresponds to `EventType::CALENDAR_DOUBLECLICKED`.
    DoubleClicked,
    /// The month has changed.
    /// Corresponds to `EventType::CALENDAR_MONTH_CHANGED`.
    MonthChanged,
    /// The year has changed.
    /// Corresponds to `EventType::CALENDAR_YEAR_CHANGED`.
    YearChanged,
    // CALENDAR_WEEKDAY_CLICKED is also available if needed
}

/// Event data for `CalendarCtrl` events.
/// This struct provides access to the date associated with the event.
#[derive(Debug)]
pub struct CalendarEventData {
    event: Event, // Calendar events are command events
}

impl CalendarEventData {
    /// Creates new `CalendarEventData` from base `Event`.
    pub(crate) fn new(event: Event) -> Self {
        Self { event }
    }

    /// Returns the ID of the calendar control that generated the event.
    pub fn get_id(&self) -> i32 {
        unsafe {
            let event_ptr = self.event._as_ptr();
            if !event_ptr.is_null() {
                return ffi::wxd_Event_GetId(event_ptr);
            }
            0
        }
    }

    /// For `MonthChanged` and `YearChanged`, this is the first day of the new month/year.
    pub fn get_date(&self) -> Option<DateTime> {
        let event_ptr = self.event._as_ptr();
        if event_ptr.is_null() {
            return None;
        }
        let date_ptr = unsafe { ffi::wxd_CalendarEvent_GetDate(event_ptr) };
        if date_ptr.is_null() {
            return None;
        }
        Some(DateTime::from(date_ptr))
    }
}

// Manual WxWidget implementation for CalendarCtrl (using WindowHandle)
impl WxWidget for CalendarCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for CalendarCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for CalendarCtrl {}

// Use the implement_widget_local_event_handlers macro
crate::implement_widget_local_event_handlers!(
    CalendarCtrl, CalendarEvent, CalendarEventData,
    SelectionChanged => selection_changed, EventType::CALENDAR_SEL_CHANGED,
    DoubleClicked => double_clicked, EventType::CALENDAR_DOUBLECLICKED,
    MonthChanged => month_changed, EventType::CALENDAR_MONTH_CHANGED,
    YearChanged => year_changed, EventType::CALENDAR_YEAR_CHANGED
);

// XRC Support - enables CalendarCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for CalendarCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        CalendarCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for CalendarCtrl
impl crate::window::FromWindowWithClassName for CalendarCtrl {
    fn class_name() -> &'static str {
        "wxCalendarCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        CalendarCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Use the widget_builder macro for CalendarCtrl
widget_builder!(
    name: CalendarCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: CalendarCtrlStyle,
    fields: {
        initial_date: Option<DateTime> = None
    },
    build_impl: |slf| {
        CalendarCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.initial_date.as_ref(),
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
);
