#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/arrstr.h>

// ArrayString helper functions
WXD_EXPORTED wxd_ArrayString_t*
wxd_ArrayString_Create()
{
    wxd_ArrayString_t* arr_str = new wxd_ArrayString_t();
    arr_str->internal_data = new wxArrayString();
    return arr_str;
}

WXD_EXPORTED void
wxd_ArrayString_Free(wxd_ArrayString_t* self)
{
    if (self) {
        if (self->internal_data) {
            delete static_cast<wxArrayString*>(self->internal_data);
        }
        delete self;
    }
}

WXD_EXPORTED int
wxd_ArrayString_GetCount(wxd_ArrayString_t* array)
{
    if (!array || !array->internal_data)
        return 0;
    wxArrayString* wx_array = static_cast<wxArrayString*>(array->internal_data);
    return wx_array->GetCount();
}

WXD_EXPORTED int
wxd_ArrayString_GetString(wxd_ArrayString_t* array, int index, char* buffer, int bufferLen)
{
    if (!array || !array->internal_data || !buffer || bufferLen <= 0)
        return -1;

    wxArrayString* wx_array = static_cast<wxArrayString*>(array->internal_data);
    if (index < 0 || index >= static_cast<int>(wx_array->GetCount()))
        return -1;

    wxString str = wx_array->Item(index);
    wxScopedCharBuffer utf8 = str.utf8_str();

    size_t len = strlen(utf8.data());
    if (len >= static_cast<size_t>(bufferLen)) {
        // Buffer too small, truncate
        len = bufferLen - 1;
    }

    memcpy(buffer, utf8.data(), len);
    buffer[len] = '\0';

    return len;
}

WXD_EXPORTED bool
wxd_ArrayString_Add(wxd_ArrayString_t* self, const char* str)
{
    if (!self || !self->internal_data)
        return false;
    wxArrayString* wx_arr_str = static_cast<wxArrayString*>(self->internal_data);
    wx_arr_str->Add(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(str));
    return true;
}

WXD_EXPORTED void
wxd_ArrayString_Clear(wxd_ArrayString_t* self)
{
    if (!self || !self->internal_data)
        return;
    wxArrayString* wx_arr_str = static_cast<wxArrayString*>(self->internal_data);
    wx_arr_str->Clear();
}

// Helper function to populate a wxd_ArrayString_t from a wxArrayString
// Exported for use by other components like file_dialog.cpp
WXD_EXPORTED void
wxd_ArrayString_AssignFromWxArrayString(wxd_ArrayString_t* target, const wxArrayString& source)
{
    if (!target || !target->internal_data)
        return;
    wxArrayString* dest_wx_arr = static_cast<wxArrayString*>(target->internal_data);
    *dest_wx_arr = source; // wxArrayString has an assignment operator
}