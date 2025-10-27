#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/toolbar.h>
#include <wx/bmpbndl.h>

// Helper to create a wxBitmapBundle from optional wxd_Bitmap_t pointers
wxBitmapBundle
CreateBundle(const wxd_Bitmap_t* bmpNormal, const wxd_Bitmap_t* bmpDisabled)
{
    const wxBitmap* normalPtr = reinterpret_cast<const wxBitmap*>(bmpNormal);
    // const wxBitmap* disabledPtr = reinterpret_cast<const wxBitmap*>(bmpDisabled);

    if (!normalPtr || !normalPtr->IsOk()) {
        // Must have at least a normal bitmap
        return wxBitmapBundle(); // Return invalid bundle
    }
    // wxToolBar::AddTool takes bundle by value, expect copy/ref-counting
    return wxBitmapBundle::FromBitmap(*normalPtr);

    // TODO: wxWidgets allows multiple bitmaps in a bundle for different states (hover, pressed)
    // and different resolutions. For now, just use the normal one.
    // If disabledPtr is valid, we could potentially add it, but wxBitmapBundle::FromBitmaps
    // isn't standard, and AddTool might handle wxNullBitmap for disabled state implicitly.
    // For now, returning bundle from just the normal bitmap is simplest.
}

WXD_EXPORTED wxd_ToolBar_t*
wxd_ToolBar_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    if (!parentWin)
        return nullptr;

    wxToolBar* tb =
        new wxToolBar(parentWin, id, wxPoint(pos.x, pos.y), wxSize(size.width, size.height), style);
    // wxToolBar is a wxWindow, cleanup is handled by parent like other controls.
    return reinterpret_cast<wxd_ToolBar_t*>(tb);
}

WXD_EXPORTED void*
wxd_ToolBar_AddTool(wxd_ToolBar_t* self, wxd_Id toolId, const char* label,
                    const wxd_Bitmap_t* bitmap, const wxd_Bitmap_t* bitmapDisabled, int kind,
                    const char* shortHelp, const char* longHelp)
{
    wxToolBar* tb = reinterpret_cast<wxToolBar*>(self);
    if (!tb)
        return nullptr;

    wxString wxLabel = wxString::FromUTF8(label ? label : "");
    wxString wxShortHelp = wxString::FromUTF8(shortHelp ? shortHelp : "");
    wxString wxLongHelp = wxString::FromUTF8(longHelp ? longHelp : "");
    wxItemKind wxKind = static_cast<wxItemKind>(kind);

    wxBitmapBundle bundle = CreateBundle(bitmap, bitmapDisabled);
    if (!bundle.IsOk()) {
        WXD_LOG_ERROR("wxd_ToolBar_AddTool: Invalid bitmap provided.");
        return nullptr;
    }

    wxToolBarToolBase* tool = tb->AddTool(toolId, wxLabel, bundle, wxShortHelp, wxKind);
    if (tool && !wxLongHelp.IsEmpty()) {
        tb->SetToolLongHelp(toolId, wxLongHelp);
    }
    // Return the tool pointer, though it's opaque for now in C API.
    return reinterpret_cast<void*>(tool);
}

WXD_EXPORTED void
wxd_ToolBar_AddSeparator(wxd_ToolBar_t* self)
{
    wxToolBar* tb = reinterpret_cast<wxToolBar*>(self);
    if (!tb)
        return;
    tb->AddSeparator();
}

WXD_EXPORTED void
wxd_ToolBar_AddControl(wxd_ToolBar_t* self, wxd_Window_t* control)
{
    wxToolBar* tb = reinterpret_cast<wxToolBar*>(self);
    wxWindow* win = reinterpret_cast<wxWindow*>(control);
    if (!tb || !win)
        return;

    // wxToolBar::AddControl expects wxControl*, perform a dynamic_cast
    wxControl* ctrl = dynamic_cast<wxControl*>(win);
    if (ctrl) {
        tb->AddControl(ctrl);
    }
    else {
        WXD_LOG_ERROR("wxd_ToolBar_AddControl: Provided window is not a wxControl.");
    }
}

