//!
//! Safe wrapper for wxSimpleBook.

use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{Window, WindowHandle, WxWidget};
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// --- Style enum using macro ---
widget_style_enum!(
    name: SimpleBookStyle,
    doc: "Window style flags for SimpleBook",
    variants: {
        Default: ffi::WXD_BK_DEFAULT, "Default style.",
        Top: ffi::WXD_BK_TOP, "Place pages at the top (no visual effect for SimpleBook).",
        Bottom: ffi::WXD_BK_BOTTOM, "Place pages at the bottom (no visual effect for SimpleBook).",
        Left: ffi::WXD_BK_LEFT, "Place pages on the left (no visual effect for SimpleBook).",
        Right: ffi::WXD_BK_RIGHT, "Place pages on the right (no visual effect for SimpleBook)."
    },
    default_variant: Default
);

/// Represents a wxSimpleBook widget.
///
/// wxSimpleBook is a book control without visual tabs. Pages are switched programmatically,
/// not through visible tabs. This makes it ideal for wizard-like interfaces or when you
/// want to control page navigation through other UI elements.
///
/// SimpleBook uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct SimpleBook {
    /// Safe handle to the underlying wxSimpleBook - automatically invalidated on destroy
    handle: WindowHandle,
}

impl SimpleBook {
    /// Creates a new SimpleBook builder.
    pub fn builder(parent: &dyn WxWidget) -> SimpleBookBuilder<'_> {
        SimpleBookBuilder::new(parent)
    }

    // Internal constructor
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SimpleBook_t) -> Self {
        SimpleBook {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw simplebook pointer, returns null if widget has been destroyed
    #[inline]
    fn simplebook_ptr(&self) -> *mut ffi::wxd_SimpleBook_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_SimpleBook_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Adds a new page to the simplebook.
    ///
    /// # Arguments
    /// * `page` - The window to be added as a page.
    /// * `text` - The text for the page (stored internally but not visually displayed).
    /// * `select` - If `true`, selects the page after adding it.
    /// * `image_id` - Optional image index (ignored for SimpleBook as it has no tabs).
    ///
    /// Returns `true` if the page was added successfully.
    /// Returns `false` if the simplebook has been destroyed.
    pub fn add_page<W: WxWidget>(&self, page: &W, text: &str, select: bool, image_id: Option<i32>) -> bool {
        let ptr = self.simplebook_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_text = CString::new(text).expect("CString::new failed");
        unsafe {
            if let Some(id) = image_id {
                ffi::wxd_SimpleBook_AddPageWithImageId(ptr, page.handle_ptr(), c_text.as_ptr(), select, id as c_int)
            } else {
                ffi::wxd_SimpleBook_AddPage(ptr, page.handle_ptr(), c_text.as_ptr(), select)
            }
        }
    }

    /// Inserts a new page at the specified position.
    ///
    /// # Arguments
    /// * `index` - The position for the new page.
    /// * `page` - The window to be added as a page.
    /// * `text` - The text for the page (stored internally but not visually displayed).
    /// * `select` - If `true`, selects the page after adding it.
    /// * `image_id` - Optional image index (ignored for SimpleBook as it has no tabs).
    ///
    /// Returns `true` if the page was inserted successfully.
    /// Returns `false` if the simplebook has been destroyed.
    pub fn insert_page<W: WxWidget>(&self, index: usize, page: &W, text: &str, select: bool, image_id: Option<i32>) -> bool {
        let ptr = self.simplebook_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_text = CString::new(text).expect("CString::new failed");
        unsafe {
            if let Some(id) = image_id {
                ffi::wxd_SimpleBook_InsertPageWithImageId(ptr, index, page.handle_ptr(), c_text.as_ptr(), select, id as c_int)
            } else {
                ffi::wxd_SimpleBook_InsertPage(ptr, index, page.handle_ptr(), c_text.as_ptr(), select)
            }
        }
    }

    /// Gets the index of the currently selected page.
    /// Returns `wxNOT_FOUND` (-1) if no page is selected or if the simplebook has been destroyed.
    pub fn selection(&self) -> i32 {
        let ptr = self.simplebook_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_SimpleBook_GetSelection(ptr) }
    }

    /// Sets the selection to the given page index.
    /// Returns the index of the previously selected page.
    /// Returns -1 if the simplebook has been destroyed.
    pub fn set_selection(&self, page: usize) -> i32 {
        let ptr = self.simplebook_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_SimpleBook_SetSelection(ptr, page as c_int) }
    }

    /// Changes the selection to the given page, returning the old selection.
    /// This function does not generate a `EVT_BOOKCTRL_PAGE_CHANGING` event.
    /// Returns -1 if the simplebook has been destroyed.
    pub fn change_selection(&self, page: usize) -> i32 {
        let ptr = self.simplebook_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_SimpleBook_ChangeSelection(ptr, page) }
    }

    /// Gets the number of pages in the simplebook.
    /// Returns 0 if the simplebook has been destroyed.
    pub fn get_page_count(&self) -> usize {
        let ptr = self.simplebook_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_SimpleBook_GetPageCount(ptr) }
    }

    /// Returns the window at the given page position.
    /// Returns `None` if the page index is out of bounds or if the simplebook has been destroyed.
    pub fn get_page(&self, index: usize) -> Option<Window> {
        let ptr = self.simplebook_ptr();
        if ptr.is_null() {
            return None;
        }
        unsafe {
            let page_ptr = ffi::wxd_SimpleBook_GetPage(ptr, index);
            if page_ptr.is_null() {
                None
            } else {
                Some(Window::from_ptr(page_ptr))
            }
        }
    }

    /// Removes the page at the given index.
    /// Returns `true` if the page was removed successfully.
    /// Returns `false` if the simplebook has been destroyed.
    pub fn remove_page(&self, index: usize) -> bool {
        let ptr = self.simplebook_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_SimpleBook_RemovePage(ptr, index) }
    }

    /// Returns the underlying WindowHandle for this simplebook.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for SimpleBook (using WindowHandle)
