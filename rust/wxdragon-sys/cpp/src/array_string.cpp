#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/arrstr.h>

// ArrayString helper functions
WXD_EXPORTED wxd_ArrayString_t*
wxd_ArrayString_Create()
{
    return reinterpret_cast<wxd_ArrayString_t*>(new (std::nothrow) wxArrayString());
}

WXD_EXPORTED void
wxd_ArrayString_Free(const wxd_ArrayString_t* self)
{
    if (self) {
        delete const_cast<wxArrayString*>(reinterpret_cast<const wxArrayString*>(self));
    }
}

WXD_EXPORTED wxd_ArrayString_t*
wxd_ArrayString_Clone(const wxd_ArrayString_t* array)
{
    if (!array)
        return nullptr;
    const wxArrayString* wx_array = reinterpret_cast<const wxArrayString*>(array);
    wxArrayString* cloned = new (std::nothrow) wxArrayString(*wx_array);
    if (!cloned)
        return nullptr;
    return reinterpret_cast<wxd_ArrayString_t*>(cloned);
}

WXD_EXPORTED int
wxd_ArrayString_GetCount(const wxd_ArrayString_t* array)
{
    if (!array)
        return 0;
    const wxArrayString* wx_array = reinterpret_cast<const wxArrayString*>(array);
    return wx_array->GetCount();
}

/**
 * Get string at specified index.
 * Returns the real length of the string, excluding the null terminator.
 * If the returned length is negative, indicates an error (invalid index or parameters).
 * If buffer is non-null and bufferLen > 0, copies up to bufferLen - 1 characters and null-terminates.
 * If buffer is null or bufferLen == 0, does not copy anything.
 */
WXD_EXPORTED int
wxd_ArrayString_GetString(const wxd_ArrayString_t* array, int index, char* buffer, size_t bufferLen)
{
    if (!array)
        return -1;

    const wxArrayString* wx_array = reinterpret_cast<const wxArrayString*>(array);
    if (index < 0 || index >= static_cast<int>(wx_array->GetCount()))
        return -1;

    wxString str = wx_array->Item(index);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(str, buffer, bufferLen);
}

WXD_EXPORTED bool
wxd_ArrayString_Add(const wxd_ArrayString_t* self, const char* str)
{
    if (!self)
        return false;
    wxArrayString* arr = const_cast<wxArrayString*>(reinterpret_cast<const wxArrayString*>(self));
    arr->Add(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(str));
    return true;
}

WXD_EXPORTED void
wxd_ArrayString_Clear(const wxd_ArrayString_t* self)
{
    if (!self)
        return;
    wxArrayString* arr = const_cast<wxArrayString*>(reinterpret_cast<const wxArrayString*>(self));
    arr->Clear();
}

// Helper function to populate a wxd_ArrayString_t from a wxArrayString
// Exported for use by other components like file_dialog.cpp
WXD_EXPORTED void
wxd_ArrayString_AssignFromWxArrayString(const wxd_ArrayString_t* target,
                                        const wxArrayString& source)
{
    if (!target)
        return;
    wxArrayString* arr = const_cast<wxArrayString*>(reinterpret_cast<const wxArrayString*>(target));
    *arr = source; // wxArrayString has an assignment operator
}