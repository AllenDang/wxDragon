#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "../src/wxd_utils.h"
#include <wx/dataview.h>
#include <wx/string.h> // For wxString methods
#include <wx/tokenzr.h> // For wxStringTokenizer
#include <wx/bitmap.h> // For wxBitmap
#include <wx/datetime.h> // For wxDateTime
#include <wx/variant.h>
#include <cstring>
#include <wx/log.h>  // For wxLogDebug and wxLogError

// Forward declarations
void drop_rust_custom_renderer_callbacks(void* ptr);

// Global storage for custom renderer callbacks keyed by unique renderer ID
struct CustomRendererCallbacks {
    void* closure_ptr = nullptr;
    void* get_size_trampoline = nullptr;
    void* render_trampoline = nullptr;
    void* set_value_trampoline = nullptr;
    void* get_value_trampoline = nullptr;
    void* has_editor_trampoline = nullptr;
    void* create_editor_trampoline = nullptr;
    void* get_value_from_editor_trampoline = nullptr;
    void* activate_cell_trampoline = nullptr;
};

// Use renderer_id as the primary key instead of (dataview_id, column_index)
typedef int RendererKey;

// Hash function for RendererKey (now just int)
struct RendererKeyHash {
    std::size_t operator()(const RendererKey& key) const {
        return std::hash<int>{}(key);
    }
};

// Global map to store custom renderer callbacks by renderer ID
static std::unordered_map<RendererKey, CustomRendererCallbacks, RendererKeyHash> g_custom_renderer_callbacks;

