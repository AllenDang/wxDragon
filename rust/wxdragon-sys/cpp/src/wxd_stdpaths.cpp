#include "wx/wxprec.h"
#ifndef WX_PRECOMP
#include "wx/wx.h"
#endif

#include "../include/wxdragon.h"
#include "wxd_utils.h"

#include "wx/stdpaths.h"

extern "C" {
WXD_EXPORTED int
wxd_StandardPaths_GetExecutablePath(char* buffer, size_t buffer_len)
{
    wxString path = wxStandardPaths::Get().GetExecutablePath();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(path, buffer, buffer_len);
}

WXD_EXPORTED int
wxd_StandardPaths_GetConfigDir(char* buffer, size_t buffer_len)
{
    wxString path = wxStandardPaths::Get().GetConfigDir();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(path, buffer, buffer_len);
}

WXD_EXPORTED int
wxd_StandardPaths_GetUserConfigDir(char* buffer, size_t buffer_len)
{
    wxString path = wxStandardPaths::Get().GetUserConfigDir();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(path, buffer, buffer_len);
}

WXD_EXPORTED int
wxd_StandardPaths_GetDataDir(char* buffer, size_t buffer_len)
{
    wxString path = wxStandardPaths::Get().GetDataDir();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(path, buffer, buffer_len);
}

WXD_EXPORTED int
wxd_StandardPaths_GetUserDataDir(char* buffer, size_t buffer_len)
{
    wxString path = wxStandardPaths::Get().GetUserDataDir();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(path, buffer, buffer_len);
}

WXD_EXPORTED int
wxd_StandardPaths_GetUserLocalDataDir(char* buffer, size_t buffer_len)
{
    wxString path = wxStandardPaths::Get().GetUserLocalDataDir();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(path, buffer, buffer_len);
}

WXD_EXPORTED int
wxd_StandardPaths_GetDocumentsDir(char* buffer, size_t buffer_len)
{
    wxString path = wxStandardPaths::Get().GetDocumentsDir();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(path, buffer, buffer_len);
}

WXD_EXPORTED int
wxd_StandardPaths_GetTempDir(char* buffer, size_t buffer_len)
{
    wxString path = wxStandardPaths::Get().GetTempDir();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(path, buffer, buffer_len);
}
}
