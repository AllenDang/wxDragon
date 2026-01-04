use std::ffi::{CStr, CString};
use std::os::raw::c_longlong;
use wxdragon_sys as ffi;

use crate::color::Colour;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by event data for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;

// --- Style enum using macro ---
widget_style_enum!(
    name: HyperlinkCtrlStyle,
    doc: "Style flags for HyperlinkCtrl.",
    variants: {
        Default: 0x0002, "Default style.",
        AlignLeft: 0x0004, "Align the text to the left (default).",
        AlignRight: 0x0008, "Align the text to the right.",
        AlignCentre: 0x0010, "Center the text.",
        NoUnderline: 0x0020, "Don't show the underline below the link."
    },
    default_variant: Default
);

/// Events emitted by HyperlinkCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperlinkCtrlEvent {
    /// Emitted when the hyperlink is clicked
    Clicked,
}

/// Event data for HyperlinkCtrl events
#[derive(Debug)]
pub struct HyperlinkCtrlEventData {
    event: Event,
}

impl HyperlinkCtrlEventData {
    /// Create a new HyperlinkCtrlEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the ID of the control that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Get the URL associated with the hyperlink control
    pub fn get_url(&self) -> Option<String> {
        // To get the URL, we need to find the HyperlinkCtrl that triggered this event
        if let Some(window_obj) = self.event.get_event_object() {
            // Create a HyperlinkCtrl from the window pointer

            let hyperlink = unsafe { HyperlinkCtrl::from_ptr(window_obj.handle_ptr() as *mut ffi::wxd_HyperlinkCtrl_t) };
            let url = hyperlink.get_url();
            if !url.is_empty() {
                return Some(url);
            }
        }
        None
    }
}

