use crate::bitmap::Bitmap; // ADDED: Import Bitmap
use crate::geometry::{Point, Size};
use crate::id::ID_ANY;
use crate::id::Id;
use crate::menus::MenuBar; // ADDED: Import MenuBar
use crate::widgets::statusbar::StatusBar; // ADDED Import
use crate::widgets::toolbar::{ToolBar, ToolBarStyle}; // Added ToolBarStyle
use crate::window::{WindowHandle, WxWidget};
// Window is used by new_from_composition for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::default::Default;
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::c_int; // Import c_longlong and c_int // ADDED for enum bitwise operations
use std::ptr;
use wxdragon_sys as ffi;

// --- Style enum using macro ---
widget_style_enum!(
    name: FrameStyle,
    doc: "Window style flags for Frame.",
    variants: {
        Default: ffi::WXD_DEFAULT_FRAME_STYLE, "Includes `wxCAPTION`, `wxRESIZE_BORDER`, `wxSYSTEM_MENU`, `wxMINIMIZE_BOX`, `wxMAXIMIZE_BOX`, `wxCLOSE_BOX`. This is the default style.",
        Caption: ffi::WXD_CAPTION, "Displays a title bar.",
        ResizeBorder: ffi::WXD_RESIZE_BORDER, "Displays a resizeable border.",
        SystemMenu: ffi::WXD_SYSTEM_MENU, "Displays a system menu.",
        CloseBox: ffi::WXD_CLOSE_BOX, "Displays a close box.",
        MaximizeBox: ffi::WXD_MAXIMIZE_BOX, "Displays a maximize box.",
        MinimizeBox: ffi::WXD_MINIMIZE_BOX, "Displays a minimize box.",
        StayOnTop: ffi::WXD_STAY_ON_TOP, "Stays on top of other windows.",
        ToolWindow: ffi::WXD_FRAME_TOOL_WINDOW, "Tool window style (typically a thin border and title bar).",
        NoTaskbar: ffi::WXD_FRAME_NO_TASKBAR, "No taskbar button (Windows only).",
        FloatOnParent: ffi::WXD_FRAME_FLOAT_ON_PARENT, "Equivalent to StayOnTop for frames.",
        ClipChildren: ffi::WXD_CLIP_CHILDREN, "Clip children to the frame."
    },
    default_variant: Default
);

/// Represents a wxFrame.
///
/// Frame uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Lifetime Management
/// The main application frame is typically created within the `wxdragon::main` closure
/// and its lifetime is extended by calling `handle.preserve(frame.clone())`.
/// This preserved frame is automatically cleaned up when the application exits.
///
/// For secondary frames or frames managed manually (e.g., created and shown
/// dynamically after the main loop has started), it is crucial to explicitly
/// call the `.destroy()` method (available via the `WxWidget` trait) when the
/// frame is no longer needed. This ensures that the underlying C++ wxFrame object
/// and its associated resources (including event handlers and Rust closures)
/// are properly deallocated.
///
/// # Example
/// ```no_run
/// # use wxdragon::prelude::*;
/// # wxdragon::main(|_app| {
/// # let main_frame = Frame::builder().build();
/// // Example of a manually managed frame
/// let secondary_frame = Frame::builder()
///     .with_title("Secondary Window")
///     .build();
/// secondary_frame.show(true);
/// // ... use secondary_frame ...
///
/// // When done with secondary_frame, explicitly destroy it:
/// secondary_frame.destroy();
/// // After calling destroy(), `secondary_frame` should not be used further.
/// # main_frame.show(true);
/// # }).unwrap();
/// ```
/// Calling `.close()` on a frame initiates the closing process, which typically also
/// leads to the frame's destruction by wxWidgets, unless the close event is vetoed.
/// `.destroy()` provides a more direct way to ensure destruction.
#[derive(Clone, Copy)]
pub struct Frame {
    /// Safe handle to the underlying wxFrame - automatically invalidated on destroy
    handle: WindowHandle,
    // Store parent pointer to manage drop behavior
    #[allow(dead_code)]
    parent_ptr: *mut ffi::wxd_Window_t,
    _marker: PhantomData<()>,
}

// --- Frame Builder ---

/// Builder pattern for creating `Frame` widgets.
// Cannot derive Default because of raw pointer field `parent_ptr`
pub struct FrameBuilder {
    // Fields store values directly, initialized by Default
    parent_ptr: *mut ffi::wxd_Window_t, // Optional parent, defaults to null
    id: Id,
    title: String,
    pos: Point,
    size: Size,
    style: FrameStyle,
    // name: String, // Removed name for now
}

// Manual implementation of Default
impl Default for FrameBuilder {
    fn default() -> Self {
        Self {
            parent_ptr: ptr::null_mut(),
            id: ID_ANY as i32,                      // Use ID_ANY from base (already i32)
            title: "wxDragon Frame".to_string(),    // Default title
            pos: Point::DEFAULT_POSITION,           // Default position
            size: Size { width: 500, height: 400 }, // Specific default size for Frame
            style: FrameStyle::Default,
            // name: String::new(),
        }
    }
}

