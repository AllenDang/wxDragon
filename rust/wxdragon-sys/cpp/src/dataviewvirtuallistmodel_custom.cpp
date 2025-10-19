#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/dataview.h>
#include <wx/log.h>
#include <wx/object.h>  // For wxIsKindOf macro
#include "wxd_utils.h"

// Forward declaration for Rust function that properly handles Box::from_raw()
extern "C" void wxd_Drop_Rust_CustomModelCallbacks(void* ptr);

// Define a custom implementation of wxDataViewVirtualListModel that uses callbacks
class WxdCustomDataViewVirtualListModel : public wxDataViewVirtualListModel {
public:
    typedef void (*GetValueCallback)(void* userdata, uint64_t row, uint64_t col, wxd_Variant_t* variant);
    typedef bool (*SetValueCallback)(void* userdata, const wxd_Variant_t* variant, uint64_t row, uint64_t col);
    typedef bool (*GetAttrCallback)(void* userdata, uint64_t row, uint64_t col, wxd_DataViewItemAttr_t* attr);
    typedef bool (*IsEnabledCallback)(void* userdata, uint64_t row, uint64_t col);

    WxdCustomDataViewVirtualListModel(unsigned int initial_size, 
                                     void* userdata,
                                     GetValueCallback get_value,
                                     SetValueCallback set_value,
                                     GetAttrCallback get_attr,
                                     IsEnabledCallback is_enabled)
        : wxDataViewVirtualListModel(initial_size),
          m_userdata(userdata),
          m_get_value(get_value),
          m_set_value(set_value),
          m_get_attr(get_attr),
          m_is_enabled(is_enabled)
    {
        wxLogDebug("WxdCustomDataViewVirtualListModel created with initial size %u", initial_size);
    }
    
    // Destructor to clean up registry and callback data
    ~WxdCustomDataViewVirtualListModel() {
        wxLogDebug("WxdCustomDataViewVirtualListModel destroyed");
        // Clean up the Rust-allocated callback data
        if (m_userdata) {
            wxd_Drop_Rust_CustomModelCallbacks(m_userdata);
        }
    }

