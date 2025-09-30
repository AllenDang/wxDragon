use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::{c_int, c_longlong};

use crate::event::{Event, EventType};
use crate::prelude::*;
use crate::window::Window;
use wxdragon_sys as ffi;

// Define style enum for AuiToolBar
widget_style_enum!(
    name: AuiToolBarStyle,
    doc: "Style flags for AuiToolBar.",
    variants: {
        Text: ffi::WXD_AUI_TB_TEXT, "Shows tool labels alongside icons.",
        NoTooltips: ffi::WXD_AUI_TB_NO_TOOLTIPS, "Disables tooltips.",
        NoAutoResize: ffi::WXD_AUI_TB_NO_AUTORESIZE, "Prevents automatic resizing of the toolbar.",
        Gripper: ffi::WXD_AUI_TB_GRIPPER, "Shows a gripper for dragging the toolbar.",
        Overflow: ffi::WXD_AUI_TB_OVERFLOW, "Allows overflow buttons for tools that don't fit.",
        Vertical: ffi::WXD_AUI_TB_VERTICAL, "Vertical orientation.",
        HorzLayout: ffi::WXD_AUI_TB_HORZ_LAYOUT, "Uses horizontal layout.",
        Horizontal: ffi::WXD_AUI_TB_HORIZONTAL, "Horizontal orientation.",
        Default: ffi::WXD_AUI_TB_GRIPPER | ffi::WXD_AUI_TB_OVERFLOW, "Default style (gripper and overflow)."
    },
    default_variant: Default
);

// Corresponds to WXDItemKindCEnum in C
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
#[derive(Default)]
pub enum ItemKind {
    #[default]
    Normal = 0, // WXD_ITEM_NORMAL
    Check = 1,     // WXD_ITEM_CHECK
    Radio = 2,     // WXD_ITEM_RADIO
    Separator = 3, // WXD_ITEM_SEPARATOR
}

/// Events for AuiToolBar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuiToolBarEvent {
    /// Tool clicked event
    ToolClicked,
    /// Tool enter event (mouse enters tool area)
    ToolEnter,
    /// Tool right-clicked event
    ToolRightClicked,
    /// Tool dropdown button clicked
    ToolDropDown,
    /// Menu event
    Menu,
}

/// Event data for an AuiToolBar event
#[derive(Debug)]
pub struct AuiToolBarEventData {
    event: Event,
}

impl AuiToolBarEventData {
    /// Create a new AuiToolBarEventData from a generic Event
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

#[derive(Clone)]
pub struct AuiToolBar {
    window: Window,
    #[allow(dead_code)]
    parent_ptr: *mut ffi::wxd_Window_t,
    _marker: PhantomData<()>,
}

impl AuiToolBar {
    fn from_ptr(ptr: *mut ffi::wxd_AuiToolBar_t, parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        AuiToolBar {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
            parent_ptr,
            _marker: PhantomData,
        }
    }

