#ifndef WXD_DATAVIEW_H
#define WXD_DATAVIEW_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Forward declarations
typedef struct wxd_DataViewCtrl_tag wxd_DataViewCtrl_t;
typedef struct wxd_DataViewRenderer_t wxd_DataViewRenderer_t;
// typedef struct wxd_DataViewColumn_tag wxd_DataViewColumn_t;

// Define the alignment enum if not defined
typedef enum {
    WXD_ALIGN_LEFT = 0,
    WXD_ALIGN_RIGHT,
    WXD_ALIGN_CENTER,
} wxd_AlignmentCEnum;

// Define wxd_Id if needed
// typedef int64_t wxd_Id; -- Already defined in wxd_types.h

// Base DataViewCtrl functions
WXD_EXPORTED wxd_Window_t*
wxd_DataViewCtrl_Create(wxd_Window_t* parent, int64_t id, const wxd_Point* pos,
                        const wxd_Size* size, int64_t style);

WXD_EXPORTED wxd_Window_t*
wxd_DataViewListCtrl_Create(wxd_Window_t* parent, int64_t id, const wxd_Point* pos,
                            const wxd_Size* size, int64_t style);

WXD_EXPORTED wxd_Window_t*
wxd_DataViewTreeCtrl_Create(wxd_Window_t* parent, int64_t id, const wxd_Point* pos,
                            const wxd_Size* size, int64_t style);

// Column management
WXD_EXPORTED wxd_DataViewColumn_t*
wxd_DataViewColumn_Create(
    const char* title, wxd_DataViewRenderer_t* renderer,
    int model_column, // native wx uses unsigned int, but int is safer for C FFI boundaries if values are not huge.
    int width,
    int align, // wxAlignment is an enum, typically int based.
    int flags  // wxDataViewColumnFlags, int based.
);

WXD_EXPORTED bool
wxd_DataViewCtrl_AppendColumn(wxd_Window_t* self, wxd_DataViewColumn_t* column);
WXD_EXPORTED bool
wxd_DataViewCtrl_PrependColumn(wxd_Window_t* self, wxd_DataViewColumn_t* column);
WXD_EXPORTED bool
wxd_DataViewCtrl_InsertColumn(wxd_Window_t* self, int64_t pos, wxd_DataViewColumn_t* column);

// Additional column management
WXD_EXPORTED int
wxd_DataViewCtrl_GetColumnCount(wxd_Window_t* self);
WXD_EXPORTED wxd_DataViewColumn_t*
wxd_DataViewCtrl_GetColumn(wxd_Window_t* self, uint32_t pos);
WXD_EXPORTED int
wxd_DataViewCtrl_GetColumnPosition(wxd_Window_t* self, wxd_DataViewColumn_t* column);
WXD_EXPORTED bool
wxd_DataViewCtrl_ClearColumns(wxd_Window_t* self);

// Item management
WXD_EXPORTED void
wxd_DataViewCtrl_Select(wxd_Window_t* self, const wxd_DataViewItem_t* item);
WXD_EXPORTED void
wxd_DataViewCtrl_Unselect(wxd_Window_t* self, const wxd_DataViewItem_t* item);
WXD_EXPORTED void
wxd_DataViewCtrl_SelectAll(wxd_Window_t* self);
WXD_EXPORTED bool
wxd_DataViewCtrl_IsSelected(wxd_Window_t* self, const wxd_DataViewItem_t* item);
WXD_EXPORTED uint32_t
wxd_DataViewCtrl_GetSelectedItemsCount(wxd_Window_t* self);
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewCtrl_GetSelection(wxd_Window_t* self);
WXD_EXPORTED void
wxd_DataViewCtrl_GetSelections(wxd_Window_t* self, const wxd_DataViewItem_t** items,
                               uint32_t max_count);
WXD_EXPORTED void
wxd_DataViewCtrl_SetSelections(wxd_Window_t* self, const wxd_DataViewItem_t* const* items,
                               uint32_t count);

WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewCtrl_GetCurrentItem(wxd_Window_t* self);
WXD_EXPORTED void
wxd_DataViewCtrl_SetCurrentItem(wxd_Window_t* self, const wxd_DataViewItem_t* item);