extern "C" {

// Function to clean up all callbacks for a specific dataview ID
// This should be called when the DataView control is destroyed
void cleanup_all_custom_renderer_callbacks_for_dataview(int dataview_id) {
    // With the new renderer_id approach, we don't automatically clean up by dataview_id
    // Individual renderers are now cleaned up when they're dropped in Rust
    // This function is kept for compatibility but does nothing
}

// Enhanced DataView control that automatically cleans up custom renderer callbacks
class WxdDataViewCtrlWithCleanup : public wxDataViewCtrl {
public:
    WxdDataViewCtrlWithCleanup(wxWindow* parent, wxWindowID id, const wxPoint& pos, const wxSize& size, long style)
        : wxDataViewCtrl(parent, id, pos, size, style) {
    }
    
    virtual ~WxdDataViewCtrlWithCleanup() {
        // No special cleanup needed - each renderer manages its own callbacks
    }
};

// Base DataViewCtrl functions
WXD_EXPORTED wxd_Window_t* wxd_DataViewCtrl_Create(wxd_Window_t* parent, int64_t id, 
                                            const wxd_Point* pos, const wxd_Size* size, 
                                            int64_t style) {
    if (!parent) return nullptr;
    
    wxWindow* p = reinterpret_cast<wxWindow*>(parent);
    wxPoint wxPos = pos ? wxPoint(pos->x, pos->y) : wxDefaultPosition;
    wxSize wxSizeObj = size ? wxSize(size->width, size->height) : wxDefaultSize;
    
    // Use the enhanced DataView control that automatically cleans up custom renderer callbacks
    WxdDataViewCtrlWithCleanup* ctrl = new WxdDataViewCtrlWithCleanup(p, id, wxPos, wxSizeObj, style);
    return reinterpret_cast<wxd_Window_t*>(ctrl);
}

WXD_EXPORTED wxd_Window_t* wxd_DataViewListCtrl_Create(wxd_Window_t* parent, int64_t id, 
                                                const wxd_Point* pos, const wxd_Size* size, 
                                                int64_t style) {
    if (!parent) return nullptr;
    
    wxWindow* p = reinterpret_cast<wxWindow*>(parent);
    wxPoint wxPos = pos ? wxPoint(pos->x, pos->y) : wxDefaultPosition;
    wxSize wxSizeObj = size ? wxSize(size->width, size->height) : wxDefaultSize;
    
    wxDataViewListCtrl* ctrl = new wxDataViewListCtrl(p, id, wxPos, wxSizeObj, style);
    return reinterpret_cast<wxd_Window_t*>(ctrl);
}

WXD_EXPORTED wxd_Window_t* wxd_DataViewTreeCtrl_Create(wxd_Window_t* parent, int64_t id, 
                                                const wxd_Point* pos, const wxd_Size* size, 
                                                int64_t style) {
    if (!parent) return nullptr;
    
    wxWindow* p = reinterpret_cast<wxWindow*>(parent);
    wxPoint wxPos = pos ? wxPoint(pos->x, pos->y) : wxDefaultPosition;
    wxSize wxSizeObj = size ? wxSize(size->width, size->height) : wxDefaultSize;
    
    wxDataViewTreeCtrl* ctrl = new wxDataViewTreeCtrl(p, id, wxPos, wxSizeObj, style);
    return reinterpret_cast<wxd_Window_t*>(ctrl);
}

// Column management
WXD_EXPORTED wxd_DataViewColumn_t* wxd_DataViewColumn_Create(const char* title, 
                                                     wxd_DataViewRenderer_t* renderer, 
                                                     int model_column,
                                                     int width, 
                                                     int align,
                                                     int flags) {
    if (!renderer) return nullptr;
    
    wxString wxTitle = wxString::FromUTF8(title ? title : "");
    wxDataViewRenderer* r = reinterpret_cast<wxDataViewRenderer*>(renderer);
    
    wxDataViewColumn* column = new wxDataViewColumn(wxTitle, r, 
                                                 static_cast<unsigned int>(model_column),
                                                 width, 
                                                 static_cast<wxAlignment>(align),
                                                 flags);
    return reinterpret_cast<wxd_DataViewColumn_t*>(column);
}

WXD_EXPORTED bool wxd_DataViewCtrl_AppendColumn(wxd_Window_t* self, wxd_DataViewColumn_t* column) {
    if (!self || !column) return false;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(column);
    
    return ctrl->AppendColumn(col);
}

WXD_EXPORTED bool wxd_DataViewCtrl_PrependColumn(wxd_Window_t* self, wxd_DataViewColumn_t* column) {
    if (!self || !column) return false;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(column);
    
    return ctrl->PrependColumn(col);
}

WXD_EXPORTED bool wxd_DataViewCtrl_InsertColumn(wxd_Window_t* self, int64_t pos, wxd_DataViewColumn_t* column) {
    if (!self || !column) return false;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(column);
    
    return ctrl->InsertColumn(static_cast<unsigned int>(pos), col);
}

// Renderer creation functions
WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewTextRenderer_Create(const char* varianttype, 
                                                             int64_t mode, 
                                                             int64_t align) {
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "string");
    wxDataViewTextRenderer* renderer = new wxDataViewTextRenderer(wxVarType, 
                                                               static_cast<wxDataViewCellMode>(mode), 
                                                               static_cast<wxAlignment>(align));
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewIconTextRenderer_Create(const char* varianttype, 
                                                                    int64_t mode, 
                                                                    int64_t align) {
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "wxDataViewIconText");
    wxDataViewIconTextRenderer* renderer = new wxDataViewIconTextRenderer(wxVarType, 
                                                                      static_cast<wxDataViewCellMode>(mode), 
                                                                      static_cast<wxAlignment>(align));
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewToggleRenderer_Create(const char* varianttype, 
                                                                 int64_t mode, 
                                                                 int64_t align) {
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "bool");
    wxDataViewToggleRenderer* renderer = new wxDataViewToggleRenderer(wxVarType, 
                                                                  static_cast<wxDataViewCellMode>(mode), 
                                                                  static_cast<wxAlignment>(align));
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewProgressRenderer_Create(const char* varianttype, 
                                                                    int64_t mode, 
                                                                    int64_t align) {
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "long");
    // The constructor signature is different from other renderers
    wxDataViewProgressRenderer* renderer = new wxDataViewProgressRenderer(
        wxEmptyString,  // label
        wxVarType,      // varianttype
        static_cast<wxDataViewCellMode>(mode)); // mode
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

// Additional renderer implementations
WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewBitmapRenderer_Create(const char* varianttype, 
                                                                 int64_t mode, 
                                                                 int64_t align) {
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "wxBitmap");
    wxDataViewBitmapRenderer* renderer = new wxDataViewBitmapRenderer(
        wxVarType, 
        static_cast<wxDataViewCellMode>(mode),
        static_cast<wxAlignment>(align));
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewDateRenderer_Create(const char* varianttype, 
                                                              int64_t mode, 
                                                              int64_t align) {
    wxString wxVarType = wxString::FromUTF8(varianttype ? varianttype : "datetime");
    wxDataViewDateRenderer* renderer = new wxDataViewDateRenderer(
        wxVarType, 
        static_cast<wxDataViewCellMode>(mode),
        static_cast<wxAlignment>(align));
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewSpinRenderer_Create(const char* varianttype, 
                                                              int64_t mode, 
                                                              int64_t align,
                                                              int32_t min,
                                                              int32_t max,
                                                              int32_t inc) {
    // The constructor order is different: min and max come first, then mode and align
    wxDataViewSpinRenderer* renderer = new wxDataViewSpinRenderer(
        min,
        max,
        static_cast<wxDataViewCellMode>(mode),
        static_cast<int>(align));
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewChoiceRenderer_Create(const char* varianttype, 
                                                                const char* choices_str,
                                                                int64_t mode, 
                                                                int64_t align) {
    wxString wxChoices = wxString::FromUTF8(choices_str ? choices_str : "");
    
    // Parse choices and create wxArrayString
    wxArrayString choices;
    wxStringTokenizer tokenizer(wxChoices, ",");
    while (tokenizer.HasMoreTokens()) {
        choices.Add(tokenizer.GetNextToken().Trim());
    }
    
    wxDataViewChoiceRenderer* renderer = new wxDataViewChoiceRenderer(
        choices,
        static_cast<wxDataViewCellMode>(mode),
        static_cast<int>(align));
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewCheckIconTextRenderer_Create(const char* varianttype, 
                                                                       int64_t mode, 
                                                                       int64_t align) {
    // This renderer doesn't accept a varianttype parameter
    wxDataViewCheckIconTextRenderer* renderer = new wxDataViewCheckIconTextRenderer(
        static_cast<wxDataViewCellMode>(mode),
        static_cast<int>(align));
        
    return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
}

// Custom Renderer Implementation - stores callbacks directly in the instance
class WxdDataViewCustomRenderer : public wxDataViewCustomRenderer {
public:
    typedef wxd_Size_t (*GetSizeCallback)(void* user_data);
    typedef bool (*RenderCallback)(void* user_data, wxd_Rect_t cell, void* dc, int state);
    typedef bool (*SetValueCallback)(void* user_data, const wxd_Variant_t* value);
    typedef void (*GetValueCallback)(void* user_data, wxd_Variant_t* value);
    typedef bool (*HasEditorCtrlCallback)(void* user_data);
    typedef void* (*CreateEditorCtrlCallback)(void* user_data, void* parent, wxd_Rect_t label_rect, const wxd_Variant_t* value);
    typedef bool (*GetValueFromEditorCtrlCallback)(void* user_data, void* editor, wxd_Variant_t* value);
    typedef bool (*ActivateCellCallback)(void* user_data, wxd_Rect_t cell, void* model, void* item, unsigned int col, void* mouse_event);

    WxdDataViewCustomRenderer(
        const wxString& varianttype,
        wxDataViewCellMode mode,
        int align,
        void* user_data,
        GetSizeCallback get_size_callback,
        RenderCallback render_callback,
        SetValueCallback set_value_callback,
        GetValueCallback get_value_callback,
        HasEditorCtrlCallback has_editor_callback,
        CreateEditorCtrlCallback create_editor_callback,
        GetValueFromEditorCtrlCallback get_value_from_editor_callback,
        ActivateCellCallback activate_cell_callback
    ) : wxDataViewCustomRenderer(varianttype, mode, align),
        m_user_data(user_data),
        m_get_size_callback(get_size_callback),
        m_render_callback(render_callback),
        m_set_value_callback(set_value_callback),
        m_get_value_callback(get_value_callback),
        m_has_editor_callback(has_editor_callback),
        m_create_editor_callback(create_editor_callback),
        m_get_value_from_editor_callback(get_value_from_editor_callback),
        m_activate_cell_callback(activate_cell_callback)
    {
        // Constructor implementation without debug logs
    }

    virtual ~WxdDataViewCustomRenderer() {
        // Destructor implementation without debug logs
    }

    // Size calculation for custom rendering
    virtual wxSize GetSize() const override {
        if (m_get_size_callback && m_user_data) {
            wxd_Size_t size = m_get_size_callback(m_user_data);
            return wxSize(size.width, size.height);
        }
        return wxSize(80, 20); // Default size
    }

    virtual bool Render(wxRect cell, wxDC *dc, int state) override {
        if (m_render_callback && m_user_data) {
            wxd_Rect_t cell_rect = {cell.x, cell.y, cell.width, cell.height};
            bool result = m_render_callback(m_user_data, cell_rect, dc, state);
            return result;
        }
        return false;
    }

    virtual bool SetValue(const wxVariant &value) override {
        if (m_set_value_callback && m_user_data) {
            // Convert wxVariant to wxd_Variant_t
            wxd_Variant_t var_data;
            if (value.GetType() == wxT("string")) {
                wxString str = value.GetString();
                var_data.type = WXD_VARIANT_TYPE_STRING;
                var_data.data.string_val = strdup(str.ToUTF8().data());
            } else if (value.GetType() == wxT("bool")) {
                var_data.type = WXD_VARIANT_TYPE_BOOL;
                var_data.data.bool_val = value.GetBool();
            } else if (value.GetType() == wxT("long")) {
                var_data.type = WXD_VARIANT_TYPE_INT32;
                var_data.data.int32_val = static_cast<int32_t>(value.GetLong());
            } else {
                // Default to string
                wxString str = value.GetString();
                var_data.type = WXD_VARIANT_TYPE_STRING;
                var_data.data.string_val = strdup(str.ToUTF8().data());
            }

            bool result = m_set_value_callback(m_user_data, &var_data);

            // Clean up any allocated string
            if (var_data.type == WXD_VARIANT_TYPE_STRING && var_data.data.string_val) {
                free(var_data.data.string_val);
            }

            return result;
        }
        return true;
    }

    virtual bool GetValue(wxVariant &value) const override {
        if (m_get_value_callback && m_user_data) {
            wxd_Variant_t var_data = {0};
            m_get_value_callback(m_user_data, &var_data);

            switch (var_data.type) {
                case WXD_VARIANT_TYPE_STRING:
                    if (var_data.data.string_val) {
                        value = wxString::FromUTF8(var_data.data.string_val);
                        wxd_Variant_Free_Rust_String(var_data.data.string_val);
                    } else {
                        value = wxString();
                    }
                    break;
                case WXD_VARIANT_TYPE_BOOL:
                    value = var_data.data.bool_val;
                    break;
                case WXD_VARIANT_TYPE_INT32:
                    value = static_cast<long>(var_data.data.int32_val);
                    break;
                default:
                    value = wxString();
                    break;
            }
            return true;
        }
        // No callback available, return an empty string variant
        value = wxString();
        return true;
    }

    // Optional editing support
    virtual bool HasEditorCtrl() const override {
        if (m_has_editor_callback && m_user_data) {
            return m_has_editor_callback(m_user_data);
        }
        return false;
    }

    virtual wxWindow* CreateEditorCtrl(wxWindow *parent, wxRect labelRect, const wxVariant &value) override {
        if (m_create_editor_callback && m_user_data) {
            // Convert value to wxd_Variant_t
            wxd_Variant_t var_data;
            if (value.GetType() == wxT("string")) {
                wxString str = value.GetString();
                var_data.type = WXD_VARIANT_TYPE_STRING;
                var_data.data.string_val = strdup(str.ToUTF8().data());
            } else {
                var_data.type = WXD_VARIANT_TYPE_STRING;
                var_data.data.string_val = strdup("");
            }

            wxd_Rect_t rect = {labelRect.x, labelRect.y, labelRect.width, labelRect.height};
            void* editor = m_create_editor_callback(m_user_data, parent, rect, &var_data);

            // Clean up
            if (var_data.data.string_val) {
                free(var_data.data.string_val);
            }

            return reinterpret_cast<wxWindow*>(editor);
        }
        return nullptr;
    }

    virtual bool GetValueFromEditorCtrl(wxWindow *editor, wxVariant &value) override {
        if (m_get_value_from_editor_callback && m_user_data && editor) {
            wxd_Variant_t var_data = {0};
            bool result = m_get_value_from_editor_callback(m_user_data, editor, &var_data);

            if (result) {
                switch (var_data.type) {
                    case WXD_VARIANT_TYPE_STRING:
                        if (var_data.data.string_val) {
                            value = wxString::FromUTF8(var_data.data.string_val);
                            wxd_Variant_Free_Rust_String(var_data.data.string_val);
                        } else {
                            value = wxString();
                        }
                        break;
                    case WXD_VARIANT_TYPE_BOOL:
                        value = var_data.data.bool_val;
                        break;
                    case WXD_VARIANT_TYPE_INT32:
                        value = static_cast<long>(var_data.data.int32_val);
                        break;
                    default:
                        value = wxString();
                        break;
                }
            }

            return result;
        }
        return false;
    }

    // Optional cell activation support
    virtual bool ActivateCell(const wxRect &cell, wxDataViewModel *model, const wxDataViewItem &item, unsigned int col, const wxMouseEvent *mouseEvent) override {
        if (m_activate_cell_callback && m_user_data) {
            wxd_Rect_t cell_rect = {cell.x, cell.y, cell.width, cell.height};
            return m_activate_cell_callback(m_user_data, cell_rect, model, (void*)item.GetID(), col, (void*)mouseEvent);
        }
        return false;
    }

private:
    void* m_user_data;
    GetSizeCallback m_get_size_callback;
    RenderCallback m_render_callback;
    SetValueCallback m_set_value_callback;
    GetValueCallback m_get_value_callback;
    HasEditorCtrlCallback m_has_editor_callback;
    CreateEditorCtrlCallback m_create_editor_callback;
    GetValueFromEditorCtrlCallback m_get_value_from_editor_callback;
    ActivateCellCallback m_activate_cell_callback;
};

// Custom renderer creation
WXD_EXPORTED wxd_DataViewRenderer_t* wxd_DataViewCustomRenderer_Create(
    const char* varianttype,
    int64_t mode,
    int64_t align,
    void* user_data,
    wxd_CustomRenderer_GetSizeCallback get_size_callback,
    wxd_CustomRenderer_RenderCallback render_callback,
    wxd_CustomRenderer_SetValueCallback set_value_callback,
    wxd_CustomRenderer_GetValueCallback get_value_callback,
    wxd_CustomRenderer_HasEditorCtrlCallback has_editor_callback,
    wxd_CustomRenderer_CreateEditorCtrlCallback create_editor_callback,
    wxd_CustomRenderer_GetValueFromEditorCtrlCallback get_value_from_editor_callback,
    wxd_CustomRenderer_ActivateCellCallback activate_cell_callback
) {
    try {
        wxString variant_type(varianttype, wxConvUTF8);
        WxdDataViewCustomRenderer* renderer = new WxdDataViewCustomRenderer(
            variant_type,
            static_cast<wxDataViewCellMode>(mode),
            static_cast<int>(align),
            user_data,
            get_size_callback,
            render_callback,
            set_value_callback,
            get_value_callback,
            has_editor_callback,
            create_editor_callback,
            get_value_from_editor_callback,
            activate_cell_callback
        );
        
        return reinterpret_cast<wxd_DataViewRenderer_t*>(renderer);
    } catch (const std::exception& e) {
        return nullptr;
    } catch (...) {
        return nullptr;
    }
}

// Function to release callbacks by renderer ID (no longer needed with direct storage)
WXD_EXPORTED void wxd_DataViewCustomRenderer_ReleaseCallbacksByKey(int32_t renderer_id) {
    // No-op: callbacks are now cleaned up automatically when renderer is destroyed
}

// Function to release all callbacks for a specific dataview ID (no longer needed)
WXD_EXPORTED void wxd_DataViewCustomRenderer_ReleaseAllCallbacksForDataView(int32_t dataview_id) {
    // No-op: callbacks are now cleaned up automatically when renderers are destroyed
}

// Cleanup function for custom renderer callbacks (legacy - no longer needed)
WXD_EXPORTED void wxd_DataViewCustomRenderer_ReleaseCallbacks(wxd_DataViewRenderer_t* renderer) {
    // No-op: callbacks are now cleaned up automatically in destructor
}

// DataViewModel implementation
class WxDDataViewModel : public wxDataViewModel {
private:
    wxd_DataViewModel_GetColumnCountCallback m_get_column_count;
    wxd_DataViewModel_GetRowCountCallback m_get_row_count;
    wxd_DataViewModel_GetValueCallback m_get_value;
    wxd_DataViewModel_SetValueCallback m_set_value;
    void* m_user_data;
    
public:
    WxDDataViewModel(
        wxd_DataViewModel_GetColumnCountCallback get_column_count,
        wxd_DataViewModel_GetRowCountCallback get_row_count,
        wxd_DataViewModel_GetValueCallback get_value,
        wxd_DataViewModel_SetValueCallback set_value,
        void* user_data)
        : m_get_column_count(get_column_count),
          m_get_row_count(get_row_count),
          m_get_value(get_value),
          m_set_value(set_value),
          m_user_data(user_data) {}
          
    // wxDataViewModel interface implementation
    virtual unsigned int GetColumnCount() const override {
        if (!m_get_column_count) return 0;
        return static_cast<unsigned int>(m_get_column_count(m_user_data));
    }
    
    virtual wxString GetColumnType(unsigned int col) const override {
        // We'll need a way to get column types...
        return wxS("string");
    }
    
    virtual void GetValue(wxVariant& variant, 
                         const wxDataViewItem& item, 
                         unsigned int col) const override {
        if (!m_get_value) return;
        
        // Convert wxDataViewItem to row index
        unsigned int row = wxDataViewItem(item).GetID() ? static_cast<unsigned int>(reinterpret_cast<uintptr_t>(item.GetID())) - 1 : 0;
        
        // Create wxd_Variant_t for the callback
        wxd_Variant_t wxd_variant;
        memset(&wxd_variant, 0, sizeof(wxd_variant));
        
        // Call the callback
        m_get_value(m_user_data, row, col, &wxd_variant);
        
        // Convert wxd_Variant_t to wxVariant
        switch (wxd_variant.type) {
            case WXD_VARIANT_TYPE_BOOL:
                variant = wxd_variant.data.bool_val;
                break;
            case WXD_VARIANT_TYPE_INT32:
                variant = static_cast<long>(wxd_variant.data.int32_val);
                break;
            case WXD_VARIANT_TYPE_INT64:
                variant = static_cast<long>(wxd_variant.data.int64_val);
                break;
            case WXD_VARIANT_TYPE_DOUBLE:
                variant = wxd_variant.data.double_val;
                break;
            case WXD_VARIANT_TYPE_STRING:
                if (wxd_variant.data.string_val) {
                    variant = wxString::FromUTF8(wxd_variant.data.string_val);
                    // Note: We don't free the string here because the callback
                    // maintains ownership. The string will be freed when the model
                    // is destroyed or updated. If we freed it here, it would 
                    // potentially cause a use-after-free error.
                } else {
                    variant = wxString();
                }
                break;
            // Handle other types as needed
            default:
                // Set an empty variant
                variant.Clear();
                break;
        }
    }
    
    virtual bool SetValue(const wxVariant& variant, 
                         const wxDataViewItem& item, 
                         unsigned int col) override {
        if (!m_set_value) return false;
        
        // Convert wxDataViewItem to row index
        unsigned int row = wxDataViewItem(item).GetID() ? static_cast<unsigned int>(reinterpret_cast<uintptr_t>(item.GetID())) - 1 : 0;
        
        // Create wxd_Variant_t for the callback
        wxd_Variant_t wxd_variant;
        memset(&wxd_variant, 0, sizeof(wxd_variant));
        
        // Convert wxVariant to wxd_Variant_t
        if (variant.GetType() == "bool") {
            wxd_variant.type = WXD_VARIANT_TYPE_BOOL;
            wxd_variant.data.bool_val = variant.GetBool();
        } else if (variant.GetType() == "long") {
            wxd_variant.type = WXD_VARIANT_TYPE_INT64;
            wxd_variant.data.int64_val = variant.GetLong();
        } else if (variant.GetType() == "double") {
            wxd_variant.type = WXD_VARIANT_TYPE_DOUBLE;
            wxd_variant.data.double_val = variant.GetDouble();
        } else if (variant.GetType() == "string") {
            wxd_variant.type = WXD_VARIANT_TYPE_STRING;
            wxd_variant.data.string_val = strdup(variant.GetString().ToUTF8().data());
        }
        // Handle other types as needed
        
        // Call the callback
        bool result = m_set_value(m_user_data, row, col, &wxd_variant);
        
        // Free any allocated memory
        if (wxd_variant.type == WXD_VARIANT_TYPE_STRING && wxd_variant.data.string_val) {
            free(wxd_variant.data.string_val);
        }
        
        return result;
    }
    
    virtual wxDataViewItem GetParent(const wxDataViewItem& item) const override {
        // For list models, items have no parent
        return wxDataViewItem(nullptr);
    }
    
    virtual bool IsContainer(const wxDataViewItem& item) const override {
        // For list models, only the invisible root is a container
        return !item.IsOk();
    }
    
    virtual unsigned int GetChildren(const wxDataViewItem& parent, 
                                    wxDataViewItemArray& children) const override {
        if (!m_get_row_count) return 0;
        
        // For a list model, only the invisible root item has children
        if (!parent.IsOk()) {
            int count = static_cast<int>(m_get_row_count(m_user_data));
            for (int i = 0; i < count; ++i) {
                // Use index as the item ID
                children.Add(wxDataViewItem(reinterpret_cast<void*>(static_cast<uintptr_t>(i + 1))));
            }
            return count;
        }
        return 0;
    }
};

// Model creation and attachment
WXD_EXPORTED wxd_DataViewModel_t* wxd_DataViewModel_Create(
    wxd_DataViewModel_GetColumnCountCallback get_column_count,
    wxd_DataViewModel_GetRowCountCallback get_row_count,
    wxd_DataViewModel_GetValueCallback get_value,
    wxd_DataViewModel_SetValueCallback set_value,
    void* user_data) {
    
    if (!get_column_count || !get_row_count || !get_value) return nullptr;
    
    WxDDataViewModel* model = new WxDDataViewModel(
        get_column_count,
        get_row_count,
        get_value,
        set_value,
        user_data);
        
    return reinterpret_cast<wxd_DataViewModel_t*>(model);
}

WXD_EXPORTED bool wxd_DataViewCtrl_AssociateModel(wxd_Window_t* self, wxd_DataViewModel_t* model) {
    if (!self || !model) return false;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewModel* m = reinterpret_cast<wxDataViewModel*>(model);
    
    // When we associate a model with a control, wxWidgets internally calls IncRef()
    // We'll make sure the model is properly referenced
    
    // Workaround for a known issue in wxWidgets: DataViewCtrl::AssociateModel
    // is taking ownership of the model, but we need to maintain it separately
    // This is why we call IncRef before passing it
    m->IncRef();
    
    // AssociateModel returns a bool indicating success/failure
    bool result = ctrl->AssociateModel(m);
    
    if (!result) {
        // If associating failed, we need to revert our IncRef
        m->DecRef();
    }
    
    return result;
}

// Selection management
WXD_EXPORTED bool wxd_DataViewCtrl_SelectRow(wxd_Window_t* self, int64_t row) {
    if (!self) return false;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewItem item(reinterpret_cast<void*>(static_cast<uintptr_t>(row + 1)));
    
    ctrl->Select(item);
    return true;
}

WXD_EXPORTED int64_t wxd_DataViewCtrl_GetSelectedRow(wxd_Window_t* self) {
    if (!self) return -1;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewItem item = ctrl->GetSelection();
    
    if (!item.IsOk()) return -1;
    
    return reinterpret_cast<uintptr_t>(item.GetID()) - 1;
}

WXD_EXPORTED void wxd_DataViewCtrl_UnselectAll(wxd_Window_t* self) {
    if (!self) return;
    
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    ctrl->UnselectAll();
}

// Standard DataViewListModel implementation
class WxDDataViewListModel : public wxDataViewListStore {
private:
    // Keep track of our columns
    struct ColumnInfo {
        wxString name;
        wxString type;
    };
    wxVector<ColumnInfo> m_columns;

public:
    // Add a column to our model
    bool AppendColumnInfo(const wxString& name, const wxString& type = "string") {
        ColumnInfo info;
        info.name = name;
        info.type = type;
        m_columns.push_back(info);
        return true;
    }
    
    // Get number of columns we've defined
    unsigned int GetColumnCount() const {
        return static_cast<unsigned int>(m_columns.size());
    }
};

// Standard models
WXD_EXPORTED wxd_DataViewModel_t* wxd_DataViewListModel_Create() {
    WxDDataViewListModel* model = new WxDDataViewListModel();
    return reinterpret_cast<wxd_DataViewModel_t*>(model);
}

WXD_EXPORTED bool wxd_DataViewListModel_AppendColumn(wxd_DataViewModel_t* self, const char* name) {
    if (!self) return false;
    
    WxDDataViewListModel* model = reinterpret_cast<WxDDataViewListModel*>(self);
    wxString wxName = wxString::FromUTF8(name ? name : "");
    
    // Actually add the column to our model
    return model->AppendColumnInfo(wxName);
}

WXD_EXPORTED bool wxd_DataViewListModel_AppendRow(wxd_DataViewModel_t* self) {
    if (!self) return false;
    
    WxDDataViewListModel* model = reinterpret_cast<WxDDataViewListModel*>(self);
    
    // Get the number of columns from our model
    size_t colCount = model->GetColumnCount();
    if (colCount == 0) {
        // No columns defined yet, can't add rows
        return false;
    }
    
    // Create proper-sized vector of variants for the new row
    wxVector<wxVariant> values;
    values.resize(colCount);  // Initialize with empty values for each column
    
    model->AppendItem(values);
    return true;
}

WXD_EXPORTED bool wxd_DataViewListModel_SetValue(wxd_DataViewModel_t* self, 
                                       uint64_t row, 
                                       uint64_t col, 
                                       const wxd_Variant_t* variant) {
    if (!self || !variant) return false;
    
    WxDDataViewListModel* model = reinterpret_cast<WxDDataViewListModel*>(self);
    
    // Make sure we have enough columns defined
    if (col >= model->GetColumnCount()) {
        return false;  // Column index out of bounds
    }
    
    // Create a wxVariant from our wxd_Variant_t
    wxVariant wxVariantValue;
    
    // Convert wxd_Variant_t to wxVariant
    switch (variant->type) {
        case WXD_VARIANT_TYPE_BOOL:
            wxVariantValue = variant->data.bool_val;
            break;
        case WXD_VARIANT_TYPE_INT32:
            wxVariantValue = static_cast<long>(variant->data.int32_val);
            break;
        case WXD_VARIANT_TYPE_INT64:
            wxVariantValue = static_cast<long>(variant->data.int64_val);
            break;
        case WXD_VARIANT_TYPE_DOUBLE:
            wxVariantValue = variant->data.double_val;
            break;
        case WXD_VARIANT_TYPE_STRING:
            if (variant->data.string_val) {
                wxVariantValue = wxString::FromUTF8(variant->data.string_val);
            } else {
                wxVariantValue = wxString();
            }
            break;
        case WXD_VARIANT_TYPE_BITMAP:
            if (variant->data.bitmap_val) {
                wxBitmap* bitmap = reinterpret_cast<wxBitmap*>(variant->data.bitmap_val);
                // Create a wxVariant with a bitmap value using native wxWidgets support
                wxVariantValue << *bitmap;
            } else {
                wxVariantValue.Clear();
            }
            break;
        case WXD_VARIANT_TYPE_DATETIME:
            {
                // Convert wxd_DateTime_t to wxDateTime
                wxDateTime dt;
                dt.Set(variant->data.datetime_val.day,
                      static_cast<wxDateTime::Month>(variant->data.datetime_val.month - 1),
                      variant->data.datetime_val.year,
                      variant->data.datetime_val.hour,
                      variant->data.datetime_val.minute,
                      variant->data.datetime_val.second);
                wxVariantValue = dt;
            }
            break;
        default:
            // Set an empty variant
            wxVariantValue.Clear();
            break;
    }
    
    // Create a wxDataViewItem for the row
    wxDataViewItem item(reinterpret_cast<void*>(static_cast<uintptr_t>(row + 1)));
    
    // Set the value
    return model->SetValue(wxVariantValue, item, static_cast<unsigned int>(col));
}

WXD_EXPORTED void wxd_Variant_Free(wxd_Variant_t* variant) {
    if (!variant) return;
    
    // Free any string data using the proper Rust deallocation function
    if (variant->type == WXD_VARIANT_TYPE_STRING && variant->data.string_val) {
        wxd_Variant_Free_Rust_String(variant->data.string_val);
        variant->data.string_val = NULL;
    }
    
    // Free the variant itself
    free(variant);
}

// Column management
WXD_EXPORTED int wxd_DataViewCtrl_GetColumnCount(wxd_Window_t* self) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return 0;
    return ctrl->GetColumnCount();
}

WXD_EXPORTED wxd_DataViewColumn_t* wxd_DataViewCtrl_GetColumn(wxd_Window_t* self, uint32_t pos) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return nullptr;
    return reinterpret_cast<wxd_DataViewColumn_t*>(ctrl->GetColumn(pos));
}

WXD_EXPORTED int wxd_DataViewCtrl_GetColumnPosition(wxd_Window_t* self, wxd_DataViewColumn_t* column) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(column);
    if (!ctrl || !col) return -1;
    return ctrl->GetColumnPosition(col);
}

WXD_EXPORTED bool wxd_DataViewCtrl_ClearColumns(wxd_Window_t* self) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return false;
    return ctrl->ClearColumns();
}

