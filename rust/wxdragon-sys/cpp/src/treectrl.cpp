#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "../src/wxd_utils.h"
#include <wx/treectrl.h>
#include <wx/imaglist.h>

// Helper class that wraps a long value so it can be used with TreeCtrl's native SetItemData
// which expects a wxTreeItemData pointer
class LongValueTreeItemData : public wxTreeItemData {
public:
    LongValueTreeItemData(long value) : m_value(value)
    {
    }
    int64_t
    GetValue() const
    {
        return m_value;
    }

private:
    int64_t m_value;
};

extern "C" {

#define WXD_UNWRAP_TREE_CTRL(ptr) reinterpret_cast<wxTreeCtrl*>(ptr)
#define WXD_WRAP_TREE_CTRL(ptr)   reinterpret_cast<wxd_TreeCtrl_t*>(ptr)

#define WXD_UNWRAP_WINDOW(ptr)       reinterpret_cast<wxWindow*>(ptr)
#define WXD_UNWRAP_TREE_ITEM_ID(ptr) reinterpret_cast<wxTreeItemId*>(ptr)
#define WXD_WRAP_TREE_ITEM_ID(ptr)   reinterpret_cast<wxd_TreeItemId_t*>(ptr)

// --- TreeCtrl ---
WXD_EXPORTED wxd_TreeCtrl_t*
wxd_TreeCtrl_Create(wxd_Window_t* parent, int id, wxd_Point pos, wxd_Size size, wxd_Style_t style)
{
    wxWindow* p = WXD_UNWRAP_WINDOW(parent);
    if (!p)
        return nullptr;

    wxPoint wxpos(pos.x, pos.y);
    wxSize wxsize(size.width, size.height);

    wxTreeCtrl* ctrl = new wxTreeCtrl(p, id, wxpos, wxsize, style);
    return WXD_WRAP_TREE_CTRL(ctrl);
}

WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_AddRoot(wxd_TreeCtrl_t* self, const char* text, int image, int selImage, void* data)
{
    wxTreeCtrl* ctrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!ctrl)
        return nullptr;

    wxString wxText = wxString::FromUTF8(text ? text : "");
    wxTreeItemId* id = new wxTreeItemId(
        ctrl->AddRoot(wxText, image, selImage, reinterpret_cast<wxTreeItemData*>(data)));

    return WXD_WRAP_TREE_ITEM_ID(id);
}

WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_AppendItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* parent, const char* text, int image,
                        int selImage, void* data)
{
    wxTreeCtrl* ctrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!ctrl)
        return nullptr;

    wxTreeItemId* parentId = WXD_UNWRAP_TREE_ITEM_ID(parent);
    if (!parentId || !parentId->IsOk())
        return nullptr;

    wxString wxText = wxString::FromUTF8(text ? text : "");
    wxTreeItemId* id = new wxTreeItemId(ctrl->AppendItem(*parentId, wxText, image, selImage,
                                                         reinterpret_cast<wxTreeItemData*>(data)));

    return WXD_WRAP_TREE_ITEM_ID(id);
}

WXD_EXPORTED void
wxd_TreeCtrl_Delete(wxd_TreeCtrl_t* self, const wxd_TreeItemId_t* item_id)
{
    wxTreeCtrl* ctrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!ctrl)
        return;

    const wxTreeItemId* id = reinterpret_cast<const wxTreeItemId*>(item_id);
    if (!id || !id->IsOk())
        return;

    ctrl->Delete(*id);
}

WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetSelection(wxd_TreeCtrl_t* self)
{
    wxTreeCtrl* ctrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!ctrl)
        return nullptr;

    wxTreeItemId* id = new wxTreeItemId(ctrl->GetSelection());
    return WXD_WRAP_TREE_ITEM_ID(id);
}

WXD_EXPORTED void
wxd_TreeCtrl_SelectItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id)
{
    wxTreeCtrl* ctrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!ctrl)
        return;

    wxTreeItemId* id = WXD_UNWRAP_TREE_ITEM_ID(item_id);
    if (!id || !id->IsOk())
        return;

    ctrl->SelectItem(*id);
}