// Visual appearance
WXD_EXPORTED int
wxd_DataViewCtrl_GetIndent(wxd_Window_t* self);
WXD_EXPORTED void
wxd_DataViewCtrl_SetIndent(wxd_Window_t* self, int indent);
WXD_EXPORTED wxd_DataViewColumn_t*
wxd_DataViewCtrl_GetExpanderColumn(wxd_Window_t* self);
WXD_EXPORTED void
wxd_DataViewCtrl_SetExpanderColumn(wxd_Window_t* self, wxd_DataViewColumn_t* column);
WXD_EXPORTED bool
wxd_DataViewCtrl_SetRowHeight(wxd_Window_t* self, int height);
WXD_EXPORTED bool
wxd_DataViewCtrl_SetAlternateRowColour(wxd_Window_t* self, const wxd_Colour_t* colour);

// Sorting controls
WXD_EXPORTED void
wxd_DataViewCtrl_ClearSorting(wxd_Window_t* self);
WXD_EXPORTED bool
wxd_DataViewCtrl_SetSortingColumn(wxd_Window_t* self, int32_t column_index, bool ascending);
// Query current sorting state: returns true if a sorting column exists and writes
// model column index and ascending flag to out parameters.
WXD_EXPORTED bool
wxd_DataViewCtrl_GetSortingState(const wxd_Window_t* self, int32_t* model_column, bool* ascending);

// Provide tree-style navigation methods for DataViewCtrl when used with tree models
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewCtrl_GetNthChild(wxd_Window_t* self, const wxd_DataViewItem_t* parent,
                             unsigned int pos);
WXD_EXPORTED void
wxd_DataViewCtrl_Expand(wxd_Window_t* self, const wxd_DataViewItem_t* item);
WXD_EXPORTED void
wxd_DataViewCtrl_EnsureVisible(wxd_Window_t* self, const wxd_DataViewItem_t* item);

// Renderer creation functions
WXD_EXPORTED wxd_DataViewRenderer_t*
wxd_DataViewTextRenderer_Create(const char* varianttype, int64_t mode, int64_t align);

WXD_EXPORTED wxd_DataViewRenderer_t*
wxd_DataViewIconTextRenderer_Create(const char* varianttype, int64_t mode, int64_t align);

WXD_EXPORTED wxd_DataViewRenderer_t*
wxd_DataViewToggleRenderer_Create(const char* varianttype, int64_t mode, int64_t align);

WXD_EXPORTED wxd_DataViewRenderer_t*
wxd_DataViewProgressRenderer_Create(const char* varianttype, int64_t mode, int64_t align);

// Additional renderer creation functions
WXD_EXPORTED wxd_DataViewRenderer_t*
wxd_DataViewBitmapRenderer_Create(const char* varianttype, int64_t mode, int64_t align);

WXD_EXPORTED wxd_DataViewRenderer_t*
wxd_DataViewDateRenderer_Create(const char* varianttype, int64_t mode, int64_t align);

WXD_EXPORTED wxd_DataViewRenderer_t*
wxd_DataViewSpinRenderer_Create(const char* varianttype, int64_t mode, int64_t align, int32_t min,
                                int32_t max, int32_t inc);

WXD_EXPORTED wxd_DataViewRenderer_t*
wxd_DataViewChoiceRenderer_Create(const char* varianttype, const char* choices, int64_t mode,
                                  int64_t align);

WXD_EXPORTED wxd_DataViewRenderer_t*
wxd_DataViewCheckIconTextRenderer_Create(const char* varianttype, int64_t mode, int64_t align);

// Model callback types
typedef uint64_t (*wxd_DataViewModel_GetColumnCountCallback)(void* user_data);
typedef uint64_t (*wxd_DataViewModel_GetRowCountCallback)(void* user_data);
typedef wxd_Variant_t* (*wxd_DataViewModel_GetValueCallback)(void* user_data, uint64_t row,
                                                             uint64_t col);
typedef bool (*wxd_DataViewModel_SetValueCallback)(void* user_data, uint64_t row, uint64_t col,
                                                   const wxd_Variant_t* variant);

WXD_EXPORTED void
wxd_DataViewModel_AddRef(wxd_DataViewModel_t* model);
WXD_EXPORTED void
wxd_DataViewModel_Release(wxd_DataViewModel_t* model);
WXD_EXPORTED int
wxd_DataViewModel_GetRefCount(const wxd_DataViewModel_t* model);