// Item management
WXD_EXPORTED void wxd_DataViewCtrl_Select(wxd_Window_t* self, const wxd_DataViewItem_t* item) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return;
    const wxDataViewItem* inner = reinterpret_cast<const wxDataViewItem*>(item);
    if (!inner) return;
    ctrl->Select(*inner);
}

WXD_EXPORTED void wxd_DataViewCtrl_Unselect(wxd_Window_t* self, const wxd_DataViewItem_t* item) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return;
    const wxDataViewItem* inner = reinterpret_cast<const wxDataViewItem*>(item);
    if (!inner) return;
    ctrl->Unselect(*inner);
}

WXD_EXPORTED void wxd_DataViewCtrl_SelectAll(wxd_Window_t* self) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return;
    ctrl->SelectAll();
}

WXD_EXPORTED bool wxd_DataViewCtrl_IsSelected(wxd_Window_t* self, const wxd_DataViewItem_t* item) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return false;
    const wxDataViewItem* inner = reinterpret_cast<const wxDataViewItem*>(item);
    if (!inner) return false;
    return ctrl->IsSelected(*inner);
}

WXD_EXPORTED uint32_t wxd_DataViewCtrl_GetSelectedItemsCount(wxd_Window_t* self) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return 0;
    return ctrl->GetSelectedItemsCount();
}

