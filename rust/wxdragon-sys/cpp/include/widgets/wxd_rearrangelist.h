#ifndef WXD_REARRANGELIST_H
#define WXD_REARRANGELIST_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Creates a wxRearrangeList control.
 * 
 * @param parent The parent window.
 * @param id The window identifier.
 * @param pos The window position.
 * @param size The window size.
 * @param order Array specifying the initial order of items (positive=checked, negative=unchecked).
 * @param orderCount Number of elements in the order array.
 * @param items Array of strings containing the items to show.
 * @param itemsCount Number of items.
 * @param style The window style.
 * @return Pointer to the created control or NULL on failure.
 */
wxd_RearrangeList_t*
wxd_RearrangeList_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                         const int* order, int orderCount, const char** items, int itemsCount,
                         wxd_Style_t style);

/**
 * Gets the current order of items in the list.
 * 
 * @param self The RearrangeList instance.
 * @param orderArray Array to store the current order.
 * @param arraySize Size of the provided array (should match the number of items).
 */
void
wxd_RearrangeList_GetCurrentOrder(wxd_RearrangeList_t* self, int* orderArray, int arraySize);

/**
 * Moves the currently selected item one position up.
 * 
 * @param self The RearrangeList instance.
 * @return true if the item was moved, false otherwise.
 */
bool
wxd_RearrangeList_MoveCurrentUp(wxd_RearrangeList_t* self);

/**
 * Moves the currently selected item one position down.
 * 
 * @param self The RearrangeList instance.
 * @return true if the item was moved, false otherwise.
 */
bool
wxd_RearrangeList_MoveCurrentDown(wxd_RearrangeList_t* self);

/**
 * Checks if the currently selected item can be moved up.
 * 
 * @param self The RearrangeList instance.
 * @return true if the item can be moved up, false otherwise.
 */
bool
wxd_RearrangeList_CanMoveCurrentUp(wxd_RearrangeList_t* self);

/**
 * Checks if the currently selected item can be moved down.
 * 
 * @param self The RearrangeList instance.
 * @return true if the item can be moved down, false otherwise.
 */
bool
wxd_RearrangeList_CanMoveCurrentDown(wxd_RearrangeList_t* self);

/**
 * Gets the index of the currently selected item.
 * 
 * @param self The RearrangeList instance.
 * @return The index of the selected item or -1 if none.
 */
int
wxd_RearrangeList_GetSelection(wxd_RearrangeList_t* self);

/**
 * Sets the selection to the item at the given index.
 * 
 * @param self The RearrangeList instance.
 * @param index The index of the item.
 * @param select true to select, false to deselect.
 */
void
wxd_RearrangeList_SetSelection(wxd_RearrangeList_t* self, int index, bool select);

/**
 * Gets the string at the specified index.
 * 
 * @param self The RearrangeList instance.
 * @param index The index of the item.
 * @param buffer Buffer to store the string.
 * @param bufferSize Size of the buffer.
 * @return The length of the string or -1 on error.
 */
int
wxd_RearrangeList_GetString(wxd_RearrangeList_t* self, int index, char* buffer, size_t bufferSize);

/**
 * Gets the number of items in the list.
 * 
 * @param self The RearrangeList instance.
 * @return The number of items.
 */
unsigned int
wxd_RearrangeList_GetCount(wxd_RearrangeList_t* self);

/**
 * Checks if the item at the given index is checked.
 * 
 * @param self The RearrangeList instance.
 * @param index The index of the item.
 * @return true if checked, false otherwise.
 */
bool
wxd_RearrangeList_IsChecked(wxd_RearrangeList_t* self, int index);

/**
 * Sets the checked state of an item in the list.
 *
 * @param self The RearrangeList instance.
 * @param index The index of the item to check/uncheck (0-based).
 * @param check true to check the item, false to uncheck it.
 */
void
wxd_RearrangeList_Check(wxd_RearrangeList_t* self, unsigned int index, bool check);

#ifdef __cplusplus
}
#endif

#endif // WXD_REARRANGELIST_H