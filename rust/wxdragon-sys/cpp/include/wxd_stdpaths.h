#ifndef WXD_STDPATHS_H
#define WXD_STDPATHS_H 1

#include "wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

WXD_EXPORTED int
wxd_StandardPaths_GetExecutablePath(char* buffer, size_t buffer_len);

WXD_EXPORTED int
wxd_StandardPaths_GetConfigDir(char* buffer, size_t buffer_len);

WXD_EXPORTED int
wxd_StandardPaths_GetUserConfigDir(char* buffer, size_t buffer_len);

WXD_EXPORTED int
wxd_StandardPaths_GetDataDir(char* buffer, size_t buffer_len);

WXD_EXPORTED int
wxd_StandardPaths_GetUserDataDir(char* buffer, size_t buffer_len);

WXD_EXPORTED int
wxd_StandardPaths_GetUserLocalDataDir(char* buffer, size_t buffer_len);

WXD_EXPORTED int
wxd_StandardPaths_GetDocumentsDir(char* buffer, size_t buffer_len);

WXD_EXPORTED int
wxd_StandardPaths_GetTempDir(char* buffer, size_t buffer_len);

#ifdef __cplusplus
}
#endif

#endif // WXD_STDPATHS_H
