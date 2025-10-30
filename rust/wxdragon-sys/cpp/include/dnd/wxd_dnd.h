#ifndef WXD_DND_H
#define WXD_DND_H

#include "../wxd_types.h"

// --- Forward declarations for new opaque types ---
typedef struct wxd_DataObject_t wxd_DataObject_t;
typedef struct wxd_TextDataObject_t wxd_TextDataObject_t;
typedef struct wxd_FileDataObject_t wxd_FileDataObject_t;
typedef struct wxd_DropSource_t wxd_DropSource_t;
typedef struct wxd_DropTarget_t wxd_DropTarget_t;
typedef struct wxd_TextDropTarget_t wxd_TextDropTarget_t;
typedef struct wxd_FileDropTarget_t wxd_FileDropTarget_t;

// Use the wxd_DragResult enum from wxd_types.h instead of redefining
typedef wxd_DragResult WXDDragResultCEnum;

// --- Event types for drag and drop ---
// (to be added to WXDEventTypeCEnum in wxd_types.h)

// --- Data Object Functions ---
WXD_EXPORTED wxd_TextDataObject_t*
wxd_TextDataObject_Create(const char* text);
WXD_EXPORTED void
wxd_TextDataObject_Destroy(wxd_TextDataObject_t* obj);
WXD_EXPORTED void
wxd_TextDataObject_SetText(wxd_TextDataObject_t* obj, const char* text);

WXD_EXPORTED wxd_FileDataObject_t*
wxd_FileDataObject_Create();
WXD_EXPORTED void
wxd_FileDataObject_Destroy(wxd_FileDataObject_t* obj);
WXD_EXPORTED int
wxd_FileDataObject_GetFilenames(wxd_FileDataObject_t* obj, wxd_ArrayString_t* filenames);
WXD_EXPORTED void
wxd_FileDataObject_AddFile(wxd_FileDataObject_t* obj, const char* filename);

// --- Drop Source Functions ---
WXD_EXPORTED wxd_DropSource_t*
wxd_DropSource_Create(wxd_Window_t* window);
WXD_EXPORTED void
wxd_DropSource_Destroy(wxd_DropSource_t* source);
WXD_EXPORTED void
wxd_DropSource_SetData(wxd_DropSource_t* source, wxd_DataObject_t* data);
WXD_EXPORTED WXDDragResultCEnum
wxd_DropSource_DoDragDrop(wxd_DropSource_t* source, bool allow_move);

// --- Drop Target Functions ---
// Base TextDropTarget
WXD_EXPORTED wxd_TextDropTarget_t*
wxd_TextDropTarget_Create(wxd_Window_t* window, void* rust_on_drop_text_fn, void* rust_closure_ptr);
WXD_EXPORTED void
wxd_TextDropTarget_Destroy(wxd_TextDropTarget_t* target);

// FileDropTarget
WXD_EXPORTED wxd_FileDropTarget_t*
wxd_FileDropTarget_Create(wxd_Window_t* window, void* rust_on_drop_files_fn,
                          void* rust_closure_ptr);
WXD_EXPORTED void
wxd_FileDropTarget_Destroy(wxd_FileDropTarget_t* target);

// Window association
WXD_EXPORTED void
wxd_Window_SetDropTarget(wxd_Window_t* window, wxd_DropTarget_t* target);

#endif // WXD_DND_H