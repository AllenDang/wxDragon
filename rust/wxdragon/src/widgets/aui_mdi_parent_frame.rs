use crate::prelude::*;
use crate::window::Window;
use std::ffi::CString;
use std::marker::PhantomData;
use wxdragon_sys as ffi;

// Define style enum for AuiMdiParentFrame
widget_style_enum!(
    name: AuiMdiParentFrameStyle,
    doc: "Style flags for AuiMdiParentFrame.",
    variants: {
        Default: ffi::WXD_DEFAULT_FRAME_STYLE, "Default frame style."
        // Add any specific AuiMdiParentFrame styles here if needed
    },
    default_variant: Default
);

#[derive(Clone)]
pub struct AuiMdiParentFrame {
    window: Window, // Composition: AuiMdiParentFrame uses a Window internally
    // Store parent pointer to manage drop behavior
    #[allow(dead_code)]
    parent_ptr: *mut ffi::wxd_Window_t,
    _marker: PhantomData<()>,
}

impl AuiMdiParentFrame {
    fn from_ptr(ptr: *mut ffi::wxd_AuiMDIParentFrame_t, parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        AuiMdiParentFrame {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
            parent_ptr,
            _marker: PhantomData,
        }
    }

    pub fn builder<'a>(parent: &'a dyn WxWidget) -> AuiMdiParentFrameBuilder<'a> {
        AuiMdiParentFrameBuilder::new(parent)
    }

    // Implementation method called by the builder
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, title: &str, pos: Point, size: Size, style: i64, name: &str) -> Self {
        let title_c = CString::new(title).expect("CString::new failed for title");
        let name_c = CString::new(name).expect("CString::new failed for name");

        let ptr = unsafe {
            ffi::wxd_AuiMDIParentFrame_Create(
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
            panic!("Failed to create wxAuiMDIParentFrame");
        }
        AuiMdiParentFrame::from_ptr(ptr, parent_ptr)
    }

    // Common methods inherited from Window can be accessed via Deref to Window
    // Add any AuiMdiParentFrame-specific methods here
}

// Use widget_builder macro to create the builder
widget_builder!(
    name: AuiMdiParentFrame,
    parent_type: &'a dyn WxWidget,
    style_type: AuiMdiParentFrameStyle,
    fields: {
        title: String = String::new(),
        name: String = "wxDragon AUI Frame".to_string()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        AuiMdiParentFrame::new_impl(
            parent_ptr,
            slf.id,
            &slf.title,
            slf.pos,
            slf.size,
            slf.style.bits(),
            &slf.name
        )
    }
);

// Implement all standard widget traits in one go
implement_widget_traits_with_target!(AuiMdiParentFrame, window, Window);
