use crate::event::WxEvtHandler;
use crate::prelude::*;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::CString;
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

/// Represents a wxAuiMDIParentFrame.
///
/// AuiMdiParentFrame uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let parent_frame = AuiMdiParentFrame::builder(&app).title("MDI Parent").build();
///
/// // AuiMdiParentFrame is Copy - no clone needed for closures!
/// parent_frame.bind_close(move |_| {
///     // Safe: if parent_frame was destroyed, this is a no-op
///     parent_frame.destroy();
/// });
///
/// // After destruction, parent_frame operations are safe no-ops
/// assert!(!parent_frame.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct AuiMdiParentFrame {
    /// Safe handle to the underlying wxAuiMDIParentFrame - automatically invalidated on destroy
    handle: WindowHandle,
}

impl AuiMdiParentFrame {
    /// Creates a new `AuiMdiParentFrameBuilder` for constructing an AUI MDI parent frame.
    pub fn builder(parent: &dyn WxWidget) -> AuiMdiParentFrameBuilder<'_> {
        AuiMdiParentFrameBuilder::new(parent)
    }

    /// Creates a new AuiMdiParentFrame (low-level constructor used by the builder)
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

        // Create a WindowHandle which automatically registers for destroy events
        AuiMdiParentFrame {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the underlying WindowHandle for this AUI MDI parent frame.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

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

// Manual WxWidget implementation for AuiMdiParentFrame (using WindowHandle)
impl WxWidget for AuiMdiParentFrame {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Note: We don't implement Deref to Window because returning a reference
// to a temporary Window is unsound. Users can access window methods through
// the WxWidget trait methods directly.

// Implement WxEvtHandler for event binding
impl WxEvtHandler for AuiMdiParentFrame {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for AuiMdiParentFrame {}

// Enable widget casting for AuiMdiParentFrame
impl crate::window::FromWindowWithClassName for AuiMdiParentFrame {
    fn class_name() -> &'static str {
        "wxAuiMDIParentFrame"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        AuiMdiParentFrame {
            handle: WindowHandle::new(ptr),
        }
    }
}
