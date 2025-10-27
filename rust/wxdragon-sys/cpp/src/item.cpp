#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/dataview.h> // For wxDataViewItem

WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewItem_Clone(const wxd_DataViewItem_t* item)
{
    if (!item)
        return reinterpret_cast<const wxd_DataViewItem_t*>(new wxDataViewItem());
    const wxDataViewItem* inner = reinterpret_cast<const wxDataViewItem*>(item);
    wxDataViewItem* heap_item = new wxDataViewItem(*inner);
    return reinterpret_cast<const wxd_DataViewItem_t*>(heap_item);
}

WXD_EXPORTED bool
wxd_DataViewItem_IsOk(const wxd_DataViewItem_t* item)
{
    if (!item)
        return false;
    const wxDataViewItem* inner = reinterpret_cast<const wxDataViewItem*>(item);
    return inner->IsOk();
}

WXD_EXPORTED const void*
wxd_DataViewItem_GetID(const wxd_DataViewItem_t* item)
{
    if (!item)
        return nullptr;
    const wxDataViewItem* inner = reinterpret_cast<const wxDataViewItem*>(item);
    return inner->GetID();
}

WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewItem_CreateFromID(const void* id)
{
    wxDataViewItem* heap_item = new wxDataViewItem(const_cast<void*>(id));
    return reinterpret_cast<const wxd_DataViewItem_t*>(heap_item);
}

WXD_EXPORTED void
wxd_DataViewItem_Release(const wxd_DataViewItem_t* item)
{
    if (!item)
        return;
    delete reinterpret_cast<const wxDataViewItem*>(item);
}