WXD_EXPORTED void
wxd_TreeCtrl_Expand(wxd_TreeCtrl_t* self, const wxd_TreeItemId_t* item_id)
{
    wxTreeCtrl* ctrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!ctrl)
        return;

    const wxTreeItemId* id = reinterpret_cast<const wxTreeItemId*>(item_id);
    if (!id || !id->IsOk())
        return;

    ctrl->Expand(*id);
}

// TreeItemId_Free
WXD_EXPORTED void
wxd_TreeItemId_Free(wxd_TreeItemId_t* item_id)
{
    wxTreeItemId* id = WXD_UNWRAP_TREE_ITEM_ID(item_id);
    delete id;
}

// TreeItemId_IsOk
WXD_EXPORTED bool
wxd_TreeItemId_IsOk(wxd_TreeItemId_t* item_id)
{
    wxTreeItemId* id = WXD_UNWRAP_TREE_ITEM_ID(item_id);
    return id && id->IsOk();
}

// TreeItemId_Clone
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeItemId_Clone(const wxd_TreeItemId_t* item_id)
{
    const wxTreeItemId* id = reinterpret_cast<const wxTreeItemId*>(item_id);
    if (!id || !id->IsOk())
        return nullptr;

    wxTreeItemId* clone = new wxTreeItemId(*id);
    return WXD_WRAP_TREE_ITEM_ID(clone);
}

// Set Item Data as a long value
WXD_EXPORTED bool
wxd_TreeCtrl_SetItemData(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id, int64_t data)
{
    wxTreeCtrl* tree = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* id = WXD_UNWRAP_TREE_ITEM_ID(item_id);
    if (!tree || !id || !id->IsOk())
        return false;

    // If data is 0, just clear the item data
    if (data == 0) {
        tree->SetItemData(*id, nullptr);
        return true;
    }

    // Create a new LongValueTreeItemData to wrap the long value
    LongValueTreeItemData* itemData = new LongValueTreeItemData(data);
    tree->SetItemData(*id, itemData);
    return true;
}

// Get Item Data as a long value
WXD_EXPORTED int64_t
wxd_TreeCtrl_GetItemData(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id)
{
    wxTreeCtrl* tree = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* id = WXD_UNWRAP_TREE_ITEM_ID(item_id);
    if (!tree || !id || !id->IsOk())
        return 0;

    wxTreeItemData* data = tree->GetItemData(*id);
    if (!data)
        return 0;

    // Try to cast to our LongValueTreeItemData type
    LongValueTreeItemData* longData = dynamic_cast<LongValueTreeItemData*>(data);
    if (longData)
        return longData->GetValue();

    // If it's not our wrapper class, return the pointer value as an integer
    // This is a fallback that shouldn't normally be needed
    return reinterpret_cast<int64_t>(data);
}

// New tree traversal functions

// Get the root item of the tree
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetRootItem(wxd_TreeCtrl_t* self)
{
    wxTreeCtrl* tree = WXD_UNWRAP_TREE_CTRL(self);
    if (!tree)
        return nullptr;

    wxTreeItemId rootId = tree->GetRootItem();
    if (!rootId.IsOk())
        return nullptr;

    wxTreeItemId* id = new wxTreeItemId(rootId);
    return WXD_WRAP_TREE_ITEM_ID(id);
}

// Get the first child of an item
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetFirstChild(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id, void** cookie)
{
    wxTreeCtrl* tree = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* id = WXD_UNWRAP_TREE_ITEM_ID(item_id);
    if (!tree || !id || !id->IsOk() || !cookie)
        return nullptr;

    // Create wxWidgets cookie
    wxTreeItemIdValue wxCookie;

    // Get the first child
    wxTreeItemId childId = tree->GetFirstChild(*id, wxCookie);
    if (!childId.IsOk())
        return nullptr;

    // Store the cookie for subsequent calls to GetNextChild
    *cookie = new wxTreeItemIdValue(wxCookie);

    // Return the child ID
    wxTreeItemId* childIdPtr = new wxTreeItemId(childId);
    return WXD_WRAP_TREE_ITEM_ID(childIdPtr);
}