WXD_EXPORTED const wxd_DataViewItem_t* wxd_DataViewCtrl_GetSelection(wxd_Window_t* self) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return nullptr;

    wxDataViewItem item = ctrl->GetSelection();
    if (!item.IsOk()) return nullptr;
    return wxd_DataViewItem_Clone(reinterpret_cast<const wxd_DataViewItem_t*>(&item));
}

WXD_EXPORTED void wxd_DataViewCtrl_GetSelections(wxd_Window_t* self, const wxd_DataViewItem_t* * items, uint32_t max_count) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl || !items || max_count == 0) return;

    wxDataViewItemArray selections;
    ctrl->GetSelections(selections);

    // Only include valid selections (IsOk) in the results. Skip invalid
    // entries and stop when we've filled up to max_count.
    uint32_t written = 0;
    uint32_t total = static_cast<uint32_t>(selections.GetCount());
    for (uint32_t i = 0; i < total && written < max_count; ++i) {
        if (!selections[i].IsOk()) continue; // skip invalid items
        items[written++] = wxd_DataViewItem_Clone(reinterpret_cast<const wxd_DataViewItem_t*>(&selections[i]));
    }
}

WXD_EXPORTED void wxd_DataViewCtrl_SetSelections(wxd_Window_t* self, const wxd_DataViewItem_t* const* items, uint32_t count) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl || !items || count == 0) return;

    wxDataViewItemArray selections;
    selections.Alloc(count);
    
    for (uint32_t i = 0; i < count; i++) {
        const wxDataViewItem* inner = reinterpret_cast<const wxDataViewItem*>(items[i]);
        selections.Add(*inner);
    }
    
    ctrl->SetSelections(selections);
}

