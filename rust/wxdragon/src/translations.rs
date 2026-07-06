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
use crate::utils::ArrayString;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};
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

    /// Install a custom [`TranslationsLoader`], replacing the default
    /// file-based loader.
    ///
    /// This lets an application supply catalog data from anywhere (e.g. bytes
    /// embedded in the binary) instead of `.mo` files on disk. wxWidgets uses a
    /// single loader, so this replaces any previously installed one; ownership
    /// of `loader` is transferred to wxWidgets, which drops it when the
    /// translations object is destroyed or another loader is installed.
    ///
    /// Call this before [`add_catalog`](Self::add_catalog) /
    /// [`add_std_catalog`](Self::add_std_catalog): loading a catalog first
    /// consults [`TranslationsLoader::available_translations`] to pick the best
    /// language, then calls [`TranslationsLoader::load_catalog`].
    pub fn set_loader<L: TranslationsLoader + 'static>(&self, loader: L) {
        if self.ptr.is_null() {
            return;
        }
        // Double-box: `Box<dyn Trait>` is a fat pointer, so box it again to get
        // a thin `*mut c_void` we can hand across FFI. Freed by `loader_destroy`.
        let user_data = Box::into_raw(Box::new(Box::new(loader) as Box<dyn TranslationsLoader>)) as *mut c_void;
        unsafe { ffi::wxd_Translations_SetRustLoader(self.ptr, &LOADER_VTABLE as *const _, user_data) };
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

/// A source of translation catalogs for [`Translations::set_loader`].
///
/// Implement this to feed wxWidgets catalog data from somewhere other than
/// `.mo` files on disk — for example bytes embedded in the binary. wxWidgets
/// asks the loader for the languages a domain supports before requesting the
/// catalog bytes for the chosen language.
pub trait TranslationsLoader {
    /// Return the raw `.mo` catalog bytes for `(domain, lang)`, or `None` if
    /// unavailable.
    ///
    /// Returns a [`Cow`] so a loader can hand back either borrowed bytes it
    /// already holds (e.g. `Cow::Borrowed(b)` over `&'static` embedded data —
    /// zero-copy) or freshly-computed owned bytes (e.g. `Cow::Owned(vec)` after
    /// decompressing/decrypting). The bytes only need to live until this call
    /// returns; wxWidgets parses them synchronously and does not retain the
    /// buffer.
    fn load_catalog(&self, domain: &str, lang: &str) -> Option<Cow<'_, [u8]>>;

    /// Return the language codes for which `domain` has a catalog. Consulted
    /// before [`load_catalog`](Self::load_catalog) to pick the best language;
    /// returning an empty list means the domain resolves to nothing.
    fn available_translations(&self, domain: &str) -> Vec<String>;
}

// The `user_data` handed to the C++ trampoline is a `*mut Box<dyn TranslationsLoader>`
// (a thin pointer to the fat trait-object box). These trampolines recover it and
// forward to the trait methods.

unsafe extern "C" fn loader_load_catalog(
    user_data: *mut c_void,
    domain: *const c_char,
    lang: *const c_char,
    sink: *mut c_void,
    emit: Option<unsafe extern "C" fn(*mut c_void, *const u8, usize)>,
) -> bool {
    if user_data.is_null() || domain.is_null() || lang.is_null() {
        return false;
    }
    let Some(emit) = emit else { return false };
    unsafe {
        let loader = &**(user_data as *mut Box<dyn TranslationsLoader>);
        let domain = CStr::from_ptr(domain).to_string_lossy();
        let lang = CStr::from_ptr(lang).to_string_lossy();
        match loader.load_catalog(&domain, &lang) {
            Some(bytes) => {
                // `bytes` alive across the call; C++ consumes them inside `emit`
                // (see the wxd_TranslationsCatalogSink contract).
                emit(sink, bytes.as_ptr(), bytes.len());
                true
            }
            None => false,
        }
    }
}

unsafe extern "C" fn loader_available(user_data: *mut c_void, domain: *const c_char, out: *mut ffi::wxd_ArrayString_t) {
    if user_data.is_null() || domain.is_null() || out.is_null() {
        return;
    }
    unsafe {
        let loader = &**(user_data as *mut Box<dyn TranslationsLoader>);
        let domain = CStr::from_ptr(domain).to_string_lossy();
        // Borrow (non-owning) the C++-owned array and reuse the shared wrapper's
        // CString + Add loop instead of hand-rolling it.
        ArrayString::from(out as *const ffi::wxd_ArrayString_t).add_many(&loader.available_translations(&domain));
    }
}

unsafe extern "C" fn loader_destroy(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    drop(unsafe { Box::from_raw(user_data as *mut Box<dyn TranslationsLoader>) });
}

static LOADER_VTABLE: ffi::wxd_RustTranslationsLoader_vtable = ffi::wxd_RustTranslationsLoader_vtable {
    load_catalog: Some(loader_load_catalog),
    available: Some(loader_available),
    destroy: Some(loader_destroy),
};

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

