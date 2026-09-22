//! Validators restricting what can be typed into a control.
//!
//! Currently only [`TextValidator`], wrapping `wxTextValidator`, is available.
//! It is attached to a control with
//! [`WxWidget::set_text_validator`](crate::window::WxWidget::set_text_validator).

use std::ffi::CString;

use wxdragon_sys as ffi;

widget_style_enum!(
    name: TextValidatorStyle,
    doc: "Style flags for TextValidator, defining what the control accepts.",
    variants: {
        None: ffi::WXD_FILTER_NONE, "No filtering at all.",
        Empty: ffi::WXD_FILTER_EMPTY, "Reject empty values.",
        Ascii: ffi::WXD_FILTER_ASCII, "Accept ASCII characters only.",
        Alpha: ffi::WXD_FILTER_ALPHA, "Accept letters only.",
        Alphanumeric: ffi::WXD_FILTER_ALPHANUMERIC, "Accept letters and digits only.",
        Digits: ffi::WXD_FILTER_DIGITS, "Accept decimal digits only.",
        XDigits: ffi::WXD_FILTER_XDIGITS, "Accept hexadecimal digits only.",
        Numeric: ffi::WXD_FILTER_NUMERIC, "Accept digits, and also the sign, the decimal separator and the exponent.",
        Space: ffi::WXD_FILTER_SPACE, "Accept spaces too, to be combined with the other flags.",
        IncludeList: ffi::WXD_FILTER_INCLUDE_LIST, "Accept only the strings given to `with_includes()`.",
        ExcludeList: ffi::WXD_FILTER_EXCLUDE_LIST, "Reject the strings given to `with_excludes()`.",
        IncludeCharList: ffi::WXD_FILTER_INCLUDE_CHAR_LIST, "Accept only the characters given to `with_char_includes()`.",
        ExcludeCharList: ffi::WXD_FILTER_EXCLUDE_CHAR_LIST, "Reject the characters given to `with_char_excludes()`."
    },
    default_variant: None
);

/// Restricts what can be typed into a text control.
///
/// ```no_run
/// use wxdragon::prelude::*;
///
/// # fn example(text_ctrl: &TextCtrl) {
/// text_ctrl.set_text_validator(&TextValidator::new(TextValidatorStyle::Digits));
/// # }
/// ```
pub struct TextValidator {
    ptr: *mut ffi::wxd_TextValidator_t,
}

impl TextValidator {
    /// Creates a validator accepting what the given style allows.
    pub fn new(style: TextValidatorStyle) -> Self {
        Self {
            ptr: unsafe { ffi::wxd_TextValidator_Create(style.bits()) },
        }
    }

    /// Sets the characters to accept, used with [`TextValidatorStyle::IncludeCharList`].
    pub fn with_char_includes(self, chars: &str) -> Self {
        if let Ok(c_chars) = CString::new(chars) {
            unsafe { ffi::wxd_TextValidator_SetCharIncludes(self.ptr, c_chars.as_ptr()) };
        }
        self
    }

    /// Sets the characters to reject, used with [`TextValidatorStyle::ExcludeCharList`].
    pub fn with_char_excludes(self, chars: &str) -> Self {
        if let Ok(c_chars) = CString::new(chars) {
            unsafe { ffi::wxd_TextValidator_SetCharExcludes(self.ptr, c_chars.as_ptr()) };
        }
        self
    }

    /// Sets the values to accept, used with [`TextValidatorStyle::IncludeList`].
    pub fn with_includes(self, items: &[&str]) -> Self {
        with_strings(items, |ptrs| unsafe {
            ffi::wxd_TextValidator_SetIncludes(self.ptr, ptrs.as_ptr(), ptrs.len())
        });
        self
    }

    /// Sets the values to reject, used with [`TextValidatorStyle::ExcludeList`].
    pub fn with_excludes(self, items: &[&str]) -> Self {
        with_strings(items, |ptrs| unsafe {
            ffi::wxd_TextValidator_SetExcludes(self.ptr, ptrs.as_ptr(), ptrs.len())
        });
        self
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::wxd_TextValidator_t {
        self.ptr
    }
}

impl Drop for TextValidator {
    fn drop(&mut self) {
        unsafe { ffi::wxd_TextValidator_Destroy(self.ptr) };
    }
}

fn with_strings(items: &[&str], f: impl FnOnce(&[*const std::os::raw::c_char])) {
    let owned: Vec<CString> = items.iter().filter_map(|item| CString::new(*item).ok()).collect();
    let ptrs: Vec<*const std::os::raw::c_char> = owned.iter().map(|item| item.as_ptr()).collect();
    f(&ptrs);
}
