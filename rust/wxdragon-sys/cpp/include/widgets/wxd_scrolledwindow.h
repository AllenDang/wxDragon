#ifndef WXD_SCROLLEDWINDOW_H
#define WXD_SCROLLEDWINDOW_H

#include "../wxd_types.h"

// --- ScrolledWindow Functions ---
WXD_EXPORTED wxd_ScrolledWindow_t*
wxd_ScrolledWindow_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                          wxd_Style_t style);

WXD_EXPORTED void
wxd_ScrolledWindow_SetScrollRate(wxd_ScrolledWindow_t* self, int xstep, int ystep);

WXD_EXPORTED void
wxd_ScrolledWindow_SetScrollbars(wxd_ScrolledWindow_t* self, int pixelsPerUnitX, int pixelsPerUnitY,
                                 int noUnitsX, int noUnitsY, int xPos, int yPos, bool noRefresh);

WXD_EXPORTED void
wxd_ScrolledWindow_EnableScrolling(wxd_ScrolledWindow_t* self, bool xScrolling, bool yScrolling);

WXD_EXPORTED void
wxd_ScrolledWindow_Scroll_Coord(wxd_ScrolledWindow_t* self, int x, int y);

WXD_EXPORTED void
wxd_ScrolledWindow_Scroll_Point(wxd_ScrolledWindow_t* self, wxd_Point pt);

WXD_EXPORTED void
wxd_ScrolledWindow_GetVirtualSize(wxd_ScrolledWindow_t* self, int* w, int* h);

WXD_EXPORTED void
wxd_ScrolledWindow_GetScrollPixelsPerUnit(wxd_ScrolledWindow_t* self, int* xUnit, int* yUnit);

#endif // WXD_SCROLLEDWINDOW_H