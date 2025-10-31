//!
//! Safe wrapper for wxSplitterWindow.

use crate::event::{Event, EventType, WindowEvents, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::WxWidget;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

/// Represents a wxSplitterWindow widget.
#[derive(Clone)]
pub struct SplitterWindow(pub(crate) *mut ffi::wxd_SplitterWindow_t);

widget_style_enum!(
    name: SplitterWindowStyle,
    doc: "Style flags for the SplitterWindow widget.",
    variants: {
        Default: ffi::WXD_SP_BORDER, "Default style with a border.",
        Horizontal: ffi::WXD_SP_HORIZONTAL, "Horizontal split mode (one pane above the other).",
        Vertical: ffi::WXD_SP_VERTICAL, "Vertical split mode (one pane beside the other).",
        PermitUnsplit: ffi::WXD_SP_PERMIT_UNSPLIT, "Always allow unsplitting, even with no minimum pane size.",
        LiveUpdate: ffi::WXD_SP_LIVE_UPDATE, "Redraw window as the sash is moved, rather than just display a line.",
        ThinSash: ffi::WXD_SP_THIN_SASH, "Use a thin sash."
    },
    default_variant: Default
);

/// Events emitted by SplitterWindow
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitterEvent {
    /// Emitted when sash position has been changed
    SashPositionChanged,
    /// Emitted while the sash is being dragged
    SashPositionChanging,
    /// Emitted when the splitter is double-clicked
    DoubleClicked,
    /// Emitted when the splitter is unsplit
    Unsplit,
}

/// Event data for a SplitterWindow event
#[derive(Debug)]
pub struct SplitterEventData {
    event: Event,
}

impl SplitterEventData {
    /// Create a new SplitterEventData from a generic Event
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

    /// Get the sash position
    pub fn get_sash_position(&self) -> Option<i32> {
        if self.event.is_null() {
            return None;
        }
        Some(unsafe { ffi::wxd_SplitterEvent_GetSashPosition(self.event.0) })
    }
}

widget_builder!(
    name: SplitterWindow,
    parent_type: &'a dyn WxWidget,
    style_type: SplitterWindowStyle,
    fields: {
    },
    build_impl: |slf| {
        let splitter_ptr = unsafe {
            ffi::wxd_SplitterWindow_Create(
                slf.parent.handle_ptr(),
                slf.id as c_int,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            )
        };
        if splitter_ptr.is_null() {
            panic!("Failed to create SplitterWindow");
        }
        unsafe { SplitterWindow::from_ptr(splitter_ptr) }
    }
);

impl SplitterWindow {
    /// Creates a new SplitterWindow builder.
    pub fn builder<W: WxWidget>(parent: &W) -> SplitterWindowBuilder<'_> {
        SplitterWindowBuilder::new(parent)
    }

    // Internal constructor - Revert back to crate-public
    // SAFETY: Caller must ensure ptr is a valid wxd_SplitterWindow_t
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SplitterWindow_t) -> Self {
        SplitterWindow(ptr)
    }

    /// Initializes the splitter to contain the given window.
    /// Should be called after creation if the splitter is not split initially.
    pub fn initialize<W: WxWidget>(&self, window: &W) {
        unsafe { ffi::wxd_SplitterWindow_Initialize(self.0, window.handle_ptr()) };
    }

    /// Splits the window vertically, putting `window1` on the left and `window2` on the right.
    ///
    /// # Arguments
    /// * `window1` - The window for the left pane.
    /// * `window2` - The window for the right pane.
    /// * `sash_position` - The initial position of the sash. If 0 or negative, a default position is used.
    ///
    /// Returns `true` on success.
    pub fn split_vertically<W1: WxWidget, W2: WxWidget>(&self, window1: &W1, window2: &W2, sash_position: i32) -> bool {
        unsafe {
            ffi::wxd_SplitterWindow_SplitVertically(self.0, window1.handle_ptr(), window2.handle_ptr(), sash_position as c_int)
        }
    }

    /// Splits the window horizontally, putting `window1` above `window2`.
    ///
    /// # Arguments
    /// * `window1` - The window for the top pane.
    /// * `window2` - The window for the bottom pane.
    /// * `sash_position` - The initial position of the sash. If 0 or negative, a default position is used.
    ///
    /// Returns `true` on success.
    pub fn split_horizontally<W1: WxWidget, W2: WxWidget>(&self, window1: &W1, window2: &W2, sash_position: i32) -> bool {
        unsafe {
            ffi::wxd_SplitterWindow_SplitHorizontally(self.0, window1.handle_ptr(), window2.handle_ptr(), sash_position as c_int)
        }
    }

    /// Unspltis the window.
    ///
    /// # Arguments
    /// * `to_remove` - Optional window to remove. If `None`, the second (right/bottom) window is removed.
    ///
    /// Returns `true` on success.
    pub fn unsplit<W: WxWidget>(&self, to_remove: Option<&W>) -> bool {
        let remove_ptr = to_remove.map_or(std::ptr::null_mut(), |w| w.handle_ptr());
        unsafe { ffi::wxd_SplitterWindow_Unsplit(self.0, remove_ptr) }
    }

    /// Sets the sash position.
    pub fn set_sash_position(&self, position: i32, redraw: bool) {
        unsafe { ffi::wxd_SplitterWindow_SetSashPosition(self.0, position as c_int, redraw) };
    }

    /// Gets the current sash position.
    pub fn sash_position(&self) -> i32 {
        unsafe { ffi::wxd_SplitterWindow_GetSashPosition(self.0) }
    }

    /// Sets the minimum pane size (applies to both panes).
    pub fn set_minimum_pane_size(&self, size: i32) {
        unsafe { ffi::wxd_SplitterWindow_SetMinimumPaneSize(self.0, size as c_int) };
    }
}

// Implement the core WxWidget trait
impl WxWidget for SplitterWindow {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.0 as *mut ffi::wxd_Window_t
    }
}

// Implement the event handling trait
impl WxEvtHandler for SplitterWindow {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.0 as *mut ffi::wxd_EvtHandler_t
    }
}

// No explicit Drop implementation needed - child widget managed by parent
impl Drop for SplitterWindow {
    fn drop(&mut self) {
        // Child widgets are typically managed by their parent in wxWidgets
    }
}

// Use the implement_widget_local_event_handlers macro to implement event handling
crate::implement_widget_local_event_handlers!(
    SplitterWindow,
    SplitterEvent,
    SplitterEventData,
    SashPositionChanged => sash_position_changed, EventType::SPLITTER_SASH_POS_CHANGED,
    SashPositionChanging => sash_position_changing, EventType::SPLITTER_SASH_POS_CHANGING,
    DoubleClicked => double_clicked, EventType::SPLITTER_DOUBLECLICKED,
    Unsplit => unsplit, EventType::SPLITTER_UNSPLIT
);

// Add WindowEvents implementation
impl WindowEvents for SplitterWindow {}

// Add XRC Support - enables SplitterWindow to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for SplitterWindow {
    unsafe fn from_xrc_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
        SplitterWindow(ptr as *mut ffi::wxd_SplitterWindow_t)
    }
}

// Manual widget casting support for SplitterWindow - tuple struct needs custom handling
impl crate::window::FromWindowWithClassName for SplitterWindow {
    fn class_name() -> &'static str {
        "wxSplitterWindow"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        SplitterWindow(ptr as *mut ffi::wxd_SplitterWindow_t)
    }
}
