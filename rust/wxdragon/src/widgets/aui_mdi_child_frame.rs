use crate::event::WxEvtHandler;
use crate::prelude::*;
use crate::widgets::aui_mdi_parent_frame::AuiMdiParentFrame;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::CString;
use std::marker::PhantomData;
use wxdragon_sys as ffi;

// Define style enum for AuiMdiChildFrame
widget_style_enum!(
    name: AuiMdiChildFrameStyle,
    doc: "Style flags for AuiMdiChildFrame.",
    variants: {
        Default: ffi::WXD_DEFAULT_FRAME_STYLE, "Default frame style."
        // Add any specific AuiMdiChildFrame styles here if needed
    },
    default_variant: Default
);

/// Represents a wxAuiMDIChildFrame.
///
/// AuiMdiChildFrame uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let child_frame = AuiMdiChildFrame::builder(&parent_frame)
///     .with_title("Child Window")
///     .build();
///
/// // Child frame is Copy - no clone needed for closures!
/// // After parent destruction, operations are safe no-ops
/// ```
#[derive(Clone, Copy)]
pub struct AuiMdiChildFrame {
    /// Safe handle to the underlying wxAuiMDIChildFrame - automatically invalidated on destroy
    handle: WindowHandle,
    #[allow(dead_code)]
    parent_ptr: *mut ffi::wxd_AuiMDIParentFrame_t,
    _marker: PhantomData<()>,
}

impl AuiMdiChildFrame {
    fn from_ptr(ptr: *mut ffi::wxd_AuiMDIChildFrame_t, parent_ptr: *mut ffi::wxd_AuiMDIParentFrame_t) -> Self {
        AuiMdiChildFrame {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
            parent_ptr,
            _marker: PhantomData,
        }
    }

    /// Creates a new builder for AuiMdiChildFrame
    pub fn builder<'a>(parent: &'a AuiMdiParentFrame) -> AuiMdiChildFrameBuilder<'a> {
        AuiMdiChildFrameBuilder::new(parent)
    }

    /// Helper to get raw child frame pointer, returns null if widget has been destroyed
    #[inline]
    #[allow(dead_code)]
    fn child_frame_ptr(&self) -> *mut ffi::wxd_AuiMDIChildFrame_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_AuiMDIChildFrame_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Returns the underlying WindowHandle for this child frame.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

/// Builder for AuiMdiChildFrame
#[derive(Clone)]
pub struct AuiMdiChildFrameBuilder<'a> {
    parent: &'a AuiMdiParentFrame,
    id: Id,
    title: String,
    pos: Point,
    size: Size,
    style: AuiMdiChildFrameStyle,
    name: String,
}

impl<'a> AuiMdiChildFrameBuilder<'a> {
    /// Creates a new builder
    pub fn new(parent: &'a AuiMdiParentFrame) -> Self {
        Self {
            parent,
            id: ID_ANY as Id,
            title: String::new(),
            pos: Point::DEFAULT_POSITION,
            size: Size::DEFAULT_SIZE,
            style: AuiMdiChildFrameStyle::Default,
            name: "wxAuiMDIChildFrame".to_string(),
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
    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the size
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets the style flags
    pub fn with_style(mut self, style: AuiMdiChildFrameStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the window name
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Builds the AuiMdiChildFrame
    pub fn build(self) -> AuiMdiChildFrame {
        let title_c = CString::new(self.title).expect("CString::new failed for title");
        let name_c = CString::new(self.name).expect("CString::new failed for name");
        let parent_ptr = self.parent.handle_ptr() as *mut ffi::wxd_AuiMDIParentFrame_t;

        let ptr = unsafe {
            ffi::wxd_AuiMDIChildFrame_Create(
                parent_ptr,
                self.id,
                title_c.as_ptr(),
                self.pos.into(),
                self.size.into(),
                self.style.bits(),
                name_c.as_ptr(),
            )
        };

        if ptr.is_null() {
            panic!("Failed to create AuiMdiChildFrame: wxWidgets returned a null pointer.");
        } else {
            AuiMdiChildFrame::from_ptr(ptr, parent_ptr)
        }
    }
}

// Manual WxWidget implementation for AuiMdiChildFrame (using WindowHandle)
impl WxWidget for AuiMdiChildFrame {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for AuiMdiChildFrame {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for AuiMdiChildFrame {}
