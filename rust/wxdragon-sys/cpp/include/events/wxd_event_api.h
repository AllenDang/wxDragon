#ifndef WXD_EVENT_API_H
#define WXD_EVENT_API_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef int wxEventType;

// --- Event Handling & Data Access --- 
WXD_EXPORTED void wxd_EvtHandler_Bind(
    wxd_EvtHandler_t* handler, 
    WXDEventTypeCEnum eventTypeC,
    void* rust_trampoline_fn,          
    void* rust_closure_ptr             
);

// ID-specific event binding for tools and menu items
WXD_EXPORTED void wxd_EvtHandler_BindWithId(wxd_EvtHandler_t* evt_handler, WXDEventTypeCEnum event_type, int id, void* callback, void* user_data);

WXD_EXPORTED int wxd_Event_GetId(wxd_Event_t* event);
WXD_EXPORTED wxd_Window_t* wxd_Event_GetEventObject(wxd_Event_t* event);
WXD_EXPORTED void wxd_Event_Skip(wxd_Event_t* event, bool skip);
WXD_EXPORTED wxEventType wxd_Event_GetEventType(wxd_Event_t* event);

WXD_EXPORTED int wxd_CommandEvent_GetString(wxd_Event_t* event, char* buffer, int buffer_len);
WXD_EXPORTED bool wxd_CommandEvent_IsChecked(wxd_Event_t* event);
WXD_EXPORTED wxd_Point wxd_MouseEvent_GetPosition(wxd_Event_t* event);
WXD_EXPORTED int wxd_KeyEvent_GetKeyCode(wxd_Event_t* event);
WXD_EXPORTED int wxd_KeyEvent_GetUnicodeKey(wxd_Event_t* event);
WXD_EXPORTED int wxd_CommandEvent_GetInt(wxd_Event_t* event);

WXD_EXPORTED int wxd_ScrollEvent_GetPosition(wxd_Event_t* event);
WXD_EXPORTED int wxd_ScrollEvent_GetOrientation(wxd_Event_t* event);

WXD_EXPORTED int wxd_NotebookEvent_GetSelection(wxd_Event_t* event);
WXD_EXPORTED int wxd_NotebookEvent_GetOldSelection(wxd_Event_t* event);

WXD_EXPORTED int wxd_SplitterEvent_GetSashPosition(wxd_Event_t* event);

WXD_EXPORTED wxd_Colour_t wxd_ColourPickerEvent_GetColour(wxd_Event_t* event);

// Corrected Tree Event Data Accessors
WXD_EXPORTED wxd_TreeItemId_t* wxd_TreeEvent_GetItem(wxd_Event_t* event);
WXD_EXPORTED int wxd_TreeEvent_GetLabel(wxd_Event_t* event, char* buffer, int buffer_len);
WXD_EXPORTED wxd_TreeItemId_t* wxd_TreeEvent_GetOldItem(wxd_Event_t* event);
WXD_EXPORTED int wxd_TreeEvent_IsEditCancelled(wxd_Event_t* event); // Returns bool as int (0 or 1)

WXD_EXPORTED int32_t wxd_ListEvent_GetItemIndex(wxd_Event_t* event);
WXD_EXPORTED int wxd_ListEvent_GetColumn(wxd_Event_t* event);
WXD_EXPORTED int wxd_ListEvent_GetLabel(wxd_Event_t* event, char* buffer, int buffer_len);
WXD_EXPORTED bool wxd_ListEvent_IsEditCancelled(wxd_Event_t* event);

// DataView event accessors
WXD_EXPORTED bool wxd_DataViewEvent_GetColumn(wxd_Event_t* event, int32_t* column);
WXD_EXPORTED bool wxd_DataViewEvent_GetRow(wxd_Event_t* event, int64_t* row);
WXD_EXPORTED bool wxd_DataViewEvent_GetValue(wxd_Event_t* event, wxd_Variant_t* value);
WXD_EXPORTED bool wxd_DataViewEvent_SetValue(wxd_Event_t* event, const wxd_Variant_t* value);
WXD_EXPORTED bool wxd_DataViewEvent_IsEditCancelled(wxd_Event_t* event);

// Rust callback for dropping closure box
WXD_EXPORTED void drop_rust_closure_box(void* ptr);

// Rust callback for cleanup notifier
WXD_EXPORTED void notify_rust_of_cleanup(wxd_Window_t* win_ptr);

// CalendarEvent specific
WXD_EXPORTED wxd_DateTime_t* wxd_CalendarEvent_GetDate(wxd_Event_t* event);

// Event type checking functions - these return non-zero if the event is of the specified type
WXD_EXPORTED int wxd_IsMouseButtonEvent(wxd_Event_t* event);
WXD_EXPORTED int wxd_IsMouseMotionEvent(wxd_Event_t* event);
WXD_EXPORTED int wxd_IsKeyboardEvent(wxd_Event_t* event);
WXD_EXPORTED int wxd_IsSizeEvent(wxd_Event_t* event);

// Gets the event's raw type (for debugging)
WXD_EXPORTED int wxd_Event_GetRawType(wxd_Event_t* event);

// Forward declaration for wxd_TreeItemId_t
// This is already in wxd_types.h, but harmless to have here if it helps bindgen temporarily
// However, it should ideally only be in wxd_types.h
// typedef struct wxd_TreeItemId_s wxd_TreeItemId_t; 
// Commenting out as it should be in wxd_types.h

// The WXDEventTypeCEnum is defined in wxd_types.h, so it should NOT be redefined here.

// --- Event Binding API ---

/// Type for closure callbacks

// Function to get the selected client data from a command event
WXD_EXPORTED void* wxd_CommandEvent_GetClientData(wxd_Event_t* self);

// CheckListBox specific event functions
WXD_EXPORTED int32_t wxd_CheckListBoxEvent_GetSelection(wxd_Event_t* self);

// Notebook specific event functions
WXD_EXPORTED int32_t wxd_NotebookEvent_GetSelection(wxd_Event_t* self);

// --- Idle Event Specific Methods ---
WXD_EXPORTED void wxd_IdleEvent_RequestMore(wxd_Event_t* event, bool needMore);
WXD_EXPORTED bool wxd_IdleEvent_MoreRequested(wxd_Event_t* event);
WXD_EXPORTED void wxd_IdleEvent_SetMode(int mode);
WXD_EXPORTED int wxd_IdleEvent_GetMode();

// Mouse wheel event functions
WXD_EXPORTED int wxd_MouseEvent_GetWheelRotation(wxd_Event_t* event);
WXD_EXPORTED int wxd_MouseEvent_GetWheelDelta(wxd_Event_t* event);

// General veto support for all event types (replaces old close event specific functions)
WXD_EXPORTED bool wxd_Event_CanVeto(wxd_Event_t* event);
WXD_EXPORTED void wxd_Event_Veto(wxd_Event_t* event);
WXD_EXPORTED bool wxd_Event_IsVetoed(wxd_Event_t* event);
WXD_EXPORTED void wxd_Event_SetCanVeto(wxd_Event_t* event, bool can_veto);

#ifdef __cplusplus
}
#endif

#endif // WXD_EVENT_API_H 