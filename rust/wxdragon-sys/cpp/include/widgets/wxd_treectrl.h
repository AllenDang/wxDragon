#ifndef WXD_TREECTRL_H
#define WXD_TREECTRL_H

#include <stdint.h>
#include "../wxd_types.h"

// --- TreeItemData functions ---
// Create and manage TreeItemData objects
WXD_EXPORTED wxd_TreeItemData_t*
wxd_TreeItemData_Create(void* client_data);
WXD_EXPORTED void
wxd_TreeItemData_Free(wxd_TreeItemData_t* data);
WXD_EXPORTED void*
wxd_TreeItemData_GetClientData(wxd_TreeItemData_t* data);
WXD_EXPORTED void
wxd_TreeItemData_SetClientData(wxd_TreeItemData_t* data, void* client_data);

// --- TreeCtrl Functions ---
WXD_EXPORTED wxd_TreeCtrl_t*
wxd_TreeCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                    wxd_Style_t style);

WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_AddRoot(wxd_TreeCtrl_t* self, const char* text, int image, int selImage, void* data);

WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_AppendItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* parent_id, const char* text,
                        int image, int selImage, void* data);
WXD_EXPORTED void
wxd_TreeCtrl_Delete(wxd_TreeCtrl_t* self, const wxd_TreeItemId_t* item_id);
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetSelection(wxd_TreeCtrl_t* self);

WXD_EXPORTED void
wxd_TreeCtrl_SelectItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id);

WXD_EXPORTED void
wxd_TreeCtrl_Expand(wxd_TreeCtrl_t* self, const wxd_TreeItemId_t* item_id);

WXD_EXPORTED int64_t
wxd_TreeCtrl_GetItemData(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id);

WXD_EXPORTED bool
wxd_TreeCtrl_SetItemData(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id, int64_t data);
WXD_EXPORTED void
wxd_TreeItemId_Free(wxd_TreeItemId_t* item_id);
WXD_EXPORTED bool
wxd_TreeItemId_IsOk(wxd_TreeItemId_t* item_id);
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeItemId_Clone(const wxd_TreeItemId_t* item_id);

// New tree traversal functions
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetRootItem(wxd_TreeCtrl_t* self);
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetFirstChild(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id, void** cookie);
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetNextChild(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id, void** cookie);
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetNextSibling(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id);
WXD_EXPORTED size_t
wxd_TreeCtrl_GetChildrenCount(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id, bool recursively);

// --- TreeCtrl ImageList and Item Image functions ---

// Enum for specifying which icon to set/get for a tree item
typedef enum : int32_t {
    WXD_TreeItemIcon_Normal = 0,      // wxTreeItemIcon_Normal
    WXD_TreeItemIcon_Selected,        // wxTreeItemIcon_Selected
    WXD_TreeItemIcon_Expanded,        // wxTreeItemIcon_Expanded
    WXD_TreeItemIcon_SelectedExpanded // wxTreeItemIcon_SelectedExpanded
} wxd_TreeItemIconType_t;

WXD_EXPORTED void
wxd_TreeCtrl_SetImageList(wxd_TreeCtrl_t* self, wxd_ImageList_t* imageList);
WXD_EXPORTED wxd_ImageList_t*
wxd_TreeCtrl_GetImageList(wxd_TreeCtrl_t* self);

WXD_EXPORTED void
wxd_TreeCtrl_SetItemImage(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, int image,
                          wxd_TreeItemIconType_t which);
WXD_EXPORTED int
wxd_TreeCtrl_GetItemImage(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId,
                          wxd_TreeItemIconType_t which);

// --- Additional TreeCtrl functions ---

// Get the text label of an item
// Returns the number of bytes written (excluding null terminator), or the required buffer size if buffer is too small
WXD_EXPORTED int
wxd_TreeCtrl_GetItemText(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, char* buffer, size_t buffer_len);

// Set the text label of an item
WXD_EXPORTED void
wxd_TreeCtrl_SetItemText(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, const char* text);

