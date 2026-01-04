//! Safe wrapper for wxPanel.

use crate::event::{MenuEvents, WindowEvents, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by new_from_composition for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use wxdragon_sys as ffi;

// --- Style enum using macro ---
widget_style_enum!(
    name: PanelStyle,
    doc: "Window style flags for Panel",
    variants: {
        TabTraversal: ffi::WXD_TAB_TRAVERSAL, "Allows the panel to participate in tab navigation. (Default)",
        BorderNone: ffi::WXD_BORDER_NONE, "No border.",
        BorderSimple: ffi::WXD_BORDER_SIMPLE, "A simple border.",
        BorderRaised: ffi::WXD_BORDER_RAISED, "A raised border.",
        BorderSunken: ffi::WXD_BORDER_SUNKEN, "A sunken border.",
        BorderStatic: ffi::WXD_BORDER_STATIC, "A static border.",
        BorderTheme: ffi::WXD_BORDER_THEME, "A theme border.",
        BorderDefault: ffi::WXD_BORDER_DEFAULT, "A default border."
    },
    default_variant: TabTraversal
);

/// Represents a wxPanel widget.
/// Panels are windows within a frame (or other window) that can contain other widgets.
///
/// Panel uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let panel = Panel::builder(&frame).build();
///
/// // Panel is Copy - no clone needed for closures!
/// panel.bind_event(move |_| {
///     // Safe: if panel was destroyed, this is a no-op
///     panel.set_background_colour(Colour::white());
/// });
///
/// // After parent destruction, panel operations are safe no-ops
/// frame.destroy();
/// assert!(!panel.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct Panel {
    /// Safe handle to the underlying wxPanel - automatically invalidated on destroy
    handle: WindowHandle,
}

impl Panel {
    /// Creates a new builder for a Panel.
    pub fn builder(parent: &dyn WxWidget) -> PanelBuilder<'_> {
        PanelBuilder::new(parent)
    }

    /// Creates a new Panel wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_Panel_t` pointer.
    /// Ownership is typically managed by the parent window in wxWidgets.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Panel_t) -> Self {
        assert!(!ptr.is_null());
        Panel {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Creates a new Panel from a raw window pointer.
    /// This is for backwards compatibility with widgets that compose Panel.
    /// The parent_ptr parameter is ignored (kept for API compatibility).
    #[allow(dead_code)]
    pub(crate) fn new_from_composition(_window: Window, _parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        // Use the window's pointer to create a new WindowHandle
        Self {
            handle: WindowHandle::new(_window.as_ptr()),
        }
    }

    /// Returns the raw underlying panel pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_Panel_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_Panel_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Helper to get raw panel pointer, returns null if widget has been destroyed
    #[inline]
    #[allow(dead_code)]
    fn panel_ptr(&self) -> *mut ffi::wxd_Panel_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_Panel_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Returns the underlying WindowHandle for this panel.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for Panel (using WindowHandle)
impl WxWidget for Panel {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for Panel {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl WindowEvents for Panel {}
impl MenuEvents for Panel {}

// Use the widget_builder macro to generate the PanelBuilder implementation
widget_builder!(
    name: Panel,
    parent_type: &'a dyn WxWidget,
    style_type: PanelStyle,
    fields: {},
    build_impl: |slf| {
        let panel_ptr = unsafe {
            ffi::wxd_Panel_Create(
                slf.parent.handle_ptr(),
                slf.id,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            )
        };

        if panel_ptr.is_null() {
            panic!("Failed to create Panel: FFI returned null pointer.");
        }

        unsafe { Panel::from_ptr(panel_ptr) }
    }
);

// XRC Support - enables Panel to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for Panel {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Panel {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Widget casting support for Panel
impl crate::window::FromWindowWithClassName for Panel {
    fn class_name() -> &'static str {
        "wxPanel"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Panel {
            handle: WindowHandle::new(ptr),
        }
    }
}
