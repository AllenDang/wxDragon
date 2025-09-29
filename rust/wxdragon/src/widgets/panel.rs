//! Safe wrapper for wxPanel.

use crate::event::WindowEvents;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
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
#[derive(Clone)]
pub struct Panel {
    window: Window, // Embed the generic Window
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
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the raw underlying panel pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_Panel_t {
        self.window.as_ptr() as *mut ffi::wxd_Panel_t
    }
}

// Apply common trait implementations for Panel
implement_widget_traits_with_target!(Panel, window, Window);

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

// Implement WindowEvents trait for Panel
impl WindowEvents for Panel {}

// Implement MenuEvents trait for Panel (for context menus)
impl crate::event::MenuEvents for Panel {}

// XRC Support - enables Panel to be created from XRC-managed pointers
impl_xrc_support!(Panel, { window });

// Widget casting support for Panel
impl_widget_cast!(Panel, "wxPanel", { window });
