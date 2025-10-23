#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"

#include "wx/dataview.h"
#include <vector>

// Declare the Rust-side drop function that knows how to free Box-allocated
// wxd_DataViewTreeModel_Callbacks structs specifically.
extern "C" void wxd_Drop_Rust_DataViewTreeModelCallbacks(wxd_DataViewTreeModel_Callbacks* ptr);

class Wxd_Callbacks_DataViewTreeModel : public wxDataViewModel
{
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
            wxd_Drop_Rust_DataViewTreeModelCallbacks(const_cast<wxd_DataViewTreeModel_Callbacks*>(m_cb));
        }
    }

    // Implement required virtuals
    unsigned int GetChildren(const wxDataViewItem &parent, wxDataViewItemArray &array) const override
    {
        if (!m_cb || !m_cb->get_children)
            return 0;

        void** items = nullptr;
        int count = 0;
        m_cb->get_children(m_cb->userdata, (void*)parent.GetID(), &items, &count);
        if (items && count > 0)
        {
            for (int i = 0; i < count; ++i)
            {
                array.push_back(wxDataViewItem(items[i]));
            }
            if (m_cb->free_children)
                m_cb->free_children(items, count);
            return array.size();
        }
        return 0;
    }

    wxDataViewItem GetParent(const wxDataViewItem &item) const override
    {
        if (!m_cb || !m_cb->get_parent)
            return wxDataViewItem(nullptr);
        void* p = m_cb->get_parent(m_cb->userdata, (void*)item.GetID());
        return wxDataViewItem(p);
    }

    bool IsContainer(const wxDataViewItem &item) const override
    {
        if (!m_cb || !m_cb->is_container)
            return false;
        return m_cb->is_container(m_cb->userdata, (void*)item.GetID());
    }

    void GetValue(wxVariant &variant, const wxDataViewItem &item, unsigned int col) const override
    {
        if (!m_cb || !m_cb->get_value)
            return;

        // Ask Rust to populate a C-compatible variant structure
        wxd_Variant_t rust_variant_data = {};
        m_cb->get_value(m_cb->userdata, (void*)item.GetID(), col, &rust_variant_data);

        // Convert the wxd_Variant_t into a wxVariant for wxWidgets
        switch (rust_variant_data.type) {
            case WXD_VARIANT_TYPE_STRING:
                if (rust_variant_data.data.string_val) {
                    variant = wxVariant(wxString::FromUTF8(rust_variant_data.data.string_val));
                    // Free the Rust-allocated string
                    wxd_Variant_Free_Rust_String(rust_variant_data.data.string_val);
                    rust_variant_data.data.string_val = nullptr;
                }
                break;

            case WXD_VARIANT_TYPE_BOOL:
                variant = wxVariant(rust_variant_data.data.bool_val != 0);
                break;

            case WXD_VARIANT_TYPE_INT32:
                variant = wxVariant(static_cast<long>(rust_variant_data.data.int32_val));
                break;

            case WXD_VARIANT_TYPE_INT64:
                // Use wxLongLong wrapper to avoid ambiguous overload resolution for wxVariant
                variant = wxVariant(wxLongLong(rust_variant_data.data.int64_val));
                break;

            case WXD_VARIANT_TYPE_DOUBLE:
                variant = wxVariant(rust_variant_data.data.double_val);
                break;

            case WXD_VARIANT_TYPE_DATETIME: {
                wxDateTime dt;
                dt.Set(
                    rust_variant_data.data.datetime_val.day,
                    static_cast<wxDateTime::Month>(rust_variant_data.data.datetime_val.month),
                    rust_variant_data.data.datetime_val.year,
                    rust_variant_data.data.datetime_val.hour,
                    rust_variant_data.data.datetime_val.minute,
                    rust_variant_data.data.datetime_val.second
                );
                variant = dt;
                break;
            }

            case WXD_VARIANT_TYPE_BITMAP_RUST_BORROWED:
                if (rust_variant_data.data.bitmap_val) {
                    wxBitmap* bmp = reinterpret_cast<wxBitmap*>(rust_variant_data.data.bitmap_val);
                    if (bmp && bmp->IsOk()) {
                        try {
                            wxBitmap copy(*bmp);
                            variant << copy;
                        } catch (...) {
                            // fallback: empty bitmap
                            wxBitmap fb(16,16); variant << fb;
                        }
                    }
                }
                break;

            case WXD_VARIANT_TYPE_BITMAP:
                if (rust_variant_data.data.bitmap_val) {
                    wxBitmap* bmp = reinterpret_cast<wxBitmap*>(rust_variant_data.data.bitmap_val);
                    if (bmp && bmp->IsOk()) {
                        try {
                            wxBitmap copy(*bmp);
                            variant << copy;
                        } catch (...) { }
                    }
                    // If this was a cloned bitmap on C++ heap, free it now
                    wxd_Bitmap_Destroy(reinterpret_cast<wxd_Bitmap_t*>(rust_variant_data.data.bitmap_val));
                    rust_variant_data.data.bitmap_val = nullptr;
                }
                break;

            default:
                // Unknown/invalid types: leave variant unchanged
                break;
        }
    }

    bool SetValue(const wxVariant &variant, const wxDataViewItem &item, unsigned int col) override
    {
        if (!m_cb || !m_cb->set_value)
            return false;

        wxd_Variant_t rust_variant = {};

        // Convert wxVariant into wxd_Variant_t
        wxString type_name = variant.GetType();
        if (type_name == "bool") {
            rust_variant.type = WXD_VARIANT_TYPE_BOOL;
            rust_variant.data.bool_val = variant.GetBool();
        } else if (type_name == "long") {
            rust_variant.type = WXD_VARIANT_TYPE_INT32;
            rust_variant.data.int32_val = static_cast<int32_t>(variant.GetLong());
        } else if (type_name == "longlong") {
            rust_variant.type = WXD_VARIANT_TYPE_INT64;
            rust_variant.data.int64_val = static_cast<int64_t>(variant.GetLongLong().GetValue());
        } else if (type_name == "double") {
            rust_variant.type = WXD_VARIANT_TYPE_DOUBLE;
            rust_variant.data.double_val = variant.GetDouble();
        } else if (type_name == "string") {
            rust_variant.type = WXD_VARIANT_TYPE_STRING;
            std::string utf8 = variant.GetString().ToUTF8().data();
            char* str = static_cast<char*>(calloc(utf8.length() + 1, sizeof(char)));
            if (str) {
                strcpy(str, utf8.c_str());
                rust_variant.data.string_val = str;
            } else {
                rust_variant.data.string_val = nullptr;
            }
        } else {
            rust_variant.type = WXD_VARIANT_TYPE_INVALID;
        }

        bool result = m_cb->set_value(m_cb->userdata, (void*)item.GetID(), col, &rust_variant);

        // Clean up any allocated memory in the variant
        if (rust_variant.type == WXD_VARIANT_TYPE_STRING && rust_variant.data.string_val) {
            free(rust_variant.data.string_val);
        }

        if (result) {
            this->ValueChanged(item, col);
        }
        return result;
    }

    bool IsEnabled(const wxDataViewItem &item, unsigned int col) const override
    {
        if (!m_cb || !m_cb->is_enabled)
            return true;
        return m_cb->is_enabled(m_cb->userdata, (void*)item.GetID(), col);
    }

    int Compare(const wxDataViewItem &item1, const wxDataViewItem &item2, unsigned int column, bool ascending) const override
    {
        if (!m_cb || !m_cb->compare)
            return wxDataViewModel::Compare(item1, item2, column, ascending);
        return m_cb->compare(m_cb->userdata, (void*)item1.GetID(), (void*)item2.GetID(), column, ascending);
    }

private:
    const wxd_DataViewTreeModel_Callbacks* m_cb;
};

extern "C" wxd_DataViewModel_t* wxd_DataViewTreeModel_CreateWithCallbacks(const wxd_DataViewTreeModel_Callbacks* cb)
{
    if (!cb) return nullptr;
    Wxd_Callbacks_DataViewTreeModel* model = new Wxd_Callbacks_DataViewTreeModel(cb);
    return reinterpret_cast<wxd_DataViewModel_t*>(model);
}
