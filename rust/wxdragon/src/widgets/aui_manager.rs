use crate::event::{Event, EventType, WxEvtHandler};
use crate::window::{Window, WindowHandle, WxWidget};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use wxdragon_sys as ffi;

// ============================================================================
// AuiManagerHandle: Safe, Copy-able handle to wxWidgets AuiManager
// ============================================================================

/// Counter for generating unique AuiManager handle IDs
static NEXT_AUI_HANDLE_ID: AtomicU64 = AtomicU64::new(1);

thread_local! {
    /// Maps handle IDs to raw AuiManager pointers
    static AUI_MANAGER_REGISTRY: RefCell<HashMap<u64, *mut ffi::wxd_AuiManager_t>> =
        RefCell::new(HashMap::new());
}

/// A safe, Copy-able handle to a wxAuiManager.
///
/// Unlike raw pointers, `AuiManagerHandle` tracks the manager's lifecycle
/// through its managed window. When the managed window is destroyed, the handle
/// becomes invalid.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct AuiManagerHandle(u64);

impl AuiManagerHandle {
    /// Creates a new handle for an AuiManager pointer.
    ///
    /// # Safety
    /// The caller must ensure `ptr` points to a valid wxAuiManager.
    fn new(ptr: *mut ffi::wxd_AuiManager_t) -> Self {
        if ptr.is_null() {
            return AuiManagerHandle(0); // Invalid handle for null pointers
        }

        let handle_id = NEXT_AUI_HANDLE_ID.fetch_add(1, Ordering::SeqCst);

        // Register the manager pointer
        AUI_MANAGER_REGISTRY.with(|r| r.borrow_mut().insert(handle_id, ptr));

        AuiManagerHandle(handle_id)
    }

    /// Get the raw pointer if the manager is still valid, `None` if destroyed.
    #[inline]
    fn get_ptr(&self) -> Option<*mut ffi::wxd_AuiManager_t> {
        if self.0 == 0 {
            return None;
        }
        AUI_MANAGER_REGISTRY.with(|r| r.borrow().get(&self.0).copied())
    }

    /// Check if the underlying manager is still valid (not destroyed).
    #[inline]
    fn is_valid(&self) -> bool {
        self.get_ptr().is_some()
    }

    /// Internal: Remove handle from registry (called when manager should be invalidated)
    fn invalidate(&self) {
        if self.0 == 0 {
            return;
        }
        AUI_MANAGER_REGISTRY.with(|r| {
            r.borrow_mut().remove(&self.0);
        });
    }
}

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
    parent_handle: WindowHandle,
    _marker: PhantomData<&'a ()>,
}

impl<'a> AuiManagerBuilder<'a> {
    /// Build the AuiManager with the configured parent window
    pub fn build(self) -> AuiManager {
        let ptr = unsafe { ffi::wxd_AuiManager_Create() };
        if ptr.is_null() {
            panic!("Failed to create AuiManager");
        }

        // Create the handle for the manager
        let handle = AuiManagerHandle::new(ptr);

        let mgr = AuiManager {
            handle,
            managed_window: self.parent_handle,
        };

        // Immediately set the managed window to ensure proper lifecycle management
        unsafe {
            ffi::wxd_AuiManager_SetManagedWindow(ptr, self.parent_ptr);
        }

        // Set up a destroy handler on the managed window to invalidate this manager
        let handle_copy = handle;
        let parent = unsafe { Window::from_ptr(self.parent_ptr) };
        parent.bind_internal(EventType::DESTROY, move |_event| {
            handle_copy.invalidate();
        });

        mgr
    }
}

/// AuiManager - Advanced User Interface manager for docking windows and toolbars
///
/// The AuiManager is responsible for managing the layout of windows within a frame.
/// It allows windows to be "docked" into different regions of the frame and provides
/// a draggable, floating interface for rearranging windows.
///
/// AuiManager uses a handle-based pattern for memory safety. When the managed window
/// is destroyed, the handle becomes invalid and all operations become safe no-ops.
///
/// # Example
/// ```ignore
/// let frame = Frame::builder().build();
/// let manager = AuiManager::builder(&frame).build();
///
/// // AuiManager is Copy - no clone needed for closures!
/// manager.on_pane_close(move |_| {
///     // Safe: if frame was destroyed, this is a no-op
///     manager.update();
/// });
///
/// // After frame destruction, manager operations are safe no-ops
/// frame.destroy();
/// assert!(!manager.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct AuiManager {
    /// Safe handle to the underlying wxAuiManager - automatically invalidated when managed window is destroyed
    handle: AuiManagerHandle,
    /// Handle to the managed window - used to track lifecycle
    managed_window: WindowHandle,
}

