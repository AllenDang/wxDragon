use crate::event::{Event, EventType, TextEvents, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by new_from_composition for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use wxdragon_sys as ffi;

use std::ffi::CString;
use std::os::raw::{c_char, c_longlong};
use std::ptr::null_mut;

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

/// Represents a wxSearchCtrl widget.
///
/// SearchCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let search = SearchCtrl::builder(&frame).value("Search...").build();
///
/// // SearchCtrl is Copy - no clone needed for closures!
/// search.bind_search_button_clicked(move |_| {
///     // Safe: if search was destroyed, this is a no-op
///     let query = search.get_value();
///     println!("Searching for: {}", query);
/// });
///
/// // After parent destruction, search operations are safe no-ops
/// frame.destroy();
/// assert!(!search.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct SearchCtrl {
    /// Safe handle to the underlying wxSearchCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl SearchCtrl {
    /// Creates a new SearchCtrl builder.
    pub fn builder(parent: &dyn WxWidget) -> SearchCtrlBuilder<'_> {
        SearchCtrlBuilder::new(parent)
    }

    /// Creates a new SearchCtrl from a raw window pointer.
    /// This is for backwards compatibility with widgets that compose SearchCtrl.
    /// The parent_ptr parameter is ignored (kept for API compatibility).
    #[allow(dead_code)]
    pub(crate) fn new_from_composition(_window: Window, _parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        // Use the window's pointer to create a new WindowHandle
        Self {
            handle: WindowHandle::new(_window.as_ptr()),
        }
    }

    /// Helper to get raw searchctrl pointer, returns null if widget has been destroyed
    #[inline]
    fn searchctrl_ptr(&self) -> *mut ffi::wxd_SearchCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_SearchCtrl_t)
            .unwrap_or(null_mut())
    }

    /// Shows or hides the search button.
    /// No-op if the control has been destroyed.
    pub fn show_search_button(&self, show: bool) {
        let ptr = self.searchctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SearchCtrl_ShowSearchButton(ptr, show) }
    }

    /// Returns whether the search button is visible.
    /// Returns false if the control has been destroyed.
    pub fn is_search_button_visible(&self) -> bool {
        let ptr = self.searchctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_SearchCtrl_IsSearchButtonVisible(ptr) }
    }

    /// Shows or hides the cancel button.
    /// No-op if the control has been destroyed.
    pub fn show_cancel_button(&self, show: bool) {
        let ptr = self.searchctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SearchCtrl_ShowCancelButton(ptr, show) }
    }

    /// Returns whether the cancel button is visible.
    /// Returns false if the control has been destroyed.
    pub fn is_cancel_button_visible(&self) -> bool {
        let ptr = self.searchctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_SearchCtrl_IsCancelButtonVisible(ptr) }
    }

    /// Sets the text value of the control.
    /// No-op if the control has been destroyed.
    pub fn set_value(&self, value: &str) {
        let ptr = self.searchctrl_ptr();
        if ptr.is_null() {
            return;
        }
        let c_value = CString::new(value).expect("CString::new failed for value");
        unsafe { ffi::wxd_SearchCtrl_SetValue(ptr, c_value.as_ptr()) }
    }

    /// Gets the current text value of the control.
    /// Returns empty string if the control has been destroyed.
    pub fn get_value(&self) -> String {
        let ptr = self.searchctrl_ptr();
        if ptr.is_null() {
            return String::new();
        }
        // First call: get required UTF-8 byte length (excluding null terminator)
        let len = unsafe { ffi::wxd_SearchCtrl_GetValue(ptr, std::ptr::null_mut(), 0) };
        if len == 0 {
            return String::new();
        }
        // Allocate buffer with space for null terminator
        let mut vec_buffer: Vec<u8> = vec![0; len + 1];
        let p = vec_buffer.as_mut_ptr() as *mut c_char;
        unsafe { ffi::wxd_SearchCtrl_GetValue(ptr, p, vec_buffer.len()) };
        vec_buffer.pop(); // remove null terminator
        String::from_utf8(vec_buffer).unwrap_or_default()
    }

    /// Returns the underlying WindowHandle for this searchctrl.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Implement TextEvents trait for SearchCtrl
impl TextEvents for SearchCtrl {}

// Manual WxWidget implementation for SearchCtrl (using WindowHandle)
impl WxWidget for SearchCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for SearchCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for SearchCtrl {}

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

        SearchCtrl {
            handle: WindowHandle::new(raw_ptr as *mut ffi::wxd_Window_t),
        }
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

// XRC Support - enables SearchCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for SearchCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        SearchCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Widget casting support for SearchCtrl
impl crate::window::FromWindowWithClassName for SearchCtrl {
    fn class_name() -> &'static str {
        "wxSearchCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        SearchCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
