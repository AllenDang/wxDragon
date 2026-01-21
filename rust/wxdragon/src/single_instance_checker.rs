//! Single instance checker to ensure only one copy of the application runs at a time.
//!
//! This module provides access to wxWidgets' wxSingleInstanceChecker class,
//! which can be used to prevent multiple instances of an application from running.
//!
//! # Example
//! ```rust,no_run
//! use wxdragon::prelude::*;
//!
//! // In your app initialization:
//! let checker = SingleInstanceChecker::new("MyApp", None);
//! if let Some(checker) = checker {
//!     if checker.is_another_running() {
//!         // Another instance is already running
//!         eprintln!("Another instance is already running!");
//!         return;
//!     }
//!     // Keep the checker alive for the lifetime of the application
//! }
//! ```

use std::ffi::CString;
use std::marker::PhantomData;
use wxdragon_sys as ffi;

/// A single instance checker that ensures only one copy of an application runs at a time.
///
/// `SingleInstanceChecker` wraps wxWidgets' wxSingleInstanceChecker class.
/// As long as an instance of this struct exists, other instances of the application
/// can detect that this instance is running via `is_another_running()`.
///
/// # Platform Notes
/// - On Windows: Uses a named mutex
/// - On Unix: Uses a lock file in the specified path (or home directory by default)
///
/// # Example
/// ```rust,no_run
/// use wxdragon::prelude::*;
///
/// // Create with explicit name
/// if let Some(checker) = SingleInstanceChecker::new("MyUniqueAppName", None) {
///     if checker.is_another_running() {
///         eprintln!("Application is already running!");
///         std::process::exit(1);
///     }
///     // Application continues...
/// }
/// ```
pub struct SingleInstanceChecker {
    ptr: *mut ffi::wxd_SingleInstanceChecker_t,
    // Marker to make this type !Send and !Sync since wxWidgets is not thread-safe
    _marker: PhantomData<*const ()>,
}

impl SingleInstanceChecker {
    /// Create a new single instance checker with the given name.
    ///
    /// # Arguments
    /// * `name` - A unique name for this application instance. This is used as the mutex name
    ///   on Windows or the lock file name on Unix. Should be unique to avoid conflicts with
    ///   other applications.
    /// * `path` - Optional path for the lock file directory (Unix only). If `None`, uses the
    ///   user's home directory. This parameter is ignored on Windows.
    ///
    /// # Returns
    /// Returns `Some(SingleInstanceChecker)` if creation succeeded, `None` if it failed.
    /// Failure doesn't mean another instance is running - use `is_another_running()` for that.
    ///
    /// # Note
    /// One possible reason for failure on Unix is that the lock file already exists but was
    /// not created by the current user. Applications should handle this gracefully rather
    /// than treating it as a fatal error, as it could be exploited for denial of service.
    pub fn new(name: &str, path: Option<&str>) -> Option<Self> {
        let c_name = CString::new(name).ok()?;
        let c_path = path.and_then(|p| CString::new(p).ok());

        let ptr = unsafe {
            ffi::wxd_SingleInstanceChecker_Create(c_name.as_ptr(), c_path.as_ref().map_or(std::ptr::null(), |s| s.as_ptr()))
        };

        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr,
                _marker: PhantomData,
            })
        }
    }

    /// Create a new single instance checker with the default name.
    ///
    /// The default name is a combination of the application name and user ID,
    /// which allows different users to run the application concurrently.
    ///
    /// # Returns
    /// Returns `Some(SingleInstanceChecker)` if creation succeeded, `None` if it failed.
    ///
    /// # Note
    /// This method requires that the wxApp has already been created, as it uses
    /// `wxApp::GetAppName()` to construct the default name.
    pub fn new_default() -> Option<Self> {
        let ptr = unsafe { ffi::wxd_SingleInstanceChecker_CreateDefault() };

        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr,
                _marker: PhantomData,
            })
        }
    }

    /// Check if another instance of the program is already running.
    ///
    /// # Returns
    /// Returns `true` if another copy of this program is already running,
    /// `false` otherwise.
    pub fn is_another_running(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_SingleInstanceChecker_IsAnotherRunning(self.ptr) }
    }
}

impl Drop for SingleInstanceChecker {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_SingleInstanceChecker_Destroy(self.ptr) };
        }
    }
}
