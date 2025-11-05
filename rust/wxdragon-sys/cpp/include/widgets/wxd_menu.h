#ifndef WXD_MENU_H
#define WXD_MENU_H

#include "../wxd_types.h"

// --- MenuBar, Menu, MenuItem Functions ---
WXD_EXPORTED wxd_MenuBar_t*
wxd_MenuBar_Create(wxd_Style_t style);

WXD_EXPORTED void
wxd_MenuBar_Append(wxd_MenuBar_t* menubar, wxd_Menu_t* menu, const char* title);

/**
 * @brief Enable or disable a menu item by id across the entire menubar.
 * This is equivalent to wxMenuBar::Enable(id, enable).
 */
WXD_EXPORTED bool
wxd_MenuBar_EnableItem(wxd_MenuBar_t* menubar, wxd_Id id, bool enable);

/**
 * @brief Query whether a menu item is enabled via the menubar.
 * This is equivalent to wxMenuBar::IsEnabled(id).
 */
WXD_EXPORTED bool
wxd_MenuBar_IsItemEnabled(const wxd_MenuBar_t* menubar, wxd_Id id);

WXD_EXPORTED wxd_Menu_t*
wxd_Menu_Create(const char* title, wxd_Style_t style);

/**
 * @brief Get the number of items in the wxMenu.
 */
WXD_EXPORTED size_t
wxd_Menu_GetMenuItemCount(const wxd_Menu_t* menu);

/**
 * @brief Get the title of the wxMenu in UTF-8.
 * If buffer is non-null and buffer_size > 0, copies up to buffer_size - 1 bytes and NUL-terminates the buffer.
 * If buffer is null or buffer_size == 0, nothing is written.
 * @param menu Pointer to wxMenu.
 * @param buffer Buffer to receive the UTF-8 title.
 * @param buffer_size Size of the buffer.
 * @return Number of bytes required to store the UTF-8 title, excluding the terminating NUL, or -1 on error.
 */
WXD_EXPORTED int
wxd_Menu_GetTitle(const wxd_Menu_t* menu, char* buffer, size_t buffer_size);

/**
 * @brief Set the title of the wxMenu.
 */
WXD_EXPORTED void
wxd_Menu_SetTitle(wxd_Menu_t* menu, const char* title);

/**
 * @brief Destroy a wxMenu.
 * WARNING: Only destroy standalone menus not owned by a wxMenuBar.
 * If a wxMenu has been appended to a wxMenuBar, the menubar owns it and
 * will delete it; destroying here would cause double free.
 */
WXD_EXPORTED void
wxd_Menu_Destroy(wxd_Menu_t* menu);

WXD_EXPORTED wxd_MenuItem_t*
wxd_Menu_Append(wxd_Menu_t* menu, wxd_Id id, const char* item, const char* helpString, int kind);

/**
 * @brief Append a submenu to a wxMenu.
 * wxMenu takes ownership of the submenu.
 */
WXD_EXPORTED const wxd_MenuItem_t*
wxd_Menu_AppendSubMenu(wxd_Menu_t* menu, wxd_Menu_t* submenu, const char* title,
                       const char* helpString);

/**
 * @brief Enable or disable a menu item by its id.
 */
WXD_EXPORTED bool
wxd_Menu_ItemEnable(wxd_Menu_t* menu, wxd_Id id, bool enable);

WXD_EXPORTED bool
wxd_Menu_IsItemEnabled(const wxd_Menu_t* menu, wxd_Id id);

WXD_EXPORTED void
wxd_Menu_AppendSeparator(wxd_Menu_t* menu);

WXD_EXPORTED void
wxd_MenuItem_Destroy(wxd_MenuItem_t* item);

// --- MenuItem State Functions ---
WXD_EXPORTED void
wxd_MenuItem_SetLabel(wxd_MenuItem_t* item, const char* label);

/**
 * Get the label of the menu item. The returned value is the length of the label (excluding null terminator),
 * if any error, -1 returned.
 * If buffer is non-null and buffer_size > 0, copies up to buffer_size - 1 bytes and NUL-terminates the buffer.
 * If buffer is null or buffer_size == 0, nothing is written.
 * @param item Pointer to wxMenuItem.
 * @param buffer Buffer to receive the UTF-8 label.
 * @param buffer_size Size of the buffer.
 * @return Number of bytes required to store the UTF-8 label, excluding the terminating NUL.
 */
WXD_EXPORTED int
wxd_MenuItem_GetLabel(const wxd_MenuItem_t* item, char* buffer, size_t buffer_size);

WXD_EXPORTED void
wxd_MenuItem_Enable(wxd_MenuItem_t* item, bool enable);

WXD_EXPORTED bool
wxd_MenuItem_IsEnabled(wxd_MenuItem_t* item);

WXD_EXPORTED void
wxd_MenuItem_Check(wxd_MenuItem_t* item, bool check);

WXD_EXPORTED bool
wxd_MenuItem_IsChecked(wxd_MenuItem_t* item);

/// Get the owning window (typically wxFrame) of a menu item via its menubar.
WXD_EXPORTED const wxd_Window_t*
wxd_MenuItem_GetOwningWindow(const wxd_MenuItem_t* item);

/// Get the integer id of the menu item.
WXD_EXPORTED int
wxd_MenuItem_GetId(const wxd_MenuItem_t* item);

#endif // WXD_MENU_H
