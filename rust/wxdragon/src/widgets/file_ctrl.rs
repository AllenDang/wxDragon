use crate::event::{Event, EventType};
use crate::implement_widget_traits_with_target;
use crate::prelude::*;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
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

#[derive(Clone)]
pub struct FileCtrl {
    window: Window, // Composition: FileCtrl IS a Window
}

impl FileCtrl {
    pub fn builder(parent: &dyn WxWidget) -> FileCtrlBuilder<'_> {
        FileCtrlBuilder::new(parent)
    }

    // Create a new FileCtrl from a window and parent pointer
    // This is intended for internal use
    fn new_impl(config: FileCtrlConfig) -> Self {
        assert!(!config.parent_ptr.is_null(), "FileCtrl requires a parent");
        let c_default_dir = CString::new(config.default_directory)
            .expect("CString::new failed for default_directory");
        let c_default_filename = CString::new(config.default_filename)
            .expect("CString::new failed for default_filename");
        let c_wild_card =
            CString::new(config.wild_card).expect("CString::new failed for wild_card");
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
        // Cast the concrete FileCtrl pointer to the base Window pointer for the wrapper
        let window = unsafe { Window::from_ptr(raw_ptr as *mut ffi::wxd_Window_t) };
        FileCtrl { window }
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

// Use the implement_widget_traits_with_target macro to implement traits
implement_widget_traits_with_target!(FileCtrl, window, Window);

// Implement event handlers for FileCtrl
crate::implement_widget_local_event_handlers!(
    FileCtrl,
    FileCtrlEvent,
    FileCtrlEventData,
    FileSelectionChanged => file_selection_changed, EventType::FILE_PICKER_CHANGED,
    FolderSelectionChanged => folder_selection_changed, EventType::DIR_PICKER_CHANGED,
    FileActivated => file_activated, EventType::LIST_ITEM_ACTIVATED
);

// Add XRC Support - enables FileCtrl to be created from XRC-managed pointers
impl_xrc_support!(FileCtrl, { window });
