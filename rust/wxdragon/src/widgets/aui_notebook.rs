use crate::event::WxEvtHandler;
use crate::prelude::*;
use crate::window::{WindowHandle, WxWidget};
// Window is used by widget_builder macro for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::CString;
use wxdragon_sys as ffi;

// Define style enum for AuiNotebook
widget_style_enum!(
    name: AuiNotebookStyle,
    doc: "Style flags for AuiNotebook.",
    variants: {
        Default: 0x00000001 | 0x00000002 | 0x00000004 | 0x00000010 | 0x00000040 | 0x00000200, "Default AuiNotebook style."
        // Add any specific AuiNotebook styles here once available via ffi constants
    },
    default_variant: Default
);

/// Represents a wxAuiNotebook.
///
/// AuiNotebook uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let notebook = AuiNotebook::builder(&frame).build();
///
/// // AuiNotebook is Copy - no clone needed for closures!
/// notebook.add_page(&panel, "Tab 1", true);
///
/// // After parent destruction, notebook operations are safe no-ops
/// frame.destroy();
/// assert!(!notebook.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct AuiNotebook {
    /// Safe handle to the underlying wxAuiNotebook - automatically invalidated on destroy
    handle: WindowHandle,
}

impl AuiNotebook {
    /// Creates a new AuiNotebook from a raw pointer.
    /// This is intended for internal use by the builder.
    fn from_ptr(ptr: *mut ffi::wxd_AuiNotebook_t) -> Self {
        AuiNotebook {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Creates a new builder for AuiNotebook
    pub fn builder<'a>(parent: &'a dyn WxWidget) -> AuiNotebookBuilder<'a> {
        AuiNotebookBuilder::new(parent)
    }

    /// Helper to get raw notebook pointer, returns null if widget has been destroyed
    #[inline]
    fn notebook_ptr(&self) -> *mut ffi::wxd_AuiNotebook_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_AuiNotebook_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Adds a page to the notebook.
    /// No-op if the notebook has been destroyed.
    /// Returns false if the notebook has been destroyed or the operation fails.
    pub fn add_page(&self, page: &impl WxWidget, caption: &str, select: bool) -> bool {
        let ptr = self.notebook_ptr();
        if ptr.is_null() {
            return false;
        }
        let caption_c = CString::new(caption).expect("CString::new failed for caption");
        unsafe {
            // Pass -1 for bitmap_id as a default, assuming no specific bitmap support yet in this wrapper
            ffi::wxd_AuiNotebook_AddPage(ptr, page.handle_ptr(), caption_c.as_ptr(), select, -1)
        }
    }

    /// Returns the number of pages in the notebook.
    /// Returns 0 if the notebook has been destroyed.
    pub fn page_count(&self) -> usize {
        let ptr = self.notebook_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_AuiNotebook_GetPageCount(ptr) as usize }
    }

    /// Sets the currently selected page.
    /// Returns 0 if the notebook has been destroyed.
    pub fn set_selection(&self, new_page: usize) -> usize {
        let ptr = self.notebook_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_AuiNotebook_SetSelection(ptr, new_page) as usize }
    }

    /// Returns the underlying WindowHandle for this notebook.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    // Add other methods like get_page, insert_page, remove_page etc. as needed
}

// Manual WxWidget implementation for AuiNotebook (using WindowHandle)
impl WxWidget for AuiNotebook {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for AuiNotebook {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for AuiNotebook {}

// Use widget_builder macro to create the builder
widget_builder!(
    name: AuiNotebook,
    parent_type: &'a dyn WxWidget,
    style_type: AuiNotebookStyle,
    fields: {},
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let ptr = unsafe {
            ffi::wxd_AuiNotebook_Create(
                parent_ptr,
                slf.id,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits(),
            )
        };
        if ptr.is_null() {
            panic!("Failed to create AuiNotebook");
        }
        // Create a WindowHandle which automatically registers for destroy events
        AuiNotebook::from_ptr(ptr)
    }
);
