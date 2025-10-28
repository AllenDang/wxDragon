//!
//! Safe wrapper for wxToolBar.

use crate::bitmap::Bitmap;
use crate::bitmap_bundle::BitmapBundle;
use crate::event::{Event, EventType, WindowEvents, WxEvtHandler};
use crate::id::Id;
use crate::menus::ItemKind; // Reuse ItemKind for tool types
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// --- ToolBarStyle Enum ---
widget_style_enum!(
    name: ToolBarStyle,
    doc: "Style flags for ToolBar widgets.",
    variants: {
        Default: ffi::WXD_TB_HORIZONTAL, "Default style, horizontal toolbar.",
        Vertical: ffi::WXD_TB_VERTICAL, "Vertical toolbar.",
        Text: ffi::WXD_TB_TEXT, "Show text labels for tools.",
        NoIcons: ffi::WXD_TB_NOICONS, "Show text only, no icons.",
        NoDivider: ffi::WXD_TB_NODIVIDER, "No divider between tool groups.",
        Flat: ffi::WXD_TB_FLAT, "Flat toolbar look.",
        Dockable: ffi::WXD_TB_DOCKABLE, "Toolbar can be dragged and docked."
    },
    default_variant: Default
);

/// Configuration for adding a tool to the toolbar
pub struct ToolConfig<'a> {
    pub tool_id: Id,
    pub label: &'a str,
    pub bitmap: &'a Bitmap,
    pub bitmap_disabled: Option<&'a Bitmap>,
    pub kind: ItemKind,
    pub short_help: &'a str,
    pub long_help: &'a str,
}

/// Events for ToolBar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolBarEvent {
    /// Menu event (tool clicked)
    Menu,
}

/// Event data for a ToolBar event
#[derive(Debug)]
pub struct ToolBarEventData {
    event: Event,
}

impl ToolBarEventData {
    /// Create a new ToolBarEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the ID of the tool that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Skip this event (allow it to be processed by the parent window)
    pub fn skip(&self, skip: bool) {
        self.event.skip(skip);
    }

    /// Get the integer value associated with this event (typically the tool ID)
    pub fn get_int(&self) -> Option<i32> {
        self.event.get_int()
    }

    /// Get whether the tool is checked (for checkable tools)
    pub fn is_checked(&self) -> Option<bool> {
        self.event.is_checked()
    }
}

/// Represents a wxToolBar control.
/// Toolbars generate `EventType::MENU` events on their parent window when a tool is clicked.
#[derive(Clone)]
pub struct ToolBar {
    window: Window, // Composition: ToolBar IS a Window (and Control)
}

