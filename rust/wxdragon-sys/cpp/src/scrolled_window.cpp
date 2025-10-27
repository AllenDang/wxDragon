#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wx/scrolwin.h"
#include "wx/window.h"
#include "wx/gdicmn.h"
#include "wxdragon.h"

extern "C" {

WXD_EXPORTED wxd_ScrolledWindow_t*
wxd_ScrolledWindow_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                          wxd_Style_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    wxPoint wxPos(pos.x, pos.y);
    wxSize wxSize(size.width, size.height);
    // wxScrolledWindow name parameter is not usually needed
    wxScrolledWindow* scrolledWin = new wxScrolledWindow(parentWin, id, wxPos, wxSize, style);
    return (wxd_ScrolledWindow_t*)scrolledWin;
}

WXD_EXPORTED void
wxd_ScrolledWindow_SetScrollRate(wxd_ScrolledWindow_t* self, int xstep, int ystep)
{
    wxScrolledWindow* scrolledWin = (wxScrolledWindow*)self;
    if (scrolledWin) {
        scrolledWin->SetScrollRate(xstep, ystep);
    }
}

WXD_EXPORTED void
wxd_ScrolledWindow_SetScrollbars(wxd_ScrolledWindow_t* self, int pixelsPerUnitX, int pixelsPerUnitY,
                                 int noUnitsX, int noUnitsY, int xPos, int yPos, bool noRefresh)
{
    wxScrolledWindow* scrolledWin = (wxScrolledWindow*)self;
    if (scrolledWin) {
        scrolledWin->SetScrollbars(pixelsPerUnitX, pixelsPerUnitY, noUnitsX, noUnitsY, xPos, yPos,
                                   noRefresh);
    }
}

WXD_EXPORTED void
wxd_ScrolledWindow_EnableScrolling(wxd_ScrolledWindow_t* self, bool xScrolling, bool yScrolling)
{
    wxScrolledWindow* scrolledWin = (wxScrolledWindow*)self;
    if (scrolledWin) {
        scrolledWin->EnableScrolling(xScrolling, yScrolling);
    }
}

WXD_EXPORTED void
wxd_ScrolledWindow_Scroll_Coord(wxd_ScrolledWindow_t* self, int x, int y)
{
    wxScrolledWindow* scrolledWin = (wxScrolledWindow*)self;
    if (scrolledWin) {
        scrolledWin->Scroll(x, y);
    }
}

WXD_EXPORTED void
wxd_ScrolledWindow_Scroll_Point(wxd_ScrolledWindow_t* self, wxd_Point pt)
{
    wxScrolledWindow* scrolledWin = (wxScrolledWindow*)self;
    if (scrolledWin) {
        scrolledWin->Scroll(wxPoint(pt.x, pt.y));
    }
}

WXD_EXPORTED void
wxd_ScrolledWindow_GetVirtualSize(wxd_ScrolledWindow_t* self, int* w, int* h)
{
    wxScrolledWindow* scrolledWin = (wxScrolledWindow*)self;
    if (scrolledWin && w && h) {
        int width, height;
        scrolledWin->GetVirtualSize(&width, &height);
        *w = width;
        *h = height;
    }
    else if (w) {
        *w = 0;
    }
    else if (h) {
        *h = 0;
    }
}

WXD_EXPORTED void
wxd_ScrolledWindow_GetScrollPixelsPerUnit(wxd_ScrolledWindow_t* self, int* xUnit, int* yUnit)
{
    wxScrolledWindow* scrolledWin = (wxScrolledWindow*)self;
    if (scrolledWin && xUnit && yUnit) {
        int xppu, yppu;
        scrolledWin->GetScrollPixelsPerUnit(&xppu, &yppu);
        *xUnit = xppu;
        *yUnit = yppu;
    }
    else if (xUnit) {
        *xUnit = 0;
    }
    else if (yUnit) {
        *yUnit = 0;
    }
}

// No wxd_ScrolledWindow_Destroy needed, parent manages lifetime.

} // extern "C"