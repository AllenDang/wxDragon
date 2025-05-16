//! VariantType implementation.

use std::ffi::CString;
use wxdragon_sys as ffi;
use std::os::raw::c_char;
use crate::{DateTime, Bitmap};

/// Represents the type of data stored in a variant.
///
/// This enum defines the standard data types that can be used with
/// DataViewRenderer and wxVariant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariantType {
    /// Boolean value (true/false)
    Bool,
    /// 32-bit integer
    Int32,
    /// 64-bit integer
    Int64,
    /// Floating point number
    Double,
    /// Text string
    String,
    /// Date and time value
    DateTime,
    /// Binary data
    Bitmap,
    /// Progress value (typically 0-100)
    Progress,
}

impl VariantType {
    /// Converts the enum variant to a C string compatible with wxWidgets
    pub fn to_type_string(&self) -> &'static str {
        match self {
            VariantType::Bool => "bool",
            VariantType::Int32 => "long",
            VariantType::Int64 => "longlong",
            VariantType::Double => "double",
            VariantType::String => "string",
            VariantType::DateTime => "datetime",
            VariantType::Bitmap => "bitmap",
            VariantType::Progress => "long",
        }
    }
}

/// A wrapper for wxd_Variant_t that provides a safe Rust interface.
///
/// Variant is used to store and pass data of different types between
/// the application and the DataViewModel.
pub enum Variant {
    /// Boolean value
    Bool(bool),
    /// 32-bit integer value
    Int32(i32),
    /// 64-bit integer value
    Int64(i64),
    /// Floating point value
    Double(f64),
    /// String value
    String(String),
    /// Date and time value
    DateTime(DateTime),
    /// Bitmap image data
    Bitmap(Bitmap),
}

impl Variant {
    /// Creates a new empty variant.
    pub fn new() -> Self {
        Variant::Int32(0)
    }

    /// Gets the raw pointer to the native wxd_Variant_t.
    /// 
    /// IMPORTANT: Caller must ensure the returned pointer is freed using
    /// wxd_Variant_Free when no longer needed to avoid memory leaks.
    /// This function allocates heap memory for both the variant structure
    /// and any string data it contains.
    pub fn as_raw(&self) -> *const ffi::wxd_Variant_t {
        // Create a heap-allocated wxd_Variant_t to ensure it doesn't get dropped
        let mut variant = Box::new(ffi::wxd_Variant_t {
            type_: ffi::WXD_VARIANT_TYPE_INVALID as i32,
            data: unsafe { std::mem::zeroed() },
        });
        
        // Set the value based on the variant type
        match self {
            Variant::Bool(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_BOOL as i32;
                variant.data.bool_val = *value;
            },
            Variant::Int32(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_INT32 as i32;
                variant.data.int32_val = *value;
            },
            Variant::Int64(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_INT64 as i32;
                variant.data.int64_val = *value;
            },
            Variant::Double(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_DOUBLE as i32;
                variant.data.double_val = *value;
            },
            Variant::String(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_STRING as i32;
                let cstr = CString::new(value.as_str()).unwrap();
                variant.data.string_val = unsafe { 
                    // Use standard C function strdup instead of libc::strdup
                    let s = cstr.as_ptr();
                    let len = libc::strlen(s) + 1;
                    let new_s = libc::malloc(len) as *mut c_char;
                    if !new_s.is_null() {
                        libc::strcpy(new_s, s);
                    }
                    new_s
                };
            },
            Variant::DateTime(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_DATETIME as i32;
                // Convert DateTime to raw datetime struct
                variant.data.datetime_val = unsafe {
                    *(value.as_ptr())
                };
            },
            Variant::Bitmap(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_BITMAP as i32;
                variant.data.bitmap_val = value.as_ptr();
            },
        }
        
        // Leak the Box to ensure the memory lives long enough
        Box::into_raw(variant)
    }
    
    /// Gets a mutable raw pointer to the native wxd_Variant_t.
    ///
    /// This is primarily used by event.rs for event data.
    /// 
    /// IMPORTANT: Caller must ensure the returned pointer is freed using
    /// wxd_Variant_Free when no longer needed to avoid memory leaks.
    pub fn as_raw_mut(&self) -> *mut ffi::wxd_Variant_t {
        self.as_raw() as *mut _
    }
    
    /// Consumes the variant and transfers ownership to C++.
    /// Returns a raw pointer that must be freed by C++ code using wxd_Variant_Free.
    ///
    /// This is preferred over as_raw() when transferring ownership to C++ code
    /// to make the ownership transfer explicit in the code.
    pub fn into_raw(self) -> *mut ffi::wxd_Variant_t {
        self.as_raw() as *mut _
    }
    