impl FrameBuilder {
    /// Sets the optional parent window.
    pub fn with_parent(mut self, parent: &impl WxWidget) -> Self {
        self.parent_ptr = parent.handle_ptr();
        self
    }

    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the frame title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the position.
    pub fn with_position(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the size.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets the window style flags.
    pub fn with_style(mut self, style: FrameStyle) -> Self {
        self.style = style;
        self
    }

    /// Builds the `Frame`.
    ///
    /// # Panics
    /// Panics if frame creation fails in the underlying C++ layer.
    pub fn build(self) -> Frame {
        let c_title = CString::new(self.title).expect("CString::new failed for title");

        let ptr = unsafe {
            ffi::wxd_Frame_Create(
                self.parent_ptr,
                self.id,
                c_title.as_ptr(),
                self.pos.into(),
                self.size.into(),
                self.style.bits() as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create wxFrame: wxWidgets returned a null pointer.");
        } else {
            Frame {
                handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
                parent_ptr: self.parent_ptr,
                _marker: PhantomData,
            }
        }
    }
}

// --- Frame Implementation ---

impl Frame {
    /// Creates a new `FrameBuilder` for constructing a frame.
    pub fn builder() -> FrameBuilder {
        FrameBuilder::default()
    }

    /// Creates a new Frame from a raw window pointer.
    /// This is for backwards compatibility with widgets that compose Frame.
    /// The parent_ptr parameter is ignored (kept for API compatibility).
    #[allow(dead_code)]
    pub(crate) fn new_from_composition(_window: Window, _parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        // Use the window's pointer to create a new WindowHandle
        Self {
            handle: WindowHandle::new(_window.as_ptr()),
            parent_ptr: _parent_ptr,
            _marker: PhantomData,
        }
    }

    /// Helper to get raw frame pointer, returns null if widget has been destroyed
    #[inline]
    fn frame_ptr(&self) -> *mut ffi::wxd_Frame_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_Frame_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Return internal window handle
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    /// Sets the frame's title.
    /// No-op if the frame has been destroyed.
    pub fn set_title(&self, title: &str) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        let title_c = CString::new(title).expect("CString::new failed");
        unsafe { ffi::wxd_Frame_SetTitle(ptr, title_c.as_ptr()) };
    }

    /// Centers the frame on the screen or parent. (wxWidgets `Centre` method)
    /// No-op if the frame has been destroyed.
    pub fn centre(&self) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Frame_Centre(ptr, ffi::WXD_ALIGN_CENTRE as i32) };
    }

    /// Centers the frame on the screen. (wxWidgets `CenterOnScreen` method)
    /// No-op if the frame has been destroyed.
    pub fn center_on_screen(&self) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Frame_CenterOnScreen(ptr) }
    }

    /// Shows the frame.
    /// No-op if the frame has been destroyed.
    pub fn show(&self, show: bool) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Frame_Show(ptr, show) };
    }

    /// Sets the menu bar for this frame.
    /// The frame takes ownership of the menu bar.
    /// No-op if the frame has been destroyed.
    pub fn set_menu_bar(&self, menu_bar: MenuBar) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        let menu_bar_ptr = unsafe { menu_bar.as_ptr() };
        // Frame takes ownership of the menu bar pointer, but MenuBar doesn't implement Drop
        // so no need to forget it
        unsafe { ffi::wxd_Frame_SetMenuBar(ptr, menu_bar_ptr) };
    }

    /// Gets the menu bar for this frame.
    /// Returns None if no menu bar is set or if the frame has been destroyed.
    pub fn get_menu_bar(&self) -> Option<MenuBar> {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return None;
        }
        let menu_bar_ptr = unsafe { ffi::wxd_Frame_GetMenuBar(ptr) };
        if menu_bar_ptr.is_null() {
            None
        } else {
            Some(unsafe { MenuBar::from_ptr(menu_bar_ptr) })
        }
    }

    /// Closes the frame.
    /// No-op if the frame has been destroyed.
    pub fn close(&self, force: bool) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        // false = don't force close, allow events like EVT_CLOSE_WINDOW
        unsafe { ffi::wxd_Frame_Close(ptr, force) };
    }

    /// Sets the frame's status bar.
    /// No-op if the frame has been destroyed.
    pub fn set_existing_status_bar(&self, status_bar: Option<&StatusBar>) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        let sb_ptr = status_bar.map_or(ptr::null_mut(), |sb| sb.as_ptr() as *mut _);
        unsafe { ffi::wxd_Frame_SetStatusBar(ptr, sb_ptr) };
    }

    /// Creates and assigns a toolbar to the frame.
    /// Returns `Some(ToolBar)` if successful, `None` otherwise or if the frame has been destroyed.
    pub fn create_tool_bar(&self, style: Option<ToolBarStyle>, id: Id) -> Option<ToolBar> {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return None;
        }
        // Use ToolBarStyle default bits if None
        let style_bits = style.map(|s| s.bits()).unwrap_or(ToolBarStyle::default().bits());

        let tb_ptr = unsafe { ffi::wxd_Frame_CreateToolBar(ptr, style_bits as ffi::wxd_Style_t, id) };
        if tb_ptr.is_null() {
            None
        } else {
            Some(unsafe { ToolBar::from_ptr(tb_ptr) })
        }
    }

    /// Creates a status bar for the frame.
    /// Returns empty StatusBar if the frame has been destroyed.
    pub fn create_status_bar(&self, number: i32, style: i64, id: Id, name: &str) -> StatusBar {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            // Return a StatusBar with null pointer if frame is invalid
            return unsafe { StatusBar::from_ptr(ptr::null_mut()) };
        }
        unsafe {
            let name_c = CString::new(name).unwrap_or_default();
            let statbar_ptr =
                ffi::wxd_Frame_CreateStatusBar(ptr, number as c_int, style as ffi::wxd_Style_t, id, name_c.as_ptr());
            StatusBar::from_ptr(statbar_ptr)
        }
    }

    /// Sets the status text in the specified field.
    /// No-op if the frame has been destroyed.
    pub fn set_status_text(&self, text: &str, number: i32) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        let c_text = CString::new(text).expect("CString::new for status text failed");
        unsafe { ffi::wxd_Frame_SetStatusText(ptr, c_text.as_ptr(), number) }
    }

    /// Gets the frame's title.
    /// Returns empty string if the frame has been destroyed.
    pub fn get_title(&self) -> String {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let c_title_ptr = unsafe { ffi::wxd_Frame_GetTitle(ptr) };
        if c_title_ptr.is_null() {
            return String::new(); // Should ideally not happen if C returns empty string for null frame
        }
        // CString::from_raw takes ownership and will free the memory.
        unsafe {
            CString::from_raw(c_title_ptr)
                .into_string()
                .unwrap_or_else(|_| String::from("Error converting title"))
        }
    }

    /// Iconizes or restores the frame.
    /// No-op if the frame has been destroyed.
    pub fn iconize(&self, iconize: bool) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Frame_Iconize(ptr, iconize) }
    }

    /// Returns true if the frame is iconized.
    /// Returns false if the frame has been destroyed.
    pub fn is_iconized(&self) -> bool {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Frame_IsIconized(ptr) }
    }

    /// Maximizes or restores the frame.
    /// No-op if the frame has been destroyed.
    pub fn maximize(&self, maximize: bool) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Frame_Maximize(ptr, maximize) }
    }

    /// Returns true if the frame is maximized.
    /// Returns false if the frame has been destroyed.
    pub fn is_maximized(&self) -> bool {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Frame_IsMaximized(ptr) }
    }

    /// Sets the frame's icon from a bitmap.
    /// The bitmap will be converted to an icon internally.
    /// No-op if the frame has been destroyed.
    pub fn set_icon(&self, bitmap: &Bitmap) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Frame_SetIconFromBitmap(ptr, bitmap.as_const_ptr()) };
    }
}

