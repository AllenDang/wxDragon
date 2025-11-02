//! Variant wrapper for typed wxVariant C API.

use crate::utils::ArrayString;
use crate::{Bitmap, DateTime};
use std::ffi::CStr;
use std::marker::PhantomData;
use std::rc::Rc;
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
    ArrayString,
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
            VariantType::ArrayString => "wxArrayString",
            VariantType::Progress => "long",
            VariantType::IconText => "wxDataViewIconText",
        }
    }
}

/// Safe Rust wrapper over a wxVariant pointer (wxd_Variant_t).
///
/// Owns the underlying wxVariant by default and destroys it in Drop.
pub struct Variant {
    ptr: *mut ffi::wxd_Variant_t,
    /// Indicates whether this Rust wrapper owns the underlying wxVariant and is responsible for destroying it.
    /// Ownership is determined by how the pointer was obtained (e.g., from `From<*mut>` vs `From<*const>`), not by the pointer's type.
    owned: bool,
    // Prevent Send/Sync: wxWidgets objects are not thread-safe and must stay on UI thread.
    _nosend_nosync: PhantomData<Rc<()>>,
}

impl Variant {
    /// Create an empty variant.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_Variant_CreateEmpty() };
        assert!(!ptr.is_null(), "Failed to create wxVariant");
        Self {
            ptr,
            owned: true,
            _nosend_nosync: PhantomData,
        }
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

    pub fn from_datetime(dt: &DateTime) -> Self {
        let mut var = Self::new();
        unsafe { ffi::wxd_Variant_SetDateTime(var.as_mut_ptr(), dt.as_const_ptr()) };
        var
    }

    pub fn from_bitmap(bmp: &Bitmap) -> Self {
        let mut var = Self::new();
        unsafe { ffi::wxd_Variant_SetBitmap(var.as_mut_ptr(), bmp.as_const_ptr()) };
        var
    }

    /// Create a new variant from a slice of Rust strings by storing a wxArrayString by value inside wxVariant.
    pub fn from_array_string(strings: &ArrayString) -> Self {
        let mut var = Self::new();
        unsafe { ffi::wxd_Variant_SetArrayString(var.as_mut_ptr(), strings.as_const_ptr()) };
        var
    }

    /// Returns a const raw pointer to the underlying wxd_Variant_t.
    ///
    /// Ownership notes:
    /// - This does not transfer ownership; the pointer is only valid while this `Variant` is alive (unless moved via `into_raw_*`).
    /// - Do not destroy this pointer yourself. Destruction is handled by the owning wrapper or by `into_raw_mut` transfer.
    pub fn as_const_ptr(&self) -> *const ffi::wxd_Variant_t {
        self.ptr as *const _
    }

    /// Returns a mutable raw pointer to the underlying wxd_Variant_t.
    ///
    /// Use with care; prefer safe methods when possible.
    ///
    /// Ownership notes:
    /// - This does not transfer ownership. If you mutate through this pointer, you must uphold exclusive access and invariants.
    /// - Do not destroy the returned pointer here; use `into_raw_mut` to transfer ownership if you need to manage lifetime manually.
    pub fn as_mut_ptr(&mut self) -> *mut ffi::wxd_Variant_t {
        self.ptr
    }

    /// Consumes self and returns a raw mutable pointer, transferring ownership to the caller.
    ///
    /// After calling this, you must NOT use the original `Variant` again.
    ///
    /// Caller responsibilities:
    /// - You now own the pointer and must destroy it exactly once with `wxd_Variant_Destroy`.
    /// - Ensure no further use-after-free occurs.
    pub fn into_raw_mut(self) -> *mut ffi::wxd_Variant_t {
        assert!(self.owned, "into_raw_mut can only be called on owning Variant instances");
        let ptr = self.ptr;
        std::mem::forget(self);
        ptr
    }

    /// Consumes a borrowed (non-owning) wrapper and returns a raw const pointer without taking ownership.
    ///
    /// Panics if called on an owning wrapper to avoid leaking the owned resource.
    ///
    /// Caller responsibilities:
    /// - This does NOT transfer ownership; do not destroy the returned pointer.
    /// - The pointer remains valid only while the original owner keeps it alive.
    pub fn into_raw_const(self) -> *const ffi::wxd_Variant_t {
        assert!(
            !self.owned,
            "into_raw_const must only be used on non-owning (borrowed) wrappers"
        );
        let ptr = self.ptr as *const _;
        std::mem::forget(self);
        ptr
    }

    /// Returns the wxVariant type name (e.g., "string", "bool").
    pub fn type_name(&self) -> String {
        // Query required length first by calling with out_len=0
        let needed = unsafe { ffi::wxd_Variant_GetTypeName_Utf8(self.as_const_ptr(), std::ptr::null_mut(), 0) };
        if needed <= 0 {
            return String::new();
        }
        let mut b = vec![
            0;
            match needed.checked_add(1) {
                Some(len) => len as usize,
                None => return String::new(),
            }
        ];
        let w = unsafe { ffi::wxd_Variant_GetTypeName_Utf8(self.as_const_ptr(), b.as_mut_ptr(), b.len()) };
        if w <= 0 {
            return String::new();
        }
        unsafe { CStr::from_ptr(b.as_ptr()).to_string_lossy().into_owned() }
    }

    pub fn get_bool(&self) -> Option<bool> {
        let mut out = false;
        let ok = unsafe { ffi::wxd_Variant_GetBool(self.as_const_ptr(), &mut out) };
        if ok { Some(out) } else { None }
    }

