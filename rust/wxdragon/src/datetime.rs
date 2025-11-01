use std::marker::PhantomData;
use std::rc::Rc;
use wxdragon_sys as ffi;

/// Represents a date and time (pointer-backed wxDateTime).
#[derive(Debug)]
pub struct DateTime {
    ptr: *mut ffi::wxd_DateTime_t,
    owned: bool,
    // Prevent Send/Sync: wxWidgets objects are not thread-safe and must stay on UI thread.
    _nosend_nosync: PhantomData<Rc<()>>,
}

use std::io::{Error, ErrorKind::InvalidInput};

impl TryFrom<DateTime> for *const ffi::wxd_DateTime_t {
    type Error = std::io::Error;
    fn try_from(dt: DateTime) -> Result<Self, Self::Error> {
        if dt.ptr.is_null() {
            Err(Error::new(InvalidInput, "DateTime pointer is null"))
        } else if dt.owned {
            Err(Error::new(InvalidInput, "Cannot convert owned DateTime to const pointer"))
        } else {
            let ptr = dt.ptr as *const ffi::wxd_DateTime_t;
            std::mem::forget(dt); // prevent drop
            Ok(ptr)
        }
    }
}

impl TryFrom<DateTime> for *mut ffi::wxd_DateTime_t {
    type Error = std::io::Error;
    fn try_from(dt: DateTime) -> Result<Self, Self::Error> {
        if dt.ptr.is_null() {
            Err(Error::new(InvalidInput, "DateTime pointer is null"))
        } else if dt.owned {
            let ptr = dt.ptr;
            std::mem::forget(dt); // prevent drop
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
        assert!(!ptr.is_null(), "invalid null pointer passed to DateTime::from");
        Self {
            ptr,
            owned: true,
            _nosend_nosync: PhantomData,
        }
    }
}

impl From<*const ffi::wxd_DateTime_t> for DateTime {
    fn from(ptr: *const ffi::wxd_DateTime_t) -> Self {
        assert!(!ptr.is_null(), "invalid null pointer passed to DateTime::from");
        Self {
            ptr: ptr as *mut _,
            owned: false,
            _nosend_nosync: PhantomData,
        }
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
        let ptr = unsafe { ffi::wxd_DateTime_FromComponents(year, c_month, day, hour, minute, second) };
        if ptr.is_null() {
            return Self::default();
        }
        Self {
            ptr,
            owned: true,
            _nosend_nosync: PhantomData,
        }
    }

    /// Creates a DateTime representing the current moment.
    pub fn now() -> Self {
        let ptr = unsafe { ffi::wxd_DateTime_Now() };
        if ptr.is_null() {
            Self::default()
        } else {
            Self {
                ptr,
                owned: true,
                _nosend_nosync: PhantomData,
            }
        }
    }

    /// Returns a const raw pointer to the underlying wxd_DateTime_t.
    ///
    /// Ownership notes:
    /// - This does not transfer ownership; the pointer is only valid while this `DateTime` (or another owner) keeps it alive.
    /// - Do not destroy this pointer yourself; the owner handles destruction or transfer via `into_raw_mut`.
    pub fn as_const_ptr(&self) -> *const ffi::wxd_DateTime_t {
        self.ptr as *const _
    }

    /// Returns a mutable raw pointer to the underlying wxd_DateTime_t.
    ///
    /// Use with care; prefer safe methods when possible.
    ///
    /// Ownership notes:
    /// - This does not transfer ownership. If you mutate through this pointer, ensure exclusive access and maintain invariants.
    /// - Do not destroy the returned pointer; use `into_raw_mut` to transfer ownership when needed.
    pub fn as_mut_ptr(&mut self) -> *mut ffi::wxd_DateTime_t {
        self.ptr
    }

