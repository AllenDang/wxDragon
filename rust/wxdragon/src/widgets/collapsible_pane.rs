//! Safe wrapper for wxCollapsiblePane.

use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{Window, WindowHandle, WxWidget};
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

// --- Style enum using macro ---
widget_style_enum!(
    name: CollapsiblePaneStyle,
    doc: "Window style flags for CollapsiblePane",
    variants: {
        Default: ffi::WXD_CP_DEFAULT_STYLE, "Default style.",
        NoTlwResize: ffi::WXD_CP_NO_TLW_RESIZE, "Prevents top-level window from resizing when pane expands/collapses."
    },
    default_variant: Default
);

/// Represents a wxCollapsiblePane widget.
/// A collapsible pane is a container with an embedded button-like control which can be
/// used by the user to collapse or expand the pane's content.
///
/// CollapsiblePane uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let pane = CollapsiblePane::builder(&frame).label("Details").build();
///
/// // CollapsiblePane is Copy - no clone needed for closures!
/// pane.bind_changed(move |_| {
///     // Safe: if pane was destroyed, this is a no-op
///     println!("Expanded: {}", pane.is_expanded());
/// });
///
/// // After parent destruction, pane operations are safe no-ops
/// frame.destroy();
/// assert!(!pane.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct CollapsiblePane {
    /// Safe handle to the underlying wxCollapsiblePane - automatically invalidated on destroy
    handle: WindowHandle,
}

impl CollapsiblePane {
    /// Creates a new builder for a CollapsiblePane.
    pub fn builder(parent: &dyn WxWidget) -> CollapsiblePaneBuilder<'_> {
        CollapsiblePaneBuilder::new(parent)
    }

    /// Creates a new CollapsiblePane from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Helper to get raw collapsible pane pointer, returns null if widget has been destroyed
    #[inline]
    fn collapsible_pane_ptr(&self) -> *mut ffi::wxd_CollapsiblePane_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_CollapsiblePane_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Returns the underlying WindowHandle for this collapsible pane.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    // --- CollapsiblePane-specific methods ---

    /// Returns true if the pane is currently expanded, false otherwise.
    /// Returns false if the pane has been destroyed.
    pub fn is_expanded(&self) -> bool {
        let ptr = self.collapsible_pane_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_CollapsiblePane_IsExpanded(ptr) }
    }

    /// Returns true if the pane is currently collapsed, false otherwise.
    /// Returns false if the pane has been destroyed.
    pub fn is_collapsed(&self) -> bool {
        let ptr = self.collapsible_pane_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_CollapsiblePane_IsCollapsed(ptr) }
    }

    /// Expands or collapses the pane.
    /// No-op if the pane has been destroyed.
    ///
    /// # Arguments
    /// * `expand` - If true, expands the pane; if false, collapses it.
    pub fn expand(&self, expand: bool) {
        let ptr = self.collapsible_pane_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_CollapsiblePane_Expand(ptr, expand) }
    }

    /// Collapses the pane.
    /// No-op if the pane has been destroyed.
    ///
    /// # Arguments
    /// * `collapse` - If true, collapses the pane; if false, expands it.
    pub fn collapse(&self, collapse: bool) {
        let ptr = self.collapsible_pane_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_CollapsiblePane_Collapse(ptr, collapse) }
    }

    /// Returns the pane window that can be used to add controls to the collapsible pane.
    /// This window is automatically shown/hidden when the pane expands/collapses.
    /// Returns None if the pane has been destroyed.
    pub fn get_pane(&self) -> Option<Window> {
        let ptr = self.collapsible_pane_ptr();
        if ptr.is_null() {
            return None;
        }
        let pane_ptr = unsafe { ffi::wxd_CollapsiblePane_GetPane(ptr) };
        if pane_ptr.is_null() {
            None
        } else {
            Some(unsafe { Window::from_ptr(pane_ptr) })
        }
    }

    /// Sets the text label for the collapsible pane button.
    /// No-op if the pane has been destroyed.
    ///
    /// # Arguments
    /// * `label` - The new label text.
    pub fn set_label(&self, label: &str) {
        let ptr = self.collapsible_pane_ptr();
        if ptr.is_null() {
            return;
        }
        let c_label = CString::new(label).expect("CString::new failed for label");
        unsafe { ffi::wxd_CollapsiblePane_SetLabel(ptr, c_label.as_ptr()) }
    }

    /// Gets the current text label of the collapsible pane button.
    /// Returns empty string if the pane has been destroyed.
    pub fn get_label(&self) -> String {
        let ptr = self.collapsible_pane_ptr();
        if ptr.is_null() {
            return String::new();
        }
        use std::ptr::null_mut;
        let len = unsafe { ffi::wxd_CollapsiblePane_GetLabel(ptr, null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }

        let mut b = vec![0; len as usize + 1]; // +1 for null terminator
        unsafe { ffi::wxd_CollapsiblePane_GetLabel(ptr, b.as_mut_ptr(), b.len()) };
        unsafe { CStr::from_ptr(b.as_ptr()).to_string_lossy().to_string() }
    }
}

