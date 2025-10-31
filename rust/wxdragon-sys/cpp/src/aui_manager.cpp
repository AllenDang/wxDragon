#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"

#include <wx/aui/framemanager.h>
#include <wx/aui/auibook.h>

// Direction constants for AddPane function (matching wxAUI constants)
#define WXD_AUI_DOCK_LEFT   (0)
#define WXD_AUI_DOCK_RIGHT  (1)
#define WXD_AUI_DOCK_TOP    (2)
#define WXD_AUI_DOCK_BOTTOM (3)
#define WXD_AUI_DOCK_CENTER (4)

// Wrap wxAuiManager as wxd_AuiManager_t
struct wxd_AuiManager_t {
    wxAuiManager* manager;

    wxd_AuiManager_t(wxAuiManager* mgr) : manager(mgr)
    {
    }
    ~wxd_AuiManager_t()
    {
    }
};

// Wrap wxAuiPaneInfo as wxd_AuiPaneInfo_t
struct wxd_AuiPaneInfo_t {
    wxAuiPaneInfo info;

    wxd_AuiPaneInfo_t()
    {
    }
    wxd_AuiPaneInfo_t(const wxAuiPaneInfo& i) : info(i)
    {
    }
};

// --- wxAuiManager implementation ---

extern "C" {

wxd_AuiManager_t*
wxd_AuiManager_Create()
{
    wxAuiManager* manager = new wxAuiManager();
    return new wxd_AuiManager_t(manager);
}

void
wxd_AuiManager_SetManagedWindow(wxd_AuiManager_t* self, wxd_Window_t* managed_window)
{
    if (!self || !self->manager || !managed_window)
        return;
    wxWindow* window = reinterpret_cast<wxWindow*>(managed_window);
    self->manager->SetManagedWindow(window);
}

wxd_Window_t*
wxd_AuiManager_GetManagedWindow(wxd_AuiManager_t* self)
{
    if (!self || !self->manager)
        return nullptr;
    wxWindow* window = self->manager->GetManagedWindow();
    if (!window)
        return nullptr;
    return reinterpret_cast<wxd_Window_t*>(window);
}

void
wxd_AuiManager_UnInit(wxd_AuiManager_t* self)
{
    if (!self || !self->manager)
        return;
    self->manager->UnInit();
}

bool
wxd_AuiManager_AddPane(wxd_AuiManager_t* self, wxd_Window_t* window, int direction,
                       const char* caption)
{
    if (!self || !self->manager || !window)
        return false;

    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wxString wx_caption = wxString::FromUTF8(caption ? caption : "");

    wxAuiPaneInfo pane_info;

    // Set direction based on the parameter
    switch (direction) {
    case WXD_AUI_DOCK_LEFT:
        pane_info.Left();
        break;
    case WXD_AUI_DOCK_RIGHT:
        pane_info.Right();
        break;
    case WXD_AUI_DOCK_TOP:
        pane_info.Top();
        break;
    case WXD_AUI_DOCK_BOTTOM:
        pane_info.Bottom();
        break;
    case WXD_AUI_DOCK_CENTER:
        pane_info.CenterPane();
        break;
    default:
        pane_info.CenterPane();
        break;
    }

    // Set caption if provided
    if (caption && *caption) {
        pane_info.Caption(wx_caption);
    }

    return self->manager->AddPane(wx_window, pane_info);
}

bool
wxd_AuiManager_AddPaneWithInfo(wxd_AuiManager_t* self, wxd_Window_t* window,
                               wxd_AuiPaneInfo_t* pane_info)
{
    if (!self || !self->manager || !window || !pane_info)
        return false;

    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    return self->manager->AddPane(wx_window, pane_info->info);
}

bool
wxd_AuiManager_Update(wxd_AuiManager_t* self)
{
    if (!self || !self->manager)
        return false;
    self->manager->Update();
    return true;
}

void
wxd_AuiManager_Delete(wxd_AuiManager_t* self)
{
    if (!self)
        return;

    if (self->manager) {
        // UnInit the manager first to ensure proper cleanup
        if (self->manager->GetManagedWindow()) {
            self->manager->UnInit();
        }
        delete self->manager;
    }

    delete self;
}

WXD_EXPORTED int
wxd_AuiManager_SavePerspective(const wxd_AuiManager_t* self, char* out, size_t out_len)
{
    if (!self || !self->manager)
        return -1;

    wxString perspective = self->manager->SavePerspective();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(perspective, out, out_len);
}

bool
wxd_AuiManager_LoadPerspective(wxd_AuiManager_t* self, const char* perspective, bool update)
{
    if (!self || !self->manager || !perspective)
        return false;

    wxString wx_perspective = wxString::FromUTF8(perspective);
    bool result = self->manager->LoadPerspective(wx_perspective, update);
    return result;
}

bool
wxd_AuiManager_DetachPane(wxd_AuiManager_t* self, wxd_Window_t* window)
{
    if (!self || !self->manager || !window)
        return false;

    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    return self->manager->DetachPane(wx_window);
}

// --- wxAuiPaneInfo implementation ---

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Create()
{
    return new wxd_AuiPaneInfo_t();
}

void
wxd_AuiPaneInfo_Delete(wxd_AuiPaneInfo_t* self)
{
    if (self)
        delete self;
}

// Chain methods return the self pointer to allow for method chaining in the Rust API
wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Name(wxd_AuiPaneInfo_t* self, const char* name)
{
    if (!self || !name)
        return self;
    self->info.Name(wxString::FromUTF8(name));
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Caption(wxd_AuiPaneInfo_t* self, const char* caption)
{
    if (!self || !caption)
        return self;
    self->info.Caption(wxString::FromUTF8(caption));
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Left(wxd_AuiPaneInfo_t* self)
{
    if (!self)
        return self;
    self->info.Left();
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Right(wxd_AuiPaneInfo_t* self)
{
    if (!self)
        return self;
    self->info.Right();
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Top(wxd_AuiPaneInfo_t* self)
{
    if (!self)
        return self;
    self->info.Top();
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Bottom(wxd_AuiPaneInfo_t* self)
{
    if (!self)
        return self;
    self->info.Bottom();
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Center(wxd_AuiPaneInfo_t* self)
{
    if (!self)
        return self;
    self->info.Center();
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Floatable(wxd_AuiPaneInfo_t* self, bool floatable)
{
    if (!self)
        return self;
    self->info.Floatable(floatable);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Dockable(wxd_AuiPaneInfo_t* self, bool dockable)
{
    if (!self)
        return self;
    self->info.Dockable(dockable);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Movable(wxd_AuiPaneInfo_t* self, bool movable)
{
    if (!self)
        return self;
    self->info.Movable(movable);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Resizable(wxd_AuiPaneInfo_t* self, bool resizable)
{
    if (!self)
        return self;
    self->info.Resizable(resizable);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_CloseButton(wxd_AuiPaneInfo_t* self, bool visible)
{
    if (!self)
        return self;
    self->info.CloseButton(visible);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_MaximizeButton(wxd_AuiPaneInfo_t* self, bool visible)
{
    if (!self)
        return self;
    self->info.MaximizeButton(visible);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_MinimizeButton(wxd_AuiPaneInfo_t* self, bool visible)
{
    if (!self)
        return self;
    self->info.MinimizeButton(visible);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_PinButton(wxd_AuiPaneInfo_t* self, bool visible)
{
    if (!self)
        return self;
    self->info.PinButton(visible);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_PaneBorder(wxd_AuiPaneInfo_t* self, bool visible)
{
    if (!self)
        return self;
    self->info.PaneBorder(visible);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Gripper(wxd_AuiPaneInfo_t* self, bool visible)
{
    if (!self)
        return self;
    self->info.Gripper(visible);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_GripperTop(wxd_AuiPaneInfo_t* self, bool attop)
{
    if (!self)
        return self;
    self->info.GripperTop(attop);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Layer(wxd_AuiPaneInfo_t* self, int layer)
{
    if (!self)
        return self;
    self->info.Layer(layer);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_MinSize(wxd_AuiPaneInfo_t* self, int width, int height)
{
    if (!self)
        return self;
    self->info.MinSize(wxSize(width, height));
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_MaxSize(wxd_AuiPaneInfo_t* self, int width, int height)
{
    if (!self)
        return self;
    self->info.MaxSize(wxSize(width, height));
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Position(wxd_AuiPaneInfo_t* self, int pos)
{
    if (!self)
        return self;
    self->info.Position(pos);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Row(wxd_AuiPaneInfo_t* self, int row)
{
    if (!self)
        return self;
    self->info.Row(row);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_CenterPane(wxd_AuiPaneInfo_t* self)
{
    if (!self)
        return self;
    self->info.CenterPane();
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_DefaultPane(wxd_AuiPaneInfo_t* self)
{
    if (!self)
        return self;
    self->info.DefaultPane();
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_ToolbarPane(wxd_AuiPaneInfo_t* self)
{
    if (!self)
        return self;
    self->info.ToolbarPane();
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_BestSize(wxd_AuiPaneInfo_t* self, int width, int height)
{
    if (!self)
        return self;
    self->info.BestSize(wxSize(width, height));
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Show(wxd_AuiPaneInfo_t* self, bool show)
{
    if (!self)
        return self;
    self->info.Show(show);
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Hide(wxd_AuiPaneInfo_t* self)
{
    if (!self)
        return self;
    self->info.Hide();
    return self;
}

wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_CaptionVisible(wxd_AuiPaneInfo_t* self, bool visible)
{
    if (!self)
        return self;
    self->info.CaptionVisible(visible);
    return self;
}

} // extern "C"