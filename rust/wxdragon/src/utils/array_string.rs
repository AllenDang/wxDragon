use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

/// A wrapper around wxArrayString that provides safe Rust APIs for interacting with wxWidgets string arrays.
///
/// This struct handles memory management for the C++ wxArrayString object and provides
/// methods to add, retrieve, and convert strings to/from the underlying array.
pub struct WxdArrayString {
    ptr: *const ffi::wxd_ArrayString_t,
    owns_ptr: bool,
}

impl AsRef<*const ffi::wxd_ArrayString_t> for WxdArrayString {
    fn as_ref(&self) -> &*const ffi::wxd_ArrayString_t {
        &self.ptr
    }
}

impl std::ops::Deref for WxdArrayString {
    type Target = *const ffi::wxd_ArrayString_t;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl Clone for WxdArrayString {
    fn clone(&self) -> Self {
        assert!(!self.ptr.is_null(), "Cannot clone WxdArrayString with null pointer");
        let new_ptr = unsafe { ffi::wxd_ArrayString_Clone(self.ptr) };
        WxdArrayString {
            ptr: new_ptr,
            owns_ptr: true,
        }
    }
}

impl WxdArrayString {
    /// Creates a new empty WxdArrayString.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_ArrayString_Create() };
        assert!(!ptr.is_null(), "Failed to create wxd_ArrayString");
        WxdArrayString { ptr, owns_ptr: true }
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

        let index = index as i32;
        // First, try with a reasonable stack buffer
        let len = unsafe { ffi::wxd_ArrayString_GetString(self.ptr, index, std::ptr::null_mut(), 0) };

        if len < 0 {
            return None; // Error
        }

        // Need a larger buffer
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_ArrayString_GetString(self.ptr, index, buf.as_mut_ptr(), buf.len()) };
        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() })
    }

    /// Adds a string to the array.
    /// Returns true if the operation was successful.
    pub fn add(&self, s: &str) -> bool {
        let c_str = match CString::new(s) {
            Ok(cs) => cs,
            Err(_) => return false,
        };

        unsafe { ffi::wxd_ArrayString_Add(self.ptr, c_str.as_ptr()) }
    }

    /// Adds multiple strings to the array.
    /// Returns the number of successfully added strings.
    pub fn add_many(&self, strings: &[&str]) -> usize {
        let mut count = 0;
        for s in strings {
            if self.add(s) {
                count += 1;
            }
        }
        count
    }

    /// Clears all strings from the array.
    pub fn clear(&self) {
        unsafe { ffi::wxd_ArrayString_Clear(self.ptr) };
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
}

impl Drop for WxdArrayString {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.owns_ptr {
            unsafe { ffi::wxd_ArrayString_Free(self.ptr) };
            self.ptr = std::ptr::null_mut();
        }
    }
}

impl Default for WxdArrayString {
    fn default() -> Self {
        Self::new()
    }
}

// Consolidated conversions: support any collection of items that can be viewed as str
impl<T: AsRef<str>> From<Vec<T>> for WxdArrayString {
    fn from(strings: Vec<T>) -> Self {
        strings.into_iter().collect()
    }
}

impl<T: AsRef<str>> From<&[T]> for WxdArrayString {
    fn from(strings: &[T]) -> Self {
        strings.iter().map(|s| s.as_ref()).collect()
    }
}

impl<S: AsRef<str>> std::iter::FromIterator<S> for WxdArrayString {
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        let array = WxdArrayString::new();
        for s in iter {
            array.add(s.as_ref());
        }
        array
    }
}

impl From<*const ffi::wxd_ArrayString_t> for WxdArrayString {
    fn from(ptr: *const ffi::wxd_ArrayString_t) -> Self {
        WxdArrayString { ptr, owns_ptr: false }
    }
}

impl From<*mut ffi::wxd_ArrayString_t> for WxdArrayString {
    fn from(ptr: *mut ffi::wxd_ArrayString_t) -> Self {
        WxdArrayString { ptr, owns_ptr: true }
    }
}

use std::io::Error;

impl TryFrom<WxdArrayString> for *const ffi::wxd_ArrayString_t {
    type Error = std::io::Error;
    fn try_from(array: WxdArrayString) -> Result<Self, Self::Error> {
        if !array.owns_ptr {
            if array.ptr.is_null() {
                Err(Error::other(
                    "Cannot convert WxdArrayString with null pointer to const raw pointer",
                ))
            } else {
                Ok(array.ptr)
            }
        } else {
            Err(Error::other("Cannot convert owned WxdArrayString to const raw pointer"))
        }
    }
}

impl TryFrom<WxdArrayString> for *mut ffi::wxd_ArrayString_t {
    type Error = std::io::Error;
    fn try_from(array: WxdArrayString) -> Result<Self, Self::Error> {
        if array.owns_ptr {
            if array.ptr.is_null() {
                Err(Error::other(
                    "Cannot convert WxdArrayString with null pointer to mutable raw pointer",
                ))
            } else {
                Ok(array.ptr as *mut ffi::wxd_ArrayString_t)
            }
        } else {
            Err(Error::other("Cannot convert non-owned WxdArrayString to mutable raw pointer"))
        }
    }
}
