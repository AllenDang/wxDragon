// Application lifecycle wrapper
// Currently, the main application logic is driven by the C wxd_Main function.
// This module might later contain wrappers for App-specific functions if needed.

use std::collections::VecDeque;
#[cfg(target_os = "macos")]
use std::ffi::c_int;
use std::ffi::{CStr, CString, c_char, c_void};
use std::sync::{Arc, LazyLock, Mutex};
use wxdragon_sys as ffi; // Import Window and WxWidget trait

// Type alias to reduce complexity
type CallbackQueue = Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send + 'static>>>>;

// Queue for storing callbacks to be executed on the main thread
static MAIN_THREAD_QUEUE: LazyLock<CallbackQueue> = LazyLock::new(|| Arc::new(Mutex::new(VecDeque::new())));

/// Schedules a callback to be executed on the main thread.
///
/// This is useful when you need to update UI elements from a background thread.
/// The callback will be executed during the next event loop iteration.
///
/// # Example
/// ```rust,no_run
/// use wxdragon::prelude::*;
/// # // Minimal stub so the snippet compiles in doctests
/// # struct DummyLabel;
/// # impl DummyLabel { fn set_label(&self, _s: &str) {} }
/// # let my_label = DummyLabel;
///
/// // In a background thread:
/// wxdragon::call_after(Box::new(move || {
///     // Update UI elements here
///     my_label.set_label("Updated from background thread");
/// }));
/// ```
pub fn call_after<F>(callback: Box<F>)
where
    F: FnOnce() + Send + 'static,
{
    let mut queue = MAIN_THREAD_QUEUE.lock().unwrap();
    queue.push_back(callback);
}

/// Processes pending callbacks queued via `call_after`.
///
/// This function is called automatically by the event loop.
/// You do not need to call this function manually.
///
/// Returns true if any callbacks were processed, false if the queue was empty.
pub fn process_main_thread_queue() -> bool {
    let mut callbacks = Vec::new();

    // Move callbacks from the queue to our local vector to minimize lock time
    {
        let mut queue = MAIN_THREAD_QUEUE.lock().unwrap();
        if queue.is_empty() {
            return false;
        }

        // Move up to 10 callbacks at a time to prevent UI freezes
        // if there are many callbacks pending
        for _ in 0..10 {
            if let Some(callback) = queue.pop_front() {
                callbacks.push(callback);
            } else {
                break;
            }
        }
    }

    // Execute callbacks outside of the lock
    for callback in callbacks {
        callback();
    }

    true // We processed some callbacks
}

// This function is called from C++ to process pending callbacks
// Returns 1 if callbacks were processed, 0 if not
#[unsafe(no_mangle)]
pub extern "C" fn process_rust_callbacks() -> i32 {
    if process_main_thread_queue() {
        1 // Callbacks were processed
    } else {
        0 // No callbacks processed
    }
}

// Function to manually trigger callback processing (useful for tests)
pub fn process_callbacks() {
    unsafe {
        ffi::wxd_App_ProcessCallbacks();
    }
}

/// Application handle for setting up app-level event handlers
///
/// This handle is passed to the closure in `wxdragon::main()` and provides
/// methods for registering application-level event handlers, including
/// macOS-specific events.
///
/// # Example
/// ```rust,no_run
/// use wxdragon::prelude::*;
///
/// wxdragon::main(|app| {
///     // Use app to register event handlers
///     app.on_open_files(|files| {
///         println!("Files opened: {:?}", files);
///     });
///
///     let frame = Frame::builder()
///         .with_title("My App")
///         .build();
///     frame.show(true);
/// })
/// .unwrap();
/// ```
#[derive(Clone, Copy)]
pub struct App {
    handle: *mut ffi::wxd_App_t,
}

impl App {
    pub(crate) fn new() -> Option<Self> {
        let handle = unsafe { ffi::wxd_GetApp() };
        if handle.is_null() { None } else { Some(App { handle }) }
    }

    /// Sets the application's top-level window.
    pub fn set_top_window<W>(&self, window: &W)
    where
        W: crate::window::WxWidget + ?Sized,
    {
        if !self.handle.is_null() {
            unsafe {
                ffi::wxd_App_SetTopWindow(self.handle, window.handle_ptr());
            }
        }
    }