    /// Creates a new builder for AuiToolBar
    pub fn builder<'a>(parent: &'a dyn WxWidget) -> AuiToolBarBuilder<'a> {
        AuiToolBarBuilder::new(parent)
    }

    /// Adds a tool to the toolbar
    pub fn add_tool(&self, tool_id: i32, label: &str, short_help_string: &str, kind: ItemKind) {
        let c_label = CString::new(label).unwrap_or_default();
        let c_short_help = CString::new(short_help_string).unwrap_or_default();
        unsafe {
            ffi::wxd_AuiToolBar_AddTool(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                tool_id as c_int,
                c_label.as_ptr(),
                // Bitmaps are currently omitted in C API
                c_short_help.as_ptr(),
                kind as ffi::WXDItemKindCEnum,
            );
        }
    }

    /// Adds a label to the toolbar
    pub fn add_label(&self, tool_id: i32, label: &str, width: i32) {
        let c_label = CString::new(label).unwrap_or_default();
        unsafe {
            ffi::wxd_AuiToolBar_AddLabel(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                tool_id as c_int,
                c_label.as_ptr(),
                width as c_int,
            );
        }
    }

    /// Adds a control to the toolbar
    pub fn add_control<C: WxWidget>(&self, control: &C, label: &str) {
        let c_label = CString::new(label).unwrap_or_default();
        unsafe {
            ffi::wxd_AuiToolBar_AddControl(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                control.handle_ptr() as *mut ffi::wxd_Control_t,
                c_label.as_ptr(),
            );
        }
    }

    /// Adds a separator to the toolbar
    pub fn add_separator(&self) {
        unsafe {
            ffi::wxd_AuiToolBar_AddSeparator(self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t);
        }
    }

    /// Adds a spacer to the toolbar
    pub fn add_spacer(&self, pixels: i32) {
        unsafe {
            ffi::wxd_AuiToolBar_AddSpacer(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                pixels as c_int,
            );
        }
    }

    /// Adds a stretch spacer to the toolbar
    pub fn add_stretch_spacer(&self, proportion: i32) {
        unsafe {
            ffi::wxd_AuiToolBar_AddStretchSpacer(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                proportion as c_int,
            );
        }
    }

    /// Realizes the toolbar (finalizes the layout after adding tools)
    pub fn realize(&self) {
        unsafe {
            ffi::wxd_AuiToolBar_Realize(self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t);
        }
    }

    /// Sets the size of tool bitmaps
    pub fn set_tool_bitmap_size(&self, size: Size) {
        unsafe {
            ffi::wxd_AuiToolBar_SetToolBitmapSize(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                size.into(),
            );
        }
    }

    /// Gets the size of tool bitmaps
    pub fn get_tool_bitmap_size(&self) -> Size {
        let ffi_size = unsafe {
            ffi::wxd_AuiToolBar_GetToolBitmapSize(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t
            )
        };
        Size::from(ffi_size)
    }

    /// Sets whether the overflow button is visible
    pub fn set_overflow_visible(&self, visible: bool) {
        unsafe {
            ffi::wxd_AuiToolBar_SetOverflowVisible(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                visible,
            );
        }
    }

    /// Gets whether the overflow button is visible
    pub fn get_overflow_visible(&self) -> bool {
        unsafe {
            ffi::wxd_AuiToolBar_GetOverflowVisible(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t
            )
        }
    }

    /// Sets whether the gripper is visible
    pub fn set_gripper_visible(&self, visible: bool) {
        unsafe {
            ffi::wxd_AuiToolBar_SetGripperVisible(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                visible,
            );
        }
    }

    /// Gets whether the gripper is visible
    pub fn get_gripper_visible(&self) -> bool {
        unsafe {
            ffi::wxd_AuiToolBar_GetGripperVisible(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t
            )
        }
    }

    /// Sets whether a tool has a dropdown
    pub fn set_tool_drop_down(&self, tool_id: i32, dropdown: bool) {
        unsafe {
            ffi::wxd_AuiToolBar_SetToolDropDown(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                tool_id as c_int,
                dropdown,
            );
        }
    }

    /// Gets whether a tool has a dropdown
    pub fn get_tool_drop_down(&self, tool_id: i32) -> bool {
        unsafe {
            ffi::wxd_AuiToolBar_GetToolDropDown(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                tool_id as c_int,
            )
        }
    }

    /// Enables or disables a tool
    pub fn enable_tool(&self, tool_id: i32, enable: bool) {
        unsafe {
            ffi::wxd_AuiToolBar_EnableTool(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                tool_id as c_int,
                enable,
            );
        }
    }

    /// Gets whether a tool is enabled
    pub fn get_tool_enabled(&self, tool_id: i32) -> bool {
        unsafe {
            ffi::wxd_AuiToolBar_GetToolEnabled(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                tool_id as c_int,
            )
        }
    }

    /// Gets the number of tools
    pub fn get_tool_count(&self) -> i32 {
        unsafe {
            ffi::wxd_AuiToolBar_GetToolCount(self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t)
                as i32
        }
    }

    /// Clears all tools
    pub fn clear_tools(&self) {
        unsafe {
            ffi::wxd_AuiToolBar_ClearTools(self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t);
        }
    }

    /// Deletes a tool
    pub fn delete_tool(&self, tool_id: i32) -> bool {
        unsafe {
            ffi::wxd_AuiToolBar_DeleteTool(
                self.window.handle_ptr() as *mut ffi::wxd_AuiToolBar_t,
                tool_id as c_int,
            )
        }
    }
}

// Use widget_builder macro to create the builder
widget_builder!(
    name: AuiToolBar,
    parent_type: &'a dyn WxWidget,
    style_type: AuiToolBarStyle,
    fields: {},
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let ptr = unsafe {
            ffi::wxd_AuiToolBar_Create(
                parent_ptr,
                slf.id as c_int,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as c_longlong,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create AuiToolBar: wxWidgets returned a null pointer.");
        } else {
            AuiToolBar::from_ptr(ptr, parent_ptr)
        }
    }
);

// Implement all standard widget traits in one go
implement_widget_traits_with_target!(AuiToolBar, window, Window);

// Use the implement_widget_local_event_handlers macro to implement event handling
crate::implement_widget_local_event_handlers!(
    AuiToolBar,
    AuiToolBarEvent,
    AuiToolBarEventData,
    ToolClicked => tool_clicked, EventType::COMMAND_BUTTON_CLICKED,
    ToolEnter => tool_enter, EventType::TOOL_ENTER,
    ToolRightClicked => tool_right_clicked, EventType::RIGHT_UP,
    ToolDropDown => tool_dropdown, EventType::COMMAND_BUTTON_CLICKED, // No specific dropdown event, so use button clicked
    Menu => menu, EventType::MENU // Add menu event support
);
