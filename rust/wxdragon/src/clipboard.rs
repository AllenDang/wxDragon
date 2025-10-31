use crate::data_object::DataObject;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

/// A struct representing the system clipboard.
///
/// The clipboard can be used to copy data to or paste data from.
/// It's typically accessed via the global `Clipboard::get()` method.
///
/// # Example
/// ```rust,no_run
/// use wxdragon::prelude::*;
///
/// // Set text to clipboard
/// let clipboard = Clipboard::get();
/// if clipboard.set_text("Hello, Clipboard!") {
///     println!("Text copied to clipboard");
/// }
///
/// // Get text from clipboard
/// if let Some(text) = clipboard.get_text() {
///     println!("Clipboard text: {}", text);
/// }
/// ```
#[derive(Clone, Copy)]
pub struct Clipboard {
    ptr: *mut ffi::wxd_Clipboard_t,
}

impl Clipboard {
    /// Get the global clipboard instance
    pub fn get() -> Self {
        let ptr = unsafe { ffi::wxd_Clipboard_Get() };
        Self { ptr }
    }

    /// Open the clipboard before accessing data
    pub fn open(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Clipboard_Open(self.ptr) }
    }

    /// Close the clipboard after accessing data
    pub fn close(&self) {
        if self.ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Clipboard_Close(self.ptr) }
    }

    /// Check if the clipboard is opened
    pub fn is_opened(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Clipboard_IsOpened(self.ptr) }
    }

    /// Add data to the clipboard
    ///
    /// # Safety
    /// This method transfers ownership of the data object to the clipboard if successful.
    /// The data object should no longer be used after a successful call.
    pub fn add_data<T: DataObject + crate::data_object::TransferOwnership>(&self, data: &mut T) -> bool {
        if self.ptr.is_null() {
            return false;
        }

        let data_ptr = data.as_data_object_ptr();
        let success = unsafe { ffi::wxd_Clipboard_AddData(self.ptr, data_ptr) };

        if success {
            // In wxWidgets, wxClipboard::AddData takes ownership of the data object
            // So we mark the Rust object as having transferred ownership
            data.transfer_ownership();
        }

        success
    }

    /// Set data to the clipboard (clears the clipboard first, then adds data)
    ///
    /// # Safety
    /// This method transfers ownership of the data object to the clipboard if successful.
    /// The data object should no longer be used after a successful call.
    pub fn set_data<T: DataObject + crate::data_object::TransferOwnership>(&self, data: &mut T) -> bool {
        if self.ptr.is_null() {
            return false;
        }

        let data_ptr = data.as_data_object_ptr();
        let success = unsafe { ffi::wxd_Clipboard_SetData(self.ptr, data_ptr) };

        if success {
            // In wxWidgets, wxClipboard::SetData takes ownership of the data object
            // So we mark the Rust object as having transferred ownership
            data.transfer_ownership();
        }

        success
    }

    /// Check if the clipboard supports a specific format
    pub fn is_format_supported(&self, format: i32) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Clipboard_IsSupported(self.ptr, format) }
    }

    /// Get data from the clipboard
    pub fn get_data<T: DataObject>(&self, data: &T) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Clipboard_GetData(self.ptr, data.as_data_object_ptr()) }
    }

    /// Clear the clipboard
    pub fn clear(&self) {
        if self.ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Clipboard_Clear(self.ptr) }
    }

    /// Flush the clipboard - makes data available after application exits
    pub fn flush(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Clipboard_Flush(self.ptr) }
    }

    /// Use primary selection (X11 systems only)
    pub fn use_primary_selection(&self, use_primary: bool) {
        if self.ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Clipboard_UsePrimarySelection(self.ptr, use_primary) }
    }

    /// Returns whether the clipboard is using primary selection
    pub fn is_using_primary_selection(&self) -> bool {
        false // Not accessible through the C API, default to false
    }

    // Convenience methods

    /// Set text to the clipboard (convenience function)
    pub fn set_text(&self, text: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }

        // Create a CString, handling null bytes gracefully
        let c_text = match CString::new(text) {
            Ok(s) => s,
            Err(_) => {
                // If text contains null bytes, create a copy without them
                let filtered: String = text.chars().filter(|&c| c != '\0').collect();
                CString::new(filtered).unwrap_or_else(|_| CString::new("").unwrap())
            }
        };

        unsafe { ffi::wxd_Clipboard_SetText(self.ptr, c_text.as_ptr()) }
    }

    /// Get text from the clipboard (convenience function)
    pub fn get_text(&self) -> Option<String> {
        if self.ptr.is_null() {
            return None;
        }

        let len = unsafe { ffi::wxd_Clipboard_GetText(self.ptr, std::ptr::null_mut(), 0) };
        if len < 0 {
            return None;
        }
        let mut buf: Vec<c_char> = vec![0; len as usize + 1];
        unsafe { ffi::wxd_Clipboard_GetText(self.ptr, buf.as_mut_ptr(), buf.len()) };
        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
    }

    /// Create a ClipboardLocker to safely manage clipboard access
    pub fn locker(&self) -> Option<ClipboardLocker<'_>> {
        ClipboardLocker::new(self)
    }
}

/// Safely wraps a clipboard with RAII for open/close operations
pub struct ClipboardLocker<'a> {
    clipboard: &'a Clipboard,
}

impl<'a> ClipboardLocker<'a> {
    /// Creates a new ClipboardLocker for the given clipboard
    pub fn new(clipboard: &'a Clipboard) -> Option<Self> {
        if clipboard.open() { Some(Self { clipboard }) } else { None }
    }

    /// Returns true if the clipboard was successfully opened
    pub fn is_valid(&self) -> bool {
        self.clipboard.is_opened()
    }

    /// Gets the reference to the wrapped clipboard
    pub fn clipboard(&self) -> &Clipboard {
        self.clipboard
    }
}

impl<'a> Drop for ClipboardLocker<'a> {
    fn drop(&mut self) {
        self.clipboard.close();
    }
}

// No need for Drop implementation since we're not allocating resources
// that need to be cleaned up when the Clipboard instance is dropped.
// The clipboard itself is a global resource.
