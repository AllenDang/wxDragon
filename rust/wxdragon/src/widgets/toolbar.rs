//!
//! Safe wrapper for wxToolBar.

use crate::bitmap::Bitmap;
use crate::bitmap_bundle::BitmapBundle;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::id::Id;
use crate::menus::ItemKind; // Reuse ItemKind for tool types
use crate::window::{WindowHandle, WxWidget};
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
///
/// ToolBar uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// Toolbars generate `EventType::MENU` events on their parent window when a tool is clicked.
#[derive(Clone, Copy)]
pub struct ToolBar {
    /// Safe handle to the underlying wxToolBar - automatically invalidated on destroy
    handle: WindowHandle,
}

impl ToolBar {
    /// Creates a `ToolBar` wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_ToolBar_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_ToolBar_t) -> Self {
        ToolBar {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw toolbar pointer, returns null if widget has been destroyed
    #[inline]
    fn toolbar_ptr(&self) -> *mut ffi::wxd_ToolBar_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_ToolBar_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Returns the underlying WindowHandle for this ToolBar.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    /// Internal helper method for adding tools with all options.
    /// Prefer using `add_tool`, `add_check_tool`, `add_radio_tool` etc.
    /// Returns true if the tool was added successfully (C++ returns non-null ptr).
    /// No-op (returns false) if the toolbar has been destroyed.
    fn add_tool_raw(&self, config: ToolConfig) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }

        let c_label = CString::new(config.label).unwrap_or_default();
        let c_short_help = CString::new(config.short_help).unwrap_or_default();
        let c_longlong_help = CString::new(config.long_help).unwrap_or_default();
        let bmp_disabled_ptr = config.bitmap_disabled.map_or(std::ptr::null(), |bmp| bmp.as_const_ptr());

        unsafe {
            let tool_ptr = ffi::wxd_ToolBar_AddTool(
                ptr,
                config.tool_id,
                c_label.as_ptr(),
                config.bitmap.as_const_ptr(),
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
    pub fn add_check_tool(&self, tool_id: Id, label: &str, bitmap: &Bitmap, short_help: &str) -> bool {
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
    pub fn add_radio_tool(&self, tool_id: Id, label: &str, bitmap: &Bitmap, short_help: &str) -> bool {
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
    /// No-op if the toolbar has been destroyed.
    pub fn add_separator(&self) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_ToolBar_AddSeparator(ptr);
        }
    }

    /// Adds an arbitrary control (like a `Choice` or `TextCtrl`) to the toolbar.
    /// The control should have the toolbar as its parent.
    /// No-op if the toolbar has been destroyed.
    pub fn add_control<W: WxWidget>(&self, control: &W) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_ToolBar_AddControl(ptr, control.handle_ptr());
        }
    }

    /// Must be called after adding tools to finalize the toolbar layout.
    /// Returns true if successful, false if toolbar has been destroyed.
    pub fn realize(&self) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ToolBar_Realize(ptr) }
    }

    /// Enables or disables a tool.
    /// No-op if the toolbar has been destroyed.
    pub fn enable_tool(&self, tool_id: Id, enable: bool) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_ToolBar_EnableTool(ptr, tool_id, enable);
        }
    }

    /// Toggles the state of a check or radio tool.
    /// No-op if the toolbar has been destroyed.
    pub fn toggle_tool(&self, tool_id: Id, toggle: bool) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_ToolBar_ToggleTool(ptr, tool_id, toggle);
        }
    }

    /// Checks if a tool is enabled.
    /// Returns false if the toolbar has been destroyed.
    pub fn is_tool_enabled(&self, tool_id: Id) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ToolBar_IsToolEnabled(ptr, tool_id) }
    }

    /// Gets the state of a check or radio tool.
    /// Returns false if the toolbar has been destroyed.
    pub fn get_tool_state(&self, tool_id: Id) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ToolBar_GetToolState(ptr, tool_id) }
    }

    /// Sets the short help string (tooltip) for a tool.
    /// No-op if the toolbar has been destroyed.
    pub fn set_tool_short_help(&self, tool_id: Id, help_string: &str) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        let c_help = CString::new(help_string).unwrap_or_default();
        unsafe { ffi::wxd_ToolBar_SetToolShortHelp(ptr, tool_id, c_help.as_ptr()) };
    }

    /// Adds a normal tool to the toolbar using a BitmapBundle instead of a Bitmap.
    /// This is preferred for high-DPI support.
    ///
    /// # Arguments
    /// * `tool_id` - ID for the tool, used in event handling.
    /// * `label` - Label shown if `TB_TEXT` style is used.
    /// * `bundle` - The bitmap bundle containing icons at various resolutions.
    /// * `short_help` - Short help string (tooltip).
    ///
    /// Returns false if the toolbar has been destroyed.
    pub fn add_tool_bundle(&self, tool_id: Id, label: &str, bundle: &BitmapBundle, short_help: &str) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }

        let c_label = CString::new(label).unwrap_or_default();
        let c_short_help = CString::new(short_help).unwrap_or_default();

        unsafe {
            ffi::wxd_ToolBar_AddToolWithBundles(
                ptr,
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
    ///
    /// Returns false if the toolbar has been destroyed.
    pub fn add_tool_bundle_raw(
        &self,
        tool_id: Id,
        label: &str,
        bundle: &BitmapBundle,
        bundle_disabled: Option<&BitmapBundle>,
        short_help: &str,
        long_help: &str,
    ) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }

        let c_label = CString::new(label).unwrap_or_default();
        let c_short_help = CString::new(short_help).unwrap_or_default();
        let c_long_help = CString::new(long_help).unwrap_or_default();
        let bundle_disabled_ptr = bundle_disabled.map_or(std::ptr::null_mut(), |b| b.as_ptr());

        unsafe {
            ffi::wxd_ToolBar_AddToolWithBundles(
                ptr,
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
    /// Returns None if the toolbar has been destroyed or the tool name is not found.
    #[cfg(feature = "xrc")]
    pub fn get_tool_by_name(&self, tool_name: &str) -> Option<crate::widgets::Tool> {
        use crate::xrc::XmlResource;

        if !self.handle.is_valid() {
            return None;
        }

        // Get the XRC ID for this tool name
        let tool_id = XmlResource::get_xrc_id(tool_name);

        if tool_id != -1 {
            Some(crate::widgets::Tool::new(self.handle, tool_id))
        } else {
            None
        }
    }
}

// --- Trait Implementations ---

// Manual WxWidget implementation for ToolBar (using WindowHandle)
impl WxWidget for ToolBar {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for ToolBar {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for ToolBar {}

// No Drop needed, wxToolBar is a Window managed by its parent frame.

// Implement widget-specific event handlers
crate::implement_widget_local_event_handlers!(
    ToolBar,
    ToolBarEvent,
    ToolBarEventData,
    Menu => menu, EventType::MENU
);

// XRC Support - enables ToolBar to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for ToolBar {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ToolBar {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for ToolBar
impl crate::window::FromWindowWithClassName for ToolBar {
    fn class_name() -> &'static str {
        "wxToolBar"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ToolBar {
            handle: WindowHandle::new(ptr),
        }
    }
}
