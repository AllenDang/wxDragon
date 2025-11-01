#ifndef WXD_ARRAY_STRING_H
#define WXD_ARRAY_STRING_H

#include "wxd_types.h"

#ifdef __cplusplus
#include <wx/arrstr.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

// ArrayString helper functions
WXD_EXPORTED wxd_ArrayString_t*
wxd_ArrayString_Create();

WXD_EXPORTED void
wxd_ArrayString_Free(wxd_ArrayString_t* self);

WXD_EXPORTED wxd_ArrayString_t*
wxd_ArrayString_Clone(const wxd_ArrayString_t* array);

WXD_EXPORTED int
wxd_ArrayString_GetCount(const wxd_ArrayString_t* array);

/**
 * Get string at specified index.
 * Returns the real length of the string, excluding the null terminator,
 * if any error, -1 returned.
 * If the returned length is negative, indicates an error (invalid index or parameters).
 * If buffer is non-null and bufferLen > 0, copies up to bufferLen - 1 characters and null-terminates.
 * If buffer is null or bufferLen == 0, does not copy anything.
 */
WXD_EXPORTED int
wxd_ArrayString_GetString(const wxd_ArrayString_t* array, int index, char* buffer,
                          size_t bufferLen);

WXD_EXPORTED bool
wxd_ArrayString_Add(wxd_ArrayString_t* self, const char* str);

WXD_EXPORTED void
wxd_ArrayString_Clear(wxd_ArrayString_t* self);

#ifdef __cplusplus
}
#endif

#endif // WXD_ARRAY_STRING_H
