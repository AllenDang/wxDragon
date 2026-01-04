//!
//! Safe wrapper for wxTreebook.

use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::CString;
use wxdragon_sys as ffi;

// --- Treebook Styles ---
widget_style_enum!(
    name: TreebookStyle,
    doc: "Style flags for Treebook widget.",
    variants: {
        Default: ffi::WXD_BK_DEFAULT, "Default style.",
        Top: ffi::WXD_BK_TOP, "Place tabs at the top.",
        Bottom: ffi::WXD_BK_BOTTOM, "Place tabs at the bottom.",
        Left: ffi::WXD_BK_LEFT, "Place tabs at the left.",
        Right: ffi::WXD_BK_RIGHT, "Place tabs at the right."
    },
    default_variant: Default
);

/// Events emitted by Treebook
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreebookEvent {
    /// Emitted when the selected page changes
    PageChanged,
    /// Emitted when the selected page is about to change
    PageChanging,
    /// Emitted when a tree node is expanded
    NodeExpanded,
    /// Emitted when a tree node is collapsed
    NodeCollapsed,
}

/// Event data for Treebook page changed/changing events
#[derive(Debug)]
pub struct TreebookEventData {
    base: Event,
}

impl TreebookEventData {
    /// Create a new NotebookEventData with the provided Event
    pub fn new(event: Event) -> Self {
        Self { base: event }
    }

    /// Skip this event (allowing the default processing to occur)
    pub fn skip(&self, skip: bool) {
        self.base.skip(skip);
    }

    /// Get the ID of the control that generated the event
    pub fn get_id(&self) -> i32 {
        self.base.get_id()
    }

    /// Gets the page that has been selected.
    /// For a `PageChanged` event, this is the new page.
    pub fn get_selection(&self) -> Option<i32> {
        if self.base.is_null() {
            return None;
        }
        let val = unsafe { ffi::wxd_NotebookEvent_GetSelection(self.base.0) };
        if val == ffi::WXD_NOT_FOUND as i32 { None } else { Some(val) }
    }

    /// Gets the page that was selected before the change.
    /// For a `PageChanged` event, this is the old page.
    pub fn get_old_selection(&self) -> Option<i32> {
        if self.base.is_null() {
            return None;
        }
        let val = unsafe { ffi::wxd_NotebookEvent_GetOldSelection(self.base.0) };
        if val == ffi::WXD_NOT_FOUND as i32 { None } else { Some(val) }
    }
}

/// Represents a wxTreebook control.
///
/// Treebook uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let treebook = Treebook::builder(&frame).build();
///
/// // Treebook is Copy - no clone needed for closures!
/// treebook.bind_page_changed(move |_| {
///     // Safe: if treebook was destroyed, this is a no-op
///     let selection = treebook.get_selection();
/// });
///
/// // After parent destruction, treebook operations are safe no-ops
/// frame.destroy();
/// assert!(!treebook.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct Treebook {
    /// Safe handle to the underlying wxTreebook - automatically invalidated on destroy
    handle: WindowHandle,
}

