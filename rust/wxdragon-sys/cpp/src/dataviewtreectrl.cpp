#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/dataview.h>
#include <wx/imaglist.h>
#include <wx/valgen.h> // For wxDefaultValidator, though not directly used by DVTC constructor

// Helper to cast to wxDataViewTreeCtrl*
static inline wxDataViewTreeCtrl* ToWxDVTC(wxd_Window_t* self) {
    return wxDynamicCast(reinterpret_cast<wxWindow*>(self), wxDataViewTreeCtrl);
}

// Helper to convert wxd_DataViewItem_t to wxDataViewItem
// wxd_DataViewItem_t.id is wxDataViewItem*
static inline wxDataViewItem ToWxDVI(wxd_DataViewItem_t item_wrapper) {
    if (item_wrapper.id == nullptr) {
        return wxDataViewItem(nullptr);
    }
    return *reinterpret_cast<wxDataViewItem*>(item_wrapper.id);
}

// Helper to convert wxDataViewItem to wxd_DataViewItem_t
// Creates a new wxDataViewItem on the heap for Rust to own.
static inline wxd_DataViewItem_t FromWxDVI(const wxDataViewItem& item) {
    if (!item.IsOk()) {
        return {nullptr}; // Return a wxd_DataViewItem_t with a null id
    }
    wxDataViewItem* heap_item = new wxDataViewItem(item);
    return {reinterpret_cast<void*>(heap_item)};
}

// --- Constructor ---
WXD_EXPORTED wxd_Window_t* wxd_DataViewTreeCtrl_new(
    wxd_Window_t* parent_ptr,
    int id,
    wxd_Point pos_rust,
    wxd_Size size_rust,
    int64_t style,
    wxd_Window_t* validator_ptr, // This parameter is not used by wxDataViewTreeCtrl constructor
    const char* name_str) // This parameter will be ignored for DVTC constructor
{
    wxWindow* parent = reinterpret_cast<wxWindow*>(parent_ptr);
    
    // wxDataViewTreeCtrl constructor takes 6 arguments:
    // wxWindow *parent, wxWindowID id, const wxPoint& pos, 
    // const wxSize& size, long style, const wxValidator& validator.
    // It does not take a name directly in this common constructor.
    // The name_str FFI parameter will be ignored for the constructor itself.

    wxDataViewTreeCtrl* ctrl = new wxDataViewTreeCtrl(
        parent,
        static_cast<wxWindowID>(id),
        wxd_cpp_utils::to_wx(pos_rust),
        wxd_cpp_utils::to_wx(size_rust),
        static_cast<long>(style),
        wxDefaultValidator // Pass default validator for the 6th argument
        // name_str is not passed to this constructor
    );
    
    // If name_str needs to be set, it would be done via SetName after construction:
    // if (name_str && *name_str) { // Check if not null and not empty
    //     ctrl->SetName(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(name_str));
    // }

    return reinterpret_cast<wxd_Window_t*>(static_cast<wxWindow*>(ctrl));
}

// --- Item Management ---
WXD_EXPORTED wxd_DataViewItem_t wxd_DataViewTreeCtrl_AppendItem(wxd_Window_t* self, wxd_DataViewItem_t parent_wrapper, const char* text, int icon) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return {nullptr};
    wxDataViewItem parent_item = ToWxDVI(parent_wrapper);
    wxString wx_text = wxString::FromUTF8(text ? text : "");
    wxDataViewItem new_item = ctrl->AppendItem(parent_item, wx_text, icon, nullptr /*client_data*/);
    return FromWxDVI(new_item);
}

WXD_EXPORTED wxd_DataViewItem_t wxd_DataViewTreeCtrl_AppendContainer(wxd_Window_t* self, wxd_DataViewItem_t parent_wrapper, const char* text, int icon, int expanded_icon) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return {nullptr};
    wxDataViewItem parent_item = ToWxDVI(parent_wrapper);
    wxString wx_text = wxString::FromUTF8(text ? text : "");
    wxDataViewItem new_item = ctrl->AppendContainer(parent_item, wx_text, icon, expanded_icon, nullptr /*client_data*/);
    return FromWxDVI(new_item);
}

WXD_EXPORTED wxd_DataViewItem_t wxd_DataViewTreeCtrl_PrependItem(wxd_Window_t* self, wxd_DataViewItem_t parent_wrapper, const char* text, int icon) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return {nullptr};
    wxDataViewItem parent_item = ToWxDVI(parent_wrapper);
    wxString wx_text = wxString::FromUTF8(text ? text : "");
    wxDataViewItem new_item = ctrl->PrependItem(parent_item, wx_text, icon, nullptr /*client_data*/);
    return FromWxDVI(new_item);
}

