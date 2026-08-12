//! Access to wxWidgets' global database of named stock colours.

use crate::color::Colour;
use crate::utils::ArrayString;
use std::ffi::CString;
use wxdragon_sys as ffi;

/// Selects which built-in colour scheme [`ColourDatabase`] uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColourDatabaseScheme {
    /// CSS/web colour names (e.g. `"MEDIUMSEAGREEN"`). The default since wxWidgets 3.3.0.
    #[default]
    Css,
    /// The legacy colour names used by wxWidgets before 3.3.0 (e.g. `"MEDIUM SEA GREEN"`).
    Traditional,
}

impl ColourDatabaseScheme {
    fn to_raw(self) -> ffi::wxd_ColourDatabaseScheme {
        match self {
            ColourDatabaseScheme::Css => ffi::wxd_ColourDatabaseScheme_WXD_COLOUR_DATABASE_SCHEME_CSS,
            ColourDatabaseScheme::Traditional => ffi::wxd_ColourDatabaseScheme_WXD_COLOUR_DATABASE_SCHEME_TRADITIONAL,
        }
    }
}

/// Provides access to wxWidgets' global database of named stock colours (e.g. looking up
/// `"MEDIUM FOREST GREEN"` or finding the name for an RGB value), backed by `wxTheColourDatabase`.
pub struct ColourDatabase;

impl ColourDatabase {
    /// Selects which built-in colour scheme the database's names use.
    ///
    /// # Example
    /// ```rust,no_run
    /// use wxdragon::utils::{ColourDatabase, ColourDatabaseScheme};
    ///
    /// ColourDatabase::use_scheme(ColourDatabaseScheme::Traditional);
    /// ```
    pub fn use_scheme(scheme: ColourDatabaseScheme) {
        unsafe { ffi::wxd_ColourDatabase_UseScheme(scheme.to_raw()) };
    }

    /// Looks up a colour by name (case-insensitive), returning `None` if the name isn't known.
    ///
    /// # Example
    /// ```rust,no_run
    /// use wxdragon::utils::ColourDatabase;
    ///
    /// if let Some(colour) = ColourDatabase::find("MEDIUM FOREST GREEN") {
    ///     println!("r={} g={} b={}", colour.r, colour.g, colour.b);
    /// }
    /// ```
    pub fn find(name: &str) -> Option<Colour> {
        let c_name = CString::new(name).ok()?;
        let mut raw = ffi::wxd_Colour_t { r: 0, g: 0, b: 0, a: 0 };
        let found = unsafe { ffi::wxd_ColourDatabase_Find(c_name.as_ptr(), &mut raw) };
        if found { Some(Colour::from(raw)) } else { None }
    }

    /// Looks up the name for a colour, returning `None` if it has no name in the database.
    ///
    /// # Example
    /// ```rust,no_run
    /// use wxdragon::prelude::*;
    /// use wxdragon::utils::ColourDatabase;
    ///
    /// let name = ColourDatabase::find_name(Colour::new(255, 0, 0, 255));
    /// ```
    pub fn find_name(colour: Colour) -> Option<String> {
        let raw = colour.to_raw();
        let len = unsafe { ffi::wxd_ColourDatabase_FindName(raw, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return None;
        }
        let mut buffer = vec![0u8; len as usize + 1];
        unsafe { ffi::wxd_ColourDatabase_FindName(raw, buffer.as_mut_ptr() as *mut std::os::raw::c_char, buffer.len()) };
        Some(unsafe {
            std::ffi::CStr::from_ptr(buffer.as_ptr() as *const std::os::raw::c_char)
                .to_string_lossy()
                .to_string()
        })
    }

    /// Adds a named colour to the database, or overwrites the colour for an existing name.
    ///
    /// # Example
    /// ```rust,no_run
    /// use wxdragon::prelude::*;
    /// use wxdragon::utils::ColourDatabase;
    ///
    /// ColourDatabase::add_colour("BRAND PURPLE", Colour::new(128, 0, 200, 255));
    /// ```
    pub fn add_colour(name: &str, colour: Colour) {
        let Ok(c_name) = CString::new(name) else {
            return;
        };
        unsafe { ffi::wxd_ColourDatabase_AddColour(c_name.as_ptr(), colour.to_raw()) };
    }

    /// Returns the names of all colours currently known to the database.
    pub fn all_names() -> Vec<String> {
        let ptr = unsafe { ffi::wxd_ColourDatabase_GetAllNames() };
        if ptr.is_null() {
            return Vec::new();
        }
        let names: ArrayString = ptr.into();
        names.get_strings()
    }
}