// Model creation and attachment
WXD_EXPORTED wxd_DataViewModel_t*
wxd_DataViewModel_Create(wxd_DataViewModel_GetColumnCountCallback get_column_count,
                         wxd_DataViewModel_GetRowCountCallback get_row_count,
                         wxd_DataViewModel_GetValueCallback get_value,
                         wxd_DataViewModel_SetValueCallback set_value, void* user_data);

WXD_EXPORTED bool
wxd_DataViewCtrl_AssociateModel(wxd_Window_t* self, wxd_DataViewModel_t* model);

// Standard models
WXD_EXPORTED wxd_DataViewModel_t*
wxd_DataViewListModel_Create();
WXD_EXPORTED bool
wxd_DataViewListModel_AppendColumn(wxd_DataViewModel_t* self, const char* name);
WXD_EXPORTED bool
wxd_DataViewListModel_AppendRow(wxd_DataViewModel_t* self);
WXD_EXPORTED bool
wxd_DataViewListModel_SetValue(wxd_DataViewModel_t* self, size_t row, size_t col,
                               const wxd_Variant_t* variant);

// Selection management
WXD_EXPORTED bool
wxd_DataViewCtrl_SelectRow(wxd_Window_t* self, int64_t row);
WXD_EXPORTED int64_t
wxd_DataViewCtrl_GetSelectedRow(wxd_Window_t* self);
WXD_EXPORTED void
wxd_DataViewCtrl_UnselectAll(wxd_Window_t* self);

// DataViewVirtualListModel functions
WXD_EXPORTED wxd_DataViewModel_t*
wxd_DataViewVirtualListModel_Create(uint64_t initial_size);
WXD_EXPORTED void
wxd_DataViewVirtualListModel_RowPrepended(wxd_DataViewModel_t* model);
WXD_EXPORTED void
wxd_DataViewVirtualListModel_RowInserted(wxd_DataViewModel_t* model, uint64_t before);
WXD_EXPORTED void
wxd_DataViewVirtualListModel_RowAppended(wxd_DataViewModel_t* model);
WXD_EXPORTED void
wxd_DataViewVirtualListModel_RowDeleted(wxd_DataViewModel_t* model, uint64_t row);
WXD_EXPORTED void
wxd_DataViewVirtualListModel_RowsDeleted(wxd_DataViewModel_t* model, int32_t* rows, int32_t count);
WXD_EXPORTED void
wxd_DataViewVirtualListModel_RowChanged(wxd_DataViewModel_t* model, uint64_t row);
WXD_EXPORTED void
wxd_DataViewVirtualListModel_RowValueChanged(wxd_DataViewModel_t* model, uint64_t row,
                                             uint64_t col);
WXD_EXPORTED void
wxd_DataViewVirtualListModel_Reset(wxd_DataViewModel_t* model, uint64_t new_size);
WXD_EXPORTED void*
wxd_DataViewVirtualListModel_GetItem(wxd_DataViewModel_t* model, uint64_t row);
WXD_EXPORTED uint64_t
wxd_DataViewVirtualListModel_GetRow(wxd_DataViewModel_t* model, void* item);

// Custom virtual list model with callbacks
typedef struct {
    bool has_text_colour;
    unsigned char text_colour_red;
    unsigned char text_colour_green;
    unsigned char text_colour_blue;
    unsigned char text_colour_alpha;

    bool has_bg_colour;
    unsigned char bg_colour_red;
    unsigned char bg_colour_green;
    unsigned char bg_colour_blue;
    unsigned char bg_colour_alpha;

    bool bold;
    bool italic;
} wxd_DataViewItemAttr_t;

typedef wxd_Variant_t* (*wxd_dataview_model_get_value_callback)(void* userdata, uint64_t row,
                                                                uint64_t col);
typedef bool (*wxd_dataview_model_set_value_callback)(void* userdata, const wxd_Variant_t* variant,
                                                      uint64_t row, uint64_t col);
typedef bool (*wxd_dataview_model_get_attr_callback)(void* userdata, uint64_t row, uint64_t col,
                                                     wxd_DataViewItemAttr_t* attr);
typedef bool (*wxd_dataview_model_is_enabled_callback)(void* userdata, uint64_t row, uint64_t col);

/**
 * @brief Creates a new custom virtual list model with callbacks, the returned model has ref count 1,
 * the caller is responsible for releasing it when done with wxd_DataViewModel_Release.
 */
