#include "wx/wxprec.h"
#ifndef WX_PRECOMP
    #include "wx/wx.h"
#endif

#include "../include/wxdragon.h"
#include "wxd_utils.h"

#include "wx/sysopt.h"

extern "C" {
    WXD_EXPORTED void wxd_SystemOptions_SetOption_String(const char* name, const char* value) {
        wxSystemOptions::SetOption(wxString::FromUTF8(name ? name : ""), wxString::FromUTF8(value ? value : ""));
    }

    WXD_EXPORTED void wxd_SystemOptions_SetOption_Int(const char* name, int value) {
        wxSystemOptions::SetOption(wxString::FromUTF8(name ? name : ""), value);
    }

    WXD_EXPORTED int wxd_SystemOptions_GetOption_String(const char* name, char* buffer, int buffer_len) {
        wxString text = wxSystemOptions::GetOption(wxString::FromUTF8(name ? name : ""));
        return wxd_cpp_utils::copy_wxstring_to_buffer(text, buffer, (size_t)buffer_len);
    }

    WXD_EXPORTED int wxd_SystemOptions_GetOption_Int(const char* name) {
        return wxSystemOptions::GetOptionInt(wxString::FromUTF8(name ? name : ""));
    }

    WXD_EXPORTED bool wxd_SystemOptions_HasOption(const char* name) {
        return wxSystemOptions::HasOption(wxString::FromUTF8(name? name : ""));
    }

    WXD_EXPORTED bool wxd_SystemOptions_IsFalse(const char* name) {
        return wxSystemOptions::IsFalse(wxString::FromUTF8(name? name : ""));
    }
}