// Manual WxWidget implementation for CollapsiblePane (using WindowHandle)
impl WxWidget for CollapsiblePane {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Note: We don't implement Deref to Window because returning a reference
// to a temporary Window is unsound. Users can access window methods through
// the WxWidget trait methods directly.

// Implement WxEvtHandler for event binding
impl WxEvtHandler for CollapsiblePane {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for CollapsiblePane {}

// Use the widget_builder macro to generate the CollapsiblePaneBuilder implementation
widget_builder!(
    name: CollapsiblePane,
    parent_type: &'a dyn WxWidget,
    style_type: CollapsiblePaneStyle,
    fields: {
        label: String = String::new(),
        name: String = "collapsiblePane".to_string()
    },
    build_impl: |slf| {
        let c_label = CString::new(&slf.label[..]).expect("CString::new failed for label");
        let c_name = CString::new(&slf.name[..]).expect("CString::new failed for name");

        let pane_ptr = unsafe {
            ffi::wxd_CollapsiblePane_Create(
                slf.parent.handle_ptr(),
                slf.id,
                c_label.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
                c_name.as_ptr(),
            )
        };

        if pane_ptr.is_null() {
            panic!("Failed to create CollapsiblePane: FFI returned null pointer.");
        }

        // Create a WindowHandle which automatically registers for destroy events
        CollapsiblePane {
            handle: WindowHandle::new(pane_ptr as *mut ffi::wxd_Window_t),
        }
    }
);

/// Events that can be emitted by a CollapsiblePane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapsiblePaneEvent {
    /// Emitted when the pane is expanded or collapsed.
    Changed,
}

/// Event data for a CollapsiblePane event.
#[derive(Debug)]
pub struct CollapsiblePaneEventData {
    event: Event,
}

impl CollapsiblePaneEventData {
    /// Create a new CollapsiblePaneEventData from a generic Event
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

    /// Get whether the pane is currently expanded
    pub fn is_expanded(&self) -> bool {
        // Note: In wxWidgets, you would typically get this from the event source
        // For now, we'll provide a way to check the current state
        // The user can call is_expanded() on the control itself
        true // Placeholder - in a real implementation you'd check the event details
    }
}

// Implement CollapsiblePane-specific event handlers
crate::implement_widget_local_event_handlers!(
    CollapsiblePane,
    CollapsiblePaneEvent,
    CollapsiblePaneEventData,
    Changed => changed, EventType::COLLAPSIBLEPANE_CHANGED
);

// XRC Support - enables CollapsiblePane to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for CollapsiblePane {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        CollapsiblePane {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for CollapsiblePane
impl crate::window::FromWindowWithClassName for CollapsiblePane {
    fn class_name() -> &'static str {
        "wxCollapsiblePane"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        CollapsiblePane {
            handle: WindowHandle::new(ptr),
        }
    }
}
