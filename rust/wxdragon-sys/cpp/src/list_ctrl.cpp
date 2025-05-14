#include "wxdragon.h"
#include <wx/wx.h>
#include <wx/listctrl.h>
#include <wx/string.h> // For wxString::FromUTF8 / wxString::ToUTF8
#include "wxd_utils.h" // Added for copy_wxstring_to_buffer

// --- wxListCtrl ---

extern "C" {

WXD_EXPORTED wxd_ListCtrl_t* wxd_ListCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style) {
    return reinterpret_cast<wxd_ListCtrl_t*>(
        new wxListCtrl(
            reinterpret_cast<wxWindow*>(parent),
            id,
            wxPoint(pos.x, pos.y),
            wxSize(size.width, size.height),
            style
        )
    );
}

WXD_EXPORTED int32_t wxd_ListCtrl_InsertColumn(wxd_ListCtrl_t* self, long col, const char* heading, int format, int width) {
    if (!self) return -1;
    return static_cast<int32_t>(reinterpret_cast<wxListCtrl*>(self)->InsertColumn(col, wxString::FromUTF8(heading), format, width));
}

WXD_EXPORTED bool wxd_ListCtrl_SetColumnWidth(wxd_ListCtrl_t* self, long col, int width) {
    if (!self) return false;
    return reinterpret_cast<wxListCtrl*>(self)->SetColumnWidth(col, width);
}

WXD_EXPORTED int wxd_ListCtrl_GetColumnWidth(wxd_ListCtrl_t* self, long col) {
    if (!self) return -1; // wxLIST_AUTOSIZE_USEHEADER or actual width
    return reinterpret_cast<wxListCtrl*>(self)->GetColumnWidth(col);
}

WXD_EXPORTED int wxd_ListCtrl_GetColumnCount(wxd_ListCtrl_t* self) {
    if (!self) return 0;
    return reinterpret_cast<wxListCtrl*>(self)->GetColumnCount();
}

WXD_EXPORTED int32_t wxd_ListCtrl_InsertItem_Simple(wxd_ListCtrl_t* self, long index, const char* label) {
    if (!self) return -1;
    wxListItem item;
    item.SetId(index); // This sets the position where item is inserted
    item.SetText(wxString::FromUTF8(label));
    // For other views, you might set image, etc.
    // item.SetMask(wxLIST_MASK_TEXT | wxLIST_MASK_IMAGE | wxLIST_MASK_DATA); // if using data/image
    return static_cast<int32_t>(reinterpret_cast<wxListCtrl*>(self)->InsertItem(item));
}

WXD_EXPORTED void wxd_ListCtrl_SetItemText(wxd_ListCtrl_t* self, long index, const char* text) {
    if (!self) return;
    // Use the 2-argument SetItemText (likely for the main item label)
    reinterpret_cast<wxListCtrl*>(self)->SetItemText(index, wxString::FromUTF8(text ? text : ""));
}

WXD_EXPORTED int wxd_ListCtrl_GetItemText(wxd_ListCtrl_t* self, long index, int col, char* buffer, int buffer_len) {
    if (!self) return -1;
    wxString text = reinterpret_cast<wxListCtrl*>(self)->GetItemText(index, col);
    // Use the utility function
    size_t source_len_no_null = wxd_cpp_utils::copy_wxstring_to_buffer(text, buffer, static_cast<size_t>(buffer_len));
    return static_cast<int>(source_len_no_null); // Return original length, caller can check against buffer_len
}

WXD_EXPORTED int wxd_ListCtrl_GetItemCount(wxd_ListCtrl_t* self) {
    if (!self) return 0;
    return reinterpret_cast<wxListCtrl*>(self)->GetItemCount();
}

WXD_EXPORTED bool wxd_ListCtrl_SetItemState(wxd_ListCtrl_t* self, long item, long state, long stateMask) {
    if (!self) return false;
    return reinterpret_cast<wxListCtrl*>(self)->SetItemState(item, state, stateMask);
}

WXD_EXPORTED int32_t wxd_ListCtrl_GetItemState(wxd_ListCtrl_t* self, long item, long stateMask) {
    if (!self) return 0; // Or some error indicator
    return static_cast<int32_t>(reinterpret_cast<wxListCtrl*>(self)->GetItemState(item, stateMask));
}