    /// Returns the current top-level window if one is set.
    pub fn get_top_window(&self) -> Option<crate::window::Window> {
        if self.handle.is_null() {
            return None;
        }
        let top = unsafe { ffi::wxd_App_GetTopWindow(self.handle) };
        if top.is_null() {
            None
        } else {
            Some(unsafe { crate::window::Window::from_ptr(top) })
        }
    }

    /// Returns true while the main loop is currently running.
    pub fn is_main_loop_running(&self) -> bool {
        if self.handle.is_null() {
            return false;
        }
        unsafe { ffi::wxd_App_IsMainLoopRunning(self.handle) }
    }

    /// Exits the main event loop at the next safe point.
    pub fn exit_main_loop(&self) {
        if !self.handle.is_null() {
            unsafe { ffi::wxd_App_ExitMainLoop(self.handle) };
        }
    }

    /// Controls whether deleting the last frame exits the app.
    pub fn set_exit_on_frame_delete(&self, exit_on_frame_delete: bool) {
        if !self.handle.is_null() {
            unsafe { ffi::wxd_App_SetExitOnFrameDelete(self.handle, exit_on_frame_delete) };
        }
    }

    /// Returns whether deleting the last frame exits the app.
    pub fn get_exit_on_frame_delete(&self) -> bool {
        if self.handle.is_null() {
            return true;
        }
        unsafe { ffi::wxd_App_GetExitOnFrameDelete(self.handle) }
    }

    /// Sets the internal application name used by wxWidgets.
    pub fn set_app_name(&self, name: &str) -> bool {
        let c_name = match CString::new(name) {
            Ok(v) => v,
            Err(_) => return false,
        };
        if !self.handle.is_null() {
            unsafe { ffi::wxd_App_SetAppName(self.handle, c_name.as_ptr()) };
        }
        true
    }

    /// Gets the internal application name used by wxWidgets.
    pub fn get_app_name(&self) -> String {
        if self.handle.is_null() {
            return String::new();
        }
        get_app_string(self.handle, ffi::wxd_App_GetAppName).unwrap_or_default()
    }

    /// Sets the user-facing application display name.
    pub fn set_app_display_name(&self, name: &str) -> bool {
        let c_name = match CString::new(name) {
            Ok(v) => v,
            Err(_) => return false,
        };
        if !self.handle.is_null() {
            unsafe { ffi::wxd_App_SetAppDisplayName(self.handle, c_name.as_ptr()) };
        }
        true
    }

    /// Gets the user-facing application display name.
    pub fn get_app_display_name(&self) -> String {
        if self.handle.is_null() {
            return String::new();
        }
        get_app_string(self.handle, ffi::wxd_App_GetAppDisplayName).unwrap_or_default()
    }

    /// Sets the vendor name used for config paths and metadata.
    pub fn set_vendor_name(&self, name: &str) -> bool {
        let c_name = match CString::new(name) {
            Ok(v) => v,
            Err(_) => return false,
        };
        if !self.handle.is_null() {
            unsafe { ffi::wxd_App_SetVendorName(self.handle, c_name.as_ptr()) };
        }
        true
    }

    /// Gets the vendor name used for config paths and metadata.
    pub fn get_vendor_name(&self) -> String {
        if self.handle.is_null() {
            return String::new();
        }
        get_app_string(self.handle, ffi::wxd_App_GetVendorName).unwrap_or_default()
    }

    /// Sets the user-facing vendor display name.
    pub fn set_vendor_display_name(&self, name: &str) -> bool {
        let c_name = match CString::new(name) {
            Ok(v) => v,
            Err(_) => return false,
        };
        if !self.handle.is_null() {
            unsafe { ffi::wxd_App_SetVendorDisplayName(self.handle, c_name.as_ptr()) };
        }
        true
    }

    /// Gets the user-facing vendor display name.
    pub fn get_vendor_display_name(&self) -> String {
        if self.handle.is_null() {
            return String::new();
        }
        get_app_string(self.handle, ffi::wxd_App_GetVendorDisplayName).unwrap_or_default()
    }
}

