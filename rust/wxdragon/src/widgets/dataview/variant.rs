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
            VariantType::Bitmap => "wxBitmap",
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
    /// Indicates whether this Rust wrapper owns the underlying wxVariant and is responsible for destroying it.
    /// Ownership is determined by how the pointer was obtained (e.g., from `From<*mut>` vs `From<*const>`), not by the pointer's type.
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

    pub fn is_owned(&self) -> bool {
        self.owned
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
        unsafe { ffi::wxd_Variant_SetDateTime(var.ptr, *dt.as_ref()) };
        var
    }

    pub fn from_bitmap(bmp: &Bitmap) -> Self {
        let var = Self::new();
        unsafe { ffi::wxd_Variant_SetBitmap(var.ptr, **bmp) };
        var
    }

    /// Returns the wxVariant type name (e.g., "string", "bool").
    pub fn type_name(&self) -> String {
        // Query required length first by calling with out_len=0
        let needed = unsafe { ffi::wxd_Variant_GetTypeName_Utf8(**self, std::ptr::null_mut(), 0) };
        if needed == 0 {
            return String::new();
        }
        let mut b = vec![
            0_u8;
            match needed.checked_add(1) {
                Some(len) => len,
                None => return String::new(),
            }
        ];
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
        let ok = unsafe { ffi::wxd_Variant_GetBool(**self, &mut out) };
        if ok { Some(out) } else { None }
    }

    pub fn get_i32(&self) -> Option<i32> {
        let mut out = 0_i32;
        let ok = unsafe { ffi::wxd_Variant_GetInt32(**self, &mut out) };
        if ok { Some(out) } else { None }
    }

    pub fn get_i64(&self) -> Option<i64> {
        let mut out = 0_i64;
        let ok = unsafe { ffi::wxd_Variant_GetInt64(**self, &mut out) };
        if ok { Some(out) } else { None }
    }

    pub fn get_f64(&self) -> Option<f64> {
        let mut out = 0_f64;
        let ok = unsafe { ffi::wxd_Variant_GetDouble(**self, &mut out) };
        if ok { Some(out) } else { None }
    }

    pub fn get_string(&self) -> Option<String> {
        let needed = unsafe { ffi::wxd_Variant_GetString_Utf8(**self, std::ptr::null_mut(), 0) };
        if needed == 0 {
            return None;
        }
        let mut buf = vec![
            0_u8;
            match needed.checked_add(1) {
                Some(len) => len,
                None => return Some(String::new()),
            }
        ];
        let len = buf.len();
        let w = unsafe { ffi::wxd_Variant_GetString_Utf8(**self, buf.as_mut_ptr() as _, len) };
        if w == 0 {
            return Some(String::new());
        }
        if let Some(pos) = buf.iter().position(|&b| b == 0) {
            buf.truncate(pos);
        }
        Some(String::from_utf8_lossy(&buf).into_owned())
    }

    pub fn get_datetime(&self) -> Option<DateTime> {
        let ptr = unsafe { ffi::wxd_Variant_GetDateTime(self.ptr) };
        if ptr.is_null() { None } else { Some(DateTime::from(ptr)) }
    }

    pub fn get_bitmap(&self) -> Option<Bitmap> {
        let ptr = unsafe { ffi::wxd_Variant_GetBitmapClone(**self) };
        if ptr.is_null() { None } else { Some(Bitmap::from(ptr)) }
    }
}

impl Clone for Variant {
    fn clone(&self) -> Self {
        let cloned = unsafe { ffi::wxd_Variant_Clone(**self) };
        Self::from(cloned)
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        if self.is_owned() && !self.ptr.is_null() {
            unsafe { ffi::wxd_Variant_Destroy(self.ptr) };
        }
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

impl From<*const ffi::wxd_Variant_t> for Variant {
    /// Does not take ownership of the raw pointer.
    fn from(ptr: *const ffi::wxd_Variant_t) -> Self {
        Variant { ptr, owned: false }
    }
}

impl From<*mut ffi::wxd_Variant_t> for Variant {
    /// Takes ownership of the raw pointer.
    fn from(ptr: *mut ffi::wxd_Variant_t) -> Self {
        let ptr = ptr as *const _;
        Variant { ptr, owned: true }
    }
}

impl TryFrom<Variant> for *const ffi::wxd_Variant_t {
    type Error = std::io::Error;
    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        if value.ptr.is_null() {
            return Err(Error::new(InvalidInput, "Variant pointer is null"));
        }
        if value.is_owned() {
            return Err(Error::new(
                InvalidData,
                "Variant owns the pointer, please use mutable version",
            ));
        }
        Ok(value.ptr)
    }
}

impl TryFrom<Variant> for *mut ffi::wxd_Variant_t {
    type Error = std::io::Error;
    fn try_from(mut value: Variant) -> Result<Self, Self::Error> {
        if value.ptr.is_null() {
            return Err(Error::new(InvalidInput, "Variant pointer is null"));
        }
        if !value.is_owned() {
            return Err(Error::new(
                InvalidData,
                "Variant does not own the pointer, please use const version",
            ));
        }
        value.owned = false;
        Ok(value.ptr as *mut _)
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

impl<'a> From<&'a DateTime> for Variant {
    fn from(value: &'a DateTime) -> Self {
        let var = Variant::new();
        unsafe { ffi::wxd_Variant_SetDateTime(var.ptr, **value) };
        var
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

use std::io::{Error, ErrorKind::InvalidData, ErrorKind::InvalidInput};

impl TryFrom<Variant> for bool {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        let type_name = value.type_name();
        value
            .get_bool()
            .ok_or(Error::new(InvalidData, format!("Not a bool, it's a {type_name}")))
    }
}

impl TryFrom<Variant> for i32 {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        let type_name = value.type_name();
        value
            .get_i32()
            .ok_or(Error::new(InvalidData, format!("Not an i32, it's a {type_name}")))
    }
}

impl TryFrom<Variant> for i64 {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        let type_name = value.type_name();
        value
            .get_i64()
            .ok_or(Error::new(InvalidData, format!("Not an i64, it's a {type_name}")))
    }
}

impl TryFrom<Variant> for f64 {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        let type_name = value.type_name();
        value
            .get_f64()
            .ok_or(Error::new(InvalidData, format!("Not an f64, it's a {type_name}")))
    }
}

impl TryFrom<Variant> for String {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        let type_name = value.type_name();
        value
            .get_string()
            .ok_or(Error::new(InvalidData, format!("Not a String, it's a {type_name}")))
    }
}

impl TryFrom<Variant> for DateTime {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        let type_name = value.type_name();
        value
            .get_datetime()
            .ok_or(Error::new(InvalidData, format!("Not a DateTime, it's a {type_name}")))
    }
}

impl TryFrom<Variant> for Bitmap {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        let type_name = value.type_name();
        value
            .get_bitmap()
            .ok_or(Error::new(InvalidData, format!("Not a Bitmap, it's a {type_name}")))
    }
}