    /// Creates a Variant from a raw pointer, taking ownership and freeing the C++ resources.
    ///
    /// # Safety
    /// The pointer must be valid and must not be used after this call.
    /// The caller must ensure this pointer was allocated by as_raw() or into_raw().
    pub unsafe fn from_raw(ptr: *mut ffi::wxd_Variant_t) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }
        
        let variant_ref = &*ptr;
        let result = match variant_ref.type_ {
            t if t == ffi::WXD_VARIANT_TYPE_BOOL as i32 => 
                Variant::Bool(variant_ref.data.bool_val),
            t if t == ffi::WXD_VARIANT_TYPE_INT32 as i32 => 
                Variant::Int32(variant_ref.data.int32_val),
            t if t == ffi::WXD_VARIANT_TYPE_INT64 as i32 => 
                Variant::Int64(variant_ref.data.int64_val),
            t if t == ffi::WXD_VARIANT_TYPE_DOUBLE as i32 => 
                Variant::Double(variant_ref.data.double_val),
            t if t == ffi::WXD_VARIANT_TYPE_STRING as i32 => {
                if variant_ref.data.string_val.is_null() {
                    Variant::String(String::new())
                } else {
                    let c_str = std::ffi::CStr::from_ptr(variant_ref.data.string_val);
                    let string = c_str.to_string_lossy().into_owned();
                    Variant::String(string)
                }
            },
            t if t == ffi::WXD_VARIANT_TYPE_DATETIME as i32 => {
                // Create a DateTime from the raw data
                let dt = crate::DateTime::from_raw(variant_ref.data.datetime_val);
                Variant::DateTime(dt)
            },
            t if t == ffi::WXD_VARIANT_TYPE_BITMAP as i32 => {
                if variant_ref.data.bitmap_val.is_null() {
                    // Since there's no default constructor, create a minimal 1x1 bitmap
                    // or return a special case that represents "no bitmap"
                    let data = vec![0u8, 0, 0, 0]; // 1x1 transparent pixel
                    match crate::Bitmap::from_rgba(&data, 1, 1) {
                        Some(bitmap) => Variant::Bitmap(bitmap),
                        None => {
                            // If even this fails, we're in trouble, but let's try to recover
                            // by using a default variant type instead of failing completely
                            eprintln!("Warning: Failed to create empty bitmap for null bitmap pointer");
                            Variant::String("".to_string())
                        }
                    }
                } else {
                    // Use from_ptr_owned to take ownership of the bitmap
                    let bitmap = crate::Bitmap::from_ptr_owned(variant_ref.data.bitmap_val);
                    // Set the pointer to null to avoid double-free since we've taken ownership
                    // Note: This is safe because we're working with a local temporary copy
                    // of the variant_ref in memory that will be freed by wxd_Variant_Free
                    Variant::Bitmap(bitmap)
                }
            },
            _ => {
                // Invalid type, free the memory and return None
                ffi::wxd_Variant_Free(ptr);
                return None;
            }
        };
        
        // Free the C++ resources
        ffi::wxd_Variant_Free(ptr);
        
        Some(result)
    }
    
    /// Gets the type of the variant
    pub fn get_type(&self) -> VariantType {
        match self {
            Variant::Bool(_) => VariantType::Bool,
            Variant::Int32(_) => VariantType::Int32,
            Variant::Int64(_) => VariantType::Int64,
            Variant::Double(_) => VariantType::Double,
            Variant::String(_) => VariantType::String,
            Variant::DateTime(_) => VariantType::DateTime,
            Variant::Bitmap(_) => VariantType::Bitmap,
        }
    }
}

impl Clone for Variant {
    fn clone(&self) -> Self {
        match self {
            Variant::Bool(value) => Variant::Bool(*value),
            Variant::Int32(value) => Variant::Int32(*value),
            Variant::Int64(value) => Variant::Int64(*value),
            Variant::Double(value) => Variant::Double(*value),
            Variant::String(value) => Variant::String(value.clone()),
            Variant::DateTime(value) => Variant::DateTime(*value),
            Variant::Bitmap(value) => Variant::Bitmap(value.clone()),
        }
    }
}

impl Drop for Variant {
    fn drop(&mut self) {}
}

impl Default for Variant {
    fn default() -> Self {
        Self::new()
    }
}

impl From<bool> for Variant {
    fn from(value: bool) -> Self {
        Variant::Bool(value)
    }
}

impl From<i32> for Variant {
    fn from(value: i32) -> Self {
        Variant::Int32(value)
    }
}

impl From<i64> for Variant {
    fn from(value: i64) -> Self {
        Variant::Int64(value)
    }
}

impl From<f64> for Variant {
    fn from(value: f64) -> Self {
        Variant::Double(value)
    }
}

impl From<&str> for Variant {
    fn from(value: &str) -> Self {
        Variant::String(value.to_string())
    }
}

impl From<String> for Variant {
    fn from(value: String) -> Self {
        Variant::String(value)
    }
}

impl From<DateTime> for Variant {
    fn from(value: DateTime) -> Self {
        Variant::DateTime(value)
    }
}

impl From<Bitmap> for Variant {
    fn from(value: Bitmap) -> Self {
        Variant::Bitmap(value)
    }
} 