fn get_app_string(
    app: *mut ffi::wxd_App_t,
    getter: unsafe extern "C" fn(*const ffi::wxd_App_t, *mut c_char, usize) -> i32,
) -> Option<String> {
    let len = unsafe { getter(app as *const ffi::wxd_App_t, std::ptr::null_mut(), 0) };
    if len < 0 {
        return None;
    }

    let mut buf = vec![0u8; len as usize + 1];
    unsafe {
        getter(app as *const ffi::wxd_App_t, buf.as_mut_ptr() as *mut c_char, buf.len());
    }
    Some(unsafe { CStr::from_ptr(buf.as_ptr() as *const c_char) }.to_string_lossy().to_string())
}

unsafe impl Send for App {}
unsafe impl Sync for App {}

/// Sets the application's top window.
///
/// This is necessary for the main event loop to run correctly.
/// Call this after creating your main Frame.
pub fn set_top_window<W>(window: &W)
where
    W: crate::window::WxWidget + ?Sized,
{
    if let Some(app) = App::new() {
        app.set_top_window(window);
    }
}

/// Gets the current wxWidgets app instance.
pub fn get_app_instance() -> Option<App> {
    App::new()
}

/// Gets the current application instance for appearance operations.
///
/// This provides a convenient way to access appearance-related functions
/// without having to import the appearance module.
///
/// # Returns
/// `Some(App)` if an application instance exists, `None` otherwise.
///
/// # Example
/// ```no_run
/// use wxdragon::prelude::*;
///
/// wxdragon::main(|_| {
///     // Enable dark mode support
///     if let Some(app) = wxdragon::app::get_app() {
///         app.set_appearance(Appearance::System);
///     }
///
///     let frame = Frame::builder()
///         .with_title("Dark Mode App")
///         .build();
///     frame.show(true);
/// })
/// .unwrap();
/// ```
pub fn get_app() -> Option<crate::appearance::App> {
    crate::appearance::get_app()
}

/// Sets the application appearance mode.
///
/// This is a convenience function that gets the app and sets its appearance.
/// On Windows, calling this with `Appearance::System` enables dark mode
/// support when the system is using a dark theme.
///
/// # Arguments
/// * `appearance` - The appearance mode to set
///
/// # Returns
/// * `AppearanceResult::Ok` - The appearance was set successfully
/// * `AppearanceResult::Failure` - Failed to set appearance (not supported)
/// * `AppearanceResult::CannotChange` - Cannot change at this time (windows exist)
///
/// # Example
/// ```no_run
/// use wxdragon::prelude::*;
///
/// wxdragon::main(|_| {
///     // Enable system appearance following (including dark mode on Windows)
///     match wxdragon::app::set_appearance(Appearance::System) {
///         AppearanceResult::Ok => println!("Dark mode support enabled"),
///         AppearanceResult::Failure => println!("Dark mode not supported"),
///         AppearanceResult::CannotChange => println!("Cannot change appearance now"),
///     }
///
///     let frame = Frame::builder()
///         .with_title("My App")
///         .build();
///     frame.show(true);
/// })
/// .unwrap();
/// ```
pub fn set_appearance(appearance: crate::appearance::Appearance) -> crate::appearance::AppearanceResult {
    use crate::appearance::AppAppearance;

    if let Some(app) = get_app() {
        app.set_appearance(appearance)
    } else {
        crate::appearance::AppearanceResult::Failure
    }
}

// Implement AppEvents trait for App
impl crate::event::AppEvents for App {
    fn on_open_files<F>(&self, callback: F)
    where
        F: Fn(Vec<String>) + Send + 'static,
    {
        #[cfg(target_os = "macos")]
        {
            let callback = Box::new(callback);
            let user_data = Box::into_raw(callback) as *mut c_void;

            unsafe { ffi::wxd_App_AddMacOpenFilesHandler(self.handle, Some(mac_open_files_trampoline::<F>), user_data) };
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = callback; // Suppress unused warning
        }
    }

