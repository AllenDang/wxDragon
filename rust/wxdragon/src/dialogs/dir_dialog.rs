use crate::geometry::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::window::WxWidget;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// Define style enum using the macro
crate::widget_style_enum!(
    name: DirDialogStyle,
    doc: "Style flags for directory dialog.",
    variants: {
        Default: ffi::WXD_DEFAULT_DIALOG_STYLE, "Default style.",
        MustExist: ffi::WXD_DD_DIR_MUST_EXIST, "The dialog will allow the user to choose only an existing directory.",
        ChangeDir: ffi::WXD_DD_CHANGE_DIR, "Change the current working directory to the directory chosen by the user."
    },
    default_variant: Default
);

// Opaque C pointer type
pub type DirDialogPtr = *mut ffi::wxd_DirDialog_t;

/// A dialog that lets the user choose a directory.
pub struct DirDialog {
    ptr: DirDialogPtr,
}

impl DirDialog {
    /// Creates a new builder for a DirDialog.
    pub fn builder<'a>(
        parent: &'a dyn WxWidget,
        message: &str,
        default_path: &str,
    ) -> DirDialogBuilder<'a> {
        DirDialogBuilder::new(parent, message, default_path)
    }

    /// Shows the dialog modally.
    pub fn show_modal(&self) -> i32 {
        unsafe { ffi::wxd_Dialog_ShowModal(self.ptr as *mut ffi::wxd_Dialog_t) }
    }

    /// Gets the path selected by the user.
    pub fn get_path(&self) -> Option<String> {
        unsafe {
            let mut buffer = vec![0 as c_char; 1024];
            let len =
                ffi::wxd_DirDialog_GetPath(self.ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            if len > 0 {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                None
            }
        }
    }

    /// Sets the path that will be selected when the dialog is shown.
    pub fn set_path(&self, path: &str) {
        unsafe {
            let c_path = CString::new(path).unwrap();
            ffi::wxd_DirDialog_SetPath(self.ptr, c_path.as_ptr());
        }
    }

    /// Gets the message shown to the user.
    pub fn get_message(&self) -> Option<String> {
        unsafe {
            let mut buffer = vec![0 as c_char; 1024];
            let len =
                ffi::wxd_DirDialog_GetMessage(self.ptr, buffer.as_mut_ptr(), buffer.len() as i32);
            if len > 0 {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                Some(c_str.to_string_lossy().into_owned())
            } else {
                None
            }
        }
    }

    /// Sets the message shown to the user.
    pub fn set_message(&self, message: &str) {
        unsafe {
            let c_message = CString::new(message).unwrap();
            ffi::wxd_DirDialog_SetMessage(self.ptr, c_message.as_ptr());
        }
    }
}

// Implement WxWidget trait
impl WxWidget for DirDialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t
    }
}

impl Drop for DirDialog {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // The dialog is destroyed automatically by wxWidgets when it's closed
            // We only need to forget about it
            self.ptr = std::ptr::null_mut();
        }
    }
}

/// Builder for DirDialog.
pub struct DirDialogBuilder<'a> {
    parent: &'a dyn WxWidget,
    message: String,
    default_path: String,
    style: i64,
    pos: Point,
    size: Size,
}

impl<'a> DirDialogBuilder<'a> {
    /// Creates a new builder with default values.
    pub fn new(parent: &'a dyn WxWidget, message: &str, default_path: &str) -> Self {
        DirDialogBuilder {
            parent,
            message: message.to_string(),
            default_path: default_path.to_string(),
            style: DirDialogStyle::default().bits(),
            pos: DEFAULT_POSITION,
            size: DEFAULT_SIZE,
        }
    }

    /// Sets the style for the dialog.
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Sets the position for the dialog.
    pub fn with_position(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the size for the dialog.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Builds the dialog.
    pub fn build(self) -> DirDialog {
        let c_message = CString::new(self.message).unwrap();
        let c_default_path = CString::new(self.default_path).unwrap();

        let ptr = unsafe {
            ffi::wxd_DirDialog_Create(
                self.parent.handle_ptr(),
                c_message.as_ptr(),
                c_default_path.as_ptr(),
                self.style,
                self.pos.x,
                self.pos.y,
                self.size.width,
                self.size.height,
            )
        };

        DirDialog { ptr }
    }
}
