#ifndef WXD_TRANSLATIONS_H
#define WXD_TRANSLATIONS_H

#include "../wxd_types.h"

// --- Translations Functions ---

// Get the global translations instance (may be null if not set)
WXD_EXPORTED wxd_Translations_t*
wxd_Translations_Get();

// Set the global translations instance (takes ownership)
// Pass null to remove the current translations object
WXD_EXPORTED void
wxd_Translations_Set(wxd_Translations_t* translations);

// Create a new translations instance
WXD_EXPORTED wxd_Translations_t*
wxd_Translations_Create();

// Destroy a translations instance (only for non-global instances)
WXD_EXPORTED void
wxd_Translations_Destroy(wxd_Translations_t* translations);

// Set the language for translations using wxLanguage enum value
WXD_EXPORTED void
wxd_Translations_SetLanguage(wxd_Translations_t* translations, int lang);

// Set the language for translations using language string (e.g., "en_US")
WXD_EXPORTED void
wxd_Translations_SetLanguageStr(wxd_Translations_t* translations, const char* lang);

// Add a message catalog for a domain
// msg_id_language specifies the language of the strings in the source code
// Returns true if the catalog was successfully loaded
WXD_EXPORTED bool
wxd_Translations_AddCatalog(wxd_Translations_t* translations,
                           const char* domain,
                           int msg_id_language);

// Add the standard wxWidgets message catalog
// Returns true if the catalog was successfully loaded
WXD_EXPORTED bool
wxd_Translations_AddStdCatalog(wxd_Translations_t* translations);

// Check if a catalog for the given domain is loaded
WXD_EXPORTED bool
wxd_Translations_IsLoaded(wxd_Translations_t* translations, const char* domain);

// Get a translated string
// Returns the length of the result (not including null terminator), or -1 if not found
// If buffer is not null and buffer_len is non-zero, copies up to buffer_len-1 characters
WXD_EXPORTED int
wxd_Translations_GetTranslatedString(wxd_Translations_t* translations,
                                     const char* orig,
                                     const char* domain,
                                     char* buffer,
                                     size_t buffer_len);

// Get a translated plural string
// n is the count used to determine which plural form to use
// Returns the length of the result (not including null terminator), or -1 if not found
WXD_EXPORTED int
wxd_Translations_GetTranslatedPluralString(wxd_Translations_t* translations,
                                           const char* singular,
                                           const char* plural,
                                           unsigned int n,
                                           const char* domain,
                                           char* buffer,
                                           size_t buffer_len);

// Get a header value from a catalog (e.g., "Content-Type", "Plural-Forms")
// Returns the length of the result (not including null terminator), or -1 if not found
WXD_EXPORTED int
wxd_Translations_GetHeaderValue(wxd_Translations_t* translations,
                                const char* header,
                                const char* domain,
                                char* buffer,
                                size_t buffer_len);

// Get the best available translation for a domain
// msg_id_language specifies what language the original strings are in
// Returns the length of the language string, or -1 if none found
WXD_EXPORTED int
wxd_Translations_GetBestTranslation(wxd_Translations_t* translations,
                                    const char* domain,
                                    int msg_id_language,
                                    char* buffer,
                                    size_t buffer_len);

// Get all available translations for a domain
// Returns the number of available translations
// If langs_buffer is not null and buffer_count > 0, fills in up to buffer_count language strings
// Each string in langs_buffer must be pre-allocated with at least 32 bytes
WXD_EXPORTED int
wxd_Translations_GetAvailableTranslations(wxd_Translations_t* translations,
                                          const char* domain,
                                          char** langs_buffer,
                                          size_t buffer_count,
                                          size_t string_buffer_len);

// --- Custom (Rust-backed) translations loader ---

// A set of callbacks that back a custom wxTranslationsLoader implemented on the
// Rust side. The C++ trampoline only constructs the wxWidgets objects the
// callbacks cannot (wxMsgCatalog / wxArrayString); all loader policy lives in
// Rust.
// Sink used by load_catalog to hand catalog bytes to C++. `emit` must be called
// (at most once) while the bytes are still alive; C++ parses them synchronously.
typedef void (*wxd_TranslationsCatalogSink)(void* sink, const uint8_t* data, size_t len);

