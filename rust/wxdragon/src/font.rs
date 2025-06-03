// Point, Size, etc. may not be needed directly here but good for consistency
use std::default::Default;
use std::ffi::{c_int, CStr};
use wxdragon_sys as ffi;

/// Specifies the general family category of a font.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum FontFamily {
    #[default]
    Default = 0,
    Decorative = 1,
    Roman = 2,
    Script = 3,
    Swiss = 4,
    Modern = 5,
    Teletype = 6,
    // Unknown = 7, // wxFONTFAMILY_UNKNOWN usually maps to MAX or is separate
}

impl FontFamily {
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

/// Specifies the style of the font (normal, italic, or slanted).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum FontStyle {
    #[default]
    Normal = 0,
    Italic = 1,
    Slant = 2,
}

impl FontStyle {
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

/// Specifies the weight of the font (e.g., normal, light, bold).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    #[default]
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Heavy = 900,
}

impl FontWeight {
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

/// Wrapper for wxFont.
#[derive(Clone, Debug)]
pub struct Font {
    ptr: *mut ffi::wxd_Font_t,
    owned: bool, // Track if this instance owns the pointer
}

unsafe impl Send for Font {}

impl Font {
    /// Creates a new default font.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_Font_Create() }; // Assumed to exist from lack of lint error
        Self { ptr, owned: true }
    }

    /// Adds a private font from a file path to the application's font database.
    /// Returns true on success, false on failure.
    pub fn add_private_font(path: &str) -> bool {
        if let Ok(c_path) = std::ffi::CString::new(path) {
            unsafe { ffi::wxd_Font_AddPrivateFont(c_path.as_ptr()) } // Assumed to exist
        } else {
            false
        }
    }

