//! Internationalization (i18n) support using wxWidgets' translations system.
//!
//! This module provides access to wxWidgets' built-in translation system,
//! which uses gettext-style .mo/.po files for message catalogs.
//!
//! # Example
//! ```rust,no_run
//! use wxdragon::prelude::*;
//!
//! // Set up translations
//! let translations = Translations::new();
//! translations.set_language(Language::French);
//!
//! // Add catalog lookup path
//! add_catalog_lookup_path_prefix("./locale");
//!
//! // Load message catalog
//! if translations.add_catalog("myapp") {
//!     // Set as global translations
//!     Translations::set_global(translations);
//! }
//!
//! // Later, translate strings
//! let hello = translate("Hello");
//! ```

use crate::language::Language;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::c_char;
use wxdragon_sys as ffi;

/// A translations manager for internationalization support.
///
/// `Translations` wraps wxWidgets' wxTranslations class, which manages
/// loading and lookup of message catalogs for UI translation.
///
/// # Singleton Pattern
/// wxWidgets uses a global translations instance. You can:
/// - Use `Translations::get()` to access the current global instance
/// - Create a new instance with `Translations::new()` and set it as global
///   with `Translations::set_global()`
///
/// # Example
/// ```rust,no_run
/// use wxdragon::prelude::*;
///
/// // Create and configure translations
/// let translations = Translations::new();
/// translations.set_language(Language::German);
/// translations.add_catalog("myapp");
///
/// // Set as the global translations instance
/// Translations::set_global(translations);
///
/// // Now translations will be used automatically
/// ```
pub struct Translations {
    ptr: *mut ffi::wxd_Translations_t,
    owned: bool,
    // Marker to make this type !Send and !Sync since wxWidgets is not thread-safe
    _marker: PhantomData<*const ()>,
}

