use crate::event::{Event, EventType, WxEvtHandler};
use crate::prelude::*;
use crate::window::Window;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use wxdragon_sys as ffi;

/// Direction for docking panes in an AuiManager
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DockDirection {
    /// Dock on the left side of the managed window
    Left = 0,
    /// Dock on the right side of the managed window
    Right = 1,
    /// Dock at the top of the managed window
    Top = 2,
    /// Dock at the bottom of the managed window
    Bottom = 3,
    /// Dock in the center of the managed window
    Center = 4,
}

/// Information about a pane in the AuiManager
#[derive(Debug)]
pub struct PaneInfo {
    ptr: *mut ffi::wxd_AuiPaneInfo_t,
}

impl Default for PaneInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl PaneInfo {
    /// Create a new PaneInfo
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_AuiPaneInfo_Create() };
        if ptr.is_null() {
            panic!("Failed to create AuiPaneInfo");
        }
        PaneInfo { ptr }
    }

    /// Set the name for this pane
    pub fn with_name(self, name: &str) -> Self {
        let c_name = CString::new(name).expect("CString::new failed for name");
        unsafe {
            ffi::wxd_AuiPaneInfo_Name(self.ptr, c_name.as_ptr());
        }
        self
    }

    /// Set the caption (title) for this pane
    pub fn with_caption(self, caption: &str) -> Self {
        let c_caption = CString::new(caption).expect("CString::new failed for caption");
        unsafe {
            ffi::wxd_AuiPaneInfo_Caption(self.ptr, c_caption.as_ptr());
        }
        self
    }

    /// Dock this pane on the left side
    pub fn left(self) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Left(self.ptr);
        }
        self
    }

    /// Dock this pane on the right side
    pub fn right(self) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Right(self.ptr);
        }
        self
    }

    /// Dock this pane at the top
    pub fn top(self) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Top(self.ptr);
        }
        self
    }

    /// Dock this pane at the bottom
    pub fn bottom(self) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Bottom(self.ptr);
        }
        self
    }

    /// Dock this pane in the center
    pub fn center(self) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Center(self.ptr);
        }
        self
    }

    /// Make this pane the center pane (main content)
    pub fn center_pane(self) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_CenterPane(self.ptr);
        }
        self
    }

    /// Set whether this pane can be floated
    pub fn floatable(self, enable: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Floatable(self.ptr, enable);
        }
        self
    }

    /// Set whether this pane can be docked
    pub fn dockable(self, enable: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Dockable(self.ptr, enable);
        }
        self
    }

    /// Set whether this pane can be moved
    pub fn movable(self, enable: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Movable(self.ptr, enable);
        }
        self
    }

    /// Set whether this pane can be resized
    pub fn resizable(self, enable: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Resizable(self.ptr, enable);
        }
        self
    }

    /// Set whether this pane has a close button
    pub fn close_button(self, visible: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_CloseButton(self.ptr, visible);
        }
        self
    }

    /// Set whether this pane has a maximize button
    pub fn maximize_button(self, visible: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_MaximizeButton(self.ptr, visible);
        }
        self
    }

    /// Set whether this pane has a minimize button
    pub fn minimize_button(self, visible: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_MinimizeButton(self.ptr, visible);
        }
        self
    }

    /// Set whether this pane has a pin button
    pub fn pin_button(self, visible: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_PinButton(self.ptr, visible);
        }
        self
    }

    /// Set whether this pane has a border
    pub fn pane_border(self, visible: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_PaneBorder(self.ptr, visible);
        }
        self
    }

    /// Set whether this pane has a gripper
    pub fn gripper(self, visible: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Gripper(self.ptr, visible);
        }
        self
    }

    /// Set whether the gripper is at the top
    pub fn gripper_top(self, attop: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_GripperTop(self.ptr, attop);
        }
        self
    }

    /// Set the layer for this pane
    pub fn layer(self, layer: i32) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Layer(self.ptr, layer);
        }
        self
    }

    /// Set the minimum size for this pane
    pub fn min_size(self, width: i32, height: i32) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_MinSize(self.ptr, width, height);
        }
        self
    }

    /// Set the maximum size for this pane
    pub fn max_size(self, width: i32, height: i32) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_MaxSize(self.ptr, width, height);
        }
        self
    }

    /// Set the row position for this pane
    pub fn row(self, row: i32) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Row(self.ptr, row);
        }
        self
    }

    /// Set the position for this pane
    pub fn position(self, position: i32) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Position(self.ptr, position);
        }
        self
    }

    /// Set default properties for this pane
    pub fn default_pane(self) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_DefaultPane(self.ptr);
        }
        self
    }

    /// Set properties for a toolbar pane
    pub fn toolbar_pane(self) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_ToolbarPane(self.ptr);
        }
        self
    }

    /// Set the best size for this pane
    pub fn best_size(self, width: i32, height: i32) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_BestSize(self.ptr, width, height);
        }
        self
    }

    /// Set whether this pane is shown
    pub fn show(self, show: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Show(self.ptr, show);
        }
        self
    }

    /// Hide this pane
    pub fn hide(self) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_Hide(self.ptr);
        }
        self
    }

    /// Set whether the caption is visible for this pane
    pub fn caption_visible(self, visible: bool) -> Self {
        unsafe {
            ffi::wxd_AuiPaneInfo_CaptionVisible(self.ptr, visible);
        }
        self
    }
}

