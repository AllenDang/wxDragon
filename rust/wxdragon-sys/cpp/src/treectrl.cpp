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

} // extern "C"