// Add event binding methods to Frame
impl Frame {
    /// Bind a handler to window events using the underlying window
    pub(crate) fn bind_window_event<F>(&self, event_type: crate::event::EventType, handler: F)
    where
        F: FnMut(crate::event::Event) + 'static,
    {
        // Use the bind_internal method provided by the WxEvtHandler trait
        <Self as crate::event::WxEvtHandler>::bind_internal(self, event_type, handler);
    }

    /// Bind a handler to menu events
    pub fn on_menu<F>(&self, handler: F)
    where
        F: FnMut(crate::event::Event) + 'static,
    {
        self.bind_window_event(crate::event::EventType::MENU, handler);
    }

    /// Convenience method for tracking menu lifecycle events
    pub fn track_menu_lifecycle<F>(&self, callback: F)
    where
        F: Fn(&str, bool) + 'static, // (event_type, is_opening)
    {
        use crate::event::MenuEvents;

        let callback_ref = std::rc::Rc::new(callback);

        let callback_open = {
            let cb = callback_ref.clone();
            move |_: crate::event::MenuEventData| cb("menu_open", true)
        };

        let callback_close = {
            let cb = callback_ref.clone();
            move |_: crate::event::MenuEventData| cb("menu_close", false)
        };

        self.on_menu_opened(callback_open);
        self.on_menu_closed(callback_close);
    }
}

// Manual WxWidget implementation for Frame (using WindowHandle)
impl WxWidget for Frame {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl crate::event::WxEvtHandler for Frame {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for Frame {}
impl crate::event::MenuEvents for Frame {}

// Manual XRC Support for Frame - complex structure needs custom handling
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for Frame {
    unsafe fn from_xrc_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
        Frame {
            handle: WindowHandle::new(ptr),
            parent_ptr: std::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

// Manual widget casting support for Frame - complex structure needs custom handling
impl crate::window::FromWindowWithClassName for Frame {
    fn class_name() -> &'static str {
        "wxFrame"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Frame {
            handle: WindowHandle::new(ptr),
            parent_ptr: std::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
