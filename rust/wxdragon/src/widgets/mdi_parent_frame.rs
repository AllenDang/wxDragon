use crate::event::WxEvtHandler;
use crate::menus::MenuBar;
use crate::prelude::*;
use crate::widgets::statusbar::StatusBar;
use crate::window::{Window, WindowHandle, WxWidget};
use std::ffi::CString;
use wxdragon_sys as ffi;

// Reuse FrameStyle as MDIParentFrame is a Frame
pub use crate::widgets::frame::FrameStyle;

/// Represents a wxMDIParentFrame.
///
/// MDIParentFrame uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct MDIParentFrame {
    /// Safe handle to the underlying wxMDIParentFrame - automatically invalidated on destroy
    handle: WindowHandle,
}

impl MDIParentFrame {
    /// Creates a new `MDIParentFrameBuilder` for constructing an MDI parent frame.
    pub fn builder() -> MDIParentFrameBuilder {
        MDIParentFrameBuilder::default()
    }

    /// Creates a new MDIParentFrame (low-level constructor used by the builder)
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, title: &str, pos: Point, size: Size, style: i64, name: &str) -> Self {
        let title_c = CString::new(title).expect("CString::new failed for title");
        let name_c = CString::new(name).expect("CString::new failed for name");

        let ptr = unsafe {
            ffi::wxd_MDIParentFrame_Create(
                parent_ptr,
                id,
                title_c.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
                name_c.as_ptr(),
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxMDIParentFrame");
        }

        // Create a WindowHandle which automatically registers for destroy events
        MDIParentFrame {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the underlying WindowHandle for this MDI parent frame.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    /// Helper to get raw frame pointer
    fn frame_ptr(&self) -> *mut ffi::wxd_Frame_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_Frame_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Sets the menu bar for this frame.
    pub fn set_menu_bar(&self, menu_bar: MenuBar) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        let menu_bar_ptr = unsafe { menu_bar.as_ptr() };
        unsafe { ffi::wxd_Frame_SetMenuBar(ptr, menu_bar_ptr) };
    }

    /// Creates a status bar for the frame.
    pub fn create_status_bar(&self, number: i32, style: i64, id: Id, name: &str) -> StatusBar {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return unsafe { StatusBar::from_ptr(std::ptr::null_mut()) };
        }
        unsafe {
            let name_c = CString::new(name).unwrap_or_default();
            let statbar_ptr = ffi::wxd_Frame_CreateStatusBar(
                ptr,
                number as std::os::raw::c_int,
                style as ffi::wxd_Style_t,
                id,
                name_c.as_ptr(),
            );
            StatusBar::from_ptr(statbar_ptr)
        }
    }

    /// Sets the status text in the specified field.
    pub fn set_status_text(&self, text: &str, number: i32) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        let c_text = CString::new(text).expect("CString::new for status text failed");
        unsafe { ffi::wxd_Frame_SetStatusText(ptr, c_text.as_ptr(), number) }
    }

    /// Shows the frame.
    pub fn show(&self, show: bool) {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Frame_Show(ptr, show) };
    }

    /// Closes the frame.
    pub fn close(&self, force: bool) -> bool {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Frame_Close(ptr, force) };
        true
    }

    /// Returns the client window for this MDI parent frame.
    pub fn get_client_window(&self) -> Option<Window> {
        let ptr = self.frame_ptr();
        if ptr.is_null() {
            return None;
        }
        let client_ptr = unsafe { ffi::wxd_MDIParentFrame_GetClientWindow(ptr) };
        if client_ptr.is_null() {
            None
        } else {
            Some(unsafe { Window::from_ptr(client_ptr) })
        }
    }
}

// Add on_menu convenience method
impl MDIParentFrame {
    pub fn on_menu<F>(&self, handler: F)
    where
        F: FnMut(crate::event::Event) + 'static,
    {
        <Self as crate::event::WxEvtHandler>::bind_internal(self, crate::event::EventType::MENU, handler);
    }
}

// MDIParentFrameBuilder
pub struct MDIParentFrameBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: Id,
    title: String,
    pos: Point,
    size: Size,
    style: FrameStyle,
    name: String,
}

impl Default for MDIParentFrameBuilder {
    fn default() -> Self {
        Self {
            parent_ptr: std::ptr::null_mut(),
            id: ID_ANY as Id,
            title: "wxDragon MDI Parent Frame".to_string(),
            pos: Point::DEFAULT_POSITION,
            size: Size { width: 800, height: 600 },
            style: FrameStyle::Default,
            name: "wxMDIParentFrame".to_string(),
        }
    }
}

impl MDIParentFrameBuilder {
    pub fn with_parent(mut self, parent: &dyn WxWidget) -> Self {
        self.parent_ptr = parent.handle_ptr();
        self
    }

    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_position(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn with_style(mut self, style: FrameStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn build(self) -> MDIParentFrame {
        MDIParentFrame::new_impl(
            self.parent_ptr,
            self.id,
            &self.title,
            self.pos,
            self.size,
            self.style.bits(),
            &self.name,
        )
    }
}

// Manual WxWidget implementation for MDIParentFrame (using WindowHandle)
impl WxWidget for MDIParentFrame {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for MDIParentFrame {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for MDIParentFrame {}
impl crate::event::MenuEvents for MDIParentFrame {}

// Enable widget casting for MDIParentFrame
impl crate::window::FromWindowWithClassName for MDIParentFrame {
    fn class_name() -> &'static str {
        "wxMDIParentFrame"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        MDIParentFrame {
            handle: WindowHandle::new(ptr),
        }
    }
}
