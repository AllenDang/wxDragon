//!
//! Safe wrapper for wxStatusBar.

// Add WindowEvents import
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_style_enum;
use crate::widgets::frame::Frame; // Parent must be a Frame
use crate::window::{Window, WxWidget}; // Import trait for handle_ptr() and Window type
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
/// Note: The StatusBar itself is typically managed by the Frame.
#[derive(Clone)]
pub struct StatusBar {
    window: Window, // Use a Window field for widget_traits_with_target
}

impl StatusBar {
    /// Creates a new StatusBar builder.
    /// The parent *must* be a `Frame`.
    pub fn builder(parent: &Frame) -> StatusBarBuilder<'_> {
        StatusBarBuilder::new(parent)
    }

    /// Creates a new StatusBar wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_StatusBar_t` pointer owned by a `Frame`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StatusBar_t) -> Self {
        let window = Window::from_ptr(ptr as *mut ffi::wxd_Window_t);
        Self { window }
    }

    /// Returns the raw underlying status bar pointer.
    /// # Safety
    /// The pointer is managed by the parent Frame.
    pub fn as_ptr(&self) -> *mut ffi::wxd_StatusBar_t {
        self.handle_ptr() as *mut ffi::wxd_StatusBar_t
    }

    /// Sets the number of fields in the status bar.
    pub fn set_fields_count(&self, count: usize) {
        unsafe {
            ffi::wxd_StatusBar_SetFieldsCount(self.as_ptr(), count as c_int);
        }
    }

    /// Sets the text for a specific field.
    pub fn set_status_text(&self, text: &str, field_index: usize) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_StatusBar_SetStatusText(self.as_ptr(), c_text.as_ptr(), field_index as c_int);
        }
    }

    /// Sets the widths of the status bar fields.
    /// `widths`: A slice containing the width for each field.
    ///   - Positive values are absolute widths.
    ///   - Negative values are proportional widths (-1, -2 means ratio 1:2).
    ///   - A width of 0 makes the field flexible.
    pub fn set_status_widths(&self, widths: &[i32]) {
        if !widths.is_empty() {
            unsafe {
                ffi::wxd_StatusBar_SetStatusWidths(
                    self.as_ptr(),
                    widths.len() as c_int,
                    widths.as_ptr(),
                );
            }
        }
    }

    /// Pushes text onto the stack for a field. Reverts on PopStatusText.
    pub fn push_status_text(&self, text: &str, field_index: usize) {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_StatusBar_PushStatusText(self.as_ptr(), c_text.as_ptr(), field_index as c_int);
        }
    }

    /// Pops the last pushed text from the stack for a field.
    pub fn pop_status_text(&self, field_index: usize) {
        unsafe {
            ffi::wxd_StatusBar_PopStatusText(self.as_ptr(), field_index as c_int);
        }
    }
}

// Apply common trait implementations for this widget
implement_widget_traits_with_target!(StatusBar, window, Window);

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
            pos: crate::geometry::DEFAULT_POSITION,
            size: crate::geometry::DEFAULT_SIZE,
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
        if self.parent.handle_ptr().is_null() {
            panic!("Cannot create StatusBar with a null parent frame");
        }

        let status_bar_ptr = unsafe {
            ffi::wxd_StatusBar_Create(
                self.parent.handle_ptr(),
                self.id,
                self.style.bits() as ffi::wxd_Style_t,
            )
        };

        if status_bar_ptr.is_null() {
            panic!("Failed to create wxStatusBar via FFI");
        }

        let status_bar = unsafe { StatusBar::from_ptr(status_bar_ptr) };

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
        unsafe {
            ffi::wxd_Frame_SetStatusBar(
                self.parent.handle_ptr() as *mut ffi::wxd_Frame_t,
                status_bar.as_ptr(),
            );
        }

        status_bar
    }
}

// Add XRC support
// XRC Support - enables StatusBar to be created from XRC-managed pointers
impl_xrc_support!(StatusBar, { window });

// Widget casting support for StatusBar
impl_widget_cast!(StatusBar, "wxStatusBar", { window });
