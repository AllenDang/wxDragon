#ifndef WXD_LISTCTRL_H
#define WXD_LISTCTRL_H

#include "../wxd_types.h"

// --- ListCtrl Functions ---
WXD_EXPORTED wxd_ListCtrl_t*
wxd_ListCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                    wxd_Style_t style);
WXD_EXPORTED int32_t
wxd_ListCtrl_InsertColumn(wxd_ListCtrl_t* self, int64_t col, const char* heading, int format,
                          int width);
WXD_EXPORTED bool
wxd_ListCtrl_SetColumnWidth(wxd_ListCtrl_t* self, int64_t col, int width);
WXD_EXPORTED int
wxd_ListCtrl_GetColumnWidth(wxd_ListCtrl_t* self, int64_t col);
WXD_EXPORTED int
wxd_ListCtrl_GetColumnCount(wxd_ListCtrl_t* self);
WXD_EXPORTED int32_t
wxd_ListCtrl_InsertItem_Simple(wxd_ListCtrl_t* self, int64_t index, const char* label);
WXD_EXPORTED void
wxd_ListCtrl_SetItemText(wxd_ListCtrl_t* self, int64_t index, const char* text);
WXD_EXPORTED bool
wxd_ListCtrl_SetItem(wxd_ListCtrl_t* self, int64_t item, int col, const char* text, int image,
                     int format, int64_t state, int64_t stateMask, int64_t data, int64_t mask);
WXD_EXPORTED int
wxd_ListCtrl_GetItemText(wxd_ListCtrl_t* self, int64_t index, int col, char* buffer,
                         int buffer_len);
WXD_EXPORTED int
wxd_ListCtrl_GetItemCount(wxd_ListCtrl_t* self);
WXD_EXPORTED bool
wxd_ListCtrl_SetItemState(wxd_ListCtrl_t* self, int64_t item, int64_t state, int64_t stateMask);
WXD_EXPORTED int32_t
wxd_ListCtrl_GetItemState(wxd_ListCtrl_t* self, int64_t item, int64_t stateMask);
WXD_EXPORTED int32_t
wxd_ListCtrl_GetNextItem(wxd_ListCtrl_t* self, int64_t item, int geometry, int state);
WXD_EXPORTED bool
wxd_ListCtrl_DeleteItem(wxd_ListCtrl_t* self, int64_t item);
WXD_EXPORTED bool
wxd_ListCtrl_DeleteAllItems(wxd_ListCtrl_t* self);
WXD_EXPORTED bool
wxd_ListCtrl_ClearAll(wxd_ListCtrl_t* self);
WXD_EXPORTED int
wxd_ListCtrl_GetSelectedItemCount(wxd_ListCtrl_t* self);
WXD_EXPORTED bool
wxd_ListCtrl_EnsureVisible(wxd_ListCtrl_t* self, int64_t item);
WXD_EXPORTED int32_t
wxd_ListCtrl_HitTest(wxd_ListCtrl_t* self, wxd_Point point, int* flags_ptr, int64_t* subitem_ptr);
WXD_EXPORTED wxd_TextCtrl_t*
wxd_ListCtrl_EditLabel(wxd_ListCtrl_t* self, int64_t item);

// --- Advanced ListCtrl Capabilities ---
// Item Data Functions
WXD_EXPORTED bool
wxd_ListCtrl_SetItemData(wxd_ListCtrl_t* self, int64_t item, int64_t data);
WXD_EXPORTED bool
wxd_ListCtrl_SetItemPtrData(wxd_ListCtrl_t* self, int64_t item, void* data);
WXD_EXPORTED int64_t
wxd_ListCtrl_GetItemData(wxd_ListCtrl_t* self, int64_t item);
WXD_EXPORTED void*
wxd_ListCtrl_GetItemPtrData(wxd_ListCtrl_t* self, int64_t item);

// Item Appearance
WXD_EXPORTED void
wxd_ListCtrl_SetItemBackgroundColour(wxd_ListCtrl_t* self, int64_t item, wxd_Colour_t colour);
WXD_EXPORTED void
wxd_ListCtrl_SetItemTextColour(wxd_ListCtrl_t* self, int64_t item, wxd_Colour_t colour);
WXD_EXPORTED wxd_Colour_t
wxd_ListCtrl_GetItemBackgroundColour(wxd_ListCtrl_t* self, int64_t item);
WXD_EXPORTED wxd_Colour_t
wxd_ListCtrl_GetItemTextColour(wxd_ListCtrl_t* self, int64_t item);

// Column Management
WXD_EXPORTED bool
wxd_ListCtrl_SetColumnsOrder(wxd_ListCtrl_t* self, int count, int* orders);
WXD_EXPORTED int*
wxd_ListCtrl_GetColumnsOrder(wxd_ListCtrl_t* self, int* count);
WXD_EXPORTED int
wxd_ListCtrl_GetColumnOrder(wxd_ListCtrl_t* self, int col);
WXD_EXPORTED int
wxd_ListCtrl_GetColumnIndexFromOrder(wxd_ListCtrl_t* self, int pos);

// Virtual List Support
WXD_EXPORTED void
wxd_ListCtrl_SetItemCount(wxd_ListCtrl_t* self, int64_t count);
WXD_EXPORTED void
wxd_ListCtrl_RefreshItem(wxd_ListCtrl_t* self, int64_t item);
WXD_EXPORTED void
wxd_ListCtrl_RefreshItems(wxd_ListCtrl_t* self, int64_t itemFrom, int64_t itemTo);

// Sorting
WXD_EXPORTED bool
wxd_ListCtrl_SortItems(wxd_ListCtrl_t* self, int (*cmpFunc)(void*, void*, void*), void* data);
WXD_EXPORTED void
wxd_ListCtrl_ShowSortIndicator(wxd_ListCtrl_t* self, int col, bool ascending);

// Image List Support
WXD_EXPORTED void
wxd_ListCtrl_SetImageList(wxd_ListCtrl_t* self, wxd_ImageList_t* imageList, int which);
WXD_EXPORTED void
wxd_ListCtrl_AssignImageList(wxd_ListCtrl_t* self, wxd_ImageList_t* imageList, int which);
WXD_EXPORTED wxd_ImageList_t*
wxd_ListCtrl_GetImageList(wxd_ListCtrl_t* self, int which);

// New function for inserting item with an image index
WXD_EXPORTED int32_t
wxd_ListCtrl_InsertItemWithImage(wxd_ListCtrl_t* self, int64_t index, const char* label,
                                 int imageIndex);

// Renamed function (SetItemImageOnly removed, SetItemImageIndex added)
WXD_EXPORTED bool
wxd_ListCtrl_SetItemImageIndex(wxd_ListCtrl_t* self, int64_t itemIndex, int imageIndex);

#endif // WXD_LISTCTRL_H