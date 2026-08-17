//! File system watcher module for wxDragon.
//!
//! This module provides a safe wrapper around wxWidgets' `wxFileSystemWatcher`,
//! which watches files and directories for changes using the native mechanism
//! for each platform (FSEvents on macOS, inotify on Linux, ReadDirectoryChangesW
//! on Windows).

use crate::event::{FSWatcherEventKind, WxEvtHandler};
use std::ffi::CString;
use wxdragon_sys as ffi;

/// Watches files and directories for changes, delivering events for creation,
/// deletion, renames, modification, and (where the platform supports it)
/// access/attribute changes.
///
/// Bind a handler directly on the watcher with
/// [`on_fswatcher_change`](crate::event::FSWatcherEvents::on_fswatcher_change);
/// there is no separate "owner" to configure, unlike [`Timer`](crate::timer::Timer).
///
/// # Example
///
/// ```rust,no_run
/// use wxdragon::prelude::*;
/// use wxdragon::fswatcher::FileSystemWatcher;
/// use wxdragon::event::FSWatcherEventKind;
///
/// let watcher = FileSystemWatcher::new();
/// watcher.add("/path/to/watch", FSWatcherEventKind::ALL);
/// watcher.on_fswatcher_change(|event| {
///     println!("{:?}: {}", event.get_change_type(), event.get_path());
/// });
/// ```
pub struct FileSystemWatcher {
    ptr: *mut ffi::wxd_FileSystemWatcher_t,
}

impl FileSystemWatcher {
    /// Creates a new, empty file system watcher. Add paths to watch with
    /// [`add`](Self::add) or [`add_tree`](Self::add_tree).
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_FileSystemWatcher_Create() };
        Self { ptr }
    }

    /// Starts watching a single file, or a directory non-recursively (only
    /// direct children are reported, not files within subdirectories).
    /// Returns false if the path could not be watched.
    pub fn add(&self, path: &str, events: FSWatcherEventKind) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let Ok(c_path) = CString::new(path) else {
            return false;
        };
        unsafe { ffi::wxd_FileSystemWatcher_Add(self.ptr, c_path.as_ptr(), events.bits()) }
    }

    /// Starts watching a directory tree recursively.
    /// Returns false if the path could not be watched.
    pub fn add_tree(&self, path: &str, events: FSWatcherEventKind) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let Ok(c_path) = CString::new(path) else {
            return false;
        };
        unsafe { ffi::wxd_FileSystemWatcher_AddTree(self.ptr, c_path.as_ptr(), events.bits()) }
    }

    /// Stops watching a path previously passed to [`add`](Self::add).
    pub fn remove(&self, path: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let Ok(c_path) = CString::new(path) else {
            return false;
        };
        unsafe { ffi::wxd_FileSystemWatcher_Remove(self.ptr, c_path.as_ptr()) }
    }

    /// Stops watching a directory tree previously passed to [`add_tree`](Self::add_tree).
    pub fn remove_tree(&self, path: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let Ok(c_path) = CString::new(path) else {
            return false;
        };
        unsafe { ffi::wxd_FileSystemWatcher_RemoveTree(self.ptr, c_path.as_ptr()) }
    }

    /// Stops watching every path currently being watched.
    pub fn remove_all(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_FileSystemWatcher_RemoveAll(self.ptr) }
    }
}

impl Default for FileSystemWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FileSystemWatcher {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_FileSystemWatcher_Destroy(self.ptr) };
            self.ptr = std::ptr::null_mut();
        }
    }
}

// wxFileSystemWatcherBase derives from wxEvtHandler and delivers events to
// itself unless SetOwner() is called, so binding directly on the watcher works.
impl WxEvtHandler for FileSystemWatcher {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

impl crate::event::FSWatcherEvents for FileSystemWatcher {}
