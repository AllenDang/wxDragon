#include "wx/wxprec.h"

#ifndef WX_PRECOMP
    #include "wx/wx.h"
#endif

#include "wx/treelist.h"
#include "../include/wxdragon.h"
#include "wxd_utils.h"

extern "C" {

// Create a new wxTreeListCtrl
WXD_EXPORTED wxd_TreeListCtrl_t* wxd_TreeListCtrl_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
) {
    wxWindow* parentWin = (wxWindow*)parent;

    // Convert style flags - wxTreeListCtrl uses different style constants
    long wxStyle = 0;
    if (style & 1) wxStyle |= wxTL_CHECKBOX;  // Checkbox style
    if (style & 2) wxStyle |= wxTL_3STATE;   // 3-state checkbox style

    wxTreeListCtrl* ctrl = new wxTreeListCtrl(
        parentWin,
        id,
        wxd_cpp_utils::to_wx(pos),
        wxd_cpp_utils::to_wx(size),
        wxStyle
    );
    return (wxd_TreeListCtrl_t*)ctrl;
}

// Column management operations
WXD_EXPORTED int wxd_TreeListCtrl_AppendColumn(wxd_TreeListCtrl_t* self, const char* text, int width, int align) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl || !text) return -1;

    wxAlignment alignment = wxALIGN_LEFT;
    switch (align) {
        case 1: alignment = wxALIGN_RIGHT; break;
        case 2: alignment = wxALIGN_CENTER; break;
        default: alignment = wxALIGN_LEFT; break;
    }

    return ctrl->AppendColumn(wxString::FromUTF8(text), width, alignment);
}

WXD_EXPORTED int wxd_TreeListCtrl_GetColumnCount(wxd_TreeListCtrl_t* self) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->GetColumnCount();
}

WXD_EXPORTED void wxd_TreeListCtrl_SetColumnWidth(wxd_TreeListCtrl_t* self, int col, int width) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        ctrl->SetColumnWidth(col, width);
    }
}

WXD_EXPORTED int wxd_TreeListCtrl_GetColumnWidth(wxd_TreeListCtrl_t* self, int col) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return 0;
    return ctrl->GetColumnWidth(col);
}

// Item management operations
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetRootItem(wxd_TreeListCtrl_t* self) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return 0;
    wxTreeListItem root = ctrl->GetRootItem();
    return (wxd_Long_t)root.GetID();
}

WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_AppendItem(wxd_TreeListCtrl_t* self, wxd_Long_t parent, const char* text) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl || !text) return 0;

    wxTreeListItem parentItem(reinterpret_cast<wxTreeListModelNode*>(parent));
    wxTreeListItem newItem = ctrl->AppendItem(parentItem, wxString::FromUTF8(text));
    return (wxd_Long_t)newItem.GetID();
}

WXD_EXPORTED void wxd_TreeListCtrl_DeleteItem(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        ctrl->DeleteItem(treeItem);
    }
}

WXD_EXPORTED void wxd_TreeListCtrl_DeleteAllItems(wxd_TreeListCtrl_t* self) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        ctrl->DeleteAllItems();
    }
}

WXD_EXPORTED void wxd_TreeListCtrl_SetItemText(wxd_TreeListCtrl_t* self, wxd_Long_t item, int col, const char* text) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl && text) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        ctrl->SetItemText(treeItem, col, wxString::FromUTF8(text));
    }
}

WXD_EXPORTED int wxd_TreeListCtrl_GetItemText(wxd_TreeListCtrl_t* self, wxd_Long_t item, int col, char* buffer, int buffer_len) {
    if (!self || !buffer || buffer_len <= 0) return -1;
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
    wxString text = ctrl->GetItemText(treeItem, col);
    return wxd_cpp_utils::copy_wxstring_to_buffer(text, buffer, (size_t)buffer_len);
}

// Tree operations
WXD_EXPORTED void wxd_TreeListCtrl_Expand(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        ctrl->Expand(treeItem);
    }
}