impl WxWidget for SimpleBook {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for SimpleBook {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for SimpleBook {}

// Use the widget_builder macro to generate the SimpleBookBuilder implementation
widget_builder!(
    name: SimpleBook,
    parent_type: &'a dyn WxWidget,
    style_type: SimpleBookStyle,
    fields: {},
    build_impl: |slf| {
        let simplebook_ptr = unsafe {
            ffi::wxd_SimpleBook_Create(
                slf.parent.handle_ptr(),
                slf.id as c_int,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            )
        };
        if simplebook_ptr.is_null() {
            panic!("Failed to create SimpleBook");
        }
        unsafe { SimpleBook::from_ptr(simplebook_ptr) }
    }
);

/// Events that can be emitted by a `SimpleBook`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleBookEvent {
    /// A simplebook page has been changed.
    PageChanged,
}

/// Event data for a `SimpleBook::PageChanged` event.
#[derive(Debug)]
pub struct SimpleBookPageChangedEvent {
    /// The base event data.
    pub base: Event,
}

impl SimpleBookPageChangedEvent {
    /// Creates new `SimpleBookPageChangedEvent` from a base `Event`.
    pub fn new(base_event: Event) -> Self {
        Self { base: base_event }
    }

    /// Gets the page that has been selected.
    /// For a `PageChanged` event, this is the new page.
    pub fn get_selection(&self) -> Option<i32> {
        if self.base.is_null() {
            return None;
        }
        // SimpleBook uses the same event infrastructure as Notebook
        let val = unsafe { ffi::wxd_NotebookEvent_GetSelection(self.base.0) };
        if val == ffi::WXD_NOT_FOUND as i32 { None } else { Some(val) }
    }

    /// Gets the page that was selected before the change.
    /// For a `PageChanged` event, this is the old page.
    pub fn get_old_selection(&self) -> Option<i32> {
        if self.base.is_null() {
            return None;
        }
        // SimpleBook uses the same event infrastructure as Notebook
        let val = unsafe { ffi::wxd_NotebookEvent_GetOldSelection(self.base.0) };
        if val == ffi::WXD_NOT_FOUND as i32 { None } else { Some(val) }
    }
}

// Use the implement_widget_local_event_handlers macro for simplebook events
crate::implement_widget_local_event_handlers!(
    SimpleBook, SimpleBookEvent, SimpleBookPageChangedEvent,
    PageChanged => page_changed, EventType::NOTEBOOK_PAGE_CHANGED
);

// XRC Support - enables SimpleBook to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for SimpleBook {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        SimpleBook {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for SimpleBook
impl crate::window::FromWindowWithClassName for SimpleBook {
    fn class_name() -> &'static str {
        "wxSimplebook"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        SimpleBook {
            handle: WindowHandle::new(ptr),
        }
    }
}
