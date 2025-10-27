// Purpose: Defines C-compatible item types for wxDragon FFI.
#ifndef WXD_ITEM_H
#define WXD_ITEM_H

#include "../wxd_types.h" // For WXD_EXPORTED and other basic types if needed

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Opaque wrapper type for wxDataViewItem used across the FFI boundary.
 * Defined as an incomplete (opaque) type so consumers treat it as a raw pointer.
 */
typedef struct wxd_DataViewItem_t wxd_DataViewItem_t;

/**
 * @brief Clones the given DataViewItem, returning a new heap-allocated instance.
 * @param item Pointer to the wxd_DataViewItem_t to clone. If null, a new empty item is created.
 * @return Pointer to the new wxDataViewItem
 */
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewItem_Clone(const wxd_DataViewItem_t* item);

/**
 * @brief Checks if the given DataViewItem is valid, i.e. not null and represents a valid item (wxDataViewItem::IsOk()).
 * @param item Pointer to the wxd_DataViewItem_t to check.
 * @return True if the item is valid, false otherwise.
 */
WXD_EXPORTED bool
wxd_DataViewItem_IsOk(const wxd_DataViewItem_t* item);

/**
 * @brief Retrieves the internal ID of the DataViewItem.
 * @param item Pointer to the wxd_DataViewItem_t to query.
 * @return The internal ID (void*) of the item, or nullptr if item is invalid.
 */
WXD_EXPORTED const void*
wxd_DataViewItem_GetID(const wxd_DataViewItem_t* item);

/**
 * @brief Creates a new DataViewItem from the given ID.
 * @param id The internal ID (void*) to wrap in a new DataViewItem.
 * @return Pointer to the new wxd_DataViewItem_t instance. You own the returned pointer and must free it with wxd_DataViewItem_Release().
 */
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewItem_CreateFromID(const void* id);

// Releases the wrapper and the heap-allocated wxDataViewItem it contains (if any).
WXD_EXPORTED void
wxd_DataViewItem_Release(const wxd_DataViewItem_t* item);

#ifdef __cplusplus
}
#endif

#endif // WXD_ITEM_H