    // Implementation of the pure virtual methods
    virtual void GetValueByRow(wxVariant &variant, unsigned int row, unsigned int col) const override {
        wxd_Variant_t rust_variant_data = {}; // Initialize to zeros/nulls
        bool destroy_cloned_bitmap_in_rust_variant = false; // Default to false

        if (m_get_value) {
            m_get_value(m_userdata, static_cast<uint64_t>(row), static_cast<uint64_t>(col), &rust_variant_data);
            // Only set destroy flag if it's the OLD WXD_VARIANT_TYPE_BITMAP type
            if (rust_variant_data.type == WXD_VARIANT_TYPE_BITMAP && rust_variant_data.data.bitmap_val != nullptr) {
                destroy_cloned_bitmap_in_rust_variant = true;
            }
            // For WXD_VARIANT_TYPE_BITMAP_RUST_BORROWED, destroy_cloned_bitmap_in_rust_variant remains false.
        } else {
            rust_variant_data.type = WXD_VARIANT_TYPE_INVALID;
        }
                
        switch(col) {
            case 0: 
            case 1: 
            case 2: 
            case 8: 
                if (rust_variant_data.type == WXD_VARIANT_TYPE_STRING && rust_variant_data.data.string_val) {
                    variant = wxVariant(wxString::FromUTF8(rust_variant_data.data.string_val));
                    wxd_Variant_Free_Rust_String(rust_variant_data.data.string_val);
                    rust_variant_data.data.string_val = nullptr;
                } else {
                    variant = wxString::Format("Row %d, Col %d", row, col);
                }
                break;
                
            case 3: 
                if (rust_variant_data.type == WXD_VARIANT_TYPE_BOOL) {
                    variant = wxVariant(rust_variant_data.data.bool_val);
                } else {
                    variant = wxVariant(false);
                }
                break;
                
            case 4: 
                if (rust_variant_data.type == WXD_VARIANT_TYPE_INT32) {
                    variant = wxVariant(static_cast<long>(rust_variant_data.data.int32_val));
                } else {
                    variant = wxVariant(static_cast<long>(0));
                }
                break;
                
            case 5: 
                if (rust_variant_data.type == WXD_VARIANT_TYPE_BITMAP_RUST_BORROWED) {
                    void* borrowed_bmp_ptr_from_rust = rust_variant_data.data.bitmap_val;
                    if (borrowed_bmp_ptr_from_rust != nullptr) {
                        wxBitmap* casted_bmp = reinterpret_cast<wxBitmap*>(borrowed_bmp_ptr_from_rust);
                        if (casted_bmp->IsOk()) {
                            try {
                                wxBitmap final_copy(*casted_bmp); // Make a copy for the wxVariant
                                if (final_copy.IsOk()) {
                                    variant << final_copy; 
                                } else {
                                    // Fallback if final_copy is not OK
                                    wxBitmap fallback_bmp_on_error(16, 16); /* Create a visible fallback */
                                    wxMemoryDC dc(fallback_bmp_on_error); 
                                    dc.SetBackground(*wxRED_BRUSH); dc.Clear(); 
                                    variant << fallback_bmp_on_error;
                                }
                            } catch (...) {
                                wxLogError("GetValueByRow: Exception during wxBitmap copy from borrowed ptr for WXD_VARIANT_TYPE_BITMAP_RUST_BORROWED.");
                                wxBitmap fallback_bmp_on_exception(16, 16); /* Create a visible fallback */
                                wxMemoryDC dc(fallback_bmp_on_exception); 
                                dc.SetBackground(*wxBLUE_BRUSH); dc.Clear(); 
                                variant << fallback_bmp_on_exception;
                            }
                        } else {
                             // borrowed bitmap pointer is not IsOk()
                             wxBitmap fallback_bmp_not_ok(16, 16); /* Create a visible fallback */
                             wxMemoryDC dc(fallback_bmp_not_ok); 
                             dc.SetBackground(*wxGREEN_BRUSH); dc.Clear(); 
                             variant << fallback_bmp_not_ok;
                            }
                    } else {
                        // borrowed_bmp_ptr_from_rust was null
                        wxBitmap fallback_bmp_null_ptr(16, 16); /* Create a visible fallback */
                        wxMemoryDC dc(fallback_bmp_null_ptr); 
                        dc.SetBackground(*wxYELLOW_BRUSH); dc.Clear(); 
                        variant << fallback_bmp_null_ptr;
                    }
                } else if (rust_variant_data.type == WXD_VARIANT_TYPE_BITMAP) { // Handle OLD type (should ideally not be used by Rust anymore for get_value)
                    void* cloned_bmp_ptr_from_rust = rust_variant_data.data.bitmap_val;
                    if (cloned_bmp_ptr_from_rust != nullptr) {
                        wxBitmap* casted_bmp = reinterpret_cast<wxBitmap*>(cloned_bmp_ptr_from_rust);
                        if (casted_bmp->IsOk()) {
                           try {
                                wxBitmap final_copy(*casted_bmp); // Make a copy for the wxVariant
                                if (final_copy.IsOk()) {
                                    variant << final_copy;
                                } else { /* Fallback for old type if copy fails */ }
                            } catch (...) { /* Fallback for old type on exception */ }
                        } else { /* Fallback for old type if not IsOk */ }
                    } else { /* Fallback for old type if null ptr */ }
                     // destroy_cloned_bitmap_in_rust_variant is true for this path, so it will be destroyed after switch.
                } else {
                    // Fallback if Rust didn't send any known bitmap variant type
                    wxBitmap default_bmp(16, 16); /* Create a black/default fallback */
                        wxMemoryDC dc(default_bmp);
                    dc.SetBackground(*wxBLACK_BRUSH); dc.Clear(); 
                    variant << default_bmp;
                }
                break;
                
            case 6: 
                if (rust_variant_data.type == WXD_VARIANT_TYPE_DATETIME) {
                    // wxLogDebug("C++ GetValueByRow (col 6) - Rust DateTime components: y=%d, m=%d, d=%d, h=%d, min=%d, s=%d", 
                    //     rust_variant_data.data.datetime_val.year,
                    //     rust_variant_data.data.datetime_val.month,
                    //     rust_variant_data.data.datetime_val.day,
                    //     rust_variant_data.data.datetime_val.hour,
                    //     rust_variant_data.data.datetime_val.minute,
                    //     rust_variant_data.data.datetime_val.second);

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
                } else {
                    wxDateTime dt = wxDateTime::Now();
                    variant = dt;
                }
                break;
                
            case 7: 
                if (rust_variant_data.type == WXD_VARIANT_TYPE_INT32) {
                    variant = wxVariant(static_cast<long>(rust_variant_data.data.int32_val));
                } else {
                    variant = wxVariant(static_cast<long>(0));
                }
                break;
                
            default:
                if (rust_variant_data.type == WXD_VARIANT_TYPE_STRING && rust_variant_data.data.string_val) {
                    variant = wxVariant(wxString::FromUTF8(rust_variant_data.data.string_val));
                    wxd_Variant_Free_Rust_String(rust_variant_data.data.string_val);
                    rust_variant_data.data.string_val = nullptr;
                } else if (rust_variant_data.type != WXD_VARIANT_TYPE_INVALID) {
                     variant = wxString::Format("Default for col %d", col);
                }
                break;
        }

        if (destroy_cloned_bitmap_in_rust_variant) {
            wxd_Bitmap_Destroy(reinterpret_cast<wxd_Bitmap_t*>(rust_variant_data.data.bitmap_val));
            rust_variant_data.data.bitmap_val = nullptr; 
        }
    }
    
