#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h" // Main header for wxDragon C API
#include <wx/aui/auibar.h>       // For wxAuiToolBar
#include <wx/window.h>           // For wxWindow
#include <wx/control.h>          // For wxControl
#include <wx/string.h>           // For wxString
#include <wx/gdicmn.h>           // For wxPoint, wxSize, wxID_ANY, etc.
#include "wxd_utils.h"           // For WXD_STR_TO_WX_STRING_UTF8_NULL_OK, etc.

// --- wxAuiToolBar ---

extern "C" {

WXD_EXPORTED wxd_AuiToolBar_t*
wxd_AuiToolBar_Create(wxd_Window_t* parent, int id, wxd_Point pos, wxd_Size size, int64_t style)
{
    wxWindow* wx_parent = (wxWindow*)parent;
    // wxID_ANY is -1, which is a common default for id.
    // wxDefaultPosition is wxPoint(-1, -1)
    // wxDefaultSize is wxSize(-1, -1)
    // style could be wxAUI_TB_DEFAULT_STYLE
    wxAuiToolBar* toolbar = new wxAuiToolBar(wx_parent, id, wxPoint(pos.x, pos.y),
                                             wxSize(size.width, size.height), style);
    return (wxd_AuiToolBar_t*)toolbar;
}

WXD_EXPORTED void
wxd_AuiToolBar_AddTool(
    wxd_AuiToolBar_t* self, int tool_id, const char* label,
    /*wxd_Bitmap_t* bitmap, wxd_Bitmap_t* disabled_bitmap,*/ const char* short_help_string,
    WXDItemKindCEnum kind)
{
    if (!self)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;

    wxString wx_label = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(label);
    wxString wx_short_help = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(short_help_string);

    wxItemKind wx_kind;
    switch (kind) {
    case WXD_ITEM_NORMAL:
        wx_kind = wxITEM_NORMAL;
        break;
    case WXD_ITEM_CHECK:
        wx_kind = wxITEM_CHECK;
        break;
    case WXD_ITEM_RADIO:
        wx_kind = wxITEM_RADIO;
        break;
    case WXD_ITEM_SEPARATOR:
        wx_kind = wxITEM_SEPARATOR;
        break;
    default:
        wx_kind = wxITEM_NORMAL;
    }

    // wxWidgets wxAuiToolBar::AddTool requires a bitmap.
    // Using wxNullBitmap for now.
    /* wxAuiToolBarItem* item = */ toolbar->AddTool(tool_id, wx_label, wxNullBitmap, wx_short_help,
                                                    wx_kind);
    // if (item) {} // item is wxAuiToolBarItem*, not used for void return
}

WXD_EXPORTED void
wxd_AuiToolBar_AddLabel(wxd_AuiToolBar_t* self, int tool_id, const char* label, int width)
{
    if (!self)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    wxString wx_label = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(label);
    /* wxAuiToolBarItem* item = */ toolbar->AddLabel(tool_id, wx_label, width);
}

WXD_EXPORTED void
wxd_AuiToolBar_AddControl(wxd_AuiToolBar_t* self, wxd_Control_t* control, const char* label)
{
    if (!self || !control)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    wxControl* wx_control = (wxControl*)control;
    wxString wx_label = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(label);
    /* wxAuiToolBarItem* item = */ toolbar->AddControl(wx_control, wx_label);
}

WXD_EXPORTED void
wxd_AuiToolBar_AddSeparator(wxd_AuiToolBar_t* self)
{
    if (!self)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    toolbar->AddSeparator();
}

WXD_EXPORTED void
wxd_AuiToolBar_AddSpacer(wxd_AuiToolBar_t* self, int pixels)
{
    if (!self)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    toolbar->AddSpacer(pixels);
}

WXD_EXPORTED void
wxd_AuiToolBar_AddStretchSpacer(wxd_AuiToolBar_t* self, int proportion)
{
    if (!self)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    toolbar->AddStretchSpacer(proportion);
}

WXD_EXPORTED void
wxd_AuiToolBar_Realize(wxd_AuiToolBar_t* self)
{
    if (!self)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    toolbar->Realize();
}

WXD_EXPORTED void
wxd_AuiToolBar_SetToolBitmapSize(wxd_AuiToolBar_t* self, wxd_Size size)
{
    if (!self)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    toolbar->SetToolBitmapSize(wxSize(size.width, size.height));
}

WXD_EXPORTED wxd_Size
wxd_AuiToolBar_GetToolBitmapSize(wxd_AuiToolBar_t* self)
{
    if (!self)
        return wxd_Size{ -1, -1 };
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    wxSize wx_size = toolbar->GetToolBitmapSize();
    return wxd_Size{ wx_size.x, wx_size.y };
}

WXD_EXPORTED void
wxd_AuiToolBar_SetOverflowVisible(wxd_AuiToolBar_t* self, bool visible)
{
    if (!self)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    toolbar->SetOverflowVisible(visible);
}

WXD_EXPORTED bool
wxd_AuiToolBar_GetOverflowVisible(wxd_AuiToolBar_t* self)
{
    if (!self)
        return false; // Default or error value
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    return toolbar->GetOverflowVisible();
}

WXD_EXPORTED void
wxd_AuiToolBar_SetGripperVisible(wxd_AuiToolBar_t* self, bool visible)
{
    if (!self)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    toolbar->SetGripperVisible(visible);
}

WXD_EXPORTED bool
wxd_AuiToolBar_GetGripperVisible(wxd_AuiToolBar_t* self)
{
    if (!self)
        return false; // Default or error value
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    return toolbar->GetGripperVisible();
}

WXD_EXPORTED void
wxd_AuiToolBar_SetToolDropDown(wxd_AuiToolBar_t* self, int tool_id, bool dropdown)
{
    if (!self)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    toolbar->SetToolDropDown(tool_id, dropdown);
}

WXD_EXPORTED bool
wxd_AuiToolBar_GetToolDropDown(wxd_AuiToolBar_t* self, int tool_id)
{
    if (!self)
        return false; // Default or error value
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    return toolbar->GetToolDropDown(tool_id);
}

WXD_EXPORTED void
wxd_AuiToolBar_EnableTool(wxd_AuiToolBar_t* self, int tool_id, bool enable)
{
    if (!self)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    toolbar->EnableTool(tool_id, enable);
}

WXD_EXPORTED bool
wxd_AuiToolBar_GetToolEnabled(wxd_AuiToolBar_t* self, int tool_id)
{
    if (!self)
        return false; // Default or error value
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    return toolbar->GetToolEnabled(tool_id);
}

WXD_EXPORTED int
wxd_AuiToolBar_GetToolCount(wxd_AuiToolBar_t* self)
{
    if (!self)
        return 0; // Default or error value
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    return toolbar->GetToolCount();
}

WXD_EXPORTED void
wxd_AuiToolBar_ClearTools(wxd_AuiToolBar_t* self)
{
    if (!self)
        return;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    toolbar->ClearTools();
}

WXD_EXPORTED bool
wxd_AuiToolBar_DeleteTool(wxd_AuiToolBar_t* self, int tool_id)
{
    if (!self)
        return false;
    wxAuiToolBar* toolbar = (wxAuiToolBar*)self;
    return toolbar->DeleteTool(tool_id);
}

} // extern "C"