WXD_EXPORTED const wxd_DataViewItem_t* wxd_DataViewCtrl_GetCurrentItem(wxd_Window_t* self) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return nullptr;

    wxDataViewItem item = ctrl->GetCurrentItem();
    if (!item.IsOk()) return nullptr;
    return wxd_DataViewItem_Clone(reinterpret_cast<const wxd_DataViewItem_t*>(&item));
}

WXD_EXPORTED void wxd_DataViewCtrl_SetCurrentItem(wxd_Window_t* self, const wxd_DataViewItem_t* item) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl || !item) return;
    const wxDataViewItem* inner = reinterpret_cast<const wxDataViewItem*>(item);
    ctrl->SetCurrentItem(*inner);
}

// Visual appearance
WXD_EXPORTED int wxd_DataViewCtrl_GetIndent(wxd_Window_t* self) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return 0;
    return ctrl->GetIndent();
}

WXD_EXPORTED void wxd_DataViewCtrl_SetIndent(wxd_Window_t* self, int indent) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return;
    ctrl->SetIndent(indent);
}

WXD_EXPORTED wxd_DataViewColumn_t* wxd_DataViewCtrl_GetExpanderColumn(wxd_Window_t* self) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return nullptr;
    return reinterpret_cast<wxd_DataViewColumn_t*>(ctrl->GetExpanderColumn());
}

