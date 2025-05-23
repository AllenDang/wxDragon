//! Safe wrapper for individual toolbar tools loaded from XRC.

use crate::event::{Event, EventType, WxEvtHandler};
use crate::id::Id;
use crate::window::{Window, WxWidget};
use wxdragon_sys as ffi;

/// Represents an individual toolbar tool loaded from XRC.
///
/// In wxWidgets, tools are not standalone widgets but are managed by their parent toolbar.
/// This wrapper provides a convenient way to access XRC-defined tools and bind events to them.
#[derive(Clone)]
pub struct Tool {
    /// Reference to the parent toolbar that manages this tool
    toolbar_window: Window,
    /// The tool's ID for event handling and toolbar operations
    tool_id: Id,
    /// The tool's XRC name for identification
    tool_name: String,
}

impl Tool {
    /// Creates a Tool wrapper from a toolbar and tool information.
    /// This is typically called by the XRC loading system.
    pub(crate) fn new(toolbar_window: Window, tool_id: Id, tool_name: String) -> Self {
        Self {
            toolbar_window,
            tool_id,
            tool_name,
        }
    }

    /// Gets the tool's ID used for event handling.
    pub fn get_tool_id(&self) -> Id {
        self.tool_id
    }

    /// Gets the tool's XRC name.
    pub fn get_tool_name(&self) -> &str {
        &self.tool_name
    }

    /// Enables or disables this tool.
    pub fn enable(&self, enable: bool) {
        unsafe {
            ffi::wxd_ToolBar_EnableTool(
                self.toolbar_window.as_ptr() as *mut ffi::wxd_ToolBar_t,
                self.tool_id,
                enable,
            );
        }
    }

    /// Toggles this tool (for checkable tools).
    pub fn toggle(&self, toggle: bool) {
        unsafe {
            ffi::wxd_ToolBar_ToggleTool(
                self.toolbar_window.as_ptr() as *mut ffi::wxd_ToolBar_t,
                self.tool_id,
                toggle,
            );
        }
    }

    /// Returns whether this tool is enabled.
    pub fn is_enabled(&self) -> bool {
        unsafe {
            ffi::wxd_ToolBar_IsToolEnabled(
                self.toolbar_window.as_ptr() as *mut ffi::wxd_ToolBar_t,
                self.tool_id,
            )
        }
    }

    /// Returns the state of this tool (checked/unchecked for checkable tools).
    pub fn get_state(&self) -> bool {
        unsafe {
            ffi::wxd_ToolBar_GetToolState(
                self.toolbar_window.as_ptr() as *mut ffi::wxd_ToolBar_t,
                self.tool_id,
            )
        }
    }

    /// Sets the short help string (tooltip) for this tool.
    pub fn set_short_help(&self, help_string: &str) {
        use std::ffi::CString;
        let c_help = CString::new(help_string).unwrap_or_default();
        unsafe {
            ffi::wxd_ToolBar_SetToolShortHelp(
                self.toolbar_window.as_ptr() as *mut ffi::wxd_ToolBar_t,
                self.tool_id,
                c_help.as_ptr(),
            );
        }
    }

    /// Binds a click event handler to this tool.
    /// This is a convenience method that binds a tool event on the parent toolbar for this tool's ID.
    pub fn on_click<F>(&self, handler: F)
    where
        F: FnMut(Event) + 'static,
    {
        // Use ID-specific binding for TOOL events
        self.toolbar_window
            .bind_with_id_internal(EventType::TOOL, self.tool_id, handler);
    }

    /// Special XRC loading method for tools.
    /// This looks up the tool by name in the parent toolbar and creates a Tool wrapper.
    pub fn from_xrc_name(toolbar: &crate::widgets::ToolBar, tool_name: &str) -> Option<Self> {
        use crate::xrc::XmlResource;

        // Get the XRC ID for this tool name
        let tool_id = XmlResource::get_xrc_id(tool_name);

        if tool_id != -1 {
            Some(Tool::new(
                unsafe { crate::window::Window::from_ptr(toolbar.handle_ptr()) },
                tool_id,
                tool_name.to_string(),
            ))
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
        self.toolbar_window.as_ptr()
    }

    fn get_id(&self) -> i32 {
        self.tool_id
    }
}

/// Event handler implementation for Tool (delegates to toolbar)
impl WxEvtHandler for Tool {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.toolbar_window.as_ptr() as *mut ffi::wxd_EvtHandler_t
    }
}