// Get the next child of an item
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetNextChild(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id, void** cookie)
{
    wxTreeCtrl* tree = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* id = WXD_UNWRAP_TREE_ITEM_ID(item_id);
    if (!tree || !id || !id->IsOk() || !cookie || !*cookie)
        return nullptr;

    // Get wxWidgets cookie
    wxTreeItemIdValue& wxCookie = *reinterpret_cast<wxTreeItemIdValue*>(*cookie);

    // Get the next child
    wxTreeItemId childId = tree->GetNextChild(*id, wxCookie);
    if (!childId.IsOk()) {
        // No more children, clean up cookie
        delete reinterpret_cast<wxTreeItemIdValue*>(*cookie);
        *cookie = nullptr;
        return nullptr;
    }

    // Return the child ID
    wxTreeItemId* childIdPtr = new wxTreeItemId(childId);
    return WXD_WRAP_TREE_ITEM_ID(childIdPtr);
}

// Get the next sibling of an item
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetNextSibling(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id)
{
    wxTreeCtrl* tree = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* id = WXD_UNWRAP_TREE_ITEM_ID(item_id);
    if (!tree || !id || !id->IsOk())
        return nullptr;

    wxTreeItemId siblingId = tree->GetNextSibling(*id);
    if (!siblingId.IsOk())
        return nullptr;

    wxTreeItemId* siblingIdPtr = new wxTreeItemId(siblingId);
    return WXD_WRAP_TREE_ITEM_ID(siblingIdPtr);
}

// Get the number of children of an item
WXD_EXPORTED size_t
wxd_TreeCtrl_GetChildrenCount(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id, bool recursively)
{
    wxTreeCtrl* tree = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* id = WXD_UNWRAP_TREE_ITEM_ID(item_id);
    if (!tree || !id || !id->IsOk())
        return 0;

    return tree->GetChildrenCount(*id, recursively);
}

// Helper to get the wxTreeEvent from the generic wxEvent
static wxTreeEvent*
GetTreeEvent(wxd_Event_t* event)
{
    if (!event)
        return nullptr;
    wxEvent* eventPtr = reinterpret_cast<wxEvent*>(event);
    if (!eventPtr->IsKindOf(wxCLASSINFO(wxTreeEvent)))
        return nullptr;
    return static_cast<wxTreeEvent*>(eventPtr);
}

// Get the item from a tree event
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeEvent_GetItem(wxd_Event_t* event)
{
    wxTreeEvent* treeEvent = GetTreeEvent(event);
    if (!treeEvent)
        return nullptr;

    wxTreeItemId itemId = treeEvent->GetItem();
    if (!itemId.IsOk())
        return nullptr;

    wxTreeItemId* id = new wxTreeItemId(itemId);
    return WXD_WRAP_TREE_ITEM_ID(id);
}

// Get the old item from a tree event (for selection change events)
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeEvent_GetOldItem(wxd_Event_t* event)
{
    wxTreeEvent* treeEvent = GetTreeEvent(event);
    if (!treeEvent)
        return nullptr;

    wxTreeItemId oldItemId = treeEvent->GetOldItem();
    if (!oldItemId.IsOk())
        return nullptr;

    wxTreeItemId* id = new wxTreeItemId(oldItemId);
    return WXD_WRAP_TREE_ITEM_ID(id);
}

// Get the label from a tree event (for label editing events)
WXD_EXPORTED int
wxd_TreeEvent_GetLabel(wxd_Event_t* event, char* buffer, size_t buffer_len)
{
    wxTreeEvent* treeEvent = GetTreeEvent(event);
    if (!treeEvent)
        return -1;

    wxString label = treeEvent->GetLabel();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(label, buffer, buffer_len);
}

// Check if label editing was cancelled
WXD_EXPORTED int
wxd_TreeEvent_IsEditCancelled(wxd_Event_t* event)
{
    wxTreeEvent* treeEvent = GetTreeEvent(event);
    if (!treeEvent)
        return 0;

    return treeEvent->IsEditCancelled() ? 1 : 0;
}

// Helper to convert wxd_TreeItemIconType_t to wxTreeItemIcon
static wxTreeItemIcon
map_to_wx_tree_item_icon(wxd_TreeItemIconType_t which_wxd)
{
    switch (which_wxd) {
    case WXD_TreeItemIcon_Normal:
        return wxTreeItemIcon_Normal;
    case WXD_TreeItemIcon_Selected:
        return wxTreeItemIcon_Selected;
    case WXD_TreeItemIcon_Expanded:
        return wxTreeItemIcon_Expanded;
    case WXD_TreeItemIcon_SelectedExpanded:
        return wxTreeItemIcon_SelectedExpanded;
    default:
        return wxTreeItemIcon_Normal; // Fallback
    }
}

