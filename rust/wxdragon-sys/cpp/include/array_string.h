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

WXD_EXPORTED int
wxd_ArrayString_GetCount(wxd_ArrayString_t* array);

WXD_EXPORTED int
wxd_ArrayString_GetString(wxd_ArrayString_t* array, int index, char* buffer, int bufferLen);

WXD_EXPORTED bool
wxd_ArrayString_Add(wxd_ArrayString_t* self, const char* str);

WXD_EXPORTED void
wxd_ArrayString_Clear(wxd_ArrayString_t* self);

#ifdef __cplusplus
}
#endif

#endif // WXD_ARRAY_STRING_H
