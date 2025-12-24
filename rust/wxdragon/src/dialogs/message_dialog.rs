use crate::dialogs::Dialog; // Use the Dialog from the parent module
use crate::window::WxWidget; // Keep this
use std::ffi::CString;
use wxdragon_sys as ffi;

// Opaque C pointer for wxMessageDialog
pub type MessageDialogPtr = *mut ffi::wxd_MessageDialog;

// Define MessageDialogStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: MessageDialogStyle,
    doc: "Style flags for MessageDialog.",
    variants: {
        OK: ffi::WXD_OK, "Default button is 'OK'.",
        Cancel: ffi::WXD_CANCEL, "Include 'Cancel' button.",
        YesNo: ffi::WXD_YES_NO, "Use 'Yes' and 'No' buttons instead of 'OK' and 'Cancel'.",
        Yes: ffi::WXD_YES, "Include 'Yes' button.",
        No: ffi::WXD_NO, "Include 'No' button.",
        IconNone: ffi::WXD_ICON_NONE, "No icon.",
        IconWarning: ffi::WXD_ICON_WARNING, "Same as IconExclamation.",
        IconError: ffi::WXD_ICON_ERROR, "Same as IconHand.",
        IconQuestion: ffi::WXD_ICON_QUESTION, "Show a question mark icon.",
        IconInformation: ffi::WXD_ICON_INFORMATION, "Show an information symbol.",
        IconAuthNeeded: ffi::WXD_ICON_AUTH_NEEDED, "Show an authentication needed symbol.",
        Centre: ffi::WXD_CENTRE, "Center the dialog on its parent."
    },
    default_variant: OK
);

// --- MessageDialog ---
#[derive(Clone)]
pub struct MessageDialog {
    // ptr: MessageDialogPtr, // This would be specific
    dialog_base: Dialog, // Composition: MessageDialog IS A Dialog
}

impl MessageDialog {
    /// Creates a new MessageDialog from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxMessageDialog.
    pub unsafe fn from_ptr(ptr: MessageDialogPtr) -> Self {
        MessageDialog {
            dialog_base: unsafe { Dialog::from_ptr(ptr as *mut ffi::wxd_Dialog_t) },
        }
    }

    pub fn builder<'a>(parent: &'a dyn WxWidget, message: &str, caption: &str) -> MessageDialogBuilder<'a> {
        MessageDialogBuilder::new(parent, message, caption)
    }

    /// Shows the dialog modally.
    /// Returns an integer value which is usually one of the standard dialog return codes
    /// (e.g., ID_OK, ID_CANCEL, ID_YES, ID_NO).
    pub fn show_modal(&self) -> i32 {
        self.dialog_base.show_modal()
    }

    pub fn as_ptr(&self) -> MessageDialogPtr {
        self.dialog_base.as_ptr() as MessageDialogPtr
    }
}

impl WxWidget for MessageDialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.dialog_base.handle_ptr()
    }
}

// MessageDialogs are Windows (via Dialog base)

// MessageDialogs are EvtHandlers (via Dialog base)

impl Drop for MessageDialog {
    fn drop(&mut self) {
        if !self.handle_ptr().is_null() {
            // Ensure Rust knows this pointer is managed by Rust for this instance.
            // If this instance is a simple wrapper not owning the underlying object,
            // (e.g. from a GetParent call), then this Drop might be too aggressive.
            // For dialogs created via builder, they are owned.
            unsafe {
                // Check if it's already being destroyed by wxWidgets or a parent
                // For dialogs shown modally, they are often auto-destroyed or parented such that
                // wxWidgets handles their deletion. However, explicit Destroy is safer if we created it.
                // The current C API relies on wxd_Window_Destroy.
                ffi::wxd_Window_Destroy(self.handle_ptr());
            }
        }
    }
}

// --- MessageDialogBuilder ---
pub struct MessageDialogBuilder<'a> {
    parent: &'a dyn WxWidget,
    message: String,
    caption: String,
    style: MessageDialogStyle, // Using the enum instead of i64
}

impl<'a> MessageDialogBuilder<'a> {
    pub fn new(parent: &'a dyn WxWidget, message: &str, caption: &str) -> Self {
        MessageDialogBuilder {
            parent,
            message: message.to_string(),
            caption: caption.to_string(),
            style: MessageDialogStyle::OK, // Default style
        }
    }

    pub fn with_style(mut self, style: MessageDialogStyle) -> Self {
        self.style = style;
        self
    }

    // Convenience methods for common styles might be added later, e.g.:
    // pub fn ok(mut self) -> Self { self.style = MessageDialogStyle::OK; self }
    // pub fn yes_no(mut self) -> Self { self.style = MessageDialogStyle::YesNo; self }

    pub fn build(self) -> MessageDialog {
        let c_message = CString::new(self.message).expect("CString::new failed for message");
        let c_caption = CString::new(self.caption).expect("CString::new failed for caption");
        let parent_ptr = self.parent.handle_ptr();
        assert!(!parent_ptr.is_null(), "MessageDialog requires a valid parent window pointer.");

        let ptr = unsafe {
            ffi::wxd_MessageDialog_Create(
                parent_ptr,
                c_message.as_ptr(),
                c_caption.as_ptr(),
                self.style.bits() as ffi::wxd_Style_t,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxMessageDialog");
        }
        unsafe { MessageDialog::from_ptr(ptr) }
    }
}
