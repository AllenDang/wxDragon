use std::ffi::CString;
use std::os::raw::{c_int, c_longlong};

use crate::event::{Event, EventType, WxEvtHandler};
use crate::prelude::*;
use crate::window::{WindowHandle, WxWidget};
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

/// Represents a wxAuiToolBar.
///
/// AuiToolBar uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let toolbar = AuiToolBar::builder(&frame).build();
///
/// // AuiToolBar is Copy - no clone needed for closures!
/// toolbar.add_tool(1, "New", "Create new file", ItemKind::Normal);
/// toolbar.realize();
///
/// // After parent destruction, toolbar operations are safe no-ops
/// frame.destroy();
/// assert!(!toolbar.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct AuiToolBar {
    /// Safe handle to the underlying wxAuiToolBar - automatically invalidated on destroy
    handle: WindowHandle,
}

impl AuiToolBar {
    /// Creates a new AuiToolBar from a raw pointer.
    /// This is intended for internal use by the builder.
    fn from_ptr(ptr: *mut ffi::wxd_AuiToolBar_t) -> Self {
        AuiToolBar {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Creates a new builder for AuiToolBar
    pub fn builder<'a>(parent: &'a dyn WxWidget) -> AuiToolBarBuilder<'a> {
        AuiToolBarBuilder::new(parent)
    }

    /// Helper to get raw toolbar pointer, returns null if widget has been destroyed
    #[inline]
    fn toolbar_ptr(&self) -> *mut ffi::wxd_AuiToolBar_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_AuiToolBar_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Adds a tool to the toolbar.
    /// No-op if the toolbar has been destroyed.
    pub fn add_tool(&self, tool_id: i32, label: &str, short_help_string: &str, kind: ItemKind) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        let c_label = CString::new(label).unwrap_or_default();
        let c_short_help = CString::new(short_help_string).unwrap_or_default();
        unsafe {
            ffi::wxd_AuiToolBar_AddTool(
                ptr,
                tool_id as c_int,
                c_label.as_ptr(),
                // Bitmaps are currently omitted in C API
                c_short_help.as_ptr(),
                kind as ffi::WXDItemKindCEnum,
            );
        }
    }

    /// Adds a label to the toolbar.
    /// No-op if the toolbar has been destroyed.
    pub fn add_label(&self, tool_id: i32, label: &str, width: i32) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        let c_label = CString::new(label).unwrap_or_default();
        unsafe { ffi::wxd_AuiToolBar_AddLabel(ptr, tool_id as c_int, c_label.as_ptr(), width as c_int) };
    }

    /// Adds a control to the toolbar.
    /// No-op if the toolbar has been destroyed.
    pub fn add_control<C: WxWidget>(&self, control: &C, label: &str) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        let c_label = CString::new(label).unwrap_or_default();
        unsafe { ffi::wxd_AuiToolBar_AddControl(ptr, control.handle_ptr() as *mut ffi::wxd_Control_t, c_label.as_ptr()) };
    }

    /// Adds a separator to the toolbar.
    /// No-op if the toolbar has been destroyed.
    pub fn add_separator(&self) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_AuiToolBar_AddSeparator(ptr) };
    }

    /// Adds a spacer to the toolbar.
    /// No-op if the toolbar has been destroyed.
    pub fn add_spacer(&self, pixels: i32) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_AuiToolBar_AddSpacer(ptr, pixels as c_int) };
    }

    /// Adds a stretch spacer to the toolbar.
    /// No-op if the toolbar has been destroyed.
    pub fn add_stretch_spacer(&self, proportion: i32) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_AuiToolBar_AddStretchSpacer(ptr, proportion as c_int) };
    }

    /// Realizes the toolbar (finalizes the layout after adding tools).
    /// No-op if the toolbar has been destroyed.
    pub fn realize(&self) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_AuiToolBar_Realize(ptr) };
    }

    /// Sets the size of tool bitmaps.
    /// No-op if the toolbar has been destroyed.
    pub fn set_tool_bitmap_size(&self, size: Size) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_AuiToolBar_SetToolBitmapSize(ptr, size.into()) };
    }

    /// Gets the size of tool bitmaps.
    /// Returns default Size if the toolbar has been destroyed.
    pub fn get_tool_bitmap_size(&self) -> Size {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return Size::default();
        }
        let ffi_size = unsafe { ffi::wxd_AuiToolBar_GetToolBitmapSize(ptr) };
        Size::from(ffi_size)
    }

    /// Sets whether the overflow button is visible.
    /// No-op if the toolbar has been destroyed.
    pub fn set_overflow_visible(&self, visible: bool) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_AuiToolBar_SetOverflowVisible(ptr, visible) };
    }

    /// Gets whether the overflow button is visible.
    /// Returns false if the toolbar has been destroyed.
    pub fn get_overflow_visible(&self) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_AuiToolBar_GetOverflowVisible(ptr) }
    }

    /// Sets whether the gripper is visible.
    /// No-op if the toolbar has been destroyed.
    pub fn set_gripper_visible(&self, visible: bool) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_AuiToolBar_SetGripperVisible(ptr, visible) };
    }

    /// Gets whether the gripper is visible.
    /// Returns false if the toolbar has been destroyed.
    pub fn get_gripper_visible(&self) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_AuiToolBar_GetGripperVisible(ptr) }
    }

    /// Sets whether a tool has a dropdown.
    /// No-op if the toolbar has been destroyed.
    pub fn set_tool_drop_down(&self, tool_id: i32, dropdown: bool) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_AuiToolBar_SetToolDropDown(ptr, tool_id as c_int, dropdown) };
    }

    /// Gets whether a tool has a dropdown.
    /// Returns false if the toolbar has been destroyed.
    pub fn get_tool_drop_down(&self, tool_id: i32) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_AuiToolBar_GetToolDropDown(ptr, tool_id as c_int) }
    }

    /// Enables or disables a tool.
    /// No-op if the toolbar has been destroyed.
    pub fn enable_tool(&self, tool_id: i32, enable: bool) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_AuiToolBar_EnableTool(ptr, tool_id as c_int, enable) };
    }

    /// Gets whether a tool is enabled.
    /// Returns false if the toolbar has been destroyed.
    pub fn get_tool_enabled(&self, tool_id: i32) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_AuiToolBar_GetToolEnabled(ptr, tool_id as c_int) }
    }

    /// Gets the number of tools.
    /// Returns 0 if the toolbar has been destroyed.
    pub fn get_tool_count(&self) -> i32 {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_AuiToolBar_GetToolCount(ptr) as i32 }
    }

    /// Clears all tools.
    /// No-op if the toolbar has been destroyed.
    pub fn clear_tools(&self) {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_AuiToolBar_ClearTools(ptr) };
    }

    /// Deletes a tool.
    /// Returns false if the toolbar has been destroyed.
    pub fn delete_tool(&self, tool_id: i32) -> bool {
        let ptr = self.toolbar_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_AuiToolBar_DeleteTool(ptr, tool_id as c_int) }
    }

    /// Returns the underlying WindowHandle for this toolbar.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
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
        }
        AuiToolBar::from_ptr(ptr)
    }
);

// Manual WxWidget implementation for AuiToolBar (using WindowHandle)
impl WxWidget for AuiToolBar {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for AuiToolBar {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for AuiToolBar {}

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
