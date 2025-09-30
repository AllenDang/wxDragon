//!
//! Safe wrapper for wxNotebook.

use crate::event::{Event, EventType};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::widgets::imagelist::ImageList;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// --- Style enum using macro ---
widget_style_enum!(
    name: NotebookStyle,
    doc: "Window style flags for Notebook",
    variants: {
        Default: ffi::WXD_NB_DEFAULT, "Default style, tabs at the top.",
        Top: ffi::WXD_NB_TOP, "Place tabs at the top.",
        Bottom: ffi::WXD_NB_BOTTOM, "Place tabs at the bottom.",
        Left: ffi::WXD_NB_LEFT, "Place tabs on the left.",
        Right: ffi::WXD_NB_RIGHT, "Place tabs on the right.",
        FixedWidth: ffi::WXD_NB_FIXEDWIDTH, "Display all tabs in a single row, possibly with arrows if too many.",
        Multiline: ffi::WXD_NB_MULTILINE, "Allow multiple lines of tabs.",
        NoPageTheme: ffi::WXD_NB_NOPAGETHEME, "Under MSW, don't draw the page theme (allows pages to have individual colours)."
    },
    default_variant: Default
);

/// Represents a wxNotebook widget.
#[derive(Clone)]
pub struct Notebook {
    window: Window,
}

impl Notebook {
    /// Creates a new Notebook builder.
    pub fn builder(parent: &dyn WxWidget) -> NotebookBuilder<'_> {
        NotebookBuilder::new(parent)
    }

    // Internal constructor
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Notebook_t) -> Self {
        Notebook {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Adds a new page to the notebook.
    ///
    /// # Arguments
    /// * `page` - The window to be added as a page.
    /// * `text` - The text for the page's tab.
    /// * `select` - If `true`, selects the page after adding it.
    /// * `image_id` - Optional image index for the page's tab.
    ///
    /// Returns `true` if the page was added successfully.
    pub fn add_page<W: WxWidget>(
        &self,
        page: &W,
        text: &str,
        select: bool,
        image_id: Option<i32>,
    ) -> bool {
        let c_text = CString::new(text).expect("CString::new failed");
        unsafe {
            if let Some(id) = image_id {
                ffi::wxd_Notebook_AddPageWithImageId(
                    self.window.as_ptr() as *mut ffi::wxd_Notebook_t,
                    page.handle_ptr(),
                    c_text.as_ptr(),
                    select,
                    id as c_int,
                )
            } else {
                ffi::wxd_Notebook_AddPage(
                    self.window.as_ptr() as *mut ffi::wxd_Notebook_t,
                    page.handle_ptr(),
                    c_text.as_ptr(),
                    select,
                )
            }
        }
    }

    /// Inserts a new page at the specified position.
    ///
    /// # Arguments
    /// * `index` - The position for the new page.
    /// * `page` - The window to be added as a page.
    /// * `text` - The text for the page's tab.
    /// * `select` - If `true`, selects the page after adding it.
    /// * `image_id` - Optional image index for the page's tab.
    ///
    /// Returns `true` if the page was inserted successfully.
    pub fn insert_page<W: WxWidget>(
        &self,
        index: usize,
        page: &W,
        text: &str,
        select: bool,
        image_id: Option<i32>,
    ) -> bool {
        let c_text = CString::new(text).expect("CString::new failed");
        unsafe {
            if let Some(id) = image_id {
                ffi::wxd_Notebook_InsertPageWithImageId(
                    self.window.as_ptr() as *mut ffi::wxd_Notebook_t,
                    index,
                    page.handle_ptr(),
                    c_text.as_ptr(),
                    select,
                    id as c_int,
                )
            } else {
                ffi::wxd_Notebook_InsertPage(
                    self.window.as_ptr() as *mut ffi::wxd_Notebook_t,
                    index,
                    page.handle_ptr(),
                    c_text.as_ptr(),
                    select,
                )
            }
        }
    }

    /// Gets the index of the currently selected page.
    /// Returns `wxNOT_FOUND` (-1) if no page is selected.
    pub fn selection(&self) -> i32 {
        unsafe { ffi::wxd_Notebook_GetSelection(self.window.as_ptr() as *mut ffi::wxd_Notebook_t) }
    }

    /// Sets the selection to the given page index.
    /// Returns the index of the previously selected page.
    pub fn set_selection(&self, page: usize) -> i32 {
        unsafe {
            ffi::wxd_Notebook_SetSelection(
                self.window.as_ptr() as *mut ffi::wxd_Notebook_t,
                page as c_int,
            )
        }
    }

    /// Changes the selection to the given page, returning the old selection.
    /// This function does not generate a `EVT_NOTEBOOK_PAGE_CHANGING` event.
    pub fn change_selection(&self, page: usize) -> i32 {
        unsafe {
            ffi::wxd_Notebook_ChangeSelection(
                self.window.as_ptr() as *mut ffi::wxd_Notebook_t,
                page,
            )
        }
    }

    /// Advances the selection, optionally wrapping to the beginning/end.
    pub fn advance_selection(&self, forward: bool) {
        unsafe {
            ffi::wxd_Notebook_AdvanceSelection(
                self.window.as_ptr() as *mut ffi::wxd_Notebook_t,
                forward,
            )
        }
    }

    /// Sets the amount of space around the icon and label in a tab.
    pub fn set_padding(&self, padding: Size) {
        unsafe {
            ffi::wxd_Notebook_SetPadding(
                self.window.as_ptr() as *mut ffi::wxd_Notebook_t,
                padding.into(),
            )
        }
    }

    /// Sets the image list for the notebook.
    /// The notebook takes ownership of the image list.
    pub fn set_image_list(&self, image_list: ImageList) {
        unsafe {
            ffi::wxd_Notebook_SetImageList(
                self.window.as_ptr() as *mut ffi::wxd_Notebook_t,
                image_list.as_ptr(),
            );
        }
        // wxNotebook takes ownership of the ImageList, so we forget it in Rust
        // to prevent a double free.
        std::mem::forget(image_list);
    }

    /// Gets the image list associated with the notebook.
    /// The notebook owns the image list, so the caller should not delete it.
    pub fn get_image_list(&self) -> Option<ImageList> {
        let ptr = unsafe {
            ffi::wxd_Notebook_GetImageList(self.window.as_ptr() as *mut ffi::wxd_Notebook_t)
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { ImageList::from_ptr_unowned(ptr) })
        }
    }

    /// Gets the number of pages in the notebook.
    pub fn get_page_count(&self) -> usize {
        unsafe { ffi::wxd_Notebook_GetPageCount(self.window.as_ptr() as *mut ffi::wxd_Notebook_t) }
    }

    /// Returns the window at the given page position.
    /// Returns `None` if the page index is out of bounds.
    pub fn get_page(&self, index: usize) -> Option<Window> {
        unsafe {
            let ptr =
                ffi::wxd_Notebook_GetPage(self.window.as_ptr() as *mut ffi::wxd_Notebook_t, index);
            if ptr.is_null() {
                None
            } else {
                Some(Window::from_ptr(ptr))
            }
        }
    }

    /// Removes the page at the given index.
    /// Returns `true` if the page was removed successfully.
    pub fn remove_page(&self, index: usize) -> bool {
        unsafe {
            ffi::wxd_Notebook_RemovePage(self.window.as_ptr() as *mut ffi::wxd_Notebook_t, index)
        }
    }
}

