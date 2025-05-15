#ifndef WXD_TREECTRL_H
#define WXD_TREECTRL_H

#include "../wxd_types.h"

// --- TreeItemData functions ---
// Create and manage TreeItemData objects
WXD_EXPORTED wxd_TreeItemData_t* wxd_TreeItemData_Create(void* client_data);
WXD_EXPORTED void wxd_TreeItemData_Free(wxd_TreeItemData_t* data);
WXD_EXPORTED void* wxd_TreeItemData_GetClientData(wxd_TreeItemData_t* data);
WXD_EXPORTED void wxd_TreeItemData_SetClientData(wxd_TreeItemData_t* data, void* client_data);

// --- TreeCtrl Functions ---
WXD_EXPORTED wxd_TreeCtrl_t* wxd_TreeCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED wxd_TreeItemId_t* wxd_TreeCtrl_AddRoot(wxd_TreeCtrl_t* self, const char* text, int image, int selImage, void* data);
WXD_EXPORTED wxd_TreeItemId_t* wxd_TreeCtrl_AppendItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* parent_id, const char* text, int image, int selImage, void* data);
WXD_EXPORTED void wxd_TreeCtrl_Delete(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id);
WXD_EXPORTED wxd_TreeItemId_t* wxd_TreeCtrl_GetSelection(wxd_TreeCtrl_t* self);
WXD_EXPORTED void wxd_TreeCtrl_SelectItem(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id);
WXD_EXPORTED int64_t wxd_TreeCtrl_GetItemData(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id);
WXD_EXPORTED bool wxd_TreeCtrl_SetItemData(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id, int64_t data);
WXD_EXPORTED void wxd_TreeItemId_Free(wxd_TreeItemId_t* item_id);
WXD_EXPORTED bool wxd_TreeItemId_IsOk(wxd_TreeItemId_t* item_id);
WXD_EXPORTED wxd_TreeItemId_t* wxd_TreeItemId_Clone(wxd_TreeItemId_t* item_id);

// New tree traversal functions
WXD_EXPORTED wxd_TreeItemId_t* wxd_TreeCtrl_GetRootItem(wxd_TreeCtrl_t* self);
WXD_EXPORTED wxd_TreeItemId_t* wxd_TreeCtrl_GetFirstChild(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id, void** cookie);
WXD_EXPORTED wxd_TreeItemId_t* wxd_TreeCtrl_GetNextChild(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id, void** cookie);
WXD_EXPORTED wxd_TreeItemId_t* wxd_TreeCtrl_GetNextSibling(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id);
WXD_EXPORTED size_t wxd_TreeCtrl_GetChildrenCount(wxd_TreeCtrl_t* self, wxd_TreeItemId_t* item_id, bool recursively);

#endif // WXD_TREECTRL_H 