WXD_EXPORTED int32_t wxd_ListCtrl_GetNextItem(wxd_ListCtrl_t* self, long item, int geometry, int state) {
    if (!self) return -1; // wxNOT_FOUND
    return static_cast<int32_t>(reinterpret_cast<wxListCtrl*>(self)->GetNextItem(item, geometry, state));
}

WXD_EXPORTED bool wxd_ListCtrl_DeleteItem(wxd_ListCtrl_t* self, long item) {
    if (!self) return false;
    return reinterpret_cast<wxListCtrl*>(self)->DeleteItem(item);
}

WXD_EXPORTED bool wxd_ListCtrl_DeleteAllItems(wxd_ListCtrl_t* self) {
    if (!self) return false;
    return reinterpret_cast<wxListCtrl*>(self)->DeleteAllItems();
}

WXD_EXPORTED bool wxd_ListCtrl_ClearAll(wxd_ListCtrl_t* self) {
    if (!self) return false;
    reinterpret_cast<wxListCtrl*>(self)->ClearAll(); // wxListCtrl::ClearAll is void, but DeleteAllColumns is bool
    return reinterpret_cast<wxListCtrl*>(self)->DeleteAllColumns();
}

WXD_EXPORTED int wxd_ListCtrl_GetSelectedItemCount(wxd_ListCtrl_t* self) {
    if (!self) return 0;
    return reinterpret_cast<wxListCtrl*>(self)->GetSelectedItemCount();
}

WXD_EXPORTED bool wxd_ListCtrl_EnsureVisible(wxd_ListCtrl_t* self, long item) {
    if (!self) return false;
    return reinterpret_cast<wxListCtrl*>(self)->EnsureVisible(item);
}

WXD_EXPORTED int32_t wxd_ListCtrl_HitTest(wxd_ListCtrl_t* self, wxd_Point point, int* flags_ptr, long* subitem_ptr) {
    if (!self || !flags_ptr || !subitem_ptr) return -1; // wxNOT_FOUND
    wxPoint pt(point.x, point.y);
    int flags;
    long subitem; // wxListCtrl uses long for subitem index in HitTest
    long item = reinterpret_cast<wxListCtrl*>(self)->HitTest(pt, flags, &subitem);
    *flags_ptr = flags;
    *subitem_ptr = subitem;
    return static_cast<int32_t>(item);
}

WXD_EXPORTED wxd_TextCtrl_t* wxd_ListCtrl_EditLabel(wxd_ListCtrl_t* self, long item) {
    if (!self) return nullptr;
    wxListCtrl* list_ctrl = reinterpret_cast<wxListCtrl*>(self);
    wxTextCtrl* text_ctrl = list_ctrl->EditLabel(item);
    return reinterpret_cast<wxd_TextCtrl_t*>(text_ctrl);
}

// --- ListCtrl Event Data Accessors ---
WXD_EXPORTED int32_t wxd_ListEvent_GetItemIndex(wxd_Event_t* event) {
    if (!event) return -1;
    wxListEvent* evt = static_cast<wxListEvent*>(reinterpret_cast<wxEvent*>(event));
    return static_cast<int32_t>(evt->GetIndex());
}

WXD_EXPORTED int wxd_ListEvent_GetColumn(wxd_Event_t* event) {
    if (!event) return -1; // Or some other error value
    wxListEvent* evt = static_cast<wxListEvent*>(reinterpret_cast<wxEvent*>(event));
    return evt->GetColumn(); // For column click events
}

WXD_EXPORTED int wxd_ListEvent_GetLabel(wxd_Event_t* event, char* buffer, int buffer_len) {
    if (!event) { // Simplified check
        if (buffer && buffer_len > 0) buffer[0] = '\0';
        return -1; // Or 0?
    }
    wxListEvent* evt = static_cast<wxListEvent*>(reinterpret_cast<wxEvent*>(event));
    wxString label = evt->GetLabel(); // or evt->GetText()
    
    size_t source_len_no_null = wxd_cpp_utils::copy_wxstring_to_buffer(label, buffer, static_cast<size_t>(buffer_len));
    return static_cast<int>(source_len_no_null);
}

WXD_EXPORTED bool wxd_ListEvent_IsEditCancelled(wxd_Event_t* event) {
    if (!event) return true; // Default to cancelled if event is null
    wxListEvent* evt = static_cast<wxListEvent*>(reinterpret_cast<wxEvent*>(event));
    return evt->IsEditCancelled();
}

// --- Advanced ListCtrl Functions ---

