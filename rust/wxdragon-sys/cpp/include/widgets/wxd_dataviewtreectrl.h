#ifndef WXD_DATAVIEWTREECTRL_H
#define WXD_DATAVIEWTREECTRL_H

#include "../wxd_types.h" // For wxd_Window_t, wxd_DataViewItem_t, wxd_ImageList_t, WXD_EXPORTED

#ifdef __cplusplus
extern "C" {
#endif

// Constructor
WXD_EXPORTED wxd_Window_t*
wxd_DataViewTreeCtrl_new(wxd_Window_t* parent, int id, wxd_Point pos, wxd_Size size, int64_t style,
                         wxd_Window_t* validator, // Typically NULL for DataViewCtrl
                         const char* name);

// Item Management
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewTreeCtrl_AppendItem(wxd_Window_t* self, const wxd_DataViewItem_t* parent,
                                const char* text, int icon);
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewTreeCtrl_AppendContainer(wxd_Window_t* self, const wxd_DataViewItem_t* parent,
                                     const char* text, int icon, int expanded_icon);
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewTreeCtrl_PrependItem(wxd_Window_t* self, const wxd_DataViewItem_t* parent,
                                 const char* text, int icon);
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewTreeCtrl_PrependContainer(wxd_Window_t* self, const wxd_DataViewItem_t* parent,
                                      const char* text, int icon, int expanded_icon);
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewTreeCtrl_InsertItem(wxd_Window_t* self, const wxd_DataViewItem_t* parent,
                                const wxd_DataViewItem_t* previous, const char* text, int icon);
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewTreeCtrl_InsertContainer(wxd_Window_t* self, const wxd_DataViewItem_t* parent,
                                     const wxd_DataViewItem_t* previous, const char* text, int icon,
                                     int expanded_icon);
WXD_EXPORTED void
wxd_DataViewTreeCtrl_DeleteItem(wxd_Window_t* self, const wxd_DataViewItem_t* item);
WXD_EXPORTED void
wxd_DataViewTreeCtrl_DeleteChildren(wxd_Window_t* self, const wxd_DataViewItem_t* item);

/**
 * Getters and Setters for Item Properties
 * Returns the length of the text (excluding null terminator) for GetItemText.
 * If the item is invalid, returns -1.
 */
WXD_EXPORTED int
wxd_DataViewTreeCtrl_GetItemText(const wxd_Window_t* self, const wxd_DataViewItem_t* item,
                                 char* out, size_t out_len);

WXD_EXPORTED void
wxd_DataViewTreeCtrl_SetItemText(wxd_Window_t* self, const wxd_DataViewItem_t* item,
                                 const char* text);

// Note: GetItemIcon/GetItemExpandedIcon (returning int index) are omitted due to wxWidgets API not directly providing index retrieval.
// Users set icons by index; if they need to get an icon, they'd typically get the wxBitmapBundle/wxIcon.
// SetItemIcon/SetItemExpandedIcon take an icon index and internally use the ImageList.
WXD_EXPORTED void
wxd_DataViewTreeCtrl_SetItemIcon(wxd_Window_t* self, const wxd_DataViewItem_t* item, int icon_idx);
WXD_EXPORTED void
wxd_DataViewTreeCtrl_SetItemExpandedIcon(wxd_Window_t* self, const wxd_DataViewItem_t* item,
                                         int icon_idx);

// Item Relationships
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewTreeCtrl_GetItemParent(wxd_Window_t* self, const wxd_DataViewItem_t* item);
WXD_EXPORTED unsigned int
wxd_DataViewTreeCtrl_GetChildCount(wxd_Window_t* self, const wxd_DataViewItem_t* parent);
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewTreeCtrl_GetNthChild(wxd_Window_t* self, const wxd_DataViewItem_t* parent,
                                 unsigned int pos);
WXD_EXPORTED bool
wxd_DataViewTreeCtrl_IsContainer(wxd_Window_t* self, const wxd_DataViewItem_t* item);

// Tree State
WXD_EXPORTED void
wxd_DataViewTreeCtrl_Expand(wxd_Window_t* self, const wxd_DataViewItem_t* item);
WXD_EXPORTED void
wxd_DataViewTreeCtrl_Collapse(wxd_Window_t* self, const wxd_DataViewItem_t* item);
WXD_EXPORTED bool
wxd_DataViewTreeCtrl_IsExpanded(wxd_Window_t* self, const wxd_DataViewItem_t* item);
WXD_EXPORTED void
wxd_DataViewTreeCtrl_DeleteAllItems(wxd_Window_t* self);

// Image List
WXD_EXPORTED void
wxd_DataViewTreeCtrl_SetImageList(wxd_Window_t* self, wxd_ImageList_t* image_list);
WXD_EXPORTED wxd_ImageList_t*
wxd_DataViewTreeCtrl_GetImageList(wxd_Window_t* self);

#ifdef __cplusplus
}
#endif

#endif // WXD_DATAVIEWTREECTRL_H