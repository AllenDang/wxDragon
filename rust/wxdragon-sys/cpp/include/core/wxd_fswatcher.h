#ifndef WXD_FSWATCHER_H
#define WXD_FSWATCHER_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Creates a new file system watcher. Events are delivered to the watcher's own
// event handler (bind to it directly with WXD_EVENT_TYPE_FSWATCHER), the same
// way wxFileSystemWatcher behaves when SetOwner() is never called.
WXD_EXPORTED wxd_FileSystemWatcher_t*
wxd_FileSystemWatcher_Create(void);

WXD_EXPORTED void
wxd_FileSystemWatcher_Destroy(wxd_FileSystemWatcher_t* watcher);

// Starts watching a single file or directory (non-recursive for directories).
// `events` is a bitwise OR of WXDFSWEventCEnum values.
WXD_EXPORTED bool
wxd_FileSystemWatcher_Add(wxd_FileSystemWatcher_t* watcher, const char* path, int events);

// Starts watching a directory tree recursively.
WXD_EXPORTED bool
wxd_FileSystemWatcher_AddTree(wxd_FileSystemWatcher_t* watcher, const char* path, int events);

WXD_EXPORTED bool
wxd_FileSystemWatcher_Remove(wxd_FileSystemWatcher_t* watcher, const char* path);

WXD_EXPORTED bool
wxd_FileSystemWatcher_RemoveTree(wxd_FileSystemWatcher_t* watcher, const char* path);

WXD_EXPORTED bool
wxd_FileSystemWatcher_RemoveAll(wxd_FileSystemWatcher_t* watcher);

// --- wxFileSystemWatcherEvent accessors ---
// These operate on the wxd_Event_t delivered to a handler bound with
// WXD_EVENT_TYPE_FSWATCHER; the caller is responsible for only calling them
// from inside such a handler.

// Bitwise OR of WXDFSWEventCEnum values describing what changed.
WXD_EXPORTED int
wxd_FileSystemWatcherEvent_GetChangeType(wxd_Event_t* event);

// Caller-owned string (strdup'd); the Rust side reclaims it with CString::from_raw, matching wxd_Frame_GetTitle.
WXD_EXPORTED char*
wxd_FileSystemWatcherEvent_GetPath(wxd_Event_t* event);

// For rename events, the new path; equal to GetPath() for other event kinds.
// Caller-owned string (strdup'd); the Rust side reclaims it with CString::from_raw, matching wxd_Frame_GetTitle.
WXD_EXPORTED char*
wxd_FileSystemWatcherEvent_GetNewPath(wxd_Event_t* event);

WXD_EXPORTED bool
wxd_FileSystemWatcherEvent_IsError(wxd_Event_t* event);

// Caller-owned string (strdup'd); the Rust side reclaims it with CString::from_raw, matching wxd_Frame_GetTitle.
WXD_EXPORTED char*
wxd_FileSystemWatcherEvent_GetErrorDescription(wxd_Event_t* event);

#ifdef __cplusplus
}
#endif

#endif // WXD_FSWATCHER_H
