use crate::bitmap::Bitmap;
use crate::bitmap_bundle::BitmapBundle;
#[cfg(any(target_os = "windows", target_os = "linux"))]
use crate::event::EventType;
use crate::event::WxEvtHandler;
#[cfg(any(target_os = "windows", target_os = "linux"))]
use crate::event::taskbar_events::{TaskBarIconEvent, TaskBarIconEventData};

use crate::menus::menu::Menu;

use std::ffi::CString;
use std::ptr;
use wxdragon_sys as ffi;

/// Represents the type of taskbar icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskBarIconType {
    /// Dock icon (typically on macOS)
    Dock,
    /// Default taskbar icon type
    #[default]
    Default,
    /// Custom status item (typically on macOS)
    CustomStatusItem,
}

impl TaskBarIconType {
    /// Convert to C enum value
    pub fn to_c_enum(self) -> ffi::wxd_TaskBarIconType_t {
        // Map to our C wrapper's enum values (wxd_taskbar.h):
        //   WXD_TASKBAR_ICON_DEFAULT = 0
        //   WXD_TASKBAR_ICON_DOCK = 1
        //   WXD_TASKBAR_ICON_CUSTOM_STATUSITEM = 2
        // These constants aren't exposed by bindgen, so use numeric values directly.
        match self {
            TaskBarIconType::Default => 0,
            TaskBarIconType::Dock => 1,
            TaskBarIconType::CustomStatusItem => 2,
        }
    }
}

/// Represents a wxTaskBarIcon - a system tray/taskbar icon.
///
/// TaskBarIcon allows applications to add an icon to the system tray or taskbar
/// that can display tooltips, show balloon messages, and respond to mouse events.
/// It can also display popup menus when clicked.
///
/// # Lifetime Management
///
/// TaskBarIcon has special lifetime semantics unlike regular widgets. Once created
/// and an icon is set, it will persist in the system tray until explicitly removed
/// or the application exits. The Rust wrapper can go out of scope without affecting
/// the visible icon.
///
/// For proper cleanup, call `remove_icon()` when you no longer need the taskbar icon,
/// or use `destroy()` for complete cleanup.
///
/// # Platform Notes
/// - Windows: Shows in the system tray (notification area)
/// - macOS: Shows in the dock or menu bar (depending on type)
/// - Linux: Shows in the system tray (implementation varies by desktop environment)
///
/// # Example
/// ```no_run
/// use wxdragon::widgets::TaskBarIcon;
/// use wxdragon::bitmap::Bitmap;
/// use wxdragon::TaskBarIconType;
///
/// let taskbar = TaskBarIcon::builder()
///     .with_icon_type(TaskBarIconType::Default)
///     .build();
///
/// // Set an icon with tooltip
/// if let Some(icon) = Bitmap::new(32, 32) {
///     taskbar.set_icon(&icon, "My Application");
/// }
///
/// // Handle right-click events (Windows only)
/// #[cfg(target_os = "windows")]
/// taskbar.on_right_up(|_event| {
///     println!("Taskbar icon right-clicked!");
/// });
///
/// // Clean up when done (optional - will be cleaned up on app exit)
/// taskbar.remove_icon();
/// ```
#[derive(Clone)]
pub struct TaskBarIcon {
    ptr: *mut ffi::wxd_TaskBarIcon_t,
}

impl TaskBarIcon {
    /// Creates a new `TaskBarIconBuilder` for constructing a taskbar icon.
    pub fn builder() -> TaskBarIconBuilder {
        TaskBarIconBuilder::new()
    }

    /// Creates a new TaskBarIcon (low-level constructor used by the builder).
    fn new_impl(icon_type: TaskBarIconType) -> Self {
        let ptr = unsafe { ffi::wxd_TaskBarIcon_Create(icon_type.to_c_enum()) };

        if ptr.is_null() {
            panic!("Failed to create TaskBarIcon widget");
        }

        TaskBarIcon { ptr }
    }

