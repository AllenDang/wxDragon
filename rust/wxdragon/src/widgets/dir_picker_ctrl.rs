/* This is a new file */
//! Safe wrapper for wxDirPickerCtrl.

use std::ffi::{CStr, CString, c_longlong};
use wxdragon_sys as ffi;

use crate::event::{Event, EventType, WxEvtHandler};
use crate::prelude::*;
use crate::window::{WindowHandle, WxWidget};
// Window is used by new_from_composition for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;

// --- Style enum using macro ---
widget_style_enum!(
    name: DirPickerCtrlStyle,
    doc: "Style flags for DirPickerCtrl widgets.",
    variants: {
        Default: ffi::WXD_DIRP_DEFAULT_STYLE, "Default style, often includes UseTextCtrl.",
        DirMustExist: ffi::WXD_DIRP_DIR_MUST_EXIST, "The directory must exist.",
        ChangeDir: ffi::WXD_DIRP_CHANGE_DIR, "Change the current working directory when a directory is selected.",
        UseTextCtrl: ffi::WXD_DIRP_USE_TEXTCTRL, "Use a text control to display the selected directory."
    },
    default_variant: Default
);

/// Events emitted by DirPickerCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirPickerCtrlEvent {
    /// Emitted when the directory is changed
    DirChanged,
}

/// Event data for DirPickerCtrl events
#[derive(Debug)]
pub struct DirPickerCtrlEventData {
    event: Event,
}

impl DirPickerCtrlEventData {
    /// Create a new DirPickerCtrlEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the ID of the control that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Get the path that was selected
    pub fn get_path(&self) -> String {
        // First, get the window that triggered this event
        if let Some(window_obj) = self.event.get_event_object() {
            // We need to find the DirPickerCtrl that corresponds to this window.
            // In wxdragon, we can create a DirPickerCtrl with the Window's handle pointer
            let dir_picker = DirPickerCtrl::from_ptr(window_obj.handle_ptr());
            return dir_picker.get_path();
        }
        String::new()
    }
}

// --- DirPickerCtrl ---
/// Represents a wxDirPickerCtrl.
///
/// DirPickerCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let dir_picker = DirPickerCtrl::builder(&frame).path("/home").build();
///
/// // DirPickerCtrl is Copy - no clone needed for closures!
/// dir_picker.bind_dir_changed(move |_| {
///     // Safe: if dir_picker was destroyed, this is a no-op
///     println!("Path: {}", dir_picker.get_path());
/// });
///
/// // After parent destruction, dir_picker operations are safe no-ops
/// frame.destroy();
/// assert!(!dir_picker.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct DirPickerCtrl {
    /// Safe handle to the underlying wxDirPickerCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl DirPickerCtrl {
    /// Creates a new DirPickerCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> DirPickerCtrlBuilder<'_> {
        DirPickerCtrlBuilder::new(parent)
    }

    /// Creates a new DirPickerCtrl from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Creates a new DirPickerCtrl from a raw window pointer.
    /// This is for backwards compatibility with widgets that compose DirPickerCtrl.
    /// The parent_ptr parameter is ignored (kept for API compatibility).
    #[allow(dead_code)]
    pub(crate) fn new_from_composition(_window: Window, _parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        // Use the window's pointer to create a new WindowHandle
        Self {
            handle: WindowHandle::new(_window.as_ptr()),
        }
    }

    /// Helper to get raw dir picker pointer, returns null if widget has been destroyed
    #[inline]
    fn dir_picker_ptr(&self) -> *mut ffi::wxd_DirPickerCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_DirPickerCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Gets the currently selected path.
    /// Returns empty string if the control has been destroyed.
    pub fn get_path(&self) -> String {
        let ptr = self.dir_picker_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_DirPickerCtrl_GetPath(ptr, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0; len as usize + 1]; // +1 for null terminator
        unsafe { ffi::wxd_DirPickerCtrl_GetPath(ptr, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() }
    }

    /// Sets the currently selected path.
    /// No-op if the control has been destroyed.
    pub fn set_path(&self, path: &str) {
        let ptr = self.dir_picker_ptr();
        if ptr.is_null() {
            return;
        }
        let c_path = CString::new(path).expect("CString::new failed for path");
        unsafe { ffi::wxd_DirPickerCtrl_SetPath(ptr, c_path.as_ptr()) };
    }

    /// Returns the underlying WindowHandle for this control.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for DirPickerCtrl (using WindowHandle)
impl WxWidget for DirPickerCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Note: We don't implement Deref to Window because returning a reference
// to a temporary Window is unsound. Users can access window methods through
// the WxWidget trait methods directly.

// Implement WxEvtHandler for event binding
impl WxEvtHandler for DirPickerCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for DirPickerCtrl {}

// Implement event handlers for DirPickerCtrl
crate::implement_widget_local_event_handlers!(
    DirPickerCtrl,
    DirPickerCtrlEvent,
    DirPickerCtrlEventData,
    DirChanged => dir_changed, EventType::DIR_PICKER_CHANGED
);

// Use the widget_builder macro to generate the DirPickerCtrlBuilder implementation
widget_builder!(
    name: DirPickerCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: DirPickerCtrlStyle,
    fields: {
        message: String = "Select a directory".to_string(),
        path: String = String::new()
    },
    build_impl: |slf| {
        assert!(!slf.parent.handle_ptr().is_null(), "DirPickerCtrl requires a parent");

        let c_message = CString::new(&slf.message[..]).expect("CString::new failed for message");
        let c_path = CString::new(&slf.path[..]).expect("CString::new failed for path");

        let ptr = unsafe {
            ffi::wxd_DirPickerCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                c_message.as_ptr(),
                c_path.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as c_longlong,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create DirPickerCtrl: FFI returned null pointer.");
        }

        // Create a WindowHandle which automatically registers for destroy events
        DirPickerCtrl {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// XRC Support - enables DirPickerCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for DirPickerCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        DirPickerCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for DirPickerCtrl
impl crate::window::FromWindowWithClassName for DirPickerCtrl {
    fn class_name() -> &'static str {
        "wxDirPickerCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        DirPickerCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
