#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/dataview.h>
#include "wxd_utils.h"

// Define a concrete implementation of wxDataViewVirtualListModel
class WxdBasicDataViewVirtualListModel : public wxDataViewVirtualListModel {
public:
    WxdBasicDataViewVirtualListModel(unsigned int initial_size)
        : wxDataViewVirtualListModel(initial_size)
    {
        WXD_LOG_TRACEF("WxdBasicDataViewVirtualListModel created with pointer %p", this);
    }
    ~WxdBasicDataViewVirtualListModel()
    {
        WXD_LOG_TRACEF("WxdBasicDataViewVirtualListModel destroyed with pointer %p", this);
    }

    // Implementation of the pure virtual methods
    virtual void
    GetValueByRow(wxVariant& variant, unsigned int row, unsigned int col) const override
    {
        // Default implementation returns a simple text string
        variant = wxVariant(wxString::Format(wxT("Item (%d, %d)"), row, col));
    }

    virtual bool
    SetValueByRow(const wxVariant& variant, unsigned int row, unsigned int col) override
    {
        // Default implementation doesn't support setting values
        return false;
    }
};

// C-style FFI functions for wxDataViewVirtualListModel

extern "C" {

// Creates a new wxDataViewVirtualListModel
wxd_DataViewModel_t*
wxd_DataViewVirtualListModel_Create(uint64_t initial_size)
{
    WxdBasicDataViewVirtualListModel* model =
        new WxdBasicDataViewVirtualListModel(static_cast<unsigned int>(initial_size));
    return reinterpret_cast<wxd_DataViewModel_t*>(model);
}

// Notify methods for row changes
void
wxd_DataViewVirtualListModel_RowPrepended(wxd_DataViewModel_t* model)
{
    wxDataViewVirtualListModel* vmodel = reinterpret_cast<wxDataViewVirtualListModel*>(model);
    if (vmodel) {
        vmodel->RowPrepended();
    }
}

void
wxd_DataViewVirtualListModel_RowInserted(wxd_DataViewModel_t* model, uint64_t before)
{
    wxDataViewVirtualListModel* vmodel = reinterpret_cast<wxDataViewVirtualListModel*>(model);
    if (vmodel) {
        vmodel->RowInserted(static_cast<unsigned int>(before));
    }
}

void
wxd_DataViewVirtualListModel_RowAppended(wxd_DataViewModel_t* model)
{
    wxDataViewVirtualListModel* vmodel = reinterpret_cast<wxDataViewVirtualListModel*>(model);
    if (vmodel) {
        vmodel->RowAppended();
    }
}

void
wxd_DataViewVirtualListModel_RowDeleted(wxd_DataViewModel_t* model, uint64_t row)
{
    wxDataViewVirtualListModel* vmodel = reinterpret_cast<wxDataViewVirtualListModel*>(model);
    if (vmodel) {
        vmodel->RowDeleted(static_cast<unsigned int>(row));
    }
}

void
wxd_DataViewVirtualListModel_RowsDeleted(wxd_DataViewModel_t* model, int32_t* rows, int32_t count)
{
    wxDataViewVirtualListModel* vmodel = reinterpret_cast<wxDataViewVirtualListModel*>(model);
    if (vmodel && rows && count > 0) {
        wxArrayInt rowsArray;
        for (int32_t i = 0; i < count; i++) {
            rowsArray.Add(rows[i]);
        }
        vmodel->RowsDeleted(rowsArray);
    }
}

void
wxd_DataViewVirtualListModel_RowChanged(wxd_DataViewModel_t* model, uint64_t row)
{
    wxDataViewVirtualListModel* vmodel = reinterpret_cast<wxDataViewVirtualListModel*>(model);
    if (vmodel) {
        vmodel->RowChanged(static_cast<unsigned int>(row));
    }
}

void
wxd_DataViewVirtualListModel_RowValueChanged(wxd_DataViewModel_t* model, uint64_t row, uint64_t col)
{
    wxDataViewVirtualListModel* vmodel = reinterpret_cast<wxDataViewVirtualListModel*>(model);
    if (vmodel) {
        vmodel->RowValueChanged(static_cast<unsigned int>(row), static_cast<unsigned int>(col));
    }
}

void
wxd_DataViewVirtualListModel_Reset(wxd_DataViewModel_t* model, uint64_t new_size)
{
    wxDataViewVirtualListModel* vmodel = reinterpret_cast<wxDataViewVirtualListModel*>(model);
    if (vmodel) {
        vmodel->Reset(static_cast<unsigned int>(new_size));
    }
}

// Conversion between row indices and wxDataViewItem
void*
wxd_DataViewVirtualListModel_GetItem(wxd_DataViewModel_t* model, uint64_t row)
{
    wxDataViewVirtualListModel* vmodel = reinterpret_cast<wxDataViewVirtualListModel*>(model);
    if (vmodel) {
        wxDataViewItem item = vmodel->GetItem(static_cast<unsigned int>(row));
        return item.GetID();
    }
    return nullptr;
}

uint64_t
wxd_DataViewVirtualListModel_GetRow(wxd_DataViewModel_t* model, void* item)
{
    wxDataViewVirtualListModel* vmodel = reinterpret_cast<wxDataViewVirtualListModel*>(model);
    if (vmodel && item) {
        wxDataViewItem dataViewItem(item);
        unsigned int row = vmodel->GetRow(dataViewItem);
        return static_cast<uint64_t>(row);
    }
    return static_cast<uint64_t>(-1); // Return -1 (as uint64_t) to indicate "not found"
}

// Custom renderer for the virtual list model
// This part will be needed if we implement user-defined value retrieval

// Define a structure to hold function pointers for callbacks
typedef struct {
    void* userdata;
    void (*get_value_callback)(void* userdata, uint64_t row, uint64_t col, wxd_Variant_t* variant);
    bool (*set_value_callback)(void* userdata, const wxd_Variant_t* variant, uint64_t row,
                               uint64_t col);
    bool (*get_attr_callback)(void* userdata, uint64_t row, uint64_t col,
                              wxd_DataViewItemAttr_t* attr);
    bool (*is_enabled_callback)(void* userdata, uint64_t row, uint64_t col);
} VirtualListModelCallbacks;

// We'll implement the callback mechanism in a future iteration if needed

} // extern "C"