WXD_EXPORTED wxd_DataViewItem_t wxd_DataViewTreeCtrl_PrependContainer(wxd_Window_t* self, wxd_DataViewItem_t parent_wrapper, const char* text, int icon, int expanded_icon) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return {nullptr};
    wxDataViewItem parent_item = ToWxDVI(parent_wrapper);
    wxString wx_text = wxString::FromUTF8(text ? text : "");
    wxDataViewItem new_item = ctrl->PrependContainer(parent_item, wx_text, icon, expanded_icon, nullptr /*client_data*/);
    return FromWxDVI(new_item);
}

WXD_EXPORTED wxd_DataViewItem_t wxd_DataViewTreeCtrl_InsertItem(wxd_Window_t* self, wxd_DataViewItem_t parent_wrapper, wxd_DataViewItem_t previous_wrapper, const char* text, int icon) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return {nullptr};
    wxDataViewItem parent_item = ToWxDVI(parent_wrapper);
    wxDataViewItem previous_item = ToWxDVI(previous_wrapper);
    wxString wx_text = wxString::FromUTF8(text ? text : "");
    wxDataViewItem new_item = ctrl->InsertItem(parent_item, previous_item, wx_text, icon, nullptr /*client_data*/);
    return FromWxDVI(new_item);
}

WXD_EXPORTED wxd_DataViewItem_t wxd_DataViewTreeCtrl_InsertContainer(wxd_Window_t* self, wxd_DataViewItem_t parent_wrapper, wxd_DataViewItem_t previous_wrapper, const char* text, int icon, int expanded_icon) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return {nullptr};
    wxDataViewItem parent_item = ToWxDVI(parent_wrapper);
    wxDataViewItem previous_item = ToWxDVI(previous_wrapper);
    wxString wx_text = wxString::FromUTF8(text ? text : "");
    wxDataViewItem new_item = ctrl->InsertContainer(parent_item, previous_item, wx_text, icon, expanded_icon, nullptr /*client_data*/);
    return FromWxDVI(new_item);
}

WXD_EXPORTED void wxd_DataViewTreeCtrl_DeleteItem(wxd_Window_t* self, wxd_DataViewItem_t item_wrapper) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return;
    wxDataViewItem item = ToWxDVI(item_wrapper);
    // It's important that item is valid for DeleteItem.
    // If item_wrapper.id was null, ToWxDVI returns an invalid wxDataViewItem,
    // and calling DeleteItem with it is fine (it will do nothing).
    ctrl->DeleteItem(item);
    // Rust owns the memory for item_wrapper.id (the heap-allocated wxDataViewItem)
    // and will free it via wxd_DataViewItem_Release.
}

WXD_EXPORTED void wxd_DataViewTreeCtrl_DeleteChildren(wxd_Window_t* self, wxd_DataViewItem_t parent_wrapper) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return;
    wxDataViewItem parent_item = ToWxDVI(parent_wrapper);
    ctrl->DeleteChildren(parent_item);
}

WXD_EXPORTED void wxd_DataViewTreeCtrl_DeleteAllItems(wxd_Window_t* self) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return;
    ctrl->DeleteAllItems();
}

// --- Item Attributes ---
WXD_EXPORTED const char* wxd_DataViewTreeCtrl_GetItemText(wxd_Window_t* self, wxd_DataViewItem_t item_wrapper) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return nullptr;
    wxDataViewItem item = ToWxDVI(item_wrapper);
    if (!item.IsOk()) return nullptr;
    wxString text = ctrl->GetItemText(item);
    return wxd_str_to_c_str(text); // Rust frees via wxd_free_string
}

WXD_EXPORTED void wxd_DataViewTreeCtrl_SetItemText(wxd_Window_t* self, wxd_DataViewItem_t item_wrapper, const char* text) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return;
    wxDataViewItem item = ToWxDVI(item_wrapper);
    if (!item.IsOk()) return;
    wxString wx_text = wxString::FromUTF8(text ? text : "");
    ctrl->SetItemText(item, wx_text);
}

