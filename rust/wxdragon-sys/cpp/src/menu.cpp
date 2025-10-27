#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/menu.h> // Include for wxMenuBar, wxMenu, wxMenuItem

extern "C" {

// --- MenuBar Functions ---
WXD_EXPORTED wxd_MenuBar_t*
wxd_MenuBar_Create(wxd_Style_t style)
{
    // Style is often 0 for default menubar
    wxMenuBar* menubar = new wxMenuBar(style);
    return reinterpret_cast<wxd_MenuBar_t*>(menubar);
}

WXD_EXPORTED void
wxd_MenuBar_Append(wxd_MenuBar_t* menubar, wxd_Menu_t* menu, const char* title)
{
    if (!menubar || !menu)
        return;
    wxMenuBar* wx_menubar = reinterpret_cast<wxMenuBar*>(menubar);
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    // wxMenuBar takes ownership of the wxMenu* pointer
    wx_menubar->Append(wx_menu, wxString::FromUTF8(title ? title : ""));
}

// --- Menu Functions ---
WXD_EXPORTED wxd_Menu_t*
wxd_Menu_Create(const char* title, wxd_Style_t style)
{
    wxMenu* menu = new wxMenu(wxString::FromUTF8(title ? title : ""), style);
    return reinterpret_cast<wxd_Menu_t*>(menu);
}

// @brief Destroy a wxMenu.
// WARNING: Only destroy standalone menus not owned by a wxMenuBar.
// If a wxMenu has been appended to a wxMenuBar, the menubar owns it and
// will delete it; destroying here would cause double free.
WXD_EXPORTED void
wxd_Menu_Destroy(wxd_Menu_t* menu)
{
    if (!menu)
        return;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    delete wx_menu;
}

WXD_EXPORTED wxd_MenuItem_t*
wxd_Menu_Append(wxd_Menu_t* menu, wxd_Id id, const char* item, const char* helpString, int kind)
{
    if (!menu)
        return nullptr;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxItemKind wx_kind = static_cast<wxItemKind>(kind);
    wxMenuItem* wx_item = wx_menu->Append(id, wxString::FromUTF8(item ? item : ""),
                                          wxString::FromUTF8(helpString ? helpString : ""),
                                          wx_kind);
    // wxMenu takes ownership of the wxMenuItem* it creates/appends.
    return reinterpret_cast<wxd_MenuItem_t*>(wx_item);
}

WXD_EXPORTED void
wxd_Menu_AppendSeparator(wxd_Menu_t* menu)
{
    if (!menu)
        return;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wx_menu->AppendSeparator();
}

// --- MenuItem Functions ---
WXD_EXPORTED void
wxd_MenuItem_Destroy(wxd_MenuItem_t* item)
{
    // Generally not needed - wxMenu manages item deletion.
    // If we created items *separately* and passed them to Append,
    // we might need this. But Append creates the item.
    // However, providing a stub might be harmless if called inappropriately.
    // wxMenuItem* wx_item = reinterpret_cast<wxMenuItem*>(item);
    // delete wx_item; // Risky - likely double free
    // Consider logging a warning if called?
}

// --- MenuItem State Functions ---
WXD_EXPORTED void
wxd_MenuItem_SetLabel(wxd_MenuItem_t* item, const char* label)
{
    if (!item)
        return;
    wxMenuItem* wx_item = reinterpret_cast<wxMenuItem*>(item);
    wx_item->SetItemLabel(wxString::FromUTF8(label ? label : ""));
}

WXD_EXPORTED char*
wxd_MenuItem_GetLabel(wxd_MenuItem_t* item)
{
    if (!item)
        return strdup(""); // Return duplicated empty string to avoid NULL
    wxMenuItem* wx_item = reinterpret_cast<wxMenuItem*>(item);
    wxString label = wx_item->GetItemLabel();
    const wxScopedCharBuffer utf8_buf = label.ToUTF8();
    if (utf8_buf.data()) {
        return strdup(utf8_buf.data()); // Allocate and copy string
    }
    return strdup(""); // Return duplicated empty string if conversion fails
}

WXD_EXPORTED void
wxd_MenuItem_Enable(wxd_MenuItem_t* item, bool enable)
{
    if (!item)
        return;
    wxMenuItem* wx_item = reinterpret_cast<wxMenuItem*>(item);
    wx_item->Enable(enable);
}

WXD_EXPORTED bool
wxd_MenuItem_IsEnabled(wxd_MenuItem_t* item)
{
    if (!item)
        return false;
    wxMenuItem* wx_item = reinterpret_cast<wxMenuItem*>(item);
    return wx_item->IsEnabled();
}

WXD_EXPORTED void
wxd_MenuItem_Check(wxd_MenuItem_t* item, bool check)
{
    if (!item)
        return;
    wxMenuItem* wx_item = reinterpret_cast<wxMenuItem*>(item);
    // Only check if it's a checkable item (Check or Radio)
    if (wx_item->IsCheckable()) {
        wx_item->Check(check);
    }
}

WXD_EXPORTED bool
wxd_MenuItem_IsChecked(wxd_MenuItem_t* item)
{
    if (!item)
        return false;
    wxMenuItem* wx_item = reinterpret_cast<wxMenuItem*>(item);
    return wx_item->IsChecked();
}

} // extern "C"