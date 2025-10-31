#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

// Include the generated FFI bindings (from bindgen)
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Conditionally include the pre-generated constants based on target OS
// Assumes files are located in `rust/wxdragon-sys/src/generated_constants/`
#[cfg(target_os = "macos")]
include!("generated_constants/wx_osx_constants.rs");

#[cfg(target_os = "windows")]
include!("generated_constants/wx_msw_constants.rs");

#[cfg(target_os = "linux")]
include!("generated_constants/wx_gtk_constants.rs");

// Fallback or error for unsupported OS for constants, if necessary.
// Alternatively, you could have a `wx_common_constants.rs` if some constants are universal
// and only OS-specific parts are in the files above.
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
compile_error!("Target OS not supported by pre-generated constants. Please add a constants file for this OS.");

mod logging4c;

// Type alias for convenience maybe?
// pub type wxWindow_t = self::wxd_Window_t; // Example

// Additional constants or utility functions specific to the sys crate itself
// (if any are ever needed, usually this file is minimal)

// REMOVED redundant/incorrect manual definitions and extern block.
// Bindgen now generates wxd_EVT_* constants directly from wxdragon.h
// into the included bindings.rs file.

// Need to find the actual values for these constants. // REMOVED Comment

// Add necessary imports for the drop function
use std::cell::RefCell;
use std::os::raw::c_void;

// Type placeholder for user data until a proper type is defined in the safe wrapper
type WindowUserData = (); // Replace with actual user data type later if needed

/// Function called by C++ (WxdRustClientData destructor) to drop the Rust `Box<RefCell<T>>`.
/// # Safety
/// The caller (C++) must ensure `user_data_ptr` is a valid pointer obtained
/// from `Box::into_raw(Box::new(RefCell::new(data)))` and that it hasn't been
/// dropped or invalidated since.
#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn drop_rust_refcell_box(user_data_ptr: *mut c_void) {
    if !user_data_ptr.is_null() {
        // Reconstitute the Box and let it drop, freeing the memory
        // and dropping the RefCell<WindowUserData>.
        let _boxed_refcell: Box<RefCell<WindowUserData>> =
            unsafe { Box::from_raw(user_data_ptr as *mut RefCell<WindowUserData>) };
        // Drop happens automatically when `_boxed_refcell` goes out of scope here.
    } else {
        // Optional: Log a warning or handle null pointer case if necessary
        // eprintln!("Warning: drop_rust_refcell_box called with null pointer.");
    }
}

/// Function to properly free a string that was allocated by Rust using CString::into_raw().
/// This must be called instead of C's free() for strings allocated by Rust.
/// # Safety
/// The caller (C++) must ensure `str_ptr` is a valid pointer obtained from
/// `CString::into_raw()` and that it hasn't been freed already.
#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn wxd_Variant_Free_Rust_String(str_ptr: *mut std::os::raw::c_char) {
    if !str_ptr.is_null() {
        // Reconstitute the CString and let it drop, properly freeing the memory
        // that was allocated by Rust's allocator
        unsafe {
            let _cstring = std::ffi::CString::from_raw(str_ptr);
            // Drop happens automatically when `_cstring` goes out of scope here.
        }
    }
}
