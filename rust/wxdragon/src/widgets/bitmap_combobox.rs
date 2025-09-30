//! Safe wrapper for wxBitmapComboBox.

use crate::bitmap::Bitmap;
use crate::event::EventType;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widgets::combobox::ComboBoxStyle;
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use wxdragon_sys as ffi;

/// Represents a wxBitmapComboBox widget.
#[derive(Clone)]
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
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Low-level constructor used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        value: &str,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
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
        let bmp_ptr = bitmap.map_or(ptr::null_mut(), |b| b.as_ptr());
        unsafe {
            ffi::wxd_BitmapComboBox_Append(self.as_ptr(), c_item.as_ptr(), bmp_ptr);
        }
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
        unsafe {
            let required_len_p1 =
                ffi::wxd_BitmapComboBox_GetString(self.as_ptr(), index as i32, ptr::null_mut(), 0);
            if required_len_p1 <= 0 {
                return String::new();
            }
            let capacity = required_len_p1 as usize;
            let mut buffer: Vec<u8> = Vec::with_capacity(capacity);
            let success_code = ffi::wxd_BitmapComboBox_GetString(
                self.as_ptr(),
                index as i32,
                buffer.as_mut_ptr() as *mut c_char,
                capacity as i32,
            );
            if success_code == 0 {
                let c_str = CStr::from_ptr(buffer.as_ptr() as *const c_char);
                String::from_utf8_lossy(c_str.to_bytes()).into_owned()
            } else {
                String::new()
            }
        }
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
        unsafe {
            let required_len_p1 =
                ffi::wxd_BitmapComboBox_GetValue(self.as_ptr(), ptr::null_mut(), 0);
            if required_len_p1 <= 0 {
                return String::new();
            }
            let capacity = required_len_p1 as usize;
            let mut buffer: Vec<u8> = Vec::with_capacity(capacity);
            let success_code = ffi::wxd_BitmapComboBox_GetValue(
                self.as_ptr(),
                buffer.as_mut_ptr() as *mut c_char,
                capacity as i32,
            );
            if success_code == 0 {
                let c_str = CStr::from_ptr(buffer.as_ptr() as *const c_char);
                String::from_utf8_lossy(c_str.to_bytes()).into_owned()
            } else {
                String::new()
            }
        }
    }

    /// Gets the bitmap associated with the item at the specified index.
    /// Returns `None` if the index is invalid or the item has no bitmap.
    pub fn get_item_bitmap(&self, n: u32) -> Option<Bitmap> {
        let bmp_ptr = unsafe { ffi::wxd_BitmapComboBox_GetItemBitmap(self.as_ptr(), n) };
        if bmp_ptr.is_null() {
            None
        } else {
            // The C++ side created a `new wxBitmap`. We take ownership.
            Some(Bitmap::from_ptr_owned(bmp_ptr))
        }
    }

    /// Sets the bitmap for the item at the specified index.
    pub fn set_item_bitmap(&self, n: u32, bitmap: &Bitmap) {
        unsafe { ffi::wxd_BitmapComboBox_SetItemBitmap(self.as_ptr(), n, bitmap.as_ptr()) };
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
