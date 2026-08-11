//! Access to standard, platform-appropriate filesystem locations (config, data, documents, etc).

use std::ffi::CStr;
use wxdragon_sys as ffi;

/// Provides access to standard, platform-appropriate filesystem locations such as the
/// user's config directory, data directory, and documents folder.
pub struct StandardPaths;

type GetterFn = unsafe extern "C" fn(*mut std::os::raw::c_char, usize) -> i32;

fn call_getter(getter: GetterFn) -> String {
    let len = unsafe { getter(std::ptr::null_mut(), 0) };
    if len <= 0 {
        return String::new();
    }
    let mut buffer = vec![0u8; len as usize + 1];
    unsafe { getter(buffer.as_mut_ptr() as *mut std::os::raw::c_char, buffer.len()) };
    unsafe { CStr::from_ptr(buffer.as_ptr() as *const std::os::raw::c_char).to_string_lossy().to_string() }
}

impl StandardPaths {
    /// Returns the full path to the running executable.
    pub fn executable_path() -> String {
        call_getter(ffi::wxd_StandardPaths_GetExecutablePath)
    }

    /// Returns the directory for application-wide read-only config files.
    pub fn config_dir() -> String {
        call_getter(ffi::wxd_StandardPaths_GetConfigDir)
    }

    /// Returns the directory for this user's config files for the application.
    pub fn user_config_dir() -> String {
        call_getter(ffi::wxd_StandardPaths_GetUserConfigDir)
    }

    /// Returns the directory for application-wide read-only data files.
    pub fn data_dir() -> String {
        call_getter(ffi::wxd_StandardPaths_GetDataDir)
    }

    /// Returns the directory for this user's data files for the application
    /// (the right place to store things like a database or cache).
    pub fn user_data_dir() -> String {
        call_getter(ffi::wxd_StandardPaths_GetUserDataDir)
    }

    /// Returns the directory for this user's local (non-roaming) data files for the
    /// application. Same as [`user_data_dir`](Self::user_data_dir) except on Windows.
    pub fn user_local_data_dir() -> String {
        call_getter(ffi::wxd_StandardPaths_GetUserLocalDataDir)
    }

    /// Returns the directory containing the current user's documents.
    pub fn documents_dir() -> String {
        call_getter(ffi::wxd_StandardPaths_GetDocumentsDir)
    }

    /// Returns a directory suitable for storing temporary files.
    pub fn temp_dir() -> String {
        call_getter(ffi::wxd_StandardPaths_GetTempDir)
    }
}
