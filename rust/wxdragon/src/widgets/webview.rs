//! Safe wrapper for wxWebView.

use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by new_from_composition for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::CString;
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// WebView Zoom Types
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebViewZoomType {
    Layout = 0,
    Text = 1,
}

impl From<WebViewZoomType> for i32 {
    fn from(val: WebViewZoomType) -> Self {
        val as i32
    }
}

// WebView Zoom Levels (Standard levels, though it can be arbitrary)
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebViewZoom {
    Tiny = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
    Largest = 4,
}

impl From<WebViewZoom> for i32 {
    fn from(val: WebViewZoom) -> Self {
        val as i32
    }
}

// WebView Reload Flags
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebViewReloadFlags {
    Default = 0,
    NoCache = 1,
}

impl From<WebViewReloadFlags> for i32 {
    fn from(val: WebViewReloadFlags) -> Self {
        val as i32
    }
}

// WebView Find Flags
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WebViewFindFlags: i32 {
        const WRAP = 0x0001;
        const ENTIRE_WORD = 0x0002;
        const MATCH_CASE = 0x0004;
        const HIGHLIGHT_RESULT = 0x0008;
        const BACKWARDS = 0x0010;
        const DEFAULT = 0;
    }
}

// WebView User Script Injection Time
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebViewUserScriptInjectionTime {
    AtDocumentStart = 0,
    AtDocumentEnd = 1,
}

impl From<WebViewUserScriptInjectionTime> for i32 {
    fn from(val: WebViewUserScriptInjectionTime) -> Self {
        val as i32
    }
}

// WebView Navigation Error
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebViewNavigationError {
    Connection = 0,
    Certificate = 1,
    Auth = 2,
    Security = 3,
    NotFound = 4,
    Request = 5,
    UserCancelled = 6,
    Other = 7,
}

// WebView Browsing Data Types
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WebViewBrowsingDataTypes: i32 {
        const COOKIES = 0x01;
        const CACHE = 0x02;
        const DOM_STORAGE = 0x04;
        const OTHER = 0x08;
        const ALL = 0x0f;
    }
}

/// WebView Backend selection.
///
/// # Platform Support
/// - **Windows**: Prefers Edge (WebView2/Chromium) when available, falls back to IE (Trident).
///   The Edge backend requires the WebView2 runtime to be installed.
/// - **macOS**: Uses WebKit (Safari engine).
/// - **Linux**: Uses WebKit2GTK.
///
/// # IE Backend Limitations
/// The IE backend (used when Edge/WebView2 is not available on Windows) has significant
/// limitations:
/// - Many modern websites may not render correctly or may show a white screen
/// - Some zoom operations are not fully supported
/// - JavaScript execution may be limited
/// - For best results on Windows, ensure the WebView2 runtime is installed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WebViewBackend {
    /// Default backend for the current platform.
    /// Uses the platform's native web view implementation.
    #[default]
    Default,
    /// Legacy Internet Explorer (Trident) backend for Windows.
    /// Limited compatibility with modern websites.
    IE,
    /// Modern Edge (WebView2/Chromium) backend for Windows.
    /// Requires WebView2 runtime.
    Edge,
    /// WebKit backend for macOS and Linux.
    WebKit,
}

impl WebViewBackend {
    /// Returns the wxWidgets backend identifier string.
    pub fn as_str(&self) -> &'static str {
        match self {
            WebViewBackend::Default => "",
            WebViewBackend::IE => "wxWebViewIE",
            WebViewBackend::Edge => "wxWebViewEdge",
            WebViewBackend::WebKit => "wxWebViewWebKit",
        }
    }
}

impl std::fmt::Display for WebViewBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Represents a wxWebView widget.
///
/// WebView uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let webview = WebView::builder(&frame).url("https://example.com").build();
///
/// // WebView is Copy - no clone needed for closures!
/// webview.bind_loaded(move |_| {
///     // Safe: if webview was destroyed, this is a no-op
///     webview.load_url("https://rust-lang.org");
/// });
///
/// // After parent destruction, webview operations are safe no-ops
/// frame.destroy();
/// assert!(!webview.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct WebView {
    /// Safe handle to the underlying wxWebView - automatically invalidated on destroy
    handle: WindowHandle,
}