WXD_EXPORTED wxd_DataViewModel_t*
wxd_DataViewVirtualListModel_CreateWithCallbacks(
    uint64_t initial_size, void* userdata, wxd_dataview_model_get_value_callback get_value_callback,
    wxd_dataview_model_set_value_callback set_value_callback,
    wxd_dataview_model_get_attr_callback get_attr_callback,
    wxd_dataview_model_is_enabled_callback is_enabled_callback);

// Custom tree model with callbacks: create a wxDataViewModel subclass that
// forwards GetParent/IsContainer/GetChildren/GetValue/SetValue/IsEnabled/Compare
// to C callbacks supplied in a struct allocated by the caller.

// Callback function types for custom CustomDataViewTreeModel (C-compatible)
typedef void* (*wxd_dataview_tree_model_get_parent_fn)(void* userdata, void* item);
typedef bool (*wxd_dataview_tree_model_is_container_fn)(void* userdata, void* item);
typedef void (*wxd_dataview_tree_model_get_children_fn)(void* userdata, void* item,
                                                        void*** out_items, int* out_count);
typedef void (*wxd_dataview_tree_model_free_children_fn)(void** items, int count);
// Use the C-compatible wxd_Variant_t bridge (same as virtual list helper)
typedef wxd_Variant_t* (*wxd_dataview_tree_model_get_value_fn)(void* userdata, void* item,
                                                               unsigned int col);
typedef bool (*wxd_dataview_tree_model_set_value_fn)(void* userdata, void* item, unsigned int col,
                                                     const wxd_Variant_t* variant);
typedef bool (*wxd_dataview_tree_model_is_enabled_fn)(void* userdata, void* item, unsigned int col);
typedef int (*wxd_dataview_tree_model_compare_fn)(void* userdata, void* item1, void* item2,
                                                  unsigned int col, bool ascending);
// Optional destructor callback for userdata. If provided, this function is
// called with the `userdata` pointer to allow the owner to reclaim it using
// the proper Rust type/destructor across the FFI boundary.
typedef void (*wxd_dataview_tree_model_userdata_free_fn)(void* userdata);

// Struct bundling callbacks. This is the public ABI for custom tree models.
typedef struct wxd_DataViewTreeModel_Callbacks {
    void* userdata;
    wxd_dataview_tree_model_userdata_free_fn userdata_free;
    wxd_dataview_tree_model_get_parent_fn get_parent;
    wxd_dataview_tree_model_is_container_fn is_container;
    wxd_dataview_tree_model_get_children_fn get_children;
    wxd_dataview_tree_model_free_children_fn free_children;
    wxd_dataview_tree_model_get_value_fn get_value;
    wxd_dataview_tree_model_set_value_fn set_value;
    wxd_dataview_tree_model_is_enabled_fn is_enabled;
    wxd_dataview_tree_model_compare_fn compare;
} wxd_DataViewTreeModel_Callbacks;

/**
 * @brief Creates a new custom tree model with callbacks, the returned model has ref count 1,
 * the caller is responsible for releasing it when done with wxd_DataViewModel_Release.
 */
WXD_EXPORTED wxd_DataViewModel_t*
wxd_DataViewTreeModel_CreateWithCallbacks(const wxd_DataViewTreeModel_Callbacks* callbacks);

// Notifications and helpers for custom tree models
WXD_EXPORTED void
wxd_DataViewTreeModel_ItemValueChanged(wxd_DataViewModel_t* model, void* item, unsigned int col);

WXD_EXPORTED void
wxd_DataViewTreeModel_ItemChanged(wxd_DataViewModel_t* model, void* item);

// Notify that a child item was added under a given parent. If parent is null, the child was added under the (invisible) root.
WXD_EXPORTED void
wxd_DataViewTreeModel_ItemAdded(wxd_DataViewModel_t* model, void* parent, void* item);

WXD_EXPORTED void
wxd_DataViewTreeModel_ItemDeleted(wxd_DataViewModel_t* model, void* parent, void* item);

WXD_EXPORTED void
wxd_DataViewTreeModel_ItemsAdded(wxd_DataViewModel_t* model, void* parent, const void* const* items,
                                 size_t count);

WXD_EXPORTED void
wxd_DataViewTreeModel_ItemsDeleted(wxd_DataViewModel_t* model, void* parent,
                                   const void* const* items, size_t count);