    pub fn get_i32(&self) -> Option<i32> {
        let mut out = 0_i32;
        let ok = unsafe { ffi::wxd_Variant_GetInt32(self.as_const_ptr(), &mut out) };
        if ok { Some(out) } else { None }
    }

    pub fn get_i64(&self) -> Option<i64> {
        let mut out = 0_i64;
        let ok = unsafe { ffi::wxd_Variant_GetInt64(self.as_const_ptr(), &mut out) };
        if ok { Some(out) } else { None }
    }

    pub fn get_f64(&self) -> Option<f64> {
        let mut out = 0_f64;
        let ok = unsafe { ffi::wxd_Variant_GetDouble(self.as_const_ptr(), &mut out) };
        if ok { Some(out) } else { None }
    }

    pub fn get_string(&self) -> Option<String> {
        let needed = unsafe { ffi::wxd_Variant_GetString_Utf8(self.as_const_ptr(), std::ptr::null_mut(), 0) };
        if needed < 0 {
            return None;
        }
        let mut buf = vec![
            0;
            match needed.checked_add(1) {
                Some(len) => len as usize,
                None => return Some(String::new()),
            }
        ];
        let w = unsafe { ffi::wxd_Variant_GetString_Utf8(self.as_const_ptr(), buf.as_mut_ptr(), buf.len()) };
        if w < 0 {
            return None;
        }
        unsafe { Some(CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()) }
    }

    pub fn get_datetime(&self) -> Option<DateTime> {
        let ptr = unsafe { ffi::wxd_Variant_GetDateTime(self.as_const_ptr()) };
        if ptr.is_null() { None } else { Some(DateTime::from(ptr)) }
    }

    pub fn get_bitmap(&self) -> Option<Bitmap> {
        let ptr = unsafe { ffi::wxd_Variant_GetBitmapClone(self.as_const_ptr()) };
        if ptr.is_null() { None } else { Some(Bitmap::from(ptr)) }
    }

    /// If this variant stores a wxArrayString, return it as an ArrayString.
    pub fn get_array_string(&self) -> Option<ArrayString> {
        let ptr = unsafe { ffi::wxd_Variant_GetArrayStringClone(self.as_const_ptr()) };
        if ptr.is_null() { None } else { Some(ArrayString::from(ptr)) }
    }
}

impl Clone for Variant {
    fn clone(&self) -> Self {
        let cloned = unsafe { ffi::wxd_Variant_Clone(self.as_const_ptr()) };
        assert!(!cloned.is_null(), "Failed to clone wxVariant");
        Self::from(cloned)
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        if self.is_owned() && !self.ptr.is_null() {
            unsafe { ffi::wxd_Variant_Destroy(self.as_mut_ptr()) };
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
        assert!(!ptr.is_null(), "invalid null pointer passed to Variant::from");
        Variant {
            ptr: ptr as *mut _,
            owned: false,
            _nosend_nosync: PhantomData,
        }
    }
}

impl From<*mut ffi::wxd_Variant_t> for Variant {
    /// Takes ownership of the raw pointer.
    fn from(ptr: *mut ffi::wxd_Variant_t) -> Self {
        assert!(!ptr.is_null(), "invalid null pointer passed to Variant::from");
        Variant {
            ptr,
            owned: true,
            _nosend_nosync: PhantomData,
        }
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
        Self::from(&value)
    }
}

impl<'a> From<&'a DateTime> for Variant {
    fn from(value: &'a DateTime) -> Self {
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

impl<T: AsRef<str>> From<Vec<T>> for Variant {
    fn from(value: Vec<T>) -> Self {
        let arr = ArrayString::from(value);
        Variant::from(&arr)
    }
}

impl<T: AsRef<str>> From<&[T]> for Variant {
    fn from(value: &[T]) -> Self {
        let arr = ArrayString::from(value);
        Variant::from(&arr)
    }
}

impl<T: AsRef<str>> From<&Vec<T>> for Variant {
    fn from(value: &Vec<T>) -> Self {
        let arr = ArrayString::from(value.as_slice());
        Variant::from(&arr)
    }
}

impl From<ArrayString> for Variant {
    fn from(value: ArrayString) -> Self {
        Variant::from_array_string(&value)
    }
}

impl From<&ArrayString> for Variant {
    fn from(value: &ArrayString) -> Self {
        Variant::from_array_string(value)
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

impl TryFrom<Variant> for ArrayString {
    type Error = std::io::Error;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        let type_name = value.type_name();
        value
            .get_array_string()
            .ok_or(Error::new(InvalidData, format!("Not a wxArrayString, it's a {type_name}")))
    }
}

#[cfg(test)]
mod tests {
    use super::Variant;
    use crate::ArrayString;

    #[test]
    fn variant_array_string_roundtrip() {
        let src = vec!["alpha", "beta", "gamma"];
        let v = Variant::from(&src);
        // Type name differs by backend; ensure it's some array string form
        let tn = v.type_name();
        println!("Variant type name: {}", tn);
        assert!(tn.contains("string"), "unexpected type name: {}", tn);
        let got = v.get_array_string().expect("expected array string");
        assert_eq!(src, got.get_strings());

        // TryFrom
        let got2: ArrayString = v.clone().try_into().expect("convert to ArrayString");
        assert_eq!(src, got2.get_strings());
    }
}
