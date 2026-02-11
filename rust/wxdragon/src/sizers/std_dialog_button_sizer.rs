use std::ops::Deref;

use crate::sizers::WxSizer as WxSizerTrait;
use crate::sizers::base::Sizer;
use crate::widgets::button::Button;
use crate::window::WxWidget;
use wxdragon_sys as ffi;

/// Represents the wxStdDialogButtonSizer.
///
/// This sizer arranges dialog buttons according to platform-specific HIG
/// (Human Interface Guidelines). It automatically reorders OK, Cancel, Yes,
/// No, Help, and Apply buttons to match the native convention on Windows,
/// macOS, and GTK+.
///
/// Valid button IDs for `add_button`: `ID_OK`, `ID_YES`, `ID_SAVE`,
/// `ID_APPLY`, `ID_CLOSE`, `ID_NO`, `ID_CANCEL`, `ID_HELP`,
/// `ID_CONTEXT_HELP`.
///
/// After adding all buttons, call `realize()` to lay them out.
#[derive(Clone, Copy)]
pub struct StdDialogButtonSizer {
    raw_specific_ptr: *mut ffi::wxd_StdDialogButtonSizer_t,
    sizer_base: Sizer,
}

impl StdDialogButtonSizer {
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StdDialogButtonSizer_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            let base_ptr = ptr as *mut ffi::wxd_Sizer_t;
            unsafe {
                Sizer::from_ptr(base_ptr).map(|sizer_base| StdDialogButtonSizer {
                    raw_specific_ptr: ptr,
                    sizer_base,
                })
            }
        }
    }

    /// Adds a button to the sizer. The button must have one of the standard
    /// IDs: `ID_OK`, `ID_YES`, `ID_SAVE`, `ID_APPLY`, `ID_CLOSE`, `ID_NO`,
    /// `ID_CANCEL`, `ID_HELP`, or `ID_CONTEXT_HELP`.
    pub fn add_button(&self, button: &Button) {
        unsafe {
            ffi::wxd_StdDialogButtonSizer_AddButton(self.raw_specific_ptr, button.handle_ptr() as *mut ffi::wxd_Button_t);
        }
    }

    /// Rearranges the buttons and applies platform-appropriate spacing.
    /// Must be called after all buttons have been added.
    pub fn realize(&self) {
        unsafe {
            ffi::wxd_StdDialogButtonSizer_Realize(self.raw_specific_ptr);
        }
    }

    /// Returns the affirmative button (OK, YES, or SAVE), if set.
    pub fn get_affirmative_button(&self) -> Option<Button> {
        unsafe {
            let ptr = ffi::wxd_StdDialogButtonSizer_GetAffirmativeButton(self.raw_specific_ptr);
            if ptr.is_null() {
                None
            } else {
                Some(Button::from_ptr(ptr as *mut ffi::wxd_Window_t))
            }
        }
    }

    /// Returns the apply button, if set.
    pub fn get_apply_button(&self) -> Option<Button> {
        unsafe {
            let ptr = ffi::wxd_StdDialogButtonSizer_GetApplyButton(self.raw_specific_ptr);
            if ptr.is_null() {
                None
            } else {
                Some(Button::from_ptr(ptr as *mut ffi::wxd_Window_t))
            }
        }
    }

    /// Returns the negative button (NO), if set.
    pub fn get_negative_button(&self) -> Option<Button> {
        unsafe {
            let ptr = ffi::wxd_StdDialogButtonSizer_GetNegativeButton(self.raw_specific_ptr);
            if ptr.is_null() {
                None
            } else {
                Some(Button::from_ptr(ptr as *mut ffi::wxd_Window_t))
            }
        }
    }

    /// Returns the cancel button (CANCEL or CLOSE), if set.
    pub fn get_cancel_button(&self) -> Option<Button> {
        unsafe {
            let ptr = ffi::wxd_StdDialogButtonSizer_GetCancelButton(self.raw_specific_ptr);
            if ptr.is_null() {
                None
            } else {
                Some(Button::from_ptr(ptr as *mut ffi::wxd_Window_t))
            }
        }
    }

    /// Returns the help button (HELP or CONTEXT_HELP), if set.
    pub fn get_help_button(&self) -> Option<Button> {
        unsafe {
            let ptr = ffi::wxd_StdDialogButtonSizer_GetHelpButton(self.raw_specific_ptr);
            if ptr.is_null() {
                None
            } else {
                Some(Button::from_ptr(ptr as *mut ffi::wxd_Window_t))
            }
        }
    }

    /// Sets a custom affirmative button (for non-standard button IDs).
    pub fn set_affirmative_button(&self, button: &Button) {
        unsafe {
            ffi::wxd_StdDialogButtonSizer_SetAffirmativeButton(
                self.raw_specific_ptr,
                button.handle_ptr() as *mut ffi::wxd_Button_t,
            );
        }
    }

    /// Sets a custom negative button (for non-standard button IDs).
    pub fn set_negative_button(&self, button: &Button) {
        unsafe {
            ffi::wxd_StdDialogButtonSizer_SetNegativeButton(self.raw_specific_ptr, button.handle_ptr() as *mut ffi::wxd_Button_t);
        }
    }

    /// Sets a custom cancel button (for non-standard button IDs).
    pub fn set_cancel_button(&self, button: &Button) {
        unsafe {
            ffi::wxd_StdDialogButtonSizer_SetCancelButton(self.raw_specific_ptr, button.handle_ptr() as *mut ffi::wxd_Button_t);
        }
    }
}

impl WxSizerTrait for StdDialogButtonSizer {
    fn as_sizer_ptr(&self) -> *mut ffi::wxd_Sizer_t {
        self.sizer_base.as_sizer_ptr()
    }
}

impl Deref for StdDialogButtonSizer {
    type Target = Sizer;
    fn deref(&self) -> &Self::Target {
        &self.sizer_base
    }
}

/// Builder for [`StdDialogButtonSizer`].
pub struct StdDialogButtonSizerBuilder;

impl StdDialogButtonSizerBuilder {
    pub fn new() -> Self {
        StdDialogButtonSizerBuilder
    }

    pub fn build(self) -> StdDialogButtonSizer {
        let ptr = unsafe { ffi::wxd_StdDialogButtonSizer_Create() };
        unsafe { StdDialogButtonSizer::from_ptr(ptr).expect("Failed to create wxStdDialogButtonSizer") }
    }
}

impl Default for StdDialogButtonSizerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
