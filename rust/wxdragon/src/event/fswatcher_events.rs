//! Event system for `FileSystemWatcher`.

use crate::event::{Event, EventType};
use wxdragon_sys as ffi;

/// Events specific to `FileSystemWatcher`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FSWatcherEvent {
    /// Fired whenever a watched path changes (create, delete, rename, modify,
    /// and so on), or when a warning/error occurs while watching.
    Changed,
}

/// The kind of change a `FSWatcherEventData` describes, matching
/// `wxFSW_EVENT_*`. Combine with bitwise OR when calling
/// [`FileSystemWatcher::add`](crate::fswatcher::FileSystemWatcher::add) or
/// [`add_tree`](crate::fswatcher::FileSystemWatcher::add_tree).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
// The `as i32` casts below are unnecessary on MSVC (where C enums are already
// `int`) but required on MinGW/GNU targets (where they default to `unsigned int`).
#[allow(clippy::unnecessary_cast)]
pub struct FSWatcherEventKind(i32);

#[allow(clippy::unnecessary_cast)]
impl FSWatcherEventKind {
    pub const CREATE: Self = Self(ffi::WXDFSWEventCEnum_WXD_FSW_EVENT_CREATE as i32);
    pub const DELETE: Self = Self(ffi::WXDFSWEventCEnum_WXD_FSW_EVENT_DELETE as i32);
    pub const RENAME: Self = Self(ffi::WXDFSWEventCEnum_WXD_FSW_EVENT_RENAME as i32);
    pub const MODIFY: Self = Self(ffi::WXDFSWEventCEnum_WXD_FSW_EVENT_MODIFY as i32);
    /// GTK only.
    pub const ACCESS: Self = Self(ffi::WXDFSWEventCEnum_WXD_FSW_EVENT_ACCESS as i32);
    /// GTK only.
    pub const ATTRIB: Self = Self(ffi::WXDFSWEventCEnum_WXD_FSW_EVENT_ATTRIB as i32);
    pub const WARNING: Self = Self(ffi::WXDFSWEventCEnum_WXD_FSW_EVENT_WARNING as i32);
    pub const ERROR: Self = Self(ffi::WXDFSWEventCEnum_WXD_FSW_EVENT_ERROR as i32);
    pub const ALL: Self = Self(ffi::WXDFSWEventCEnum_WXD_FSW_EVENT_ALL as i32);

    pub fn bits(self) -> i32 {
        self.0
    }

    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for FSWatcherEventKind {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// Event data for a `FileSystemWatcher` event.
#[derive(Debug)]
pub struct FSWatcherEventData {
    event: Event,
}

impl FSWatcherEventData {
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// What kind of change this event describes.
    pub fn get_change_type(&self) -> FSWatcherEventKind {
        let ptr = self.event._as_ptr();
        if ptr.is_null() {
            return FSWatcherEventKind(0);
        }
        FSWatcherEventKind(unsafe { ffi::wxd_FileSystemWatcherEvent_GetChangeType(ptr) })
    }

    /// The path the change occurred at.
    pub fn get_path(&self) -> String {
        let ptr = self.event._as_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let c_str = unsafe { ffi::wxd_FileSystemWatcherEvent_GetPath(ptr) };
        string_from_owned_c_str(c_str)
    }

    /// For rename events, the new path; equal to [`get_path`](Self::get_path)
    /// for other event kinds.
    pub fn get_new_path(&self) -> String {
        let ptr = self.event._as_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let c_str = unsafe { ffi::wxd_FileSystemWatcherEvent_GetNewPath(ptr) };
        string_from_owned_c_str(c_str)
    }

    /// True if this event describes a warning or error rather than a change.
    pub fn is_error(&self) -> bool {
        let ptr = self.event._as_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_FileSystemWatcherEvent_IsError(ptr) }
    }

    /// Description of the warning/error, if [`is_error`](Self::is_error) is true.
    pub fn get_error_description(&self) -> String {
        let ptr = self.event._as_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let c_str = unsafe { ffi::wxd_FileSystemWatcherEvent_GetErrorDescription(ptr) };
        string_from_owned_c_str(c_str)
    }
}

fn string_from_owned_c_str(ptr: *mut std::os::raw::c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    // The C side hands us a strdup'd string it no longer owns. Read it via a
    // borrowing CStr and free it with wxd_FreeCString, NOT CString::from_raw,
    // which would deallocate C-runtime memory with Rust's global allocator.
    let s = unsafe { std::ffi::CStr::from_ptr(ptr) }.to_string_lossy().into_owned();
    unsafe { ffi::wxd_FreeCString(ptr) };
    s
}

// Use the macro to implement the trait
crate::implement_category_event_handlers!(FSWatcherEvents, FSWatcherEvent, FSWatcherEventData,
    Changed => fswatcher_change, EventType::FSWATCHER
);
