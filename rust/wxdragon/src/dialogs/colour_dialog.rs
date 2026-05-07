use std::ffi::CString;

use crate::{color::Colour, dialogs::Dialog, window::WxWidget};
use wxdragon_sys as ffi;

/// Wrapper for wxColourDialog.
/// A dialog for choosing a colour.
#[derive(Clone, Copy)]
pub struct ColourDialog {
    dialog_base: Dialog,
}

/// Builder for ColourDialog
pub struct ColourDialogBuilder<'a, W: WxWidget> {
    parent: &'a W,
    title: String,
    initial_colour: Option<Colour>,
    choose_full: bool,
    custom_colours: std::collections::HashMap<i32, Colour>,
}

impl ColourDialog {
    /// Creates a builder for a colour dialog.
    pub fn builder<'a, W: WxWidget>(parent: &'a W) -> ColourDialogBuilder<'a, W> {
        ColourDialogBuilder {
            parent,
            title: "Choose a colour".to_string(),
            initial_colour: None,
            choose_full: false,
            custom_colours: std::collections::HashMap::new(),
        }
    }

    /// Creates a new ColourDialog wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxColourDialog.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_ColourDialog_t) -> Self {
        ColourDialog {
            dialog_base: unsafe { Dialog::from_ptr(ptr as *mut ffi::wxd_Dialog_t) },
        }
    }

    fn as_ptr(&self) -> *mut ffi::wxd_ColourDialog_t {
        self.dialog_base.as_ptr() as *mut ffi::wxd_ColourDialog_t
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL). If the dialog was accepted (ID_OK), you can call
    /// get_colour() to retrieve the selected colour.
    pub fn show_modal(&self) -> i32 {
        self.dialog_base.show_modal()
    }

    /// Get the selected colour from the dialog.
    /// Returns None if the colour data is not available.
    pub fn get_colour(&self) -> Option<Colour> {
        let data_ptr = unsafe { ffi::wxd_ColourDialog_GetColourData(self.as_ptr()) };
        if data_ptr.is_null() {
            None
        } else {
            // Get the colour directly from the colour data
            let colour = unsafe { ffi::wxd_ColourData_GetColour(data_ptr) };
            Some(Colour::from(colour))
        }
    }
}

impl<'a, W: WxWidget> ColourDialogBuilder<'a, W> {
    /// Set the dialog title
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the initial colour to use
    pub fn with_initial_colour(mut self, colour: Colour) -> Self {
        self.initial_colour = Some(colour);
        self
    }

    /// Under Windows, determines whether the Windows colour dialog will display
    /// the full dialog with custom colour selection controls when it is first shown.
    pub fn with_choose_full(mut self, choose_full: bool) -> Self {
        self.choose_full = choose_full;
        self
    }

    /// Sets custom colours (index from 0 to 15) to be displayed in the
    /// bottom row of custom colour swatches.
    pub fn with_custom_colour(mut self, index: i32, colour: Colour) -> Self {
        self.custom_colours.insert(index, colour);
        self
    }

    /// Build the ColourDialog
    pub fn build(self) -> ColourDialog {
        let c_title = CString::new(self.title).expect("CString::new failed for title");

        // Create a temporary ColourData if we have an initial colour, choose_full flag or custom colours
        let has_custom_data = self.initial_colour.is_some() || self.choose_full || !self.custom_colours.is_empty();

        let colour_data_ptr = if has_custom_data {
            let data_ptr = unsafe { ffi::wxd_ColourData_Create() };
            if !data_ptr.is_null() {
                if let Some(colour) = self.initial_colour {
                    unsafe { ffi::wxd_ColourData_SetColour(data_ptr, colour.into()) };
                }

                if self.choose_full {
                    unsafe { ffi::wxd_ColourData_SetChooseFull(data_ptr, true) };
                }

                for (index, colour) in self.custom_colours {
                    unsafe { ffi::wxd_ColourData_SetCustomColour(data_ptr, index, colour.into()) };
                }
            }
            data_ptr
        } else {
            std::ptr::null_mut()
        };

        let parent_ptr = self.parent.handle_ptr();
        assert!(!parent_ptr.is_null(), "ColourDialog requires a valid parent window pointer.");

        let ptr = unsafe { ffi::wxd_ColourDialog_Create(parent_ptr, c_title.as_ptr(), colour_data_ptr) };

        // Clean up the temporary ColourData if we created one
        if !colour_data_ptr.is_null() {
            unsafe { ffi::wxd_ColourData_Destroy(colour_data_ptr) };
        }

        if ptr.is_null() {
            panic!("Failed to create wxColourDialog");
        }

        unsafe { ColourDialog::from_ptr(ptr) }
    }
}

impl WxWidget for ColourDialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.dialog_base.handle_ptr()
    }
}
