use crate::dialogs::Dialog;
use crate::geometry::{Point, Size};
use crate::utils::ArrayString;
use crate::window::WxWidget;
use std::ffi::CString;
use wxdragon_sys as ffi;

// Define style enum using the macro
crate::widget_style_enum!(
    name: MultiChoiceDialogStyle,
    doc: "Style flags for multi choice dialog.",
    variants: {
        Default: ffi::WXD_OK | ffi::WXD_CANCEL | ffi::WXD_CENTRE, "Default style with OK, Cancel buttons and centered dialog.",
        Ok: ffi::WXD_OK, "Style flag for OK button.",
        Cancel: ffi::WXD_CANCEL, "Style flag for Cancel button.",
        Centre: ffi::WXD_CENTRE, "Style flag to center the dialog."
    },
    default_variant: Default
);

// Opaque C pointer type
pub type MultiChoiceDialogPtr = *mut ffi::wxd_MultiChoiceDialog_t;

// --- MultiChoiceDialog ---
#[derive(Clone)]
pub struct MultiChoiceDialog {
    dialog_base: Dialog,
}

impl MultiChoiceDialog {
    /// Creates a new builder for a MultiChoiceDialog.
    pub fn builder<'a>(
        parent: &'a dyn WxWidget,
        message: &str,
        caption: &str,
        choices: &[&'a str],
    ) -> MultiChoiceDialogBuilder<'a> {
        MultiChoiceDialogBuilder::new(parent, message, caption, choices)
    }

    /// Creates a new MultiChoiceDialog wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxMultiChoiceDialog.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_MultiChoiceDialog_t) -> Self {
        MultiChoiceDialog {
            dialog_base: unsafe { Dialog::from_ptr(ptr as *mut ffi::wxd_Dialog_t) },
        }
    }

    fn as_ptr(&self) -> MultiChoiceDialogPtr {
        self.dialog_base.as_ptr() as MultiChoiceDialogPtr
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL).
    pub fn show_modal(&self) -> i32 {
        self.dialog_base.show_modal()
    }

    /// Gets the indices of the selections made by the user.
    /// Returns an empty vector if no selections were made or the dialog was cancelled.
    pub fn get_selections(&self) -> Vec<i32> {
        // Allocate a buffer for up to 1000 selections (quite generous)
        let mut selections = vec![0i32; 1000];
        let mut count = 0i32;

        unsafe { ffi::wxd_MultiChoiceDialog_GetSelections(self.as_ptr(), selections.as_mut_ptr(), &mut count) };

        if count <= 0 {
            return Vec::new();
        }

        // Resize to actual number of selections
        selections.truncate(count as usize);
        selections
    }

    /// Sets the selected indices.
    pub fn set_selections(&self, selections: &[i32]) {
        if selections.is_empty() {
            return;
        }

        unsafe { ffi::wxd_MultiChoiceDialog_SetSelections(self.as_ptr(), selections.as_ptr(), selections.len() as i32) };
    }

    /// Gets the strings of the selections made by the user.
    /// Returns an empty vector if no selections were made or the dialog was cancelled.
    pub fn get_string_selections(&self) -> Vec<String> {
        let mut selections = ArrayString::new();

        unsafe { ffi::wxd_MultiChoiceDialog_GetStringSelections(self.as_ptr(), selections.as_mut_ptr()) };

        selections.into()
    }
}

// Implement WxWidget by delegating to the inner Dialog
impl WxWidget for MultiChoiceDialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.dialog_base.handle_ptr()
    }
}

// Implement Drop
impl Drop for MultiChoiceDialog {
    fn drop(&mut self) {
        // The Dialog's drop will be called automatically
    }
}

// --- MultiChoiceDialogBuilder ---
pub struct MultiChoiceDialogBuilder<'a> {
    parent: &'a dyn WxWidget,
    message: String,
    caption: String,
    choices: Vec<&'a str>,
    style: MultiChoiceDialogStyle,
    pos: Point,
    size: Size,
}

impl<'a> MultiChoiceDialogBuilder<'a> {
    pub fn new(parent: &'a dyn WxWidget, message: &str, caption: &str, choices: &[&'a str]) -> Self {
        MultiChoiceDialogBuilder {
            parent,
            message: message.to_string(),
            caption: caption.to_string(),
            choices: choices.to_vec(),
            style: MultiChoiceDialogStyle::Default,
            pos: Point::DEFAULT_POSITION,
            size: Size::DEFAULT_SIZE,
        }
    }

    pub fn with_style(mut self, style: MultiChoiceDialogStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn build(self) -> MultiChoiceDialog {
        let c_message = CString::new(self.message).expect("CString::new failed for message");
        let c_caption = CString::new(self.caption).expect("CString::new failed for caption");
        let parent_ptr = self.parent.handle_ptr();
        assert!(
            !parent_ptr.is_null(),
            "MultiChoiceDialog requires a valid parent window pointer."
        );

        // Convert the choices to a ArrayString
        let choices_array = ArrayString::from(&self.choices[..]);

        let ptr = unsafe {
            ffi::wxd_MultiChoiceDialog_Create(
                parent_ptr,
                c_message.as_ptr(),
                c_caption.as_ptr(),
                choices_array.as_const_ptr(),
                self.style.bits() as ffi::wxd_Style_t,
                self.pos.x,
                self.pos.y,
                self.size.width,
                self.size.height,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxMultiChoiceDialog");
        }
        unsafe { MultiChoiceDialog::from_ptr(ptr) }
    }
}