impl AuiManager {
    /// Create a new AuiManager builder, which requires a parent window to build
    pub fn builder(parent: &impl WxWidget) -> AuiManagerBuilder<'_> {
        let parent_ptr = parent.handle_ptr();
        // Try to look up existing handle, or create a new one if needed
        let parent_handle = WindowHandle::from_ptr(parent_ptr).unwrap_or_else(|| WindowHandle::new(parent_ptr));

        AuiManagerBuilder {
            parent_ptr,
            parent_handle,
            _marker: PhantomData,
        }
    }

    /// Helper to get raw AuiManager pointer, returns null if manager has been destroyed
    #[inline]
    fn manager_ptr(&self) -> *mut ffi::wxd_AuiManager_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    /// Check if the AuiManager is still valid (managed window not destroyed)
    pub fn is_valid(&self) -> bool {
        self.handle.is_valid() && self.managed_window.is_valid()
    }

    /// Set the window that this AuiManager will manage
    /// No-op if the manager has been destroyed.
    pub fn set_managed_window(&self, window: &impl WxWidget) {
        let ptr = self.manager_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_AuiManager_SetManagedWindow(ptr, window.handle_ptr());
        }
    }

    /// Get the window that this AuiManager is managing
    /// Returns None if the manager has been destroyed.
    pub fn get_managed_window(&self) -> Option<Window> {
        let ptr = self.manager_ptr();
        if ptr.is_null() {
            return None;
        }
        let window_ptr = unsafe { ffi::wxd_AuiManager_GetManagedWindow(ptr) };
        if window_ptr.is_null() {
            None
        } else {
            Some(unsafe { Window::from_ptr(window_ptr) })
        }
    }

    /// Uninitialize the manager (detaches from the managed window)
    /// No-op if the manager has been destroyed.
    pub fn uninit(&self) {
        let ptr = self.manager_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_AuiManager_UnInit(ptr);
        }
    }

    /// Add a pane to the manager with a simple direction
    /// Returns false if the manager has been destroyed.
    pub fn add_pane(&self, window: &impl WxWidget, direction: DockDirection, caption: &str) -> bool {
        let ptr = self.manager_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_caption = CString::new(caption).expect("CString::new failed for caption");
        unsafe { ffi::wxd_AuiManager_AddPane(ptr, window.handle_ptr(), direction as i32, c_caption.as_ptr()) }
    }

    /// Add a pane with detailed pane information
    /// Returns false if the manager has been destroyed.
    pub fn add_pane_with_info(&self, window: &impl WxWidget, pane_info: PaneInfo) -> bool {
        let ptr = self.manager_ptr();
        if ptr.is_null() {
            return false;
        }
        // The pane_info is still managed by Rust and will be dropped automatically
        unsafe { ffi::wxd_AuiManager_AddPaneWithInfo(ptr, window.handle_ptr(), pane_info.ptr) }
    }

    /// Update the manager's layout (must be called after adding/removing panes)
    /// Returns false if the manager has been destroyed.
    pub fn update(&self) -> bool {
        let ptr = self.manager_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_AuiManager_Update(ptr) }
    }

    /// Save the current layout as a perspective string
    /// Returns empty string if the manager has been destroyed.
    pub fn save_perspective(&self) -> String {
        let ptr = self.manager_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_AuiManager_SavePerspective(ptr, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }

        // Create a buffer to hold the perspective string
        let mut b = vec![0; len as usize + 1]; // +1 for null terminator
        unsafe { ffi::wxd_AuiManager_SavePerspective(ptr, b.as_mut_ptr(), b.len()) };
        unsafe { CStr::from_ptr(b.as_ptr()).to_string_lossy().to_string() }
    }

    /// Load a perspective from a string
    /// Returns false if the manager has been destroyed.
    pub fn load_perspective(&self, perspective: &str, update: bool) -> bool {
        let ptr = self.manager_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_perspective = CString::new(perspective).expect("CString::new failed for perspective");
        unsafe { ffi::wxd_AuiManager_LoadPerspective(ptr, c_perspective.as_ptr(), update) }
    }

    /// Detach a pane from the manager
    /// Returns false if the manager has been destroyed.
    pub fn detach_pane(&self, window: &impl WxWidget) -> bool {
        let ptr = self.manager_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_AuiManager_DetachPane(ptr, window.handle_ptr()) }
    }
}

// Implement WxEvtHandler for AuiManager to allow event binding
impl WxEvtHandler for AuiManager {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.manager_ptr() as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for AuiManager {}

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