impl Treebook {
    /// Creates a new Treebook builder.
    pub fn builder(parent: &dyn WxWidget) -> TreebookBuilder<'_> {
        TreebookBuilder::new(parent)
    }

    // Internal constructor
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Treebook_t) -> Self {
        Treebook {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw treebook pointer, returns null if widget has been destroyed
    #[inline]
    fn treebook_ptr(&self) -> *mut ffi::wxd_Treebook_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_Treebook_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Internal implementation used by the builder.
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, pos: Point, size: Size, style: i64) -> Self {
        assert!(!parent_ptr.is_null(), "Treebook parent cannot be null");

        let ptr = unsafe {
            ffi::wxd_Treebook_new(
                parent_ptr,
                id,
                pos.x,
                pos.y,
                size.width,
                size.height,
                style as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create wxTreebook");
        }

        unsafe { Treebook::from_ptr(ptr) }
    }

    /// Adds a new page to the treebook control.
    /// Returns the index of the added page, or -1 on failure.
    /// No-op if the treebook has been destroyed.
    pub fn add_page<W: WxWidget>(&self, page: &W, text: &str, select: bool, image_id: i32) -> i32 {
        let ptr = self.treebook_ptr();
        if ptr.is_null() {
            return -1;
        }
        let page_ptr = page.handle_ptr();
        let text_c = CString::new(text).unwrap_or_default();
        unsafe { ffi::wxd_Treebook_AddPage(ptr, page_ptr, text_c.as_ptr(), select as i32, image_id) }
    }

    /// Adds a new sub-page to the last top-level page added to the treebook control.
    /// Returns the index of the added page, or -1 on failure.
    /// No-op if the treebook has been destroyed.
    pub fn add_sub_page<W: WxWidget>(&self, page: &W, text: &str, select: bool, image_id: i32) -> i32 {
        let ptr = self.treebook_ptr();
        if ptr.is_null() {
            return -1;
        }
        let page_ptr = page.handle_ptr();
        let text_c = CString::new(text).unwrap_or_default();
        unsafe { ffi::wxd_Treebook_AddSubPage(ptr, page_ptr, text_c.as_ptr(), select as i32, image_id) }
    }

    /// Gets the number of pages in the treebook.
    /// Returns 0 if the treebook has been destroyed.
    pub fn get_page_count(&self) -> i32 {
        let ptr = self.treebook_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Treebook_GetPageCount(ptr) }
    }

    /// Gets the currently selected page, or -1 if none is selected.
    /// Returns -1 if the treebook has been destroyed.
    pub fn get_selection(&self) -> i32 {
        let ptr = self.treebook_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Treebook_GetSelection(ptr) }
    }

    /// Sets the selection to the given page index.
    /// Returns the index of the previously selected page.
    /// Returns -1 if the treebook has been destroyed.
    pub fn set_selection(&self, n: usize) -> i32 {
        let ptr = self.treebook_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Treebook_SetSelection(ptr, n) }
    }

    /// Sets the text for the given page.
    /// No-op if the treebook has been destroyed.
    pub fn set_page_text(&self, n: usize, text: &str) {
        let ptr = self.treebook_ptr();
        if ptr.is_null() {
            return;
        }
        let text_c = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_Treebook_SetPageText(ptr, n, text_c.as_ptr());
        }
    }

    /// Gets the text for the given page.
    /// Returns empty string if the treebook has been destroyed.
    pub fn get_page_text(&self, n: usize) -> String {
        let ptr = self.treebook_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            // First call to get the size needed
            let needed_len_with_null = ffi::wxd_Treebook_GetPageText(ptr, n, std::ptr::null_mut(), 0);
            if needed_len_with_null <= 1 {
                // 0 or 1 means error or empty string
                return String::new();
            }

            let buffer_size = needed_len_with_null as usize;
            let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);

            // Second call to actually get the string
            let copied_len_with_null = ffi::wxd_Treebook_GetPageText(ptr, n, buffer.as_mut_ptr() as *mut i8, buffer_size as i32);

            if copied_len_with_null <= 0 {
                // Check for error on second call
                return String::new();
            }

            // Assuming the C++ side returned needed size including null,
            // and successfully copied that amount (or truncated).
            // The actual number of bytes excluding null is needed_len_with_null - 1.
            let actual_len = (copied_len_with_null - 1) as usize;
            buffer.set_len(actual_len.min(buffer_size - 1)); // Set length to actual content size (excluding null)

            String::from_utf8_lossy(&buffer).into_owned()
        }
    }

    /// Returns the underlying WindowHandle for this treebook.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
    // TODO: Add other wxBookCtrlBase methods like GetPage, InsertPage, DeletePage etc.
}

// Manual WxWidget implementation for Treebook (using WindowHandle)
impl WxWidget for Treebook {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for Treebook {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for Treebook {}

// Use the widget_builder macro for Treebook
widget_builder!(
    name: Treebook,
    parent_type: &'a dyn WxWidget,
    style_type: TreebookStyle,
    fields: {},
    build_impl: |slf| {
        Treebook::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
);

// Implement Treebook-specific event handlers
crate::implement_widget_local_event_handlers!(
    Treebook,
    TreebookEvent,
    TreebookEventData,
    PageChanged => page_changed, EventType::TREEBOOK_PAGE_CHANGED,
    PageChanging => page_changing, EventType::TREEBOOK_PAGE_CHANGING,
    NodeExpanded => node_expanded, EventType::TREEBOOK_NODE_EXPANDED,
    NodeCollapsed => node_collapsed, EventType::TREEBOOK_NODE_COLLAPSED
);

// XRC Support - enables Treebook to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for Treebook {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Treebook {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for Treebook
impl crate::window::FromWindowWithClassName for Treebook {
    fn class_name() -> &'static str {
        "wxTreebook"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Treebook {
            handle: WindowHandle::new(ptr),
        }
    }
}