impl Translations {
    /// Get the global translations instance.
    ///
    /// Returns `Some(Translations)` if a global instance exists, `None` otherwise.
    /// The returned instance is not owned (won't be destroyed when dropped).
    pub fn get() -> Option<Self> {
        let ptr = unsafe { ffi::wxd_Translations_Get() };
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr,
                owned: false,
                _marker: PhantomData,
            })
        }
    }

    /// Create a new translations instance.
    ///
    /// The instance is owned and will be destroyed when dropped,
    /// unless it's set as the global instance via `set_global()`.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_Translations_Create() };
        Self {
            ptr,
            owned: true,
            _marker: PhantomData,
        }
    }

    /// Set this translations instance as the global instance.
    ///
    /// This transfers ownership to wxWidgets. The instance will be
    /// managed by wxWidgets and destroyed when a new global instance
    /// is set or the application exits.
    ///
    /// After calling this, the `Translations` instance no longer owns
    /// the underlying pointer.
    pub fn set_global(mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_Translations_Set(self.ptr) };
            self.owned = false;
        }
    }

    /// Set the language for translations using a `Language` enum value.
    ///
    /// This determines which message catalog language is used.
    pub fn set_language(&self, lang: Language) {
        if self.ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Translations_SetLanguage(self.ptr, lang.as_i32()) };
    }

    /// Set the language for translations using a language string.
    ///
    /// The string should be a language code like "en", "en_US", "fr_FR", etc.
    pub fn set_language_str(&self, lang: &str) {
        if self.ptr.is_null() {
            return;
        }
        let c_lang = match CString::new(lang) {
            Ok(s) => s,
            Err(_) => return,
        };
        unsafe { ffi::wxd_Translations_SetLanguageStr(self.ptr, c_lang.as_ptr()) };
    }

    /// Add a message catalog for the given domain.
    ///
    /// The domain is typically the application or library name.
    /// Returns `true` if the catalog was successfully loaded.
    ///
    /// Uses `Language::English` as the default message ID language.
    pub fn add_catalog(&self, domain: &str) -> bool {
        self.add_catalog_with_lang(domain, Language::English)
    }

    /// Add a message catalog with explicit source language.
    ///
    /// The `msg_id_language` specifies what language the original
    /// strings in the source code are in. This helps wxWidgets
    /// find the best translation when the exact target language
    /// isn't available.
    pub fn add_catalog_with_lang(&self, domain: &str, msg_id_language: Language) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_domain = match CString::new(domain) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Translations_AddCatalog(self.ptr, c_domain.as_ptr(), msg_id_language.as_i32()) }
    }

    /// Add the standard wxWidgets message catalog.
    ///
    /// This loads wxWidgets' own translations for standard UI elements
    /// like "OK", "Cancel", etc.
    pub fn add_std_catalog(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Translations_AddStdCatalog(self.ptr) }
    }

    /// Check if a catalog for the given domain is loaded.
    pub fn is_loaded(&self, domain: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_domain = match CString::new(domain) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Translations_IsLoaded(self.ptr, c_domain.as_ptr()) }
    }

    /// Get a translated string.
    ///
    /// Returns the translated string if found, otherwise `None`.
    ///
    /// # Arguments
    /// * `orig` - The original string to translate
    /// * `domain` - Optional domain to search in (use empty string for default)
    pub fn get_string(&self, orig: &str, domain: &str) -> Option<String> {
        if self.ptr.is_null() {
            return None;
        }
        let c_orig = CString::new(orig).ok()?;
        let c_domain = CString::new(domain).ok()?;

        // First get the length
        let len = unsafe {
            ffi::wxd_Translations_GetTranslatedString(self.ptr, c_orig.as_ptr(), c_domain.as_ptr(), std::ptr::null_mut(), 0)
        };

        if len < 0 {
            return None;
        }

        // Now get the actual string
        let mut buf: Vec<c_char> = vec![0; len as usize + 1];
        unsafe {
            ffi::wxd_Translations_GetTranslatedString(self.ptr, c_orig.as_ptr(), c_domain.as_ptr(), buf.as_mut_ptr(), buf.len())
        };

        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
    }

    /// Get a plural-form translated string.
    ///
    /// Returns the appropriate plural form based on the count `n`.
    ///
    /// # Arguments
    /// * `singular` - The singular form of the string
    /// * `plural` - The plural form of the string
    /// * `n` - The count used to determine plural form
    /// * `domain` - Optional domain to search in (use empty string for default)
    pub fn get_plural_string(&self, singular: &str, plural: &str, n: u32, domain: &str) -> Option<String> {
        if self.ptr.is_null() {
            return None;
        }
        let c_singular = CString::new(singular).ok()?;
        let c_plural = CString::new(plural).ok()?;
        let c_domain = CString::new(domain).ok()?;

        // First get the length
        let len = unsafe {
            ffi::wxd_Translations_GetTranslatedPluralString(
                self.ptr,
                c_singular.as_ptr(),
                c_plural.as_ptr(),
                n,
                c_domain.as_ptr(),
                std::ptr::null_mut(),
                0,
            )
        };

        if len < 0 {
            return None;
        }

        // Now get the actual string
        let mut buf: Vec<c_char> = vec![0; len as usize + 1];
        unsafe {
            ffi::wxd_Translations_GetTranslatedPluralString(
                self.ptr,
                c_singular.as_ptr(),
                c_plural.as_ptr(),
                n,
                c_domain.as_ptr(),
                buf.as_mut_ptr(),
                buf.len(),
            )
        };

        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
    }

    /// Get a header value from a catalog.
    ///
    /// Standard headers include "Content-Type", "Plural-Forms", etc.
    pub fn get_header_value(&self, header: &str, domain: &str) -> Option<String> {
        if self.ptr.is_null() {
            return None;
        }
        let c_header = CString::new(header).ok()?;
        let c_domain = CString::new(domain).ok()?;

        let len = unsafe {
            ffi::wxd_Translations_GetHeaderValue(self.ptr, c_header.as_ptr(), c_domain.as_ptr(), std::ptr::null_mut(), 0)
        };

        if len < 0 {
            return None;
        }

        let mut buf: Vec<c_char> = vec![0; len as usize + 1];
        unsafe {
            ffi::wxd_Translations_GetHeaderValue(self.ptr, c_header.as_ptr(), c_domain.as_ptr(), buf.as_mut_ptr(), buf.len())
        };

        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
    }

    /// Get the best available translation for a domain.
    ///
    /// Returns the language code of the best available translation,
    /// or `None` if no translations are available.
    pub fn get_best_translation(&self, domain: &str) -> Option<String> {
        self.get_best_translation_with_lang(domain, Language::English)
    }

    /// Get the best available translation for a domain with explicit source language.
    pub fn get_best_translation_with_lang(&self, domain: &str, msg_id_language: Language) -> Option<String> {
        if self.ptr.is_null() {
            return None;
        }
        let c_domain = CString::new(domain).ok()?;

        let len = unsafe {
            ffi::wxd_Translations_GetBestTranslation(
                self.ptr,
                c_domain.as_ptr(),
                msg_id_language.as_i32(),
                std::ptr::null_mut(),
                0,
            )
        };

        if len < 0 {
            return None;
        }

        let mut buf: Vec<c_char> = vec![0; len as usize + 1];
        unsafe {
            ffi::wxd_Translations_GetBestTranslation(
                self.ptr,
                c_domain.as_ptr(),
                msg_id_language.as_i32(),
                buf.as_mut_ptr(),
                buf.len(),
            )
        };

        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
    }

    /// Get all available translations for a domain.
    ///
    /// Returns a list of language codes for which translations are available.
    pub fn get_available_translations(&self, domain: &str) -> Vec<String> {
        if self.ptr.is_null() {
            return Vec::new();
        }
        let c_domain = match CString::new(domain) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        // First get the count
        let count =
            unsafe { ffi::wxd_Translations_GetAvailableTranslations(self.ptr, c_domain.as_ptr(), std::ptr::null_mut(), 0, 0) };

        if count <= 0 {
            return Vec::new();
        }

        // Allocate buffers
        let string_buf_len: usize = 32; // Language codes are short
        let mut buffers: Vec<Vec<c_char>> = (0..count).map(|_| vec![0 as c_char; string_buf_len]).collect();
        let mut ptrs: Vec<*mut c_char> = buffers.iter_mut().map(|b| b.as_mut_ptr()).collect();

        unsafe {
            ffi::wxd_Translations_GetAvailableTranslations(
                self.ptr,
                c_domain.as_ptr(),
                ptrs.as_mut_ptr(),
                count as usize,
                string_buf_len,
            )
        };

        // Convert to Strings
        buffers
            .iter()
            .filter_map(|buf| {
                let cstr = unsafe { CStr::from_ptr(buf.as_ptr()) };
                let s = cstr.to_string_lossy().to_string();
                if s.is_empty() { None } else { Some(s) }
            })
            .collect()
    }
}

