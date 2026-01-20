#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/translation.h>
#include <wx/intl.h>

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
wxd_FileTranslationsLoader_AddCatalogLookupPathPrefix(const char* prefix)
{
    if (!prefix)
        return;
    wxFileTranslationsLoader::AddCatalogLookupPathPrefix(
        wxString::FromUTF8(prefix));
}

} // extern "C"
