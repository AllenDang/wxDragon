//! VariantType implementation.

use crate::{Bitmap, DateTime};
use std::ffi::CString;
use wxdragon_sys as ffi;

/// Represents the type of data stored in a variant.
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
    /// Bitmap image
    Bitmap,
    /// Progress value (typically 0-100)
    Progress,
    /// wxDataViewIconText
    IconText,
}

impl VariantType {
    /// Converts the enum variant to a C string compatible with wxWidgets
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
    /// Raw, borrowed pointer to a wxBitmap for FFI (used by DataViewCtrl GetValue)
    BitmapBorrowed(*mut ffi::wxd_Bitmap_t),
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
            }
            Variant::Int32(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_INT32 as i32;
                variant.data.int32_val = *value;
            }
            Variant::Int64(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_INT64 as i32;
                variant.data.int64_val = *value;
            }
            Variant::Double(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_DOUBLE as i32;
                variant.data.double_val = *value;
            }
            Variant::String(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_STRING as i32;
                variant.data.string_val =
                    CString::new(value.as_str()).unwrap_or_default().into_raw();
            }
            Variant::DateTime(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_DATETIME as i32;
                // Convert DateTime to raw datetime struct
                variant.data.datetime_val = unsafe { *(value.as_ptr()) };
            }
            Variant::Bitmap(value) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_BITMAP_RUST_BORROWED as i32;
                variant.data.bitmap_val = value.as_ptr();
            }
            Variant::BitmapBorrowed(ptr) => {
                variant.type_ = ffi::WXD_VARIANT_TYPE_BITMAP_RUST_BORROWED as i32;
                variant.data.bitmap_val = *ptr;
            }
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
            t if t == ffi::WXD_VARIANT_TYPE_BOOL as i32 => Variant::Bool(variant_ref.data.bool_val),
            t if t == ffi::WXD_VARIANT_TYPE_INT32 as i32 => {
                Variant::Int32(variant_ref.data.int32_val)
            }
            t if t == ffi::WXD_VARIANT_TYPE_INT64 as i32 => {
                Variant::Int64(variant_ref.data.int64_val)
            }
            t if t == ffi::WXD_VARIANT_TYPE_DOUBLE as i32 => {
                Variant::Double(variant_ref.data.double_val)
            }
            t if t == ffi::WXD_VARIANT_TYPE_STRING as i32 => {
                if variant_ref.data.string_val.is_null() {
                    Variant::String(String::new())
                } else {
                    let c_str = std::ffi::CStr::from_ptr(variant_ref.data.string_val);
                    let string = c_str.to_string_lossy().into_owned();
                    Variant::String(string)
                }
            }
            t if t == ffi::WXD_VARIANT_TYPE_DATETIME as i32 => {
                // Create a DateTime from the raw data
                let dt = crate::DateTime::from_raw(variant_ref.data.datetime_val);
                Variant::DateTime(dt)
            }
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
                            log::warn!(
                                "Warning: Failed to create empty bitmap for null bitmap pointer"
                            );
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
            }
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
            Variant::BitmapBorrowed(_) => VariantType::Bitmap,
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
            Variant::Bitmap(bitmap) => Variant::Bitmap(bitmap.clone()),
            Variant::BitmapBorrowed(ptr) => Variant::BitmapBorrowed(*ptr),
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

impl std::fmt::Debug for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::Bool(val) => write!(f, "Bool({val})"),
            Variant::Int32(val) => write!(f, "Int32({val})"),
            Variant::Int64(val) => write!(f, "Int64({val})"),
            Variant::Double(val) => write!(f, "Double({val})"),
            Variant::String(val) => write!(f, "String({val})"),
            Variant::DateTime(val) => write!(f, "DateTime({val:?})"),
            Variant::Bitmap(_) => write!(f, "Bitmap(...)"),
            Variant::BitmapBorrowed(ptr) => write!(f, "BitmapBorrowed({ptr:?})"),
        }
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

impl<'a> From<&'a Bitmap> for Variant {
    fn from(value: &'a Bitmap) -> Self {
        Variant::BitmapBorrowed(value.as_borrowable_ptr())
    }
}

