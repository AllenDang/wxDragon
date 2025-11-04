#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"

#include "wx/dataview.h"
#include <vector>

// Declare the Rust-side drop function that knows how to free Box-allocated
// wxd_DataViewTreeModel_Callbacks structs specifically.
extern "C" void
wxd_Drop_Rust_DataViewTreeModelCallbacks(wxd_DataViewTreeModel_Callbacks* ptr);

class Wxd_Callbacks_DataViewTreeModel : public wxDataViewModel {
public:
    Wxd_Callbacks_DataViewTreeModel(const wxd_DataViewTreeModel_Callbacks* cb)
    {
        m_cb = cb;
        WXD_LOG_TRACEF("Wxd_Callbacks_DataViewTreeModel created with pointer %p", this);
    }

    virtual ~Wxd_Callbacks_DataViewTreeModel()
    {
        WXD_LOG_TRACEF("Wxd_Callbacks_DataViewTreeModel destroyed with pointer %p", this);
        if (m_cb) {
            // Call into Rust to reclaim and drop the callback struct
            wxd_Drop_Rust_DataViewTreeModelCallbacks(
                const_cast<wxd_DataViewTreeModel_Callbacks*>(m_cb));
        }
    }

    // Implement required virtuals
    unsigned int
    GetChildren(const wxDataViewItem& parent, wxDataViewItemArray& array) const override
    {
        if (!m_cb || !m_cb->get_children)
            return 0;

        void** items = nullptr;
        int count = 0;
        m_cb->get_children(m_cb->userdata, (void*)parent.GetID(), &items, &count);
        if (items && count > 0) {
            for (int i = 0; i < count; ++i) {
                array.push_back(wxDataViewItem(items[i]));
            }
            if (m_cb->free_children)
                m_cb->free_children(items, count);
            return array.size();
        }
        return 0;
    }

    wxDataViewItem
    GetParent(const wxDataViewItem& item) const override
    {
        if (!m_cb || !m_cb->get_parent)
            return wxDataViewItem(nullptr);
        void* p = m_cb->get_parent(m_cb->userdata, (void*)item.GetID());
        return wxDataViewItem(p);
    }

    bool
    IsContainer(const wxDataViewItem& item) const override
    {
        if (!m_cb || !m_cb->is_container)
            return false;
        return m_cb->is_container(m_cb->userdata, (void*)item.GetID());
    }

    void
    GetValue(wxVariant& variant, const wxDataViewItem& item, unsigned int col) const override
    {
        if (!m_cb || !m_cb->get_value)
            return;

        // Ask Rust to populate a C-compatible variant structure
        wxd_Variant_t* rust_variant_data =
            m_cb->get_value(m_cb->userdata, (void*)item.GetID(), col);
        if (!rust_variant_data)
            return;
        // Convert wxd_Variant_t to wxVariant
        variant = *reinterpret_cast<wxVariant*>(rust_variant_data);
        delete reinterpret_cast<wxVariant*>(rust_variant_data);
    }

    bool
    SetValue(const wxVariant& variant, const wxDataViewItem& item, unsigned int col) override
    {
        if (!m_cb || !m_cb->set_value)
            return false;

        const wxd_Variant_t* rust_variant = reinterpret_cast<const wxd_Variant_t*>(&variant);

        bool result = m_cb->set_value(m_cb->userdata, (void*)item.GetID(), col, rust_variant);

        if (result) {
            this->ValueChanged(item, col);
        }
        return result;
    }

    bool
    IsEnabled(const wxDataViewItem& item, unsigned int col) const override
    {
        if (!m_cb || !m_cb->is_enabled)
            return true;
        return m_cb->is_enabled(m_cb->userdata, (void*)item.GetID(), col);
    }

    int
    Compare(const wxDataViewItem& item1, const wxDataViewItem& item2, unsigned int column,
            bool ascending) const override
    {
        if (!m_cb || !m_cb->compare)
            return wxDataViewModel::Compare(item1, item2, column, ascending);
        return m_cb->compare(m_cb->userdata, (void*)item1.GetID(), (void*)item2.GetID(), column,
                             ascending);
    }

private:
    const wxd_DataViewTreeModel_Callbacks* m_cb;
};

