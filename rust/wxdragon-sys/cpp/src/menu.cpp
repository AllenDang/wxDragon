#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/menu.h>  // Include for wxMenuBar, wxMenu, wxMenuItem
#include <wx/frame.h> // Needed for obtaining owning frame from menubar
#include <cstring>    // C runtime for strlen/memcpy

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

WXD_EXPORTED bool
wxd_MenuBar_EnableItem(wxd_MenuBar_t* menubar, wxd_Id id, bool enable)
{
    if (!menubar)
        return false;
    wxMenuBar* wx_menubar = reinterpret_cast<wxMenuBar*>(menubar);
    wxMenuItem* item = wx_menubar->FindItem(id);
    if (!item)
        return false;
    wx_menubar->Enable(id, enable);
    return true;
}

WXD_EXPORTED bool
wxd_MenuBar_IsItemEnabled(const wxd_MenuBar_t* menubar, wxd_Id id)
{
    if (!menubar)
        return false;
    const wxMenuBar* wx_menubar = reinterpret_cast<const wxMenuBar*>(menubar);
    return wx_menubar->IsEnabled(id);
}

WXD_EXPORTED void
wxd_MenuBar_CheckItem(wxd_MenuBar_t* menubar, wxd_Id id, bool check)
{
    if (!menubar)
        return;
    wxMenuBar* wx_menubar = reinterpret_cast<wxMenuBar*>(menubar);
    wx_menubar->Check(id, check);
}

WXD_EXPORTED bool
wxd_MenuBar_IsItemChecked(const wxd_MenuBar_t* menubar, wxd_Id id)
{
    if (!menubar)
        return false;
    const wxMenuBar* wx_menubar = reinterpret_cast<const wxMenuBar*>(menubar);
    return wx_menubar->IsChecked(id);
}

WXD_EXPORTED wxd_MenuItem_t*
wxd_MenuBar_FindItem(wxd_MenuBar_t* menubar, wxd_Id id, wxd_Menu_t** menu)
{
    if (!menubar)
        return nullptr;
    wxMenuBar* wx_menubar = reinterpret_cast<wxMenuBar*>(menubar);
    wxMenu* wx_found_menu = nullptr;
    wxMenuItem* item = wx_menubar->FindItem(id, menu ? &wx_found_menu : nullptr);
    
    if (menu) {
        *menu = reinterpret_cast<wxd_Menu_t*>(wx_found_menu);
    }
    
    return reinterpret_cast<wxd_MenuItem_t*>(item);
}

// --- Menu Functions ---
WXD_EXPORTED wxd_Menu_t*
wxd_Menu_Create(const char* title, wxd_Style_t style)
{
    wxMenu* menu = new wxMenu(wxString::FromUTF8(title ? title : ""), style);
    return reinterpret_cast<wxd_Menu_t*>(menu);
}

WXD_EXPORTED size_t
wxd_Menu_GetMenuItemCount(const wxd_Menu_t* menu)
{
    if (!menu)
        return 0;
    const wxMenu* wx_menu = reinterpret_cast<const wxMenu*>(menu);
    return wx_menu->GetMenuItemCount();
}

// @brief Get the title of the wxMenu in UTF-8.
// If buffer is non-null and buffer_size > 0, copies up to buffer_size - 1 bytes and NUL-terminates the buffer.
// If buffer is null or buffer_size == 0, nothing is written.
// @param menu Pointer to wxMenu.
// @param buffer Buffer to receive the UTF-8 title.
// @param buffer_size Size of the buffer.
// @return Number of bytes required to store the UTF-8 title, excluding the terminating NUL, or -1 on error.
WXD_EXPORTED int
wxd_Menu_GetTitle(const wxd_Menu_t* menu, char* buffer, size_t buffer_size)
{
    if (!menu) {
        return -1;
    }

    const wxMenu* wx_menu = reinterpret_cast<const wxMenu*>(menu);
    const wxString title = wx_menu->GetTitle();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(title, buffer, buffer_size);
}

