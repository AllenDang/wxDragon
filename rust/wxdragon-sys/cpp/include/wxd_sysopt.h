#ifndef WXD_SYSOPT_H
#define WXD_SYSOPT_H 1

#include "wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

WXD_EXPORTED void wxd_SystemOptions_SetOption_String(const char* name, const char* value);
WXD_EXPORTED void wxd_SystemOptions_SetOption_Int(const char* name, int value);
WXD_EXPORTED int wxd_SystemOptions_GetOption_String(const char* name, char* buffer, int buffer_len);
WXD_EXPORTED int wxd_SystemOptions_GetOption_Int(const char* name);
WXD_EXPORTED bool wxd_SystemOptions_HasOption(const char* name);
WXD_EXPORTED bool wxd_SystemOptions_IsFalse(const char* name);

#ifdef __cplusplus
}
#endif

#endif // WXD_SYSOPT_H
