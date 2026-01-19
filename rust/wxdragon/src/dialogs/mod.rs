use crate::Bitmap;
use crate::event::WxEvtHandler;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::CString;
use std::marker::PhantomData;
use wxdragon_sys as ffi;

pub mod colour_dialog;
pub mod dir_dialog;
pub mod file_dialog;
pub mod font_dialog;
pub mod message_dialog;
pub mod multi_choice_dialog;
pub mod progress_dialog;
pub mod single_choice_dialog;
pub mod text_entry_dialog;

// Define DialogStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: DialogStyle,
    doc: "Style flags for Dialog.",
    variants: {
        DefaultDialogStyle: ffi::WXD_DEFAULT_DIALOG_STYLE, "Default dialog style (includes Caption, SystemMenu, CloseBox).",
        Caption: ffi::WXD_CAPTION, "Show a caption on the dialog.",
        ResizeBorder: ffi::WXD_RESIZE_BORDER, "Allow the dialog to be resized.",
        SystemMenu: ffi::WXD_SYSTEM_MENU, "Show the system menu (on systems that have one).",
        CloseBox: ffi::WXD_CLOSE_BOX, "Show a close box on the dialog.",
        MaximizeBox: ffi::WXD_MAXIMIZE_BOX, "Show a maximize box on the dialog.",
        MinimizeBox: ffi::WXD_MINIMIZE_BOX, "Show a minimize box on the dialog.",
        StayOnTop: ffi::WXD_STAY_ON_TOP, "Keep the dialog on top of other windows."
    },
    default_variant: DefaultDialogStyle
);

// --- Dialog --- (Base struct for dialogs)
/// Represents a wxDialog.
///
/// Dialog uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed, the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Lifetime Management
/// Dialog instances are typically shown modally and should be destroyed after use.
/// Call the `.destroy()` method (available via the `WxWidget` trait) when the dialog
/// is no longer needed to ensure proper cleanup.
#[derive(Clone, Copy)]
pub struct Dialog {
    /// Safe handle to the underlying wxDialog - automatically invalidated on destroy
    handle: WindowHandle,
    _marker: PhantomData<()>,
}

impl Dialog {
    /// Creates a new Dialog from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxDialog.
    pub unsafe fn from_ptr(ptr: *mut ffi::wxd_Dialog_t) -> Self {
        Dialog {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
            _marker: PhantomData,
        }
    }

