//! Miscellaneous system utility functions.
//!
//! This module provides access to various system-level functions that don't
//! belong to any specific widget or component.

use std::ffi::CString;
use wxdragon_sys as ffi;

/// Produces an audible beep sound using the system's default beep.
///
/// # Example
/// ```rust,no_run
/// use wxdragon::utils::bell;
///
/// // Play a system beep
/// bell();
/// ```
pub fn bell() {
    unsafe { ffi::wxd_Bell() }
}

/// Flags for `launch_default_browser` function.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BrowserLaunchFlags {
    /// Default behavior - open in new window if possible.
    #[default]
    Default = 0,
    /// Open URL in a new browser window, if possible.
    NewWindow = 0x0001,
}

/// Opens the given URL in the default browser.
///
/// Returns `true` if the browser was successfully launched, `false` otherwise.
///
/// # Arguments
/// * `url` - The URL to open (can be a web address or a file:// URL).
///
/// # Example
/// ```rust,no_run
/// use wxdragon::utils::{launch_default_browser, BrowserLaunchFlags};
///
/// // Open a web page
/// if launch_default_browser("https://www.example.com", BrowserLaunchFlags::Default) {
///     println!("Browser launched successfully");
/// }
///
/// // Open in a new window (if supported by the browser)
/// launch_default_browser("https://www.rust-lang.org", BrowserLaunchFlags::NewWindow);
/// ```
pub fn launch_default_browser(url: &str, flags: BrowserLaunchFlags) -> bool {
    let c_url = match CString::new(url) {
        Ok(s) => s,
        Err(_) => return false,
    };
    unsafe { ffi::wxd_LaunchDefaultBrowser(c_url.as_ptr(), flags as i32) }
}
