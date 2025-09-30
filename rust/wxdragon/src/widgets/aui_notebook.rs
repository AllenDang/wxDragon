use crate::prelude::*;
use crate::window::Window;
use std::ffi::CString;
use std::marker::PhantomData;
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

#[derive(Clone)]
pub struct AuiNotebook {
    window: Window,
    #[allow(dead_code)]
    parent_ptr: *mut ffi::wxd_Window_t,
    _marker: PhantomData<()>,
}

impl AuiNotebook {
    fn from_ptr(ptr: *mut ffi::wxd_AuiNotebook_t, parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        AuiNotebook {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
            parent_ptr,
            _marker: PhantomData,
        }
    }

    /// Creates a new builder for AuiNotebook
    pub fn builder<'a>(parent: &'a dyn WxWidget) -> AuiNotebookBuilder<'a> {
        AuiNotebookBuilder::new(parent)
    }

    /// Adds a page to the notebook
    pub fn add_page(&self, page: &impl WxWidget, caption: &str, select: bool) -> bool {
        let caption_c = CString::new(caption).expect("CString::new failed for caption");
        unsafe {
            // Pass -1 for bitmap_id as a default, assuming no specific bitmap support yet in this wrapper
            ffi::wxd_AuiNotebook_AddPage(
                self.window.handle_ptr() as *mut ffi::wxd_AuiNotebook_t,
                page.handle_ptr(),
                caption_c.as_ptr(),
                select,
                -1,
            )
        }
    }

    /// Returns the number of pages in the notebook
    pub fn page_count(&self) -> usize {
        unsafe {
            ffi::wxd_AuiNotebook_GetPageCount(
                self.window.handle_ptr() as *mut ffi::wxd_AuiNotebook_t
            ) as usize
        }
    }

    /// Sets the currently selected page
    pub fn set_selection(&self, new_page: usize) -> usize {
        unsafe {
            ffi::wxd_AuiNotebook_SetSelection(
                self.window.handle_ptr() as *mut ffi::wxd_AuiNotebook_t,
                new_page,
            ) as usize
        }
    }

    // Add other methods like get_page, insert_page, remove_page etc. as needed
}

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
        AuiNotebook::from_ptr(ptr, parent_ptr)
    }
);

// Implement all standard widget traits in one go
implement_widget_traits_with_target!(AuiNotebook, window, Window);
