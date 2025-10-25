//! Variant wrapper for typed wxVariant C API.

use crate::{Bitmap, DateTime};
use wxdragon_sys as ffi;

/// Represents the type of data stored in a variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariantType {
    Bool,
    Int32,
    Int64,
    Double,
    String,
    DateTime,
    Bitmap,
    Progress,
    IconText,
}

impl VariantType {
    pub fn as_str(&self) -> &'static str {
        match self {
            VariantType::Bool => "bool",
            VariantType::Int32 => "long",
            VariantType::Int64 => "longlong",
            VariantType::Double => "double",
            VariantType::String => "string",
            VariantType::DateTime => "datetime",
            VariantType::Bitmap => "bitmap",
            VariantType::Progress => "long",
            VariantType::IconText => "wxDataViewIconText",
        }
    }
}

/// Safe Rust wrapper over a wxVariant pointer (wxd_Variant_t).
///
/// Owns the underlying wxVariant by default and destroys it in Drop.
pub struct Variant {
    ptr: *const ffi::wxd_Variant_t,
    owned: bool,
}

impl AsRef<*const ffi::wxd_Variant_t> for Variant {
    fn as_ref(&self) -> &*const ffi::wxd_Variant_t {
        // unsafe { &*((&self.ptr) as *const *mut ffi::wxd_Variant_t as *const *const ffi::wxd_Variant_t) }
        &self.ptr
    }
}

impl std::ops::Deref for Variant {
    type Target = *const ffi::wxd_Variant_t;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Variant {
    /// Create an empty variant.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_Variant_CreateEmpty() };
        Self { ptr, owned: true }
    }

    /// Create a new variant and set it to a bool value.
    pub fn from_bool(v: bool) -> Self {
        let var = Self::new();
        unsafe { ffi::wxd_Variant_SetBool(var.ptr, v) };
        var
    }

    pub fn from_i32(v: i32) -> Self {
        let var = Self::new();
        unsafe { ffi::wxd_Variant_SetInt32(var.ptr, v) };
        var
    }

    pub fn from_i64(v: i64) -> Self {
        let var = Self::new();
        unsafe { ffi::wxd_Variant_SetInt64(var.ptr, v) };
        var
    }

    pub fn from_f64(v: f64) -> Self {
        let var = Self::new();
        unsafe { ffi::wxd_Variant_SetDouble(var.ptr, v) };
        var
    }

    pub fn from_string<S: AsRef<str>>(s: S) -> Self {
        let var = Self::new();
        let b = s.as_ref().as_bytes();
        unsafe { ffi::wxd_Variant_SetString_Utf8(var.ptr, b.as_ptr() as _, b.len() as i32) };
        var
    }

    pub fn from_datetime(dt: DateTime) -> Self {
        let var = Self::new();
        let raw = unsafe { *dt.as_ptr() };
        unsafe { ffi::wxd_Variant_SetDateTime(var.ptr, raw) };
        var
    }

    pub fn from_bitmap(bmp: &Bitmap) -> Self {
        let var = Self::new();
        let p = **bmp as *const ffi::wxd_Bitmap_t;
        unsafe { ffi::wxd_Variant_SetBitmap(var.ptr, p) };
        var
    }

    /// Returns const pointer for passing to FFI where C++ does not take ownership.
    pub fn as_const_ptr(&self) -> *const ffi::wxd_Variant_t {
        self.ptr as *const _
    }

    /// Transfers ownership to the caller (typically C++). After this call,
    /// Drop will not destroy the variant.
    pub fn into_raw(mut self) -> *const ffi::wxd_Variant_t {
        let p = self.ptr;
        self.owned = false;
        self.ptr = std::ptr::null();
        p
    }