/// Information about a language.
///
/// Wraps `wxLanguageInfo`. This structure provides details about a language
/// supported by wxWidgets, such as its description and canonical name.
#[derive(Clone, Copy)]
pub struct LanguageInfo {
    ptr: *const ffi::wxd_LanguageInfo_t,
}

impl LanguageInfo {
    /// Get the user-readable description of the language (e.g. "French").
    pub fn description(&self) -> String {
        if self.ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_LanguageInfo_GetDescription(self.ptr, std::ptr::null_mut(), 0) };
        if len < 0 {
            return String::new();
        }
        let mut buf = vec![0u8; len as usize + 1];
        unsafe {
            ffi::wxd_LanguageInfo_GetDescription(self.ptr, buf.as_mut_ptr() as *mut _, buf.len());
        }
        // Remove null terminator
        if let Some(last) = buf.last()
            && *last == 0
        {
            buf.pop();
        }
        String::from_utf8_lossy(&buf).to_string()
    }

    /// Get the canonical name of the language (e.g. "fr_FR").
    pub fn canonical_name(&self) -> String {
        if self.ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_LanguageInfo_GetCanonicalName(self.ptr, std::ptr::null_mut(), 0) };
        if len < 0 {
            return String::new();
        }
        let mut buf = vec![0u8; len as usize + 1];
        unsafe {
            ffi::wxd_LanguageInfo_GetCanonicalName(self.ptr, buf.as_mut_ptr() as *mut _, buf.len());
        }
        if let Some(last) = buf.last()
            && *last == 0
        {
            buf.pop();
        }
        String::from_utf8_lossy(&buf).to_string()
    }

    /// Get the native description of the language (e.g. "Français").
    pub fn native_description(&self) -> String {
        if self.ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_LanguageInfo_GetDescriptionNative(self.ptr, std::ptr::null_mut(), 0) };
        if len < 0 {
            return String::new();
        }
        let mut buf = vec![0u8; len as usize + 1];
        unsafe {
            ffi::wxd_LanguageInfo_GetDescriptionNative(self.ptr, buf.as_mut_ptr() as *mut _, buf.len());
        }
        // Remove null terminator
        if let Some(last) = buf.last()
            && *last == 0
        {
            buf.pop();
        }
        String::from_utf8_lossy(&buf).to_string()
    }
}

/// Locale-related helper functions.
///
/// Provides access to wxWidgets' locale database to look up language names and information.
pub struct Locale;

impl Locale {
    /// Get the English name of the given language (e.g. "French").
    pub fn get_language_name(lang: Language) -> Option<String> {
        let len = unsafe { ffi::wxd_Locale_GetLanguageName(lang.as_i32(), std::ptr::null_mut(), 0) };
        if len < 0 {
            return None;
        }
        let mut buf = vec![0u8; len as usize + 1];
        unsafe {
            ffi::wxd_Locale_GetLanguageName(lang.as_i32(), buf.as_mut_ptr() as *mut _, buf.len());
        }
        if let Some(last) = buf.last()
            && *last == 0
        {
            buf.pop();
        }
        Some(String::from_utf8_lossy(&buf).to_string())
    }

    /// Get the canonical name of the given language (e.g. "fr_FR").
    pub fn get_language_canonical_name(lang: Language) -> Option<String> {
        let len = unsafe { ffi::wxd_Locale_GetLanguageCanonicalName(lang.as_i32(), std::ptr::null_mut(), 0) };
        if len < 0 {
            return None;
        }
        let mut buf = vec![0u8; len as usize + 1];
        unsafe {
            ffi::wxd_Locale_GetLanguageCanonicalName(lang.as_i32(), buf.as_mut_ptr() as *mut _, buf.len());
        }
        if let Some(last) = buf.last()
            && *last == 0
        {
            buf.pop();
        }
        Some(String::from_utf8_lossy(&buf).to_string())
    }

    /// Find language info from a locale string (e.g. "fr", "en_US").
    pub fn find_language_info(locale: &str) -> Option<LanguageInfo> {
        let c_locale = CString::new(locale).ok()?;
        let ptr = unsafe { ffi::wxd_Locale_FindLanguageInfo(c_locale.as_ptr()) };
        if ptr.is_null() { None } else { Some(LanguageInfo { ptr }) }
    }

    /// Get the language info for the given language id.
    pub fn get_language_info(lang: Language) -> Option<LanguageInfo> {
        let ptr = unsafe { ffi::wxd_Locale_GetLanguageInfo(lang.as_i32()) };
        if ptr.is_null() { None } else { Some(LanguageInfo { ptr }) }
    }

    /// Get the system default language.
    pub fn get_system_language() -> Language {
        let lang_id = unsafe { ffi::wxd_Locale_GetSystemLanguage() };
        Language::from_i32(lang_id).unwrap_or(Language::Unknown)
    }
}