// --- HyperlinkCtrl --- //
/// Represents a wxHyperlinkCtrl.
///
/// HyperlinkCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct HyperlinkCtrl {
    /// Safe handle to the underlying wxHyperlinkCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl HyperlinkCtrl {
    /// Creates a new HyperlinkCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> HyperlinkCtrlBuilder<'_> {
        HyperlinkCtrlBuilder::new(parent)
    }

    /// Helper to get raw hyperlink pointer, returns null if widget has been destroyed
    #[inline]
    fn hyperlink_ptr(&self) -> *mut ffi::wxd_HyperlinkCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_HyperlinkCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Gets the URL associated with the hyperlink.
    /// Returns empty string if the hyperlink has been destroyed.
    pub fn get_url(&self) -> String {
        let ptr = self.hyperlink_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_HyperlinkCtrl_GetURL(ptr, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_HyperlinkCtrl_GetURL(ptr, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() }
    }

    /// Sets the URL associated with the hyperlink.
    /// No-op if the hyperlink has been destroyed.
    pub fn set_url(&self, url: &str) {
        let ptr = self.hyperlink_ptr();
        if ptr.is_null() {
            return;
        }
        let c_url = CString::new(url).expect("CString::new failed for url");
        unsafe { ffi::wxd_HyperlinkCtrl_SetURL(ptr, c_url.as_ptr()) }
    }

    /// Returns whether the hyperlink has been visited.
    /// Returns false if the hyperlink has been destroyed.
    pub fn get_visited(&self) -> bool {
        let ptr = self.hyperlink_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_HyperlinkCtrl_GetVisited(ptr) }
    }

    /// Sets whether the hyperlink has been visited.
    /// No-op if the hyperlink has been destroyed.
    pub fn set_visited(&self, visited: bool) {
        let ptr = self.hyperlink_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_HyperlinkCtrl_SetVisited(ptr, visited) }
    }

    /// Gets the colour used when the mouse hovers over the hyperlink.
    /// Returns black if the hyperlink has been destroyed.
    pub fn get_hover_colour(&self) -> Colour {
        let ptr = self.hyperlink_ptr();
        if ptr.is_null() {
            return Colour::rgb(0, 0, 0);
        }
        let val = unsafe { ffi::wxd_HyperlinkCtrl_GetHoverColour(ptr) };
        Colour::from_u32(val as u32)
    }

    /// Sets the colour used when the mouse hovers over the hyperlink.
    /// No-op if the hyperlink has been destroyed.
    pub fn set_hover_colour(&self, colour: Colour) {
        let ptr = self.hyperlink_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_HyperlinkCtrl_SetHoverColour(ptr, colour.as_u32() as std::os::raw::c_ulong) }
    }

    /// Gets the normal colour of the hyperlink.
    /// Returns black if the hyperlink has been destroyed.
    pub fn get_normal_colour(&self) -> Colour {
        let ptr = self.hyperlink_ptr();
        if ptr.is_null() {
            return Colour::rgb(0, 0, 0);
        }
        let val = unsafe { ffi::wxd_HyperlinkCtrl_GetNormalColour(ptr) };
        Colour::from_u32(val as u32)
    }

    /// Sets the normal colour of the hyperlink.
    /// No-op if the hyperlink has been destroyed.
    pub fn set_normal_colour(&self, colour: Colour) {
        let ptr = self.hyperlink_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_HyperlinkCtrl_SetNormalColour(ptr, colour.as_u32() as std::os::raw::c_ulong) }
    }

    /// Gets the colour of the visited hyperlink.
    /// Returns black if the hyperlink has been destroyed.
    pub fn get_visited_colour(&self) -> Colour {
        let ptr = self.hyperlink_ptr();
        if ptr.is_null() {
            return Colour::rgb(0, 0, 0);
        }
        let val = unsafe { ffi::wxd_HyperlinkCtrl_GetVisitedColour(ptr) };
        Colour::from_u32(val as u32)
    }

    /// Sets the colour of the visited hyperlink.
    /// No-op if the hyperlink has been destroyed.
    pub fn set_visited_colour(&self, colour: Colour) {
        let ptr = self.hyperlink_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_HyperlinkCtrl_SetVisitedColour(ptr, colour.as_u32() as std::os::raw::c_ulong) }
    }

    /// Creates a HyperlinkCtrl from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_HyperlinkCtrl_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_HyperlinkCtrl_t) -> Self {
        HyperlinkCtrl {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the underlying WindowHandle for this hyperlink control.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for HyperlinkCtrl (using WindowHandle)
impl WxWidget for HyperlinkCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for HyperlinkCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for HyperlinkCtrl {}

// Implement event handlers for HyperlinkCtrl
crate::implement_widget_local_event_handlers!(
    HyperlinkCtrl,
    HyperlinkCtrlEvent,
    HyperlinkCtrlEventData,
    Clicked => clicked, EventType::COMMAND_HYPERLINK
);

// XRC Support - enables HyperlinkCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for HyperlinkCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        HyperlinkCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Use the widget_builder macro to generate the HyperlinkCtrlBuilder implementation
widget_builder!(
    name: HyperlinkCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: HyperlinkCtrlStyle,
    fields: {
        label: String = String::new(),
        url: String = String::new()
    },
    build_impl: |slf| {
        let c_label = CString::new(&slf.label[..]).expect("CString::new failed for label");
        let c_url = CString::new(&slf.url[..]).expect("CString::new failed for url");
        let raw_ptr = unsafe {
            ffi::wxd_HyperlinkCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                c_label.as_ptr(),
                c_url.as_ptr(),
                slf.pos.x,
                slf.pos.y,
                slf.size.width,
                slf.size.height,
                slf.style.bits() as c_longlong,
            )
        };
        if raw_ptr.is_null() {
            panic!("Failed to create wxHyperlinkCtrl");
        }
        HyperlinkCtrl {
            handle: WindowHandle::new(raw_ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// Enable widget casting for HyperlinkCtrl
impl crate::window::FromWindowWithClassName for HyperlinkCtrl {
    fn class_name() -> &'static str {
        "wxHyperlinkCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        HyperlinkCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