WXD_EXPORTED void wxd_DataViewCtrl_SetExpanderColumn(wxd_Window_t* self, wxd_DataViewColumn_t* column) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(column);
    if (!ctrl || !col) return;
    ctrl->SetExpanderColumn(col);
}

WXD_EXPORTED bool wxd_DataViewCtrl_SetRowHeight(wxd_Window_t* self, int height) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl) return false;
    return ctrl->SetRowHeight(height);
}

WXD_EXPORTED bool wxd_DataViewCtrl_SetAlternateRowColour(wxd_Window_t* self, const wxd_Colour_t* colour) {
    wxDataViewCtrl* ctrl = reinterpret_cast<wxDataViewCtrl*>(self);
    if (!ctrl || !colour) return false;
    
    wxColour wxColour(colour->r, colour->g, colour->b, colour->a);
    return ctrl->SetAlternateRowColour(wxColour);
}

// DataViewColumn property implementations
WXD_EXPORTED void wxd_DataViewColumn_SetTitle(wxd_DataViewColumn_t* self, const char* title) {
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(self);
    if (col) {
        col->SetTitle(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title));
    }
}

WXD_EXPORTED void wxd_DataViewColumn_SetResizeable(wxd_DataViewColumn_t* self, bool resizeable) {
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(self);
    if (col) {
        col->SetResizeable(resizeable);
    }
}

WXD_EXPORTED bool wxd_DataViewColumn_IsResizeable(wxd_DataViewColumn_t* self) {
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(self);
    if (col) {
        return col->IsResizeable();
    }
    return false; // Default if col is null
}

WXD_EXPORTED void wxd_DataViewColumn_SetSortable(wxd_DataViewColumn_t* self, bool sortable) {
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(self);
    if (col) {
        col->SetSortable(sortable);
    }
}

WXD_EXPORTED bool wxd_DataViewColumn_IsSortable(wxd_DataViewColumn_t* self) {
    wxDataViewColumn* col = reinterpret_cast<wxDataViewColumn*>(self);
    if (col) {
        return col->IsSortable();
    }
    return false; // Default if col is null
}

} // extern "C" 