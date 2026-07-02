#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/translation.h>
#include <wx/intl.h>
#include <wx/uilocale.h>
#include <wx/arrstr.h>

// A wxTranslationsLoader that forwards to Rust callbacks. The C++ side is a
// dumb trampoline: it only builds the wxWidgets objects the callbacks cannot
// (wxMsgCatalog / wxArrayString); all loader policy lives in Rust.
class WxdRustTranslationsLoader : public wxTranslationsLoader
{
public:
    WxdRustTranslationsLoader(const wxd_RustTranslationsLoader_vtable* vtable,
                              void* user_data)
        : m_vtable(*vtable), m_user_data(user_data)
    {
    }

    ~WxdRustTranslationsLoader() override
    {
        if (m_vtable.destroy)
            m_vtable.destroy(m_user_data);
    }

    // Context passed as the `sink` to the Rust load_catalog callback. Rust calls
    // emit_catalog(sink, data, len) while the bytes are alive; we build the
    // catalog right there.
    struct CatalogSink
    {
        const wxString* domain;
        wxMsgCatalog* catalog;
    };

    static void emit_catalog(void* sink, const uint8_t* data, size_t len)
    {
        if (!sink || !data)
            return;
        CatalogSink* s = reinterpret_cast<CatalogSink*>(sink);
        // Bytes are consumed synchronously; wxMsgCatalog parses into its own
        // hash and does not retain the buffer, so a non-owned view is safe even
        // when the Rust side owns (and will drop) the bytes after this returns.
        s->catalog = wxMsgCatalog::CreateFromData(
            wxScopedCharBuffer::CreateNonOwned(
                reinterpret_cast<const char*>(data), len),
            *s->domain);
    }

    wxMsgCatalog* LoadCatalog(const wxString& domain,
                              const wxString& lang) override
    {
        if (!m_vtable.load_catalog)
            return nullptr;
        const wxScopedCharBuffer domainUtf8 = domain.utf8_str();
        const wxScopedCharBuffer langUtf8 = lang.utf8_str();
        CatalogSink sink{ &domain, nullptr };
        m_vtable.load_catalog(m_user_data, domainUtf8.data(), langUtf8.data(),
                              &sink, &emit_catalog);
        return sink.catalog;
    }

    wxArrayString GetAvailableTranslations(const wxString& domain) const override
    {
        wxArrayString langs;
        if (m_vtable.available) {
            const wxScopedCharBuffer domainUtf8 = domain.utf8_str();
            m_vtable.available(m_user_data, domainUtf8.data(),
                               reinterpret_cast<wxd_ArrayString_t*>(&langs));
        }
        return langs;
    }

private:
    wxd_RustTranslationsLoader_vtable m_vtable;
    void* m_user_data;
};

extern "C" {

wxd_Translations_t*
wxd_Translations_Get()
{
    return reinterpret_cast<wxd_Translations_t*>(wxTranslations::Get());
}

void
wxd_Translations_Set(wxd_Translations_t* translations)
{
    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);
    wxTranslations::Set(wx_translations);
}

wxd_Translations_t*
wxd_Translations_Create()
{
    wxTranslations* translations = new wxTranslations();
    return reinterpret_cast<wxd_Translations_t*>(translations);
}

void
wxd_Translations_Destroy(wxd_Translations_t* translations)
{
    if (!translations)
        return;
    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);
    delete wx_translations;
}

void
wxd_Translations_SetLanguage(wxd_Translations_t* translations, int lang)
{
    if (!translations)
        return;
    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);
    wx_translations->SetLanguage(static_cast<wxLanguage>(lang));
}

void
wxd_Translations_SetLanguageStr(wxd_Translations_t* translations,
                                const char* lang)
{
    if (!translations)
        return;
    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);
    wx_translations->SetLanguage(wxString::FromUTF8(lang ? lang : ""));
}

bool
wxd_Translations_AddCatalog(wxd_Translations_t* translations,
                           const char* domain,
                           int msg_id_language)
{
    if (!translations || !domain)
        return false;
    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);
    return wx_translations->AddCatalog(wxString::FromUTF8(domain),
                                       static_cast<wxLanguage>(msg_id_language));
}

bool
wxd_Translations_AddStdCatalog(wxd_Translations_t* translations)
{
    if (!translations)
        return false;
    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);
    return wx_translations->AddStdCatalog();
}

bool
wxd_Translations_IsLoaded(wxd_Translations_t* translations, const char* domain)
{
    if (!translations || !domain)
        return false;
    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);
    return wx_translations->IsLoaded(wxString::FromUTF8(domain));
}

