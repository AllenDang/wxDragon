//! Safe wrapper for individual toolbar tools loaded from XRC.

use crate::event::{Event, EventType, WxEvtHandler};
use crate::id::Id;
use crate::window::{Window, WindowHandle, WxWidget};
use wxdragon_sys as ffi;

/// Represents an individual toolbar tool loaded from XRC.
///
/// In wxWidgets, tools are not standalone widgets but are managed by their parent toolbar.
/// This wrapper provides a convenient way to access XRC-defined tools and bind events to them.
///
/// Tool uses `WindowHandle` internally for the parent toolbar reference, providing safe
/// memory management. If the toolbar is destroyed, operations on the Tool become safe no-ops.
#[derive(Clone, Copy)]
pub struct Tool {
    /// Safe handle to the parent toolbar - automatically invalidated when toolbar is destroyed
    toolbar_handle: WindowHandle,
    /// The tool's ID for event handling and toolbar operations
    tool_id: Id,
}

impl Tool {
    /// Creates a Tool wrapper from a toolbar and tool information.
    /// This is typically called by the XRC loading system.
    #[cfg(feature = "xrc")]
    pub(crate) fn new(toolbar_handle: WindowHandle, tool_id: Id) -> Self {
        Self { toolbar_handle, tool_id }
    }

    /// Helper to get raw toolbar pointer, returns null if toolbar has been destroyed
    #[inline]
    fn toolbar_ptr(&self) -> *mut ffi::wxd_ToolBar_t {
        self.toolbar_handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_ToolBar_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Check if this tool's parent toolbar is still valid.
    pub fn is_valid(&self) -> bool {
        self.toolbar_handle.is_valid()
    }

    /// Gets the tool's ID used for event handling.
    pub fn get_tool_id(&self) -> Id {
        self.tool_id
    }

    /// Enables or disables this tool.
    /// No-op if the toolbar has been destroyed.
    pub fn enable(&self, enable: bool) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ToolBar_EnableTool(ptr, self.tool_id, enable) };
    }

    /// Toggles this tool (for checkable tools).
    /// No-op if the toolbar has been destroyed.
    pub fn toggle(&self, toggle: bool) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ToolBar_ToggleTool(ptr, self.tool_id, toggle) };
    }

    /// Returns whether this tool is enabled.
    /// Returns false if the toolbar has been destroyed.
    pub fn is_enabled(&self) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ToolBar_IsToolEnabled(ptr, self.tool_id) }
    }

    /// Returns the state of this tool (checked/unchecked for checkable tools).
    /// Returns false if the toolbar has been destroyed.
    pub fn get_state(&self) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ToolBar_GetToolState(ptr, self.tool_id) }
    }

    /// Sets the short help string (tooltip) for this tool.
    /// No-op if the toolbar has been destroyed.
    pub fn set_short_help(&self, help_string: &str) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        use std::ffi::CString;
        let c_help = CString::new(help_string).unwrap_or_default();
        unsafe {
            ffi::wxd_ToolBar_SetToolShortHelp(ptr, self.tool_id, c_help.as_ptr());
        }
    }

    /// Binds a click handler using a platform-appropriate route.
    ///
    /// Platform default routing:
    /// - Windows (MSW + XRC) / Linux (GTK): bind as a MENU command on the top-level frame
    /// - macOS: bind as a TOOL event on the parent toolbar
    ///
    /// Use `on_click_via_menu` or `on_click_via_tool` to override explicitly.
    /// No-op if the toolbar has been destroyed.
    pub fn on_click<F>(&self, handler: F)
    where
        F: FnMut(Event) + 'static,
    {
        self.on_click_via_menu(handler);
    }

    /// Binds a click event handler for this tool as a `MENU` command on the top-level frame.
    ///
    /// This is useful on platforms where toolbar commands are routed as menu commands
    /// to the owning frame (notably MSW with some XRC configurations). If you just need
    /// the frame-level MENU route, call this; otherwise prefer `on_click` which binds both.
    /// No-op if the toolbar has been destroyed.
    fn on_click_via_menu<F>(&self, handler: F)
    where
        F: FnMut(Event) + 'static,
    {
        if let Some(frame_win) = self.top_level_window() {
            frame_win.bind_with_id_internal(EventType::MENU, self.tool_id, handler);
        }
    }

    // Internal helper: locate top-level parent window (typically a Frame)
    fn top_level_window(&self) -> Option<Window> {
        let mut current_ptr = self.toolbar_handle.get_ptr()?;
        loop {
            let parent_ptr = unsafe { ffi::wxd_Window_GetParent(current_ptr) };
            if parent_ptr.is_null() {
                return Some(unsafe { Window::from_ptr(current_ptr) });
            }
            current_ptr = parent_ptr;
        }
    }

    /// Special XRC loading method for tools.
    /// This looks up the tool by name in the parent toolbar and creates a Tool wrapper.
    #[cfg(feature = "xrc")]
    pub fn from_xrc_name(toolbar: &crate::widgets::ToolBar, tool_name: &str) -> Option<Self> {
        use crate::xrc::XmlResource;

        // Get the XRC ID for this tool name
        let tool_id = XmlResource::get_xrc_id(tool_name);

        if tool_id != -1 {
            Some(Tool::new(toolbar.window_handle(), tool_id))
        } else {
            None
        }
    }
}

/// Implement WxWidget for Tool (delegating to toolbar window)
impl WxWidget for Tool {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        // Tools don't have their own window handle - they're part of the toolbar
        // Return the toolbar's handle for XRC compatibility
        self.toolbar_handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn get_id(&self) -> i32 {
        self.tool_id
    }
}

/// Event handler implementation for Tool (delegates to toolbar)
impl WxEvtHandler for Tool {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.toolbar_handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for Tool {}
