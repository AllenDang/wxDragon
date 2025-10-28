//! TaskBarIcon event types and data structures
//!
//! **Note**: This module is only available on Windows and Linux platforms where TaskBarIcon events are supported.
//!
//! ## Platform Support
//!
//! TaskBarIcon events have different levels of support across platforms:
//!
//! - **Windows**: Full support for all events including mouse movements, clicks, double-clicks, and balloon tooltips
//! - **Linux**: Limited support - only left down and left double-click events are available
//! - **macOS**: ❌ Not supported - macOS uses menu-based interaction only
//!
//! ## Usage
//!
//! For maximum cross-platform compatibility, use the `popup_menu()` method to show context menus
//! rather than relying on mouse events. The basic events (left down, left double-click) are
//! available on Windows and Linux, while Windows-specific events are only available when
//! compiling for Windows.
//!
//! ## Example
//!
//! ```rust,no_run
//! use wxdragon::prelude::*;
//!
//! let taskbar = TaskBarIcon::builder().build();
//!
//! // Cross-platform events (Windows and Linux)
//! taskbar.on_left_down(|_| {
//!     println!("Left click on taskbar icon");
//! });
//!
//! // Windows-only events (conditional compilation)
//! #[cfg(target_os = "windows")]
//! taskbar.on_balloon_click(|_| {
//!     println!("Balloon tooltip clicked");
//! });
//! ```
//!
//! # Platform Support
//!
//! TaskBarIcon events have different levels of support across platforms:
//!
//! ## Windows
//! - Full support for all mouse events (move, up/down, double-click)
//! - Balloon tooltip events (timeout, click)
//!
//! ## Linux/GTK
//! - Limited support: left down, left double-click, popup menu
//! - No mouse up/move events, no balloon events
//!
//! ## macOS
//! - No mouse events (uses menu-based approach)
//! - Events may not fire on macOS - use `CreatePopupMenu` instead
//!
//! For maximum compatibility, use `CreatePopupMenu` for menu handling rather than relying on mouse events.

#![cfg(any(target_os = "windows", target_os = "linux"))]

use crate::event::Event;
use crate::event::event_data::MouseEventData;
use crate::geometry::Point;

/// Event types specific to TaskBarIcon widgets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskBarIconEvent {
    /// Left mouse button pressed on the taskbar icon (Windows, Linux)
    LeftDown,
    /// Left mouse button double-clicked on the taskbar icon (Windows, Linux)
    LeftDoubleClick,

    // Windows-only events
    #[cfg(target_os = "windows")]
    /// Mouse movement over the taskbar icon (Windows only)
    Move,
    #[cfg(target_os = "windows")]
    /// Left mouse button released on the taskbar icon (Windows only)
    LeftUp,
    #[cfg(target_os = "windows")]
    /// Right mouse button pressed on the taskbar icon (Windows only)
    RightDown,
    #[cfg(target_os = "windows")]
    /// Right mouse button released on the taskbar icon (Windows only)
    RightUp,
    #[cfg(target_os = "windows")]
    /// Right mouse button double-clicked on the taskbar icon (Windows only)
    RightDoubleClick,
    #[cfg(target_os = "windows")]
    /// Balloon tooltip timeout (Windows only)
    BalloonTimeout,
    #[cfg(target_os = "windows")]
    /// Balloon tooltip clicked (Windows only)
    BalloonClick,
}

// Event conversion methods are handled by the implement_widget_local_event_handlers macro

/// Data associated with TaskBarIcon events
#[derive(Debug)]
pub struct TaskBarIconEventData {
    pub event: MouseEventData,
}

impl TaskBarIconEventData {
    pub fn new(event: Event) -> Self {
        Self {
            event: MouseEventData::new(event),
        }
    }

    /// Create from a raw Event if it's a taskbar event
    pub fn from_event(event: Event) -> Option<Self> {
        // The macro handles event type validation, so we can create the event data directly
        Some(Self::new(event))
    }

    /// Get the mouse position for the event
    pub fn get_position(&self) -> Option<Point> {
        self.event.get_position()
    }
}
