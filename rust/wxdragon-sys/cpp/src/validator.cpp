#include <wx/wxprec.h>
#include <wx/wx.h>

#include "../include/wxdragon.h"

#if wxUSE_VALIDATORS

#include <wx/valtext.h>

#include "wxd_utils.h"

namespace
{

wxArrayString
ToArrayString(const char* const* items, size_t count)
{
    wxArrayString strings;
    if (items) {
        for (size_t n = 0; n < count; n++) {
            if (items[n]) {
                strings.Add(wxString::FromUTF8(items[n]));
            }
        }
    }

    return strings;
}

} // anonymous namespace

extern "C" {

WXD_EXPORTED wxd_TextValidator_t*
wxd_TextValidator_Create(int64_t style)
{
    return reinterpret_cast<wxd_TextValidator_t*>(
        new wxTextValidator(static_cast<long>(style)));
}

WXD_EXPORTED void
wxd_TextValidator_Destroy(wxd_TextValidator_t* self)
{
    delete reinterpret_cast<wxTextValidator*>(self);
}

WXD_EXPORTED void
wxd_TextValidator_SetCharIncludes(wxd_TextValidator_t* self, const char* chars)
{
    if (!self) return;
    reinterpret_cast<wxTextValidator*>(self)->SetCharIncludes(
        chars ? wxString::FromUTF8(chars) : wxString());
}

WXD_EXPORTED void
wxd_TextValidator_SetCharExcludes(wxd_TextValidator_t* self, const char* chars)
{
    if (!self) return;
    reinterpret_cast<wxTextValidator*>(self)->SetCharExcludes(
        chars ? wxString::FromUTF8(chars) : wxString());
}

WXD_EXPORTED void
wxd_TextValidator_SetIncludes(wxd_TextValidator_t* self, const char* const* items, size_t count)
{
    if (!self) return;
    reinterpret_cast<wxTextValidator*>(self)->SetIncludes(ToArrayString(items, count));
}

WXD_EXPORTED void
wxd_TextValidator_SetExcludes(wxd_TextValidator_t* self, const char* const* items, size_t count)
{
    if (!self) return;
    reinterpret_cast<wxTextValidator*>(self)->SetExcludes(ToArrayString(items, count));
}

WXD_EXPORTED void
wxd_Window_SetTextValidator(wxd_Window_t* self, wxd_TextValidator_t* validator)
{
    if (!self || !validator) return;
    reinterpret_cast<wxWindow*>(self)->SetValidator(
        *reinterpret_cast<wxTextValidator*>(validator));
}

WXD_EXPORTED bool
wxd_Window_Validate(wxd_Window_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxWindow*>(self)->Validate();
}

} // extern "C"

#endif // wxUSE_VALIDATORS
