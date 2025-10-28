use crate::dialogs::Dialog;
use crate::geometry::{DEFAULT_POSITION, DEFAULT_SIZE, Point, Size};
use crate::utils::WxdArrayString;
use crate::window::WxWidget;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// Define style enum using the macro
crate::widget_style_enum!(
    name: SingleChoiceDialogStyle,
    doc: "Style flags for single choice dialog.",
    variants: {
        Default: ffi::WXD_OK | ffi::WXD_CANCEL | ffi::WXD_CENTRE, "Default style with OK, Cancel buttons and centered dialog.",
        Ok: ffi::WXD_OK, "Style flag for OK button.",
        Cancel: ffi::WXD_CANCEL, "Style flag for Cancel button.",
        Centre: ffi::WXD_CENTRE, "Style flag to center the dialog."
    },
    default_variant: Default
);

// Opaque C pointer type
pub type SingleChoiceDialogPtr = *mut ffi::wxd_SingleChoiceDialog_t;

// --- SingleChoiceDialog ---
#[derive(Clone)]
pub struct SingleChoiceDialog {
    dialog_base: Dialog,
}

impl SingleChoiceDialog {
    /// Creates a new builder for a SingleChoiceDialog.
    pub fn builder<'a>(
        parent: &'a dyn WxWidget,
        message: &str,
        caption: &str,
        choices: &[&'a str],
    ) -> SingleChoiceDialogBuilder<'a> {
        SingleChoiceDialogBuilder::new(parent, message, caption, choices)
    }

    /// Creates a new SingleChoiceDialog wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxSingleChoiceDialog.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_SingleChoiceDialog_t) -> Self {
        SingleChoiceDialog {
            dialog_base: unsafe { Dialog::from_ptr(ptr as *mut ffi::wxd_Dialog_t) },
        }
    }

    fn as_ptr(&self) -> SingleChoiceDialogPtr {
        self.dialog_base.as_ptr() as SingleChoiceDialogPtr
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL).
    pub fn show_modal(&self) -> i32 {
        self.dialog_base.show_modal()
    }

    /// Gets the index of the selection made by the user.
    /// Returns -1 if no selection was made or the dialog was cancelled.
    pub fn get_selection(&self) -> i32 {
        unsafe { ffi::wxd_SingleChoiceDialog_GetSelection(self.as_ptr()) }
    }

    /// Sets the selection to the given index.
    pub fn set_selection(&self, selection: i32) {
        unsafe { ffi::wxd_SingleChoiceDialog_SetSelection(self.as_ptr(), selection) }
    }

    /// Gets the string of the selection made by the user.
    /// Returns `None` if no selection was made, the dialog was cancelled, or an error occurred.
    pub fn get_string_selection(&self) -> Option<String> {
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024]; // Reasonable buffer size
            let len_needed = ffi::wxd_SingleChoiceDialog_GetStringSelection(
                self.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            );

            if len_needed < 0 {
                return None; // Error or dialog cancelled
            }

            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                // Allocate exact size if needed
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_SingleChoiceDialog_GetStringSelection(
                    self.as_ptr(),
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop();
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None // Error on second call
                }
            }
        }
    }
}

// Implement WxWidget by delegating to the inner Dialog
impl WxWidget for SingleChoiceDialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.dialog_base.handle_ptr()
    }
}

// Implement Drop
impl Drop for SingleChoiceDialog {
    fn drop(&mut self) {
        // The Dialog's drop will be called automatically
    }
}

// --- SingleChoiceDialogBuilder ---
pub struct SingleChoiceDialogBuilder<'a> {
    parent: &'a dyn WxWidget,
    message: String,
    caption: String,
    choices: Vec<&'a str>,
    style: SingleChoiceDialogStyle,
    pos: Point,
    size: Size,
}

impl<'a> SingleChoiceDialogBuilder<'a> {
    pub fn new(
        parent: &'a dyn WxWidget,
        message: &str,
        caption: &str,
        choices: &[&'a str],
    ) -> Self {
        SingleChoiceDialogBuilder {
            parent,
            message: message.to_string(),
            caption: caption.to_string(),
            choices: choices.to_vec(),
            style: SingleChoiceDialogStyle::Default,
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
        }
    }

    pub fn with_style(mut self, style: SingleChoiceDialogStyle) -> Self {
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

    pub fn build(self) -> SingleChoiceDialog {
        let c_message = CString::new(self.message).expect("CString::new failed for message");
        let c_caption = CString::new(self.caption).expect("CString::new failed for caption");
        let parent_ptr = self.parent.handle_ptr();
        assert!(
            !parent_ptr.is_null(),
            "SingleChoiceDialog requires a valid parent window pointer."
        );

        // Convert the choices to a wxdArrayString
        let choices_array = WxdArrayString::from(&self.choices[..]);

        let ptr = unsafe {
            ffi::wxd_SingleChoiceDialog_Create(
                parent_ptr,
                c_message.as_ptr(),
                c_caption.as_ptr(),
                choices_array.as_ptr(),
                self.style.bits() as ffi::wxd_Style_t,
                self.pos.x,
                self.pos.y,
                self.size.width,
                self.size.height,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxSingleChoiceDialog");
        }
        unsafe { SingleChoiceDialog::from_ptr(ptr) }
    }
}