extern "C" wxd_DataViewModel_t*
wxd_DataViewTreeModel_CreateWithCallbacks(const wxd_DataViewTreeModel_Callbacks* cb)
{
    if (!cb)
        return nullptr;
    Wxd_Callbacks_DataViewTreeModel* model = new Wxd_Callbacks_DataViewTreeModel(cb);
    return reinterpret_cast<wxd_DataViewModel_t*>(model);
}

extern "C" void
wxd_DataViewTreeModel_ItemValueChanged(wxd_DataViewModel_t* model, void* item, unsigned int col)
{
    if (!model)
        return;
    auto* m = reinterpret_cast<Wxd_Callbacks_DataViewTreeModel*>(model);
    m->ValueChanged(wxDataViewItem(item), col);
}

extern "C" void
wxd_DataViewTreeModel_ItemChanged(wxd_DataViewModel_t* model, void* item)
{
    if (!model)
        return;
    auto* m = reinterpret_cast<Wxd_Callbacks_DataViewTreeModel*>(model);
    m->ItemChanged(wxDataViewItem(item));
}

extern "C" void
wxd_DataViewTreeModel_ItemAdded(wxd_DataViewModel_t* model, void* parent, void* item)
{
    if (!model)
        return;
    Wxd_Callbacks_DataViewTreeModel* m = reinterpret_cast<Wxd_Callbacks_DataViewTreeModel*>(model);
    m->ItemAdded(wxDataViewItem(parent), wxDataViewItem(item));
}

extern "C" void
wxd_DataViewTreeModel_ItemDeleted(wxd_DataViewModel_t* model, void* parent, void* item)
{
    if (!model)
        return;
    Wxd_Callbacks_DataViewTreeModel* m = reinterpret_cast<Wxd_Callbacks_DataViewTreeModel*>(model);
    m->ItemDeleted(wxDataViewItem(parent), wxDataViewItem(item));
}

extern "C" void
wxd_DataViewTreeModel_ItemsAdded(wxd_DataViewModel_t* model, void* parent, const void* const* items,
                                 size_t count)
{
    if (!model || !items)
        return;
    Wxd_Callbacks_DataViewTreeModel* m = reinterpret_cast<Wxd_Callbacks_DataViewTreeModel*>(model);

    wxDataViewItemArray array;
    for (size_t i = 0; i < count; ++i) {
        array.push_back(wxDataViewItem((void*)items[i]));
    }
    m->ItemsAdded(wxDataViewItem(parent), array);
}

// ItemsDeleted
extern "C" void
wxd_DataViewTreeModel_ItemsDeleted(wxd_DataViewModel_t* model, void* parent,
                                   const void* const* items, size_t count)
{
    if (!model || !items)
        return;
    Wxd_Callbacks_DataViewTreeModel* m = reinterpret_cast<Wxd_Callbacks_DataViewTreeModel*>(model);

    wxDataViewItemArray array;
    for (size_t i = 0; i < count; ++i) {
        array.push_back(wxDataViewItem((void*)items[i]));
    }
    m->ItemsDeleted(wxDataViewItem(parent), array);
}

// ItemsChanged
extern "C" void
wxd_DataViewTreeModel_ItemsChanged(wxd_DataViewModel_t* model, const void* const* items,
                                   size_t count)
{
    if (!model || !items)
        return;
    Wxd_Callbacks_DataViewTreeModel* m = reinterpret_cast<Wxd_Callbacks_DataViewTreeModel*>(model);

    wxDataViewItemArray array;
    for (size_t i = 0; i < count; ++i) {
        array.push_back(wxDataViewItem((void*)items[i]));
    }
    m->ItemsChanged(array);
}

// Cleared
extern "C" void
wxd_DataViewTreeModel_Cleared(wxd_DataViewModel_t* model)
{
    if (!model)
        return;
    Wxd_Callbacks_DataViewTreeModel* m = reinterpret_cast<Wxd_Callbacks_DataViewTreeModel*>(model);
    m->Cleared();
}
