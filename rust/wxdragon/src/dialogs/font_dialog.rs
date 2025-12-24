use std::ffi::CString;

use crate::{dialogs::Dialog, font::Font, font_data::FontData, window::WxWidget};
use wxdragon_sys as ffi;

/// Wrapper for wxFontDialog.
/// A dialog for choosing a font.
#[derive(Clone, Copy)]
pub struct FontDialog {
    dialog_base: Dialog,
}

/// Builder for FontDialog
pub struct FontDialogBuilder<'a, W: WxWidget> {
    parent: &'a W,
    title: String,
    font_data: Option<&'a FontData>,
}

impl FontDialog {
    /// Creates a builder for a font dialog.
    pub fn builder<'a, W: WxWidget>(parent: &'a W) -> FontDialogBuilder<'a, W> {
        FontDialogBuilder {
            parent,
            title: "Choose a font".to_string(),
            font_data: None,
        }
    }

    /// Creates a new FontDialog wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxFontDialog.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_FontDialog_t) -> Self {
        FontDialog {
            dialog_base: unsafe { Dialog::from_ptr(ptr as *mut ffi::wxd_Dialog_t) },
        }
    }

    fn as_ptr(&self) -> *mut ffi::wxd_FontDialog_t {
        self.dialog_base.as_ptr() as *mut ffi::wxd_FontDialog_t
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL). If the dialog was accepted (ID_OK), you can call
    /// get_font() to retrieve the selected font.
    pub fn show_modal(&self) -> i32 {
        self.dialog_base.show_modal()
    }

    /// Get the selected font from the dialog.
    /// This will create a new Font object that you own.
    /// Returns None if the font is not available or invalid.
    pub fn get_font(&self) -> Option<Font> {
        let font_ptr = unsafe { ffi::wxd_FontDialog_GetFont(self.as_ptr()) };
        if font_ptr.is_null() {
            None
        } else {
            // Create a Font wrapper that takes ownership of the pointer
            Some(unsafe { Font::from_ptr(font_ptr, true) })
        }
    }

    /// Get the font data from the dialog.
    /// Note: This returns a reference to the internal font data, which is only
    /// valid as long as the dialog exists.
    pub fn get_font_data(&self) -> Option<FontData> {
        let data_ptr = unsafe { ffi::wxd_FontDialog_GetFontData(self.as_ptr()) };
        if data_ptr.is_null() {
            None
        } else {
            // Create a FontData that doesn't take ownership
            Some(FontData { ptr: data_ptr })
        }
    }
}

impl<'a, W: WxWidget> FontDialogBuilder<'a, W> {
    /// Set the dialog title
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the font data to use
    pub fn with_font_data(mut self, font_data: &'a FontData) -> Self {
        self.font_data = Some(font_data);
        self
    }

    /// Build the FontDialog
    pub fn build(self) -> FontDialog {
        let c_title = CString::new(self.title).expect("CString::new failed for title");
        let font_data_ptr = self.font_data.map_or(std::ptr::null_mut(), |d| d.as_ptr());
        let parent_ptr = self.parent.handle_ptr();
        assert!(!parent_ptr.is_null(), "FontDialog requires a valid parent window pointer.");

        let ptr = unsafe { ffi::wxd_FontDialog_Create(parent_ptr, c_title.as_ptr(), font_data_ptr) };

        if ptr.is_null() {
            panic!("Failed to create wxFontDialog");
        }

        unsafe { FontDialog::from_ptr(ptr) }
    }
}

impl WxWidget for FontDialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.dialog_base.handle_ptr()
    }
}