WXD_EXPORTED void
wxd_TreeCtrl_SetImageList(wxd_TreeCtrl_t* self, wxd_ImageList_t* imageList)
{
    wxTreeCtrl* treeCtrl = reinterpret_cast<wxTreeCtrl*>(self);
    wxImageList* wx_imageList = reinterpret_cast<wxImageList*>(imageList);
    if (treeCtrl) {
        treeCtrl->SetImageList(wx_imageList); // wxTreeCtrl takes ownership of the image list
    }
}

WXD_EXPORTED wxd_ImageList_t*
wxd_TreeCtrl_GetImageList(wxd_TreeCtrl_t* self)
{
    wxTreeCtrl* treeCtrl = reinterpret_cast<wxTreeCtrl*>(self);
    if (!treeCtrl)
        return nullptr;
    return reinterpret_cast<wxd_ImageList_t*>(treeCtrl->GetImageList());
}

WXD_EXPORTED void
wxd_TreeCtrl_SetItemImage(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, int image,
                          wxd_TreeItemIconType_t which)
{
    wxTreeCtrl* treeCtrl = reinterpret_cast<wxTreeCtrl*>(self);
    wxTreeItemId* wx_itemId = reinterpret_cast<wxTreeItemId*>(itemId);
    if (treeCtrl && wx_itemId && wx_itemId->IsOk()) {
        treeCtrl->SetItemImage(*wx_itemId, image, map_to_wx_tree_item_icon(which));
    }
}

WXD_EXPORTED int
wxd_TreeCtrl_GetItemImage(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId,
                          wxd_TreeItemIconType_t which)
{
    wxTreeCtrl* treeCtrl = reinterpret_cast<wxTreeCtrl*>(self);
    wxTreeItemId* wx_itemId = reinterpret_cast<wxTreeItemId*>(itemId);
    if (treeCtrl && wx_itemId && wx_itemId->IsOk()) {
        return treeCtrl->GetItemImage(*wx_itemId, map_to_wx_tree_item_icon(which));
    }
    return -1; // Default/error value
}

// Get the text label of an item
WXD_EXPORTED int
wxd_TreeCtrl_GetItemText(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, char* buffer, size_t buffer_len)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) {
        return -1;
    }

    wxString text = treeCtrl->GetItemText(*wx_itemId);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(text, buffer, buffer_len);
}

// Scrolls and/or expands items to ensure that the given item is visible
WXD_EXPORTED void
wxd_TreeCtrl_EnsureVisible(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) {
        return;
    }

    treeCtrl->EnsureVisible(*wx_itemId);
}

// Sets the currently focused item
WXD_EXPORTED void
wxd_TreeCtrl_SetFocusedItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) {
        return;
    }

    treeCtrl->SetFocusedItem(*wx_itemId);
}

// Expands all items in the tree
WXD_EXPORTED void
wxd_TreeCtrl_ExpandAll(wxd_TreeCtrl_t* self)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!treeCtrl) return;
    treeCtrl->ExpandAll();
}

// Set item text
WXD_EXPORTED void
wxd_TreeCtrl_SetItemText(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, const char* text)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->SetItemText(*wx_itemId, wxString::FromUTF8(text ? text : ""));
}

// Get focused item
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetFocusedItem(wxd_TreeCtrl_t* self)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!treeCtrl) return nullptr;
    wxTreeItemId focusedId = treeCtrl->GetFocusedItem();
    if (!focusedId.IsOk()) return nullptr;
    return WXD_WRAP_TREE_ITEM_ID(new wxTreeItemId(focusedId));
}

// Collapse
WXD_EXPORTED void
wxd_TreeCtrl_Collapse(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->Collapse(*wx_itemId);
}

// CollapseAll
WXD_EXPORTED void
wxd_TreeCtrl_CollapseAll(wxd_TreeCtrl_t* self)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!treeCtrl) return;
    treeCtrl->CollapseAll();
}

// CollapseAllChildren
WXD_EXPORTED void
wxd_TreeCtrl_CollapseAllChildren(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->CollapseAllChildren(*wx_itemId);
}

