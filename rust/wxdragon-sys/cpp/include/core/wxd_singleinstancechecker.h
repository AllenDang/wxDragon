#ifndef WXD_SINGLEINSTANCECHECKER_H
#define WXD_SINGLEINSTANCECHECKER_H

#include "../wxd_types.h"

// --- SingleInstanceChecker Functions ---

// Create a new SingleInstanceChecker with explicit name and path.
// name: The name used as mutex name (Win32) or lock file name (Unix). Must not be null.
// path: Optional path for lock file directory (Unix only). Can be null for default (home dir).
// Returns a new instance, or null if creation failed.
WXD_EXPORTED wxd_SingleInstanceChecker_t*
wxd_SingleInstanceChecker_Create(const char* name, const char* path);

// Create a new SingleInstanceChecker with default name.
// Uses a combination of app name and user ID.
// Note: Must be called after wxApp is created.
// Returns a new instance, or null if creation failed.
WXD_EXPORTED wxd_SingleInstanceChecker_t*
wxd_SingleInstanceChecker_CreateDefault();

// Destroy a SingleInstanceChecker instance.
WXD_EXPORTED void
wxd_SingleInstanceChecker_Destroy(wxd_SingleInstanceChecker_t* checker);

// Check if another instance of the program is already running.
// Returns true if another instance is running, false otherwise.
WXD_EXPORTED bool
wxd_SingleInstanceChecker_IsAnotherRunning(wxd_SingleInstanceChecker_t* checker);

#endif // WXD_SINGLEINSTANCECHECKER_H