    fn on_open_url<F>(&self, callback: F)
    where
        F: Fn(String) + Send + 'static,
    {
        #[cfg(target_os = "macos")]
        {
            let callback = Box::new(callback);
            let user_data = Box::into_raw(callback) as *mut c_void;

            unsafe { ffi::wxd_App_AddMacOpenURLHandler(self.handle, Some(mac_open_url_trampoline::<F>), user_data) };
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = callback;
        }
    }

    fn on_new_file<F>(&self, callback: F)
    where
        F: Fn() + Send + 'static,
    {
        #[cfg(target_os = "macos")]
        {
            let callback = Box::new(callback);
            let user_data = Box::into_raw(callback) as *mut c_void;

            unsafe { ffi::wxd_App_AddMacNewFileHandler(self.handle, Some(mac_new_file_trampoline::<F>), user_data) };
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = callback;
        }
    }

    fn on_reopen_app<F>(&self, callback: F)
    where
        F: Fn() + Send + 'static,
    {
        #[cfg(target_os = "macos")]
        {
            let callback = Box::new(callback);
            let user_data = Box::into_raw(callback) as *mut c_void;

            unsafe { ffi::wxd_App_AddMacReopenAppHandler(self.handle, Some(mac_reopen_app_trampoline::<F>), user_data) };
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = callback;
        }
    }

    fn on_print_files<F>(&self, callback: F)
    where
        F: Fn(Vec<String>) + Send + 'static,
    {
        #[cfg(target_os = "macos")]
        {
            let callback = Box::new(callback);
            let user_data = Box::into_raw(callback) as *mut c_void;

            unsafe { ffi::wxd_App_AddMacPrintFilesHandler(self.handle, Some(mac_print_files_trampoline::<F>), user_data) };
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = callback;
        }
    }
}

// Trampoline functions for macOS
#[cfg(target_os = "macos")]
unsafe extern "C" fn mac_open_files_trampoline<F>(user_data: *mut c_void, files: *mut *const c_char, count: c_int)
where
    F: Fn(Vec<String>) + Send + 'static,
{
    if user_data.is_null() || files.is_null() {
        return;
    }

    let callback = unsafe { &*(user_data as *const F) };

    let mut file_list = Vec::new();
    for i in 0..count as isize {
        let file_ptr = unsafe { *files.offset(i) };
        if !file_ptr.is_null()
            && let Ok(file_str) = unsafe { CStr::from_ptr(file_ptr) }.to_str()
        {
            file_list.push(file_str.to_string());
        }
    }

    callback(file_list);
}

#[cfg(target_os = "macos")]
unsafe extern "C" fn mac_open_url_trampoline<F>(user_data: *mut c_void, url: *const c_char)
where
    F: Fn(String) + Send + 'static,
{
    if user_data.is_null() || url.is_null() {
        return;
    }

    let callback = unsafe { &*(user_data as *const F) };
    if let Ok(url_str) = unsafe { CStr::from_ptr(url) }.to_str() {
        callback(url_str.to_string());
    }
}

#[cfg(target_os = "macos")]
unsafe extern "C" fn mac_new_file_trampoline<F>(user_data: *mut c_void)
where
    F: Fn() + Send + 'static,
{
    if user_data.is_null() {
        return;
    }

    let callback = unsafe { &*(user_data as *const F) };
    callback();
}

#[cfg(target_os = "macos")]
unsafe extern "C" fn mac_reopen_app_trampoline<F>(user_data: *mut c_void)
where
    F: Fn() + Send + 'static,
{
    if user_data.is_null() {
        return;
    }

    let callback = unsafe { &*(user_data as *const F) };
    callback();
}

#[cfg(target_os = "macos")]
unsafe extern "C" fn mac_print_files_trampoline<F>(user_data: *mut c_void, files: *mut *const c_char, count: c_int)
where
    F: Fn(Vec<String>) + Send + 'static,
{
    if user_data.is_null() || files.is_null() {
        return;
    }

    let callback = unsafe { &*(user_data as *const F) };

    let mut file_list = Vec::new();
    for i in 0..count as isize {
        let file_ptr = unsafe { *files.offset(i) };
        if !file_ptr.is_null()
            && let Ok(file_str) = unsafe { CStr::from_ptr(file_ptr) }.to_str()
        {
            file_list.push(file_str.to_string());
        }
    }

    callback(file_list);
}