impl WebView {
    /// Creates a new WebView builder.
    pub fn builder(parent: &dyn WxWidget) -> WebViewBuilder<'_> {
        WebViewBuilder::new(parent)
    }

    /// Creates a new WebView from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Creates a new WebView from a raw window pointer.
    /// This is for backwards compatibility with widgets that compose WebView.
    /// The parent_ptr parameter is ignored (kept for API compatibility).
    #[allow(dead_code)]
    pub(crate) fn new_from_composition(_window: Window, _parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        // Use the window's pointer to create a new WindowHandle
        Self {
            handle: WindowHandle::new(_window.as_ptr()),
        }
    }

    /// Creates a new WebView (low-level constructor used by the builder)
    #[allow(clippy::too_many_arguments)]
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        url: Option<&str>,
        pos: Point,
        size: Size,
        style: i64,
        name: Option<&str>,
        backend: Option<&str>,
    ) -> Self {
        assert!(!parent_ptr.is_null(), "WebView requires a parent");
        let c_url = url.map(|s| CString::new(s).unwrap_or_default());
        let c_name = name.map(|s| CString::new(s).unwrap_or_default());
        let c_backend = backend.map(|s| CString::new(s).unwrap_or_default());

        // Get raw pointers while keeping CStrings alive
        let url_ptr = c_url.as_ref().map(|c| c.as_ptr()).unwrap_or(std::ptr::null());
        let name_ptr = c_name.as_ref().map(|c| c.as_ptr()).unwrap_or(std::ptr::null());
        let backend_ptr = c_backend.as_ref().map(|c| c.as_ptr()).unwrap_or(std::ptr::null());

        let ptr = unsafe {
            ffi::wxd_WebView_Create(
                parent_ptr,
                id,
                url_ptr,
                pos.into(),
                size.into(),
                style as _,
                name_ptr,
                backend_ptr,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create WebView widget");
        }

        // Note: Zoom operations on IE backend are disabled in the C++ layer
        // to avoid assertion failures. All zoom-related calls become no-ops on IE.

        // Create a WindowHandle which automatically registers for destroy events
        WebView {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw webview pointer, returns null if widget has been destroyed
    #[inline]
    fn webview_ptr(&self) -> *mut ffi::wxd_WebView_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_WebView_t)
            .unwrap_or(std::ptr::null_mut())
    }

    // --- Navigation ---

    /// Loads the specified URL.
    /// No-op if the webview has been destroyed.
    pub fn load_url(&self, url: &str) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        let c_url = CString::new(url).unwrap_or_default();
        unsafe { ffi::wxd_WebView_LoadURL(ptr, c_url.as_ptr()) };
    }

    /// Reloads the current page.
    /// No-op if the webview has been destroyed.
    pub fn reload(&self, flags: WebViewReloadFlags) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_Reload(ptr, flags.into()) };
    }

    /// Stops the current page loading.
    /// No-op if the webview has been destroyed.
    pub fn stop(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_Stop(ptr) };
    }

    /// Returns whether the webview can navigate back.
    /// Returns false if the webview has been destroyed.
    pub fn can_go_back(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_CanGoBack(ptr) }
    }

    /// Returns whether the webview can navigate forward.
    /// Returns false if the webview has been destroyed.
    pub fn can_go_forward(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_CanGoForward(ptr) }
    }

    /// Navigates back in the history.
    /// No-op if the webview has been destroyed.
    pub fn go_back(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_GoBack(ptr) };
    }

    /// Navigates forward in the history.
    /// No-op if the webview has been destroyed.
    pub fn go_forward(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_GoForward(ptr) };
    }

    /// Clears the navigation history.
    /// No-op if the webview has been destroyed.
    pub fn clear_history(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_ClearHistory(ptr) };
    }

    // --- State ---

    /// Returns whether the webview is currently busy loading a page.
    /// Returns false if the webview has been destroyed.
    pub fn is_busy(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_IsBusy(ptr) }
    }

    /// Returns the current URL.
    /// Returns empty string if the webview has been destroyed.
    pub fn get_current_url(&self) -> String {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 2048];
            let len = ffi::wxd_WebView_GetCurrentURL(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    /// Returns the current page title.
    /// Returns empty string if the webview has been destroyed.
    pub fn get_current_title(&self) -> String {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_WebView_GetCurrentTitle(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    /// Returns the page source (HTML).
    /// Returns empty string if the webview has been destroyed.
    pub fn get_page_source(&self) -> String {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return String::new();
        }
        // Page source can be large, use dynamic buffer resizing
        unsafe {
            // First call with moderate buffer to get the size
            let mut buffer: Vec<c_char> = vec![0; 1024 * 64]; // 64KB initial buffer
            let len = ffi::wxd_WebView_GetPageSource(ptr, buffer.as_mut_ptr(), buffer.len() as i32);

            if len < 0 {
                return String::new(); // Error
            }

            // Check if we need a larger buffer
            if len >= buffer.len() as i32 {
                // Allocate larger buffer and retry
                buffer = vec![0; len as usize + 1];
                let len2 = ffi::wxd_WebView_GetPageSource(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
                if len2 < 0 {
                    return String::new(); // Error on second call
                }
            }

            let actual_len = std::cmp::min(len as usize, buffer.len() - 1);
            let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, actual_len);
            String::from_utf8_lossy(byte_slice).to_string()
        }
    }

    /// Returns the page text content (without HTML tags).
    /// Returns empty string if the webview has been destroyed.
    pub fn get_page_text(&self) -> String {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return String::new();
        }
        // Page text can be large, use dynamic buffer resizing
        unsafe {
            // First call with moderate buffer to get the size
            let mut buffer: Vec<c_char> = vec![0; 1024 * 64]; // 64KB initial buffer
            let len = ffi::wxd_WebView_GetPageText(ptr, buffer.as_mut_ptr(), buffer.len() as i32);

            if len < 0 {
                return String::new(); // Error
            }

            // Check if we need a larger buffer
            if len >= buffer.len() as i32 {
                // Allocate larger buffer and retry
                buffer = vec![0; len as usize + 1];
                let len2 = ffi::wxd_WebView_GetPageText(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
                if len2 < 0 {
                    return String::new(); // Error on second call
                }
            }

            let actual_len = std::cmp::min(len as usize, buffer.len() - 1);
            let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, actual_len);
            String::from_utf8_lossy(byte_slice).to_string()
        }
    }

    // --- Zoom ---

    /// Returns whether the zoom type can be set.
    /// Returns false if the webview has been destroyed.
    pub fn can_set_zoom_type(&self, zoom_type: WebViewZoomType) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_CanSetZoomType(ptr, zoom_type.into()) }
    }

    /// Returns the current zoom level.
    /// Returns Medium if the webview has been destroyed.
    pub fn get_zoom(&self) -> WebViewZoom {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return WebViewZoom::Medium;
        }
        let val = unsafe { ffi::wxd_WebView_GetZoom(ptr) };
        match val {
            0 => WebViewZoom::Tiny,
            1 => WebViewZoom::Small,
            2 => WebViewZoom::Medium,
            3 => WebViewZoom::Large,
            4 => WebViewZoom::Largest,
            _ => WebViewZoom::Medium,
        }
    }

    /// Returns the current zoom type.
    /// Returns Layout if the webview has been destroyed.
    pub fn get_zoom_type(&self) -> WebViewZoomType {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return WebViewZoomType::Layout;
        }
        let val = unsafe { ffi::wxd_WebView_GetZoomType(ptr) };
        match val {
            0 => WebViewZoomType::Layout,
            1 => WebViewZoomType::Text,
            _ => WebViewZoomType::Layout,
        }
    }

    /// Sets the zoom level.
    /// No-op if the webview has been destroyed.
    pub fn set_zoom(&self, zoom: WebViewZoom) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_SetZoom(ptr, zoom.into()) };
    }

    /// Sets the zoom type.
    /// No-op if the webview has been destroyed.
    pub fn set_zoom_type(&self, zoom_type: WebViewZoomType) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_SetZoomType(ptr, zoom_type.into()) };
    }

    // --- Scripting ---

    /// Runs JavaScript code and returns the result.
    /// Returns None if the webview has been destroyed or if there was an error.
    pub fn run_script(&self, javascript: &str) -> Option<String> {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return None;
        }
        let c_script = CString::new(javascript).unwrap_or_default();
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 4096];
            let len = ffi::wxd_WebView_RunScript(ptr, c_script.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);

            if len < 0 {
                return None; // Error
            }

            // Check if we need a larger buffer
            if len >= buffer.len() as i32 {
                // Allocate larger buffer and retry
                buffer = vec![0; len as usize + 1];
                let len2 = ffi::wxd_WebView_RunScript(ptr, c_script.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);
                if len2 < 0 {
                    return None; // Error on second call
                }
            }

            let actual_len = std::cmp::min(len as usize, buffer.len() - 1);
            let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, actual_len);
            Some(String::from_utf8_lossy(byte_slice).to_string())
        }
    }

    // --- Clipboard ---

    /// Returns whether the webview can cut.
    /// Returns false if the webview has been destroyed.
    pub fn can_cut(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_CanCut(ptr) }
    }

    /// Returns whether the webview can copy.
    /// Returns false if the webview has been destroyed.
    pub fn can_copy(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_CanCopy(ptr) }
    }

    /// Returns whether the webview can paste.
    /// Returns false if the webview has been destroyed.
    pub fn can_paste(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_CanPaste(ptr) }
    }

    /// Cuts the selected content.
    /// No-op if the webview has been destroyed.
    pub fn cut(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_Cut(ptr) };
    }

    /// Copies the selected content.
    /// No-op if the webview has been destroyed.
    pub fn copy(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_Copy(ptr) };
    }

    /// Pastes content from clipboard.
    /// No-op if the webview has been destroyed.
    pub fn paste(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_Paste(ptr) };
    }

    /// Returns whether the webview can undo.
    /// Returns false if the webview has been destroyed.
    pub fn can_undo(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_CanUndo(ptr) }
    }

    /// Returns whether the webview can redo.
    /// Returns false if the webview has been destroyed.
    pub fn can_redo(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_CanRedo(ptr) }
    }

    /// Undoes the last action.
    /// No-op if the webview has been destroyed.
    pub fn undo(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_Undo(ptr) };
    }

    /// Redoes the last undone action.
    /// No-op if the webview has been destroyed.
    pub fn redo(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_Redo(ptr) };
    }

    // --- Selection ---

    /// Selects all content.
    /// No-op if the webview has been destroyed.
    pub fn select_all(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_SelectAll(ptr) };
    }

    /// Returns whether there is a selection.
    /// Returns false if the webview has been destroyed.
    pub fn has_selection(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_HasSelection(ptr) }
    }

    /// Deletes the current selection.
    /// No-op if the webview has been destroyed.
    pub fn delete_selection(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_DeleteSelection(ptr) };
    }

    /// Returns the selected text.
    /// Returns empty string if the webview has been destroyed.
    pub fn get_selected_text(&self) -> String {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 4096];
            let len = ffi::wxd_WebView_GetSelectedText(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    /// Returns the selected HTML source.
    /// Returns empty string if the webview has been destroyed.
    pub fn get_selected_source(&self) -> String {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 4096];
            let len = ffi::wxd_WebView_GetSelectedSource(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    /// Clears the current selection.
    /// No-op if the webview has been destroyed.
    pub fn clear_selection(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_ClearSelection(ptr) };
    }

    // --- Editing ---

    /// Returns whether the webview is editable.
    /// Returns false if the webview has been destroyed.
    pub fn is_editable(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_IsEditable(ptr) }
    }

    /// Sets whether the webview is editable.
    /// No-op if the webview has been destroyed.
    pub fn set_editable(&self, enable: bool) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_SetEditable(ptr, enable) };
    }

    // --- Printing ---

    /// Opens the print dialog.
    /// No-op if the webview has been destroyed.
    pub fn print(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_Print(ptr) };
    }

    // --- Context Menu & Dev Tools ---

    /// Enables or disables the context menu.
    /// No-op if the webview has been destroyed.
    pub fn enable_context_menu(&self, enable: bool) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_EnableContextMenu(ptr, enable) };
    }

    /// Returns whether the context menu is enabled.
    /// Returns false if the webview has been destroyed.
    pub fn is_context_menu_enabled(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_IsContextMenuEnabled(ptr) }
    }

    /// Enables or disables access to developer tools.
    /// No-op if the webview has been destroyed.
    pub fn enable_access_to_dev_tools(&self, enable: bool) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_EnableAccessToDevTools(ptr, enable) };
    }

    /// Returns whether access to developer tools is enabled.
    /// Returns false if the webview has been destroyed.
    pub fn is_access_to_dev_tools_enabled(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_IsAccessToDevToolsEnabled(ptr) }
    }

    /// Shows the developer tools.
    /// Returns false if the webview has been destroyed or if showing failed.
    pub fn show_dev_tools(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_ShowDevTools(ptr) }
    }

    /// Enables or disables browser accelerator keys.
    /// No-op if the webview has been destroyed.
    pub fn enable_browser_accelerator_keys(&self, enable: bool) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_EnableBrowserAcceleratorKeys(ptr, enable) };
    }

    /// Returns whether browser accelerator keys are enabled.
    /// Returns false if the webview has been destroyed.
    pub fn are_browser_accelerator_keys_enabled(&self) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_WebView_AreBrowserAcceleratorKeysEnabled(ptr) }
    }

    // --- Zoom Factor ---

    /// Returns the current zoom factor.
    /// Returns 1.0 if the webview has been destroyed.
    pub fn get_zoom_factor(&self) -> f32 {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return 1.0;
        }
        unsafe { ffi::wxd_WebView_GetZoomFactor(ptr) }
    }

    /// Sets the zoom factor.
    /// No-op if the webview has been destroyed.
    pub fn set_zoom_factor(&self, zoom: f32) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_SetZoomFactor(ptr, zoom) };
    }

    // --- Page Loading ---

    /// Sets the page content from HTML string.
    /// No-op if the webview has been destroyed.
    pub fn set_page(&self, html: &str, base_url: &str) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        let c_html = CString::new(html).unwrap_or_default();
        let c_base_url = CString::new(base_url).unwrap_or_default();
        unsafe {
            ffi::wxd_WebView_SetPage(ptr, c_html.as_ptr(), c_base_url.as_ptr());
        }
    }

    /// Finds text in the page.
    /// Returns 0 if the webview has been destroyed.
    pub fn find(&self, text: &str, flags: WebViewFindFlags) -> i64 {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return 0;
        }
        let c_text = CString::new(text).unwrap_or_default();
        unsafe { ffi::wxd_WebView_Find(ptr, c_text.as_ptr(), flags.bits()) as i64 }
    }

    // --- History ---

    /// Enables or disables history.
    /// No-op if the webview has been destroyed.
    pub fn enable_history(&self, enable: bool) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_EnableHistory(ptr, enable) };
    }

    // --- Configuration ---

    /// Sets the user agent string.
    /// Returns false if the webview has been destroyed.
    pub fn set_user_agent(&self, user_agent: &str) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_user_agent = CString::new(user_agent).unwrap_or_default();
        unsafe { ffi::wxd_WebView_SetUserAgent(ptr, c_user_agent.as_ptr()) }
    }

    /// Returns the user agent string.
    /// Returns empty string if the webview has been destroyed.
    pub fn get_user_agent(&self) -> String {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_WebView_GetUserAgent(ptr, buffer.as_mut_ptr(), buffer.len() as i32);

            if len < 0 {
                return String::new(); // Error
            }

            // Check if we need a larger buffer
            if len >= buffer.len() as i32 {
                // Allocate larger buffer and retry
                buffer = vec![0; len as usize + 1];
                let len2 = ffi::wxd_WebView_GetUserAgent(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
                if len2 < 0 {
                    return String::new(); // Error on second call
                }
            }

            let actual_len = std::cmp::min(len as usize, buffer.len() - 1);
            let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, actual_len);
            String::from_utf8_lossy(byte_slice).to_string()
        }
    }

    /// Sets the proxy configuration.
    /// Returns false if the webview has been destroyed.
    pub fn set_proxy(&self, proxy: &str) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_proxy = CString::new(proxy).unwrap_or_default();
        unsafe { ffi::wxd_WebView_SetProxy(ptr, c_proxy.as_ptr()) }
    }

    // --- Advanced Scripting ---

    /// Adds a script message handler.
    /// Returns false if the webview has been destroyed.
    pub fn add_script_message_handler(&self, name: &str) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_name = CString::new(name).unwrap_or_default();
        unsafe { ffi::wxd_WebView_AddScriptMessageHandler(ptr, c_name.as_ptr()) }
    }

    /// Removes a script message handler.
    /// Returns false if the webview has been destroyed.
    pub fn remove_script_message_handler(&self, name: &str) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_name = CString::new(name).unwrap_or_default();
        unsafe { ffi::wxd_WebView_RemoveScriptMessageHandler(ptr, c_name.as_ptr()) }
    }

    /// Adds a user script to be injected into pages.
    /// Returns false if the webview has been destroyed.
    pub fn add_user_script(&self, javascript: &str, injection_time: WebViewUserScriptInjectionTime) -> bool {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_javascript = CString::new(javascript).unwrap_or_default();
        unsafe { ffi::wxd_WebView_AddUserScript(ptr, c_javascript.as_ptr(), injection_time as i32) }
    }

    /// Removes all user scripts.
    /// No-op if the webview has been destroyed.
    pub fn remove_all_user_scripts(&self) {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_WebView_RemoveAllUserScripts(ptr) };
    }

    // --- Native Backend ---

    /// Returns a pointer to the native backend.
    /// Returns null if the webview has been destroyed.
    pub fn get_native_backend(&self) -> *mut std::os::raw::c_void {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return std::ptr::null_mut();
        }
        unsafe { ffi::wxd_WebView_GetNativeBackend(ptr) }
    }

    /// Returns the backend name.
    /// Returns empty string if the webview has been destroyed.
    pub fn get_backend(&self) -> String {
        let ptr = self.webview_ptr();
        if ptr.is_null() {
            return String::new();
        }
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 256];
            let len = ffi::wxd_WebView_GetBackend(ptr, buffer.as_mut_ptr(), buffer.len() as i32);

            if len < 0 {
                return String::new();
            }

            // Check if we need a larger buffer
            if len >= buffer.len() as i32 {
                buffer = vec![0; len as usize + 1];
                ffi::wxd_WebView_GetBackend(ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            }

            let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
            String::from_utf8_lossy(byte_slice).to_string()
        }
    }

    /// Checks if a specific WebView backend is available on the current system.
    ///
    /// # Arguments
    /// * `backend` - The backend to check.
    ///
    /// # Returns
    /// `true` if the backend is available and can be used, `false` otherwise.
    ///
    /// # Example
    /// ```no_run
    /// use wxdragon::widgets::{WebView, WebViewBackend};
    ///
    /// if WebView::is_backend_available(WebViewBackend::Edge) {
    ///     println!("Edge backend is available!");
    /// }
    /// ```
    pub fn is_backend_available(backend: WebViewBackend) -> bool {
        let c_backend = CString::new(backend.as_str()).unwrap_or_default();
        unsafe { ffi::wxd_WebView_IsBackendAvailable(c_backend.as_ptr()) }
    }

    /// Returns the underlying WindowHandle for this webview.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Implement WebViewEvents trait for WebView
