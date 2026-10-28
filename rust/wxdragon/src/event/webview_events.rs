//! Event system for WebView controls.

use crate::event::event_data::CommandEventData;
use crate::event::{Event, EventType};
use std::ffi::CStr;
use wxdragon_sys as ffi;

/// Events specific to WebView controls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebViewEvent {
    /// Fired when the WebView is created
    Created,
    /// Fired before navigating to a new URL (can be vetoed)
    Navigating,
    /// Fired after navigation has started
    Navigated,
    /// Fired when the page has fully loaded
    Loaded,
    /// Fired when a navigation error occurs
    Error,
    /// Fired when a new window is requested
    NewWindow,
    /// Fired with new window features
    NewWindowFeatures,
    /// Fired when the page title changes
    TitleChanged,
    /// Fired when fullscreen state changes
    FullscreenChanged,
    /// Fired when a script message is received
    ScriptMessageReceived,
    /// Fired with script execution result
    ScriptResult,
    /// Fired when window close is requested
    WindowCloseRequested,
    /// Fired when browsing data has been cleared
    BrowsingDataCleared,
}

/// Event data for WebView events
#[derive(Debug)]
pub struct WebViewEventData {
    pub event: CommandEventData,
}

impl WebViewEventData {
    pub fn new(event: Event) -> Self {
        Self {
            event: CommandEventData::new(event),
        }
    }

    /// Get the widget ID that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Get the event's string: the message of a script message event, the title of a
    /// title change. A navigation event's URL is [`get_url`](Self::get_url).
    pub fn get_string(&self) -> Option<String> {
        self.event.get_string()
    }

    /// Get the URL the event is about: the page being navigated to or loaded, or the
    /// link that asked for a new window.
    pub fn get_url(&self) -> Option<String> {
        self.read_string(ffi::wxd_WebViewEvent_GetURL)
    }

    /// Get the name of the frame the event is about, empty for the main frame.
    pub fn get_target(&self) -> Option<String> {
        self.read_string(ffi::wxd_WebViewEvent_GetTarget)
    }

    fn read_string(
        &self,
        getter: unsafe extern "C" fn(*const ffi::wxd_Event_t, *mut std::os::raw::c_char, usize) -> i32,
    ) -> Option<String> {
        let event = self.event.event.0;
        if event.is_null() {
            return None;
        }
        let len = unsafe { getter(event, std::ptr::null_mut(), 0) };
        if len < 0 {
            return None;
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { getter(event, buf.as_mut_ptr(), buf.len()) };
        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
    }

    /// Get the integer value associated with the event (e.g., navigation flags, error code)
    pub fn get_int(&self) -> Option<i32> {
        self.event.get_int()
    }
}

// Use the macro to implement the trait
#[cfg(feature = "webview")]
crate::implement_category_event_handlers!(WebViewEvents, WebViewEvent, WebViewEventData,
    Created => created, EventType::WEBVIEW_CREATED,
    Navigating => navigating, EventType::WEBVIEW_NAVIGATING,
    Navigated => navigated, EventType::WEBVIEW_NAVIGATED,
    Loaded => loaded, EventType::WEBVIEW_LOADED,
    Error => error, EventType::WEBVIEW_ERROR,
    NewWindow => new_window, EventType::WEBVIEW_NEWWINDOW,
    NewWindowFeatures => new_window_features, EventType::WEBVIEW_NEWWINDOW_FEATURES,
    TitleChanged => title_changed, EventType::WEBVIEW_TITLE_CHANGED,
    FullscreenChanged => fullscreen_changed, EventType::WEBVIEW_FULLSCREEN_CHANGED,
    ScriptMessageReceived => script_message_received, EventType::WEBVIEW_SCRIPT_MESSAGE_RECEIVED,
    ScriptResult => script_result, EventType::WEBVIEW_SCRIPT_RESULT,
    WindowCloseRequested => window_close_requested, EventType::WEBVIEW_WINDOW_CLOSE_REQUESTED,
    BrowsingDataCleared => browsing_data_cleared, EventType::WEBVIEW_BROWSING_DATA_CLEARED
);
