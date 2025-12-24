use crate::event::{Event, EventType, TextEvents};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{Window, WxWidget};
use wxdragon_sys as ffi;

use std::ffi::CString;
use std::os::raw::{c_char, c_longlong};

// --- Style enum using macro ---
widget_style_enum!(
    name: SearchCtrlStyle,
    doc: "Style flags for SearchCtrl",
    variants: {
        Default: 0, "Default style.",
        ProcessEnter: ffi::WXD_TE_PROCESS_ENTER, "Process Enter key press."
    },
    default_variant: Default
);

/// Events emitted by SearchCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchCtrlEvent {
    /// Emitted when the search button is clicked
    SearchButtonClicked,
    /// Emitted when the cancel button is clicked
    CancelButtonClicked,
}

/// Event data for a SearchCtrl event
#[derive(Debug)]
pub struct SearchCtrlEventData {
    event: Event,
}

impl SearchCtrlEventData {
    /// Create a new SearchCtrlEventData from a generic Event
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

    /// Get the current text in the search control
    pub fn get_string(&self) -> Option<String> {
        self.event.get_string()
    }
}

// --- SearchCtrl --- //

#[derive(Clone, Copy)]
pub struct SearchCtrl {
    window: Window,
}

impl SearchCtrl {
    pub fn builder(parent: &dyn WxWidget) -> SearchCtrlBuilder<'_> {
        SearchCtrlBuilder::new(parent)
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SearchCtrl_t) -> Self {
        SearchCtrl {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
        }
    }

    /// Get the raw underlying search ctrl pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_SearchCtrl_t {
        self.window.handle_ptr() as *mut ffi::wxd_SearchCtrl_t
    }

    pub fn show_search_button(&self, show: bool) {
        unsafe { ffi::wxd_SearchCtrl_ShowSearchButton(self.as_ptr(), show) }
    }

    pub fn is_search_button_visible(&self) -> bool {
        unsafe { ffi::wxd_SearchCtrl_IsSearchButtonVisible(self.as_ptr()) }
    }

    pub fn show_cancel_button(&self, show: bool) {
        unsafe { ffi::wxd_SearchCtrl_ShowCancelButton(self.as_ptr(), show) }
    }

    pub fn is_cancel_button_visible(&self) -> bool {
        unsafe { ffi::wxd_SearchCtrl_IsCancelButtonVisible(self.as_ptr()) }
    }

    pub fn set_value(&self, value: &str) {
        let c_value = CString::new(value).expect("CString::new failed for value");
        unsafe { ffi::wxd_SearchCtrl_SetValue(self.as_ptr(), c_value.as_ptr()) }
    }

    pub fn get_value(&self) -> String {
        // First call: get required UTF-8 byte length (excluding null terminator)
        let len = unsafe { ffi::wxd_SearchCtrl_GetValue(self.as_ptr(), std::ptr::null_mut(), 0) };
        if len == 0 {
            return String::new();
        }
        // Allocate buffer with space for null terminator
        let mut vec_buffer: Vec<u8> = vec![0; len + 1];
        let p = vec_buffer.as_mut_ptr() as *mut c_char;
        unsafe { ffi::wxd_SearchCtrl_GetValue(self.as_ptr(), p, vec_buffer.len()) };
        vec_buffer.pop(); // remove null terminator
        String::from_utf8(vec_buffer).unwrap_or_default()
    }
}

// Apply common trait implementations
implement_widget_traits_with_target!(SearchCtrl, window, Window);

// Use the widget_builder macro to generate the SearchCtrlBuilder implementation
widget_builder!(
    name: SearchCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: SearchCtrlStyle,
    fields: {
        value: String = String::new()
    },
    build_impl: |slf| {
        let c_value = CString::new(slf.value.as_str()).expect("CString::new failed for value");
        let raw_ptr = unsafe {
            ffi::wxd_SearchCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                c_value.as_ptr(),
                slf.pos.x,
                slf.pos.y,
                slf.size.width,
                slf.size.height,
                slf.style.bits() as c_longlong,
            )
        };

        if raw_ptr.is_null() {
            panic!("Failed to create wxSearchCtrl");
        }

        unsafe { SearchCtrl::from_ptr(raw_ptr) }
    }
);

// Implement SearchCtrl-specific event handlers
crate::implement_widget_local_event_handlers!(
    SearchCtrl,
    SearchCtrlEvent,
    SearchCtrlEventData,
    SearchButtonClicked => search_button_clicked, EventType::COMMAND_SEARCHCTRL_SEARCH_BTN,
    CancelButtonClicked => cancel_button_clicked, EventType::COMMAND_SEARCHCTRL_CANCEL_BTN
);

// Implement standard WindowEvents and TextEvents traits
impl TextEvents for SearchCtrl {}

// Add XRC Support - enables SearchCtrl to be created from XRC-managed pointers
impl_xrc_support!(SearchCtrl, { window });

// Widget casting support for SearchCtrl
impl_widget_cast!(SearchCtrl, "wxSearchCtrl", { window });