WXD_EXPORTED void wxd_DataViewTreeCtrl_SetItemIcon(wxd_Window_t* self, wxd_DataViewItem_t item_wrapper, int icon_idx) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return;
    wxDataViewItem item = ToWxDVI(item_wrapper);
    if (!item.IsOk()) return;

    wxImageList* img_list = ctrl->GetImageList();
    if (img_list && icon_idx >= 0 && icon_idx < img_list->GetImageCount()) {
        wxIcon icon = img_list->GetIcon(icon_idx);
        if (icon.IsOk()) {
            // wxDataViewTreeCtrl::SetItemIcon takes wxBitmapBundle
            ctrl->SetItemIcon(item, wxBitmapBundle(icon));
        }
    } else if (icon_idx == -1) {
        // Allow clearing the icon by passing -1
        ctrl->SetItemIcon(item, wxBitmapBundle());
    }
}

WXD_EXPORTED void wxd_DataViewTreeCtrl_SetItemExpandedIcon(wxd_Window_t* self, wxd_DataViewItem_t item_wrapper, int icon_idx) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return;
    wxDataViewItem item = ToWxDVI(item_wrapper);
    if (!item.IsOk() || !ctrl->IsContainer(item)) return;

    wxImageList* img_list = ctrl->GetImageList();
    if (img_list && icon_idx >= 0 && icon_idx < img_list->GetImageCount()) {
        wxIcon icon = img_list->GetIcon(icon_idx);
        if (icon.IsOk()) {
            ctrl->SetItemExpandedIcon(item, wxBitmapBundle(icon));
        }
    } else if (icon_idx == -1) {
        ctrl->SetItemExpandedIcon(item, wxBitmapBundle());
    }
}

// --- Item Relationships ---
WXD_EXPORTED wxd_DataViewItem_t wxd_DataViewTreeCtrl_GetItemParent(wxd_Window_t* self, wxd_DataViewItem_t item_wrapper) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return {nullptr};
    wxDataViewItem item = ToWxDVI(item_wrapper);
    if (!item.IsOk()) return {nullptr}; // Cannot get parent of an invalid item
    wxDataViewItem parent_item = ctrl->GetItemParent(item);
    return FromWxDVI(parent_item);
}

WXD_EXPORTED unsigned int wxd_DataViewTreeCtrl_GetChildCount(wxd_Window_t* self, wxd_DataViewItem_t parent_wrapper) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return 0;
    wxDataViewItem parent_item = ToWxDVI(parent_wrapper);
    // GetChildCount on an invalid parent_item is okay (means root items)
    return ctrl->GetChildCount(parent_item);
}

WXD_EXPORTED wxd_DataViewItem_t wxd_DataViewTreeCtrl_GetNthChild(wxd_Window_t* self, wxd_DataViewItem_t parent_wrapper, unsigned int pos) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return {nullptr};
    wxDataViewItem parent_item = ToWxDVI(parent_wrapper);
    wxDataViewItem child_item = ctrl->GetNthChild(parent_item, pos);
    return FromWxDVI(child_item);
}

WXD_EXPORTED bool wxd_DataViewTreeCtrl_IsContainer(wxd_Window_t* self, wxd_DataViewItem_t item_wrapper) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return false;
    wxDataViewItem item = ToWxDVI(item_wrapper);
    if (!item.IsOk()) return false;
    return ctrl->IsContainer(item);
}

// --- Tree State ---
WXD_EXPORTED void wxd_DataViewTreeCtrl_Expand(wxd_Window_t* self, wxd_DataViewItem_t item_wrapper) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return;
    wxDataViewItem item = ToWxDVI(item_wrapper);
    if (item.IsOk()) ctrl->Expand(item);
}

WXD_EXPORTED void wxd_DataViewTreeCtrl_Collapse(wxd_Window_t* self, wxd_DataViewItem_t item_wrapper) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return;
    wxDataViewItem item = ToWxDVI(item_wrapper);
    if (item.IsOk()) ctrl->Collapse(item);
}

WXD_EXPORTED bool wxd_DataViewTreeCtrl_IsExpanded(wxd_Window_t* self, wxd_DataViewItem_t item_wrapper) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return false;
    wxDataViewItem item = ToWxDVI(item_wrapper);
    if (!item.IsOk()) return false;
    return ctrl->IsExpanded(item);
}

// --- Image List ---
WXD_EXPORTED void wxd_DataViewTreeCtrl_SetImageList(wxd_Window_t* self, wxd_ImageList_t* image_list_ptr) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return;
    wxImageList* img_list = reinterpret_cast<wxImageList*>(image_list_ptr);
    ctrl->SetImageList(img_list);
}

WXD_EXPORTED wxd_ImageList_t* wxd_DataViewTreeCtrl_GetImageList(wxd_Window_t* self) {
    wxDataViewTreeCtrl* ctrl = ToWxDVTC(self);
    if (!ctrl) return nullptr;
    return reinterpret_cast<wxd_ImageList_t*>(ctrl->GetImageList());
} 