impl Default for Translations {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Translations {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            unsafe { ffi::wxd_Translations_Destroy(self.ptr) };
        }
    }
}

/// Add a catalog lookup path prefix.
///
/// This adds a directory to search for translation files (.mo files).
/// The path should contain subdirectories named by language code
/// (e.g., "fr", "de", "es") containing the .mo files.
///
/// # Example
/// ```rust,no_run
/// use wxdragon::translations::add_catalog_lookup_path_prefix;
///
/// // Add ./locale as a search path
/// // Translation files should be in ./locale/fr/LC_MESSAGES/myapp.mo etc.
/// add_catalog_lookup_path_prefix("./locale");
/// ```
pub fn add_catalog_lookup_path_prefix(prefix: &str) {
    let c_prefix = match CString::new(prefix) {
        Ok(s) => s,
        Err(_) => return,
    };
    unsafe { ffi::wxd_FileTranslationsLoader_AddCatalogLookupPathPrefix(c_prefix.as_ptr()) };
}

/// Translate a string using the global translations instance.
///
/// Returns the translated string if translations are set up and
/// a translation exists, otherwise returns the original string.
///
/// # Example
/// ```rust,no_run
/// use wxdragon::translations::translate;
///
/// let hello = translate("Hello");
/// ```
pub fn translate(s: &str) -> String {
    if let Some(translations) = Translations::get()
        && let Some(translated) = translations.get_string(s, "")
    {
        return translated;
    }
    s.to_string()
}

/// Translate a plural string using the global translations instance.
///
/// Returns the appropriate plural form based on the count `n`.
/// If translations aren't available, returns `singular` if n == 1,
/// otherwise returns `plural`.
///
/// # Example
/// ```rust,no_run
/// use wxdragon::translations::translate_plural;
///
/// let file_count = 5;
/// let msg = translate_plural("1 item selected", "%d items selected", file_count);
/// ```
pub fn translate_plural(singular: &str, plural: &str, n: u32) -> String {
    if let Some(translations) = Translations::get()
        && let Some(translated) = translations.get_plural_string(singular, plural, n, "")
    {
        return translated;
    }
    if n == 1 { singular.to_string() } else { plural.to_string() }
}
