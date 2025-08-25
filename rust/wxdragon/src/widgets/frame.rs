use crate::bitmap::Bitmap; // ADDED: Import Bitmap
use crate::event::WindowEvents;
use crate::geometry::{Point, Size, DEFAULT_POSITION};
use crate::id::Id;
use crate::id::ID_ANY;
use crate::menus::MenuBar; // ADDED: Import MenuBar
use crate::widget_style_enum;
use crate::widgets::statusbar::StatusBar; // ADDED Import
use crate::widgets::toolbar::{ToolBar, ToolBarStyle}; // Added ToolBarStyle
use crate::window::{Window, WxWidget};
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
/// ```no_run
/// # use wxdragon::prelude::*;
/// # wxdragon::main(|handle| {
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
/// # handle.preserve(main_frame);
/// # true
/// # });
/// ```
/// Calling `.close()` on a frame initiates the closing process, which typically also
/// leads to the frame's destruction by wxWidgets, unless the close event is vetoed.
/// `.destroy()` provides a more direct way to ensure destruction.
#[derive(Clone)]
pub struct Frame {
    window: Window, // Composition: Frame uses a Window internally
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
            id: ID_ANY as i32, // Use ID_ANY from base (already i32)
            title: "wxDragon Frame".to_string(), // Default title
            pos: DEFAULT_POSITION, // Default position
            size: Size {
                width: 500,
                height: 400,
            }, // Specific default size for Frame
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
                window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
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

    /// Return internal window
    pub fn get_window(&self) -> Window {
        return self.window;
    }

    /// Sets the frame's title.
    pub fn set_title(&self, title: &str) {
        let title_c = CString::new(title).expect("CString::new failed");
        unsafe {
            ffi::wxd_Frame_SetTitle(
                self.window.as_ptr() as *mut ffi::wxd_Frame_t,
                title_c.as_ptr(),
            );
        }
    }

    /// Centers the frame on the screen or parent. (wxWidgets `Centre` method)
    pub fn centre(&self) {
        unsafe {
            ffi::wxd_Frame_Centre(
                self.window.as_ptr() as *mut ffi::wxd_Frame_t,
                ffi::WXD_ALIGN_CENTRE as i32,
            );
        }
    }

    /// Centers the frame on the screen. (wxWidgets `CenterOnScreen` method)
    pub fn center_on_screen(&self) {
        unsafe { ffi::wxd_Frame_CenterOnScreen(self.window.as_ptr() as *mut ffi::wxd_Frame_t) }
    }

    /// Shows the frame.
    pub fn show(&self, show: bool) {
        unsafe {
            ffi::wxd_Frame_Show(self.window.as_ptr() as *mut ffi::wxd_Frame_t, show);
        }
    }

    /// Sets the menu bar for this frame.
    /// The frame takes ownership of the menu bar.
    pub fn set_menu_bar(&self, menu_bar: MenuBar) {
        let menu_bar_ptr = unsafe { menu_bar.as_ptr() };
        // Frame takes ownership of the menu bar pointer, but MenuBar doesn't implement Drop
        // so no need to forget it
        unsafe {
            ffi::wxd_Frame_SetMenuBar(self.window.as_ptr() as *mut _, menu_bar_ptr);
        }
    }

    /// Gets the menu bar for this frame.
    /// Returns None if no menu bar is set.
    pub fn get_menu_bar(&self) -> Option<MenuBar> {
        let menu_bar_ptr =
            unsafe { ffi::wxd_Frame_GetMenuBar(self.window.as_ptr() as *mut ffi::wxd_Frame_t) };
        if menu_bar_ptr.is_null() {
            None
        } else {
            Some(unsafe { MenuBar::from_ptr(menu_bar_ptr) })
        }
    }

    /// Closes the frame.
    pub fn close(&self, force: bool) {
        unsafe {
            // false = don't force close, allow events like EVT_CLOSE_WINDOW
            ffi::wxd_Frame_Close(self.window.as_ptr() as *mut ffi::wxd_Frame_t, force);
        }
    }

