// Application lifecycle wrapper
// Currently, the main application logic is driven by the C wxd_Main function.
// This module might later contain wrappers for App-specific functions if needed.

use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::ffi::{c_char, c_void, CString};
use std::sync::{Arc, Mutex};
use wxdragon_sys as ffi; // Import Window and WxWidget trait

// Type alias to reduce complexity
type CallbackQueue = Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send + 'static>>>>;

// Queue for storing callbacks to be executed on the main thread
lazy_static! {
    static ref MAIN_THREAD_QUEUE: CallbackQueue = Arc::new(Mutex::new(VecDeque::new()));
}

/// Schedules a callback to be executed on the main thread.
///
/// This is useful when you need to update UI elements from a background thread.
/// The callback will be executed during the next event loop iteration.
///
/// # Example
/// ```
/// use wxdragon::prelude::*;
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
#[no_mangle]
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

/// Sets the application's top window.
///
/// This is necessary for the main event loop to run correctly.
/// Call this after creating your main Frame.
pub fn set_top_window<W>(window: &W)
where
    W: crate::window::WxWidget + ?Sized,
{
    let app_ptr = unsafe { ffi::wxd_GetApp() };
    if !app_ptr.is_null() {
        unsafe {
            ffi::wxd_App_SetTopWindow(app_ptr, window.handle_ptr());
        }
    }
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
/// });
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
/// });
/// ```
pub fn set_appearance(
    appearance: crate::appearance::Appearance,
) -> crate::appearance::AppearanceResult {
    use crate::appearance::AppAppearance;

    if let Some(app) = get_app() {
        app.set_appearance(appearance)
    } else {
        crate::appearance::AppearanceResult::Failure
    }
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
/// wxdragon::main(|_| {
///     let frame = Frame::builder()
///         .with_title("My App")
///         .build();
///     frame.show(true);
///     
///     // No need to preserve the frame - wxWidgets manages it
/// });
/// ```
pub fn main<F>(on_init: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(()) + 'static,
{
    let on_init_boxed: Box<Box<dyn FnOnce(())>> = Box::new(Box::new(on_init));
    let user_data_ptr = Box::into_raw(on_init_boxed) as *mut c_void;

    // Prepare arguments for wxd_Main (using a default program name)
    let exit_code = unsafe {
        let prog_name = CString::new("wxRustApp").expect("Failed to create CString for app name");
        let mut argv: [*mut c_char; 2] = [prog_name.into_raw(), std::ptr::null_mut()];
        let argc: i32 = 1;

        // Call the C entry point, passing the trampoline and the closure data
        let result = ffi::wxd_Main(
            argc,
            argv.as_mut_ptr(),
            Some(on_init_trampoline),
            user_data_ptr,
        );

        let _ = CString::from_raw(argv[0]);
        result
    };

    if exit_code != 0 {
        panic!("Application exited with code: {exit_code}");
    }

    Ok(())
}

// Trampoline function to call the Rust closure from C
unsafe extern "C" fn on_init_trampoline(user_data: *mut c_void) -> bool {
    if user_data.is_null() {
        return false;
    }

    // Cast back to Box<dyn FnOnce(())>
    let closure_box: Box<Box<dyn FnOnce(())>> = Box::from_raw(user_data as *mut _);

    // Call the closure, catching potential panics
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        (*closure_box)(()) // Call the closure itself
    }));

    // Process the result
    match result {
        Ok(_) => true, // Always return success if no panic occurred
        Err(_) => {
            eprintln!("Panic caught in Rust AppOnInit callback!");
            false // Indicate failure on panic
        }
    }
}
