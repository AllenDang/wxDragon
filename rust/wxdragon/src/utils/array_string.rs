use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

/// A wrapper around wxArrayString that provides safe Rust APIs for interacting with wxWidgets string arrays.
///
/// This struct handles memory management for the C++ wxArrayString object and provides
/// methods to add, retrieve, and convert strings to/from the underlying array.
pub struct WxdArrayString {
    ptr: *mut ffi::wxd_ArrayString_t,
    owns_ptr: bool,
}

impl WxdArrayString {
    /// Creates a new empty WxdArrayString.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_ArrayString_Create() };
        assert!(!ptr.is_null(), "Failed to create wxd_ArrayString");
        WxdArrayString {
            ptr,
            owns_ptr: true,
        }
    }

    /// Creates a WxdArrayString from an existing wxd_ArrayString_t pointer.
    ///
    /// # Safety
    /// The pointer must be a valid pointer to a wxd_ArrayString_t. If `take_ownership` is true,
    /// this struct will free the pointer when dropped. If false, the caller is responsible
    /// for freeing the pointer.
    pub unsafe fn from_ptr(ptr: *mut ffi::wxd_ArrayString_t, take_ownership: bool) -> Self {
        WxdArrayString {
            ptr,
            owns_ptr: take_ownership,
        }
    }

    /// Returns the number of strings in the array.
    pub fn get_count(&self) -> usize {
        unsafe { ffi::wxd_ArrayString_GetCount(self.ptr) as usize }
    }

    /// Returns true if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.get_count() == 0
    }

    /// Gets a string at the specified index.
    /// Returns None if the index is out of bounds or if an error occurs.
    pub fn get_string(&self, index: usize) -> Option<String> {
        if index >= self.get_count() {
            return None;
        }

        unsafe {
            // First, try with a reasonable stack buffer
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed = ffi::wxd_ArrayString_GetString(
                self.ptr,
                index as i32,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None; // Error
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                // String fit in the buffer
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                // Need a larger buffer
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_ArrayString_GetString(
                    self.ptr,
                    index as i32,
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );

                if len_copied == len_needed {
                    vec_buffer.pop(); // Remove null terminator
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None // Error on second call
                }
            }
        }
    }

    /// Adds a string to the array.
    /// Returns true if the operation was successful.
    pub fn add(&mut self, s: &str) -> bool {
        let c_str = match CString::new(s) {
            Ok(cs) => cs,
            Err(_) => return false,
        };

        unsafe { ffi::wxd_ArrayString_Add(self.ptr, c_str.as_ptr()) }
    }

    /// Adds multiple strings to the array.
    /// Returns the number of successfully added strings.
    pub fn add_many(&mut self, strings: &[&str]) -> usize {
        let mut count = 0;
        for s in strings {
            if self.add(s) {
                count += 1;
            }
        }
        count
    }

    /// Clears all strings from the array.
    pub fn clear(&mut self) {
        unsafe {
            ffi::wxd_ArrayString_Clear(self.ptr);
        }
    }

    /// Converts this WxdArrayString into a `Vec<String>`.
    /// This consumes the WxdArrayString if it owns the pointer.
    pub fn into_vec(self) -> Vec<String> {
        let count = self.get_count();
        let mut vec = Vec::with_capacity(count);

        for i in 0..count {
            if let Some(s) = self.get_string(i) {
                vec.push(s);
            } else {
                // Handle error getting string by pushing an empty string
                // to maintain index correspondence
                vec.push(String::new());
            }
        }

        // Only leak the pointer if we're not taking ownership
        let _ = std::mem::ManuallyDrop::new(self);

        vec
    }

    /// Gets all strings from the array as a `Vec<String>` without consuming the WxdArrayString.
    pub fn get_strings(&self) -> Vec<String> {
        let count = self.get_count();
        let mut vec = Vec::with_capacity(count);

        for i in 0..count {
            if let Some(s) = self.get_string(i) {
                vec.push(s);
            } else {
                vec.push(String::new());
            }
        }

        vec
    }

    /// Gets the raw pointer to the wxd_ArrayString_t.
    /// This is useful when passing the array to wxWidgets functions.
    ///
    /// # Safety
    /// The caller must ensure that the pointer is not used after this WxdArrayString is dropped
    /// if `owns_ptr` is true.
    pub fn as_ptr(&self) -> *mut ffi::wxd_ArrayString_t {
        self.ptr
    }

    /// Detaches the pointer from this WxdArrayString, causing it to no longer own the pointer.
    /// This is useful when you need to pass ownership of the array to a wxWidgets function.
    ///
    /// # Safety
    /// The caller is responsible for freeing the pointer after calling this function.
    pub unsafe fn detach(&mut self) -> *mut ffi::wxd_ArrayString_t {
        self.owns_ptr = false;
        self.ptr
    }
}

impl Drop for WxdArrayString {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.owns_ptr {
            unsafe {
                ffi::wxd_ArrayString_Free(self.ptr);
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}

impl Default for WxdArrayString {
    fn default() -> Self {
        Self::new()
    }
}

// Implement From<Vec<String>> and From<&[String]> for convenient creation
impl From<Vec<String>> for WxdArrayString {
    fn from(strings: Vec<String>) -> Self {
        let mut array = Self::new();
        for s in strings {
            array.add(&s);
        }
        array
    }
}

impl From<&[String]> for WxdArrayString {
    fn from(strings: &[String]) -> Self {
        let mut array = Self::new();
        for s in strings {
            array.add(s);
        }
        array
    }
}

impl From<&[&str]> for WxdArrayString {
    fn from(strings: &[&str]) -> Self {
        let mut array = Self::new();
        array.add_many(strings);
        array
    }
}
