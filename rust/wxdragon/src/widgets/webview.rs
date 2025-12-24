//! Safe wrapper for wxWebView.

use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{Window, WxWidget};
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
#[derive(Clone, Copy)]
pub struct WebView {
    window: Window,
}

impl WebView {
    /// Creates a new WebView builder.
    pub fn builder(parent: &dyn WxWidget) -> WebViewBuilder<'_> {
        WebViewBuilder::new(parent)
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

        WebView {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
        }
    }

    // --- Navigation ---

    pub fn load_url(&self, url: &str) {
        let c_url = CString::new(url).unwrap_or_default();
        unsafe { ffi::wxd_WebView_LoadURL(self.as_ptr(), c_url.as_ptr()) };
    }

    pub fn reload(&self, flags: WebViewReloadFlags) {
        unsafe { ffi::wxd_WebView_Reload(self.as_ptr(), flags.into()) };
    }

    pub fn stop(&self) {
        unsafe { ffi::wxd_WebView_Stop(self.as_ptr()) };
    }

    pub fn can_go_back(&self) -> bool {
        unsafe { ffi::wxd_WebView_CanGoBack(self.as_ptr()) }
    }

    pub fn can_go_forward(&self) -> bool {
        unsafe { ffi::wxd_WebView_CanGoForward(self.as_ptr()) }
    }

    pub fn go_back(&self) {
        unsafe { ffi::wxd_WebView_GoBack(self.as_ptr()) };
    }

    pub fn go_forward(&self) {
        unsafe { ffi::wxd_WebView_GoForward(self.as_ptr()) };
    }

    pub fn clear_history(&self) {
        unsafe { ffi::wxd_WebView_ClearHistory(self.as_ptr()) };
    }

    // --- State ---

    pub fn is_busy(&self) -> bool {
        unsafe { ffi::wxd_WebView_IsBusy(self.as_ptr()) }
    }

    pub fn get_current_url(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 2048];
            let len = ffi::wxd_WebView_GetCurrentURL(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    pub fn get_current_title(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_WebView_GetCurrentTitle(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    pub fn get_page_source(&self) -> String {
        // Page source can be large, use dynamic buffer resizing
        unsafe {
            // First call with moderate buffer to get the size
            let mut buffer: Vec<c_char> = vec![0; 1024 * 64]; // 64KB initial buffer
            let len = ffi::wxd_WebView_GetPageSource(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);

            if len < 0 {
                return String::new(); // Error
            }

            // Check if we need a larger buffer
            if len >= buffer.len() as i32 {
                // Allocate larger buffer and retry
                buffer = vec![0; len as usize + 1];
                let len2 = ffi::wxd_WebView_GetPageSource(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);
                if len2 < 0 {
                    return String::new(); // Error on second call
                }
            }

            let actual_len = std::cmp::min(len as usize, buffer.len() - 1);
            let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, actual_len);
            String::from_utf8_lossy(byte_slice).to_string()
        }
    }

    pub fn get_page_text(&self) -> String {
        // Page text can be large, use dynamic buffer resizing
        unsafe {
            // First call with moderate buffer to get the size
            let mut buffer: Vec<c_char> = vec![0; 1024 * 64]; // 64KB initial buffer
            let len = ffi::wxd_WebView_GetPageText(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);

            if len < 0 {
                return String::new(); // Error
            }

            // Check if we need a larger buffer
            if len >= buffer.len() as i32 {
                // Allocate larger buffer and retry
                buffer = vec![0; len as usize + 1];
                let len2 = ffi::wxd_WebView_GetPageText(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);
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

    pub fn can_set_zoom_type(&self, zoom_type: WebViewZoomType) -> bool {
        unsafe { ffi::wxd_WebView_CanSetZoomType(self.as_ptr(), zoom_type.into()) }
    }

    pub fn get_zoom(&self) -> WebViewZoom {
        let val = unsafe { ffi::wxd_WebView_GetZoom(self.as_ptr()) };
        match val {
            0 => WebViewZoom::Tiny,
            1 => WebViewZoom::Small,
            2 => WebViewZoom::Medium,
            3 => WebViewZoom::Large,
            4 => WebViewZoom::Largest,
            _ => WebViewZoom::Medium,
        }
    }

    pub fn get_zoom_type(&self) -> WebViewZoomType {
        let val = unsafe { ffi::wxd_WebView_GetZoomType(self.as_ptr()) };
        match val {
            0 => WebViewZoomType::Layout,
            1 => WebViewZoomType::Text,
            _ => WebViewZoomType::Layout,
        }
    }

    pub fn set_zoom(&self, zoom: WebViewZoom) {
        unsafe { ffi::wxd_WebView_SetZoom(self.as_ptr(), zoom.into()) };
    }

    pub fn set_zoom_type(&self, zoom_type: WebViewZoomType) {
        unsafe { ffi::wxd_WebView_SetZoomType(self.as_ptr(), zoom_type.into()) };
    }

    // --- Scripting ---

    pub fn run_script(&self, javascript: &str) -> Option<String> {
        let c_script = CString::new(javascript).unwrap_or_default();
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 4096];
            let len = ffi::wxd_WebView_RunScript(self.as_ptr(), c_script.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);

            if len < 0 {
                return None; // Error
            }

            // Check if we need a larger buffer
            if len >= buffer.len() as i32 {
                // Allocate larger buffer and retry
                buffer = vec![0; len as usize + 1];
                let len2 = ffi::wxd_WebView_RunScript(self.as_ptr(), c_script.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);
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

    pub fn can_cut(&self) -> bool {
        unsafe { ffi::wxd_WebView_CanCut(self.as_ptr()) }
    }

    pub fn can_copy(&self) -> bool {
        unsafe { ffi::wxd_WebView_CanCopy(self.as_ptr()) }
    }

    pub fn can_paste(&self) -> bool {
        unsafe { ffi::wxd_WebView_CanPaste(self.as_ptr()) }
    }

    pub fn cut(&self) {
        unsafe { ffi::wxd_WebView_Cut(self.as_ptr()) };
    }

    pub fn copy(&self) {
        unsafe { ffi::wxd_WebView_Copy(self.as_ptr()) };
    }

    pub fn paste(&self) {
        unsafe { ffi::wxd_WebView_Paste(self.as_ptr()) };
    }

    pub fn can_undo(&self) -> bool {
        unsafe { ffi::wxd_WebView_CanUndo(self.as_ptr()) }
    }

    pub fn can_redo(&self) -> bool {
        unsafe { ffi::wxd_WebView_CanRedo(self.as_ptr()) }
    }

    pub fn undo(&self) {
        unsafe { ffi::wxd_WebView_Undo(self.as_ptr()) };
    }

    pub fn redo(&self) {
        unsafe { ffi::wxd_WebView_Redo(self.as_ptr()) };
    }

    // --- Selection ---

    pub fn select_all(&self) {
        unsafe { ffi::wxd_WebView_SelectAll(self.as_ptr()) };
    }

    pub fn has_selection(&self) -> bool {
        unsafe { ffi::wxd_WebView_HasSelection(self.as_ptr()) }
    }

    pub fn delete_selection(&self) {
        unsafe { ffi::wxd_WebView_DeleteSelection(self.as_ptr()) };
    }

    pub fn get_selected_text(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 4096];
            let len = ffi::wxd_WebView_GetSelectedText(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    pub fn get_selected_source(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 4096];
            let len = ffi::wxd_WebView_GetSelectedSource(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);
            if len >= 0 {
                let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
                String::from_utf8_lossy(byte_slice).to_string()
            } else {
                String::new()
            }
        }
    }

    pub fn clear_selection(&self) {
        unsafe { ffi::wxd_WebView_ClearSelection(self.as_ptr()) };
    }

    // --- Editing ---

    pub fn is_editable(&self) -> bool {
        unsafe { ffi::wxd_WebView_IsEditable(self.as_ptr()) }
    }

    pub fn set_editable(&self, enable: bool) {
        unsafe { ffi::wxd_WebView_SetEditable(self.as_ptr(), enable) };
    }

    // --- Printing ---

    pub fn print(&self) {
        unsafe { ffi::wxd_WebView_Print(self.as_ptr()) };
    }

    // --- Context Menu & Dev Tools ---

    pub fn enable_context_menu(&self, enable: bool) {
        unsafe { ffi::wxd_WebView_EnableContextMenu(self.as_ptr(), enable) };
    }

    pub fn is_context_menu_enabled(&self) -> bool {
        unsafe { ffi::wxd_WebView_IsContextMenuEnabled(self.as_ptr()) }
    }

    pub fn enable_access_to_dev_tools(&self, enable: bool) {
        unsafe { ffi::wxd_WebView_EnableAccessToDevTools(self.as_ptr(), enable) };
    }

    pub fn is_access_to_dev_tools_enabled(&self) -> bool {
        unsafe { ffi::wxd_WebView_IsAccessToDevToolsEnabled(self.as_ptr()) }
    }

    pub fn show_dev_tools(&self) -> bool {
        unsafe { ffi::wxd_WebView_ShowDevTools(self.as_ptr()) }
    }

    pub fn enable_browser_accelerator_keys(&self, enable: bool) {
        unsafe { ffi::wxd_WebView_EnableBrowserAcceleratorKeys(self.as_ptr(), enable) };
    }

    pub fn are_browser_accelerator_keys_enabled(&self) -> bool {
        unsafe { ffi::wxd_WebView_AreBrowserAcceleratorKeysEnabled(self.as_ptr()) }
    }

    // --- Zoom Factor ---

    pub fn get_zoom_factor(&self) -> f32 {
        unsafe { ffi::wxd_WebView_GetZoomFactor(self.as_ptr()) }
    }

    pub fn set_zoom_factor(&self, zoom: f32) {
        unsafe { ffi::wxd_WebView_SetZoomFactor(self.as_ptr(), zoom) };
    }

    // --- Page Loading ---

    pub fn set_page(&self, html: &str, base_url: &str) {
        let c_html = CString::new(html).unwrap_or_default();
        let c_base_url = CString::new(base_url).unwrap_or_default();
        unsafe {
            ffi::wxd_WebView_SetPage(self.as_ptr(), c_html.as_ptr(), c_base_url.as_ptr());
        }
    }

    pub fn find(&self, text: &str, flags: WebViewFindFlags) -> i64 {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe { ffi::wxd_WebView_Find(self.as_ptr(), c_text.as_ptr(), flags.bits()) as i64 }
    }

    // --- History ---

    pub fn enable_history(&self, enable: bool) {
        unsafe { ffi::wxd_WebView_EnableHistory(self.as_ptr(), enable) };
    }

    // --- Configuration ---

    pub fn set_user_agent(&self, user_agent: &str) -> bool {
        let c_user_agent = CString::new(user_agent).unwrap_or_default();
        unsafe { ffi::wxd_WebView_SetUserAgent(self.as_ptr(), c_user_agent.as_ptr()) }
    }

    pub fn get_user_agent(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 1024];
            let len = ffi::wxd_WebView_GetUserAgent(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);

            if len < 0 {
                return String::new(); // Error
            }

            // Check if we need a larger buffer
            if len >= buffer.len() as i32 {
                // Allocate larger buffer and retry
                buffer = vec![0; len as usize + 1];
                let len2 = ffi::wxd_WebView_GetUserAgent(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);
                if len2 < 0 {
                    return String::new(); // Error on second call
                }
            }

            let actual_len = std::cmp::min(len as usize, buffer.len() - 1);
            let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, actual_len);
            String::from_utf8_lossy(byte_slice).to_string()
        }
    }

    pub fn set_proxy(&self, proxy: &str) -> bool {
        let c_proxy = CString::new(proxy).unwrap_or_default();
        unsafe { ffi::wxd_WebView_SetProxy(self.as_ptr(), c_proxy.as_ptr()) }
    }

    // --- Advanced Scripting ---

    pub fn add_script_message_handler(&self, name: &str) -> bool {
        let c_name = CString::new(name).unwrap_or_default();
        unsafe { ffi::wxd_WebView_AddScriptMessageHandler(self.as_ptr(), c_name.as_ptr()) }
    }

    pub fn remove_script_message_handler(&self, name: &str) -> bool {
        let c_name = CString::new(name).unwrap_or_default();
        unsafe { ffi::wxd_WebView_RemoveScriptMessageHandler(self.as_ptr(), c_name.as_ptr()) }
    }

    pub fn add_user_script(&self, javascript: &str, injection_time: WebViewUserScriptInjectionTime) -> bool {
        let c_javascript = CString::new(javascript).unwrap_or_default();
        unsafe { ffi::wxd_WebView_AddUserScript(self.as_ptr(), c_javascript.as_ptr(), injection_time as i32) }
    }

    pub fn remove_all_user_scripts(&self) {
        unsafe { ffi::wxd_WebView_RemoveAllUserScripts(self.as_ptr()) };
    }

    // --- Native Backend ---

    pub fn get_native_backend(&self) -> *mut std::os::raw::c_void {
        unsafe { ffi::wxd_WebView_GetNativeBackend(self.as_ptr()) }
    }

    pub fn get_backend(&self) -> String {
        unsafe {
            let mut buffer: Vec<c_char> = vec![0; 256];
            let len = ffi::wxd_WebView_GetBackend(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);

            if len < 0 {
                return String::new();
            }

            // Check if we need a larger buffer
            if len >= buffer.len() as i32 {
                buffer = vec![0; len as usize + 1];
                ffi::wxd_WebView_GetBackend(self.as_ptr(), buffer.as_mut_ptr(), buffer.len() as i32);
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

    fn as_ptr(&self) -> *mut ffi::wxd_WebView_t {
        self.window.as_ptr() as *mut ffi::wxd_WebView_t
    }
}

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

implement_widget_traits_with_target!(WebView, window, Window);

// Note: WebView doesn't have XRC support in wxWidgets, so we don't provide it either
// Users should create WebView programmatically using the builder pattern

widget_style_enum!(
    name: WebViewStyle,
    doc: "Style flags for `WebView`.",
    variants: {
        Default: 0, "Default style."
    },
    default_variant: Default
);

impl_widget_cast!(WebView, "wxWebView", { window });

// Import WebView event types
#[cfg(feature = "webview")]
use crate::event::WebViewEvents;

// Implement WebViewEvents trait for WebView
#[cfg(feature = "webview")]
impl WebViewEvents for WebView {}