// CollapseAndReset
WXD_EXPORTED void
wxd_TreeCtrl_CollapseAndReset(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->CollapseAndReset(*wx_itemId);
}

// Toggle
WXD_EXPORTED void
wxd_TreeCtrl_Toggle(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->Toggle(*wx_itemId);
}

// IsExpanded
WXD_EXPORTED bool
wxd_TreeCtrl_IsExpanded(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return false;
    return treeCtrl->IsExpanded(*wx_itemId);
}

// IsSelected
WXD_EXPORTED bool
wxd_TreeCtrl_IsSelected(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return false;
    return treeCtrl->IsSelected(*wx_itemId);
}

// UnselectAll
WXD_EXPORTED void
wxd_TreeCtrl_UnselectAll(wxd_TreeCtrl_t* self)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!treeCtrl) return;
    treeCtrl->UnselectAll();
}

// UnselectItem
WXD_EXPORTED void
wxd_TreeCtrl_UnselectItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->UnselectItem(*wx_itemId);
}

// SelectAll - Note: Not available in wxTreeCtrl on all platforms
// Use SelectChildren or iterate through items instead
WXD_EXPORTED void
wxd_TreeCtrl_SelectAll(wxd_TreeCtrl_t* self)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!treeCtrl) return;
    // SelectAll is not available in generic wxTreeCtrl
    // Users should iterate through items and select them individually
    (void)treeCtrl;
}

// GetSelections
WXD_EXPORTED size_t
wxd_TreeCtrl_GetSelections(wxd_TreeCtrl_t* self, wxd_TreeItemId_t** items, size_t max_items)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!treeCtrl) return 0;

    wxArrayTreeItemIds selections;
    size_t count = treeCtrl->GetSelections(selections);

    if (items && max_items > 0) {
        size_t to_copy = (count < max_items) ? count : max_items;
        for (size_t i = 0; i < to_copy; i++) {
            items[i] = WXD_WRAP_TREE_ITEM_ID(new wxTreeItemId(selections[i]));
        }
    }
    return count;
}

// GetItemParent
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetItemParent(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return nullptr;
    wxTreeItemId parentId = treeCtrl->GetItemParent(*wx_itemId);
    if (!parentId.IsOk()) return nullptr;
    return WXD_WRAP_TREE_ITEM_ID(new wxTreeItemId(parentId));
}

// GetPrevSibling
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetPrevSibling(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return nullptr;
    wxTreeItemId siblingId = treeCtrl->GetPrevSibling(*wx_itemId);
    if (!siblingId.IsOk()) return nullptr;
    return WXD_WRAP_TREE_ITEM_ID(new wxTreeItemId(siblingId));
}

// GetLastChild
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_GetLastChild(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return nullptr;
    wxTreeItemId lastChildId = treeCtrl->GetLastChild(*wx_itemId);
    if (!lastChildId.IsOk()) return nullptr;
    return WXD_WRAP_TREE_ITEM_ID(new wxTreeItemId(lastChildId));
}

// IsVisible
WXD_EXPORTED bool
wxd_TreeCtrl_IsVisible(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return false;
    return treeCtrl->IsVisible(*wx_itemId);
}

// ItemHasChildren
WXD_EXPORTED bool
wxd_TreeCtrl_ItemHasChildren(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return false;
    return treeCtrl->ItemHasChildren(*wx_itemId);
}

// IsBold
WXD_EXPORTED bool
wxd_TreeCtrl_IsBold(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return false;
    return treeCtrl->IsBold(*wx_itemId);
}

// SetItemBold
WXD_EXPORTED void
wxd_TreeCtrl_SetItemBold(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, bool bold)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->SetItemBold(*wx_itemId, bold);
}

// SetItemTextColour
WXD_EXPORTED void
wxd_TreeCtrl_SetItemTextColour(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, wxd_Colour_t colour)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->SetItemTextColour(*wx_itemId, wxColour(colour.r, colour.g, colour.b, colour.a));
}

// GetItemTextColour
WXD_EXPORTED wxd_Colour_t
wxd_TreeCtrl_GetItemTextColour(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxd_Colour_t result = {0, 0, 0, 255};
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return result;
    wxColour col = treeCtrl->GetItemTextColour(*wx_itemId);
    result.r = col.Red(); result.g = col.Green(); result.b = col.Blue(); result.a = col.Alpha();
    return result;
}

