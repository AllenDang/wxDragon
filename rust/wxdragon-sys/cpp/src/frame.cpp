#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"
#include <wx/frame.h>
#include <wx/string.h>
#include <wx/gdicmn.h>
#include <wx/menu.h>
#include <wx/statusbr.h>

// --- Frame Functions Implementation ---

wxd_Frame_t*
wxd_Frame_Create(wxd_Window_t* parent, wxd_Id id, const char* title, wxd_Point pos, wxd_Size size,
                 wxd_Style_t style)
{
    wxWindow* wx_parent = reinterpret_cast<wxWindow*>(parent);

    // Create the wxFrame object
    wxFrame* wx_frame =
        new wxFrame(wx_parent, id,
                    wxString::FromUTF8(title ? title : ""), // Ensure non-null string
                    wxd_cpp_utils::to_wx(pos), wxd_cpp_utils::to_wx(size), style);

    // Return the opaque handle
    // Note: We are not creating/attaching the WxdEventHandler here.
    // It will be created lazily when the first event is bound via GetOrCreateEventHandler.
    return reinterpret_cast<wxd_Frame_t*>(wx_frame);
}

void
wxd_Frame_Destroy(wxd_Frame_t* frame)
{
    // This function tells wxWidgets to destroy the window.
    // The actual deletion of the C++ object and our WxdEventHandler
    // happens later in the event loop after the wxEVT_DESTROY event.
    if (!frame)
        return;
    wxFrame* wx_frame = reinterpret_cast<wxFrame*>(frame);
    wx_frame->Destroy();
}

void
wxd_Frame_Show(wxd_Frame_t* frame, bool show)
{
    if (!frame)
        return;
    wxFrame* wx_frame = reinterpret_cast<wxFrame*>(frame);
    wx_frame->Show(show);
}

void
wxd_Frame_SetTitle(wxd_Frame_t* frame, const char* title)
{
    if (!frame)
        return;
    wxFrame* wx_frame = reinterpret_cast<wxFrame*>(frame);
    wx_frame->SetTitle(wxString::FromUTF8(title ? title : ""));
}

void
wxd_Frame_Centre(wxd_Frame_t* frame, wxd_Direction_t direction)
{
    if (!frame)
        return;
    wxFrame* wx_frame = reinterpret_cast<wxFrame*>(frame);
    wx_frame->Centre(static_cast<wxDirection>(direction));
}

WXD_EXPORTED void
wxd_Frame_Close(wxd_Frame_t* frame, bool force)
{
    wxFrame* wx_frame = reinterpret_cast<wxFrame*>(frame);
    if (wx_frame)
        wx_frame->Close(force);
}

WXD_EXPORTED void
wxd_Frame_SetMenuBar(wxd_Frame_t* frame, wxd_MenuBar_t* menubar)
{
    wxFrame* fr = (wxFrame*)frame;
    wxMenuBar* mb = (wxMenuBar*)menubar;
    if (fr) {
        fr->SetMenuBar(mb);
    }
}

WXD_EXPORTED wxd_MenuBar_t*
wxd_Frame_GetMenuBar(wxd_Frame_t* frame)
{
    if (!frame)
        return nullptr;
    wxFrame* fr = reinterpret_cast<wxFrame*>(frame);
    wxMenuBar* mb = fr->GetMenuBar();
    return reinterpret_cast<wxd_MenuBar_t*>(mb);
}

WXD_EXPORTED void
wxd_Frame_SetStatusBar(wxd_Frame_t* frame, wxd_StatusBar_t* statusBar)
{
    if (frame && statusBar) {
        ((wxFrame*)frame)->SetStatusBar((wxStatusBar*)statusBar);
    }
}

// ADDED: Implementation for wxd_Frame_SetToolBar
WXD_EXPORTED void
wxd_Frame_SetToolBar(wxd_Frame_t* frame, wxd_ToolBar_t* toolBar)
{
    if (!frame)
        return;
    // Note: toolBar can be NULL to remove the toolbar
    reinterpret_cast<wxFrame*>(frame)->SetToolBar(reinterpret_cast<wxToolBar*>(toolBar));
}