    /// Sets the frame's status bar.
    pub fn set_existing_status_bar(&self, status_bar: Option<&StatusBar>) {
        let sb_ptr = status_bar.map_or(ptr::null_mut(), |sb| sb.as_ptr() as *mut _);
        unsafe {
            ffi::wxd_Frame_SetStatusBar(self.window.as_ptr() as *mut ffi::wxd_Frame_t, sb_ptr);
        }
    }

    /// Creates and assigns a toolbar to the frame.
    /// Returns `Some(ToolBar)` if successful, `None` otherwise.
    pub fn create_tool_bar(&self, style: Option<ToolBarStyle>, id: Id) -> Option<ToolBar> {
        let style_bits = style
            .map(|s| s.bits())
            .unwrap_or(ToolBarStyle::default().bits()); // Use ToolBarStyle default bits if None

        let tb_ptr = unsafe {
            ffi::wxd_Frame_CreateToolBar(
                self.window.as_ptr() as *mut _,
                style_bits as ffi::wxd_Style_t, // Use bits()
                id,
            )
        };
        if tb_ptr.is_null() {
            None
        } else {
            Some(unsafe { ToolBar::from_ptr(tb_ptr) })
        }
    }

    /// Creates a status bar for the frame.
    pub fn create_status_bar(&self, number: i32, style: i64, id: Id, name: &str) -> StatusBar {
        unsafe {
            let name_c = CString::new(name).unwrap_or_default();
            let statbar_ptr = ffi::wxd_Frame_CreateStatusBar(
                self.window.as_ptr() as *mut ffi::wxd_Frame_t,
                number as c_int,
                style as ffi::wxd_Style_t,
                id,
                name_c.as_ptr(),
            );
            StatusBar::from_ptr(statbar_ptr)
        }
    }

    // New safe wrapper methods for wxFrame
    pub fn set_status_text(&self, text: &str, number: i32) {
        let c_text = CString::new(text).expect("CString::new for status text failed");
        unsafe {
            ffi::wxd_Frame_SetStatusText(
                self.window.as_ptr() as *mut ffi::wxd_Frame_t,
                c_text.as_ptr(),
                number,
            )
        }
    }

    pub fn get_title(&self) -> String {
        unsafe {
            let c_title_ptr =
                ffi::wxd_Frame_GetTitle(self.window.as_ptr() as *mut ffi::wxd_Frame_t);
            if c_title_ptr.is_null() {
                return String::new(); // Should ideally not happen if C returns empty string for null frame
            }
            // CString::from_raw takes ownership and will free the memory.
            CString::from_raw(c_title_ptr)
                .into_string()
                .unwrap_or_else(|_| String::from("Error converting title"))
        }
    }

    pub fn iconize(&self, iconize: bool) {
        unsafe { ffi::wxd_Frame_Iconize(self.window.as_ptr() as *mut ffi::wxd_Frame_t, iconize) }
    }

    pub fn is_iconized(&self) -> bool {
        unsafe { ffi::wxd_Frame_IsIconized(self.window.as_ptr() as *mut ffi::wxd_Frame_t) }
    }

    pub fn maximize(&self, maximize: bool) {
        unsafe { ffi::wxd_Frame_Maximize(self.window.as_ptr() as *mut ffi::wxd_Frame_t, maximize) }
    }

    pub fn is_maximized(&self) -> bool {
        unsafe { ffi::wxd_Frame_IsMaximized(self.window.as_ptr() as *mut ffi::wxd_Frame_t) }
    }

    /// Sets the frame's icon from a bitmap.
    /// The bitmap will be converted to an icon internally.
    pub fn set_icon(&self, bitmap: &Bitmap) {
        unsafe {
            ffi::wxd_Frame_SetIconFromBitmap(
                self.window.as_ptr() as *mut ffi::wxd_Frame_t,
                bitmap.as_ptr(),
            );
        }
    }
}

// Implement WindowEvents trait for Frame
impl WindowEvents for Frame {}

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
}

implement_widget_traits_with_target!(Frame, window, Window);

// Manual XRC Support for Frame - complex structure needs custom handling
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for Frame {
    unsafe fn from_xrc_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
        Frame {
            window: Window::from_ptr(ptr),
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
            window: Window::from_ptr(ptr),
            parent_ptr: std::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
