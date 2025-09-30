//! Safe wrapper for wxCollapsiblePane.

use crate::event::{Event, EventType};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
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
#[derive(Clone)]
pub struct CollapsiblePane {
    window: Window, // Embed the generic Window
}

impl CollapsiblePane {
    /// Creates a new builder for a CollapsiblePane.
    pub fn builder(parent: &dyn WxWidget) -> CollapsiblePaneBuilder<'_> {
        CollapsiblePaneBuilder::new(parent)
    }

    /// Creates a new CollapsiblePane wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_CollapsiblePane_t` pointer.
    /// Ownership is typically managed by the parent window in wxWidgets.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_CollapsiblePane_t) -> Self {
        assert!(!ptr.is_null());
        CollapsiblePane {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the raw underlying collapsible pane pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_CollapsiblePane_t {
        self.window.as_ptr() as *mut ffi::wxd_CollapsiblePane_t
    }

    // --- CollapsiblePane-specific methods ---

    /// Returns true if the pane is currently expanded, false otherwise.
    pub fn is_expanded(&self) -> bool {
        unsafe { ffi::wxd_CollapsiblePane_IsExpanded(self.as_ptr()) }
    }

    /// Returns true if the pane is currently collapsed, false otherwise.
    pub fn is_collapsed(&self) -> bool {
        unsafe { ffi::wxd_CollapsiblePane_IsCollapsed(self.as_ptr()) }
    }

    /// Expands or collapses the pane.
    ///
    /// # Arguments
    /// * `expand` - If true, expands the pane; if false, collapses it.
    pub fn expand(&self, expand: bool) {
        unsafe { ffi::wxd_CollapsiblePane_Expand(self.as_ptr(), expand) }
    }

    /// Collapses the pane.
    ///
    /// # Arguments
    /// * `collapse` - If true, collapses the pane; if false, expands it.
    pub fn collapse(&self, collapse: bool) {
        unsafe { ffi::wxd_CollapsiblePane_Collapse(self.as_ptr(), collapse) }
    }

    /// Returns the pane window that can be used to add controls to the collapsible pane.
    /// This window is automatically shown/hidden when the pane expands/collapses.
    pub fn get_pane(&self) -> Option<Window> {
        let ptr = unsafe { ffi::wxd_CollapsiblePane_GetPane(self.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Window::from_ptr(ptr) })
        }
    }

    /// Sets the text label for the collapsible pane button.
    ///
    /// # Arguments
    /// * `label` - The new label text.
    pub fn set_label(&self, label: &str) {
        let c_label = CString::new(label).expect("CString::new failed for label");
        unsafe { ffi::wxd_CollapsiblePane_SetLabel(self.as_ptr(), c_label.as_ptr()) }
    }

    /// Gets the current text label of the collapsible pane button.
    pub fn get_label(&self) -> String {
        let c_str = unsafe { ffi::wxd_CollapsiblePane_GetLabel(self.as_ptr()) };
        if c_str.is_null() {
            return String::new();
        }

        let result = unsafe {
            let cstr = std::ffi::CStr::from_ptr(c_str);
            let string = cstr.to_string_lossy().into_owned();
            // Free the C string allocated by strdup in C++
            ffi::wxd_free_string(c_str);
            string
        };

        result
    }
}

// Apply common trait implementations for CollapsiblePane
implement_widget_traits_with_target!(CollapsiblePane, window, Window);

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

        unsafe { CollapsiblePane::from_ptr(pane_ptr) }
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

// Add XRC Support - enables CollapsiblePane to be created from XRC-managed pointers
impl_xrc_support!(CollapsiblePane, { window });

// Widget casting support for CollapsiblePane
impl_widget_cast!(CollapsiblePane, "wxCollapsiblePane", { window });
