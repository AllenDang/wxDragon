//!
//! Safe wrapper for wxStatusBar.

use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::widgets::frame::Frame; // Parent must be a Frame
use crate::window::{WindowHandle, WxWidget};
// Window is used by impl_xrc_support and impl_widget_cast macros for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// --- Constants (Styles) ---
// Default constant (needs to be added to const_extractor later)
pub const STB_DEFAULT_STYLE: i64 = 0;

// Define a style enum for StatusBar
widget_style_enum!(
    name: StatusBarStyle,
    doc: "Style flags for StatusBar widget.",
    variants: {
        Default: 0, "Default style with no special behavior."
    },
    default_variant: Default
);

/// Represents a wxStatusBar attached to a Frame.
///
/// StatusBar uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// Note: The StatusBar itself is typically managed by the Frame.
///
/// # Example
/// ```ignore
/// let status_bar = StatusBar::builder(&frame).build();
///
/// // StatusBar is Copy - no clone needed for closures!
/// status_bar.set_status_text("Ready", 0);
///
/// // After parent destruction, status_bar operations are safe no-ops
/// frame.destroy();
/// assert!(!status_bar.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct StatusBar {
    /// Safe handle to the underlying wxStatusBar - automatically invalidated on destroy
    handle: WindowHandle,
}

impl StatusBar {
    /// Creates a new StatusBar builder.
    /// The parent *must* be a `Frame`.
    pub fn builder(parent: &Frame) -> StatusBarBuilder<'_> {
        StatusBarBuilder::new(parent)
    }

    /// Creates a new StatusBar wrapper from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StatusBar_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw status bar pointer, returns null if widget has been destroyed
    #[inline]
    fn statusbar_ptr(&self) -> *mut ffi::wxd_StatusBar_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_StatusBar_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Returns the raw underlying status bar pointer for interop.
    /// Returns null if the status bar has been destroyed.
    /// This is primarily for use by other widgets like Frame.
    pub fn as_ptr(&self) -> *mut ffi::wxd_StatusBar_t {
        self.statusbar_ptr()
    }

    /// Returns the underlying WindowHandle for this status bar.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    /// Sets the number of fields in the status bar.
    /// No-op if the status bar has been destroyed.
    pub fn set_fields_count(&self, count: usize) {
        let ptr = self.statusbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_StatusBar_SetFieldsCount(ptr, count as c_int);
        }
    }

    /// Sets the text for a specific field.
    /// No-op if the status bar has been destroyed.
    pub fn set_status_text(&self, text: &str, field_index: usize) {
        let ptr = self.statusbar_ptr();
        if ptr.is_null() {
            return;
        }
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_StatusBar_SetStatusText(ptr, c_text.as_ptr(), field_index as c_int);
        }
    }

    /// Sets the widths of the status bar fields.
    ///
    /// `widths`: A slice containing the width for each field.
    /// - Positive values are absolute widths.
    /// - Negative values are proportional widths (-1, -2 means ratio 1:2).
    /// - A width of 0 makes the field flexible.
    ///
    /// No-op if the status bar has been destroyed.
    pub fn set_status_widths(&self, widths: &[i32]) {
        let ptr = self.statusbar_ptr();
        if ptr.is_null() {
            return;
        }
        if !widths.is_empty() {
            unsafe { ffi::wxd_StatusBar_SetStatusWidths(ptr, widths.len() as c_int, widths.as_ptr()) };
        }
    }

    /// Pushes text onto the stack for a field. Reverts on PopStatusText.
    /// No-op if the status bar has been destroyed.
    pub fn push_status_text(&self, text: &str, field_index: usize) {
        let ptr = self.statusbar_ptr();
        if ptr.is_null() {
            return;
        }
        let c_text = CString::new(text).unwrap_or_default();
        unsafe { ffi::wxd_StatusBar_PushStatusText(ptr, c_text.as_ptr(), field_index as c_int) };
    }

    /// Pops the last pushed text from the stack for a field.
    /// No-op if the status bar has been destroyed.
    pub fn pop_status_text(&self, field_index: usize) {
        let ptr = self.statusbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_StatusBar_PopStatusText(ptr, field_index as c_int) };
    }
}

// Manual WxWidget implementation for StatusBar (using WindowHandle)
impl WxWidget for StatusBar {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for StatusBar {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for StatusBar {}

// --- Builder Pattern manually implemented ---
#[derive(Clone)]
pub struct StatusBarBuilder<'a> {
    parent: &'a Frame,
    id: Id,
    pos: Point,
    size: Size,
    style: StatusBarStyle,
    fields_count: Option<usize>,
    status_widths: Option<Vec<i32>>,
    initial_texts: Option<Vec<(usize, String)>>,
}

impl<'a> StatusBarBuilder<'a> {
    pub fn new(parent: &'a Frame) -> Self {
        Self {
            parent,
            id: crate::id::ID_ANY as Id,
            pos: Point::DEFAULT_POSITION,
            size: Size::DEFAULT_SIZE,
            style: StatusBarStyle::Default,
            fields_count: None,
            status_widths: None,
            initial_texts: None,
        }
    }

    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the position.
    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the size.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets the window style flags.
    pub fn with_style(mut self, style: StatusBarStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the initial number of fields.
    pub fn with_fields_count(mut self, count: usize) -> Self {
        self.fields_count = Some(count);
        self
    }

    /// Sets the initial status widths.
    pub fn with_status_widths(mut self, widths: Vec<i32>) -> Self {
        self.status_widths = Some(widths);
        self
    }

    /// Adds initial text for a specific field.
    pub fn add_initial_text(mut self, field_index: usize, text: &str) -> Self {
        let texts = self.initial_texts.get_or_insert_with(Vec::new);
        // Remove existing text for this index if present
        texts.retain(|(idx, _)| *idx != field_index);
        texts.push((field_index, text.to_string()));
        self
    }

    /// Creates the `StatusBar` and attaches it to the parent `Frame`.
    /// Returns the `StatusBar` wrapper.
    /// Panics if creation fails (FFI returns null) or parent frame is invalid.
    pub fn build(self) -> StatusBar {
        let parent_ptr = self.parent.handle_ptr();
        if parent_ptr.is_null() {
            panic!("Cannot create StatusBar with a null parent frame");
        }

        let status_bar_ptr = unsafe { ffi::wxd_StatusBar_Create(parent_ptr, self.id, self.style.bits() as ffi::wxd_Style_t) };

        if status_bar_ptr.is_null() {
            panic!("Failed to create wxStatusBar via FFI");
        }

        // Create a StatusBar with WindowHandle which automatically registers for destroy events
        let status_bar = StatusBar {
            handle: WindowHandle::new(status_bar_ptr as *mut ffi::wxd_Window_t),
        };

        // Apply configurations
        if let Some(count) = self.fields_count {
            status_bar.set_fields_count(count);
        }
        if let Some(widths) = &self.status_widths {
            status_bar.set_status_widths(widths);
        }
        if let Some(texts) = &self.initial_texts {
            for (index, text) in texts {
                status_bar.set_status_text(text, *index);
            }
        }

        // Attach the status bar to the frame (Frame takes ownership)
        unsafe { ffi::wxd_Frame_SetStatusBar(parent_ptr as *mut ffi::wxd_Frame_t, status_bar.statusbar_ptr()) };

        status_bar
    }
}

// XRC Support - enables StatusBar to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for StatusBar {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        StatusBar {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for StatusBar
impl crate::window::FromWindowWithClassName for StatusBar {
    fn class_name() -> &'static str {
        "wxStatusBar"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        StatusBar {
            handle: WindowHandle::new(ptr),
        }
    }
}
