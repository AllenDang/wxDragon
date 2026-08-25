use std::ffi::CStr;
use std::os::raw::c_char;

/// Reads a string from an FFI getter that copies UTF-8 into a caller-provided
/// buffer and returns the length written (or needed, when called with a null
/// buffer), or a negative value on failure.
///
/// The getter is called twice: once with a null buffer to learn the length,
/// then with a buffer of that size.
pub(crate) fn read_ffi_string(reader: impl Fn(*mut c_char, usize) -> i32) -> Option<String> {
    let needed = reader(std::ptr::null_mut(), 0);
    if needed < 0 {
        return None;
    }
    let mut buffer = vec![0; needed as usize + 1];
    if reader(buffer.as_mut_ptr(), buffer.len()) < 0 {
        return None;
    }
    Some(unsafe { CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy().into_owned())
}
