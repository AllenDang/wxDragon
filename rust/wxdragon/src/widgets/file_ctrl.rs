use crate::event::{Event, EventType, WxEvtHandler};
use crate::prelude::*;
use crate::window::{WindowHandle, WxWidget};
// Window is used by impl_xrc_support for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

// Default wildcard pattern for FileCtrl
const ALL_FILES_PATTERN: &str = "*.*";

// Define the FileCtrlStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: FileCtrlStyle,
    doc: "Style flags for `FileCtrl`.",
    variants: {
        Open: ffi::WXD_FC_OPEN, "Default style for opening files.",
        Save: ffi::WXD_FC_SAVE, "For saving files.",
        Multiple: ffi::WXD_FC_MULTIPLE, "Allow multiple files to be selected.",
        NoShowHidden: ffi::WXD_FC_NOSHOWHIDDEN, "Don't show hidden files."
    },
    default_variant: Open
);

/// Events emitted by FileCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileCtrlEvent {
    /// Emitted when a file selection changes
    FileSelectionChanged,
    /// Emitted when a folder selection changes
    FolderSelectionChanged,
    /// Emitted when a file is activated (typically by double-clicking)
    FileActivated,
}

/// Event data for FileCtrl events
#[derive(Debug)]
pub struct FileCtrlEventData {
    event: Event,
}

impl FileCtrlEventData {
    /// Create a new FileCtrlEventData from a generic Event
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
}

/// Configuration for creating a FileCtrl
#[derive(Debug)]
struct FileCtrlConfig {
    pub parent_ptr: *mut ffi::wxd_Window_t,
    pub id: Id,
    pub default_directory: String,
    pub default_filename: String,
    pub wild_card: String,
    pub style: i64,
    pub pos: Point,
    pub size: Size,
    pub name: String,
}

/// Represents a wxFileCtrl.
///
/// FileCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let file_ctrl = FileCtrl::builder(&frame).build();
///
/// // FileCtrl is Copy - no clone needed for closures!
/// file_ctrl.bind_file_selection_changed(move |_| {
///     // Safe: if file_ctrl was destroyed, this is a no-op
///     if let Some(path) = file_ctrl.get_path() {
///         println!("Selected: {}", path);
///     }
/// });
///
/// // After parent destruction, file_ctrl operations are safe no-ops
/// frame.destroy();
/// assert!(!file_ctrl.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct FileCtrl {
    /// Safe handle to the underlying wxFileCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl FileCtrl {
    /// Creates a new `FileCtrlBuilder` for constructing a file control.
    pub fn builder(parent: &dyn WxWidget) -> FileCtrlBuilder<'_> {
        FileCtrlBuilder::new(parent)
    }

    /// Creates a new FileCtrl from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Creates a new FileCtrl (low-level constructor used by the builder)
    fn new_impl(config: FileCtrlConfig) -> Self {
        assert!(!config.parent_ptr.is_null(), "FileCtrl requires a parent");
        let c_default_dir = CString::new(config.default_directory).expect("CString::new failed for default_directory");
        let c_default_filename = CString::new(config.default_filename).expect("CString::new failed for default_filename");
        let c_wild_card = CString::new(config.wild_card).expect("CString::new failed for wild_card");
        let c_name = CString::new(config.name).expect("CString::new failed for name");

        let raw_ptr = unsafe {
            ffi::wxd_FileCtrl_Create(
                config.parent_ptr,
                config.id,
                c_default_dir.as_ptr(),
                c_default_filename.as_ptr(),
                c_wild_card.as_ptr(),
                config.style,
                config.pos.x,
                config.pos.y,
                config.size.width,
                config.size.height,
                c_name.as_ptr(),
            )
        };
        if raw_ptr.is_null() {
            panic!("Failed to create wxFileCtrl via FFI");
        }

        // Create a WindowHandle which automatically registers for destroy events
        FileCtrl {
            handle: WindowHandle::new(raw_ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw file control pointer, returns null if widget has been destroyed
    #[inline]
    fn file_ctrl_ptr(&self) -> *mut ffi::wxd_FileCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_FileCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Returns the underlying WindowHandle for this file control.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

impl FileCtrl {
    /// Get the currently selected path in the FileCtrl.
    /// Returns None if the file control has been destroyed or no path is selected.
    pub fn get_path(&self) -> Option<String> {
        let ptr = self.file_ctrl_ptr();
        if ptr.is_null() {
            return None;
        }
        let len = unsafe { ffi::wxd_FileCtrl_GetPath(ptr, std::ptr::null_mut(), 0) };
        if len == 0 {
            return None; // No path selected or error
        }
        // Now allocate buffer and retrieve the path
        let mut buf = vec![0; len + 1]; // +1 for null terminator
        unsafe { ffi::wxd_FileCtrl_GetPath(ptr, buf.as_mut_ptr(), buf.len()) };
        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
    }
}

// Use the widget_builder macro to generate the FileCtrlBuilder implementation
widget_builder!(
    name: FileCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: FileCtrlStyle,
    fields: {
        default_directory: String = String::new(),
        default_filename: String = String::new(),
        wild_card: String = ALL_FILES_PATTERN.to_string(),
        name: String = "FileCtrl".to_string()
    },
    build_impl: |slf| {
        FileCtrl::new_impl(FileCtrlConfig {
            parent_ptr: slf.parent.handle_ptr(),
            id: slf.id,
            default_directory: slf.default_directory,
            default_filename: slf.default_filename,
            wild_card: slf.wild_card,
            style: slf.style.bits(),
            pos: slf.pos,
            size: slf.size,
            name: slf.name,
        })
    }
);

// Manual WxWidget implementation for FileCtrl (using WindowHandle)
impl WxWidget for FileCtrl {
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
impl WxEvtHandler for FileCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for FileCtrl {}

// Implement event handlers for FileCtrl
crate::implement_widget_local_event_handlers!(
    FileCtrl,
    FileCtrlEvent,
    FileCtrlEventData,
    FileSelectionChanged => file_selection_changed, EventType::FILE_PICKER_CHANGED,
    FolderSelectionChanged => folder_selection_changed, EventType::DIR_PICKER_CHANGED,
    FileActivated => file_activated, EventType::LIST_ITEM_ACTIVATED
);

// XRC Support - enables FileCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for FileCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        FileCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for FileCtrl
impl crate::window::FromWindowWithClassName for FileCtrl {
    fn class_name() -> &'static str {
        "wxFileCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        FileCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