    /// Creates a Dialog wrapper for an XRC-managed object.
    /// This dialog will not be destroyed when dropped as it's managed by XRC.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `ptr` is a valid pointer to a wxDialog object
    /// - The dialog object pointed to by `ptr` remains valid for the lifetime of the returned Dialog
    /// - No other code is concurrently accessing or modifying the dialog object
    /// - The dialog was properly initialized by wxWidgets XRC loading
    pub unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Dialog_t) -> Self {
        Dialog {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
            _marker: PhantomData,
        }
    }

    /// Helper to get raw dialog pointer, returns null if dialog has been destroyed
    #[inline]
    fn dialog_ptr(&self) -> *mut ffi::wxd_Dialog_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_Dialog_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Returns the underlying WindowHandle for this Dialog.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    /// Sets the icon for the dialog.
    /// No-op if the dialog has been destroyed.
    pub fn set_icon(&self, bitmap: &Bitmap) {
        let ptr = self.dialog_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Dialog_SetIcon(ptr, bitmap.as_const_ptr()) }
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL, ID_YES, ID_NO).
    /// Returns -1 if the dialog has been destroyed.
    pub fn show_modal(&self) -> i32 {
        let ptr = self.dialog_ptr();
        if ptr.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_Dialog_ShowModal(ptr) }
    }

    /// Ends the modal dialog with the given return code.
    /// This method should be called from event handlers to close the dialog.
    /// The return code is what will be returned by show_modal().
    /// No-op if the dialog has been destroyed.
    pub fn end_modal(&self, ret_code: i32) {
        let ptr = self.dialog_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Dialog_EndModal(ptr, ret_code) }
    }

    /// Sets the identifier of the button which should work like the standard "Cancel" button.
    ///
    /// When the button with this ID is clicked, the dialog is closed. Also, when the user
    /// presses ESC in the dialog or closes it using the close button in the title bar,
    /// this is mapped to clicking the button with the specified ID.
    ///
    /// # Special Values
    /// - `ID_ANY` (default): Maps ESC to `ID_CANCEL` if present, otherwise to the affirmative ID
    /// - `ID_NONE`: Disables ESC key handling entirely
    /// - Any other ID: Maps ESC to that specific button ID
    ///
    /// # Example
    /// ```ignore
    /// use wxdragon::{Dialog, id::{ID_NONE, ID_CANCEL}};
    ///
    /// // Disable ESC key closing the dialog
    /// dialog.set_escape_id(ID_NONE);
    ///
    /// // Use a custom button for ESC
    /// dialog.set_escape_id(my_custom_button_id);
    /// ```
    pub fn set_escape_id(&self, id: i32) {
        let ptr = self.dialog_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Dialog_SetEscapeId(ptr, id) }
    }

    /// Gets the identifier of the button to map presses of ESC button to.
    ///
    /// Returns the escape ID, which may be a specific button ID or one of the special values
    /// `ID_ANY` (default behavior) or `ID_NONE` (ESC disabled).
    pub fn get_escape_id(&self) -> i32 {
        let ptr = self.dialog_ptr();
        if ptr.is_null() {
            return crate::id::ID_NONE;
        }
        unsafe { ffi::wxd_Dialog_GetEscapeId(ptr) }
    }

    /// Sets the identifier to be used as the OK button.
    ///
    /// When the button with this ID is pressed, the dialog calls validation and
    /// data transfer, and if both succeed, closes the dialog with the affirmative ID
    /// as the return code.
    ///
    /// By default, the affirmative ID is `ID_OK`.
    pub fn set_affirmative_id(&self, id: i32) {
        let ptr = self.dialog_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Dialog_SetAffirmativeId(ptr, id) }
    }

    /// Gets the identifier of the button which works like standard OK button.
    ///
    /// Returns `ID_OK` by default.
    pub fn get_affirmative_id(&self) -> i32 {
        let ptr = self.dialog_ptr();
        if ptr.is_null() {
            return crate::id::ID_OK;
        }
        unsafe { ffi::wxd_Dialog_GetAffirmativeId(ptr) }
    }

    /// Sets the return code for this dialog.
    ///
    /// A return code is normally associated with a modal dialog, where `show_modal()`
    /// returns a code to the application. The `end_modal()` method calls this internally.
    pub fn set_return_code(&self, ret_code: i32) {
        let ptr = self.dialog_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Dialog_SetReturnCode(ptr, ret_code) }
    }

    /// Gets the return code for this dialog.
    ///
    /// Returns the value that was set by `set_return_code()` or `end_modal()`.
    pub fn get_return_code(&self) -> i32 {
        let ptr = self.dialog_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Dialog_GetReturnCode(ptr) }
    }

    /// Returns the raw underlying dialog pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_Dialog_t {
        self.dialog_ptr()
    }

    /// Creates a new builder for a generic Dialog.
    pub fn builder<'a>(parent: &'a dyn WxWidget, title: &str) -> DialogBuilder<'a> {
        DialogBuilder::new(parent, title)
    }
}

// Manual WxWidget implementation for Dialog (using WindowHandle)
impl WxWidget for Dialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for Dialog {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for Dialog {}

// XRC Support - enables Dialog to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for Dialog {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Dialog {
            handle: WindowHandle::new(ptr),
            _marker: PhantomData,
        }
    }
}

// Dialogs are windows
// Remove: impl WindowMethods for Dialog {}

// Dialogs are event handlers -> This comes from WxEvtHandler
// (Already removed EvtHandlerMethods)

// No explicit Drop for Dialog base struct here. Actual dialog instances (like MessageDialog)
// will be wrapped, and their Drop will call wxd_Window_Destroy on the pointer,
// which is appropriate as wxDialog inherits from wxWindow.

// --- DialogBuilder ---
/// Builder for creating generic Dialog instances.
pub struct DialogBuilder<'a> {
    parent: &'a dyn WxWidget,
    title: String,
    style: DialogStyle,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl<'a> DialogBuilder<'a> {
    /// Creates a new DialogBuilder with the given parent and title.
    pub fn new(parent: &'a dyn WxWidget, title: &str) -> Self {
        DialogBuilder {
            parent,
            title: title.to_string(),
            style: DialogStyle::DefaultDialogStyle,
            x: -1,
            y: -1,
            width: -1,
            height: -1,
        }
    }

    /// Sets the dialog style.
    pub fn with_style(mut self, style: DialogStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the dialog position.
    pub fn with_position(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the dialog size.
    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Builds the Dialog.
    pub fn build(self) -> Dialog {
        let parent = self.parent.handle_ptr();
        let title = CString::new(self.title).unwrap_or_else(|_| CString::new("").unwrap());
        let s = self.style.bits() as ffi::wxd_Style_t;

        let dialog = unsafe { ffi::wxd_Dialog_Create(parent, title.as_ptr(), s, self.x, self.y, self.width, self.height) };

        if dialog.is_null() {
            panic!("Failed to create Dialog");
        }

        unsafe { Dialog::from_ptr(dialog) }
    }
}