// Scrolls and/or expands items to ensure that the given item is visible
WXD_EXPORTED void
wxd_TreeCtrl_EnsureVisible(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Sets the currently focused item (the item that has keyboard focus)
WXD_EXPORTED void
wxd_TreeCtrl_SetFocusedItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Gets the currently focused item
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetFocusedItem(wxd_TreeCtrl_t* self);

// --- Expand/Collapse functions ---

// Expands all items in the tree
WXD_EXPORTED void
wxd_TreeCtrl_ExpandAll(wxd_TreeCtrl_t* self);

// Collapses the given item
WXD_EXPORTED void
wxd_TreeCtrl_Collapse(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Collapses all items in the tree
WXD_EXPORTED void
wxd_TreeCtrl_CollapseAll(wxd_TreeCtrl_t* self);

// Collapses the given item and all its children
WXD_EXPORTED void
wxd_TreeCtrl_CollapseAllChildren(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Collapses the item and removes all children
WXD_EXPORTED void
wxd_TreeCtrl_CollapseAndReset(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Toggles the expand/collapse state of the given item
WXD_EXPORTED void
wxd_TreeCtrl_Toggle(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Checks if the given item is expanded
WXD_EXPORTED bool
wxd_TreeCtrl_IsExpanded(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// --- Selection functions ---

// Checks if the given item is selected
WXD_EXPORTED bool
wxd_TreeCtrl_IsSelected(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Unselects all items
WXD_EXPORTED void
wxd_TreeCtrl_UnselectAll(wxd_TreeCtrl_t* self);

// Unselects the given item
WXD_EXPORTED void
wxd_TreeCtrl_UnselectItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Selects all items (only for trees with TR_MULTIPLE style)
WXD_EXPORTED void
wxd_TreeCtrl_SelectAll(wxd_TreeCtrl_t* self);

// Gets all selected items (for multi-selection trees)
// Returns the number of selected items, fills the array with item pointers
// If items is NULL, just returns the count
WXD_EXPORTED size_t
wxd_TreeCtrl_GetSelections(wxd_TreeCtrl_t* self, wxd_TreeItemId_t** items, size_t max_items);

// --- Navigation functions ---

// Gets the parent of the given item
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetItemParent(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Gets the previous sibling of the given item
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetPrevSibling(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Gets the last child of the given item
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetLastChild(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// --- Item state functions ---

// Checks if the given item is visible
WXD_EXPORTED bool
wxd_TreeCtrl_IsVisible(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Checks if the given item has children
WXD_EXPORTED bool
wxd_TreeCtrl_ItemHasChildren(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Checks if the given item is bold
WXD_EXPORTED bool
wxd_TreeCtrl_IsBold(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Sets the item to bold or normal
WXD_EXPORTED void
wxd_TreeCtrl_SetItemBold(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, bool bold);

// --- Item styling functions ---

// Sets the text colour of the given item
WXD_EXPORTED void
wxd_TreeCtrl_SetItemTextColour(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, wxd_Colour_t colour);

// Gets the text colour of the given item
WXD_EXPORTED wxd_Colour_t
wxd_TreeCtrl_GetItemTextColour(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Sets the background colour of the given item
WXD_EXPORTED void
wxd_TreeCtrl_SetItemBackgroundColour(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, wxd_Colour_t colour);

// Gets the background colour of the given item
WXD_EXPORTED wxd_Colour_t
wxd_TreeCtrl_GetItemBackgroundColour(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Sets the font of the given item
WXD_EXPORTED void
wxd_TreeCtrl_SetItemFont(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, wxd_Font_t* font);

// Gets the font of the given item
WXD_EXPORTED wxd_Font_t*
wxd_TreeCtrl_GetItemFont(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// --- Item management functions ---

// Inserts an item after the given previous item
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_InsertItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* parent, wxd_TreeItemId_t* idPrevious,
                        const char* text, int image, int selImage, void* data);

// Inserts an item at the given position (0-based index)
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_InsertItemBefore(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* parent, size_t pos,
                              const char* text, int image, int selImage, void* data);

// Prepends an item as the first child
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_PrependItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* parent, const char* text,
                         int image, int selImage, void* data);

// Deletes all items in the tree
WXD_EXPORTED void
wxd_TreeCtrl_DeleteAllItems(wxd_TreeCtrl_t* self);

// Deletes all children of the given item
WXD_EXPORTED void
wxd_TreeCtrl_DeleteChildren(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Gets the total number of items in the tree
WXD_EXPORTED size_t
wxd_TreeCtrl_GetCount(wxd_TreeCtrl_t* self);

// --- Label editing functions ---

// Starts editing the label of the given item
WXD_EXPORTED wxd_TextCtrl_t*
wxd_TreeCtrl_EditLabel(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Ends label editing (cancel = true to discard changes)
WXD_EXPORTED void
wxd_TreeCtrl_EndEditLabel(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, bool discardChanges);

// Gets the edit control used for label editing (if currently editing)
WXD_EXPORTED wxd_TextCtrl_t*
wxd_TreeCtrl_GetEditControl(wxd_TreeCtrl_t* self);

// --- Other functions ---

// Scrolls to the given item (without expanding)
WXD_EXPORTED void
wxd_TreeCtrl_ScrollTo(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Sorts the children of the given item alphabetically
WXD_EXPORTED void
wxd_TreeCtrl_SortChildren(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Hit test - returns the item at the given point
// flags receives information about the hit test result
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_HitTest(wxd_TreeCtrl_t* self, wxd_Point point, int* flags);

// Gets the bounding rectangle of an item
// If textOnly is true, only gets the rectangle of the label
WXD_EXPORTED bool
wxd_TreeCtrl_GetBoundingRect(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, wxd_Rect* rect, bool textOnly);

// Sets the state image list
WXD_EXPORTED void
wxd_TreeCtrl_SetStateImageList(wxd_TreeCtrl_t* self, wxd_ImageList_t* imageList);

// Gets the state image list
WXD_EXPORTED wxd_ImageList_t*
wxd_TreeCtrl_GetStateImageList(wxd_TreeCtrl_t* self);

// Sets the state image for an item
WXD_EXPORTED void
wxd_TreeCtrl_SetItemState(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, int state);

// Gets the state image for an item
WXD_EXPORTED int
wxd_TreeCtrl_GetItemState(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId);

// Sets whether the item has a button (+/-) to expand/collapse
WXD_EXPORTED void
wxd_TreeCtrl_SetItemHasChildren(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, bool has);

// Enables or disables an item (grays it out when disabled)
WXD_EXPORTED void
wxd_TreeCtrl_EnableItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, bool enable);

// Tree hit test flags
typedef enum : int32_t {
    WXD_TREE_HITTEST_ABOVE = 0x0001,
    WXD_TREE_HITTEST_BELOW = 0x0002,
    WXD_TREE_HITTEST_NOWHERE = 0x0004,
    WXD_TREE_HITTEST_ONITEMBUTTON = 0x0008,
    WXD_TREE_HITTEST_ONITEMICON = 0x0010,
    WXD_TREE_HITTEST_ONITEMINDENT = 0x0020,
    WXD_TREE_HITTEST_ONITEMLABEL = 0x0040,
    WXD_TREE_HITTEST_ONITEMRIGHT = 0x0080,
    WXD_TREE_HITTEST_ONITEMSTATEICON = 0x0100,
    WXD_TREE_HITTEST_TOLEFT = 0x0200,
    WXD_TREE_HITTEST_TORIGHT = 0x0400,
    WXD_TREE_HITTEST_ONITEMUPPERPART = 0x0800,
    WXD_TREE_HITTEST_ONITEMLOWERPART = 0x1000,
    WXD_TREE_HITTEST_ONITEM = WXD_TREE_HITTEST_ONITEMICON | WXD_TREE_HITTEST_ONITEMLABEL
} wxd_TreeHitTestFlags;

#endif // WXD_TREECTRL_H