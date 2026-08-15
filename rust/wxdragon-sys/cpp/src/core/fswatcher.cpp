#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../../include/wxdragon.h"
#include "../../include/core/wxd_fswatcher.h"
#include <wx/fswatcher.h>

extern "C" {

WXD_EXPORTED wxd_FileSystemWatcher_t*
wxd_FileSystemWatcher_Create(void)
{
    wxFileSystemWatcher* watcher = new wxFileSystemWatcher();
    return reinterpret_cast<wxd_FileSystemWatcher_t*>(watcher);
}

WXD_EXPORTED void
wxd_FileSystemWatcher_Destroy(wxd_FileSystemWatcher_t* watcher)
{
    if (!watcher)
        return;
    delete reinterpret_cast<wxFileSystemWatcher*>(watcher);
}

WXD_EXPORTED bool
wxd_FileSystemWatcher_Add(wxd_FileSystemWatcher_t* watcher, const char* path, int events)
{
    if (!watcher || !path)
        return false;
    wxFileSystemWatcher* wx_watcher = reinterpret_cast<wxFileSystemWatcher*>(watcher);
    return wx_watcher->Add(wxFileName(wxString::FromUTF8(path)), events);
}

WXD_EXPORTED bool
wxd_FileSystemWatcher_AddTree(wxd_FileSystemWatcher_t* watcher, const char* path, int events)
{
    if (!watcher || !path)
        return false;
    wxFileSystemWatcher* wx_watcher = reinterpret_cast<wxFileSystemWatcher*>(watcher);
    return wx_watcher->AddTree(wxFileName::DirName(wxString::FromUTF8(path)), events);
}

WXD_EXPORTED bool
wxd_FileSystemWatcher_Remove(wxd_FileSystemWatcher_t* watcher, const char* path)
{
    if (!watcher || !path)
        return false;
    wxFileSystemWatcher* wx_watcher = reinterpret_cast<wxFileSystemWatcher*>(watcher);
    return wx_watcher->Remove(wxFileName(wxString::FromUTF8(path)));
}

WXD_EXPORTED bool
wxd_FileSystemWatcher_RemoveTree(wxd_FileSystemWatcher_t* watcher, const char* path)
{
    if (!watcher || !path)
        return false;
    wxFileSystemWatcher* wx_watcher = reinterpret_cast<wxFileSystemWatcher*>(watcher);
    return wx_watcher->RemoveTree(wxFileName::DirName(wxString::FromUTF8(path)));
}

WXD_EXPORTED bool
wxd_FileSystemWatcher_RemoveAll(wxd_FileSystemWatcher_t* watcher)
{
    if (!watcher)
        return false;
    wxFileSystemWatcher* wx_watcher = reinterpret_cast<wxFileSystemWatcher*>(watcher);
    return wx_watcher->RemoveAll();
}

WXD_EXPORTED int
wxd_FileSystemWatcherEvent_GetChangeType(wxd_Event_t* event)
{
    if (!event)
        return 0;
    wxFileSystemWatcherEvent* wx_event = reinterpret_cast<wxFileSystemWatcherEvent*>(event);
    return wx_event->GetChangeType();
}

WXD_EXPORTED char*
wxd_FileSystemWatcherEvent_GetPath(wxd_Event_t* event)
{
    if (!event)
        return strdup("");
    wxFileSystemWatcherEvent* wx_event = reinterpret_cast<wxFileSystemWatcherEvent*>(event);
    return strdup(wx_event->GetPath().GetFullPath().ToUTF8().data());
}

WXD_EXPORTED char*
wxd_FileSystemWatcherEvent_GetNewPath(wxd_Event_t* event)
{
    if (!event)
        return strdup("");
    wxFileSystemWatcherEvent* wx_event = reinterpret_cast<wxFileSystemWatcherEvent*>(event);
    return strdup(wx_event->GetNewPath().GetFullPath().ToUTF8().data());
}

WXD_EXPORTED bool
wxd_FileSystemWatcherEvent_IsError(wxd_Event_t* event)
{
    if (!event)
        return false;
    wxFileSystemWatcherEvent* wx_event = reinterpret_cast<wxFileSystemWatcherEvent*>(event);
    return wx_event->IsError();
}

WXD_EXPORTED char*
wxd_FileSystemWatcherEvent_GetErrorDescription(wxd_Event_t* event)
{
    if (!event)
        return strdup("");
    wxFileSystemWatcherEvent* wx_event = reinterpret_cast<wxFileSystemWatcherEvent*>(event);
    return strdup(wx_event->GetErrorDescription().ToUTF8().data());
}

} // extern "C"