// Item Data Functions
WXD_EXPORTED bool wxd_ListCtrl_SetItemData(wxd_ListCtrl_t* self, long item, long data) {
    if (!self) return false;
    return reinterpret_cast<wxListCtrl*>(self)->SetItemData(item, data);
}

WXD_EXPORTED bool wxd_ListCtrl_SetItemPtrData(wxd_ListCtrl_t* self, long item, void* data) {
    if (!self) return false;
    return reinterpret_cast<wxListCtrl*>(self)->SetItemPtrData(item, wxPtrToUInt(data));
}

WXD_EXPORTED long wxd_ListCtrl_GetItemData(wxd_ListCtrl_t* self, long item) {
    if (!self) return 0;
    return reinterpret_cast<wxListCtrl*>(self)->GetItemData(item);
}

WXD_EXPORTED void* wxd_ListCtrl_GetItemPtrData(wxd_ListCtrl_t* self, long item) {
    if (!self) return nullptr;
    wxUIntPtr data = reinterpret_cast<wxListCtrl*>(self)->GetItemData(item);
    return wxUIntToPtr(data);
}

// Item Appearance
WXD_EXPORTED void wxd_ListCtrl_SetItemBackgroundColour(wxd_ListCtrl_t* self, long item, wxd_Colour_t colour) {
    if (!self) return;
    wxColour wxColor(colour.r, colour.g, colour.b, colour.a);
    reinterpret_cast<wxListCtrl*>(self)->SetItemBackgroundColour(item, wxColor);
}

WXD_EXPORTED void wxd_ListCtrl_SetItemTextColour(wxd_ListCtrl_t* self, long item, wxd_Colour_t colour) {
    if (!self) return;
    wxColour wxColor(colour.r, colour.g, colour.b, colour.a);
    reinterpret_cast<wxListCtrl*>(self)->SetItemTextColour(item, wxColor);
}

WXD_EXPORTED wxd_Colour_t wxd_ListCtrl_GetItemBackgroundColour(wxd_ListCtrl_t* self, long item) {
    wxd_Colour_t colour = {0, 0, 0, 255}; // Default black, fully opaque
    if (!self) return colour;
    
    wxColour wxColor = reinterpret_cast<wxListCtrl*>(self)->GetItemBackgroundColour(item);
    colour.r = wxColor.Red();
    colour.g = wxColor.Green();
    colour.b = wxColor.Blue();
    colour.a = wxColor.Alpha();
    return colour;
}

WXD_EXPORTED wxd_Colour_t wxd_ListCtrl_GetItemTextColour(wxd_ListCtrl_t* self, long item) {
    wxd_Colour_t colour = {0, 0, 0, 255}; // Default black, fully opaque
    if (!self) return colour;
    
    wxColour wxColor = reinterpret_cast<wxListCtrl*>(self)->GetItemTextColour(item);
    colour.r = wxColor.Red();
    colour.g = wxColor.Green();
    colour.b = wxColor.Blue();
    colour.a = wxColor.Alpha();
    return colour;
}

// Column Management
WXD_EXPORTED bool wxd_ListCtrl_SetColumnsOrder(wxd_ListCtrl_t* self, int count, int* orders) {
    if (!self || !orders) return false;
    
    wxArrayInt orderArray;
    for (int i = 0; i < count; i++) {
        orderArray.Add(orders[i]);
    }
    
    return reinterpret_cast<wxListCtrl*>(self)->SetColumnsOrder(orderArray);
}

WXD_EXPORTED int* wxd_ListCtrl_GetColumnsOrder(wxd_ListCtrl_t* self, int* count) {
    if (!self || !count) return nullptr;
    
    wxArrayInt orderArray = reinterpret_cast<wxListCtrl*>(self)->GetColumnsOrder();
    *count = orderArray.GetCount();
    
    if (*count == 0) return nullptr;
    
    int* result = (int*)malloc(*count * sizeof(int));
    if (!result) return nullptr;
    
    for (int i = 0; i < *count; i++) {
        result[i] = orderArray[i];
    }
    
    return result;
}

WXD_EXPORTED int wxd_ListCtrl_GetColumnOrder(wxd_ListCtrl_t* self, int col) {
    if (!self) return -1;
    return reinterpret_cast<wxListCtrl*>(self)->GetColumnOrder(col);
}

WXD_EXPORTED int wxd_ListCtrl_GetColumnIndexFromOrder(wxd_ListCtrl_t* self, int pos) {
    if (!self) return -1;
    return reinterpret_cast<wxListCtrl*>(self)->GetColumnIndexFromOrder(pos);
}

