use wxdragon_sys as ffi;

/// Represents a date and time.
#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    raw: ffi::wxd_DateTime_t,
}

impl DateTime {
    /// Creates a new DateTime from individual components.
    /// Note: `month` is 1-12 (January = 1).
    ///
    /// Returns an invalid DateTime if any parameters are out of range:
    /// - year must be positive
    /// - month must be between 1 and 12
    /// - day must be positive and <= days in the given month/year
    /// - hour must be between 0 and 23
    /// - minute must be between 0 and 59
    /// - second must be between 0 and 59
    pub fn new(year: i32, month: u16, day: i16, hour: i16, minute: i16, second: i16) -> Self {
        // Validate parameters before trying to create a DateTime
        if year <= 0
            || !(1..=12).contains(&month)
            || !(1..=31).contains(&day)
            || !(0..24).contains(&hour)
            || !(0..60).contains(&minute)
            || !(0..60).contains(&second)
        {
            return Self::default();
        }

        // Convert 1-based month (used in Rust API) to 0-based (used in C API)
        let c_month = month - 1;
        // Create the DateTime object
        let dt = Self {
            raw: unsafe {
                ffi::wxd_DateTime_FromComponents(year, c_month, day, hour, minute, second)
            },
        };

        // Check if it's a valid date/time
        if !dt.is_valid() {
            return Self::default();
        }

        dt
    }

    /// Creates a DateTime representing the current moment.
    pub fn now() -> Self {
        unsafe { Self::from_raw(ffi::wxd_DateTime_Now()) }
    }

    /// Creates a DateTime from the raw FFI struct.
    /// This is typically used when receiving a DateTime from the C++ layer.
    pub(crate) fn from_raw(raw: ffi::wxd_DateTime_t) -> Self {
        Self { raw }
    }

    /// Returns a pointer to the raw FFI struct.
    /// This is used when passing a DateTime to the C++ layer.
    pub(crate) fn as_ptr(&self) -> *const ffi::wxd_DateTime_t {
        &self.raw as *const _
    }

    pub fn year(&self) -> i32 {
        self.raw.year
    }
    /// Gets the month (1-12, January is 1).
    pub fn month(&self) -> u16 {
        // raw.month is 0-11
        self.raw.month + 1
    }
    pub fn day(&self) -> i16 {
        self.raw.day
    }
    pub fn hour(&self) -> i16 {
        self.raw.hour
    }
    pub fn minute(&self) -> i16 {
        self.raw.minute
    }
    pub fn second(&self) -> i16 {
        self.raw.second
    }

    /// Checks if the date is valid according to wxWidgets rules.
    pub fn is_valid(&self) -> bool {
        // For year 0, it's typically invalid from our C++ side conventions
        if self.raw.year == 0 && self.raw.month == 0 && self.raw.day == 0 {
            return false;
        }

        unsafe { ffi::wxd_DateTime_IsValid(&self.raw) }
    }
}

impl Default for DateTime {
    /// Returns a default DateTime, which might be an invalid date or today's date
    /// depending on the underlying FFI::wxd_DateTime_Default() implementation.
    /// Typically this represents an uninitialized or default state.
    fn default() -> Self {
        unsafe { Self::from_raw(ffi::wxd_DateTime_Default()) }
    }
}
