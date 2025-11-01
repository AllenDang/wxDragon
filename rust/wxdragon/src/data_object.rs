use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

/// Represents a format for data transfer operations.
pub struct DataFormat {
    format: i32,
}

impl DataFormat {
    /// Creates a new data format with the specified format type.
    pub fn new(format: i32) -> Self {
        Self { format }
    }

    /// Gets the internal format value.
    pub fn get_format(&self) -> i32 {
        self.format
    }

    /// Text format
    pub const TEXT: i32 = 1; // wxDF_TEXT

    /// Bitmap format
    pub const BITMAP: i32 = 2; // wxDF_BITMAP

    /// File format
    pub const FILENAME: i32 = 4; // wxDF_FILENAME
}

/// Trait that all data objects must implement.
pub trait DataObject {
    /// Gets the raw pointer to the underlying data object.
    fn as_data_object_ptr(&self) -> *mut ffi::wxd_DataObject_t;
}

/// Trait for objects that can transfer ownership of their underlying data.
pub trait TransferOwnership {
    /// Marks the object as having transferred ownership to another component.
    fn transfer_ownership(&mut self);
}

/// Base class for data objects.
#[derive(Clone)]
pub struct DataObjectBase {
    ptr: *mut ffi::wxd_DataObject_t,
    owned: bool,
}

impl DataObjectBase {
    /// Creates a new data object from a raw pointer.
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_DataObject_t, owned: bool) -> Self {
        Self { ptr, owned }
    }

    /// Gets the raw pointer.
    pub(crate) fn as_ptr(&self) -> *mut ffi::wxd_DataObject_t {
        self.ptr
    }

    /// Flags the object as having transferred ownership to another component
    /// (like clipboard). After this call, the Drop implementation will not
    /// free the underlying C++ object.
    pub(crate) fn transfer_ownership(&mut self) {
        self.owned = false;
    }
}

impl TransferOwnership for DataObjectBase {
    fn transfer_ownership(&mut self) {
        self.owned = false;
    }
}

impl DataObject for DataObjectBase {
    fn as_data_object_ptr(&self) -> *mut ffi::wxd_DataObject_t {
        self.ptr
    }
}

impl Drop for DataObjectBase {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            // If we own the object, we need to free it
            // Currently, there is no explicit destroy function in the C API
            // for base DataObject type, as it should be handled by the
            // derived types' Drop implementations
        }
    }
}

/// Data object for simple text data
#[derive(Clone)]
pub struct TextDataObject {
    data_object: DataObjectBase,
}

impl TextDataObject {
    /// Creates a new text data object with the specified text.
    pub fn new(text: &str) -> Self {
        let c_text = CString::new(text).unwrap_or_default();
        let ptr = unsafe { ffi::wxd_TextDataObject_Create(c_text.as_ptr()) };
        Self {
            data_object: DataObjectBase::from_ptr(ptr as *mut ffi::wxd_DataObject_t, true),
        }
    }

