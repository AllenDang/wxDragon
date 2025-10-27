use wxdragon_sys as ffi;

/// Represents a date and time (pointer-backed wxDateTime).
#[derive(Debug)]
pub struct DateTime {
    ptr: *const ffi::wxd_DateTime_t,
    owned: bool,
}

impl AsRef<*const ffi::wxd_DateTime_t> for DateTime {
    fn as_ref(&self) -> &*const ffi::wxd_DateTime_t {
        &self.ptr
    }
}

impl std::ops::Deref for DateTime {
    type Target = *const ffi::wxd_DateTime_t;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

use std::io::{Error, ErrorKind::InvalidInput};

impl TryFrom<DateTime> for *const ffi::wxd_DateTime_t {
    type Error = std::io::Error;
    fn try_from(dt: DateTime) -> Result<Self, Self::Error> {
        if dt.ptr.is_null() {
            Err(Error::new(InvalidInput, "DateTime pointer is null"))
        } else {
            Ok(dt.ptr)
        }
    }
}

impl TryFrom<DateTime> for *mut ffi::wxd_DateTime_t {
    type Error = std::io::Error;
    fn try_from(mut dt: DateTime) -> Result<Self, Self::Error> {
        if dt.ptr.is_null() {
            Err(Error::new(InvalidInput, "DateTime pointer is null"))
        } else if dt.owned {
            let ptr = dt.ptr as *mut ffi::wxd_DateTime_t;
            dt.ptr = std::ptr::null();
            Ok(ptr)
        } else {
            Err(Error::new(
                InvalidInput,
                "Cannot convert non-owned DateTime to mutable pointer",
            ))
        }
    }
}

impl From<*mut ffi::wxd_DateTime_t> for DateTime {
    fn from(ptr: *mut ffi::wxd_DateTime_t) -> Self {
        Self { ptr, owned: true }
    }
}

impl From<*const ffi::wxd_DateTime_t> for DateTime {
    fn from(ptr: *const ffi::wxd_DateTime_t) -> Self {
        Self { ptr, owned: false }
    }
}

impl DateTime {
    /// Returns whether this DateTime owns its underlying pointer.
    pub fn is_owned(&self) -> bool {
        self.owned
    }

    /// Creates a new DateTime from individual components.
    /// Note: `month` is 1-12 (January = 1).
    pub fn new(year: i32, month: u16, day: i16, hour: i16, minute: i16, second: i16) -> Self {
        if year <= 0
            || !(1..=12).contains(&month)
            || !(1..=31).contains(&day)
            || !(0..24).contains(&hour)
            || !(0..60).contains(&minute)
            || !(0..60).contains(&second)
        {
            return Self::default();
        }

        let c_month = month - 1; // convert to 0-based
        let ptr =
            unsafe { ffi::wxd_DateTime_FromComponents(year, c_month, day, hour, minute, second) };
        if ptr.is_null() {
            return Self::default();
        }
        Self { ptr, owned: true }
    }

    /// Creates a DateTime representing the current moment.
    pub fn now() -> Self {
        let ptr = unsafe { ffi::wxd_DateTime_Now() };
        if ptr.is_null() {
            Self::default()
        } else {
            Self { ptr, owned: true }
        }
    }

    pub fn year(&self) -> i32 {
        unsafe { ffi::wxd_DateTime_GetYear(self.ptr) }
    }
    /// Gets the month (1-12, January is 1).
    pub fn month(&self) -> u16 {
        // FFI returns 0-11
        unsafe { ffi::wxd_DateTime_GetMonth(self.ptr) + 1 }
    }
    pub fn day(&self) -> i16 {
        unsafe { ffi::wxd_DateTime_GetDay(self.ptr) }
    }
    pub fn hour(&self) -> i16 {
        unsafe { ffi::wxd_DateTime_GetHour(self.ptr) }
    }
    pub fn minute(&self) -> i16 {
        unsafe { ffi::wxd_DateTime_GetMinute(self.ptr) }
    }
    pub fn second(&self) -> i16 {
        unsafe { ffi::wxd_DateTime_GetSecond(self.ptr) }
    }

    /// Checks if the date is valid according to wxWidgets rules.
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::wxd_DateTime_IsValid(self.ptr) }
    }
}

impl Default for DateTime {
    fn default() -> Self {
        let ptr = unsafe { ffi::wxd_DateTime_Default() };
        Self { ptr, owned: true }
    }
}

impl Drop for DateTime {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.owned {
            unsafe { ffi::wxd_DateTime_Destroy(self.ptr) };
        }
    }
}

impl Clone for DateTime {
    fn clone(&self) -> Self {
        let ptr = unsafe { ffi::wxd_DateTime_Clone(self.ptr) };
        Self { ptr, owned: true }
    }
}
