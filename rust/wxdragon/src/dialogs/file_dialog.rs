use crate::dialogs::Dialog;
use crate::geometry::{Point, Size};
use crate::utils::ArrayString;
use crate::window::WxWidget;
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

// Define FileDialogStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: FileDialogStyle,
    doc: "Style flags for FileDialog.",
    variants: {
        Open: ffi::WXD_FD_OPEN, "Creates an open file dialog (cannot be combined with Save).",
        Save: ffi::WXD_FD_SAVE, "Creates a save file dialog (cannot be combined with Open).",
        OverwritePrompt: ffi::WXD_FD_OVERWRITE_PROMPT, "For save dialog only: prompt for a confirmation if a file with the same name already exists.",
        FileMustExist: ffi::WXD_FD_FILE_MUST_EXIST, "For open dialog only: the user may only select files that actually exist.",
        Multiple: ffi::WXD_FD_MULTIPLE, "For open dialog only: allows selecting multiple files.",
        ChangeDir: ffi::WXD_FD_CHANGE_DIR, "Change the current working directory to the directory where the file(s) chosen by the user are.",
        Preview: ffi::WXD_FD_PREVIEW, "Show the preview of the selected files (currently only supported by wxGTK)."
    },
    default_variant: Open
);

// Opaque C pointer type
pub type FileDialogPtr = *mut ffi::wxd_FileDialog_t;

// --- FileDialog ---
#[derive(Clone)] // Cloning FileDialog clones the underlying Dialog pointer
pub struct FileDialog {
    dialog_base: Dialog,
}

impl FileDialog {
    /// Creates a new builder for a FileDialog.
    pub fn builder<'a>(parent: &'a dyn WxWidget) -> FileDialogBuilder<'a> {
        FileDialogBuilder::new(parent)
    }

    /// Creates a new FileDialog wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxFileDialog.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_FileDialog_t) -> Self {
        FileDialog {
            dialog_base: unsafe { Dialog::from_ptr(ptr as *mut ffi::wxd_Dialog_t) },
        }
    }

    fn as_ptr(&self) -> FileDialogPtr {
        self.dialog_base.as_ptr() as FileDialogPtr
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL).
    pub fn show_modal(&self) -> i32 {
        self.dialog_base.show_modal()
    }

    /// Gets the full path of the selected file.
    /// Returns `None` if the dialog was cancelled or an error occurred.
    pub fn get_path(&self) -> Option<String> {
        let mut buffer = [0; 2048]; // Larger buffer for paths
        let len = unsafe { ffi::wxd_FileDialog_GetPath(self.as_ptr(), buffer.as_mut_ptr(), buffer.len()) };

        if len < 0 {
            return None;
        }

        if len < buffer.len() as i32 {
            return Some(unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().to_string() });
        }
        let mut buf = vec![0; len as usize + 1];
        let len_copied = unsafe { ffi::wxd_FileDialog_GetPath(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
        if len_copied == len {
            Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
        } else {
            None
        }
    }

    /// Gets the full paths of the selected files (for multi-select dialogs).
    pub fn get_paths(&self) -> Vec<String> {
        let mut arr_str = ArrayString::new();
        unsafe { ffi::wxd_FileDialog_GetPaths(self.as_ptr(), arr_str.as_mut_ptr()) };
        arr_str.into()
    }

    /// Gets the filename part of the selected file.
    /// Returns `None` if the dialog was cancelled or an error occurred.
    pub fn get_filename(&self) -> Option<String> {
        let mut buffer = [0; 1024];
        let len = unsafe { ffi::wxd_FileDialog_GetFilename(self.as_ptr(), buffer.as_mut_ptr(), buffer.len()) };

        if len < 0 {
            return None;
        }

        if len < buffer.len() as i32 {
            return Some(unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().to_string() });
        }
        let mut buf = vec![0; len as usize + 1];
        let len2 = unsafe { ffi::wxd_FileDialog_GetFilename(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
        if len2 == len {
            Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
        } else {
            None
        }
    }

    /// Gets the filenames of the selected files (for multi-select dialogs).
    pub fn get_filenames(&self) -> Vec<String> {
        let mut arr_str = ArrayString::new();
        unsafe { ffi::wxd_FileDialog_GetFilenames(self.as_ptr(), arr_str.as_mut_ptr()) };
        arr_str.into()
    }

    /// Gets the directory part of the selected path.
    /// Returns `None` if the dialog was cancelled or an error occurred.
    pub fn get_directory(&self) -> Option<String> {
        let mut buffer = [0; 2048];
        let len = unsafe { ffi::wxd_FileDialog_GetDirectory(self.as_ptr(), buffer.as_mut_ptr(), buffer.len()) };

        if len < 0 {
            return None;
        }

        if len < buffer.len() as i32 {
            Some(unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().to_string() })
        } else {
            let mut buf = vec![0; len as usize + 1];
            let len2 = unsafe { ffi::wxd_FileDialog_GetDirectory(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
            if len2 == len {
                Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
            } else {
                None
            }
        }
    }

    /// Gets the index of the filter currently selected.
    pub fn get_filter_index(&self) -> i32 {
        unsafe { ffi::wxd_FileDialog_GetFilterIndex(self.as_ptr()) }
    }

    /// Gets the message that will be displayed on the dialog.
    pub fn get_message(&self) -> Option<String> {
        let mut buffer = [0; 1024];
        let len = unsafe { ffi::wxd_FileDialog_GetMessage(self.as_ptr(), buffer.as_mut_ptr(), buffer.len()) };

        if len < 0 {
            return None;
        }

        if len < buffer.len() as i32 {
            return Some(unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned() });
        }
        let mut buf = vec![0; len as usize + 1];
        let len2 = unsafe { ffi::wxd_FileDialog_GetMessage(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
        if len2 == len {
            Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() })
        } else {
            None
        }
    }

    /// Gets the wildcard filter string.
    pub fn get_wildcard(&self) -> Option<String> {
        let mut buffer = [0; 1024];
        let len = unsafe { ffi::wxd_FileDialog_GetWildcard(self.as_ptr(), buffer.as_mut_ptr(), buffer.len()) };

        if len < 0 {
            return None;
        }

        if len < buffer.len() as i32 {
            return Some(unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned() });
        }
        let mut buf = vec![0; len as usize + 1];
        let len2 = unsafe { ffi::wxd_FileDialog_GetWildcard(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
        if len2 == len {
            Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() })
        } else {
            None
        }
    }

    /// Gets the index of the file type filter currently selected in dialog.
    /// Currently this function is fully implemented under macOS and MSW and always returns `-1` elsewhere.
    pub fn get_currently_selected_filter_index(&self) -> i32 {
        unsafe { ffi::wxd_FileDialog_GetCurrentlySelectedFilterIndex(self.as_ptr()) }
    }

    /// Sets the default directory.
    pub fn set_directory(&self, directory: &str) {
        let c_directory = CString::new(directory).expect("CString::new failed for directory");
        unsafe {
            ffi::wxd_FileDialog_SetDirectory(self.as_ptr(), c_directory.as_ptr());
        }
    }

    /// Sets the default filename.
    pub fn set_filename(&self, filename: &str) {
        let c_filename = CString::new(filename).expect("CString::new failed for filename");
        unsafe {
            ffi::wxd_FileDialog_SetFilename(self.as_ptr(), c_filename.as_ptr());
        }
    }

    /// Sets the default filter index, starting from zero.
    pub fn set_filter_index(&self, filter_index: i32) {
        unsafe {
            ffi::wxd_FileDialog_SetFilterIndex(self.as_ptr(), filter_index);
        }
    }

    /// Sets the message that will be displayed on the dialog.
    pub fn set_message(&self, message: &str) {
        let c_message = CString::new(message).expect("CString::new failed for message");
        unsafe {
            ffi::wxd_FileDialog_SetMessage(self.as_ptr(), c_message.as_ptr());
        }
    }

    /// Sets the path (the combined directory and filename that will be returned when the dialog is dismissed).
    pub fn set_path(&self, path: &str) {
        let c_path = CString::new(path).expect("CString::new failed for path");
        unsafe {
            ffi::wxd_FileDialog_SetPath(self.as_ptr(), c_path.as_ptr());
        }
    }

    /// Sets the wildcard, which can contain multiple file types.
    /// For example: "BMP files (*.bmp)|*.bmp|GIF files (*.gif)|*.gif".
    pub fn set_wildcard(&self, wildcard: &str) {
        let c_wildcard = CString::new(wildcard).expect("CString::new failed for wildcard");
        unsafe {
            ffi::wxd_FileDialog_SetWildcard(self.as_ptr(), c_wildcard.as_ptr());
        }
    }
}

