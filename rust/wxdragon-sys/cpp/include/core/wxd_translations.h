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

// --- FileTranslationsLoader Functions ---

// Add a catalog lookup path prefix (static method)
// The path is where translation files (.mo files) are searched for
WXD_EXPORTED void
wxd_FileTranslationsLoader_AddCatalogLookupPathPrefix(const char* prefix);

#endif // WXD_TRANSLATIONS_H