impl Drop for PaneInfo {
    fn drop(&mut self) {
        // Note: There is a potential memory management issue here.
        // When a PaneInfo is added to the AuiManager via add_pane_with_info,
        // the wxWidgets C++ side makes a copy of the pane info.
        // We need to be careful about deleting the original here,
        // but this is necessary to avoid leaks for PaneInfo objects
        // that aren't added to a manager.
        unsafe {
            ffi::wxd_AuiPaneInfo_Delete(self.ptr);
        }
    }
}

/// Builder for AuiManager that ensures it's always attached to a window
pub struct AuiManagerBuilder<'a> {
    parent_ptr: *mut ffi::wxd_Window_t,
    _marker: PhantomData<&'a ()>,
}

impl<'a> AuiManagerBuilder<'a> {
    /// Build the AuiManager with the configured parent window
    pub fn build(self) -> AuiManager {
        let ptr = unsafe { ffi::wxd_AuiManager_Create() };
        if ptr.is_null() {
            panic!("Failed to create AuiManager");
        }

        let mgr = AuiManager {
            ptr,
            _marker: PhantomData,
        };

        // Immediately set the managed window to ensure proper lifecycle management
        unsafe {
            ffi::wxd_AuiManager_SetManagedWindow(mgr.ptr, self.parent_ptr);
        }

        mgr
    }
}

/// AuiManager - Advanced User Interface manager for docking windows and toolbars
///
/// The AuiManager is responsible for managing the layout of windows within a frame.
/// It allows windows to be "docked" into different regions of the frame and provides
/// a draggable, floating interface for rearranging windows.
#[derive(Debug)]
pub struct AuiManager {
    ptr: *mut ffi::wxd_AuiManager_t,
    _marker: PhantomData<()>,
}

// Implement WxEvtHandler for AuiManager to allow event binding
impl WxEvtHandler for AuiManager {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

impl AuiManager {
    /// Create a new AuiManager builder, which requires a parent window to build
    pub fn builder(parent: &impl WxWidget) -> AuiManagerBuilder<'_> {
        AuiManagerBuilder {
            parent_ptr: parent.handle_ptr(),
            _marker: PhantomData,
        }
    }

    /// Set the window that this AuiManager will manage
    pub fn set_managed_window(&self, window: &impl WxWidget) {
        unsafe {
            ffi::wxd_AuiManager_SetManagedWindow(self.ptr, window.handle_ptr());
        }
    }

    /// Get the window that this AuiManager is managing
    pub fn get_managed_window(&self) -> Option<Window> {
        let ptr = unsafe { ffi::wxd_AuiManager_GetManagedWindow(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Window::from_ptr(ptr) })
        }
    }

    /// Uninitialize the manager (detaches from the managed window)
    pub fn uninit(&self) {
        unsafe {
            ffi::wxd_AuiManager_UnInit(self.ptr);
        }
    }

    /// Add a pane to the manager with a simple direction
    pub fn add_pane(
        &self,
        window: &impl WxWidget,
        direction: DockDirection,
        caption: &str,
    ) -> bool {
        let c_caption = CString::new(caption).expect("CString::new failed for caption");
        unsafe {
            ffi::wxd_AuiManager_AddPane(
                self.ptr,
                window.handle_ptr(),
                direction as i32,
                c_caption.as_ptr(),
            )
        }
    }

    /// Add a pane with detailed pane information
    pub fn add_pane_with_info(&self, window: &impl WxWidget, pane_info: PaneInfo) -> bool {
        // The pane_info is still managed by Rust and will be dropped automatically
        unsafe { ffi::wxd_AuiManager_AddPaneWithInfo(self.ptr, window.handle_ptr(), pane_info.ptr) }
    }

    /// Update the manager's layout (must be called after adding/removing panes)
    pub fn update(&self) -> bool {
        unsafe { ffi::wxd_AuiManager_Update(self.ptr) }
    }

    /// Save the current layout as a perspective string
    pub fn save_perspective(&self) -> String {
        let len = unsafe { ffi::wxd_AuiManager_SavePerspective(self.ptr, std::ptr::null_mut(), 0) };
        if len == 0 {
            return String::new();
        }

        // Create a buffer to hold the perspective string
        let mut b = vec![0; len + 1]; // +1 for null terminator
        unsafe { ffi::wxd_AuiManager_SavePerspective(self.ptr, b.as_mut_ptr(), b.len()) };
        unsafe { CStr::from_ptr(b.as_ptr()).to_string_lossy().to_string() }
    }

    /// Load a perspective from a string
    pub fn load_perspective(&self, perspective: &str, update: bool) -> bool {
        let c_perspective = CString::new(perspective).expect("CString::new failed for perspective");
        unsafe { ffi::wxd_AuiManager_LoadPerspective(self.ptr, c_perspective.as_ptr(), update) }
    }

    /// Detach a pane from the manager
    pub fn detach_pane(&self, window: &impl WxWidget) -> bool {
        unsafe { ffi::wxd_AuiManager_DetachPane(self.ptr, window.handle_ptr()) }
    }
}