WXD_EXPORTED void
wxd_Menu_SetTitle(wxd_Menu_t* menu, const char* title)
{
    if (!menu)
        return;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wx_menu->SetTitle(wxString::FromUTF8(title ? title : ""));
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

WXD_EXPORTED const wxd_MenuItem_t*
wxd_Menu_AppendSubMenu(wxd_Menu_t* menu, wxd_Menu_t* submenu, const char* title,
                       const char* helpString)
{
    if (!menu || !submenu)
        return nullptr;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxMenu* wx_submenu = reinterpret_cast<wxMenu*>(submenu);
    wxMenuItem* wx_item = wx_menu->AppendSubMenu(wx_submenu, wxString::FromUTF8(title ? title : ""),
                                                 wxString::FromUTF8(helpString ? helpString : ""));
    // wxMenu takes ownership of the wxMenuItem* it creates/appends.
    return reinterpret_cast<const wxd_MenuItem_t*>(wx_item);
}

WXD_EXPORTED bool
wxd_Menu_ItemEnable(wxd_Menu_t* menu, wxd_Id id, bool enable)
{
    if (!menu)
        return false;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxMenuItem* item = wx_menu->FindItem(id);
    if (!item)
        return false;
    item->Enable(enable);
    return true;
}

WXD_EXPORTED bool
wxd_Menu_IsItemEnabled(const wxd_Menu_t* menu, wxd_Id id)
{
    if (!menu)
        return false;
    const wxMenu* wx_menu = reinterpret_cast<const wxMenu*>(menu);
    const wxMenuItem* item = wx_menu->FindItem(id);
    if (!item)
        return false;
    return item->IsEnabled();
}

WXD_EXPORTED void
wxd_Menu_CheckItem(wxd_Menu_t* menu, wxd_Id id, bool check)
{
    if (!menu)
        return;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wx_menu->Check(id, check);
}

WXD_EXPORTED bool
wxd_Menu_IsItemChecked(const wxd_Menu_t* menu, wxd_Id id)
{
    if (!menu)
        return false;
    const wxMenu* wx_menu = reinterpret_cast<const wxMenu*>(menu);
    return wx_menu->IsChecked(id);
}

WXD_EXPORTED wxd_MenuItem_t*
wxd_Menu_FindItem(wxd_Menu_t* menu, wxd_Id id)
{
    if (!menu)
        return nullptr;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxMenuItem* item = wx_menu->FindItem(id);
    return reinterpret_cast<wxd_MenuItem_t*>(item);
}

WXD_EXPORTED void
wxd_Menu_AppendSeparator(wxd_Menu_t* menu)
{
    if (!menu)
        return;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wx_menu->AppendSeparator();
}

WXD_EXPORTED wxd_MenuItem_t*
wxd_Menu_Insert(wxd_Menu_t* menu, size_t pos, wxd_Id id, const char* item, const char* helpString, int kind)
{
    if (!menu)
        return nullptr;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxItemKind wx_kind = static_cast<wxItemKind>(kind);
    // wxMenu::Insert takes: pos, id, text, help, kind
    wxMenuItem* wx_item = wx_menu->Insert(pos, id, wxString::FromUTF8(item ? item : ""),
                                          wxString::FromUTF8(helpString ? helpString : ""),
                                          wx_kind);
    return reinterpret_cast<wxd_MenuItem_t*>(wx_item);
}

WXD_EXPORTED const wxd_MenuItem_t*
wxd_Menu_InsertSubMenu(wxd_Menu_t* menu, size_t pos, wxd_Menu_t* submenu, const char* title, const char* helpString)
{
    if (!menu || !submenu)
        return nullptr;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxMenu* wx_submenu = reinterpret_cast<wxMenu*>(submenu);
    wxMenuItem* wx_item = wx_menu->Insert(pos, wxID_ANY, wxString::FromUTF8(title ? title : ""),
                                          wx_submenu, wxString::FromUTF8(helpString ? helpString : ""));
    return reinterpret_cast<const wxd_MenuItem_t*>(wx_item);
}

WXD_EXPORTED wxd_MenuItem_t*
wxd_Menu_InsertSeparator(wxd_Menu_t* menu, size_t pos)
{
    if (!menu)
        return nullptr;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxMenuItem* wx_item = wx_menu->InsertSeparator(pos);
    return reinterpret_cast<wxd_MenuItem_t*>(wx_item);
}

WXD_EXPORTED wxd_MenuItem_t*
wxd_Menu_Prepend(wxd_Menu_t* menu, wxd_Id id, const char* item, const char* helpString, int kind)
{
    if (!menu)
        return nullptr;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxItemKind wx_kind = static_cast<wxItemKind>(kind);
    wxMenuItem* wx_item = wx_menu->Prepend(id, wxString::FromUTF8(item ? item : ""),
                                           wxString::FromUTF8(helpString ? helpString : ""),
                                           wx_kind);
    return reinterpret_cast<wxd_MenuItem_t*>(wx_item);
}

WXD_EXPORTED const wxd_MenuItem_t*
wxd_Menu_PrependSubMenu(wxd_Menu_t* menu, wxd_Menu_t* submenu, const char* title, const char* helpString)
{
    if (!menu || !submenu)
        return nullptr;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxMenu* wx_submenu = reinterpret_cast<wxMenu*>(submenu);
    wxMenuItem* wx_item = wx_menu->Prepend(wxID_ANY, wxString::FromUTF8(title ? title : ""),
                                           wx_submenu, wxString::FromUTF8(helpString ? helpString : ""));
    return reinterpret_cast<const wxd_MenuItem_t*>(wx_item);
}

WXD_EXPORTED wxd_MenuItem_t*
wxd_Menu_PrependSeparator(wxd_Menu_t* menu)
{
    if (!menu)
        return nullptr;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxMenuItem* wx_item = wx_menu->PrependSeparator();
    return reinterpret_cast<wxd_MenuItem_t*>(wx_item);
}

WXD_EXPORTED wxd_MenuItem_t*
wxd_Menu_Remove(wxd_Menu_t* menu, wxd_Id id)
{
    if (!menu)
        return nullptr;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxMenuItem* wx_item = wx_menu->Remove(id);
    return reinterpret_cast<wxd_MenuItem_t*>(wx_item);
}

WXD_EXPORTED wxd_MenuItem_t*
wxd_Menu_RemoveItem(wxd_Menu_t* menu, wxd_MenuItem_t* item)
{
    if (!menu || !item)
        return nullptr;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxMenuItem* wx_item = reinterpret_cast<wxMenuItem*>(item);
    wxMenuItem* removed = wx_menu->Remove(wx_item);
    return reinterpret_cast<wxd_MenuItem_t*>(removed);
}

WXD_EXPORTED bool
wxd_Menu_Delete(wxd_Menu_t* menu, wxd_Id id)
{
    if (!menu)
        return false;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    return wx_menu->Delete(id);
}

WXD_EXPORTED bool
wxd_Menu_DeleteItem(wxd_Menu_t* menu, wxd_MenuItem_t* item)
{
    if (!menu || !item)
        return false;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxMenuItem* wx_item = reinterpret_cast<wxMenuItem*>(item);
    return wx_menu->Delete(wx_item);
}

WXD_EXPORTED wxd_MenuItem_t*
wxd_Menu_FindItemByPosition(const wxd_Menu_t* menu, size_t pos)
{
    if (!menu)
        return nullptr;
    const wxMenu* wx_menu = reinterpret_cast<const wxMenu*>(menu);
    wxMenuItem* item = wx_menu->FindItemByPosition(pos);
    return reinterpret_cast<wxd_MenuItem_t*>(item);
}

WXD_EXPORTED int
wxd_Menu_GetHelpString(const wxd_Menu_t* menu, wxd_Id id, char* buffer, size_t buffer_size)
{
    if (!menu)
        return -1;
    const wxMenu* wx_menu = reinterpret_cast<const wxMenu*>(menu);
    wxString help = wx_menu->GetHelpString(id);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(help, buffer, buffer_size);
}

WXD_EXPORTED void
wxd_Menu_SetHelpString(wxd_Menu_t* menu, wxd_Id id, const char* helpString)
{
    if (!menu)
        return;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wx_menu->SetHelpString(id, wxString::FromUTF8(helpString ? helpString : ""));
}

WXD_EXPORTED void
wxd_Menu_UpdateUI(wxd_Menu_t* menu, wxd_EvtHandler_t* source)
{
    if (!menu)
        return;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wxEvtHandler* wx_source = source ? reinterpret_cast<wxEvtHandler*>(source) : nullptr;
    wx_menu->UpdateUI(wx_source);
}

WXD_EXPORTED void
wxd_Menu_Break(wxd_Menu_t* menu)
{
    if (!menu)
        return;
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    wx_menu->Break();
}

// --- MenuBar Extended Functions ---

WXD_EXPORTED wxd_Menu_t*
wxd_MenuBar_GetMenu(const wxd_MenuBar_t* menubar, size_t index)
{
    if (!menubar)
        return nullptr;
    const wxMenuBar* wx_menubar = reinterpret_cast<const wxMenuBar*>(menubar);
    wxMenu* menu = wx_menubar->GetMenu(index);
    return reinterpret_cast<wxd_Menu_t*>(menu);
}

WXD_EXPORTED size_t
wxd_MenuBar_GetMenuCount(const wxd_MenuBar_t* menubar)
{
    if (!menubar)
        return 0;
    const wxMenuBar* wx_menubar = reinterpret_cast<const wxMenuBar*>(menubar);
    return wx_menubar->GetMenuCount();
}

WXD_EXPORTED int
wxd_MenuBar_FindMenu(const wxd_MenuBar_t* menubar, const char* title)
{
    if (!menubar)
        return wxNOT_FOUND;
    const wxMenuBar* wx_menubar = reinterpret_cast<const wxMenuBar*>(menubar);
    return wx_menubar->FindMenu(wxString::FromUTF8(title ? title : ""));
}

WXD_EXPORTED void
wxd_MenuBar_EnableTop(wxd_MenuBar_t* menubar, size_t pos, bool enable)
{
    if (!menubar)
        return;
    wxMenuBar* wx_menubar = reinterpret_cast<wxMenuBar*>(menubar);
    wx_menubar->EnableTop(pos, enable);
}

WXD_EXPORTED int
wxd_MenuBar_GetMenuLabel(const wxd_MenuBar_t* menubar, size_t pos, char* buffer, size_t buffer_size)
{
    if (!menubar)
        return -1;
    const wxMenuBar* wx_menubar = reinterpret_cast<const wxMenuBar*>(menubar);
    wxString label = wx_menubar->GetMenuLabel(pos);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(label, buffer, buffer_size);
}

WXD_EXPORTED void
wxd_MenuBar_SetMenuLabel(wxd_MenuBar_t* menubar, size_t pos, const char* label)
{
    if (!menubar)
        return;
    wxMenuBar* wx_menubar = reinterpret_cast<wxMenuBar*>(menubar);
    wx_menubar->SetMenuLabel(pos, wxString::FromUTF8(label ? label : ""));
}

WXD_EXPORTED wxd_Menu_t*
wxd_MenuBar_Replace(wxd_MenuBar_t* menubar, size_t pos, wxd_Menu_t* menu, const char* title)
{
    if (!menubar || !menu)
        return nullptr;
    wxMenuBar* wx_menubar = reinterpret_cast<wxMenuBar*>(menubar);
    wxMenu* wx_menu = reinterpret_cast<wxMenu*>(menu);
    // wxMenuBar takes ownership of the new menu
    wxMenu* old = wx_menubar->Replace(pos, wx_menu, wxString::FromUTF8(title ? title : ""));
    return reinterpret_cast<wxd_Menu_t*>(old);
}

// --- MenuItem Functions ---
WXD_EXPORTED void
wxd_MenuItem_Destroy(wxd_MenuItem_t* item)
{
    if (!item)
        return;
    wxMenuItem* wx_item = reinterpret_cast<wxMenuItem*>(item);
    delete wx_item;
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

WXD_EXPORTED int
wxd_MenuItem_GetLabel(const wxd_MenuItem_t* item, char* buffer, size_t buffer_size)
{
    if (!item)
        return -1;
    const wxMenuItem* wx_item = reinterpret_cast<const wxMenuItem*>(item);
    wxString label = wx_item->GetItemLabel();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(label, buffer, buffer_size);
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

// Get the owning window (typically the wxFrame) for a menu item via its menubar
WXD_EXPORTED const wxd_Window_t*
wxd_MenuItem_GetOwningWindow(const wxd_MenuItem_t* item)
{
    if (!item)
        return nullptr;
    const wxMenuItem* wx_item = reinterpret_cast<const wxMenuItem*>(item);
    wxMenu* menu = wx_item->GetMenu();
    if (!menu)
        return nullptr;
    wxMenuBar* mbar = menu->GetMenuBar();
    if (!mbar)
        return nullptr;
    wxFrame* frame = mbar->GetFrame();
    return reinterpret_cast<const wxd_Window_t*>(frame);
}

// Get the integer id of the menu item
WXD_EXPORTED int
wxd_MenuItem_GetId(const wxd_MenuItem_t* item)
{
    if (!item)
        return -1;
    const wxMenuItem* wx_item = reinterpret_cast<const wxMenuItem*>(item);
    return wx_item->GetId();
}

WXD_EXPORTED wxd_Menu_t*
wxd_MenuItem_GetSubMenu(const wxd_MenuItem_t* item)
{
    if (!item)
        return nullptr;
    const wxMenuItem* wx_item = reinterpret_cast<const wxMenuItem*>(item);
    return reinterpret_cast<wxd_Menu_t*>(wx_item->GetSubMenu());
}

WXD_EXPORTED void
wxd_MenuItem_SetBitmap(wxd_MenuItem_t* item, const wxd_Bitmap_t* bitmap)
{
    if (!item)
        return;
    wxMenuItem* wx_item = reinterpret_cast<wxMenuItem*>(item);
    const wxBitmap* wx_bitmap = reinterpret_cast<const wxBitmap*>(bitmap);
    wx_item->SetBitmap(wx_bitmap ? *wx_bitmap : wxNullBitmap);
}

WXD_EXPORTED wxd_Bitmap_t*
wxd_MenuItem_GetBitmap(const wxd_MenuItem_t* item)
{
    if (!item)
        return nullptr;
    const wxMenuItem* wx_item = reinterpret_cast<const wxMenuItem*>(item);
    const wxBitmap& bmp = wx_item->GetBitmap();
    if (!bmp.IsOk())
        return nullptr;
    return reinterpret_cast<wxd_Bitmap_t*>(new wxBitmap(bmp));
}

} // extern "C"