WXD_EXPORTED void
wxd_DataViewTreeModel_ItemsChanged(wxd_DataViewModel_t* model, const void* const* items,
                                   size_t count);

WXD_EXPORTED void
wxd_DataViewTreeModel_Cleared(wxd_DataViewModel_t* model);

// DataViewCtrl functions
WXD_EXPORTED wxd_DataViewColumn_t*
wxd_DataViewCtrl_CreateTextColumn(wxd_Window_t* ctrl, const char* label, uint32_t model_column,
                                  wxd_DataViewCellModeCEnum mode, int width,
                                  wxd_AlignmentCEnum align, int flags);

// Setting column properties after creation
WXD_EXPORTED void
wxd_DataViewColumn_SetTitle(wxd_DataViewColumn_t* self, const char* title);

WXD_EXPORTED void
wxd_DataViewColumn_SetResizeable(wxd_DataViewColumn_t* self, bool resizeable);
WXD_EXPORTED bool
wxd_DataViewColumn_IsResizeable(wxd_DataViewColumn_t* self);
WXD_EXPORTED void
wxd_DataViewColumn_SetSortable(wxd_DataViewColumn_t* self, bool sortable);
WXD_EXPORTED bool
wxd_DataViewColumn_IsSortable(wxd_DataViewColumn_t* self);
// TODO: Add other properties like Reorderable, Hidden, Alignment, Width etc. as needed

// Custom Renderer Callbacks
typedef struct {
    int width;
    int height;
} wxd_Size_t;

typedef struct {
    int x;
    int y;
    int width;
    int height;
} wxd_Rect_t;

// Custom renderer callback function types
typedef wxd_Size_t (*wxd_CustomRenderer_GetSizeCallback)(void* user_data);
typedef bool (*wxd_CustomRenderer_RenderCallback)(void* user_data, wxd_Rect_t cell, void* dc,
                                                  int state);
typedef bool (*wxd_CustomRenderer_SetValueCallback)(void* user_data, const wxd_Variant_t* value);
typedef wxd_Variant_t* (*wxd_CustomRenderer_GetValueCallback)(void* user_data);
typedef bool (*wxd_CustomRenderer_HasEditorCtrlCallback)(void* user_data);
typedef void* (*wxd_CustomRenderer_CreateEditorCtrlCallback)(void* user_data, void* parent,
                                                             wxd_Rect_t label_rect,
                                                             const wxd_Variant_t* value);
typedef wxd_Variant_t* (*wxd_CustomRenderer_GetValueFromEditorCtrlCallback)(void* user_data,
                                                                            void* editor);
typedef bool (*wxd_CustomRenderer_ActivateCellCallback)(void* user_data, wxd_Rect_t cell,
                                                        void* model, void* item, unsigned int col,
                                                        void* mouse_event);

// Custom renderer creation
WXD_EXPORTED wxd_DataViewRenderer_t*
wxd_DataViewCustomRenderer_Create(
    const char* varianttype, int64_t mode, int64_t align, void* user_data,
    wxd_CustomRenderer_GetSizeCallback get_size_callback,
    wxd_CustomRenderer_RenderCallback render_callback,
    wxd_CustomRenderer_SetValueCallback set_value_callback,
    wxd_CustomRenderer_GetValueCallback get_value_callback,
    wxd_CustomRenderer_HasEditorCtrlCallback has_editor_callback,
    wxd_CustomRenderer_CreateEditorCtrlCallback create_editor_callback,
    wxd_CustomRenderer_GetValueFromEditorCtrlCallback get_value_from_editor_callback,
    wxd_CustomRenderer_ActivateCellCallback activate_cell_callback);

// Function to release callbacks by renderer ID
WXD_EXPORTED void
wxd_DataViewCustomRenderer_ReleaseCallbacksByKey(int32_t renderer_id);

// Function to release all callbacks for a specific dataview ID (deprecated - kept for compatibility)
WXD_EXPORTED void
wxd_DataViewCustomRenderer_ReleaseAllCallbacksForDataView(int32_t dataview_id);

// Cleanup function for custom renderer callbacks (legacy)
WXD_EXPORTED void
wxd_DataViewCustomRenderer_ReleaseCallbacks(wxd_DataViewRenderer_t* renderer);