    /// Gets the text from the data object.
    pub fn get_text(&self) -> String {
        let ptr = self.data_object.as_ptr() as *mut ffi::wxd_TextDataObject_t;
        let len = unsafe { ffi::wxd_TextDataObject_GetText(ptr, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buf: Vec<i8> = vec![0; len as usize + 1];
        unsafe { ffi::wxd_TextDataObject_GetText(ptr, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() }
    }

    /// Sets the text contained in this data object.
    pub fn set_text(&mut self, text: &str) {
        let text_cstring = CString::new(text).unwrap_or_default();
        unsafe {
            ffi::wxd_TextDataObject_SetText(
                self.data_object.as_ptr() as *mut ffi::wxd_TextDataObject_t,
                text_cstring.as_ptr(),
            );
        }
    }

    /// Gets the underlying DataObject.
    pub fn as_data_object(&self) -> &DataObjectBase {
        &self.data_object
    }

    /// Gets the underlying DataObject as mutable.
    pub fn as_data_object_mut(&mut self) -> &mut DataObjectBase {
        &mut self.data_object
    }
}

impl DataObject for TextDataObject {
    fn as_data_object_ptr(&self) -> *mut ffi::wxd_DataObject_t {
        self.data_object.as_ptr()
    }
}

impl Drop for TextDataObject {
    fn drop(&mut self) {
        if !self.data_object.as_ptr().is_null() && self.data_object.owned {
            unsafe { ffi::wxd_TextDataObject_Destroy(self.data_object.as_ptr() as *mut ffi::wxd_TextDataObject_t) };
        }
    }
}

impl TransferOwnership for TextDataObject {
    fn transfer_ownership(&mut self) {
        self.data_object.transfer_ownership();
    }
}

/// Data object for file paths
#[derive(Clone)]
pub struct FileDataObject {
    data_object: DataObjectBase,
}

impl FileDataObject {
    /// Creates a new file data object.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_FileDataObject_Create() };
        Self {
            data_object: DataObjectBase::from_ptr(ptr as *mut ffi::wxd_DataObject_t, true),
        }
    }

    /// Adds a file to the data object.
    pub fn add_file(&mut self, file_path: &str) {
        let c_path = CString::new(file_path).unwrap_or_default();
        unsafe { ffi::wxd_FileDataObject_AddFile(self.data_object.as_ptr() as *mut ffi::wxd_FileDataObject_t, c_path.as_ptr()) };
    }

    /// Gets the number of files in the data object.
    pub fn get_file_count(&self) -> usize {
        unsafe { ffi::wxd_FileDataObject_GetFileCount(self.data_object.as_ptr() as *mut ffi::wxd_FileDataObject_t) as usize }
    }

    /// Gets a file path from the data object at the specified index.
    pub fn get_file(&self, index: usize) -> String {
        let obj = self.data_object.as_ptr() as *mut ffi::wxd_FileDataObject_t;
        let mut buffer = vec![0; 1024]; // Initial buffer size
        let len = unsafe { ffi::wxd_FileDataObject_GetFile(obj, index as i32, buffer.as_mut_ptr(), buffer.len()) };

        if len > 0 {
            unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().to_string() }
        } else {
            String::new()
        }
    }

    /// Gets all files from the data object.
    pub fn get_files(&self) -> Vec<String> {
        let count = self.get_file_count();
        let mut files = Vec::with_capacity(count);
        for i in 0..count {
            files.push(self.get_file(i));
        }
        files
    }

    /// Gets the filenames contained in this data object.
    /// This is an alias for get_files() to maintain compatibility with the DND API.
    pub fn get_filenames(&self) -> Vec<String> {
        self.get_files()
    }

    /// Gets the underlying DataObject.
    pub fn as_data_object(&self) -> &DataObjectBase {
        &self.data_object
    }

    /// Gets the underlying DataObject as mutable.
    pub fn as_data_object_mut(&mut self) -> &mut DataObjectBase {
        &mut self.data_object
    }
}

impl DataObject for FileDataObject {
    fn as_data_object_ptr(&self) -> *mut ffi::wxd_DataObject_t {
        self.data_object.as_ptr()
    }
}

impl Drop for FileDataObject {
    fn drop(&mut self) {
        if !self.data_object.as_ptr().is_null() && self.data_object.owned {
            unsafe { ffi::wxd_FileDataObject_Destroy(self.data_object.as_ptr() as *mut ffi::wxd_FileDataObject_t) };
        }
    }
}

impl Default for FileDataObject {
    fn default() -> Self {
        Self::new()
    }
}

impl TransferOwnership for FileDataObject {
    fn transfer_ownership(&mut self) {
        self.data_object.transfer_ownership();
    }
}

/// Data object for bitmap data
#[derive(Clone)]
pub struct BitmapDataObject {
    data_object: DataObjectBase,
}

impl BitmapDataObject {
    /// Creates a new bitmap data object with the specified bitmap.
    pub fn new(bitmap: &crate::bitmap::Bitmap) -> Self {
        let ptr = unsafe { ffi::wxd_BitmapDataObject_Create(bitmap.as_const_ptr()) };
        Self {
            data_object: DataObjectBase::from_ptr(ptr as *mut ffi::wxd_DataObject_t, true),
        }
    }

    /// Gets the bitmap from the data object.
    pub fn get_bitmap(&self) -> Option<crate::bitmap::Bitmap> {
        let ptr = unsafe { ffi::wxd_BitmapDataObject_GetBitmap(self.data_object.as_ptr() as *mut ffi::wxd_BitmapDataObject_t) };

        if ptr.is_null() {
            None
        } else {
            // Create a bitmap object that takes ownership of the pointer
            Some(crate::bitmap::Bitmap::from(ptr))
        }
    }

    /// Gets the underlying DataObject.
    pub fn as_data_object(&self) -> &DataObjectBase {
        &self.data_object
    }

    /// Gets the underlying DataObject as mutable.
    pub fn as_data_object_mut(&mut self) -> &mut DataObjectBase {
        &mut self.data_object
    }
}

impl DataObject for BitmapDataObject {
    fn as_data_object_ptr(&self) -> *mut ffi::wxd_DataObject_t {
        self.data_object.as_ptr()
    }
}

impl Drop for BitmapDataObject {
    fn drop(&mut self) {
        if !self.data_object.as_ptr().is_null() && self.data_object.owned {
            // Currently, there's no explicit destroy function for BitmapDataObject in the C API,
            // But the destructor will be called via the normal C++ destruction process
            // when the DataObject is freed
            // unsafe {
            //     ffi::wxd_BitmapDataObject_Destroy(
            //         self.data_object.as_ptr() as *mut ffi::wxd_BitmapDataObject_t
            //     );
            // }
        }
    }
}

impl TransferOwnership for BitmapDataObject {
    fn transfer_ownership(&mut self) {
        self.data_object.transfer_ownership();
    }
}
