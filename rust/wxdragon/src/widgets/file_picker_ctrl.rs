/* This is a new file */
//! Safe wrapper for wxFilePickerCtrl.

use std::ffi::{c_longlong, CString};
use wxdragon_sys as ffi;

use crate::event::{Event, EventType};
use crate::implement_widget_traits_with_target;
use crate::prelude::*;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};

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
#[derive(Clone)]
pub struct FilePickerCtrl {
    window: Window, // Embed Window
}

impl FilePickerCtrl {
    /// Creates a new FilePickerCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> FilePickerCtrlBuilder<'_> {
        FilePickerCtrlBuilder::new(parent)
    }

    /// Gets the currently selected path.
    pub fn get_path(&self) -> String {
        unsafe {
            let c_str = ffi::wxd_FilePickerCtrl_GetPath(
                self.window.as_ptr() as *mut ffi::wxd_FilePickerCtrl_t
            );
            if c_str.is_null() {
                String::new()
            } else {
                let rust_str = CString::from_raw(c_str as *mut _)
                    .to_string_lossy()
                    .into_owned();
                rust_str
            }
        }
    }

    /// Sets the currently selected path.
    pub fn set_path(&self, path: &str) {
        let c_path = CString::new(path).expect("CString::new failed for path");
        unsafe {
            ffi::wxd_FilePickerCtrl_SetPath(
                self.window.as_ptr() as *mut ffi::wxd_FilePickerCtrl_t,
                c_path.as_ptr(),
            );
        }
    }

    /// Creates a FilePickerCtrl from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_FilePickerCtrl_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_FilePickerCtrl_t) -> Self {
        FilePickerCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

// Implement event handlers for FilePickerCtrl
crate::implement_widget_local_event_handlers!(
    FilePickerCtrl,
    FilePickerCtrlEvent,
    FilePickerCtrlEventData,
    FileChanged => file_changed, EventType::FILE_PICKER_CHANGED
);

// Add XRC Support - enables FilePickerCtrl to be created from XRC-managed pointers
impl_xrc_support!(FilePickerCtrl, { window });

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
        } else {
            unsafe { FilePickerCtrl::from_ptr(ptr) }
        }
    }
);

// Use the implement_widget_traits_with_target macro to implement traits
implement_widget_traits_with_target!(FilePickerCtrl, window, Window);