#[cfg(feature = "webview")]
use crate::event::WebViewEvents;

#[cfg(feature = "webview")]
impl WebViewEvents for WebView {}

// Manual WxWidget implementation for WebView (using WindowHandle)
impl WxWidget for WebView {
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
impl WxEvtHandler for WebView {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for WebView {}

// Use the widget_builder macro to generate the WebViewBuilder implementation
widget_builder!(
    name: WebView,
    parent_type: &'a dyn WxWidget,
    style_type: WebViewStyle,
    fields: {
        url: Option<String> = None,
        name: String = "webView".to_string(),
        backend: WebViewBackend = WebViewBackend::Default
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        WebView::new_impl(
            parent_ptr,
            slf.id,
            slf.url.as_deref(),
            slf.pos,
            slf.size,
            slf.style.bits(),
            Some(slf.name.as_str()),
            Some(slf.backend.as_str()),
        )
    }
);

// XRC Support - enables WebView to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for WebView {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        WebView {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Note: WebView doesn't have XRC support in wxWidgets, so we don't provide it either
// Users should create WebView programmatically using the builder pattern

// Define the WebViewStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: WebViewStyle,
    doc: "Style flags for `WebView`.",
    variants: {
        Default: 0, "Default style."
    },
    default_variant: Default
);

// Enable widget casting for WebView
impl crate::window::FromWindowWithClassName for WebView {
    fn class_name() -> &'static str {
        "wxWebView"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        WebView {
            handle: WindowHandle::new(ptr),
        }
    }
}
