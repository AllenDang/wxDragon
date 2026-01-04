//! Safe wrapper for wxBitmapComboBox.

use crate::bitmap::Bitmap;
use crate::event::{EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::widgets::combobox::ComboBoxStyle;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::{CStr, CString};
use std::ptr;
use wxdragon_sys as ffi;

/// Represents a wxBitmapComboBox widget.
///
/// BitmapComboBox uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct BitmapComboBox {
    handle: WindowHandle,
}

impl BitmapComboBox {
    /// Creates a new `BitmapComboBoxBuilder`.
    pub fn builder(parent: &dyn WxWidget) -> BitmapComboBoxBuilder<'_> {
        BitmapComboBoxBuilder::new(parent)
    }

    /// Helper to get raw bitmap combobox pointer, returns null if widget has been destroyed
    #[inline]
    fn bitmap_combobox_ptr(&self) -> *mut ffi::wxd_BitmapComboBox_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_BitmapComboBox_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Appends an item with an optional bitmap.
    /// No-op if the bitmap combobox has been destroyed.
    pub fn append(&self, item: &str, bitmap: Option<&Bitmap>) {
        let ptr = self.bitmap_combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let c_item = CString::new(item).expect("CString::new failed for item");
        let bmp_ptr = bitmap.map_or(ptr::null(), |b| b.as_const_ptr());
        unsafe { ffi::wxd_BitmapComboBox_Append(ptr, c_item.as_ptr(), bmp_ptr) };
    }

    /// Removes all items from the control.
    /// No-op if the bitmap combobox has been destroyed.
    pub fn clear(&self) {
        let ptr = self.bitmap_combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_BitmapComboBox_Clear(ptr) };
    }

    /// Gets the index of the currently selected item or -1 if none.
    /// Returns -1 if the bitmap combobox has been destroyed.
    pub fn get_selection(&self) -> i32 {
        let ptr = self.bitmap_combobox_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_BitmapComboBox_GetSelection(ptr) }
    }

    /// Sets the selection to the given item index.
    /// No-op if the bitmap combobox has been destroyed.
    pub fn set_selection(&self, index: i32) {
        let ptr = self.bitmap_combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_BitmapComboBox_SetSelection(ptr, index) };
    }

    /// Gets the string at the specified index.
    /// Returns empty string if the bitmap combobox has been destroyed or index is invalid.
    pub fn get_string(&self, index: u32) -> String {
        let ptr = self.bitmap_combobox_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_BitmapComboBox_GetString(ptr, index as i32, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_BitmapComboBox_GetString(ptr, index as i32, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() }
    }

    /// Gets the number of items in the control.
    /// Returns 0 if the bitmap combobox has been destroyed.
    pub fn get_count(&self) -> u32 {
        let ptr = self.bitmap_combobox_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_BitmapComboBox_GetCount(ptr) }
    }

    /// Sets the text value in the text entry part of the control.
    /// No-op if the bitmap combobox has been destroyed.
    pub fn set_value(&self, value: &str) {
        let ptr = self.bitmap_combobox_ptr();
        if ptr.is_null() {
            return;
        }
        let c_value = CString::new(value).expect("CString::new failed for value");
        unsafe { ffi::wxd_BitmapComboBox_SetValue(ptr, c_value.as_ptr()) };
    }

    /// Gets the text from the text entry part of the control.
    /// Returns empty string if the bitmap combobox has been destroyed.
    pub fn get_value(&self) -> String {
        let ptr = self.bitmap_combobox_ptr();
        if ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_BitmapComboBox_GetValue(ptr, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_BitmapComboBox_GetValue(ptr, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() }
    }

    /// Gets the bitmap associated with the item at the specified index.
    /// Returns `None` if the index is invalid, the item has no bitmap, or the bitmap combobox has been destroyed.
    pub fn get_item_bitmap(&self, n: u32) -> Option<Bitmap> {
        let ptr = self.bitmap_combobox_ptr();
        if ptr.is_null() {
            return None;
        }
        let bmp_ptr = unsafe { ffi::wxd_BitmapComboBox_GetItemBitmap(ptr, n) };
        if bmp_ptr.is_null() {
            None
        } else {
            // The C++ side created a `new wxBitmap`. We take ownership.
            Some(Bitmap::from(bmp_ptr))
        }
    }

    /// Sets the bitmap for the item at the specified index.
    /// No-op if the bitmap combobox has been destroyed.
    pub fn set_item_bitmap(&self, n: u32, bitmap: &Bitmap) {
        let ptr = self.bitmap_combobox_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_BitmapComboBox_SetItemBitmap(ptr, n, bitmap.as_const_ptr()) };
    }
}

// Use the widget_builder macro for BitmapComboBox
widget_builder!(
    name: BitmapComboBox,
    parent_type: &'a dyn WxWidget,
    style_type: ComboBoxStyle,
    fields: {
        value: String = String::new()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        assert!(!parent_ptr.is_null(), "BitmapComboBox requires a parent");

        let c_value = CString::new(slf.value.as_str()).expect("Invalid CString for BitmapComboBox value");

        unsafe {
            let ptr = ffi::wxd_BitmapComboBox_Create(
                parent_ptr,
                slf.id,
                c_value.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            );

            if ptr.is_null() {
                panic!("Failed to create BitmapComboBox widget");
            }

            BitmapComboBox {
                handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t)
            }
        }
    }
);

// Manual WxWidget implementation for BitmapComboBox (using WindowHandle)
impl WxWidget for BitmapComboBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for BitmapComboBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for BitmapComboBox {}

// Implement the ComboBox events for BitmapComboBox
use crate::implement_widget_local_event_handlers;
use crate::widgets::combobox::{ComboBoxEvent, ComboBoxEventData};

// Implement the event handlers for BitmapComboBox
implement_widget_local_event_handlers!(
    BitmapComboBox,
    ComboBoxEvent,
    ComboBoxEventData,
    Selected => selection_changed, EventType::COMMAND_COMBOBOX_SELECTED
);

// Also implement TextEvents for text entry capabilities
use crate::event::TextEvents;
impl TextEvents for BitmapComboBox {}

// Add XRC Support - enables BitmapComboBox to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for BitmapComboBox {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        BitmapComboBox {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Widget casting support for BitmapComboBox
impl crate::window::FromWindowWithClassName for BitmapComboBox {
    fn class_name() -> &'static str {
        "wxBitmapComboBox"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        BitmapComboBox {
            handle: WindowHandle::new(ptr),
        }
    }
}
