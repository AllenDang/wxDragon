#ifndef WXD_EVENT_API_H
#define WXD_EVENT_API_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef int wxEventType;

// Bind event handler with token for later unbinding
WXD_EXPORTED void
wxd_EvtHandler_Bind(wxd_EvtHandler_t* handler, WXDEventTypeCEnum event_type,
                    void* rust_trampoline_fn, void* rust_closure_ptr, size_t token);

// Bind event handler with ID and token
WXD_EXPORTED void
wxd_EvtHandler_BindWithId(wxd_EvtHandler_t* handler, WXDEventTypeCEnum event_type, int id,
                          void* rust_trampoline_fn, void* rust_closure_ptr, size_t token);

// Unbind event handler by token
// Returns true if handler was found and removed, false otherwise
WXD_EXPORTED bool
wxd_EvtHandler_Unbind(wxd_EvtHandler_t* handler, size_t token);

// Unbind all event handlers currently bound to this handler.
// Returns the number of handlers removed.
WXD_EXPORTED size_t
wxd_EvtHandler_UnbindAll(wxd_EvtHandler_t* handler);

WXD_EXPORTED int
wxd_Event_GetId(wxd_Event_t* event);
WXD_EXPORTED wxd_Window_t*
wxd_Event_GetEventObject(wxd_Event_t* event);
WXD_EXPORTED void
wxd_Event_Skip(wxd_Event_t* event, bool skip);
WXD_EXPORTED WXDEventTypeCEnum
wxd_Event_GetEventType(wxd_Event_t* event);

/**
 * Get string from wxCommandEvent.
 * Returns the length of the string (excluding null terminator), if any error, -1 returned.
 * If buffer is non-null and buffer_len > 0, copies up to buffer_len - 1 characters and null-terminates.
 * If buffer is null or buffer_len == 0, does not copy anything.
 */
WXD_EXPORTED int
wxd_CommandEvent_GetString(const wxd_Event_t* event, char* buffer, size_t buffer_len);

WXD_EXPORTED bool
wxd_CommandEvent_IsChecked(wxd_Event_t* event);
WXD_EXPORTED wxd_Point
wxd_MouseEvent_GetPosition(wxd_Event_t* event);
WXD_EXPORTED int
wxd_KeyEvent_GetKeyCode(wxd_Event_t* event);
WXD_EXPORTED int
wxd_KeyEvent_GetUnicodeKey(wxd_Event_t* event);

// Modifier key functions for keyboard events
WXD_EXPORTED bool
wxd_KeyEvent_ControlDown(wxd_Event_t* event);
WXD_EXPORTED bool
wxd_KeyEvent_ShiftDown(wxd_Event_t* event);
WXD_EXPORTED bool
wxd_KeyEvent_AltDown(wxd_Event_t* event);
WXD_EXPORTED bool
wxd_KeyEvent_MetaDown(wxd_Event_t* event);
WXD_EXPORTED bool
wxd_KeyEvent_CmdDown(wxd_Event_t* event);
WXD_EXPORTED int
wxd_CommandEvent_GetInt(wxd_Event_t* event);

WXD_EXPORTED int
wxd_ScrollEvent_GetPosition(wxd_Event_t* event);
WXD_EXPORTED int
wxd_ScrollEvent_GetOrientation(wxd_Event_t* event);

WXD_EXPORTED int
wxd_NotebookEvent_GetSelection(wxd_Event_t* event);
WXD_EXPORTED int
wxd_NotebookEvent_GetOldSelection(wxd_Event_t* event);

WXD_EXPORTED int
wxd_SplitterEvent_GetSashPosition(wxd_Event_t* event);

WXD_EXPORTED wxd_Colour_t
wxd_ColourPickerEvent_GetColour(wxd_Event_t* event);

// Corrected Tree Event Data Accessors
WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeEvent_GetItem(wxd_Event_t* event);

WXD_EXPORTED int
wxd_TreeEvent_GetLabel(wxd_Event_t* event, char* buffer, size_t buffer_len);

WXD_EXPORTED wxd_TreeItemId_t*
wxd_TreeEvent_GetOldItem(wxd_Event_t* event);

WXD_EXPORTED int
wxd_TreeEvent_IsEditCancelled(wxd_Event_t* event); // Returns bool as int (0 or 1)

WXD_EXPORTED int32_t
wxd_ListEvent_GetItemIndex(wxd_Event_t* event);
WXD_EXPORTED int
wxd_ListEvent_GetColumn(wxd_Event_t* event);

/**
 * Get the label from the ListEvent.
 * Returns the length of the label (excluding null terminator), if the event is null, -1 returned.
 * If buffer is non-null and buffer_len > 0, copies up to buffer_len - 1 characters and null-terminates.
 * If buffer is null or buffer_len == 0, does not copy anything.
 */
WXD_EXPORTED int
wxd_ListEvent_GetLabel(const wxd_Event_t* event, char* buffer, size_t buffer_len);

WXD_EXPORTED bool
wxd_ListEvent_IsEditCancelled(wxd_Event_t* event);