WXD_EXPORTED void wxd_TreeListCtrl_Collapse(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        ctrl->Collapse(treeItem);
    }
}

WXD_EXPORTED bool wxd_TreeListCtrl_IsExpanded(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return false;
    wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
    return ctrl->IsExpanded(treeItem);
}

// Selection operations
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetSelection(wxd_TreeListCtrl_t* self) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return 0;
    wxTreeListItem selection = ctrl->GetSelection();
    return (wxd_Long_t)selection.GetID();
}

WXD_EXPORTED void wxd_TreeListCtrl_SelectItem(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        ctrl->Select(treeItem);
    }
}

WXD_EXPORTED void wxd_TreeListCtrl_UnselectAll(wxd_TreeListCtrl_t* self) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        ctrl->UnselectAll();
    }
}

// Checkbox operations
WXD_EXPORTED void wxd_TreeListCtrl_CheckItem(wxd_TreeListCtrl_t* self, wxd_Long_t item, int state) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        wxCheckBoxState checkState = wxCHK_UNCHECKED;
        switch (state) {
            case 1: checkState = wxCHK_CHECKED; break;
            case 2: checkState = wxCHK_UNDETERMINED; break;
            default: checkState = wxCHK_UNCHECKED; break;
        }
        ctrl->CheckItem(treeItem, checkState);
    }
}

WXD_EXPORTED int wxd_TreeListCtrl_GetCheckedState(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return 0;
    wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
    wxCheckBoxState state = ctrl->GetCheckedState(treeItem);
    switch (state) {
        case wxCHK_CHECKED: return 1;
        case wxCHK_UNDETERMINED: return 2;
        default: return 0;
    }
}

WXD_EXPORTED void wxd_TreeListCtrl_CheckItemRecursively(wxd_TreeListCtrl_t* self, wxd_Long_t item, int state) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        wxCheckBoxState checkState = wxCHK_UNCHECKED;
        switch (state) {
            case 1: checkState = wxCHK_CHECKED; break;
            case 2: checkState = wxCHK_UNDETERMINED; break;
            default: checkState = wxCHK_UNCHECKED; break;
        }
        ctrl->CheckItemRecursively(treeItem, checkState);
    }
}

WXD_EXPORTED void wxd_TreeListCtrl_UpdateItemParentState(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        ctrl->UpdateItemParentStateRecursively(treeItem);
    }
}

// Additional column operations
WXD_EXPORTED bool wxd_TreeListCtrl_DeleteColumn(wxd_TreeListCtrl_t* self, unsigned col) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return false;
    return ctrl->DeleteColumn(col);
}

WXD_EXPORTED void wxd_TreeListCtrl_ClearColumns(wxd_TreeListCtrl_t* self) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        ctrl->ClearColumns();
    }
}

WXD_EXPORTED int wxd_TreeListCtrl_WidthFor(wxd_TreeListCtrl_t* self, const char* text) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl || !text) return -1;
    return ctrl->WidthFor(wxString::FromUTF8(text));
}

// Item insertion methods
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_InsertItem(wxd_TreeListCtrl_t* self, wxd_Long_t parent, wxd_Long_t previous, const char* text) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl || !text) return 0;

    wxTreeListItem parentItem(reinterpret_cast<wxTreeListModelNode*>(parent));
    wxTreeListItem previousItem(reinterpret_cast<wxTreeListModelNode*>(previous));
    wxTreeListItem newItem = ctrl->InsertItem(parentItem, previousItem, wxString::FromUTF8(text));
    return (wxd_Long_t)newItem.GetID();
}

WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_PrependItem(wxd_TreeListCtrl_t* self, wxd_Long_t parent, const char* text) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl || !text) return 0;

    wxTreeListItem parentItem(reinterpret_cast<wxTreeListModelNode*>(parent));
    wxTreeListItem newItem = ctrl->PrependItem(parentItem, wxString::FromUTF8(text));
    return (wxd_Long_t)newItem.GetID();
}

