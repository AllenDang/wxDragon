#ifndef WXD_DROPTARGET_API_H
#define WXD_DROPTARGET_API_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Type definitions for callback function pointers
typedef bool (*wxd_OnDropText_Callback)(const char* text, int x, int y, void* userData);
typedef bool (*wxd_OnDropFiles_Callback)(const wxd_ArrayString_t* filenames, int x, int y,
                                         void* userData);
// Cleanup callback to free userData allocated on the Rust side
typedef void (*wxd_FreeUserData_Callback)(void* userData);

// Extended callbacks for full wxDropTarget API
typedef wxd_DragResult (*wxd_OnEnter_Callback)(int x, int y, wxd_DragResult defResult,
                                               void* userData);
typedef wxd_DragResult (*wxd_OnDragOver_Callback)(int x, int y, wxd_DragResult defResult,
                                                  void* userData);
typedef void (*wxd_OnLeave_Callback)(void* userData);
typedef bool (*wxd_OnDrop_Callback)(int x, int y, void* userData);
typedef wxd_DragResult (*wxd_OnData_Callback)(int x, int y, wxd_DragResult defResult,
                                              void* userData);

// Create text drop target with full callback set
WXD_EXPORTED wxd_TextDropTarget_t*
wxd_TextDropTarget_CreateFull(wxd_Window_t* window, wxd_OnEnter_Callback onEnter,
                              wxd_OnDragOver_Callback onDragOver, wxd_OnLeave_Callback onLeave,
                              wxd_OnDrop_Callback onDrop, wxd_OnData_Callback onData,
                              wxd_OnDropText_Callback onDropText, void* userData,
                              wxd_FreeUserData_Callback freeUserData);

// Create file drop target with full callback set
WXD_EXPORTED wxd_FileDropTarget_t*
wxd_FileDropTarget_CreateFull(wxd_Window_t* window, wxd_OnEnter_Callback onEnter,
                              wxd_OnDragOver_Callback onDragOver, wxd_OnLeave_Callback onLeave,
                              wxd_OnDrop_Callback onDrop, wxd_OnData_Callback onData,
                              wxd_OnDropFiles_Callback onDropFiles, void* userData,
                              wxd_FreeUserData_Callback freeUserData);

// Create text drop target (simplified version)
WXD_EXPORTED wxd_TextDropTarget_t*
wxd_TextDropTarget_Create(wxd_Window_t* window, void* onDropTextCallback, void* userData);

// Create file drop target (simplified version)
WXD_EXPORTED wxd_FileDropTarget_t*
wxd_FileDropTarget_Create(wxd_Window_t* window, void* onDropFilesCallback, void* userData);

// Destroy drop targets
WXD_EXPORTED void
wxd_TextDropTarget_Destroy(wxd_TextDropTarget_t* dropTarget);
WXD_EXPORTED void
wxd_FileDropTarget_Destroy(wxd_FileDropTarget_t* dropTarget);

#ifdef __cplusplus
}
#endif

#endif // WXD_DROPTARGET_API_H