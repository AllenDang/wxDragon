#ifndef WXD_SPLITTERWINDOW_H
#define WXD_SPLITTERWINDOW_H

#include "../wxd_types.h"

// --- SplitterWindow Functions ---
WXD_EXPORTED wxd_SplitterWindow_t*
wxd_SplitterWindow_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                          wxd_Style_t style);

WXD_EXPORTED bool
wxd_SplitterWindow_SplitVertically(wxd_SplitterWindow_t* self, wxd_Window_t* window1,
                                   wxd_Window_t* window2, int sashPosition);

WXD_EXPORTED bool
wxd_SplitterWindow_SplitHorizontally(wxd_SplitterWindow_t* self, wxd_Window_t* window1,
                                     wxd_Window_t* window2, int sashPosition);

WXD_EXPORTED bool
wxd_SplitterWindow_Unsplit(wxd_SplitterWindow_t* self, wxd_Window_t* toRemove);

WXD_EXPORTED void
wxd_SplitterWindow_Initialize(wxd_SplitterWindow_t* self, wxd_Window_t* window);

WXD_EXPORTED void
wxd_SplitterWindow_SetSashPosition(wxd_SplitterWindow_t* self, int position, bool redraw);

WXD_EXPORTED int
wxd_SplitterWindow_GetSashPosition(wxd_SplitterWindow_t* self);

WXD_EXPORTED void
wxd_SplitterWindow_SetMinimumPaneSize(wxd_SplitterWindow_t* self, int paneSize);

#endif // WXD_SPLITTERWINDOW_H