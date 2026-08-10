#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wx/statusbr.h"
#include "wx/window.h" // Base class
#include "wx/string.h" // For wxString conversions
#include "wxdragon.h"

extern "C" {

WXD_EXPORTED wxd_StatusBar_t*
wxd_StatusBar_Create(wxd_Window_t* parent, wxd_Id id, wxd_Style_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    // TODO: Add runtime check to ensure parent is actually a wxFrame?
    // For now, assume the Rust wrapper enforces this.
    wxStatusBar* statusBar = new wxStatusBar(parentWin, id, style);
    return (wxd_StatusBar_t*)statusBar;
}

WXD_EXPORTED void
wxd_StatusBar_SetFieldsCount(wxd_StatusBar_t* self, int count)
{
    wxStatusBar* statusBar = (wxStatusBar*)self;
    if (statusBar) {
        // Need to cast count to unsigned
        statusBar->SetFieldsCount(static_cast<unsigned int>(count));
    }
}

WXD_EXPORTED void
wxd_StatusBar_SetStatusText(wxd_StatusBar_t* self, const char* text, int fieldIndex)
{
    wxStatusBar* statusBar = (wxStatusBar*)self;
    if (statusBar) {
        statusBar->SetStatusText(wxString::FromUTF8(text ? text : ""), fieldIndex);
    }
}

WXD_EXPORTED void
wxd_StatusBar_SetStatusWidths(wxd_StatusBar_t* self, int count, const int* widths)
{
    wxStatusBar* statusBar = (wxStatusBar*)self;
    if (statusBar && count > 0 && widths) {
        // wxWidgets takes count and a pointer to int array
        statusBar->SetStatusWidths(count, widths);
    }
}

WXD_EXPORTED void
wxd_StatusBar_PushStatusText(wxd_StatusBar_t* self, const char* text, int fieldIndex)
{
    wxStatusBar* statusBar = (wxStatusBar*)self;
    if (statusBar) {
        statusBar->PushStatusText(wxString::FromUTF8(text ? text : ""), fieldIndex);
    }
}

WXD_EXPORTED void
wxd_StatusBar_PopStatusText(wxd_StatusBar_t* self, int fieldIndex)
{
    wxStatusBar* statusBar = (wxStatusBar*)self;
    if (statusBar) {
        statusBar->PopStatusText(fieldIndex);
    }
}

WXD_EXPORTED wxd_Rect
wxd_StatusBar_GetFieldRect(wxd_StatusBar_t* self, int fieldIndex)
{
    wxd_Rect rect = {0, 0, 0, 0};
    wxStatusBar* statusBar = (wxStatusBar*)self;
    if (!statusBar) return rect;
    wxRect r;
    if (statusBar->GetFieldRect(fieldIndex, r)) {
        rect.x = r.x; rect.y = r.y; rect.width = r.width; rect.height = r.height;
    }
    return rect;
}

WXD_EXPORTED int
wxd_StatusBar_GetFieldsCount(wxd_StatusBar_t* self)
{
    wxStatusBar* statusBar = (wxStatusBar*)self;
    return statusBar ? static_cast<int>(statusBar->GetFieldsCount()) : 0;
}

WXD_EXPORTED void
wxd_StatusBar_SetStatusStyles(wxd_StatusBar_t* self, int count, const int* styles)
{
    wxStatusBar* statusBar = (wxStatusBar*)self;
    if (statusBar && count > 0 && styles) {
        statusBar->SetStatusStyles(count, styles);
    }
}

WXD_EXPORTED void
wxd_StatusBar_SetMinHeight(wxd_StatusBar_t* self, int height)
{
    wxStatusBar* statusBar = (wxStatusBar*)self;
    if (statusBar) {
        statusBar->SetMinHeight(height);
    }
}

// No wxd_StatusBar_Destroy needed, frame manages lifetime when SetStatusBar is called.

} // extern "C"