// DataView event accessors
WXD_EXPORTED bool
wxd_DataViewEvent_GetColumn(wxd_Event_t* event, int32_t* column);
WXD_EXPORTED bool
wxd_DataViewEvent_GetRow(wxd_Event_t* event, int64_t* row);
WXD_EXPORTED const wxd_DataViewItem_t*
wxd_DataViewEvent_GetItem(wxd_Event_t* event);

/**
 * Get a variant value from the DataViewEvent.
 * Returns a cloned variant; caller is responsible for freeing it.
 */
WXD_EXPORTED wxd_Variant_t*
wxd_DataViewEvent_GetValue(wxd_Event_t* event);

WXD_EXPORTED bool
wxd_DataViewEvent_SetValue(wxd_Event_t* event, const wxd_Variant_t* value);
WXD_EXPORTED bool
wxd_DataViewEvent_IsEditCancelled(wxd_Event_t* event);

// Position accessor for DataView events (returns {-1,-1} if not available)
WXD_EXPORTED wxd_Point
wxd_DataViewEvent_GetPosition(const wxd_Event_t* event);

// Returns true if the sorted column order is ascending (only meaningful for wxEVT_DATAVIEW_COLUMN_SORTED)
WXD_EXPORTED bool
wxd_DataViewEvent_GetSortOrder(const wxd_Event_t* event, bool* ascending);

// TreeListCtrl event accessors
WXD_EXPORTED wxd_Long_t
wxd_TreeListEvent_GetItem(wxd_Event_t* event);
WXD_EXPORTED int
wxd_TreeListEvent_GetColumn(wxd_Event_t* event);
WXD_EXPORTED int
wxd_TreeListEvent_GetOldCheckedState(wxd_Event_t* event);

// Callback implemented in Rust to drop the Box<dyn FnMut(Event)>.
void
drop_rust_event_closure_box(void* ptr);

// Rust callback for cleanup notifier
WXD_EXPORTED void
notify_rust_of_cleanup(wxd_Window_t* win_ptr);

// CalendarEvent specific
WXD_EXPORTED wxd_DateTime_t*
wxd_CalendarEvent_GetDate(wxd_Event_t* event);

// Event type checking functions - these return non-zero if the event is of the specified type
WXD_EXPORTED int
wxd_IsMouseButtonEvent(wxd_Event_t* event);
WXD_EXPORTED int
wxd_IsMouseMotionEvent(wxd_Event_t* event);
WXD_EXPORTED int
wxd_IsKeyboardEvent(wxd_Event_t* event);
WXD_EXPORTED int
wxd_IsSizeEvent(wxd_Event_t* event);

// Gets the event's raw type (for debugging)
WXD_EXPORTED int
wxd_Event_GetRawType(wxd_Event_t* event);

// The WXDEventTypeCEnum is defined in wxd_types.h, so it should NOT be redefined here.

// --- Event Binding API ---

/// Type for closure callbacks

// Function to get the selected client data from a command event
WXD_EXPORTED void*
wxd_CommandEvent_GetClientData(wxd_Event_t* self);

// CheckListBox specific event functions
WXD_EXPORTED int32_t
wxd_CheckListBoxEvent_GetSelection(wxd_Event_t* self);

// Notebook specific event functions
WXD_EXPORTED int32_t
wxd_NotebookEvent_GetSelection(wxd_Event_t* self);

// --- Idle Event Specific Methods ---
WXD_EXPORTED void
wxd_IdleEvent_RequestMore(wxd_Event_t* event, bool needMore);
WXD_EXPORTED bool
wxd_IdleEvent_MoreRequested(wxd_Event_t* event);
WXD_EXPORTED void
wxd_IdleEvent_SetMode(int mode);
WXD_EXPORTED int
wxd_IdleEvent_GetMode();

// Mouse wheel event functions
WXD_EXPORTED int
wxd_MouseEvent_GetWheelRotation(wxd_Event_t* event);
WXD_EXPORTED int
wxd_MouseEvent_GetWheelDelta(wxd_Event_t* event);

// General veto support for all event types (replaces old close event specific functions)
WXD_EXPORTED bool
wxd_Event_CanVeto(wxd_Event_t* event);
WXD_EXPORTED void
wxd_Event_Veto(wxd_Event_t* event);
WXD_EXPORTED bool
wxd_Event_IsVetoed(wxd_Event_t* event);
WXD_EXPORTED void
wxd_Event_SetCanVeto(wxd_Event_t* event, bool can_veto);

// NEW: Menu event specific accessors
WXD_EXPORTED int
wxd_MenuEvent_GetMenuId(wxd_Event_t* event);
WXD_EXPORTED bool
wxd_MenuEvent_IsPopup(wxd_Event_t* event);

// NEW: Context menu event specific accessors
WXD_EXPORTED wxd_Point
wxd_ContextMenuEvent_GetPosition(wxd_Event_t* event);

#ifdef __cplusplus
}
#endif

#endif // WXD_EVENT_API_H