/// Represents a UI locale.
///
/// Wraps `wxUILocale`. This class provides access to the current UI locale settings.
pub struct UILocale {
    ptr: *mut ffi::wxd_UILocale_t,
}

impl UILocale {
    /// Get the object corresponding to the current locale.
    pub fn get_current() -> Self {
        let ptr = unsafe { ffi::wxd_UILocale_GetCurrent() };
        Self { ptr }
    }

    /// Get the locale name.
    pub fn get_name(&self) -> String {
        if self.ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_UILocale_GetName(self.ptr, std::ptr::null_mut(), 0) };
        if len < 0 {
            return String::new();
        }
        let mut buf = vec![0u8; len as usize + 1];
        unsafe {
            ffi::wxd_UILocale_GetName(self.ptr, buf.as_mut_ptr() as *mut _, buf.len());
        }
        // Remove null terminator
        if let Some(last) = buf.last()
            && *last == 0
        {
            buf.pop();
        }
        String::from_utf8_lossy(&buf).to_string()
    }

    /// Get the language info for this locale.
    pub fn get_info(&self) -> Option<LanguageInfo> {
        if self.ptr.is_null() {
            return None;
        }
        let lang_id = unsafe { ffi::wxd_UILocale_GetLanguage(self.ptr) };
        Locale::get_language_info(Language::from_i32(lang_id).unwrap_or(Language::Unknown))
    }
}

impl Drop for UILocale {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_UILocale_Destroy(self.ptr) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal little-endian gettext `.mo` catalog from `entries`
    /// (which must be sorted by original string). Enough for wxWidgets to parse
    /// and look up messages.
    fn make_mo(entries: &[(&str, &str)]) -> Vec<u8> {
        let n = entries.len();
        let header_size = 28usize; // 7 * u32
        let orig_table = header_size;
        let trans_table = orig_table + n * 8;
        let data_start = trans_table + n * 8;

        let mut strings: Vec<u8> = Vec::new();
        let mut orig_tab: Vec<(usize, usize)> = Vec::new();
        for (o, _) in entries {
            let off = data_start + strings.len();
            orig_tab.push((o.len(), off));
            strings.extend_from_slice(o.as_bytes());
            strings.push(0);
        }
        let mut trans_tab: Vec<(usize, usize)> = Vec::new();
        for (_, t) in entries {
            let off = data_start + strings.len();
            trans_tab.push((t.len(), off));
            strings.extend_from_slice(t.as_bytes());
            strings.push(0);
        }

        let mut out: Vec<u8> = Vec::new();
        let mut w = |v: usize| out.extend_from_slice(&(v as u32).to_le_bytes());
        w(0x950412de); // magic
        w(0); // revision
        w(n); // number of strings
        w(orig_table);
        w(trans_table);
        w(0); // hash table size
        w(0); // hash table offset
        for (len, off) in orig_tab.iter().chain(trans_tab.iter()) {
            out.extend_from_slice(&(*len as u32).to_le_bytes());
            out.extend_from_slice(&(*off as u32).to_le_bytes());
        }
        out.extend_from_slice(&strings);
        out
    }

    struct FixtureLoader {
        mo: Vec<u8>,
    }

    impl TranslationsLoader for FixtureLoader {
        fn load_catalog(&self, domain: &str, lang: &str) -> Option<Cow<'_, [u8]>> {
            if domain == "wxdtest" && lang == "fr" {
                // Return freshly-owned bytes (not a borrow of `self.mo`) so the
                // test also proves the owned/compute-on-demand path: the Vec is
                // dropped right after this returns, so it must be consumed
                // inside the emit call, not after.
                Some(Cow::Owned(self.mo.clone()))
            } else {
                None
            }
        }

        fn available_translations(&self, domain: &str) -> Vec<String> {
            if domain == "wxdtest" {
                vec!["fr".to_string()]
            } else {
                Vec::new()
            }
        }
    }

    #[test]
    fn rust_loader_serves_embedded_catalog() {
        // Include the gettext metadata header ("") so the charset is declared;
        // originals must stay sorted ("" sorts before "Hello").
        let mo = make_mo(&[("", "Content-Type: text/plain; charset=UTF-8\n"), ("Hello", "Bonjour")]);

        let translations = Translations::new();
        translations.set_loader(FixtureLoader { mo });
        translations.set_language_str("fr");

        // The loader's available_translations must flow through so AddCatalog
        // can select "fr".
        let avail = translations.get_available_translations("wxdtest");
        assert!(avail.contains(&"fr".to_string()), "available translations: {avail:?}");

        // Loading the catalog goes through load_catalog -> CreateFromData.
        assert!(
            translations.add_catalog("wxdtest"),
            "add_catalog should load the catalog supplied by the Rust loader"
        );
        assert_eq!(
            translations.get_string("Hello", "wxdtest").as_deref(),
            Some("Bonjour"),
            "string should be translated via the embedded catalog"
        );

        // A domain the loader doesn't serve resolves to nothing.
        assert!(translations.get_available_translations("other").is_empty());
    }
}
