//! Miscellaneous system utility functions.
//!
//! This module provides access to various system-level functions that don't
//! belong to any specific widget or component.

use crate::geometry::Point;
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

/// Opens the given file or document in its default application, e.g. a PDF in the
/// system's PDF viewer or a spreadsheet in Excel.
///
/// Returns `true` if the application was successfully launched, `false` otherwise.
///
/// # Arguments
/// * `path` - Path to the file or document to open.
///
/// # Example
/// ```rust,no_run
/// use wxdragon::utils::launch_default_application;
///
/// if launch_default_application("report.pdf") {
///     println!("Default application launched successfully");
/// }
/// ```
pub fn launch_default_application(path: &str) -> bool {
    let c_path = match CString::new(path) {
        Ok(s) => s,
        Err(_) => return false,
    };
    unsafe { ffi::wxd_LaunchDefaultApplication(c_path.as_ptr(), 0) }
}

/// Returns the current global position of the mouse pointer, in screen coordinates.
///
/// # Example
/// ```rust,no_run
/// use wxdragon::utils::get_mouse_position;
///
/// let pos = get_mouse_position();
/// println!("Mouse is at ({}, {})", pos.x, pos.y);
/// ```
pub fn get_mouse_position() -> Point {
    unsafe { ffi::wxd_GetMousePosition() }.into()
}

/// Returns `true` if the given key is currently pressed down.
///
/// Unlike key events, this can be called at any time - not just from within an event
/// handler - and can check the state of any key, not just the one that triggered the
/// current event.
///
/// # Arguments
/// * `keycode` - The key code to check, either a [`crate::keycode`] `WXK_*` constant or
///   the ASCII/Unicode code point of a printable character.
///
/// # Example
/// ```rust,no_run
/// use wxdragon::keycode::WXK_SHIFT;
/// use wxdragon::utils::get_key_state;
///
/// if get_key_state(WXK_SHIFT) {
///     println!("Shift is currently held down");
/// }
/// ```
pub fn get_key_state(keycode: i32) -> bool {
    unsafe { ffi::wxd_GetKeyState(keycode) }
}