// ADDED: Implementation for wxd_Frame_CreateToolBar
WXD_EXPORTED wxd_ToolBar_t*
wxd_Frame_CreateToolBar(wxd_Frame_t* frame, wxd_Style_t style, wxd_Id id)
{
    if (!frame)
        return nullptr;
    wxFrame* fr = reinterpret_cast<wxFrame*>(frame);
    // wxFrame::CreateToolBar creates, sets, and manages the toolbar.
    // It calls Realize() internally.
    wxToolBar* tb = fr->CreateToolBar(style, id);
    return reinterpret_cast<wxd_ToolBar_t*>(tb);
}

// ADDED: Implementation for wxd_Frame_CreateStatusBar
WXD_EXPORTED wxd_StatusBar_t*
wxd_Frame_CreateStatusBar(wxd_Frame_t* frame, int number, wxd_Style_t style, wxd_Id id,
                          const char* name)
{
    if (!frame)
        return nullptr;
    wxFrame* fr = reinterpret_cast<wxFrame*>(frame);
    // wxFrame::CreateStatusBar creates, sets, and manages the status bar.
    wxStatusBar* sb = fr->CreateStatusBar(number, style, id, wxString::FromUTF8(name ? name : ""));
    return reinterpret_cast<wxd_StatusBar_t*>(sb);
}

// New Frame method implementations
void
wxd_Frame_CenterOnScreen(wxd_Frame_t* frame)
{
    if (frame) {
        ((wxFrame*)frame)->CenterOnScreen();
    }
}

void
wxd_Frame_SetStatusText(wxd_Frame_t* frame, const char* text, int number)
{
    if (frame && text) {
        ((wxFrame*)frame)->SetStatusText(wxString::FromUTF8(text), number);
    }
}

char*
wxd_Frame_GetTitle(wxd_Frame_t* frame)
{
    if (frame) {
        wxString title = ((wxFrame*)frame)->GetTitle();
        return strdup(title.ToUTF8().data());
    }
    return strdup(""); // Return empty string if frame is null to avoid null pointer issues
}

void
wxd_Frame_Iconize(wxd_Frame_t* frame, bool iconize)
{
    if (frame) {
        ((wxFrame*)frame)->Iconize(iconize);
    }
}

bool
wxd_Frame_IsIconized(wxd_Frame_t* frame)
{
    if (frame) {
        return ((wxFrame*)frame)->IsIconized();
    }
    return false;
}

void
wxd_Frame_Maximize(wxd_Frame_t* frame, bool maximize)
{
    if (frame) {
        ((wxFrame*)frame)->Maximize(maximize);
    }
}

bool
wxd_Frame_IsMaximized(wxd_Frame_t* frame)
{
    if (frame) {
        return ((wxFrame*)frame)->IsMaximized();
    }
    return false;
}

void
wxd_Frame_SetIconFromBitmap(wxd_Frame_t* frame, const wxd_Bitmap_t* bitmap)
{
    if (!frame || !bitmap)
        return;

    wxFrame* wx_frame = reinterpret_cast<wxFrame*>(frame);
    const wxBitmap* wx_bitmap = reinterpret_cast<const wxBitmap*>(bitmap);

    if (!wx_bitmap->IsOk())
        return;

    // Convert wxBitmap to wxIcon and set it
    wxIcon icon;
    icon.CopyFromBitmap(*wx_bitmap);
    if (icon.IsOk()) {
        wx_frame->SetIcon(icon);
    }
}

// If a general wxd_rust_string_free is needed for other cases, it would go here or in a common utils.cpp
// For example:
// extern "C" void wxd_rust_string_free(char* str) {
//     if (str) {
//         free(str);
//     }
// }
