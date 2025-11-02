use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::rc::Rc;
use wxdragon_sys as ffi;

/// A wrapper around wxArrayString that provides safe Rust APIs for interacting with wxWidgets string arrays.
///
/// This struct handles memory management for the C++ wxArrayString object and provides
/// methods to add, retrieve, and convert strings to/from the underlying array.
pub struct ArrayString {
    ptr: *mut ffi::wxd_ArrayString_t,
    owns_ptr: bool,
    // Prevent Send/Sync: wxWidgets objects are not thread-safe and must stay on UI thread.
    _nosend_nosync: PhantomData<Rc<()>>,
}

impl Clone for ArrayString {
    fn clone(&self) -> Self {
        assert!(!self.ptr.is_null(), "Cannot clone ArrayString with null pointer");
        let new_ptr = unsafe { ffi::wxd_ArrayString_Clone(self.ptr) };
        assert!(!new_ptr.is_null(), "Failed to clone wxArrayString");
        ArrayString {
            ptr: new_ptr,
            owns_ptr: true,
            _nosend_nosync: PhantomData,
        }
    }
}

impl ArrayString {
    /// Creates a new empty ArrayString.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_ArrayString_Create() };
        assert!(!ptr.is_null(), "Failed to create wxArrayString");
        ArrayString {
            ptr,
            owns_ptr: true,
            _nosend_nosync: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.get_count()
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
    pub fn add<T: AsRef<str>>(&mut self, s: T) -> bool {
        let c_str = match CString::new(s.as_ref()) {
            Ok(cs) => cs,
            Err(_) => return false,
        };

        unsafe { ffi::wxd_ArrayString_Add(self.ptr, c_str.as_ptr()) }
    }

    /// Adds multiple strings to the array.
    /// Returns the number of successfully added strings.
    pub fn add_many<T: AsRef<str>>(&mut self, strings: &[T]) -> usize {
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
        unsafe { ffi::wxd_ArrayString_Clear(self.ptr) };
    }

    /// Gets all strings from the array as a `Vec<String>` without consuming the ArrayString.
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

    /// Returns a const raw pointer to the underlying wxd_ArrayString_t.
    ///
    /// Ownership notes:
    /// - This does not transfer ownership; the pointer is only valid while this wrapper (or another owner) keeps it alive.
    /// - Do not free this pointer yourself. The owner is responsible for freeing it (see `into_raw_mut`).
    pub fn as_const_ptr(&self) -> *const ffi::wxd_ArrayString_t {
        self.ptr as *const _
    }

    /// Returns a mutable raw pointer to the underlying wxd_ArrayString_t.
    ///
    /// Use with extreme care; prefer safe methods when possible.
    ///
    /// Ownership notes:
    /// - This does not transfer ownership. If you mutate through this pointer, you must ensure exclusive access.
    /// - Do not free through this pointer; use `into_raw_mut` if you need to assume ownership and manage lifetime manually.
    pub fn as_mut_ptr(&mut self) -> *mut ffi::wxd_ArrayString_t {
        self.ptr
    }

    /// Consumes self and returns a raw mutable pointer, transferring ownership to the caller.
    ///
    /// After calling this, you must NOT use the original `ArrayString` again.
    ///
    /// Caller responsibilities:
    /// - You now own the pointer and must free it exactly once with `wxd_ArrayString_Free`.
    pub fn into_raw_mut(mut self) -> *mut ffi::wxd_ArrayString_t {
        assert!(
            self.owns_ptr,
            "into_raw_mut can only be called on owning ArrayString instances"
        );
        let ptr = self.ptr;
        self.ptr = std::ptr::null_mut();
        ptr
    }

    /// Consumes a borrowed (non-owning) wrapper and returns a raw const pointer without taking ownership.
    ///
    /// Panics if called on an owning wrapper to avoid leaking the owned resource.
    ///
    /// Caller responsibilities:
    /// - This does NOT transfer ownership; do not free the returned pointer.
    /// - The pointer remains valid only while the original owner keeps it alive.
    pub fn into_raw_const(self) -> *const ffi::wxd_ArrayString_t {
        assert!(
            !self.owns_ptr,
            "into_raw_const must only be used on non-owning (borrowed) wrappers"
        );
        self.ptr as *const _
    }
}

impl Drop for ArrayString {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.owns_ptr {
            unsafe { ffi::wxd_ArrayString_Free(self.ptr) };
            self.ptr = std::ptr::null_mut();
        }
    }
}

impl Default for ArrayString {
    fn default() -> Self {
        Self::new()
    }
}

// Consolidated conversions: support any collection of items that can be viewed as str
impl<T: AsRef<str>> From<Vec<T>> for ArrayString {
    fn from(strings: Vec<T>) -> Self {
        strings.into_iter().collect()
    }
}

impl<T: AsRef<str>> From<&[T]> for ArrayString {
    fn from(strings: &[T]) -> Self {
        strings.iter().map(|s| s.as_ref()).collect()
    }
}

impl<S: AsRef<str>> std::iter::FromIterator<S> for ArrayString {
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        let mut array = ArrayString::new();
        for s in iter {
            array.add(s.as_ref());
        }
        array
    }
}

impl From<ArrayString> for Vec<String> {
    fn from(array: ArrayString) -> Self {
        array.get_strings()
    }
}

/// Creates an `ArrayString` from a const pointer to `wxd_ArrayString_t`.
///
/// # Safety
///
/// The caller must ensure that:
/// - `ptr` is a valid, properly aligned pointer to a `wxd_ArrayString_t` object
/// - The pointed-to `wxd_ArrayString_t` object remains valid for the lifetime of the returned `ArrayString`
/// - The pointer is not null (panics will occur on operations if null)
///
/// # Ownership Semantics
///
/// This implementation creates a **borrowed** reference (non-owning). The returned `ArrayString`
/// will NOT free the underlying wxWidgets object when dropped. The caller retains ownership and
/// must ensure the object is properly freed.
impl From<*const ffi::wxd_ArrayString_t> for ArrayString {
    fn from(ptr: *const ffi::wxd_ArrayString_t) -> Self {
        assert!(!ptr.is_null(), "invalid null pointer passed to ArrayString::from");
        ArrayString {
            ptr: ptr as *mut _,
            owns_ptr: false,
            _nosend_nosync: PhantomData,
        }
    }
}

/// Creates an `ArrayString` from a mutable pointer to `wxd_ArrayString_t`.
///
/// # Safety
///
/// The caller must ensure that:
/// - `ptr` is a valid, properly aligned pointer to a `wxd_ArrayString_t` object
/// - The caller transfers full ownership of the object to the returned `ArrayString`
/// - No other code will free or access the object after this call
/// - The pointer is not null (panics will occur on operations if null)
///
/// # Ownership Semantics
///
/// This implementation creates an **owning** reference. The returned `ArrayString` takes
/// ownership of the underlying wxWidgets object and WILL free it when dropped via
/// `wxd_ArrayString_Free`. The caller must not free the object manually after this call.
impl From<*mut ffi::wxd_ArrayString_t> for ArrayString {
    fn from(ptr: *mut ffi::wxd_ArrayString_t) -> Self {
        assert!(!ptr.is_null(), "invalid null pointer passed to ArrayString::from");
        ArrayString {
            ptr,
            owns_ptr: true,
            _nosend_nosync: PhantomData,
        }
    }
}
