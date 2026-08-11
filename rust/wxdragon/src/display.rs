//! Access to information about connected displays/monitors.
//!
//! Wraps `wxDisplay`, letting an app enumerate monitors and query their geometry,
//! work area, DPI/scale factor, and name — necessary for placing windows correctly
//! on multi-monitor setups where displays can differ in resolution, position, or scaling.
//!
//! # Example
//!
//! ```no_run
//! use wxdragon::display::Display;
//!
//! for display in Display::all() {
//!     println!(
//!         "{}: {:?} @ {}x scale",
//!         display.name(),
//!         display.geometry(),
//!         display.scale_factor()
//!     );
//! }
//! ```

use crate::geometry::{Point, Rect, Size};
use crate::window::WxWidget;
use std::ffi::CStr;
use wxdragon_sys as ffi;

/// Represents one connected display/monitor.
///
/// Displays are identified by index, from `0` up to (but not including) [`Display::count`].
/// A `Display` is a lightweight handle to that index; it does not hold any native resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Display {
    index: u32,
}

impl Display {
    /// Returns the number of currently connected displays.
    pub fn count() -> u32 {
        unsafe { ffi::wxd_Display_GetCount() }
    }

    /// Returns an iterator over all currently connected displays, in index order.
    pub fn all() -> impl Iterator<Item = Display> {
        (0..Self::count()).map(|index| Display { index })
    }

    /// Returns the display at the given index, or `None` if the index is out of range.
    pub fn new(index: u32) -> Option<Self> {
        let display = Display { index };
        if display.is_ok() { Some(display) } else { None }
    }

    /// Returns the display containing the given point, or `None` if it doesn't belong to any display.
    pub fn from_point(point: Point) -> Option<Self> {
        let index = unsafe { ffi::wxd_Display_GetFromPoint(point.into()) };
        if index < 0 {
            None
        } else {
            Some(Display { index: index as u32 })
        }
    }

    /// Returns the display with the biggest intersection with the given rectangle,
    /// or `None` if the rectangle doesn't intersect any display.
    pub fn from_rect(rect: Rect) -> Option<Self> {
        let index = unsafe { ffi::wxd_Display_GetFromRect(rect.into()) };
        if index < 0 {
            None
        } else {
            Some(Display { index: index as u32 })
        }
    }

    /// Returns the display containing the given window, or `None` if the window isn't shown.
    pub fn from_window(window: &dyn WxWidget) -> Option<Self> {
        let index = unsafe { ffi::wxd_Display_GetFromWindow(window.handle_ptr()) };
        if index < 0 {
            None
        } else {
            Some(Display { index: index as u32 })
        }
    }

    /// Returns this display's index.
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Returns `true` if this display is currently connected.
    pub fn is_ok(&self) -> bool {
        unsafe { ffi::wxd_Display_IsOk(self.index) }
    }

    /// Returns the display's full geometry (position and size) in screen coordinates.
    pub fn geometry(&self) -> Rect {
        unsafe { ffi::wxd_Display_GetGeometry(self.index) }.into()
    }

    /// Returns the display's usable client area, excluding taskbars, docks, and similar chrome.
    pub fn client_area(&self) -> Rect {
        unsafe { ffi::wxd_Display_GetClientArea(self.index) }.into()
    }

    /// Returns the display's colour depth in bits per pixel, or 0 if unknown.
    pub fn depth(&self) -> i32 {
        unsafe { ffi::wxd_Display_GetDepth(self.index) }
    }

    /// Returns the display's resolution in pixels per inch.
    pub fn ppi(&self) -> Size {
        unsafe { ffi::wxd_Display_GetPPI(self.index) }.into()
    }

    /// Returns the display's content scale factor (e.g. `2.0` for a 200%-scaled display).
    pub fn scale_factor(&self) -> f64 {
        unsafe { ffi::wxd_Display_GetScaleFactor(self.index) }
    }

    /// Returns the display's name, which may be empty if the platform doesn't provide one.
    pub fn name(&self) -> String {
        let len = unsafe { ffi::wxd_Display_GetName(self.index, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buffer = vec![0u8; len as usize + 1];
        unsafe { ffi::wxd_Display_GetName(self.index, buffer.as_mut_ptr() as *mut std::os::raw::c_char, buffer.len()) };
        unsafe {
            CStr::from_ptr(buffer.as_ptr() as *const std::os::raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Returns `true` if this is the primary display (usually, but not always, index 0).
    pub fn is_primary(&self) -> bool {
        unsafe { ffi::wxd_Display_IsPrimary(self.index) }
    }

    /// Clears any cached display information. Call this after receiving a display
    /// connected/disconnected notification from the OS.
    pub fn invalidate_cache() {
        unsafe { ffi::wxd_Display_InvalidateCache() };
    }
}