// SetItemBackgroundColour
WXD_EXPORTED void
wxd_TreeCtrl_SetItemBackgroundColour(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, wxd_Colour_t colour)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->SetItemBackgroundColour(*wx_itemId, wxColour(colour.r, colour.g, colour.b, colour.a));
}

// GetItemBackgroundColour
WXD_EXPORTED wxd_Colour_t
wxd_TreeCtrl_GetItemBackgroundColour(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxd_Colour_t result = {255, 255, 255, 255};
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return result;
    wxColour col = treeCtrl->GetItemBackgroundColour(*wx_itemId);
    result.r = col.Red(); result.g = col.Green(); result.b = col.Blue(); result.a = col.Alpha();
    return result;
}

// SetItemFont
WXD_EXPORTED void
wxd_TreeCtrl_SetItemFont(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, wxd_Font_t* font)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    wxFont* wx_font = reinterpret_cast<wxFont*>(font);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk() || !wx_font) return;
    treeCtrl->SetItemFont(*wx_itemId, *wx_font);
}

// GetItemFont
WXD_EXPORTED wxd_Font_t*
wxd_TreeCtrl_GetItemFont(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return nullptr;
    wxFont font = treeCtrl->GetItemFont(*wx_itemId);
    if (!font.IsOk()) return nullptr;
    return reinterpret_cast<wxd_Font_t*>(new wxFont(font));
}

// InsertItem (after another item)
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_InsertItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* parent, wxd_TreeItemId_t* idPrevious,
                        const char* text, int image, int selImage, void* data)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_parent = WXD_UNWRAP_TREE_ITEM_ID(parent);
    wxTreeItemId* wx_prev = WXD_UNWRAP_TREE_ITEM_ID(idPrevious);
    if (!treeCtrl || !wx_parent || !wx_parent->IsOk() || !wx_prev || !wx_prev->IsOk()) return nullptr;
    wxString wxText = wxString::FromUTF8(text ? text : "");
    wxTreeItemId newId = treeCtrl->InsertItem(*wx_parent, *wx_prev, wxText, image, selImage,
                                               reinterpret_cast<wxTreeItemData*>(data));
    if (!newId.IsOk()) return nullptr;
    return WXD_WRAP_TREE_ITEM_ID(new wxTreeItemId(newId));
}

// InsertItemBefore (at position)
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_InsertItemBefore(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* parent, size_t pos,
                              const char* text, int image, int selImage, void* data)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_parent = WXD_UNWRAP_TREE_ITEM_ID(parent);
    if (!treeCtrl || !wx_parent || !wx_parent->IsOk()) return nullptr;
    wxString wxText = wxString::FromUTF8(text ? text : "");
    wxTreeItemId newId = treeCtrl->InsertItem(*wx_parent, pos, wxText, image, selImage,
                                               reinterpret_cast<wxTreeItemData*>(data));
    if (!newId.IsOk()) return nullptr;
    return WXD_WRAP_TREE_ITEM_ID(new wxTreeItemId(newId));
}

// PrependItem
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_PrependItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* parent, const char* text,
                         int image, int selImage, void* data)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_parent = WXD_UNWRAP_TREE_ITEM_ID(parent);
    if (!treeCtrl || !wx_parent || !wx_parent->IsOk()) return nullptr;
    wxString wxText = wxString::FromUTF8(text ? text : "");
    wxTreeItemId newId = treeCtrl->PrependItem(*wx_parent, wxText, image, selImage,
                                                reinterpret_cast<wxTreeItemData*>(data));
    if (!newId.IsOk()) return nullptr;
    return WXD_WRAP_TREE_ITEM_ID(new wxTreeItemId(newId));
}

// DeleteAllItems
WXD_EXPORTED void
wxd_TreeCtrl_DeleteAllItems(wxd_TreeCtrl_t* self)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!treeCtrl) return;
    treeCtrl->DeleteAllItems();
}

// DeleteChildren
WXD_EXPORTED void
wxd_TreeCtrl_DeleteChildren(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->DeleteChildren(*wx_itemId);
}

