use crate::dialogs::Dialog;
use crate::geometry::{Point, Size};
use crate::window::WxWidget;
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

// Define style enum using the macro
crate::widget_style_enum!(
    name: TextEntryDialogStyle,
    doc: "Style flags for text entry dialog.",
    variants: {
        Default: ffi::WXD_OK | ffi::WXD_CANCEL | ffi::WXD_CENTRE, "Default style with OK, Cancel buttons and centered dialog.",
        Ok: ffi::WXD_OK, "Style flag for OK button.",
        Cancel: ffi::WXD_CANCEL, "Style flag for Cancel button.",
        Centre: ffi::WXD_CENTRE, "Style flag to center the dialog.",
        Password: ffi::WXD_TE_PASSWORD, "Style flag for password text entry.",
        ProcessEnter: ffi::WXD_TE_PROCESS_ENTER, "Style flag to process Enter key in the text control."
    },
    default_variant: Default
);

// Opaque C pointer type
pub type TextEntryDialogPtr = *mut ffi::wxd_TextEntryDialog_t;

// --- TextEntryDialog ---
#[derive(Clone)]
pub struct TextEntryDialog {
    dialog_base: Dialog,
}

impl TextEntryDialog {
    /// Creates a new builder for a TextEntryDialog.
    pub fn builder<'a>(parent: &'a dyn WxWidget, message: &str, caption: &str) -> TextEntryDialogBuilder<'a> {
        TextEntryDialogBuilder::new(parent, message, caption)
    }

    /// Creates a new TextEntryDialog wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxTextEntryDialog.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_TextEntryDialog_t) -> Self {
        TextEntryDialog {
            dialog_base: unsafe { Dialog::from_ptr(ptr as *mut ffi::wxd_Dialog_t) },
        }
    }

    fn as_ptr(&self) -> TextEntryDialogPtr {
        self.dialog_base.as_ptr() as TextEntryDialogPtr
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL).
    pub fn show_modal(&self) -> i32 {
        self.dialog_base.show_modal()
    }

    /// Gets the text entered by the user.
    /// Returns `None` if the dialog was cancelled or an error occurred retrieving the value.
    pub fn get_value(&self) -> Option<String> {
        let mut buffer = [0; 1024]; // Reasonable buffer size
        let len = unsafe { ffi::wxd_TextEntryDialog_GetValue(self.as_ptr(), buffer.as_mut_ptr(), buffer.len()) };

        if len < 0 {
            return None; // Error or dialog cancelled before value retrieved?
        }

        if len < buffer.len() as i32 {
            Some(unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned() })
        } else {
            // Allocate exact size if needed
            let mut buf = vec![0; len as usize + 1];
            let len_copied = unsafe { ffi::wxd_TextEntryDialog_GetValue(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
            if len_copied == len {
                Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
            } else {
                None // Error on second call
            }
        }
    }
}

// Implement WxWidget by delegating to the inner Dialog
impl WxWidget for TextEntryDialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.dialog_base.handle_ptr()
    }
}

// Implement Drop
impl Drop for TextEntryDialog {
    fn drop(&mut self) {
        // The composed Dialog's Drop will be called automatically,
        // which calls wxd_Window_Destroy on the pointer.
    }
}

// --- TextEntryDialogBuilder ---
pub struct TextEntryDialogBuilder<'a> {
    parent: &'a dyn WxWidget,
    message: String,
    caption: String,
    default_value: String,
    style: TextEntryDialogStyle,
    pos: Point,
    size: Size, // Often unused, but kept for consistency
}

impl<'a> TextEntryDialogBuilder<'a> {
    pub fn new(parent: &'a dyn WxWidget, message: &str, caption: &str) -> Self {
        TextEntryDialogBuilder {
            parent,
            message: message.to_string(),
            caption: caption.to_string(),
            default_value: String::new(),
            style: TextEntryDialogStyle::Default,
            pos: Point::DEFAULT_POSITION,
            size: Size::DEFAULT_SIZE,
        }
    }

    pub fn with_default_value(mut self, value: &str) -> Self {
        self.default_value = value.to_string();
        self
    }

    pub fn with_style(mut self, style: TextEntryDialogStyle) -> Self {
        self.style = style;
        self
    }

    /// Convenience method to add password style flag.
    pub fn password(mut self) -> Self {
        self.style |= TextEntryDialogStyle::Password;
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

    pub fn build(self) -> TextEntryDialog {
        let c_message = CString::new(self.message).expect("CString::new failed for message");
        let c_caption = CString::new(self.caption).expect("CString::new failed for caption");
        let c_default_value = CString::new(self.default_value).expect("CString::new failed for default_value");
        let parent_ptr = self.parent.handle_ptr();
        assert!(
            !parent_ptr.is_null(),
            "TextEntryDialog requires a valid parent window pointer."
        );

        let ptr = unsafe {
            ffi::wxd_TextEntryDialog_Create(
                parent_ptr,
                c_message.as_ptr(),
                c_caption.as_ptr(),
                c_default_value.as_ptr(),
                self.style.bits() as ffi::wxd_Style_t,
                self.pos.x,
                self.pos.y,
                self.size.width,
                self.size.height,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxTextEntryDialog");
        }
        unsafe { TextEntryDialog::from_ptr(ptr) }
    }
}