// Virtual List Support
WXD_EXPORTED void wxd_ListCtrl_SetItemCount(wxd_ListCtrl_t* self, long count) {
    if (!self) return;
    reinterpret_cast<wxListCtrl*>(self)->SetItemCount(count);
}

WXD_EXPORTED void wxd_ListCtrl_RefreshItem(wxd_ListCtrl_t* self, long item) {
    if (!self) return;
    reinterpret_cast<wxListCtrl*>(self)->RefreshItem(item);
}

WXD_EXPORTED void wxd_ListCtrl_RefreshItems(wxd_ListCtrl_t* self, long itemFrom, long itemTo) {
    if (!self) return;
    reinterpret_cast<wxListCtrl*>(self)->RefreshItems(itemFrom, itemTo);
}

// Sorting - This is a bit tricky because of the callback
// We'll need a mapping system or to adapt this for Rust usage
struct SortCallbackData {
    int (*cmpFunc)(void*, void*, void*);
    void* userData;
};

int WXDLLEXPORT wxListCompareFunction(wxIntPtr item1, wxIntPtr item2, wxIntPtr sortData) {
    SortCallbackData* cbData = (SortCallbackData*)sortData;
    if (!cbData || !cbData->cmpFunc) return 0;
    
    return cbData->cmpFunc((void*)item1, (void*)item2, cbData->userData);
}

WXD_EXPORTED bool wxd_ListCtrl_SortItems(wxd_ListCtrl_t* self, int (*cmpFunc)(void*, void*, void*), void* data) {
    if (!self || !cmpFunc) return false;
    
    // Create a persistent callback data structure
    SortCallbackData* cbData = new SortCallbackData();
    cbData->cmpFunc = cmpFunc;
    cbData->userData = data;
    
    bool result = reinterpret_cast<wxListCtrl*>(self)->SortItems(wxListCompareFunction, (wxIntPtr)cbData);
    
    // Clean up after sorting is done
    delete cbData;
    
    return result;
}

WXD_EXPORTED void wxd_ListCtrl_ShowSortIndicator(wxd_ListCtrl_t* self, int col, bool ascending) {
    if (!self) return;
    reinterpret_cast<wxListCtrl*>(self)->ShowSortIndicator(col, ascending);
}

// Image List Support
// Note: This will need proper integration with wxdragon's ImageList implementation
WXD_EXPORTED void wxd_ListCtrl_SetImageList(wxd_ListCtrl_t* self, void* imageList, int which) {
    if (!self || !imageList) return;
    reinterpret_cast<wxListCtrl*>(self)->SetImageList(reinterpret_cast<wxImageList*>(imageList), which);
}

WXD_EXPORTED void wxd_ListCtrl_AssignImageList(wxd_ListCtrl_t* self, void* imageList, int which) {
    if (!self || !imageList) return;
    reinterpret_cast<wxListCtrl*>(self)->AssignImageList(reinterpret_cast<wxImageList*>(imageList), which);
}

WXD_EXPORTED void* wxd_ListCtrl_GetImageList(wxd_ListCtrl_t* self, int which) {
    if (!self) return nullptr;
    return reinterpret_cast<void*>(reinterpret_cast<wxListCtrl*>(self)->GetImageList(which));
}

WXD_EXPORTED bool wxd_ListCtrl_SetItem(wxd_ListCtrl_t* self, long item, int col, const char* text, int image, int format, long state, long stateMask, long data, long mask) {
    if (!self) return false;
    
    wxListItem listItem;
    listItem.SetId(item);
    listItem.SetColumn(col);
    
    if (mask & wxLIST_MASK_TEXT) {
        listItem.SetText(wxString::FromUTF8(text ? text : ""));
    }
    
    if (mask & wxLIST_MASK_IMAGE) {
        listItem.SetImage(image);
    }
    
    if (mask & wxLIST_MASK_FORMAT) {
        listItem.SetAlign(static_cast<wxListColumnFormat>(format));
    }
    
    if (mask & wxLIST_MASK_STATE) {
        listItem.SetState(state);
        listItem.SetStateMask(stateMask);
    }
    
    if (mask & wxLIST_MASK_DATA) {
        listItem.SetData(data);
    }
    
    listItem.SetMask(mask);
    
    return reinterpret_cast<wxListCtrl*>(self)->SetItem(listItem);
}

} // extern "C" 