// GetCount
WXD_EXPORTED size_t
wxd_TreeCtrl_GetCount(wxd_TreeCtrl_t* self)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!treeCtrl) return 0;
    return treeCtrl->GetCount();
}

// EditLabel
WXD_EXPORTED wxd_TextCtrl_t*
wxd_TreeCtrl_EditLabel(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return nullptr;
    wxTextCtrl* editCtrl = treeCtrl->EditLabel(*wx_itemId);
    return reinterpret_cast<wxd_TextCtrl_t*>(editCtrl);
}

// EndEditLabel
WXD_EXPORTED void
wxd_TreeCtrl_EndEditLabel(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, bool discardChanges)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->EndEditLabel(*wx_itemId, discardChanges);
}

// GetEditControl
WXD_EXPORTED wxd_TextCtrl_t*
wxd_TreeCtrl_GetEditControl(wxd_TreeCtrl_t* self)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!treeCtrl) return nullptr;
    return reinterpret_cast<wxd_TextCtrl_t*>(treeCtrl->GetEditControl());
}

// ScrollTo
WXD_EXPORTED void
wxd_TreeCtrl_ScrollTo(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->ScrollTo(*wx_itemId);
}

// SortChildren
WXD_EXPORTED void
wxd_TreeCtrl_SortChildren(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->SortChildren(*wx_itemId);
}

// HitTest
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeCtrl_HitTest(wxd_TreeCtrl_t* self, wxd_Point point, int* flags)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!treeCtrl) return nullptr;
    int hitFlags = 0;
    wxTreeItemId hitId = treeCtrl->HitTest(wxPoint(point.x, point.y), hitFlags);
    if (flags) *flags = hitFlags;
    if (!hitId.IsOk()) return nullptr;
    return WXD_WRAP_TREE_ITEM_ID(new wxTreeItemId(hitId));
}

// GetBoundingRect
WXD_EXPORTED bool
wxd_TreeCtrl_GetBoundingRect(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, wxd_Rect* rect, bool textOnly)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk() || !rect) return false;
    wxRect wxRect;
    bool result = treeCtrl->GetBoundingRect(*wx_itemId, wxRect, textOnly);
    if (result) {
        rect->x = wxRect.x; rect->y = wxRect.y;
        rect->width = wxRect.width; rect->height = wxRect.height;
    }
    return result;
}

// SetStateImageList
WXD_EXPORTED void
wxd_TreeCtrl_SetStateImageList(wxd_TreeCtrl_t* self, wxd_ImageList_t* imageList)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxImageList* wx_imageList = reinterpret_cast<wxImageList*>(imageList);
    if (!treeCtrl) return;
    treeCtrl->SetStateImageList(wx_imageList);
}

// GetStateImageList
WXD_EXPORTED wxd_ImageList_t*
wxd_TreeCtrl_GetStateImageList(wxd_TreeCtrl_t* self)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    if (!treeCtrl) return nullptr;
    return reinterpret_cast<wxd_ImageList_t*>(treeCtrl->GetStateImageList());
}

// SetItemState
WXD_EXPORTED void
wxd_TreeCtrl_SetItemState(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, int state)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->SetItemState(*wx_itemId, state);
}

// GetItemState
WXD_EXPORTED int
wxd_TreeCtrl_GetItemState(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return -1;
    return treeCtrl->GetItemState(*wx_itemId);
}

// SetItemHasChildren
WXD_EXPORTED void
wxd_TreeCtrl_SetItemHasChildren(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, bool has)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    treeCtrl->SetItemHasChildren(*wx_itemId, has);
}

// EnableItem - Note: This may not be available in all wxWidgets versions
WXD_EXPORTED void
wxd_TreeCtrl_EnableItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* itemId, bool enable)
{
    wxTreeCtrl* treeCtrl = WXD_UNWRAP_TREE_CTRL(self);
    wxTreeItemId* wx_itemId = WXD_UNWRAP_TREE_ITEM_ID(itemId);
    if (!treeCtrl || !wx_itemId || !wx_itemId->IsOk()) return;
    // EnableItem may not be available in older wxWidgets versions
    // treeCtrl->EnableItem(*wx_itemId, enable);
    (void)enable; // Suppress unused parameter warning for now
}

} // extern "C"