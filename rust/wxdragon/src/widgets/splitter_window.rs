//!
//! Safe wrapper for wxSplitterWindow.

use crate::event::{Event, EventType, WindowEvents, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::os::raw::c_int;
use wxdragon_sys as ffi;

/// Represents a wxSplitterWindow widget.
///
/// SplitterWindow uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let splitter = SplitterWindow::builder(&frame).build();
///
/// // SplitterWindow is Copy - no clone needed for closures!
/// splitter.split_vertically(&left_panel, &right_panel, 200);
///
/// // After parent destruction, splitter operations are safe no-ops
/// frame.destroy();
/// assert!(!splitter.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct SplitterWindow {
    /// Safe handle to the underlying wxSplitterWindow - automatically invalidated on destroy
    handle: WindowHandle,
}

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
        SplitterWindow {
            handle: WindowHandle::new(splitter_ptr as *mut ffi::wxd_Window_t),
        }
    }
);

impl SplitterWindow {
    /// Creates a new SplitterWindow builder.
    pub fn builder<W: WxWidget>(parent: &W) -> SplitterWindowBuilder<'_> {
        SplitterWindowBuilder::new(parent)
    }

    /// Helper to get raw splitter pointer, returns null if widget has been destroyed
    #[inline]
    fn splitter_ptr(&self) -> *mut ffi::wxd_SplitterWindow_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_SplitterWindow_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Initializes the splitter to contain the given window.
    /// Should be called after creation if the splitter is not split initially.
    /// No-op if the splitter has been destroyed.
    pub fn initialize<W: WxWidget>(&self, window: &W) {
        let ptr = self.splitter_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SplitterWindow_Initialize(ptr, window.handle_ptr()) };
    }

    /// Splits the window vertically, putting `window1` on the left and `window2` on the right.
    ///
    /// # Arguments
    /// * `window1` - The window for the left pane.
    /// * `window2` - The window for the right pane.
    /// * `sash_position` - The initial position of the sash. If 0 or negative, a default position is used.
    ///
    /// Returns `true` on success, `false` if the splitter has been destroyed.
    pub fn split_vertically<W1: WxWidget, W2: WxWidget>(&self, window1: &W1, window2: &W2, sash_position: i32) -> bool {
        let ptr = self.splitter_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe {
            ffi::wxd_SplitterWindow_SplitVertically(ptr, window1.handle_ptr(), window2.handle_ptr(), sash_position as c_int)
        }
    }

    /// Splits the window horizontally, putting `window1` above `window2`.
    ///
    /// # Arguments
    /// * `window1` - The window for the top pane.
    /// * `window2` - The window for the bottom pane.
    /// * `sash_position` - The initial position of the sash. If 0 or negative, a default position is used.
    ///
    /// Returns `true` on success, `false` if the splitter has been destroyed.
    pub fn split_horizontally<W1: WxWidget, W2: WxWidget>(&self, window1: &W1, window2: &W2, sash_position: i32) -> bool {
        let ptr = self.splitter_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe {
            ffi::wxd_SplitterWindow_SplitHorizontally(ptr, window1.handle_ptr(), window2.handle_ptr(), sash_position as c_int)
        }
    }

    /// Unsplits the window.
    ///
    /// # Arguments
    /// * `to_remove` - Optional window to remove. If `None`, the second (right/bottom) window is removed.
    ///
    /// Returns `true` on success, `false` if the splitter has been destroyed.
    pub fn unsplit<W: WxWidget>(&self, to_remove: Option<&W>) -> bool {
        let ptr = self.splitter_ptr();
        if ptr.is_null() {
            return false;
        }
        let remove_ptr = to_remove.map_or(std::ptr::null_mut(), |w| w.handle_ptr());
        unsafe { ffi::wxd_SplitterWindow_Unsplit(ptr, remove_ptr) }
    }

    /// Sets the sash position.
    /// No-op if the splitter has been destroyed.
    pub fn set_sash_position(&self, position: i32, redraw: bool) {
        let ptr = self.splitter_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SplitterWindow_SetSashPosition(ptr, position as c_int, redraw) };
    }

    /// Gets the current sash position.
    /// Returns 0 if the splitter has been destroyed.
    pub fn sash_position(&self) -> i32 {
        let ptr = self.splitter_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_SplitterWindow_GetSashPosition(ptr) }
    }

    /// Sets the minimum pane size (applies to both panes).
    /// No-op if the splitter has been destroyed.
    pub fn set_minimum_pane_size(&self, size: i32) {
        let ptr = self.splitter_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_SplitterWindow_SetMinimumPaneSize(ptr, size as c_int) };
    }

    /// Returns the underlying WindowHandle for this splitter window.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for SplitterWindow (using WindowHandle)
impl WxWidget for SplitterWindow {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for SplitterWindow {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
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

// XRC Support - enables SplitterWindow to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for SplitterWindow {
    unsafe fn from_xrc_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
        SplitterWindow {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for SplitterWindow
impl crate::window::FromWindowWithClassName for SplitterWindow {
    fn class_name() -> &'static str {
        "wxSplitterWindow"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        SplitterWindow {
            handle: WindowHandle::new(ptr),
        }
    }
}