    /// Sets the taskbar icon and tooltip.
    ///
    /// # Arguments
    /// * `icon` - The bitmap to display as the taskbar icon
    /// * `tooltip` - Optional tooltip text to show when hovering over the icon
    ///
    /// # Returns
    /// `true` if the icon was set successfully, `false` otherwise.
    pub fn set_icon(&self, icon: &Bitmap, tooltip: &str) -> bool {
        let c_tooltip = CString::new(tooltip).expect("CString::new failed");
        unsafe { ffi::wxd_TaskBarIcon_SetIcon(self.ptr, icon.as_const_ptr(), c_tooltip.as_ptr()) }
    }

    /// Sets the taskbar icon using a bitmap bundle and tooltip.
    ///
    /// # Arguments
    /// * `icon_bundle` - The bitmap bundle to display as the taskbar icon
    /// * `tooltip` - Optional tooltip text to show when hovering over the icon
    ///
    /// # Returns
    /// `true` if the icon was set successfully, `false` otherwise.
    pub fn set_icon_bundle(&self, icon_bundle: &BitmapBundle, tooltip: &str) -> bool {
        let c_tooltip = CString::new(tooltip).expect("CString::new failed");
        unsafe { ffi::wxd_TaskBarIcon_SetIconBundle(self.ptr, icon_bundle.as_ptr(), c_tooltip.as_ptr()) }
    }

    /// Removes the taskbar icon.
    ///
    /// # Returns
    /// `true` if the icon was removed successfully, `false` otherwise.
    pub fn remove_icon(&self) -> bool {
        unsafe { ffi::wxd_TaskBarIcon_RemoveIcon(self.ptr) }
    }

    /// Checks if the taskbar icon is currently installed/visible.
    ///
    /// # Returns
    /// `true` if an icon is currently installed, `false` otherwise.
    pub fn is_icon_installed(&self) -> bool {
        unsafe { ffi::wxd_TaskBarIcon_IsIconInstalled(self.ptr) }
    }

    /// Shows a balloon tooltip (Windows only).
    ///
    /// # Arguments
    /// * `title` - The title of the balloon tooltip
    /// * `text` - The text content of the balloon tooltip
    /// * `timeout` - Timeout in milliseconds (0 for default)
    /// * `flags` - Icon flags (e.g., wxICON_INFORMATION, wxICON_WARNING, wxICON_ERROR)
    /// * `icon` - Optional custom icon for the balloon
    ///
    /// # Returns
    /// `true` if the balloon was shown successfully, `false` otherwise.
    ///
    /// # Platform Notes
    /// This feature is only available on Windows. On other platforms, this method
    /// will return `false`.
    pub fn show_balloon(&self, title: &str, text: &str, timeout: u32, flags: i32, icon: Option<&BitmapBundle>) -> bool {
        let c_title = CString::new(title).expect("CString::new failed");
        let c_text = CString::new(text).expect("CString::new failed");
        let icon_ptr = icon.map(|i| i.as_ptr()).unwrap_or(ptr::null_mut());

        unsafe { ffi::wxd_TaskBarIcon_ShowBalloon(self.ptr, c_title.as_ptr(), c_text.as_ptr(), timeout, flags, icon_ptr) }
    }

    /// Shows a popup menu at the current mouse position.
    ///
    /// # Arguments
    /// * `menu` - The menu to display
    ///
    /// # Returns
    /// `true` if the menu was shown successfully, `false` otherwise.
    ///
    /// # Note
    /// This method manually shows a popup menu at the current mouse position.
    /// For automatic popup menus when the taskbar icon is clicked, use `set_popup_menu()` instead.
    ///
    /// # Example
    /// ```no_run
    /// # let taskbar: wxdragon::widgets::TaskBarIcon = unimplemented!();
    /// use wxdragon::menus::Menu;
    ///
    /// let mut menu = Menu::builder().with_title("Popup Menu")
    ///     .append_item(101, "Option 1", "First option")
    ///     .append_item(102, "Option 2", "Second option")
    ///     .append_separator()
    ///     .append_item(103, "Exit", "Exit application")
    ///     .build();
    /// taskbar.popup_menu(&mut menu);
    /// ```
    pub fn popup_menu(&self, menu: &mut Menu) -> bool {
        unsafe { ffi::wxd_TaskBarIcon_PopupMenu(self.ptr, menu.as_mut_ptr()) }
    }

