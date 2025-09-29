#ifndef WXD_TREELISTCTRL_H
#define WXD_TREELISTCTRL_H

#include "../wxd_types.h"

// --- TreeListCtrl Functions ---

// Creation and basic operations
WXD_EXPORTED wxd_TreeListCtrl_t* wxd_TreeListCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);

// Column management
WXD_EXPORTED int wxd_TreeListCtrl_AppendColumn(wxd_TreeListCtrl_t* self, const char* text, int width, int align);
WXD_EXPORTED int wxd_TreeListCtrl_GetColumnCount(wxd_TreeListCtrl_t* self);
WXD_EXPORTED void wxd_TreeListCtrl_SetColumnWidth(wxd_TreeListCtrl_t* self, int col, int width);
WXD_EXPORTED int wxd_TreeListCtrl_GetColumnWidth(wxd_TreeListCtrl_t* self, int col);
WXD_EXPORTED bool wxd_TreeListCtrl_DeleteColumn(wxd_TreeListCtrl_t* self, unsigned col);
WXD_EXPORTED void wxd_TreeListCtrl_ClearColumns(wxd_TreeListCtrl_t* self);
WXD_EXPORTED int wxd_TreeListCtrl_WidthFor(wxd_TreeListCtrl_t* self, const char* text);

// Item management
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetRootItem(wxd_TreeListCtrl_t* self);
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_AppendItem(wxd_TreeListCtrl_t* self, wxd_Long_t parent, const char* text);
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_InsertItem(wxd_TreeListCtrl_t* self, wxd_Long_t parent, wxd_Long_t previous, const char* text);
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_PrependItem(wxd_TreeListCtrl_t* self, wxd_Long_t parent, const char* text);
WXD_EXPORTED void wxd_TreeListCtrl_DeleteItem(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED void wxd_TreeListCtrl_DeleteAllItems(wxd_TreeListCtrl_t* self);
WXD_EXPORTED void wxd_TreeListCtrl_SetItemText(wxd_TreeListCtrl_t* self, wxd_Long_t item, int col, const char* text);
WXD_EXPORTED int wxd_TreeListCtrl_GetItemText(wxd_TreeListCtrl_t* self, wxd_Long_t item, int col, char* buffer, int buffer_len);
WXD_EXPORTED void wxd_TreeListCtrl_SetItemImage(wxd_TreeListCtrl_t* self, wxd_Long_t item, int closed, int opened);

// Tree operations
WXD_EXPORTED void wxd_TreeListCtrl_Expand(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED void wxd_TreeListCtrl_Collapse(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED bool wxd_TreeListCtrl_IsExpanded(wxd_TreeListCtrl_t* self, wxd_Long_t item);

// Tree navigation
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetItemParent(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetFirstChild(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetNextSibling(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetNextItem(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetFirstItem(wxd_TreeListCtrl_t* self);

// Selection operations
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetSelection(wxd_TreeListCtrl_t* self);
WXD_EXPORTED void wxd_TreeListCtrl_SelectItem(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED void wxd_TreeListCtrl_UnselectAll(wxd_TreeListCtrl_t* self);
WXD_EXPORTED unsigned wxd_TreeListCtrl_GetSelections(wxd_TreeListCtrl_t* self, wxd_Long_t* selections, unsigned max_count);
WXD_EXPORTED void wxd_TreeListCtrl_Select(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED void wxd_TreeListCtrl_Unselect(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED bool wxd_TreeListCtrl_IsSelected(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED void wxd_TreeListCtrl_SelectAll(wxd_TreeListCtrl_t* self);

// Visibility
WXD_EXPORTED void wxd_TreeListCtrl_EnsureVisible(wxd_TreeListCtrl_t* self, wxd_Long_t item);

// Checkbox operations
WXD_EXPORTED void wxd_TreeListCtrl_CheckItem(wxd_TreeListCtrl_t* self, wxd_Long_t item, int state);
WXD_EXPORTED int wxd_TreeListCtrl_GetCheckedState(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED void wxd_TreeListCtrl_CheckItemRecursively(wxd_TreeListCtrl_t* self, wxd_Long_t item, int state);
WXD_EXPORTED void wxd_TreeListCtrl_UpdateItemParentState(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED void wxd_TreeListCtrl_UncheckItem(wxd_TreeListCtrl_t* self, wxd_Long_t item);
WXD_EXPORTED bool wxd_TreeListCtrl_AreAllChildrenInState(wxd_TreeListCtrl_t* self, wxd_Long_t item, int state);

// Sorting
WXD_EXPORTED void wxd_TreeListCtrl_SetSortColumn(wxd_TreeListCtrl_t* self, unsigned col, bool ascending);
WXD_EXPORTED bool wxd_TreeListCtrl_GetSortColumn(wxd_TreeListCtrl_t* self, unsigned* col, bool* ascending);

#endif // WXD_TREELISTCTRL_H