int
wxd_Translations_GetTranslatedString(wxd_Translations_t* translations,
                                     const char* orig,
                                     const char* domain,
                                     char* buffer,
                                     size_t buffer_len)
{
    if (!translations || !orig)
        return -1;

    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);

    wxString wx_domain;
    if (domain && domain[0] != '\0') {
        wx_domain = wxString::FromUTF8(domain);
    }

    const wxString* result =
        wx_translations->GetTranslatedString(wxString::FromUTF8(orig),
                                             wx_domain.empty() ? wxString()
                                                               : wx_domain);
    if (!result)
        return -1;

    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(*result, buffer,
                                                       buffer_len);
}

int
wxd_Translations_GetTranslatedPluralString(wxd_Translations_t* translations,
                                           const char* singular,
                                           const char* plural,
                                           unsigned int n,
                                           const char* domain,
                                           char* buffer,
                                           size_t buffer_len)
{
    if (!translations || !singular)
        return -1;

    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);

    wxString wx_domain;
    if (domain && domain[0] != '\0') {
        wx_domain = wxString::FromUTF8(domain);
    }

    // wxTranslations::GetTranslatedString with n parameter looks up the
    // appropriate plural form from the catalog. The plural string argument
    // is only used as a fallback when no translation is found.
    const wxString* result = wx_translations->GetTranslatedString(
        wxString::FromUTF8(singular),
        n,
        wx_domain.empty() ? wxString() : wx_domain);

    if (!result) {
        // No translation found - return the appropriate fallback
        if (plural && n != 1) {
            return (int)wxd_cpp_utils::copy_wxstring_to_buffer(
                wxString::FromUTF8(plural), buffer, buffer_len);
        }
        return -1;
    }

    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(*result, buffer,
                                                       buffer_len);
}

int
wxd_Translations_GetHeaderValue(wxd_Translations_t* translations,
                                const char* header,
                                const char* domain,
                                char* buffer,
                                size_t buffer_len)
{
    if (!translations || !header)
        return -1;

    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);

    wxString wx_domain;
    if (domain && domain[0] != '\0') {
        wx_domain = wxString::FromUTF8(domain);
    }

    wxString result = wx_translations->GetHeaderValue(
        wxString::FromUTF8(header),
        wx_domain.empty() ? wxString() : wx_domain);

    if (result.empty())
        return -1;

    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(result, buffer,
                                                       buffer_len);
}

int
wxd_Translations_GetBestTranslation(wxd_Translations_t* translations,
                                    const char* domain,
                                    int msg_id_language,
                                    char* buffer,
                                    size_t buffer_len)
{
    if (!translations || !domain)
        return -1;

    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);

    wxString result = wx_translations->GetBestTranslation(
        wxString::FromUTF8(domain), static_cast<wxLanguage>(msg_id_language));

    if (result.empty())
        return -1;

    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(result, buffer,
                                                       buffer_len);
}

int
wxd_Translations_GetAvailableTranslations(wxd_Translations_t* translations,
                                          const char* domain,
                                          char** langs_buffer,
                                          size_t buffer_count,
                                          size_t string_buffer_len)
{
    if (!translations || !domain)
        return 0;

    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);

    wxArrayString langs =
        wx_translations->GetAvailableTranslations(wxString::FromUTF8(domain));

    int count = (int)langs.GetCount();

    // If buffer provided, fill it in
    if (langs_buffer && buffer_count > 0 && string_buffer_len > 0) {
        size_t to_copy = (size_t)count < buffer_count ? (size_t)count
                                                      : buffer_count;
        for (size_t i = 0; i < to_copy; i++) {
            if (langs_buffer[i]) {
                wxd_cpp_utils::copy_wxstring_to_buffer(langs[i], langs_buffer[i],
                                                       string_buffer_len);
            }
        }
    }

    return count;
}

void
wxd_Translations_SetRustLoader(wxd_Translations_t* translations,
                               const wxd_RustTranslationsLoader_vtable* vtable,
                               void* user_data)
{
    if (!translations || !vtable)
        return;
    wxTranslations* wx_translations =
        reinterpret_cast<wxTranslations*>(translations);
    // wxTranslations takes ownership of the loader and deletes it (calling our
    // destructor, which releases user_data via vtable.destroy).
    wx_translations->SetLoader(
        new WxdRustTranslationsLoader(vtable, user_data));
}

