use std::ffi::CString;
use std::ptr;

use crate::{dialogs::Dialog, window::WxWidget};
use wxdragon_sys as ffi;

// Define ProgressDialogStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: ProgressDialogStyle,
    doc: "Style flags for ProgressDialog.",
    variants: {
        AppModal: ffi::WXD_PD_APP_MODAL, "Dialog is modal for the application (all windows are disabled while the progress dialog exists).",
        AutoHide: ffi::WXD_PD_AUTO_HIDE, "Automatically hide the dialog when it reaches the maximum value.",
        Smooth: ffi::WXD_PD_SMOOTH, "Display smooth progress bar (not supported by all platforms).",
        CanAbort: ffi::WXD_PD_CAN_ABORT, "Display a Cancel button that allows aborting the operation.",
        CanSkip: ffi::WXD_PD_CAN_SKIP, "Display a Skip button that allows skipping a part of the operation.",
        ElapsedTime: ffi::WXD_PD_ELAPSED_TIME, "Display elapsed time.",
        EstimatedTime: ffi::WXD_PD_ESTIMATED_TIME, "Display estimated time.",
        RemainingTime: ffi::WXD_PD_REMAINING_TIME, "Display remaining time."
    },
    default_variant: AppModal
);

/// Wrapper for wxProgressDialog.
/// A dialog that shows a progress bar and optional text.
#[derive(Clone)]
pub struct ProgressDialog {
    dialog_base: Dialog,
}

/// Builder for ProgressDialog
pub struct ProgressDialogBuilder<'a, W: WxWidget> {
    parent: &'a W,
    title: String,
    message: String,
    maximum: i32,
    style: ProgressDialogStyle,
}

impl ProgressDialog {
    /// Creates a builder for a progress dialog.
    pub fn builder<'a, W: WxWidget>(parent: &'a W, title: &str, message: &str, maximum: i32) -> ProgressDialogBuilder<'a, W> {
        ProgressDialogBuilder {
            parent,
            title: title.to_string(),
            message: message.to_string(),
            maximum,
            style: ProgressDialogStyle::AutoHide | ProgressDialogStyle::AppModal,
        }
    }

    /// Creates a new ProgressDialog wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxProgressDialog.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_ProgressDialog_t) -> Self {
        ProgressDialog {
            dialog_base: unsafe { Dialog::from_ptr(ptr as *mut ffi::wxd_Dialog_t) },
        }
    }

    fn as_ptr(&self) -> *mut ffi::wxd_ProgressDialog_t {
        self.dialog_base.as_ptr() as *mut ffi::wxd_ProgressDialog_t
    }

    /// Updates the dialog, setting the progress bar to the given value
    /// and, optionally, updating the message shown.
    /// Returns `true` if the "Cancel" button has not been pressed.
    ///
    /// - `value`: The new value of the progress meter (between 0 and the maximum)
    /// - `newmsg`: The new progress message to display in the dialog, or None to not change it
    pub fn update(&self, value: i32, newmsg: Option<&str>) -> bool {
        let c_newmsg = newmsg.map(|s| CString::new(s).expect("CString::new failed"));
        let newmsg_ptr = c_newmsg.as_ref().map_or(ptr::null(), |cs| cs.as_ptr());

        let mut skip = false;

        unsafe { ffi::wxd_ProgressDialog_Update(self.as_ptr(), value, newmsg_ptr, &mut skip) }
    }

    /// Updates the dialog, setting the progress bar to the given value
    /// and, optionally, updating the message shown.
    /// Returns a tuple with:
    /// - `continue`: `true` if the "Cancel" button has not been pressed
    /// - `skipped`: `true` if the "Skip" button has been pressed
    ///
    /// - `value`: The new value of the progress meter (between 0 and the maximum)
    /// - `newmsg`: The new progress message to display in the dialog, or None to not change it
    pub fn update_with_skip(&self, value: i32, newmsg: Option<&str>) -> (bool, bool) {
        let c_newmsg = newmsg.map(|s| CString::new(s).expect("CString::new failed"));
        let newmsg_ptr = c_newmsg.as_ref().map_or(ptr::null(), |cs| cs.as_ptr());

        let mut skip = false;
        let result = unsafe { ffi::wxd_ProgressDialog_Update(self.as_ptr(), value, newmsg_ptr, &mut skip) };

        (result, skip)
    }

    /// Switches the progress dialog to indeterminate mode and makes the gauge control
    /// run back and forth.
    ///
    /// - `newmsg`: The new progress message to display in the dialog, or None to not change it
    pub fn pulse(&self, newmsg: Option<&str>) -> bool {
        let c_newmsg = newmsg.map(|s| CString::new(s).expect("CString::new failed"));
        let newmsg_ptr = c_newmsg.as_ref().map_or(ptr::null(), |cs| cs.as_ptr());

        let mut skip = false;

        unsafe { ffi::wxd_ProgressDialog_Pulse(self.as_ptr(), newmsg_ptr, &mut skip) }
    }

    /// Switches the progress dialog to indeterminate mode and makes the gauge control
    /// run back and forth.
    /// Returns a tuple with:
    /// - `continue`: `true` if the "Cancel" button has not been pressed
    /// - `skipped`: `true` if the "Skip" button has been pressed
    ///
    /// - `newmsg`: The new progress message to display in the dialog, or None to not change it
    pub fn pulse_with_skip(&self, newmsg: Option<&str>) -> (bool, bool) {
        let c_newmsg = newmsg.map(|s| CString::new(s).expect("CString::new failed"));
        let newmsg_ptr = c_newmsg.as_ref().map_or(ptr::null(), |cs| cs.as_ptr());

        let mut skip = false;
        let result = unsafe { ffi::wxd_ProgressDialog_Pulse(self.as_ptr(), newmsg_ptr, &mut skip) };

        (result, skip)
    }

    /// Resumes dialog after calling update(value, "", skip) or Pulse("", skip) if `skip` was true.
    pub fn resume(&self) {
        unsafe {
            ffi::wxd_ProgressDialog_Resume(self.as_ptr());
        }
    }

    /// Returns the current value of the progress meter.
    pub fn get_value(&self) -> i32 {
        unsafe { ffi::wxd_ProgressDialog_GetValue(self.as_ptr()) }
    }

    /// Returns the maximum value of the progress meter.
    pub fn get_range(&self) -> i32 {
        unsafe { ffi::wxd_ProgressDialog_GetRange(self.as_ptr()) }
    }

    /// Returns whether the Cancel button has been pressed.
    pub fn was_cancelled(&self) -> bool {
        unsafe { ffi::wxd_ProgressDialog_WasCancelled(self.as_ptr()) }
    }

    /// Returns whether the Skip button has been pressed.
    pub fn was_skipped(&self) -> bool {
        unsafe { ffi::wxd_ProgressDialog_WasSkipped(self.as_ptr()) }
    }
}