// Helper function to free Rust custom renderer callbacks
WXD_EXPORTED void
drop_rust_custom_renderer_callbacks(void* ptr);

// =============================================================================
// DataViewListModel (DataViewListStore) - CRUD Operations
// =============================================================================

// Row count
WXD_EXPORTED uint32_t
wxd_DataViewListModel_GetItemCount(wxd_DataViewModel_t* self);

// Item operations - Adding
WXD_EXPORTED bool
wxd_DataViewListModel_PrependRow(wxd_DataViewModel_t* self);
WXD_EXPORTED bool
wxd_DataViewListModel_InsertRow(wxd_DataViewModel_t* self, uint32_t pos);

// Item operations - Removing
WXD_EXPORTED bool
wxd_DataViewListModel_DeleteItem(wxd_DataViewModel_t* self, uint32_t row);
WXD_EXPORTED bool
wxd_DataViewListModel_DeleteAllItems(wxd_DataViewModel_t* self);

// Get value
WXD_EXPORTED wxd_Variant_t*
wxd_DataViewListModel_GetValue(wxd_DataViewModel_t* self, size_t row, size_t col);

// =============================================================================
// DataViewListCtrl - CRUD Operations
// =============================================================================

// Item operations - Adding (with values)
WXD_EXPORTED bool
wxd_DataViewListCtrl_AppendItem(wxd_Window_t* self, const wxd_Variant_t* const* values,
                                uint32_t count, uintptr_t data);
WXD_EXPORTED bool
wxd_DataViewListCtrl_PrependItem(wxd_Window_t* self, const wxd_Variant_t* const* values,
                                 uint32_t count, uintptr_t data);
WXD_EXPORTED bool
wxd_DataViewListCtrl_InsertItem(wxd_Window_t* self, uint32_t row,
                                const wxd_Variant_t* const* values, uint32_t count, uintptr_t data);

// Item operations - Removing
WXD_EXPORTED bool
wxd_DataViewListCtrl_DeleteItem(wxd_Window_t* self, uint32_t row);
WXD_EXPORTED void
wxd_DataViewListCtrl_DeleteAllItems(wxd_Window_t* self);

// Row count
WXD_EXPORTED uint32_t
wxd_DataViewListCtrl_GetItemCount(wxd_Window_t* self);

// Get/Set values
WXD_EXPORTED void
wxd_DataViewListCtrl_SetValue(wxd_Window_t* self, uint32_t row, uint32_t col,
                              const wxd_Variant_t* value);
WXD_EXPORTED wxd_Variant_t*
wxd_DataViewListCtrl_GetValue(wxd_Window_t* self, uint32_t row, uint32_t col);

// Text convenience methods
WXD_EXPORTED void
wxd_DataViewListCtrl_SetTextValue(wxd_Window_t* self, uint32_t row, uint32_t col, const char* value);
WXD_EXPORTED const char*
wxd_DataViewListCtrl_GetTextValue(wxd_Window_t* self, uint32_t row, uint32_t col);

// Toggle convenience methods
WXD_EXPORTED void
wxd_DataViewListCtrl_SetToggleValue(wxd_Window_t* self, uint32_t row, uint32_t col, bool value);
WXD_EXPORTED bool
wxd_DataViewListCtrl_GetToggleValue(wxd_Window_t* self, uint32_t row, uint32_t col);

// Row/Item conversion
WXD_EXPORTED int32_t
wxd_DataViewListCtrl_ItemToRow(wxd_Window_t* self, const wxd_DataViewItem_t* item);
WXD_EXPORTED wxd_DataViewItem_t*
wxd_DataViewListCtrl_RowToItem(wxd_Window_t* self, int32_t row);

// Selection
WXD_EXPORTED void
wxd_DataViewListCtrl_UnselectRow(wxd_Window_t* self, uint32_t row);
WXD_EXPORTED bool
wxd_DataViewListCtrl_IsRowSelected(wxd_Window_t* self, uint32_t row);

// Item data
WXD_EXPORTED void
wxd_DataViewListCtrl_SetItemData(wxd_Window_t* self, const wxd_DataViewItem_t* item, uintptr_t data);
WXD_EXPORTED uintptr_t
wxd_DataViewListCtrl_GetItemData(wxd_Window_t* self, const wxd_DataViewItem_t* item);

#ifdef __cplusplus
}
#endif

#endif /* WXD_DATAVIEW_H */