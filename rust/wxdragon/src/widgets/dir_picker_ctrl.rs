/* This is a new file */
//! Safe wrapper for wxDirPickerCtrl.

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
            unsafe {
                let dir_picker = DirPickerCtrl::from_ptr(
                    window_obj.handle_ptr() as *mut ffi::wxd_DirPickerCtrl_t
                );
                return dir_picker.get_path();
            }
        }
        String::new()
    }
}

// --- DirPickerCtrl ---
#[derive(Clone)]
pub struct DirPickerCtrl {
    window: Window, // Embed Window
}

impl DirPickerCtrl {
    /// Creates a new DirPickerCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> DirPickerCtrlBuilder<'_> {
        DirPickerCtrlBuilder::new(parent)
    }

    /// Gets the currently selected path.
    pub fn get_path(&self) -> String {
        unsafe {
            let c_str = ffi::wxd_DirPickerCtrl_GetPath(
                self.window.as_ptr() as *mut ffi::wxd_DirPickerCtrl_t
            );
            if c_str.is_null() {
                String::new()
            } else {
                // Free the string using wxd_free_string when we're done with it
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
            ffi::wxd_DirPickerCtrl_SetPath(
                self.window.as_ptr() as *mut ffi::wxd_DirPickerCtrl_t,
                c_path.as_ptr(),
            );
        }
    }

    /// Creates a DirPickerCtrl from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_DirPickerCtrl_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_DirPickerCtrl_t) -> Self {
        DirPickerCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

// Implement event handlers for DirPickerCtrl
crate::implement_widget_local_event_handlers!(
    DirPickerCtrl,
    DirPickerCtrlEvent,
    DirPickerCtrlEventData,
    DirChanged => dir_changed, EventType::DIR_PICKER_CHANGED
);

// Add XRC Support - enables DirPickerCtrl to be created from XRC-managed pointers
impl_xrc_support!(DirPickerCtrl, { window });

// Widget casting support for DirPickerCtrl
impl_widget_cast!(DirPickerCtrl, "wxDirPickerCtrl", { window });

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
        } else {
            unsafe { DirPickerCtrl::from_ptr(ptr) }
        }
    }
);

// Use the implement_widget_traits_with_target macro to implement traits
implement_widget_traits_with_target!(DirPickerCtrl, window, Window);
