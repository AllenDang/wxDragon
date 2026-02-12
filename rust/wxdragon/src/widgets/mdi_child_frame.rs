use crate::event::WxEvtHandler;
use crate::prelude::*;
use crate::widgets::mdi_parent_frame::MDIParentFrame;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::CString;
use std::marker::PhantomData;
use wxdragon_sys as ffi;

// Reuse FrameStyle
pub use crate::widgets::frame::FrameStyle;

/// Represents a wxMDIChildFrame.
///
/// MDIChildFrame uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct MDIChildFrame {
    /// Safe handle to the underlying wxMDIChildFrame - automatically invalidated on destroy
    handle: WindowHandle,
    _marker: PhantomData<()>,
}

impl MDIChildFrame {
    /// Creates a new builder for MDIChildFrame
    pub fn builder(parent: &MDIParentFrame) -> MDIChildFrameBuilder<'_> {
        MDIChildFrameBuilder::new(parent)
    }

    /// Returns the underlying WindowHandle for this child frame.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    /// Shows the child frame.
    pub fn show(&self, show: bool) {
        let ptr = self.handle.get_ptr();
        if let Some(ptr) = ptr {
            unsafe { ffi::wxd_Frame_Show(ptr as *mut ffi::wxd_Frame_t, show) };
        }
    }
}

/// Builder for MDIChildFrame
pub struct MDIChildFrameBuilder<'a> {
    parent: &'a MDIParentFrame,
    id: Id,
    title: String,
    pos: Point,
    size: Size,
    style: FrameStyle,
    name: String,
}

impl<'a> MDIChildFrameBuilder<'a> {
    /// Creates a new builder
    pub fn new(parent: &'a MDIParentFrame) -> Self {
        Self {
            parent,
            id: ID_ANY as Id,
            title: "wxDragon MDI Child Frame".to_string(),
            pos: Point::DEFAULT_POSITION,
            size: Size::DEFAULT_SIZE,
            style: FrameStyle::Default,
            name: "wxMDIChildFrame".to_string(),
        }
    }

    /// Sets the window identifier
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the frame title
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the position
    pub fn with_position(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the size
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets the style flags
    pub fn with_style(mut self, style: FrameStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the window name
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Builds the MDIChildFrame
    pub fn build(self) -> MDIChildFrame {
        let title_c = CString::new(self.title).expect("CString::new failed for title");
        let name_c = CString::new(self.name).expect("CString::new failed for name");
        let parent_ptr = self.parent.handle_ptr() as *mut ffi::wxd_Frame_t;

        let ptr = unsafe {
            ffi::wxd_MDIChildFrame_Create(
                parent_ptr,
                self.id,
                title_c.as_ptr(),
                self.pos.into(),
                self.size.into(),
                self.style.bits() as ffi::wxd_Style_t,
                name_c.as_ptr(),
            )
        };

        if ptr.is_null() {
            panic!("Failed to create MDIChildFrame: wxWidgets returned a null pointer.");
        } else {
            MDIChildFrame {
                handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
                _marker: PhantomData,
            }
        }
    }
}

// Manual WxWidget implementation for MDIChildFrame (using WindowHandle)
impl WxWidget for MDIChildFrame {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for MDIChildFrame {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for MDIChildFrame {}
impl crate::event::MenuEvents for MDIChildFrame {}

// Enable widget casting for MDIChildFrame
impl crate::window::FromWindowWithClassName for MDIChildFrame {
    fn class_name() -> &'static str {
        "wxMDIChildFrame"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        MDIChildFrame {
            handle: WindowHandle::new(ptr),
            _marker: PhantomData,
        }
    }
}
