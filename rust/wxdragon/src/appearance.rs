//! Application appearance and dark mode support.
//!
//! This module provides support for wxWidgets 3.3.0+ appearance features, including
//! dark mode detection and control. The functionality requires wxWidgets 3.3.0 or later.
//!
//! # Example
//!
//! ```no_run
//! use wxdragon::prelude::*;
//! use wxdragon::appearance::*;
//!
//! wxdragon::main(|_| {
//!     // Enable system appearance following (including dark mode on Windows)
//!     if let Some(app) = wxdragon::app::get_app() {
//!         app.set_appearance(Appearance::System);
//!     }
//!
//!     // Check if the system is using dark mode
//!     if let Some(system_appearance) = get_system_appearance() {
//!         if system_appearance.is_dark() {
//!             println!("System is using dark mode");
//!         }
//!     }
//!
//!     let frame = Frame::builder()
//!         .with_title("Dark Mode Demo")
//!         .build();
//!     frame.show(true);
//! });
//! ```

use std::ffi::CStr;
use wxdragon_sys as ffi;

// Use FFI-generated constants (following the cursor pattern)

/// Application appearance modes for dark mode support.
///
/// This enum controls how the application handles dark/light theming.
/// Requires wxWidgets 3.3.0 or later for full functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum Appearance {
    /// Force light mode appearance regardless of system settings.
    Light,
    /// Force dark mode appearance regardless of system settings.
    Dark,
    /// Follow the system appearance settings (recommended).
    /// This enables dark mode on Windows when the system is using dark theme.
    System,
}

/// Result of setting the application appearance.
///
/// Returned by `SetAppearance` to indicate the outcome of the operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum AppearanceResult {
    /// The appearance was set successfully.
    Ok,
    /// Failed to set the appearance (e.g., not supported on this platform).
    Failure,
    /// Cannot change the appearance at this time (e.g., windows already exist).
    CannotChange,
}

impl From<Appearance> for ffi::wxd_Appearance {
    fn from(appearance: Appearance) -> Self {
        match appearance {
            Appearance::Light => ffi::wxd_Appearance_WXD_APPEARANCE_LIGHT,
            Appearance::Dark => ffi::wxd_Appearance_WXD_APPEARANCE_DARK,
            Appearance::System => ffi::wxd_Appearance_WXD_APPEARANCE_SYSTEM,
        }
    }
}

impl From<ffi::wxd_Appearance> for Appearance {
    fn from(appearance: ffi::wxd_Appearance) -> Self {
        match appearance {
            ffi::wxd_Appearance_WXD_APPEARANCE_LIGHT => Appearance::Light,
            ffi::wxd_Appearance_WXD_APPEARANCE_DARK => Appearance::Dark,
            ffi::wxd_Appearance_WXD_APPEARANCE_SYSTEM => Appearance::System,
            _ => Appearance::System, // Default fallback
        }
    }
}

impl From<ffi::wxd_AppearanceResult> for AppearanceResult {
    fn from(result: ffi::wxd_AppearanceResult) -> Self {
        match result {
            ffi::wxd_AppearanceResult_WXD_APPEARANCE_RESULT_OK => AppearanceResult::Ok,
            ffi::wxd_AppearanceResult_WXD_APPEARANCE_RESULT_FAILURE => AppearanceResult::Failure,
            ffi::wxd_AppearanceResult_WXD_APPEARANCE_RESULT_CANNOT_CHANGE => AppearanceResult::CannotChange,
            _ => AppearanceResult::Failure, // Default fallback
        }
    }
}

/// Information about the current system appearance.
///
/// This provides methods to query the system's current theme preferences,
/// particularly useful for determining if dark mode is active.
pub struct SystemAppearance {
    ptr: *mut ffi::wxd_SystemAppearance_t,
}