// Apply common trait implementations for Notebook
implement_widget_traits_with_target!(Notebook, window, Window);

// Use the widget_builder macro to generate the NotebookBuilder implementation
widget_builder!(
    name: Notebook,
    parent_type: &'a dyn WxWidget,
    style_type: NotebookStyle,
    fields: {},
    build_impl: |slf| {
        let notebook_ptr = unsafe {
            ffi::wxd_Notebook_Create(
                slf.parent.handle_ptr(),
                slf.id as c_int,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            )
        };
        if notebook_ptr.is_null() {
            panic!("Failed to create Notebook");
        }
        unsafe { Notebook::from_ptr(notebook_ptr) }
    }
);

/// Events that can be emitted by a `Notebook`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotebookEvent {
    /// A notebook page has been changed.
    PageChanged,
}

/// Event data for a `Notebook::PageChanged` event.
#[derive(Debug)]
pub struct NotebookPageChangedEvent {
    /// The base event data.
    pub base: Event,
}

impl NotebookPageChangedEvent {
    /// Creates new `NotebookPageChangedEvent` from a base `Event`.
    pub fn new(base_event: Event) -> Self {
        Self { base: base_event }
    }

    /// Gets the page that has been selected.
    /// For a `PageChanged` event, this is the new page.
    pub fn get_selection(&self) -> Option<i32> {
        if self.base.is_null() {
            return None;
        }
        let val = unsafe { ffi::wxd_NotebookEvent_GetSelection(self.base.0) };
        if val == ffi::WXD_NOT_FOUND as i32 {
            None
        } else {
            Some(val)
        }
    }

    /// Gets the page that was selected before the change.
    /// For a `PageChanged` event, this is the old page.
    pub fn get_old_selection(&self) -> Option<i32> {
        if self.base.is_null() {
            return None;
        }
        let val = unsafe { ffi::wxd_NotebookEvent_GetOldSelection(self.base.0) };
        if val == ffi::WXD_NOT_FOUND as i32 {
            None
        } else {
            Some(val)
        }
    }
}

// Use the implement_widget_local_event_handlers macro for notebook events
crate::implement_widget_local_event_handlers!(
    Notebook, NotebookEvent, NotebookPageChangedEvent,
    PageChanged => page_changed, EventType::NOTEBOOK_PAGE_CHANGED
);

// Add XRC Support - enables Notebook to be created from XRC-managed pointers
impl_xrc_support!(Notebook, { window });

// Widget casting support for Notebook
impl_widget_cast!(Notebook, "wxNotebook", { window });