impl<'a, W: WxWidget> ProgressDialogBuilder<'a, W> {
    /// Set the style flags for the progress dialog
    pub fn with_style(mut self, style: ProgressDialogStyle) -> Self {
        self.style = style;
        self
    }

    /// Add a style flag to the existing style flags
    pub fn add_style(mut self, style_flag: ProgressDialogStyle) -> Self {
        self.style |= style_flag;
        self
    }

    /// Add the CanAbort flag to allow cancelling the operation
    pub fn can_abort(self) -> Self {
        self.add_style(ProgressDialogStyle::CanAbort)
    }

    /// Add the CanSkip flag to allow skipping parts of the operation
    pub fn can_skip(self) -> Self {
        self.add_style(ProgressDialogStyle::CanSkip)
    }

    /// Add the ElapsedTime flag to show elapsed time
    pub fn show_elapsed_time(self) -> Self {
        self.add_style(ProgressDialogStyle::ElapsedTime)
    }

    /// Add the EstimatedTime flag to show estimated time
    pub fn show_estimated_time(self) -> Self {
        self.add_style(ProgressDialogStyle::EstimatedTime)
    }

    /// Add the RemainingTime flag to show remaining time
    pub fn show_remaining_time(self) -> Self {
        self.add_style(ProgressDialogStyle::RemainingTime)
    }

    /// Add the Smooth flag for a smooth progress bar
    pub fn smooth(self) -> Self {
        self.add_style(ProgressDialogStyle::Smooth)
    }

    /// Build the ProgressDialog
    pub fn build(self) -> ProgressDialog {
        let c_title = CString::new(self.title).expect("CString::new failed for title");
        let c_message = CString::new(self.message).expect("CString::new failed for message");
        let parent_ptr = self.parent.handle_ptr();
        assert!(
            !parent_ptr.is_null(),
            "ProgressDialog requires a valid parent window pointer."
        );

        let ptr = unsafe {
            ffi::wxd_ProgressDialog_Create(
                parent_ptr,
                c_title.as_ptr(),
                c_message.as_ptr(),
                self.maximum,
                self.style.bits(),
            )
        };

        if ptr.is_null() {
            panic!("Failed to create wxProgressDialog");
        }

        unsafe { ProgressDialog::from_ptr(ptr) }
    }
}

impl WxWidget for ProgressDialog {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.dialog_base.handle_ptr()
    }
}

// Add a Drop implementation for ProgressDialog for proper cleanup
impl Drop for ProgressDialog {
    fn drop(&mut self) {
        unsafe {
            // Use destroy from the Window trait to clean up resources
            ffi::wxd_Window_Destroy(self.handle_ptr());
        }
    }
}
