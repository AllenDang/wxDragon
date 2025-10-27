#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/splitter.h>

// Implementation for wxd_SplitterWindow_Create
WXD_EXPORTED wxd_SplitterWindow_t*
wxd_SplitterWindow_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                          wxd_Style_t style)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    // Need to ensure style includes wxSP_NOBORDER if parent is already providing border
    // or manage borders appropriately.
    wxSplitterWindow* splitter = new wxSplitterWindow(parentWin, id, wxPoint(pos.x, pos.y),
                                                      wxSize(size.width, size.height), style);
    // Note: wxSplitterWindow manages its children, so attaching cleanup notifiers
    // to the *child panes* might be necessary if they aren't otherwise owned by Rust.
    // The splitter itself will be cleaned up by its parent.
    return reinterpret_cast<wxd_SplitterWindow_t*>(splitter);
}

// Implementation for wxd_SplitterWindow_SplitVertically
WXD_EXPORTED bool
wxd_SplitterWindow_SplitVertically(wxd_SplitterWindow_t* self, wxd_Window_t* window1,
                                   wxd_Window_t* window2, int sashPosition)
{
    wxSplitterWindow* splitter = reinterpret_cast<wxSplitterWindow*>(self);
    wxWindow* win1 = reinterpret_cast<wxWindow*>(window1);
    wxWindow* win2 = reinterpret_cast<wxWindow*>(window2);
    if (!splitter || !win1 || !win2)
        return false;
    return splitter->SplitVertically(win1, win2, sashPosition);
}

// Implementation for wxd_SplitterWindow_SplitHorizontally
WXD_EXPORTED bool
wxd_SplitterWindow_SplitHorizontally(wxd_SplitterWindow_t* self, wxd_Window_t* window1,
                                     wxd_Window_t* window2, int sashPosition)
{
    wxSplitterWindow* splitter = reinterpret_cast<wxSplitterWindow*>(self);
    wxWindow* win1 = reinterpret_cast<wxWindow*>(window1);
    wxWindow* win2 = reinterpret_cast<wxWindow*>(window2);
    if (!splitter || !win1 || !win2)
        return false;
    return splitter->SplitHorizontally(win1, win2, sashPosition);
}

// Implementation for wxd_SplitterWindow_Unsplit
WXD_EXPORTED bool
wxd_SplitterWindow_Unsplit(wxd_SplitterWindow_t* self, wxd_Window_t* toRemove)
{
    wxSplitterWindow* splitter = reinterpret_cast<wxSplitterWindow*>(self);
    wxWindow* winToRemove = reinterpret_cast<wxWindow*>(toRemove);
    if (!splitter)
        return false;
    // winToRemove can be null, in which case the inactive pane is removed.
    return splitter->Unsplit(winToRemove);
}

// Implementation for wxd_SplitterWindow_Initialize
WXD_EXPORTED void
wxd_SplitterWindow_Initialize(wxd_SplitterWindow_t* self, wxd_Window_t* window)
{
    wxSplitterWindow* splitter = reinterpret_cast<wxSplitterWindow*>(self);
    wxWindow* win = reinterpret_cast<wxWindow*>(window);
    if (!splitter || !win)
        return;
    splitter->Initialize(win);
}

// Implementation for wxd_SplitterWindow_SetSashPosition
WXD_EXPORTED void
wxd_SplitterWindow_SetSashPosition(wxd_SplitterWindow_t* self, int position, bool redraw)
{
    wxSplitterWindow* splitter = reinterpret_cast<wxSplitterWindow*>(self);
    if (!splitter)
        return;
    splitter->SetSashPosition(position, redraw);
}

// Implementation for wxd_SplitterWindow_GetSashPosition
WXD_EXPORTED int
wxd_SplitterWindow_GetSashPosition(wxd_SplitterWindow_t* self)
{
    wxSplitterWindow* splitter = reinterpret_cast<wxSplitterWindow*>(self);
    if (!splitter)
        return 0;
    return splitter->GetSashPosition();
}

// Implementation for wxd_SplitterWindow_SetMinimumPaneSize
WXD_EXPORTED void
wxd_SplitterWindow_SetMinimumPaneSize(wxd_SplitterWindow_t* self, int paneSize)
{
    wxSplitterWindow* splitter = reinterpret_cast<wxSplitterWindow*>(self);
    if (!splitter)
        return;
    splitter->SetMinimumPaneSize(paneSize);
}

// Implementation for wxd_SplitterEvent_GetSashPosition
WXD_EXPORTED int
wxd_SplitterEvent_GetSashPosition(wxd_Event_t* event)
{
    if (!event)
        return 0;
    wxSplitterEvent* splitterEvent =
        dynamic_cast<wxSplitterEvent*>(reinterpret_cast<wxEvent*>(event));
    if (!splitterEvent)
        return 0; // Or maybe -1 to indicate error?
    return splitterEvent->GetSashPosition();
}