use crate::prelude::*;
use crate::widgets::aui_mdi_parent_frame::AuiMdiParentFrame;
use crate::window::Window;
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

#[derive(Clone)]
pub struct AuiMdiChildFrame {
    window: Window,
    #[allow(dead_code)]
    parent_ptr: *mut ffi::wxd_AuiMDIParentFrame_t,
    _marker: PhantomData<()>,
}

impl AuiMdiChildFrame {
    fn from_ptr(ptr: *mut ffi::wxd_AuiMDIChildFrame_t, parent_ptr: *mut ffi::wxd_AuiMDIParentFrame_t) -> Self {
        AuiMdiChildFrame {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
            parent_ptr,
            _marker: PhantomData,
        }
    }

    /// Creates a new builder for AuiMdiChildFrame
    pub fn builder<'a>(parent: &'a AuiMdiParentFrame) -> AuiMdiChildFrameBuilder<'a> {
        AuiMdiChildFrameBuilder::new(parent)
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
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
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

// Implement all standard widget traits in one go
implement_widget_traits_with_target!(AuiMdiChildFrame, window, Window);