/// Converts a Variant to a C wxd_Variant_t
pub fn to_raw_variant(value: &Variant) -> ffi::wxd_Variant_t {
    let mut result = ffi::wxd_Variant_t {
        type_: 0,
        data: unsafe { std::mem::zeroed() },
    };

    match value {
        Variant::Bool(val) => {
            result.type_ = ffi::WXD_VARIANT_TYPE_BOOL as i32;
            result.data.bool_val = *val;
        }
        Variant::Int32(val) => {
            result.type_ = ffi::WXD_VARIANT_TYPE_INT32 as i32;
            result.data.int32_val = *val;
        }
        Variant::Int64(val) => {
            result.type_ = ffi::WXD_VARIANT_TYPE_INT64 as i32;
            result.data.int64_val = *val;
        }
        Variant::Double(val) => {
            result.type_ = ffi::WXD_VARIANT_TYPE_DOUBLE as i32;
            result.data.double_val = *val;
        }
        Variant::String(val) => {
            result.type_ = ffi::WXD_VARIANT_TYPE_STRING as i32;

            // Use proper string duplication to ensure C++ can safely free it
            result.data.string_val = CString::new(val.as_str()).unwrap_or_default().into_raw();
        }
        Variant::DateTime(val) => {
            result.type_ = ffi::WXD_VARIANT_TYPE_DATETIME as i32;
            unsafe {
                result.data.datetime_val = *val.as_ptr();
            }
        }
        Variant::Bitmap(val) => {
            // This path is for an owned Bitmap, uses the FFI-cloned mechanism
            result.type_ = ffi::WXD_VARIANT_TYPE_BITMAP as i32;
            let original_rust_owned_ptr = val.as_ptr();
            if original_rust_owned_ptr.is_null() {
                result.data.bitmap_val = std::ptr::null_mut();
            } else {
                // Ask C++ to clone the bitmap. This new bitmap is on the C++ heap.
                // C++ GetValueByRow will be responsible for Destroying this clone later.
                let cloned_ptr_on_cpp_heap =
                    unsafe { ffi::wxd_Bitmap_Clone(original_rust_owned_ptr) };
                result.data.bitmap_val = cloned_ptr_on_cpp_heap;
            }
        }
        Variant::BitmapBorrowed(borrowed_ptr) => {
            // New path for borrowed bitmap pointer
            result.type_ = ffi::WXD_VARIANT_TYPE_BITMAP_RUST_BORROWED as i32;
            result.data.bitmap_val = *borrowed_ptr; // Pass the borrowed pointer directly
        }
    }

    result
}

/// Converts a C wxd_Variant_t to a Variant
///
/// # Safety
/// The caller must ensure the raw pointer is valid and points to a properly initialized wxd_Variant_t.
pub unsafe fn from_raw_variant(raw: *const ffi::wxd_Variant_t) -> Variant {
    if raw.is_null() {
        return Variant::String(String::new());
    }

    match (*raw).type_ {
        t if t == ffi::WXD_VARIANT_TYPE_BOOL as i32 => Variant::Bool((*raw).data.bool_val),
        t if t == ffi::WXD_VARIANT_TYPE_INT32 as i32 => Variant::Int32((*raw).data.int32_val),
        t if t == ffi::WXD_VARIANT_TYPE_INT64 as i32 => Variant::Int64((*raw).data.int64_val),
        t if t == ffi::WXD_VARIANT_TYPE_DOUBLE as i32 => Variant::Double((*raw).data.double_val),
        t if t == ffi::WXD_VARIANT_TYPE_STRING as i32 => {
            if (*raw).data.string_val.is_null() {
                Variant::String(String::new())
            } else {
                let c_str = std::ffi::CStr::from_ptr((*raw).data.string_val);
                Variant::String(c_str.to_string_lossy().to_string())
            }
        }
        t if t == ffi::WXD_VARIANT_TYPE_DATETIME as i32 => {
            // Create a DateTime from the raw data
            let dt = crate::DateTime::from_raw((*raw).data.datetime_val);
            Variant::DateTime(dt)
        }
        t if t == ffi::WXD_VARIANT_TYPE_BITMAP as i32 => {
            if (*raw).data.bitmap_val.is_null() {
                // Create a minimal 1x1 transparent bitmap as fallback
                match crate::Bitmap::from_rgba(&[0, 0, 0, 0], 1, 1) {
                    Some(bitmap) => Variant::Bitmap(bitmap),
                    None => Variant::String(String::new()), // Last resort fallback
                }
            } else {
                // For bitmaps from C++, we need to clone them as we don't own them
                let ptr = (*raw).data.bitmap_val;
                let cloned_ptr = ffi::wxd_Bitmap_Clone(ptr);
                if !cloned_ptr.is_null() {
                    let bitmap = crate::Bitmap::from_ptr_owned(cloned_ptr);
                    Variant::Bitmap(bitmap)
                } else {
                    // If clone fails, fallback to a small placeholder
                    match crate::Bitmap::from_rgba(&[255, 0, 0, 255], 1, 1) {
                        Some(bitmap) => Variant::Bitmap(bitmap),
                        None => Variant::String(String::new()),
                    }
                }
            }
        }
        _ => {
            // Default for unknown/unsupported types
            Variant::String(String::new())
        }
    }
}