    /// Sets a menu that will be automatically displayed when the taskbar icon is clicked.
    ///
    /// This is the standard way to provide popup menus for TaskBarIcon. When the user
    /// right-clicks (or in some cases left-clicks) the taskbar icon, the menu will be
    /// displayed automatically.
    ///
    /// # Arguments
    /// * `menu` - The menu to display automatically when the icon is clicked
    ///
    /// # Platform Notes
    /// - **Windows**: Menu appears on right-click
    /// - **macOS**: Menu appears on click (behavior depends on icon type)
    /// - **Linux**: Menu appears on right-click (behavior varies by desktop environment)
    ///
    /// # Example
    /// ```no_run
    /// use wxdragon::menus::Menu;
    /// use wxdragon::widgets::TaskBarIcon;
    ///
    /// let taskbar: TaskBarIcon = TaskBarIcon::builder().build();
    ///
    /// // Create popup menu
    /// let mut menu = Menu::builder().with_title("Taskbar Menu")
    ///     .append_item(101, "Show Window", "Show the main window")
    ///     .append_item(102, "Settings", "Open settings")
    ///     .append_separator()
    ///     .append_item(103, "Exit", "Exit application")
    ///     .build();
    ///
    /// // Set as automatic popup menu
    /// taskbar.set_popup_menu(&mut menu);
    ///
    /// // Now the menu will appear automatically when the taskbar icon is clicked
    /// ```
    pub fn set_popup_menu(&self, menu: &mut Menu) {
        unsafe { ffi::wxd_TaskBarIcon_SetPopupMenu(self.ptr, menu.as_mut_ptr()) }
    }

    /// Gets the currently set automatic popup menu.
    ///
    /// # Returns
    /// `Some(Menu)` if a popup menu is set, `None` otherwise.
    ///
    /// # Note
    /// The returned menu is a reference to the original menu set with `set_popup_menu()`.
    /// Modifying this menu will affect the popup menu behavior.
    pub fn get_popup_menu(&self) -> Option<Menu> {
        let menu_ptr = unsafe { ffi::wxd_TaskBarIcon_GetPopupMenu(self.ptr) };
        if menu_ptr.is_null() {
            None
        } else {
            Some(Menu::from(menu_ptr))
        }
    }

    /// Explicitly destroys the TaskBarIcon and removes it from the system tray.
    ///
    /// This method provides complete cleanup of the TaskBarIcon, removing it from
    /// the system tray and freeing its resources. Unlike `remove_icon()` which only
    /// hides the icon, this method destroys the entire TaskBarIcon object.
    ///
    /// After calling this method, the TaskBarIcon should not be used further.
    ///
    /// # Example
    /// ```no_run
    /// use wxdragon::widgets::TaskBarIcon;
    /// let taskbar = TaskBarIcon::builder().build();
    /// let icon = wxdragon::Bitmap::new(100, 100).unwrap();
    /// taskbar.set_icon(&icon, "My App");
    ///
    /// // Later, when shutting down:
    /// taskbar.destroy();
    /// ```
    pub fn destroy(&self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_TaskBarIcon_Destroy(self.ptr) };
        }
    }

    /// Returns the raw pointer to the underlying wxTaskBarIcon object.
    pub fn as_ptr(&self) -> *mut ffi::wxd_TaskBarIcon_t {
        self.ptr
    }
}

impl Drop for TaskBarIcon {
    fn drop(&mut self) {
        // TaskBarIcon has special lifetime semantics - it should persist
        // until explicitly removed or the application exits.
        // Unlike regular child widgets, TaskBarIcon needs to stay alive independently
        // of its Rust wrapper to remain visible in the system tray.
        //
        // We intentionally do not destroy the C++ object here to prevent
        // the icon from disappearing when the Rust wrapper goes out of scope.
        // The icon will be cleaned up automatically when the application exits,
        // or can be explicitly removed by calling remove_icon().
        //
        // This follows the same pattern as AuiManager which also has special
        // lifetime requirements.

        // Note: This means TaskBarIcon objects should be used carefully to avoid
        // memory leaks. Use remove_icon() to explicitly clean up when needed.
    }
}