impl ToolBar {
    /// Creates a `ToolBar` wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_ToolBar_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_ToolBar_t) -> Self {
        ToolBar {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
        }
    }

    /// Internal helper method for adding tools with all options.
    /// Prefer using `add_tool`, `add_check_tool`, `add_radio_tool` etc.
    /// Returns true if the tool was added successfully (C++ returns non-null ptr).
    /// # Safety
    /// Requires valid toolbar pointer and CString conversions must succeed.
    fn add_tool_raw(&self, config: ToolConfig) -> bool {
        let c_label = CString::new(config.label).unwrap_or_default();
        let c_short_help = CString::new(config.short_help).unwrap_or_default();
        let c_longlong_help = CString::new(config.long_help).unwrap_or_default();
        let bmp_disabled_ptr = config.bitmap_disabled.map_or(std::ptr::null(), |bmp| **bmp);

        unsafe {
            let tool_ptr = ffi::wxd_ToolBar_AddTool(
                self.window.as_ptr() as *mut _,
                config.tool_id,
                c_label.as_ptr(),
                **config.bitmap,
                bmp_disabled_ptr,
                config.kind as c_int,
                c_short_help.as_ptr(),
                c_longlong_help.as_ptr(),
            );
            !tool_ptr.is_null()
        }
    }

    /// Adds a normal tool to the toolbar.
    ///
    /// # Arguments
    /// * `tool_id` - ID for the tool, used in event handling.
    /// * `label` - Label shown if `TB_TEXT` style is used.
    /// * `bitmap` - The bitmap for the tool's normal state.
    /// * `short_help` - Short help string (tooltip).
    pub fn add_tool(&self, tool_id: Id, label: &str, bitmap: &Bitmap, short_help: &str) -> bool {
        self.add_tool_raw(ToolConfig {
            tool_id,
            label,
            bitmap,
            bitmap_disabled: None,
            kind: ItemKind::Normal,
            short_help,
            long_help: "",
        })
    }

    /// Adds a check tool (toggle tool) to the toolbar.
    pub fn add_check_tool(
        &self,
        tool_id: Id,
        label: &str,
        bitmap: &Bitmap,
        short_help: &str,
    ) -> bool {
        self.add_tool_raw(ToolConfig {
            tool_id,
            label,
            bitmap,
            bitmap_disabled: None,
            kind: ItemKind::Check,
            short_help,
            long_help: "",
        })
    }

    /// Adds a radio tool to the toolbar.
    /// Radio tools require grouping with separators or other radio tools.
    pub fn add_radio_tool(
        &self,
        tool_id: Id,
        label: &str,
        bitmap: &Bitmap,
        short_help: &str,
    ) -> bool {
        self.add_tool_raw(ToolConfig {
            tool_id,
            label,
            bitmap,
            bitmap_disabled: None,
            kind: ItemKind::Radio,
            short_help,
            long_help: "",
        })
    }

    /// Adds a separator.
    pub fn add_separator(&self) {
        unsafe {
            ffi::wxd_ToolBar_AddSeparator(self.window.as_ptr() as *mut _);
        }
    }

    /// Adds an arbitrary control (like a `Choice` or `TextCtrl`) to the toolbar.
    /// The control should have the toolbar as its parent.
    pub fn add_control<W: WxWidget>(&self, control: &W) {
        unsafe {
            ffi::wxd_ToolBar_AddControl(self.window.as_ptr() as *mut _, control.handle_ptr());
        }
    }

    /// Must be called after adding tools to finalize the toolbar layout.
    /// Returns true if successful.
    pub fn realize(&self) -> bool {
        unsafe { ffi::wxd_ToolBar_Realize(self.window.as_ptr() as *mut _) }
    }

    /// Enables or disables a tool.
    pub fn enable_tool(&self, tool_id: Id, enable: bool) {
        unsafe {
            ffi::wxd_ToolBar_EnableTool(self.window.as_ptr() as *mut _, tool_id, enable);
        }
    }

    /// Toggles the state of a check or radio tool.
    pub fn toggle_tool(&self, tool_id: Id, toggle: bool) {
        unsafe {
            ffi::wxd_ToolBar_ToggleTool(self.window.as_ptr() as *mut _, tool_id, toggle);
        }
    }

    /// Checks if a tool is enabled.
    pub fn is_tool_enabled(&self, tool_id: Id) -> bool {
        unsafe { ffi::wxd_ToolBar_IsToolEnabled(self.window.as_ptr() as *mut _, tool_id) }
    }

    /// Gets the state of a check or radio tool.
    pub fn get_tool_state(&self, tool_id: Id) -> bool {
        unsafe { ffi::wxd_ToolBar_GetToolState(self.window.as_ptr() as *mut _, tool_id) }
    }

    /// Sets the short help string (tooltip) for a tool.
    pub fn set_tool_short_help(&self, tool_id: Id, help_string: &str) {
        let c_help = CString::new(help_string).unwrap_or_default();
        unsafe {
            ffi::wxd_ToolBar_SetToolShortHelp(
                self.window.as_ptr() as *mut _,
                tool_id,
                c_help.as_ptr(),
            );
        }
    }

    /// Adds a normal tool to the toolbar using a BitmapBundle instead of a Bitmap.
    /// This is preferred for high-DPI support.
    ///
    /// # Arguments
    /// * `tool_id` - ID for the tool, used in event handling.
    /// * `label` - Label shown if `TB_TEXT` style is used.
    /// * `bundle` - The bitmap bundle containing icons at various resolutions.
    /// * `short_help` - Short help string (tooltip).
    pub fn add_tool_bundle(
        &self,
        tool_id: Id,
        label: &str,
        bundle: &BitmapBundle,
        short_help: &str,
    ) -> bool {
        let c_label = CString::new(label).unwrap_or_default();
        let c_short_help = CString::new(short_help).unwrap_or_default();

        unsafe {
            ffi::wxd_ToolBar_AddToolWithBundles(
                self.window.as_ptr() as *mut _,
                tool_id,
                c_label.as_ptr(),
                bundle.as_ptr(),
                std::ptr::null_mut(), // No disabled bitmap bundle
                c_short_help.as_ptr(),
                std::ptr::null(), // No long help
            )
        }
    }

    /// Adds a normal tool to the toolbar with more options, using BitmapBundle.
    ///
    /// # Arguments
    /// * `tool_id` - ID for the tool, used in event handling.
    /// * `label` - Label shown if `TB_TEXT` style is used.
    /// * `bundle` - The bitmap bundle for the tool's normal state.
    /// * `bundle_disabled` - Optional bitmap bundle for the tool's disabled state.
    /// * `kind` - Type of tool (normal, check, radio).
    /// * `short_help` - Short help string (tooltip).
    /// * `long_help` - Long help string (status bar).
    pub fn add_tool_bundle_raw(
        &self,
        tool_id: Id,
        label: &str,
        bundle: &BitmapBundle,
        bundle_disabled: Option<&BitmapBundle>,
        short_help: &str,
        long_help: &str,
    ) -> bool {
        let c_label = CString::new(label).unwrap_or_default();
        let c_short_help = CString::new(short_help).unwrap_or_default();
        let c_long_help = CString::new(long_help).unwrap_or_default();
        let bundle_disabled_ptr = bundle_disabled.map_or(std::ptr::null_mut(), |b| b.as_ptr());

        unsafe {
            ffi::wxd_ToolBar_AddToolWithBundles(
                self.window.as_ptr() as *mut _,
                tool_id,
                c_label.as_ptr(),
                bundle.as_ptr(),
                bundle_disabled_ptr,
                c_short_help.as_ptr(),
                c_long_help.as_ptr(),
            )
        }
    }

    /// Gets a tool by its XRC name.
    /// Returns a Tool wrapper that can be used for event binding and operations.
    #[cfg(feature = "xrc")]
    pub fn get_tool_by_name(&self, tool_name: &str) -> Option<crate::widgets::Tool> {
        use crate::xrc::XmlResource;

        // Get the XRC ID for this tool name
        let tool_id = XmlResource::get_xrc_id(tool_name);

        if tool_id != -1 {
            Some(crate::widgets::Tool::new(
                self.window,
                tool_id,
                tool_name.to_string(),
            ))
        } else {
            None
        }
    }
}

// --- Trait Implementations ---

impl WxWidget for ToolBar {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

// Add WxEvtHandler implementation
impl WxEvtHandler for ToolBar {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.handle_ptr() as *mut ffi::wxd_EvtHandler_t
    }
}

// ToolBar doesn't handle events directly, the parent (Frame) does via EventType::MENU
// So, no WxEvtHandler impl needed for ToolBar itself.

// No Drop needed, wxToolBar is a Window managed by its parent frame.

// Implement widget-specific event handlers
crate::implement_widget_local_event_handlers!(
    ToolBar,
    ToolBarEvent,
    ToolBarEventData,
    Menu => menu, EventType::MENU
);

impl WindowEvents for ToolBar {}

// Add XRC support
// XRC Support - enables ToolBar to be created from XRC-managed pointers
impl_xrc_support!(ToolBar, { window });

// Widget casting support for ToolBar
impl_widget_cast!(ToolBar, "wxToolBar", { window });