    /// Creates a new font with specified details.
    /// Returns `None` if the font cannot be created (e.g., invalid parameters or face name not found).
    pub fn new_with_details(
        point_size: i32,
        family: i32, // Takes i32, matching FONTFAMILY_* underlying values
        style: i32,  // Takes i32, matching FONTSTYLE_* underlying values
        weight: i32, // Takes i32, matching FONTWEIGHT_* underlying values
        underlined: bool,
        face_name: &str,
    ) -> Option<Self> {
        let c_face_name = match std::ffi::CString::new(face_name) {
            Ok(s) => s,
            Err(_) => return None,
        };
        let ptr = unsafe {
            // This FFI function seems to be available based on lack of lint errors for it.
            ffi::wxd_Font_CreateEx(
                point_size,
                family,
                style,
                weight,
                underlined,
                c_face_name.as_ptr(),
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr, owned: true })
        }
    }

    /// Create a Font wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxFont.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Font_t, owned: bool) -> Self {
        Self { ptr, owned }
    }

    /// Get the raw pointer.
    pub(crate) fn as_ptr(&self) -> *mut ffi::wxd_Font_t {
        self.ptr
    }

    /// Get the point size of the font.
    pub fn get_point_size(&self) -> i32 {
        unsafe { ffi::wxd_Font_GetPointSize(self.ptr) }
    }

    /// Get the font family.
    pub fn get_family(&self) -> FontFamily {
        let val = unsafe { ffi::wxd_Font_GetFamily(self.ptr) };
        unsafe { std::mem::transmute(val as i32) } // Ensure FFI result is i32 for transmute
    }

    /// Get the font style.
    pub fn get_style(&self) -> FontStyle {
        let val = unsafe { ffi::wxd_Font_GetStyle(self.ptr) };
        unsafe { std::mem::transmute(val as i32) }
    }

    /// Get the font weight.
    pub fn get_weight(&self) -> FontWeight {
        let val = unsafe { ffi::wxd_Font_GetWeight(self.ptr) };
        unsafe { std::mem::transmute(val as i32) }
    }

    /// Get whether the font is underlined.
    pub fn is_underlined(&self) -> bool {
        unsafe { ffi::wxd_Font_GetUnderlined(self.ptr) }
    }

    /// Get the font face name.
    pub fn get_face_name(&self) -> String {
        let mut buffer = vec![0u8; 256];
        let len = unsafe {
            ffi::wxd_Font_GetFaceName(
                self.ptr,
                buffer.as_mut_ptr() as *mut i8,
                buffer.len() as i32,
            )
        };
        if len > 0 {
            buffer.resize((len + 1) as usize, 0);
            let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
            c_str.to_string_lossy().into_owned()
        } else {
            String::new()
        }
    }

    /// Check if the font is valid.
    pub fn is_ok(&self) -> bool {
        unsafe { ffi::wxd_Font_IsOk(self.ptr) }
    }

    /// Sets the weight of the font.
    pub fn set_weight(&mut self, weight: FontWeight) {
        unsafe {
            ffi::wxd_Font_SetWeight(self.ptr, weight.as_i32());
        }
    }

    /// Creates a bold version of this font
    pub fn make_bold(&mut self) {
        self.set_weight(FontWeight::Bold);
    }

    /// Sets the point size of the font.
    pub fn set_point_size(&mut self, point_size: i32) {
        unsafe {
            ffi::wxd_Font_SetPointSize(self.ptr, point_size);
        }
    }

    /// Sets the font family.
    pub fn set_family(&mut self, family: FontFamily) {
        unsafe {
            ffi::wxd_Font_SetFamily(self.ptr, family.as_i32());
        }
    }

    /// Sets the font style.
    pub fn set_style(&mut self, style: FontStyle) {
        unsafe {
            ffi::wxd_Font_SetStyle(self.ptr, style.as_i32());
        }
    }

    /// Sets whether the font is underlined.
    pub fn set_underlined(&mut self, underlined: bool) {
        unsafe {
            ffi::wxd_Font_SetUnderlined(self.ptr, underlined);
        }
    }

    /// Creates an owned clone of this font.
    ///
    /// This creates a new C++ wxFont object that is a deep copy of the original,
    /// which is useful when you need to pass a font to a function that takes ownership.
    pub fn to_owned(&self) -> Self {
        if self.ptr.is_null() {
            return Self::new();
        }

        // Create a new Font object by cloning this one
        let new_font = Self::new_with_details(
            self.get_point_size(),
            self.get_family() as i32,
            self.get_style() as i32,
            self.get_weight() as i32,
            self.is_underlined(),
            &self.get_face_name(),
        );

        // If for some reason we couldn't clone, return a default font
        new_font.unwrap_or_default()
    }

    pub fn builder() -> FontBuilder {
        FontBuilder::default()
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            // Set the pointer to null after destroying to prevent use-after-free
            let ptr = std::mem::replace(&mut self.ptr, std::ptr::null_mut());
            unsafe {
                ffi::wxd_Font_Destroy(ptr);
            }
        }
    }
}

/// Builder for creating `Font` objects.
#[derive(Default)]
pub struct FontBuilder {
    point_size: i32,
    family: FontFamily,
    style: FontStyle,
    weight: FontWeight,
    underline: bool,
    face_name: String,
}

impl FontBuilder {
    pub fn with_point_size(mut self, size: i32) -> Self {
        self.point_size = size;
        self
    }

    pub fn with_family(mut self, family: FontFamily) -> Self {
        self.family = family;
        self
    }

    pub fn with_style(mut self, style: FontStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    pub fn with_face_name(mut self, name: &str) -> Self {
        self.face_name = name.to_string();
        self
    }

    pub fn build(self) -> Option<Font> {
        let point_size = if self.point_size == 0 {
            10
        } else {
            self.point_size
        };

        Font::new_with_details(
            point_size as c_int,
            self.family.as_i32(), // Convert enum to i32
            self.style.as_i32(),  // Convert enum to i32
            self.weight.as_i32(), // Convert enum to i32
            self.underline,
            &self.face_name,
        )
    }
}