// Tree navigation methods
WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetItemParent(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return 0;

    wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
    wxTreeListItem parent = ctrl->GetItemParent(treeItem);
    return (wxd_Long_t)parent.GetID();
}

WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetFirstChild(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return 0;

    wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
    wxTreeListItem child = ctrl->GetFirstChild(treeItem);
    return (wxd_Long_t)child.GetID();
}

WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetNextSibling(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return 0;

    wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
    wxTreeListItem sibling = ctrl->GetNextSibling(treeItem);
    return (wxd_Long_t)sibling.GetID();
}

WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetNextItem(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return 0;

    wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
    wxTreeListItem nextItem = ctrl->GetNextItem(treeItem);
    return (wxd_Long_t)nextItem.GetID();
}

WXD_EXPORTED wxd_Long_t wxd_TreeListCtrl_GetFirstItem(wxd_TreeListCtrl_t* self) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return 0;

    wxTreeListItem firstItem = ctrl->GetFirstItem();
    return (wxd_Long_t)firstItem.GetID();
}

// Item attribute methods

WXD_EXPORTED void wxd_TreeListCtrl_SetItemImage(wxd_TreeListCtrl_t* self, wxd_Long_t item, int closed, int opened) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        ctrl->SetItemImage(treeItem, closed, opened);
    }
}

// Multi-selection support
WXD_EXPORTED unsigned wxd_TreeListCtrl_GetSelections(wxd_TreeListCtrl_t* self, wxd_Long_t* selections, unsigned max_count) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl || !selections) return 0;

    wxTreeListItems items;
    unsigned count = ctrl->GetSelections(items);

    unsigned result_count = wxMin(count, max_count);
    for (unsigned i = 0; i < result_count; i++) {
        selections[i] = (wxd_Long_t)items[i].GetID();
    }

    return count;
}

WXD_EXPORTED void wxd_TreeListCtrl_Select(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        ctrl->Select(treeItem);
    }
}

WXD_EXPORTED void wxd_TreeListCtrl_Unselect(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        ctrl->Unselect(treeItem);
    }
}

WXD_EXPORTED bool wxd_TreeListCtrl_IsSelected(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return false;

    wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
    return ctrl->IsSelected(treeItem);
}

WXD_EXPORTED void wxd_TreeListCtrl_SelectAll(wxd_TreeListCtrl_t* self) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        ctrl->SelectAll();
    }
}

// Visibility methods
WXD_EXPORTED void wxd_TreeListCtrl_EnsureVisible(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        ctrl->EnsureVisible(treeItem);
    }
}

// Additional checkbox methods
WXD_EXPORTED void wxd_TreeListCtrl_UncheckItem(wxd_TreeListCtrl_t* self, wxd_Long_t item) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
        ctrl->UncheckItem(treeItem);
    }
}

WXD_EXPORTED bool wxd_TreeListCtrl_AreAllChildrenInState(wxd_TreeListCtrl_t* self, wxd_Long_t item, int state) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return false;

    wxTreeListItem treeItem(reinterpret_cast<wxTreeListModelNode*>(item));
    wxCheckBoxState checkState = static_cast<wxCheckBoxState>(state);
    return ctrl->AreAllChildrenInState(treeItem, checkState);
}

// Sorting methods
WXD_EXPORTED void wxd_TreeListCtrl_SetSortColumn(wxd_TreeListCtrl_t* self, unsigned col, bool ascending) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (ctrl) {
        ctrl->SetSortColumn(col, ascending);
    }
}

WXD_EXPORTED bool wxd_TreeListCtrl_GetSortColumn(wxd_TreeListCtrl_t* self, unsigned* col, bool* ascending) {
    wxTreeListCtrl* ctrl = (wxTreeListCtrl*)self;
    if (!ctrl) return false;

    return ctrl->GetSortColumn(col, ascending);
}

} // extern "C"