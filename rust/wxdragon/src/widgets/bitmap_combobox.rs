//! Safe wrapper for wxBitmapComboBox.

use crate::bitmap::Bitmap;
use crate::event::EventType;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::widgets::combobox::ComboBoxStyle;
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::ptr;
use wxdragon_sys as ffi;

/// Represents a wxBitmapComboBox widget.
#[derive(Clone, Copy)]
pub struct BitmapComboBox {
    window: Window,
}

impl BitmapComboBox {
    /// Creates a new `BitmapComboBoxBuilder`.
    pub fn builder(parent: &dyn WxWidget) -> BitmapComboBoxBuilder<'_> {
        BitmapComboBoxBuilder::new(parent)
    }

    /// Creates a `BitmapComboBox` from a raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is valid and represents a `wxBitmapComboBox`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_BitmapComboBox_t) -> Self {
        BitmapComboBox {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
        }
    }

    /// Low-level constructor used by the builder.
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, value: &str, pos: Point, size: Size, style: i64) -> Self {
        assert!(!parent_ptr.is_null(), "BitmapComboBox requires a parent");
        let c_value = CString::new(value).expect("CString::new failed for value");

        let ptr = unsafe {
            ffi::wxd_BitmapComboBox_Create(
                parent_ptr,
                id,
                c_value.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxBitmapComboBox");
        }
        unsafe { BitmapComboBox::from_ptr(ptr) }
    }

    /// Appends an item with an optional bitmap.
    pub fn append(&self, item: &str, bitmap: Option<&Bitmap>) {
        let c_item = CString::new(item).expect("CString::new failed for item");
        let bmp_ptr = bitmap.map_or(ptr::null(), |b| b.as_const_ptr());
        unsafe { ffi::wxd_BitmapComboBox_Append(self.as_ptr(), c_item.as_ptr(), bmp_ptr) };
    }

    /// Removes all items from the control.
    pub fn clear(&self) {
        unsafe { ffi::wxd_BitmapComboBox_Clear(self.as_ptr()) };
    }

    /// Gets the index of the currently selected item or -1 if none.
    pub fn get_selection(&self) -> i32 {
        unsafe { ffi::wxd_BitmapComboBox_GetSelection(self.as_ptr()) }
    }

    /// Sets the selection to the given item index.
    pub fn set_selection(&self, index: i32) {
        unsafe { ffi::wxd_BitmapComboBox_SetSelection(self.as_ptr(), index) };
    }

    /// Gets the string at the specified index.
    pub fn get_string(&self, index: u32) -> String {
        let len = unsafe { ffi::wxd_BitmapComboBox_GetString(self.as_ptr(), index as i32, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_BitmapComboBox_GetString(self.as_ptr(), index as i32, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() }
    }

    /// Gets the number of items in the control.
    pub fn get_count(&self) -> u32 {
        unsafe { ffi::wxd_BitmapComboBox_GetCount(self.as_ptr()) }
    }

    /// Sets the text value in the text entry part of the control.
    pub fn set_value(&self, value: &str) {
        let c_value = CString::new(value).expect("CString::new failed for value");
        unsafe { ffi::wxd_BitmapComboBox_SetValue(self.as_ptr(), c_value.as_ptr()) };
    }

    /// Gets the text from the text entry part of the control.
    pub fn get_value(&self) -> String {
        let len = unsafe { ffi::wxd_BitmapComboBox_GetValue(self.as_ptr(), std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_BitmapComboBox_GetValue(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() }
    }

    /// Gets the bitmap associated with the item at the specified index.
    /// Returns `None` if the index is invalid or the item has no bitmap.
    pub fn get_item_bitmap(&self, n: u32) -> Option<Bitmap> {
        let bmp_ptr = unsafe { ffi::wxd_BitmapComboBox_GetItemBitmap(self.as_ptr(), n) };
        if bmp_ptr.is_null() {
            None
        } else {
            // The C++ side created a `new wxBitmap`. We take ownership.
            Some(Bitmap::from(bmp_ptr))
        }
    }

    /// Sets the bitmap for the item at the specified index.
    pub fn set_item_bitmap(&self, n: u32, bitmap: &Bitmap) {
        unsafe { ffi::wxd_BitmapComboBox_SetItemBitmap(self.as_ptr(), n, bitmap.as_const_ptr()) };
    }

    /// Returns the raw wxBitmapComboBox pointer.
    fn as_ptr(&self) -> *mut ffi::wxd_BitmapComboBox_t {
        self.window.as_ptr() as *mut _
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
        BitmapComboBox::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            &slf.value,
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
);

// Apply common trait implementations for this widget
implement_widget_traits_with_target!(BitmapComboBox, window, Window);

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
impl_xrc_support!(BitmapComboBox, { window });

// Widget casting support for BitmapComboBox
impl_widget_cast!(BitmapComboBox, "wxBitmapComboBox", { window });