void
wxd_FileTranslationsLoader_AddCatalogLookupPathPrefix(const char* prefix)
{
    if (!prefix)
        return;
    wxFileTranslationsLoader::AddCatalogLookupPathPrefix(
        wxString::FromUTF8(prefix));
}

// --- Locale Functions Implementation ---

int
wxd_Locale_GetLanguageName(int lang, char* buffer, size_t buffer_len)
{
    wxString name = wxLocale::GetLanguageName(static_cast<wxLanguage>(lang));
    if (name.empty())
        return -1;
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(name, buffer, buffer_len);
}

int
wxd_Locale_GetLanguageCanonicalName(int lang, char* buffer, size_t buffer_len)
{
    const wxLanguageInfo* info = wxLocale::GetLanguageInfo(static_cast<wxLanguage>(lang));
    if (!info)
        return -1;
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(info->CanonicalName, buffer, buffer_len);
}

const wxd_LanguageInfo_t*
wxd_Locale_FindLanguageInfo(const char* locale)
{
    if (!locale)
        return nullptr;
    const wxLanguageInfo* info = wxLocale::FindLanguageInfo(wxString::FromUTF8(locale));
    return reinterpret_cast<const wxd_LanguageInfo_t*>(info);
}

const wxd_LanguageInfo_t*
wxd_Locale_GetLanguageInfo(int lang)
{
    const wxLanguageInfo* info = wxLocale::GetLanguageInfo(static_cast<wxLanguage>(lang));
    return reinterpret_cast<const wxd_LanguageInfo_t*>(info);
}

int
wxd_Locale_GetSystemLanguage()
{
    return wxLocale::GetSystemLanguage();
}

// --- LanguageInfo Functions Implementation ---

int
wxd_LanguageInfo_GetDescription(const wxd_LanguageInfo_t* info, char* buffer, size_t buffer_len)
{
    if (!info)
        return -1;
    const wxLanguageInfo* wx_info = reinterpret_cast<const wxLanguageInfo*>(info);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(wx_info->Description, buffer, buffer_len);
}

int
wxd_LanguageInfo_GetDescriptionNative(const wxd_LanguageInfo_t* info, char* buffer, size_t buffer_len)
{
    if (!info)
        return -1;
    const wxLanguageInfo* wx_info = reinterpret_cast<const wxLanguageInfo*>(info);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(wx_info->DescriptionNative, buffer, buffer_len);
}

int
wxd_LanguageInfo_GetCanonicalName(const wxd_LanguageInfo_t* info, char* buffer, size_t buffer_len)
{
    if (!info)
        return -1;
    const wxLanguageInfo* wx_info = reinterpret_cast<const wxLanguageInfo*>(info);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(wx_info->CanonicalName, buffer, buffer_len);
}

// --- UILocale Functions Implementation ---

wxd_UILocale_t*
wxd_UILocale_GetCurrent()
{
    // wxUILocale::GetCurrent() returns a reference to the current locale
    // We create a copy to store in our opaque pointer
    const wxUILocale& current = wxUILocale::GetCurrent();
    return reinterpret_cast<wxd_UILocale_t*>(new wxUILocale(current));
}

void
wxd_UILocale_Destroy(wxd_UILocale_t* locale)
{
    if (locale) {
        delete reinterpret_cast<wxUILocale*>(locale);
    }
}

int
wxd_UILocale_GetName(const wxd_UILocale_t* locale, char* buffer, size_t buffer_len)
{
    if (!locale)
        return -1;
    const wxUILocale* wx_locale = reinterpret_cast<const wxUILocale*>(locale);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(wx_locale->GetName(), buffer, buffer_len);
}

int
wxd_UILocale_GetLanguage(const wxd_UILocale_t* locale)
{
    if (!locale)
        return 0; // wxLANGUAGE_UNKNOWN is 0 in Rust enum but 1 in C++? 
        // Rust: Default=0, Unknown=1. 
        // wxWidgets: wxLANGUAGE_UNKNOWN is usually 1. wxLANGUAGE_DEFAULT is 0.
        // Let's use the constant if possible, or 0/1. 
        // Actually, let's just return the int value.
        
    const wxUILocale* wx_locale = reinterpret_cast<const wxUILocale*>(locale);
    
    // Try to find via name
    const wxLanguageInfo* info = wxLocale::FindLanguageInfo(wx_locale->GetName());
    if (info)
        return info->Language;
        
    return 1; // wxLANGUAGE_UNKNOWN
}

} // extern "C"