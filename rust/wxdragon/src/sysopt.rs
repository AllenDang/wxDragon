use std::ffi::{CStr, CString};

pub struct SystemOptions;

impl SystemOptions {
    pub fn set_option_by_string(name: &str, value: &str) {
        let name = CString::new(name).unwrap();
        let value = CString::new(value).unwrap();
        unsafe { crate::ffi::wxd_SystemOptions_SetOption_String(name.as_ptr(), value.as_ptr()) };
    }

    pub fn set_option_by_int(name: &str, value: i32) {
        let name = CString::new(name).unwrap();
        unsafe { crate::ffi::wxd_SystemOptions_SetOption_Int(name.as_ptr(), value) };
    }

    pub fn get_option_by_string(name: &str) -> Option<String> {
        let name = std::ffi::CString::new(name).unwrap();
        if !unsafe { crate::ffi::wxd_SystemOptions_HasOption(name.as_ptr()) } {
            return None;
        }
        let len = unsafe { crate::ffi::wxd_SystemOptions_GetOption_String(name.as_ptr(), std::ptr::null_mut(), 0) };
        if len < 0 {
            return None;
        }
        let mut buffer = vec![0; len as usize + 1];
        unsafe { crate::ffi::wxd_SystemOptions_GetOption_String(name.as_ptr(), buffer.as_mut_ptr(), buffer.len()) };
        Some(unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().to_string() })
    }

    pub fn get_option_by_int(name: &str) -> Option<i32> {
        let name = std::ffi::CString::new(name).unwrap();
        if unsafe { !crate::ffi::wxd_SystemOptions_HasOption(name.as_ptr()) } {
            return None;
        }
        unsafe { Some(crate::ffi::wxd_SystemOptions_GetOption_Int(name.as_ptr())) }
    }
}