WXD_EXPORTED bool
wxd_ToolBar_Realize(wxd_ToolBar_t* self)
{
    wxToolBar* tb = reinterpret_cast<wxToolBar*>(self);
    if (!tb)
        return false;
    return tb->Realize();
}

WXD_EXPORTED void
wxd_ToolBar_EnableTool(wxd_ToolBar_t* self, wxd_Id toolId, bool enable)
{
    wxToolBar* tb = reinterpret_cast<wxToolBar*>(self);
    if (!tb)
        return;
    tb->EnableTool(toolId, enable);
}

WXD_EXPORTED void
wxd_ToolBar_ToggleTool(wxd_ToolBar_t* self, wxd_Id toolId, bool toggle)
{
    wxToolBar* tb = reinterpret_cast<wxToolBar*>(self);
    if (!tb)
        return;
    tb->ToggleTool(toolId, toggle);
}

WXD_EXPORTED bool
wxd_ToolBar_IsToolEnabled(wxd_ToolBar_t* self, wxd_Id toolId)
{
    wxToolBar* tb = reinterpret_cast<wxToolBar*>(self);
    if (!tb)
        return false; // Or maybe true? Let's assume false if toolbar invalid.
    return tb->GetToolEnabled(toolId);
}

WXD_EXPORTED bool
wxd_ToolBar_GetToolState(wxd_ToolBar_t* self, wxd_Id toolId)
{
    wxToolBar* tb = reinterpret_cast<wxToolBar*>(self);
    if (!tb)
        return false;
    return tb->GetToolState(toolId);
}

WXD_EXPORTED void
wxd_ToolBar_SetToolShortHelp(wxd_ToolBar_t* self, wxd_Id toolId, const char* helpString)
{
    wxToolBar* tb = reinterpret_cast<wxToolBar*>(self);
    if (!tb)
        return;
    tb->SetToolShortHelp(toolId, wxString::FromUTF8(helpString ? helpString : ""));
}

// BitmapBundle support
WXD_EXPORTED bool
wxd_ToolBar_AddToolWithBundle(wxd_ToolBar_t* toolbar, wxd_Id id, const char* label,
                              wxd_BitmapBundle_t* bitmap)
{
    if (!toolbar)
        return false;

    wxToolBar* tb = reinterpret_cast<wxToolBar*>(toolbar);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bitmap);
    wxString wx_label = wxString::FromUTF8(label ? label : "");

    wxToolBarToolBase* tool = tb->AddTool(id, wx_label, bundlePtr ? *bundlePtr : wxBitmapBundle());

    return tool != nullptr;
}

WXD_EXPORTED bool
wxd_ToolBar_AddToolWithBundles(wxd_ToolBar_t* toolbar, wxd_Id id, const char* label,
                               wxd_BitmapBundle_t* bitmap, wxd_BitmapBundle_t* bitmapDisabled,
                               const char* shortHelp, const char* longHelp)
{
    if (!toolbar)
        return false;

    wxToolBar* tb = reinterpret_cast<wxToolBar*>(toolbar);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bitmap);
    wxBitmapBundle* disabledBundlePtr = reinterpret_cast<wxBitmapBundle*>(bitmapDisabled);

    wxString wx_label = wxString::FromUTF8(label ? label : "");
    wxString wx_shortHelp = wxString::FromUTF8(shortHelp ? shortHelp : "");
    wxString wx_longHelp = wxString::FromUTF8(longHelp ? longHelp : "");

    wxToolBarToolBase* tool = tb->AddTool(id, wx_label, bundlePtr ? *bundlePtr : wxBitmapBundle(),
                                          disabledBundlePtr ? *disabledBundlePtr : wxBitmapBundle(),
                                          wxITEM_NORMAL, wx_shortHelp, wx_longHelp);

    return tool != nullptr;
}