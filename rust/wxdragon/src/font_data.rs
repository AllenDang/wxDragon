use crate::color::Colour;
use crate::font::Font;
use wxdragon_sys as ffi;

/// Wrapper for wxFontData, used primarily with FontDialog
pub struct FontData {
    pub(crate) ptr: *mut ffi::wxd_FontData_t,
}

impl FontData {
    /// Create a new FontData instance with default values
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_FontData_Create() };
        Self { ptr }
    }

    /// Enable or disable font effects (underline, strikethrough, etc.)
    pub fn with_enable_effects(mut self, enable: bool) -> Self {
        self.set_enable_effects(enable);
        self
    }

    /// Set whether font effects (underline, strikethrough, etc.) are enabled
    pub fn set_enable_effects(&mut self, enable: bool) {
        unsafe {
            ffi::wxd_FontData_EnableEffects(self.ptr, enable);
        }
    }

    /// Check if font effects are enabled
    pub fn get_enable_effects(&self) -> bool {
        unsafe { ffi::wxd_FontData_GetEnableEffects(self.ptr) }
    }

    /// Set the initial font to be selected in the dialog
    pub fn with_initial_font(mut self, font: &Font) -> Self {
        self.set_initial_font(font);
        self
    }

    /// Set the initial font to be selected in the dialog
    pub fn set_initial_font(&mut self, font: &Font) {
        unsafe {
            ffi::wxd_FontData_SetInitialFont(self.ptr, font.as_ptr());
        }
    }

    /// Get the chosen colour from the font dialog
    pub fn get_chosen_colour(&self) -> Option<Colour> {
        let c = unsafe { ffi::wxd_FontData_GetChosenColour(self.ptr) };
        Some(Colour::new(c.r, c.g, c.b, c.a))
    }

    /// Set the initial colour to be selected in the dialog
    pub fn set_colour(&mut self, colour: &Colour) {
        unsafe {
            ffi::wxd_FontData_SetColour(self.ptr, colour.to_raw());
        }
    }

    /// Get the encoding
    pub fn get_encoding(&self) -> i32 {
        unsafe { ffi::wxd_FontData_GetEncoding(self.ptr) }
    }

    /// Set the encoding
    pub fn set_encoding(&mut self, encoding: i32) {
        unsafe {
            ffi::wxd_FontData_SetEncoding(self.ptr, encoding);
        }
    }

    /// Get the raw pointer, used for passing to FontDialog
    pub(crate) fn as_ptr(&self) -> *mut ffi::wxd_FontData_t {
        self.ptr
    }
}

impl Default for FontData {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FontData {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::wxd_FontData_Destroy(self.ptr);
            }
        }
    }
}

// Don't implement Send/Sync as this contains a raw pointer
// that should not be shared between threads