impl Drop for AuiManager {
    fn drop(&mut self) {
        // We need to have special handling for AuiManager
        // Do not call delete directly as it can cause issues with dragging
        // This is intentionally left empty to prevent premature cleanup
        // wxWidgets will handle resource cleanup when the managed window is destroyed

        // Note: The original implementation was:
        // unsafe { ffi::wxd_AuiManager_Delete(self.ptr); }
        // But this caused issues with pane dragging
    }
}

// Re-export PaneInfo to make it easier to use
pub use PaneInfo as AuiPaneInfo;

// Add enum for AuiManager events
/// Events specific to AuiManager
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuiManagerEvent {
    /// Fired when a button is clicked on a pane
    PaneButton,
    /// Fired when a pane close button is clicked
    PaneClose,
    /// Fired when a pane is maximized
    PaneMaximize,
    /// Fired when a maximized pane is restored
    PaneRestore,
    /// Fired when a pane is activated
    PaneActivated,
    /// Fired when the AUI manager is rendering
    Render,
}

/// Event data for AuiManager events
#[derive(Debug)]
pub struct AuiManagerEventData {
    /// The raw event from wxWidgets
    event: Event,
}

impl AuiManagerEventData {
    /// Create a new AuiManagerEventData from an Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Gets the ID associated with this event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Gets the pane affected by this event.
    /// This will return the Window associated with the pane if available.
    pub fn get_pane(&self) -> Option<Window> {
        self.event.get_event_object()
    }

    /// Skip this event (allow default processing to occur)
    pub fn skip(&self) {
        self.event.skip(true);
    }
}

// Implement event handling for AuiManager
impl AuiManager {
    /// Bind a handler for the pane button event
    pub fn on_pane_button<F>(&self, callback: F)
    where
        F: FnMut(AuiManagerEventData) + 'static,
    {
        self.bind_aui_event(EventType::AUI_PANE_BUTTON, callback);
    }

    /// Bind a handler for the pane close event
    pub fn on_pane_close<F>(&self, callback: F)
    where
        F: FnMut(AuiManagerEventData) + 'static,
    {
        self.bind_aui_event(EventType::AUI_PANE_CLOSE, callback);
    }

    /// Bind a handler for the pane maximize event
    pub fn on_pane_maximize<F>(&self, callback: F)
    where
        F: FnMut(AuiManagerEventData) + 'static,
    {
        self.bind_aui_event(EventType::AUI_PANE_MAXIMIZE, callback);
    }

    /// Bind a handler for the pane restore event
    pub fn on_pane_restore<F>(&self, callback: F)
    where
        F: FnMut(AuiManagerEventData) + 'static,
    {
        self.bind_aui_event(EventType::AUI_PANE_RESTORE, callback);
    }

    /// Bind a handler for the pane activated event
    pub fn on_pane_activated<F>(&self, callback: F)
    where
        F: FnMut(AuiManagerEventData) + 'static,
    {
        self.bind_aui_event(EventType::AUI_PANE_ACTIVATED, callback);
    }

    /// Bind a handler for the render event
    pub fn on_render<F>(&self, callback: F)
    where
        F: FnMut(AuiManagerEventData) + 'static,
    {
        self.bind_aui_event(EventType::AUI_RENDER, callback);
    }

    // Internal helper to bind AUI events
    fn bind_aui_event<F>(&self, event_type: EventType, mut callback: F)
    where
        F: FnMut(AuiManagerEventData) + 'static,
    {
        self.bind_internal(event_type, move |event| {
            let data = AuiManagerEventData::new(event);
            callback(data);
        });
    }
}