impl SystemAppearance {
    /// Creates a SystemAppearance from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a wxd_SystemAppearance_t object.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SystemAppearance_t) -> Option<Self> {
        if ptr.is_null() { None } else { Some(SystemAppearance { ptr }) }
    }

    /// Returns `true` if the system is using a dark theme.
    ///
    /// This method checks if the current system theme is explicitly recognized
    /// as being a dark theme or if the default window background is dark.
    ///
    /// # Returns
    /// `true` if dark mode is active, `false` otherwise.
    pub fn is_dark(&self) -> bool {
        unsafe { ffi::wxd_SystemAppearance_IsDark(self.ptr) }
    }

    /// Returns `true` if the default window background is significantly darker than the foreground.
    ///
    /// This is used internally by `is_dark()` if there is no platform-specific way to
    /// determine whether a dark mode is being used. Generally not useful to call directly.
    ///
    /// # Returns
    /// `true` if the background is darker than the foreground, `false` otherwise.
    pub fn is_using_dark_background(&self) -> bool {
        unsafe { ffi::wxd_SystemAppearance_IsUsingDarkBackground(self.ptr) }
    }

    /// Returns the system appearance name if available.
    ///
    /// This is currently only implemented for macOS and returns a not necessarily
    /// user-readable string such as "NSAppearanceNameAqua" there and an empty
    /// string under all other platforms.
    ///
    /// # Returns
    /// The appearance name as a `String`, or an empty string if not available.
    pub fn get_name(&self) -> String {
        let len = unsafe { ffi::wxd_SystemAppearance_GetName(self.ptr, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut b = vec![0; len as usize + 1];
        unsafe { ffi::wxd_SystemAppearance_GetName(self.ptr, b.as_mut_ptr(), b.len()) };
        unsafe { CStr::from_ptr(b.as_ptr()).to_string_lossy().to_string() }
    }
}

impl Drop for SystemAppearance {
    fn drop(&mut self) {
        unsafe {
            ffi::wxd_SystemAppearance_Destroy(self.ptr);
        }
    }
}

// Make SystemAppearance thread-safe for use across threads
unsafe impl Send for SystemAppearance {}
unsafe impl Sync for SystemAppearance {}

/// Gets the current system appearance information.
///
/// This function returns information about the current system appearance,
/// which can be used to determine if the system is using dark mode.
///
/// # Returns
/// `Some(SystemAppearance)` if the system appearance can be determined,
/// `None` if the feature is not supported (requires wxWidgets 3.3.0+).
///
/// # Example
/// ```no_run
/// use wxdragon::appearance::*;
///
/// if let Some(appearance) = get_system_appearance() {
///     if appearance.is_dark() {
///         println!("System is in dark mode");
///         println!("Appearance name: {}", appearance.get_name());
///     } else {
///         println!("System is in light mode");
///     }
/// } else {
///     println!("System appearance detection not available");
/// }
/// ```
pub fn get_system_appearance() -> Option<SystemAppearance> {
    unsafe {
        let ptr = ffi::wxd_SystemSettings_GetAppearance();
        SystemAppearance::from_ptr(ptr)
    }
}

/// Checks if the system is currently using dark mode.
///
/// This is a convenience function that gets the system appearance and
/// checks if it's dark.
///
/// # Returns
/// `true` if the system is using dark mode, `false` otherwise or if
/// the system appearance cannot be determined.
///
/// # Example
/// ```no_run
/// use wxdragon::appearance::*;
///
/// if is_system_dark_mode() {
///     println!("System is using dark mode");
/// } else {
///     println!("System is using light mode or detection unavailable");
/// }
/// ```
pub fn is_system_dark_mode() -> bool {
    get_system_appearance()
        .map(|appearance| appearance.is_dark())
        .unwrap_or(false)
}

/// Extension trait for App to add appearance support.
pub trait AppAppearance {
    /// Sets the application appearance mode.
    ///
    /// This method controls how the application handles dark/light theming.
    /// On Windows, calling this with `Appearance::System` enables dark mode
    /// support when the system is using a dark theme.
    ///
    /// Note: This requires wxWidgets 3.3.0 or later. On older versions,
    /// this method will return `AppearanceResult::Failure`.
    ///
    /// # Arguments
    /// * `appearance` - The appearance mode to set
    ///
    /// # Returns
    /// * `AppearanceResult::Ok` - The appearance was set successfully
    /// * `AppearanceResult::Failure` - Failed to set appearance (not supported)
    /// * `AppearanceResult::CannotChange` - Cannot change at this time (windows exist)
    ///
    /// # Example
    /// ```no_run
    /// use wxdragon::prelude::*;
    /// use wxdragon::appearance::*;
    ///
    /// if let Some(app) = wxdragon::app::get_app() {
    ///     match app.set_appearance(Appearance::System) {
    ///         AppearanceResult::Ok => println!("Dark mode support enabled"),
    ///         AppearanceResult::Failure => println!("Dark mode not supported"),
    ///         AppearanceResult::CannotChange => println!("Cannot change appearance now"),
    ///     }
    /// }
    /// ```
    fn set_appearance(&self, appearance: Appearance) -> AppearanceResult;
}

/// App wrapper for appearance functionality.
pub struct App {
    ptr: *mut ffi::wxd_App_t,
}

impl App {
    /// Creates an App wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a wxd_App_t object.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_App_t) -> Option<Self> {
        if ptr.is_null() { None } else { Some(App { ptr }) }
    }
}

impl AppAppearance for App {
    fn set_appearance(&self, appearance: Appearance) -> AppearanceResult {
        unsafe {
            let result = ffi::wxd_App_SetAppearance(self.ptr, appearance.into());
            result.into()
        }
    }
}

/// Gets the current application instance for appearance operations.
///
/// # Returns
/// `Some(App)` if an application instance exists, `None` otherwise.
///
/// # Example
/// ```no_run
/// use wxdragon::appearance::*;
///
/// if let Some(app) = get_app() {
///     app.set_appearance(Appearance::System);
/// }
/// ```
pub fn get_app() -> Option<App> {
    unsafe {
        let ptr = ffi::wxd_GetApp();
        App::from_ptr(ptr)
    }
}
