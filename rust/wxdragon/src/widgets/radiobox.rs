use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use wxdragon_sys as ffi;

/// Events emitted by RadioBox
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioBoxEvent {
    /// Emitted when selection changes in the radio box
    Selected,
}

/// Event data for RadioBox events
#[derive(Debug)]
pub struct RadioBoxEventData {
    event: Event,
}

impl RadioBoxEventData {
    /// Create a new RadioBoxEventData from a generic Event
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

    /// Get the selected item index
    pub fn get_selection(&self) -> Option<i32> {
        self.event.get_int()
    }
}

/// Configuration for creating a RadioBox
#[derive(Debug)]
struct RadioBoxConfig<'a> {
    pub parent_ptr: *mut ffi::wxd_Window_t,
    pub id: Id,
    pub label: &'a str,
    pub choices: &'a [&'a str],
    pub major_dimension: i32,
    pub pos: Point,
    pub size: Size,
    pub style: i64,
}

/// Represents a wxRadioBox control.
///
/// RadioBox uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct RadioBox {
    /// Safe handle to the underlying wxRadioBox - automatically invalidated on destroy
    handle: WindowHandle,
}

impl RadioBox {
    /// Creates a new `RadioBoxBuilder`.
    pub fn builder<'a>(parent: &'a dyn WxWidget, choices: &'a [&'a str]) -> RadioBoxBuilder<'a> {
        // Create a new builder with the parent and convert choices to Strings
        let mut builder = RadioBoxBuilder::new(parent);
        builder.choices = choices.iter().map(|&s| s.to_string()).collect();
        builder
    }

    /// Creates a `RadioBox` from a raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is valid and represents a `wxRadioBox`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_RadioBox_t) -> Self {
        RadioBox {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Low-level constructor used by the builder.
    fn new_impl(config: RadioBoxConfig) -> Self {
        assert!(!config.parent_ptr.is_null(), "RadioBox requires a parent");
        let c_label = CString::new(config.label).expect("CString::new failed for label");

        let c_choices: Vec<CString> = config
            .choices
            .iter()
            .map(|&s| CString::new(s).expect("CString::new failed for choice"))
            .collect();
        let c_choices_ptrs: Vec<*const c_char> = c_choices.iter().map(|cs| cs.as_ptr()).collect();

        let ptr = unsafe {
            ffi::wxd_RadioBox_Create(
                config.parent_ptr,
                config.id,
                c_label.as_ptr(),
                config.pos.into(),
                config.size.into(),
                config.choices.len() as i32,
                c_choices_ptrs.as_ptr(),
                config.major_dimension,
                config.style as ffi::wxd_Style_t,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxRadioBox");
        }
        unsafe { RadioBox::from_ptr(ptr) }
    }

    /// Helper to get raw radiobox pointer, returns null if widget has been destroyed
    #[inline]
    fn radiobox_ptr(&self) -> *mut ffi::wxd_RadioBox_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_RadioBox_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Gets the currently selected item index.
    /// Returns -1 if the radiobox has been destroyed or no selection.
    pub fn get_selection(&self) -> i32 {
        let ptr = self.radiobox_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_RadioBox_GetSelection(ptr) }
    }

    /// Sets the selected item index.
    /// No-op if the radiobox has been destroyed.
    pub fn set_selection(&self, n: i32) {
        let ptr = self.radiobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RadioBox_SetSelection(ptr, n) }
    }

    /// Gets the label of the item at the given index.
    /// Returns empty string if the radiobox has been destroyed or index is invalid.
    pub fn get_string(&self, n: i32) -> String {
        let ptr = self.radiobox_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_RadioBox_GetString(ptr, n, ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buffer = vec![0; len as usize + 1];
        unsafe { ffi::wxd_RadioBox_GetString(ptr, n, buffer.as_mut_ptr(), buffer.len()) };
        unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().to_string() }
    }

    /// Gets the number of items in the radiobox.
    /// Returns 0 if the radiobox has been destroyed.
    pub fn get_count(&self) -> u32 {
        let ptr = self.radiobox_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_RadioBox_GetCount(ptr) }
    }

    /// Enables or disables an individual item.
    /// Returns false if the radiobox has been destroyed.
    pub fn enable_item(&self, n: i32, enable: bool) -> bool {
        let ptr = self.radiobox_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RadioBox_EnableItem(ptr, n, enable) }
    }

    /// Checks if an individual item is enabled.
    /// Returns false if the radiobox has been destroyed.
    pub fn is_item_enabled(&self, n: i32) -> bool {
        let ptr = self.radiobox_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RadioBox_IsItemEnabled(ptr, n) }
    }

    /// Shows or hides an individual item.
    /// Returns false if the radiobox has been destroyed.
    pub fn show_item(&self, n: i32, show: bool) -> bool {
        let ptr = self.radiobox_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RadioBox_ShowItem(ptr, n, show) }
    }

    /// Checks if an individual item is shown.
    /// Returns false if the radiobox has been destroyed.
    pub fn is_item_shown(&self, n: i32) -> bool {
        let ptr = self.radiobox_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RadioBox_IsItemShown(ptr, n) }
    }

    /// Returns the underlying WindowHandle for this radiobox.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for RadioBox (using WindowHandle)
impl WxWidget for RadioBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for RadioBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for RadioBox {}

// Use the widget_builder macro for RadioBox
widget_builder!(
    name: RadioBox,
    parent_type: &'a dyn WxWidget,
    style_type: RadioBoxStyle,
    fields: {
        label: String = String::new(),
        choices: Vec<String> = Vec::new(),
        major_dimension: i32 = 0
    },
    build_impl: |slf| {
        // Convert Vec<String> to Vec<&str> for the new_impl function
        let choices_refs: Vec<&str> = slf.choices.iter().map(|s| s.as_str()).collect();

        RadioBox::new_impl(RadioBoxConfig {
            parent_ptr: slf.parent.handle_ptr(),
            id: slf.id,
            label: &slf.label,
            choices: &choices_refs,
            major_dimension: slf.major_dimension,
            pos: slf.pos,
            size: slf.size,
            style: slf.style.bits(),
        })
    }
);

// Define the RadioBoxStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: RadioBoxStyle,
    doc: "Style flags for RadioBox widgets.",
    variants: {
        Default: 0, "Default layout (wxWidgets decides based on major dimension).",
        SpecifyCols: ffi::WXD_RA_SPECIFY_COLS, "Arrange items in columns primarily.",
        SpecifyRows: ffi::WXD_RA_SPECIFY_ROWS, "Arrange items in rows primarily."
    },
    default_variant: Default
);

// Use the implement_widget_local_event_handlers macro for event handling
crate::implement_widget_local_event_handlers!(
    RadioBox,
    RadioBoxEvent,
    RadioBoxEventData,
    Selected => selected, EventType::COMMAND_RADIOBOX_SELECTED
);

// XRC Support - enables RadioBox to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for RadioBox {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        RadioBox {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for RadioBox
impl crate::window::FromWindowWithClassName for RadioBox {
    fn class_name() -> &'static str {
        "wxRadioBox"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        RadioBox {
            handle: WindowHandle::new(ptr),
        }
    }
}