impl WxEvtHandler for TaskBarIcon {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        unsafe { ffi::wxd_TaskBarIcon_GetEvtHandler(self.ptr) }
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for TaskBarIcon {}

// Add menu event handling to TaskBarIcon
impl TaskBarIcon {
    /// Bind a handler to menu events from the TaskBarIcon's popup menu
    pub fn on_menu<F>(&self, handler: F)
    where
        F: FnMut(crate::event::Event) + 'static,
    {
        <Self as crate::event::WxEvtHandler>::bind_internal(self, crate::event::EventType::MENU, handler);
    }
}

// Manual event implementation for TaskBarIcon due to platform-specific events
impl TaskBarIcon {
    /// Internal binding method for TaskBarIcon events
    ///
    /// Only available on Windows and Linux where TaskBarIcon events are supported.
    #[doc(hidden)]
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    pub(crate) fn bind_taskbar_event<F>(&self, event: TaskBarIconEvent, mut callback: F)
    where
        F: FnMut(TaskBarIconEventData) + 'static,
    {
        // Map enum variant to EventType
        let event_type = match event {
            TaskBarIconEvent::LeftDown => EventType::TASKBAR_LEFT_DOWN,
            TaskBarIconEvent::LeftDoubleClick => EventType::TASKBAR_LEFT_DCLICK,

            #[cfg(target_os = "windows")]
            TaskBarIconEvent::Move => EventType::TASKBAR_MOVE,
            #[cfg(target_os = "windows")]
            TaskBarIconEvent::LeftUp => EventType::TASKBAR_LEFT_UP,
            #[cfg(target_os = "windows")]
            TaskBarIconEvent::RightDown => EventType::TASKBAR_RIGHT_DOWN,
            #[cfg(target_os = "windows")]
            TaskBarIconEvent::RightUp => EventType::TASKBAR_RIGHT_UP,
            #[cfg(target_os = "windows")]
            TaskBarIconEvent::RightDoubleClick => EventType::TASKBAR_RIGHT_DCLICK,
            #[cfg(target_os = "windows")]
            TaskBarIconEvent::BalloonTimeout => EventType::TASKBAR_BALLOON_TIMEOUT,
            #[cfg(target_os = "windows")]
            TaskBarIconEvent::BalloonClick => EventType::TASKBAR_BALLOON_CLICK,
        };

        // Create wrapper
        let wrapper = move |event: crate::event::Event| {
            let typed_event = TaskBarIconEventData::new(event);
            callback(typed_event);
        };

        // Use internal bind method
        crate::event::WxEvtHandler::bind_internal(self, event_type, wrapper);
    }

    /// Binds a handler to taskbar icon left mouse button down
    ///
    /// Note: Only available on Windows and Linux. Not supported on macOS.
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    pub fn on_left_down<F>(&self, callback: F)
    where
        F: FnMut(TaskBarIconEventData) + 'static,
    {
        self.bind_taskbar_event(TaskBarIconEvent::LeftDown, callback)
    }

    /// Binds a handler to taskbar icon left mouse button double-click
    ///
    /// Note: Only available on Windows and Linux. Not supported on macOS.
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    pub fn on_left_double_click<F>(&self, callback: F)
    where
        F: FnMut(TaskBarIconEventData) + 'static,
    {
        self.bind_taskbar_event(TaskBarIconEvent::LeftDoubleClick, callback)
    }
}

// Windows-only event handlers
#[cfg(target_os = "windows")]
impl TaskBarIcon {
    /// Binds a handler to taskbar icon mouse movement (Windows only)
    pub fn on_move<F>(&self, callback: F)
    where
        F: FnMut(TaskBarIconEventData) + 'static,
    {
        self.bind_taskbar_event(TaskBarIconEvent::Move, callback)
    }

