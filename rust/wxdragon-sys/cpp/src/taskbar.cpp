#include "wx/wxprec.h"
#ifndef WX_PRECOMP
#include "wx/wx.h"
#endif

#include "../include/wxdragon.h"

#if wxUSE_TASKBARICON

#include "wx/taskbar.h"
#include "wx/menu.h"
#include "wx/bitmap.h"
#include "wx/platinfo.h"

#ifdef __WXOSX__
#include "wx/osx/private.h"
#endif

// Custom TaskBarIcon class that exposes events properly and supports auto popup menus
class wxdTaskBarIcon : public wxTaskBarIcon {
public:
    explicit wxdTaskBarIcon(wxTaskBarIconType iconType)
        : wxTaskBarIcon(iconType), m_popupMenu(nullptr)
    {
    }

    // Make the event handler accessible
    wxEvtHandler*
    GetEvtHandler()
    {
        return static_cast<wxEvtHandler*>(this);
    }

    // Set the popup menu that will be shown automatically when the icon is clicked
    void
    SetPopupMenu(wxMenu* menu)
    {
        m_popupMenu = menu;
    }

    // Get the current popup menu
    wxMenu*
    GetPopupMenu() const
    {
        return m_popupMenu;
    }

protected:
    // Override CreatePopupMenu to provide automatic popup menu functionality
    virtual wxMenu*
    CreatePopupMenu() override
    {
        // If a popup menu has been set, return a copy of it
        // Note: wxWidgets will take ownership of the returned menu and delete it after use
        if (m_popupMenu) {
            // Create a copy of the menu since wxWidgets will delete the returned menu
            wxMenu* menuCopy = new wxMenu();

            // Copy all items from the original menu
            const wxMenuItemList& items = m_popupMenu->GetMenuItems();
            for (wxMenuItemList::const_iterator it = items.begin(); it != items.end(); ++it) {
                wxMenuItem* item = *it;
                if (item->IsSeparator()) {
                    menuCopy->AppendSeparator();
                }
                else {
                    menuCopy->Append(item->GetId(), item->GetItemLabel(), item->GetHelp(),
                                     item->GetKind());
                    // Copy enable/check state
                    menuCopy->Enable(item->GetId(), item->IsEnabled());
                    if (item->IsCheckable()) {
                        menuCopy->Check(item->GetId(), item->IsChecked());
                    }
                }
            }
            return menuCopy;
        }

        // Return nullptr if no popup menu is set (no automatic popup)
        return nullptr;
    }

private:
    wxMenu* m_popupMenu; // Pointer to the popup menu template (not owned by this class)
    wxDECLARE_NO_COPY_CLASS(wxdTaskBarIcon);
};