/// Runs the wxWidgets application main loop, providing a safe entry point.
///
/// This function initializes wxWidgets and starts the event loop. It takes a closure
/// `on_init` that will be called once after basic initialization but before the
/// main event loop begins.
///
/// # Panics
/// Panics if initialization fails or if the program name cannot be converted to a CString.
///
/// # Example
/// ```no_run
/// use wxdragon::prelude::*;
///
/// wxdragon::main(|app| {
///     // app is the App handle for registering event handlers
///     app.on_open_files(|files| {
///         println!("Files: {:?}", files);
///     });
///
///     let frame = Frame::builder()
///         .with_title("My App")
///         .build();
///     frame.show(true);
///
///     // No need to preserve the frame - wxWidgets manages it
/// })
/// .unwrap();
/// ```
pub fn main<F>(on_init: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(App) + 'static,
{
    // Prepare arguments for wxd_Main from real command line
    // We collect all args (including program name), convert to CString, build a null-terminated argv.
    let exit_code = unsafe {
        // Prepare payload for the C trampoline. We keep ownership on Rust side and
        // only take() the Option in the trampoline. After wxd_Main returns, we
        // reclaim and drop the Box to avoid leaks even if OnInit wasn't called.
        let payload = Box::new(OnInitPayload {
            cb: Some(Box::new(on_init)),
        });
        let user_data_ptr = Box::into_raw(payload) as *mut c_void;

        // Forward all OS arguments to wxWidgets; our App overrides accept any params safely.
        let args: Vec<CString> = std::env::args_os()
            .map(|os| {
                // Preserve content best-effort: use lossy UTF-8; wxWidgets on Windows supports wide args internally
                let s = os.to_string_lossy();
                CString::new(s.as_bytes()).unwrap_or_else(|_| CString::new("").unwrap())
            })
            .collect();

        let _owned_prog: Option<CString>;
        let mut raw_args: Vec<*mut c_char> = if args.is_empty() {
            let pn = CString::new("wxRustApp").expect("CString for app name");
            let ptr = pn.as_ptr() as *mut c_char;
            // Note: keep `pn` alive until function end by binding it (drops after wxd_Main).
            _owned_prog = Some(pn);
            vec![ptr]
        } else {
            args.iter().map(|c| c.as_ptr() as *mut c_char).collect()
        };
        // Append null terminator as argv[argc] expected by some consumers
        raw_args.push(std::ptr::null_mut());

        let argc: i32 = (raw_args.len() as i32) - 1; // exclude trailing null
        let argv_ptr = raw_args.as_mut_ptr();

        // Call the C entry point, passing the trampoline and the closure data
        let code = ffi::wxd_Main(argc, argv_ptr, Some(on_init_trampoline), user_data_ptr);

        // Reclaim and drop the payload Box to free memory in all cases.
        // If the trampoline ran, cb was taken() and executed (now None).
        // If it didn't, dropping here frees the closure too.
        let _ = Box::from_raw(user_data_ptr as *mut OnInitPayload);

        code
    };

    if exit_code != 0 {
        panic!("Application exited with code: {exit_code}");
    }

    Ok(())
}

struct OnInitPayload {
    cb: Option<Box<dyn FnOnce(App)>>,
}

// Trampoline function to call the Rust closure from C
unsafe extern "C" fn on_init_trampoline(user_data: *mut c_void) -> bool {
    if user_data.is_null() {
        return false;
    }

    // Borrow the payload and take the callback out.
    let payload = unsafe { &mut *(user_data as *mut OnInitPayload) };
    let Some(cb) = payload.cb.take() else {
        return false;
    };

    // Create App instance to pass to the callback
    let app = match App::new() {
        Some(app) => app,
        None => {
            log::error!("Failed to get app instance");
            return false;
        }
    };

    // Call the closure with the App instance, catching potential panics
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cb(app)));

    // Process the result
    match result {
        Ok(_) => true, // Always return success if no panic occurred
        Err(_) => {
            log::error!("Panic caught in Rust AppOnInit callback!");
            false // Indicate failure on panic
        }
    }
}