    /// # Safety
    /// Clone from a const wxd_Variant_t* by calling the C++ Clone helper.
    pub unsafe fn from_const_ptr_clone(ptr: *const ffi::wxd_Variant_t) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }
        let cloned = ffi::wxd_Variant_Clone(ptr);
        if cloned.is_null() {
            None
        } else {
            Some(Self {
                ptr: cloned,
                owned: true,
            })
        }
    }

    /// Returns the wxVariant type name (e.g., "string", "bool").
    pub fn type_name(&self) -> String {
        // Query required length first by calling with out_len=0
        let needed = unsafe { ffi::wxd_Variant_GetTypeName_Utf8(**self, std::ptr::null_mut(), 0) };
        if needed == 0 {
            return String::new();
        }
        let mut b = vec![0u8; needed as usize + 1];
        let w = unsafe { ffi::wxd_Variant_GetTypeName_Utf8(**self, b.as_mut_ptr() as _, b.len()) };
        if w == 0 {
            return String::new();
        }
        // Ensure we drop trailing NUL, if any
        if let Some(pos) = b.iter().position(|&b| b == 0) {
            b.truncate(pos);
        }
        String::from_utf8_lossy(&b).into_owned()
    }

    pub fn get_bool(&self) -> Option<bool> {
        let mut out = false;
        let ok = unsafe { ffi::wxd_Variant_GetBool(self.as_const_ptr(), &mut out) };
        if ok {
            Some(out)
        } else {
            None
        }
    }

    pub fn get_i32(&self) -> Option<i32> {
        let mut out = 0i32;
        let ok = unsafe { ffi::wxd_Variant_GetInt32(self.as_const_ptr(), &mut out) };
        if ok {
            Some(out)
        } else {
            None
        }
    }

    pub fn get_i64(&self) -> Option<i64> {
        let mut out = 0i64;
        let ok = unsafe { ffi::wxd_Variant_GetInt64(self.as_const_ptr(), &mut out) };
        if ok {
            Some(out)
        } else {
            None
        }
    }

    pub fn get_f64(&self) -> Option<f64> {
        let mut out = 0f64;
        let ok = unsafe { ffi::wxd_Variant_GetDouble(self.as_const_ptr(), &mut out) };
        if ok {
            Some(out)
        } else {
            None
        }
    }

    pub fn get_string(&self) -> Option<String> {
        let needed = unsafe { ffi::wxd_Variant_GetString_Utf8(**self, std::ptr::null_mut(), 0) };
        if needed == 0 {
            return None;
        }
        let mut buf = vec![0_u8; needed + 1];
        let written =
            unsafe { ffi::wxd_Variant_GetString_Utf8(**self, buf.as_mut_ptr() as _, buf.len()) };
        if written == 0 {
            return Some(String::new());
        }
        if let Some(pos) = buf.iter().position(|&b| b == 0) {
            buf.truncate(pos);
        }
        Some(String::from_utf8_lossy(&buf).into_owned())
    }

    pub fn get_datetime(&self) -> Option<DateTime> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::wxd_DateTime_t>() };
        let ok = unsafe { ffi::wxd_Variant_GetDateTime(self.ptr, &mut raw) };
        if ok {
            Some(DateTime::from_raw(raw))
        } else {
            None
        }
    }

    pub fn get_bitmap(&self) -> Option<Bitmap> {
        let ptr = unsafe { ffi::wxd_Variant_GetBitmapClone(**self) };
        if ptr.is_null() {
            None
        } else {
            Some(Bitmap::from_ptr_owned(ptr))
        }
    }
}

impl Clone for Variant {
    fn clone(&self) -> Self {
        let cloned = unsafe { ffi::wxd_Variant_Clone(self.as_const_ptr()) };
        Self {
            ptr: cloned,
            owned: true,
        }
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            unsafe { ffi::wxd_Variant_Destroy(self.ptr) };
        }
        self.ptr = std::ptr::null_mut();
        self.owned = false;
    }
}

impl Default for Variant {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variant(type={})", self.type_name())
    }
}

impl From<bool> for Variant {
    fn from(value: bool) -> Self {
        Self::from_bool(value)
    }
}

impl From<i32> for Variant {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl From<i64> for Variant {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl From<f64> for Variant {
    fn from(value: f64) -> Self {
        Self::from_f64(value)
    }
}

impl From<&str> for Variant {
    fn from(value: &str) -> Self {
        Self::from_string(value)
    }
}

impl From<String> for Variant {
    fn from(value: String) -> Self {
        Self::from_string(value)
    }
}

impl From<DateTime> for Variant {
    fn from(value: DateTime) -> Self {
        Self::from_datetime(value)
    }
}

impl From<Bitmap> for Variant {
    fn from(value: Bitmap) -> Self {
        Self::from_bitmap(&value)
    }
}

impl<'a> From<&'a Bitmap> for Variant {
    fn from(value: &'a Bitmap) -> Self {
        Self::from_bitmap(value)
    }
}

use std::io::{Error, ErrorKind::InvalidData};

impl TryFrom<Variant> for bool {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        value.get_bool().ok_or(Error::new(
            InvalidData,
            format!("Not a bool, it's a {}", value.type_name()),
        ))
    }
}

impl TryFrom<Variant> for i32 {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        value.get_i32().ok_or(Error::new(
            InvalidData,
            format!("Not an i32, it's a {}", value.type_name()),
        ))
    }
}

impl TryFrom<Variant> for i64 {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        value.get_i64().ok_or(Error::new(
            InvalidData,
            format!("Not an i64, it's a {}", value.type_name()),
        ))
    }
}

impl TryFrom<Variant> for f64 {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        value.get_f64().ok_or(Error::new(
            InvalidData,
            format!("Not an f64, it's a {}", value.type_name()),
        ))
    }
}

impl TryFrom<Variant> for String {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        value.get_string().ok_or(Error::new(
            InvalidData,
            format!("Not a String, it's a {}", value.type_name()),
        ))
    }
}

impl TryFrom<Variant> for DateTime {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        value.get_datetime().ok_or(Error::new(
            InvalidData,
            format!("Not a DateTime, it's a {}", value.type_name()),
        ))
    }
}

impl TryFrom<Variant> for Bitmap {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        value.get_bitmap().ok_or(Error::new(
            InvalidData,
            format!("Not a Bitmap, it's a {}", value.type_name()),
        ))
    }
}
