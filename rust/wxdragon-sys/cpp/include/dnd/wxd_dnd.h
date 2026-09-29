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

#endif // WXD_DND_H