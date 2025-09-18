use crate::event::WindowEvents;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
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
/// # Lifetime Management
/// Dialog instances are typically shown modally and should be destroyed after use.
/// Call the `.destroy()` method (available via the `WxWidget` trait) when the dialog
/// is no longer needed to ensure proper cleanup.
#[derive(Clone)]
pub struct Dialog {
    window: Window, // Composition: Dialog uses a Window internally
    // Store parent pointer to manage drop behavior
    #[allow(dead_code)]
    parent_ptr: *mut ffi::wxd_Window_t,
    _marker: PhantomData<()>,
}

impl Dialog {
    /// Creates a new Dialog from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxDialog.
    pub unsafe fn from_ptr(ptr: *mut ffi::wxd_Dialog_t) -> Self {
        Dialog {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
            parent_ptr: std::ptr::null_mut(),
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
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
            parent_ptr: std::ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL, ID_YES, ID_NO).
    pub fn show_modal(&self) -> i32 {
        unsafe { ffi::wxd_Dialog_ShowModal(self.window.as_ptr() as *mut ffi::wxd_Dialog_t) }
    }

    /// Ends the modal dialog with the given return code.
    /// This method should be called from event handlers to close the dialog.
    /// The return code is what will be returned by show_modal().
    pub fn end_modal(&self, ret_code: i32) {
        unsafe {
            ffi::wxd_Dialog_EndModal(self.window.as_ptr() as *mut ffi::wxd_Dialog_t, ret_code)
        }
    }

    /// Returns the raw underlying dialog pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_Dialog_t {
        self.window.as_ptr() as *mut ffi::wxd_Dialog_t
    }

    /// Creates a new builder for a generic Dialog.
    pub fn builder<'a>(parent: &'a dyn WxWidget, title: &str) -> DialogBuilder<'a> {
        DialogBuilder::new(parent, title)
    }
}

// Apply common trait implementations for Dialog
implement_widget_traits_with_target!(Dialog, window, Window);

// Implement WindowEvents trait for Dialog
impl WindowEvents for Dialog {}

// XRC Support - enables Dialog to be created from XRC-managed pointers
impl_xrc_support!(Dialog, {
    window,
    parent_ptr: std::ptr::null_mut(),
    _marker: PhantomData
});

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
        let parent_ptr = self.parent.handle_ptr();
        let c_title = CString::new(self.title).unwrap_or_else(|_| CString::new("").unwrap());

        let dialog_ptr = unsafe {
            ffi::wxd_Dialog_Create(
                parent_ptr,
                c_title.as_ptr(),
                self.style.bits() as ffi::wxd_Style_t,
                self.x,
                self.y,
                self.width,
                self.height,
            )
        };

        if dialog_ptr.is_null() {
            panic!("Failed to create Dialog");
        }

        unsafe { Dialog::from_ptr(dialog_ptr) }
    }
}