    /// Consumes self and returns a raw mutable pointer, transferring ownership to the caller.
    ///
    /// After calling this, you must NOT use the original `DateTime` again.
    ///
    /// Caller responsibilities:
    /// - You now own the pointer and must destroy it exactly once with `wxd_DateTime_Destroy`.
    /// - Ensure no use-after-free.
    pub fn into_raw_mut(self) -> *mut ffi::wxd_DateTime_t {
        self.try_into()
            .expect("into_raw_mut can only be called on owning DateTime instances")
    }

    /// Consumes a borrowed (non-owning) wrapper and returns a raw const pointer without taking ownership.
    ///
    /// Panics if called on an owning wrapper to avoid leaking the owned resource.
    ///
    /// Caller responsibilities:
    /// - This does NOT transfer ownership; do not destroy the returned pointer.
    /// - The pointer remains valid only while the original owner keeps it alive.
    pub fn into_raw_const(self) -> *const ffi::wxd_DateTime_t {
        self.try_into()
            .expect("into_raw_const must only be used on non-owning (borrowed) wrappers")
    }

    pub fn year(&self) -> i32 {
        unsafe { ffi::wxd_DateTime_GetYear(self.as_const_ptr()) }
    }
    /// Gets the month (1-12, January is 1).
    pub fn month(&self) -> u16 {
        // FFI returns 0-11
        unsafe { ffi::wxd_DateTime_GetMonth(self.as_const_ptr()) + 1 }
    }
    pub fn day(&self) -> i16 {
        unsafe { ffi::wxd_DateTime_GetDay(self.as_const_ptr()) }
    }
    pub fn hour(&self) -> i16 {
        unsafe { ffi::wxd_DateTime_GetHour(self.as_const_ptr()) }
    }
    pub fn minute(&self) -> i16 {
        unsafe { ffi::wxd_DateTime_GetMinute(self.as_const_ptr()) }
    }
    pub fn second(&self) -> i16 {
        unsafe { ffi::wxd_DateTime_GetSecond(self.as_const_ptr()) }
    }

    /// Checks if the date is valid according to wxWidgets rules.
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::wxd_DateTime_IsValid(self.as_const_ptr()) }
    }
}

impl Default for DateTime {
    fn default() -> Self {
        let ptr = unsafe { ffi::wxd_DateTime_Default() };
        assert!(!ptr.is_null(), "Failed to create default wxDateTime");
        Self {
            ptr,
            owned: true,
            _nosend_nosync: PhantomData,
        }
    }
}

impl Drop for DateTime {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.owned {
            unsafe { ffi::wxd_DateTime_Destroy(self.as_mut_ptr()) };
            self.ptr = std::ptr::null_mut();
        }
    }
}

impl Clone for DateTime {
    fn clone(&self) -> Self {
        let ptr = unsafe { ffi::wxd_DateTime_Clone(self.as_const_ptr()) };
        assert!(!ptr.is_null(), "Failed to clone wxDateTime");
        Self {
            ptr,
            owned: true,
            _nosend_nosync: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DateTime;
    use wxdragon_sys as ffi;

    #[test]
    fn datetime_now_clone_into_raw_then_destroy() {
        // Create an owned DateTime representing now
        let dt = DateTime::now();
        assert!(dt.is_owned(), "DateTime::now should produce an owned instance");
        assert!(dt.is_valid(), "DateTime::now should be valid");

        // Clone the DateTime; clone is also owned
        let clone = dt.clone();
        assert!(clone.is_owned(), "cloned DateTime should be owned");
        assert!(clone.is_valid(), "cloned DateTime should be valid");

        // Transfer ownership out of the clone and destroy manually
        let raw = clone.into_raw_mut();
        assert!(!raw.is_null(), "raw pointer from into_raw_mut should be non-null");
        unsafe { ffi::wxd_DateTime_Destroy(raw) };

        // When this test ends, `dt` will be dropped and should destroy its own handle.
        // If ownership transfer or Drop were incorrect, this test would double-free or leak.
    }
}
