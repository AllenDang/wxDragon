/* This is a new file */
//! Safe wrapper for wxFilePickerCtrl.

use std::ffi::{CString, c_longlong};
use wxdragon_sys as ffi;

use crate::event::{Event, EventType, WxEvtHandler};
use crate::prelude::*;
use crate::window::{WindowHandle, WxWidget};
// Window is used by new_from_composition for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;

// --- Style enum using macro ---
widget_style_enum!(
    name: FilePickerCtrlStyle,
    doc: "Style flags for FilePickerCtrl widgets.",
    variants: {
        DefaultStyle: ffi::WXD_FLP_DEFAULT_STYLE, "Default style, usually a combination of flags.",
        Open: ffi::WXD_FLP_OPEN, "For opening files.",
        Save: ffi::WXD_FLP_SAVE, "For saving files.",
        OverwritePrompt: ffi::WXD_FLP_OVERWRITE_PROMPT, "Prompt before overwriting an existing file (Save mode only).",
        FileMustExist: ffi::WXD_FLP_FILE_MUST_EXIST, "The selected file must exist (Open mode only).",
        ChangeDir: ffi::WXD_FLP_CHANGE_DIR, "Change the current working directory when a file is selected.",
        UseTextCtrl: ffi::WXD_FLP_USE_TEXTCTRL, "Use a text control to display the selected file."
    },
    default_variant: DefaultStyle
);

/// Events emitted by FilePickerCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilePickerCtrlEvent {
    /// Emitted when the file is changed
    FileChanged,
}

/// Event data for FilePickerCtrl events
#[derive(Debug)]
pub struct FilePickerCtrlEventData {
    event: Event,
}

impl FilePickerCtrlEventData {
    /// Create a new FilePickerCtrlEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the ID of the control that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }
}

// --- FilePickerCtrl ---
/// Represents a wxFilePickerCtrl, which allows the user to select a file.
///
/// FilePickerCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let file_picker = FilePickerCtrl::builder(&frame).build();
///
/// // FilePickerCtrl is Copy - no clone needed for closures!
/// file_picker.bind_file_changed(move |_| {
///     // Safe: if file_picker was destroyed, this is a no-op
///     let path = file_picker.get_path();
/// });
///
/// // After parent destruction, file_picker operations are safe no-ops
/// frame.destroy();
/// assert!(!file_picker.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct FilePickerCtrl {
    /// Safe handle to the underlying wxFilePickerCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl FilePickerCtrl {
    /// Creates a new FilePickerCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> FilePickerCtrlBuilder<'_> {
        FilePickerCtrlBuilder::new(parent)
    }

    /// Creates a new FilePickerCtrl from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Creates a new FilePickerCtrl from a raw window pointer.
    /// This is for backwards compatibility with widgets that compose FilePickerCtrl.
    /// The parent_ptr parameter is ignored (kept for API compatibility).
    #[allow(dead_code)]
    pub(crate) fn new_from_composition(_window: Window, _parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        // Use the window's pointer to create a new WindowHandle
        Self {
            handle: WindowHandle::new(_window.as_ptr()),
        }
    }

    /// Helper to get raw file picker pointer, returns null if widget has been destroyed
    #[inline]
    fn file_picker_ptr(&self) -> *mut ffi::wxd_FilePickerCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_FilePickerCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Gets the currently selected path.
    /// Returns empty string if the file picker has been destroyed.
    pub fn get_path(&self) -> String {
        let ptr = self.file_picker_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let c_str = unsafe { ffi::wxd_FilePickerCtrl_GetPath(ptr) };
        if c_str.is_null() {
            String::new()
        } else {
            unsafe { CString::from_raw(c_str as *mut _).to_string_lossy().into_owned() }
        }
    }

    /// Sets the currently selected path.
    /// No-op if the file picker has been destroyed.
    pub fn set_path(&self, path: &str) {
        let ptr = self.file_picker_ptr();
        if ptr.is_null() {
            return;
        }
        let c_path = CString::new(path).expect("CString::new failed for path");
        unsafe { ffi::wxd_FilePickerCtrl_SetPath(ptr, c_path.as_ptr()) };
    }

    /// Returns the underlying WindowHandle for this file picker.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Implement event handlers for FilePickerCtrl
crate::implement_widget_local_event_handlers!(
    FilePickerCtrl,
    FilePickerCtrlEvent,
    FilePickerCtrlEventData,
    FileChanged => file_changed, EventType::FILE_PICKER_CHANGED
);

// Manual WxWidget implementation for FilePickerCtrl (using WindowHandle)
impl WxWidget for FilePickerCtrl {
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
impl WxEvtHandler for FilePickerCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for FilePickerCtrl {}

// Add XRC Support - enables FilePickerCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for FilePickerCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        FilePickerCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Use the widget_builder macro to generate the FilePickerCtrlBuilder implementation
widget_builder!(
    name: FilePickerCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: FilePickerCtrlStyle,
    fields: {
        message: String = "Select a file".to_string(),
        wildcard: String = "*.*".to_string(),
        path: String = String::new()
    },
    build_impl: |slf| {
        assert!(!slf.parent.handle_ptr().is_null(), "FilePickerCtrl requires a parent");

        let c_message = CString::new(&slf.message[..]).expect("CString::new failed for message");
        let c_wildcard = CString::new(&slf.wildcard[..]).expect("CString::new failed for wildcard");
        let c_path = CString::new(&slf.path[..]).expect("CString::new failed for path");

        let ptr = unsafe {
            ffi::wxd_FilePickerCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                c_message.as_ptr(),
                c_wildcard.as_ptr(),
                c_path.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as c_longlong,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create FilePickerCtrl: FFI returned null pointer.");
        }

        // Create a WindowHandle which automatically registers for destroy events
        FilePickerCtrl {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// Enable widget casting for FilePickerCtrl
impl crate::window::FromWindowWithClassName for FilePickerCtrl {
    fn class_name() -> &'static str {
        "wxFilePickerCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        FilePickerCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