// Implement WxWidget by delegating to the inner Dialog
impl WxWidget for FileDialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.dialog_base.handle_ptr()
    }
}

// FileDialogs are Windows (via Dialog base)
// FileDialogs are EvtHandlers (via Dialog base)

// Implement Drop
impl Drop for FileDialog {
    fn drop(&mut self) {
        // The composed Dialog's Drop will be called automatically,
        // which calls wxd_Window_Destroy on the pointer.
    }
}

// --- FileDialogBuilder ---
pub struct FileDialogBuilder<'a> {
    parent: &'a dyn WxWidget,
    message: String,
    default_dir: String,
    default_file: String,
    wildcard: String,
    style: FileDialogStyle,
    pos: Point,
    size: Size, // Often unused for FileDialog, but kept for consistency
}

impl<'a> FileDialogBuilder<'a> {
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        FileDialogBuilder {
            parent,
            message: "Choose a file".to_string(), // Default message
            default_dir: String::new(),
            default_file: String::new(),
            wildcard: "*.*".to_string(), // Default wildcard
            style: FileDialogStyle::Open,
            pos: Point::DEFAULT_POSITION,
            size: Size::DEFAULT_SIZE,
        }
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn with_default_dir(mut self, dir: &str) -> Self {
        self.default_dir = dir.to_string();
        self
    }

    pub fn with_default_file(mut self, file: &str) -> Self {
        self.default_file = file.to_string();
        self
    }

    pub fn with_wildcard(mut self, wildcard: &str) -> Self {
        self.wildcard = wildcard.to_string();
        self
    }

    pub fn with_style(mut self, style: FileDialogStyle) -> Self {
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

    pub fn build(self) -> FileDialog {
        let c_message = CString::new(self.message).expect("CString::new failed for message");
        let c_default_dir = CString::new(self.default_dir).expect("CString::new failed for default_dir");
        let c_default_file = CString::new(self.default_file).expect("CString::new failed for default_file");
        let c_wildcard = CString::new(self.wildcard).expect("CString::new failed for wildcard");
        let parent_ptr = self.parent.handle_ptr();
        assert!(!parent_ptr.is_null(), "FileDialog requires a valid parent window pointer.");

        let ptr = unsafe {
            ffi::wxd_FileDialog_Create(
                parent_ptr,
                c_message.as_ptr(),
                c_default_dir.as_ptr(),
                c_default_file.as_ptr(),
                c_wildcard.as_ptr(),
                self.style.bits() as ffi::wxd_Style_t,
                self.pos.x,
                self.pos.y, // Pass position components
                self.size.width,
                self.size.height, // Pass size components
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxFileDialog");
        }
        unsafe { FileDialog::from_ptr(ptr) }
    }
}
