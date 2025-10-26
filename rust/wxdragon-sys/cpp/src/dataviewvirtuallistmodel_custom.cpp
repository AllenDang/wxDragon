#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/dataview.h>
#include <wx/log.h>
#include <wx/object.h> // For wxIsKindOf macro
#include "wxd_utils.h"

// Forward declaration for Rust function that properly handles Box::from_raw()
extern "C" void
wxd_Drop_Rust_CustomModelCallbacks(void* ptr);

// Define a custom implementation of wxDataViewVirtualListModel that uses callbacks
class WxdCustomDataViewVirtualListModel : public wxDataViewVirtualListModel {
public:
    WxdCustomDataViewVirtualListModel(unsigned int initial_size, void* userdata,
                                      wxd_dataview_model_get_value_callback get_value,
                                      wxd_dataview_model_set_value_callback set_value,
                                      wxd_dataview_model_get_attr_callback get_attr,
                                      wxd_dataview_model_is_enabled_callback is_enabled)
        : wxDataViewVirtualListModel(initial_size), m_userdata(userdata), m_get_value(get_value),
          m_set_value(set_value), m_get_attr(get_attr), m_is_enabled(is_enabled)
    {
        WXD_LOG_TRACEF("WxdCustomDataViewVirtualListModel created with pointer %p", this);
    }

    // Destructor to clean up registry and callback data
    ~WxdCustomDataViewVirtualListModel()
    {
        WXD_LOG_TRACEF("WxdCustomDataViewVirtualListModel destroyed with pointer %p", this);
        // Clean up the Rust-allocated callback data
        if (m_userdata) {
            wxd_Drop_Rust_CustomModelCallbacks(m_userdata);
        }
    }

    // Implementation of the pure virtual methods
    virtual void
    GetValueByRow(wxVariant& variant, unsigned int row, unsigned int col) const override
    {
        if (m_get_value) {
            wxd_Variant_t* rust_variant_data =
                m_get_value(m_userdata, static_cast<uint64_t>(row), static_cast<uint64_t>(col));
            if (rust_variant_data) {
                // Convert wxd_Variant_t to wxVariant
                variant = *reinterpret_cast<wxVariant*>(rust_variant_data);
                delete reinterpret_cast<wxVariant*>(rust_variant_data);
            }
            else {
                // Handle null return from Rust
                variant.Clear();
            }
        }
        else {
            // Default behavior if no callback is provided
            variant = wxString::Format("Row %d, Col %d", row, col);
        }
    }

    virtual bool
    SetValueByRow(const wxVariant& variant, unsigned int row, unsigned int col) override
    {
        if (m_set_value) {
            const wxd_Variant_t* rust_variant = reinterpret_cast<const wxd_Variant_t*>(&variant);
            return m_set_value(m_userdata, rust_variant, static_cast<uint64_t>(row),
                               static_cast<uint64_t>(col));
        }
        return false;
    }

    virtual bool
    GetAttrByRow(unsigned int row, unsigned int col, wxDataViewItemAttr& attr) const override
    {
        if (m_get_attr) {
            wxd_DataViewItemAttr_t rust_attr;
            bool has_attr = m_get_attr(m_userdata, static_cast<uint64_t>(row),
                                       static_cast<uint64_t>(col), &rust_attr);

            if (has_attr) {
                if (rust_attr.has_text_colour) {
                    attr.SetColour(wxColour(rust_attr.text_colour_red, rust_attr.text_colour_green,
                                            rust_attr.text_colour_blue,
                                            rust_attr.text_colour_alpha));
                }

                if (rust_attr.has_bg_colour) {
                    attr.SetBackgroundColour(
                        wxColour(rust_attr.bg_colour_red, rust_attr.bg_colour_green,
                                 rust_attr.bg_colour_blue, rust_attr.bg_colour_alpha));
                }

                if (rust_attr.bold) {
                    attr.SetBold(true);
                }

                if (rust_attr.italic) {
                    attr.SetItalic(true);
                }

                return true;
            }
        }
        return false;
    }

    virtual bool
    IsEnabledByRow(unsigned int row, unsigned int col) const override
    {
        if (m_is_enabled) {
            return m_is_enabled(m_userdata, static_cast<uint64_t>(row), static_cast<uint64_t>(col));
        }
        return true;
    }

private:
    void* m_userdata;
    wxd_dataview_model_get_value_callback m_get_value;
    wxd_dataview_model_set_value_callback m_set_value;
    wxd_dataview_model_get_attr_callback m_get_attr;
    wxd_dataview_model_is_enabled_callback m_is_enabled;
};

// @brief Creates a new custom virtual list model with callbacks, the returned model has ref count 1,
// the caller is responsible for releasing it when done with wxd_DataViewModel_Release.
extern "C" wxd_DataViewModel_t*
wxd_DataViewVirtualListModel_CreateWithCallbacks(
    uint64_t initial_size, void* userdata, wxd_dataview_model_get_value_callback get_value_callback,
    wxd_dataview_model_set_value_callback set_value_callback,
    wxd_dataview_model_get_attr_callback get_attr_callback,
    wxd_dataview_model_is_enabled_callback is_enabled_callback)
{
    if (!userdata || !get_value_callback) {
        return nullptr;
    }

    // Create model and ensure it stays alive, the reference count now is 1
    WxdCustomDataViewVirtualListModel* model =
        new WxdCustomDataViewVirtualListModel(static_cast<unsigned int>(initial_size), userdata,
                                              get_value_callback, set_value_callback,
                                              get_attr_callback, is_enabled_callback);

    return reinterpret_cast<wxd_DataViewModel_t*>(model);
}