    virtual bool SetValueByRow(const wxVariant &variant, unsigned int row, unsigned int col) override {
        if (m_set_value) {
            wxd_Variant_t rust_variant;
            
            // Convert wxVariant to wxd_Variant_t
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
                char* str = static_cast<char*>(malloc(utf8.length() + 1));
                if (str) {
                    strcpy(str, utf8.c_str());
                    rust_variant.data.string_val = str;
                } else {
                    rust_variant.data.string_val = nullptr;
                }
            } else {
                // Unsupported type
                rust_variant.type = WXD_VARIANT_TYPE_INVALID;
            }
            
            bool result = m_set_value(m_userdata, &rust_variant, static_cast<uint64_t>(row), static_cast<uint64_t>(col));
            
            // Clean up any allocated memory in the variant
            if (rust_variant.type == WXD_VARIANT_TYPE_STRING && rust_variant.data.string_val) {
                free(rust_variant.data.string_val);
            }
            
            return result;
        }
        return false;
    }
    
    virtual bool GetAttrByRow(unsigned int row, unsigned int col, wxDataViewItemAttr &attr) const override {
        if (m_get_attr) {
            wxd_DataViewItemAttr_t rust_attr;
            bool has_attr = m_get_attr(m_userdata, static_cast<uint64_t>(row), static_cast<uint64_t>(col), &rust_attr);
            
            if (has_attr) {
                if (rust_attr.has_text_colour) {
                    attr.SetColour(wxColour(
                        rust_attr.text_colour_red,
                        rust_attr.text_colour_green,
                        rust_attr.text_colour_blue,
                        rust_attr.text_colour_alpha
                    ));
                }
                
                if (rust_attr.has_bg_colour) {
                    attr.SetBackgroundColour(wxColour(
                        rust_attr.bg_colour_red,
                        rust_attr.bg_colour_green,
                        rust_attr.bg_colour_blue,
                        rust_attr.bg_colour_alpha
                    ));
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
    
    virtual bool IsEnabledByRow(unsigned int row, unsigned int col) const override {
        if (m_is_enabled) {
            return m_is_enabled(m_userdata, static_cast<uint64_t>(row), static_cast<uint64_t>(col));
        }
        return true;
    }

private:
    void* m_userdata;
    GetValueCallback m_get_value;
    SetValueCallback m_set_value;
    GetAttrCallback m_get_attr;
    IsEnabledCallback m_is_enabled;
};

// @brief Creates a new custom virtual list model with callbacks, the returned model has ref count 1,
// the caller is responsible for releasing it when done with wxd_DataViewModel_Release.
extern "C" wxd_DataViewModel_t* wxd_DataViewVirtualListModel_CreateWithCallbacks(
    uint64_t initial_size,
    void* userdata,
    void (*get_value_callback)(void* userdata, uint64_t row, uint64_t col, wxd_Variant_t* variant),
    bool (*set_value_callback)(void* userdata, const wxd_Variant_t* variant, uint64_t row, uint64_t col),
    bool (*get_attr_callback)(void* userdata, uint64_t row, uint64_t col, wxd_DataViewItemAttr_t* attr),
    bool (*is_enabled_callback)(void* userdata, uint64_t row, uint64_t col)
) {
    if (!userdata || !get_value_callback) {
        return nullptr;
    }
    
    // Create model and ensure it stays alive, the reference count now is 1
    WxdCustomDataViewVirtualListModel* model = new WxdCustomDataViewVirtualListModel(
        static_cast<unsigned int>(initial_size),
        userdata,
        get_value_callback,
        set_value_callback,
        get_attr_callback,
        is_enabled_callback
    );

    return reinterpret_cast<wxd_DataViewModel_t*>(model);
}