typedef struct wxd_RustTranslationsLoader_vtable {
    // Provide the raw .mo catalog bytes for (domain, lang) by calling
    // `emit(sink, data, len)` exactly once with the bytes while they are alive;
    // C++ builds the catalog synchronously inside `emit`. Return true if a
    // catalog was emitted, false if unavailable. Passing bytes through `emit`
    // (rather than out-params C++ reads after the call) lets a loader return
    // freshly-computed/owned bytes (e.g. decompressed) as well as borrowed ones;
    // the bytes only need to outlive the `emit` call.
    bool (*load_catalog)(void* user_data,
                         const char* domain,
                         const char* lang,
                         void* sink,
                         wxd_TranslationsCatalogSink emit);

    // Append every available language for `domain` to `out` (a wxArrayString)
    // via wxd_ArrayString_Add.
    void (*available)(void* user_data, const char* domain, wxd_ArrayString_t* out);

    // Release user_data. Called from the loader's destructor.
    void (*destroy)(void* user_data);
} wxd_RustTranslationsLoader_vtable;

// Install a Rust-backed loader on `translations`, replacing any existing loader.
// wxTranslations takes ownership of the loader; the vtable is copied, so it need
// not outlive this call. `user_data` is owned by the loader and released via
// vtable.destroy when the loader is destroyed.
WXD_EXPORTED void
wxd_Translations_SetRustLoader(wxd_Translations_t* translations,
                               const wxd_RustTranslationsLoader_vtable* vtable,
                               void* user_data);

// --- FileTranslationsLoader Functions ---

// Add a catalog lookup path prefix (static method)
// The path is where translation files (.mo files) are searched for
WXD_EXPORTED void
wxd_FileTranslationsLoader_AddCatalogLookupPathPrefix(const char* prefix);

// --- Locale Functions ---

// Get the English name of the given language (e.g. "French")
WXD_EXPORTED int
wxd_Locale_GetLanguageName(int lang, char* buffer, size_t buffer_len);

// Get the canonical name of the given language (e.g. "fr_FR")
WXD_EXPORTED int
wxd_Locale_GetLanguageCanonicalName(int lang, char* buffer, size_t buffer_len);

// Find language info from a locale string (e.g. "fr", "en_US")
// Returns null if not found
WXD_EXPORTED const wxd_LanguageInfo_t*
wxd_Locale_FindLanguageInfo(const char* locale);

// Get the language info for the given language id
// Returns null if not found
WXD_EXPORTED const wxd_LanguageInfo_t*
wxd_Locale_GetLanguageInfo(int lang);

// Get the system default language (e.g. wxLANGUAGE_ENGLISH_US)
WXD_EXPORTED int
wxd_Locale_GetSystemLanguage();

// --- LanguageInfo Functions ---

// Get the description of the language (e.g. "French")
WXD_EXPORTED int
wxd_LanguageInfo_GetDescription(const wxd_LanguageInfo_t* info, char* buffer, size_t buffer_len);

// Get the native description of the language (e.g. "Français")
WXD_EXPORTED int
wxd_LanguageInfo_GetDescriptionNative(const wxd_LanguageInfo_t* info, char* buffer, size_t buffer_len);

// Get the canonical name of the language (e.g. "fr_FR")
WXD_EXPORTED int
wxd_LanguageInfo_GetCanonicalName(const wxd_LanguageInfo_t* info, char* buffer, size_t buffer_len);

// --- UILocale Functions ---

// Get the current UI locale
// Returns a new wxd_UILocale_t instance that must be freed with wxd_UILocale_Destroy
WXD_EXPORTED wxd_UILocale_t*
wxd_UILocale_GetCurrent();

// Destroy a wxd_UILocale_t instance
WXD_EXPORTED void
wxd_UILocale_Destroy(wxd_UILocale_t* locale);

// Get the name of the locale
WXD_EXPORTED int
wxd_UILocale_GetName(const wxd_UILocale_t* locale, char* buffer, size_t buffer_len);

// Get the language ID of this locale (returns wxLANGUAGE_UNKNOWN if unknown)
WXD_EXPORTED int
wxd_UILocale_GetLanguage(const wxd_UILocale_t* locale);

#endif // WXD_TRANSLATIONS_H