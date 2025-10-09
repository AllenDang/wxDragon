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
WXD_EXPORTED const wxd_DataViewItem_t* wxd_DataViewItem_Clone(const wxd_DataViewItem_t* item);

/**
 * @brief Checks if the given DataViewItem is valid, i.e. not null and represents a valid item (wxDataViewItem::IsOk()).
 * @param item Pointer to the wxd_DataViewItem_t to check.
 * @return True if the item is valid, false otherwise.
 */
WXD_EXPORTED bool wxd_DataViewItem_IsOk(const wxd_DataViewItem_t* item);

// Releases the wrapper and the heap-allocated wxDataViewItem it contains (if any).
WXD_EXPORTED void wxd_DataViewItem_Release(const wxd_DataViewItem_t* item);

#ifdef __cplusplus
}
#endif

#endif // WXD_ITEM_H 