    /// Binds a handler to taskbar icon left mouse button up (Windows only)
    pub fn on_left_up<F>(&self, callback: F)
    where
        F: FnMut(TaskBarIconEventData) + 'static,
    {
        self.bind_taskbar_event(TaskBarIconEvent::LeftUp, callback)
    }

    /// Binds a handler to taskbar icon right mouse button down (Windows only)
    pub fn on_right_down<F>(&self, callback: F)
    where
        F: FnMut(TaskBarIconEventData) + 'static,
    {
        self.bind_taskbar_event(TaskBarIconEvent::RightDown, callback)
    }

    /// Binds a handler to taskbar icon right mouse button up (Windows only)
    pub fn on_right_up<F>(&self, callback: F)
    where
        F: FnMut(TaskBarIconEventData) + 'static,
    {
        self.bind_taskbar_event(TaskBarIconEvent::RightUp, callback)
    }

    /// Binds a handler to taskbar icon right mouse button double-click (Windows only)
    pub fn on_right_double_click<F>(&self, callback: F)
    where
        F: FnMut(TaskBarIconEventData) + 'static,
    {
        self.bind_taskbar_event(TaskBarIconEvent::RightDoubleClick, callback)
    }

    /// Binds a handler to taskbar icon balloon timeout (Windows only)
    pub fn on_balloon_timeout<F>(&self, callback: F)
    where
        F: FnMut(TaskBarIconEventData) + 'static,
    {
        self.bind_taskbar_event(TaskBarIconEvent::BalloonTimeout, callback)
    }

    /// Binds a handler to taskbar icon balloon click (Windows only)
    pub fn on_balloon_click<F>(&self, callback: F)
    where
        F: FnMut(TaskBarIconEventData) + 'static,
    {
        self.bind_taskbar_event(TaskBarIconEvent::BalloonClick, callback)
    }
}

/// Builder for creating TaskBarIcon instances.
#[derive(Clone)]
pub struct TaskBarIconBuilder {
    icon_type: TaskBarIconType,
    initial_icon: Option<Bitmap>,
    initial_icon_bundle: Option<BitmapBundle>,
    initial_tooltip: String,
}

impl TaskBarIconBuilder {
    /// Creates a new TaskBarIconBuilder with default settings.
    pub fn new() -> Self {
        Self {
            icon_type: TaskBarIconType::Default,
            initial_icon: None,
            initial_icon_bundle: None,
            initial_tooltip: String::new(),
        }
    }

    /// Sets the taskbar icon type.
    pub fn with_icon_type(mut self, icon_type: TaskBarIconType) -> Self {
        self.icon_type = icon_type;
        self
    }

    /// Sets an initial icon to display.
    pub fn with_icon(mut self, icon: Bitmap) -> Self {
        self.initial_icon = Some(icon);
        self
    }

    /// Sets an initial icon bundle to display.
    pub fn with_icon_bundle(mut self, icon_bundle: BitmapBundle) -> Self {
        self.initial_icon_bundle = Some(icon_bundle);
        self
    }

    /// Sets the initial tooltip text.
    pub fn with_tooltip(mut self, tooltip: &str) -> Self {
        self.initial_tooltip = tooltip.to_string();
        self
    }

    /// Builds the TaskBarIcon.
    pub fn build(self) -> TaskBarIcon {
        let taskbar = TaskBarIcon::new_impl(self.icon_type);

        // Set initial icon if provided
        if let Some(ref bundle) = self.initial_icon_bundle {
            taskbar.set_icon_bundle(bundle, &self.initial_tooltip);
        } else if let Some(ref icon) = self.initial_icon {
            taskbar.set_icon(icon, &self.initial_tooltip);
        }

        taskbar
    }
}

impl Default for TaskBarIconBuilder {
    fn default() -> Self {
        Self::new()
    }
}

widget_style_enum!(
    name: TaskBarIconStyle,
    doc: "Style flags for `TaskBarIcon` (currently no specific styles).",
    variants: {
        Default: 0, "Default style (no special behavior)."
    },
    default_variant: Default
);
