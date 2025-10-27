#ifndef WXD_NOTEBOOK_H
#define WXD_NOTEBOOK_H

#include "../wxdragon.h" // For WXD_EXPORTED, wxd_Window_t, wxd_Id, wxd_Point, wxd_Size, wxd_Style_t

// --- Notebook ---
WXD_EXPORTED wxd_Notebook_t*
wxd_Notebook_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                    wxd_Style_t style);
WXD_EXPORTED bool
wxd_Notebook_AddPage(wxd_Notebook_t* self, wxd_Window_t* page, const char* text, bool select);
WXD_EXPORTED int
wxd_Notebook_GetSelection(wxd_Notebook_t* self);
WXD_EXPORTED int
wxd_Notebook_SetSelection(wxd_Notebook_t* self, int page);

WXD_EXPORTED size_t
wxd_Notebook_GetPageCount(wxd_Notebook_t* self);
WXD_EXPORTED wxd_Window_t*
wxd_Notebook_GetPage(wxd_Notebook_t* self, size_t n);
WXD_EXPORTED bool
wxd_Notebook_RemovePage(wxd_Notebook_t* self, size_t n);

// Event data accessors (if specific to Notebook events beyond general wxEvent)
// Example: if wxNotebookEvent has specific data not in wxBookCtrlEvent
// WXD_EXPORTED int wxd_NotebookEvent_GetSomethingSpecific(wxd_Event_t* event);
// From notebook.cpp, it seems wxBookCtrlEvent accessors are sufficient, which are general.
// wxd_NotebookEvent_GetSelection and wxd_NotebookEvent_GetOldSelection are already in notebook.cpp
// but they cast to wxBookCtrlEvent. These could be declared in a more general
// book_ctrl_event.h or similar if such events are common across multiple book controls.
// For now, keeping them as they are, and they are likely picked up by bindgen through wxdragon.h -> notebook.cpp indirectly or should be in a C event header.
// The existing event API in `events/wxd_event_api.h` has generic `wxd_BookCtrlEvent_GetSelection` etc.
// so no specific notebook event accessors needed here.

WXD_EXPORTED void
wxd_Notebook_SetPadding(wxd_Notebook_t* self, wxd_Size padding);
WXD_EXPORTED int
wxd_Notebook_ChangeSelection(wxd_Notebook_t* self, size_t page);
WXD_EXPORTED void
wxd_Notebook_AdvanceSelection(wxd_Notebook_t* self, bool forward);

// ImageList support
WXD_EXPORTED void
wxd_Notebook_SetImageList(wxd_Notebook_t* self, wxd_ImageList_t* imageList);
WXD_EXPORTED wxd_ImageList_t*
wxd_Notebook_GetImageList(wxd_Notebook_t* self);

// Modified page manipulation to include imageId
WXD_EXPORTED bool
wxd_Notebook_AddPageWithImageId(wxd_Notebook_t* self, wxd_Window_t* page, const char* text,
                                bool select, int imageId);
WXD_EXPORTED bool
wxd_Notebook_InsertPageWithImageId(wxd_Notebook_t* self, size_t index, wxd_Window_t* page,
                                   const char* text, bool select, int imageId);

WXD_EXPORTED bool
wxd_Notebook_InsertPage(wxd_Notebook_t* self, size_t index, wxd_Window_t* page, const char* text,
                        bool select);

#endif // WXD_NOTEBOOK_H