extern "C" {

// TaskBarIcon creation and destruction
wxd_TaskBarIcon_t*
wxd_TaskBarIcon_Create(wxd_TaskBarIconType_t iconType)
{
    wxTaskBarIconType wxIconType;
    switch (iconType) {
    case WXD_TASKBAR_ICON_DEFAULT:
        // On macOS, use CustomStatusItem for menu bar (system tray)
        // On other platforms, use the platform's default
#ifdef __WXOSX__
        wxIconType = wxTBI_CUSTOM_STATUSITEM;
#else
        wxIconType = wxTBI_DOCK;
#endif
        break;
    case WXD_TASKBAR_ICON_DOCK:
        wxIconType = wxTBI_DOCK;
        break;
    case WXD_TASKBAR_ICON_CUSTOM_STATUSITEM:
        wxIconType = wxTBI_CUSTOM_STATUSITEM;
        break;
    default:
        return nullptr;
    }

    wxdTaskBarIcon* taskBarIcon = new wxdTaskBarIcon(wxIconType);
    return reinterpret_cast<wxd_TaskBarIcon_t*>(taskBarIcon);
}

void
wxd_TaskBarIcon_Destroy(wxd_TaskBarIcon_t* taskbar)
{
    if (taskbar) {
        wxdTaskBarIcon* wx_taskbar = reinterpret_cast<wxdTaskBarIcon*>(taskbar);
        delete wx_taskbar;
    }
}

// Icon operations
bool
wxd_TaskBarIcon_SetIcon(wxd_TaskBarIcon_t* taskbar, const wxd_Bitmap_t* icon, const char* tooltip)
{
    if (!taskbar)
        return false;

    wxdTaskBarIcon* wx_taskbar = reinterpret_cast<wxdTaskBarIcon*>(taskbar);

    if (icon) {
        const wxBitmap* wx_bitmap = reinterpret_cast<const wxBitmap*>(icon);
        wxString wx_tooltip = tooltip ? wxString::FromUTF8(tooltip) : wxString();

        // Convert wxBitmap to wxBitmapBundle
        wxBitmapBundle bundle(*wx_bitmap);

        return wx_taskbar->SetIcon(bundle, wx_tooltip);
    }
    else {
        // Remove icon if no bitmap provided
        return wx_taskbar->RemoveIcon();
    }
}

bool
wxd_TaskBarIcon_SetIconBundle(wxd_TaskBarIcon_t* taskbar, const wxd_BitmapBundle_t* iconBundle,
                              const char* tooltip)
{
    if (!taskbar)
        return false;

    wxdTaskBarIcon* wx_taskbar = reinterpret_cast<wxdTaskBarIcon*>(taskbar);

    if (iconBundle) {
        const wxBitmapBundle* wx_bundle = reinterpret_cast<const wxBitmapBundle*>(iconBundle);
        wxString wx_tooltip = tooltip ? wxString::FromUTF8(tooltip) : wxString();
        return wx_taskbar->SetIcon(*wx_bundle, wx_tooltip);
    }
    else {
        // Remove icon if no bundle provided
        return wx_taskbar->RemoveIcon();
    }
}

bool
wxd_TaskBarIcon_RemoveIcon(wxd_TaskBarIcon_t* taskbar)
{
    if (!taskbar)
        return false;

    wxdTaskBarIcon* wx_taskbar = reinterpret_cast<wxdTaskBarIcon*>(taskbar);
    return wx_taskbar->RemoveIcon();
}

bool
wxd_TaskBarIcon_IsIconInstalled(wxd_TaskBarIcon_t* taskbar)
{
    if (!taskbar)
        return false;

    wxdTaskBarIcon* wx_taskbar = reinterpret_cast<wxdTaskBarIcon*>(taskbar);
    return wx_taskbar->IsIconInstalled();
}

// Balloon tooltip operations (Windows only)
bool
wxd_TaskBarIcon_ShowBalloon(wxd_TaskBarIcon_t* taskbar, const char* title, const char* text,
                            unsigned int timeout, int flags, const wxd_BitmapBundle_t* icon)
{
    if (!taskbar)
        return false;

    wxdTaskBarIcon* wx_taskbar = reinterpret_cast<wxdTaskBarIcon*>(taskbar);

    wxString wx_title = title ? wxString::FromUTF8(title) : wxString();
    wxString wx_text = text ? wxString::FromUTF8(text) : wxString();

#if wxUSE_TASKBARICON_BALLOONS
    if (icon) {
        const wxBitmapBundle* wx_bundle = reinterpret_cast<const wxBitmapBundle*>(icon);
        return wx_taskbar->ShowBalloon(wx_title, wx_text, timeout, flags, *wx_bundle);
    }
    else {
        return wx_taskbar->ShowBalloon(wx_title, wx_text, timeout, flags);
    }
#else
    // Balloon tooltips not supported on this platform
    return false;
#endif
}

// Menu operations
bool
wxd_TaskBarIcon_PopupMenu(wxd_TaskBarIcon_t* taskbar, wxd_Menu_t* menu)
{
    if (!taskbar || !menu)
        return false;

    wxdTaskBarIcon* wx_taskbar = reinterpret_cast<wxdTaskBarIcon*>(taskbar);
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);

    return wx_taskbar->PopupMenu(wx_menu);
}

// Event handler pointer for binding events
wxd_EvtHandler_t*
wxd_TaskBarIcon_GetEvtHandler(wxd_TaskBarIcon_t* taskbar)
{
    if (!taskbar)
        return nullptr;

    wxdTaskBarIcon* wx_taskbar = reinterpret_cast<wxdTaskBarIcon*>(taskbar);
    return reinterpret_cast<wxd_EvtHandler_t*>(wx_taskbar->GetEvtHandler());
}

// Automatic popup menu operations
void
wxd_TaskBarIcon_SetPopupMenu(wxd_TaskBarIcon_t* taskbar, wxd_Menu_t* menu)
{
    if (!taskbar)
        return;

    wxdTaskBarIcon* wx_taskbar = reinterpret_cast<wxdTaskBarIcon*>(taskbar);
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);

    wx_taskbar->SetPopupMenu(wx_menu);
}

const wxd_Menu_t*
wxd_TaskBarIcon_GetPopupMenu(wxd_TaskBarIcon_t* taskbar)
{
    if (!taskbar)
        return nullptr;

    wxdTaskBarIcon* wx_taskbar = reinterpret_cast<wxdTaskBarIcon*>(taskbar);
    const wxMenu* wx_menu = wx_taskbar->GetPopupMenu();

    return reinterpret_cast<const wxd_Menu_t*>(wx_menu);
}

} // extern "C"

#endif // wxUSE_TASKBARICON