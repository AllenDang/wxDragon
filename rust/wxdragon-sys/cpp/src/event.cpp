#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
// #include "../include/events/wxd_event_api.h" // No longer needed, wxd_Event_t defined in wxd_types.h (via wxdragon.h)
#include <unordered_map>
#include <vector>  // For std::vector used in closureMap
#include <memory> // For std::unique_ptr if we want safer memory management
#include <tuple>  // For std::pair used in map key
#include <wx/event.h>
#include <wx/app.h>
#include <wx/window.h> // For wxCloseEvent
#include <wx/tglbtn.h> // ADDED for wxEVT_TOGGLEBUTTON
#include <wx/treectrl.h> // ADDED: For wxEVT_TREE_* constants
#include <wx/slider.h> // ADDED: For wxEVT_SCROLL_CHANGED etc.
#include <wx/spinctrl.h> // ADDED: For wxEVT_SPINCTRL
#include <wx/spinbutt.h> // ADDED: For wxEVT_SPIN*
#include <wx/notebook.h> // ADDED: For wxEVT_NOTEBOOK_PAGE_CHANGED
#include <wx/splitter.h> // ADDED: For wxEVT_SPLITTER_*
#include <wx/listctrl.h> // ADDED: For wxEVT_LIST_*
#include <wx/clrpicker.h> // ADDED: For wxEVT_COLOURPICKER_CHANGED
#include <wx/dateevt.h> // ADDED: For wxEVT_DATE_CHANGED
#include <wx/treebook.h> // ADDED: For wxEVT_TREEBOOK_*
#include <wx/srchctrl.h> // ADDED: For wxEVT_SEARCHCTRL_SEARCH_BTN, wxEVT_SEARCHCTRL_CANCEL_BTN
#include <wx/hyperlink.h> // ADDED: For wxHyperlinkEvent
#include <wx/calctrl.h> // ADDED: For wxCalendarCtrl events
#include <wx/filepicker.h> // ADDED: For wxEVT_FILEPICKER_CHANGED and wxEVT_DIRPICKER_CHANGED
#include <wx/fontpicker.h> // ADDED: For wxEVT_FONTPICKER_CHANGED
#include <wx/notifmsg.h> // For wxNotificationMessage events
#include <wx/dnd.h> // ADDED: For drag and drop events (wxEVT_BEGIN_DRAG, wxEVT_DROP_TEXT, etc.)
#include <wx/timectrl.h> // ADDED: For wxTimePickerCtrl and wxEVT_TIME_CHANGED
#if wxdUSE_MEDIACTRL
#include <wx/mediactrl.h> // ADDED: For MediaCtrl events
#endif
#include <wx/dataview.h> // ADDED: For DataView events
#include <wx/grid.h>
#if wxdUSE_STC
#include <wx/stc/stc.h> // ADDED: For StyledTextCtrl events
#endif
#include "../src/wxd_utils.h" // For WXD_STR_TO_WX_STRING_UTF8_NULL_OK, etc.
#if wxdUSE_AUI
#include <wx/aui/framemanager.h> // ADDED: For wxEVT_AUI_* constants
#endif
#include <wx/dynarray.h>     // For wxEVT_REARRANGE_LIST
#include <wx/log.h>
#include <wx/utils.h>
#include <wx/rearrangectrl.h> // ADDED: For wxEVT_REARRANGE_LIST
#include <wx/collpane.h> // ADDED: For wxEVT_COLLAPSIBLEPANE_CHANGED
#if wxdUSE_RICHTEXT
#include <wx/richtext/richtextctrl.h> // ADDED: For richtext events
#endif

struct wxd_Event_t { wxEvent* event; };

// --- Internal C++ Structures/Classes (Not exposed in C API) ---

// Define a hash function for std::pair<int, int>
struct PairHash {
    template <class T1, class T2>
    std::size_t operator() (const std::pair<T1, T2>& p) const {
        auto hash1 = std::hash<T1>{}(p.first);
        auto hash2 = std::hash<T2>{}(p.second);
        // Simple combination hash - consider better alternatives if collisions become an issue
        return hash1 ^ (hash2 << 1); 
    }
};

// Structure to hold the Rust closure information
struct RustClosureInfo {
    void* closure_ptr = nullptr;
    // We might need the trampoline pointer here too if it varies,
    // but for now, assume a single global trampoline `rust_event_handler_trampoline`.
    wxd_ClosureCallback rust_trampoline = nullptr; // Store the trampoline func ptr
};

// Forward declarations
class WxdEventHandler;
static wxEventType get_wx_event_type_for_c_enum(WXDEventTypeCEnum c_enum_val);

// Check if an event type typically supports veto (derived from wxNotifyEvent or wxCloseEvent)
static bool IsVetableEventType(wxEventType eventType) {
    // Close events
    if (eventType == wxEVT_CLOSE_WINDOW || 
        eventType == wxEVT_END_SESSION || 
        eventType == wxEVT_QUERY_END_SESSION) {
        return true;
    }
    
    // Tree control events (many are wxNotifyEvent derived)
    if (eventType == wxEVT_TREE_BEGIN_LABEL_EDIT ||
        eventType == wxEVT_TREE_END_LABEL_EDIT ||
        eventType == wxEVT_TREE_SEL_CHANGING ||
        eventType == wxEVT_TREE_ITEM_COLLAPSING ||
        eventType == wxEVT_TREE_ITEM_EXPANDING ||
        eventType == wxEVT_TREE_BEGIN_DRAG ||
        eventType == wxEVT_TREE_BEGIN_RDRAG ||
        eventType == wxEVT_TREE_END_DRAG) {
        return true;
    }
    
    // List control events
    if (eventType == wxEVT_LIST_BEGIN_LABEL_EDIT ||
        eventType == wxEVT_LIST_END_LABEL_EDIT ||
        eventType == wxEVT_LIST_DELETE_ITEM ||
        eventType == wxEVT_LIST_BEGIN_DRAG ||
        eventType == wxEVT_LIST_BEGIN_RDRAG) {
        return true;
    }
    
    // Notebook/book events (only *_CHANGING events are vetable)
    if (eventType == wxEVT_NOTEBOOK_PAGE_CHANGING ||
        eventType == wxEVT_TREEBOOK_PAGE_CHANGING) {
        return true;
    }
    
    // DataView events  
    if (eventType == wxEVT_DATAVIEW_ITEM_EDITING_STARTED ||
        eventType == wxEVT_DATAVIEW_ITEM_COLLAPSING ||
        eventType == wxEVT_DATAVIEW_ITEM_EXPANDING) {
        return true;
    }
    
    // Splitter events (only *_CHANGING events are vetable)
    if (eventType == wxEVT_SPLITTER_SASH_POS_CHANGING) {
        return true;
    }
    
    // AUI events
    #if wxdUSE_AUI
    if (eventType == wxEVT_AUI_PANE_CLOSE) {
        return true;
    }
    #endif
    
    return false;
}

// General function to check if any wxWidgets event was vetoed
static bool IsEventVetoed(wxEvent& event) {
    // Try wxCloseEvent first (handles close events)
    wxCloseEvent* closeEvent = wxDynamicCast(&event, wxCloseEvent);
    if (closeEvent) {
        return closeEvent->GetVeto();
    }
    
    // Try wxNotifyEvent (handles most other vetable events)
    wxNotifyEvent* notifyEvent = wxDynamicCast(&event, wxNotifyEvent);
    if (notifyEvent) {
        return !notifyEvent->IsAllowed(); // IsAllowed() returns false if vetoed
    }
    
    // For other event types that might support veto, add specific checks here
    // Note: Most vetable events derive from wxNotifyEvent or wxCloseEvent
    
    return false; // Event doesn't support veto or wasn't vetoed
}

// ClientData class to hold our handler pointer and ensure deletion
class WxdHandlerClientData : public wxClientData {
public:
    WxdEventHandler* handler; // Pointer to the handler associated with the window

    WxdHandlerClientData(WxdEventHandler* h) : handler(h) {}
    virtual ~WxdHandlerClientData(); // Defined after WxdEventHandler
};

// Custom Event Handler class to connect wx events to Rust closures
class WxdEventHandler : public wxEvtHandler {
public:
    // Map (eventType, widgetId) pair to a vector of Rust closure info
    std::unordered_map<std::pair<wxEventType, wxd_Id>, std::vector<RustClosureInfo>, PairHash> closureMap; 
    // Track whether we've already bound DispatchEvent to wxWidgets for each event key
    std::unordered_map<std::pair<wxEventType, wxd_Id>, bool, PairHash> wx_bindings_made;
    wxd_EvtHandler_t* c_handle = nullptr; // Changed type to wxd_EvtHandler_t*
    wxEvtHandler* ownerHandler = nullptr; // Store the actual wxEvtHandler*

    WxdEventHandler(wxd_EvtHandler_t* handle, wxEvtHandler* owner) : c_handle(handle), ownerHandler(owner) {}

    // Destructor - Now needs to notify Rust to drop closures via drop_rust_closure_box
    ~WxdEventHandler(); // Declaration moved, definition below

    // The new dispatch method that handles multiple closures per event
    void DispatchEvent(wxEvent& event);
    
    // Special dispatch method for close events with correct signature
    void DispatchCloseEvent(wxCloseEvent& event);
};

// Define WxdHandlerClientData destructor (no change needed here, it still just deletes the handler)
WxdHandlerClientData::~WxdHandlerClientData() {
    delete handler;
}

// --- Declare the Rust helper functions ---
// These functions will be implemented in the Rust `wxdragon` crate.
extern "C" {
    // The trampoline function implemented in Rust
    // Its signature MUST match wxd_ClosureCallback in wxdragon.h
    // void rust_event_handler_trampoline(wxd_Event_t* event_ptr, void* user_data);
    
    // Function implemented in Rust to drop the Box<dyn FnMut(Event)>.
    void drop_rust_closure_box(void* ptr);
}

// WxdEventHandler Destructor Implementation
WxdEventHandler::~WxdEventHandler() {
    // wxLogDebug("WxdEventHandler destroying for handler %p. Notifying Rust to drop closures.", ownerHandler);
    for (auto const& [key, closure_vector] : closureMap) {
        for (auto const& info : closure_vector) {
            if (info.closure_ptr) {
                // Tell Rust to drop the Box corresponding to this pointer
                drop_rust_closure_box(info.closure_ptr);
            }
        }
    }
    // Clear the maps (optional, as the handler is being destroyed)
    closureMap.clear();
    wx_bindings_made.clear();
}

// New DispatchEvent method that handles multiple closures per event
void WxdEventHandler::DispatchEvent(wxEvent& event) {
    wxEventType eventType = event.GetEventType();
    wxd_Id id = event.GetId(); // Get the widget ID from the event

    // Create keys for specific ID and wxID_ANY
    std::pair<wxEventType, wxd_Id> key_specific_id = {eventType, id};
    std::pair<wxEventType, wxd_Id> key_any_id = {eventType, wxID_ANY};
    
    bool event_consumed = false;

    // Process Specific ID Handlers first
    auto it_specific = closureMap.find(key_specific_id);
    if (it_specific != closureMap.end()) {
        for (auto const& info : it_specific->second) {
            if (info.closure_ptr && info.rust_trampoline) {
                // Reset skip to true before each handler call
                event.Skip(true);
                
                // Call the Rust trampoline function
                info.rust_trampoline(info.closure_ptr, reinterpret_cast<wxd_Event_t*>(&event));
                
                // Check if this handler consumed the event
                if (!event.GetSkipped()) {
                    event_consumed = true;
                    break; // Stop processing further handlers
                }
            }
        }
    }

    // Process wxID_ANY Handlers (if not already consumed)
    if (!event_consumed) {
        auto it_any = closureMap.find(key_any_id);
        if (it_any != closureMap.end()) {
            for (auto const& info : it_any->second) {
                if (info.closure_ptr && info.rust_trampoline) {
                    // Reset skip to true before each handler call
                    event.Skip(true);
                    
                    // Call the Rust trampoline function
                    info.rust_trampoline(info.closure_ptr, reinterpret_cast<wxd_Event_t*>(&event));
                    
                    // Check if this handler consumed the event
                    if (!event.GetSkipped()) {
                        event_consumed = true;
                        break; // Stop processing further handlers
                    }
                }
            }
        }
    }

    // Set final event state
    if (event_consumed) {
        event.Skip(false);
    } else {
        // General handling for all vetable events
        if (IsEventVetoed(event)) {
            // Event was vetoed, don't allow it to continue to default handlers
            event.Skip(false);
        } else {
            event.Skip(true);
        }
    }
}

// Special dispatch method for close events
void WxdEventHandler::DispatchCloseEvent(wxCloseEvent& event) {
    // Call the general DispatchEvent method
    DispatchEvent(event);
    
    // After processing, if the event was vetoed, we're done
    if (event.GetVeto()) {
        return;
    }
}

// --- C API Implementation --- 

// Gets the handler associated with the wxEvtHandler via client data,
// creating it if it doesn't exist.
WxdEventHandler* GetOrCreateEventHandler(wxEvtHandler* handler, wxd_EvtHandler_t* c_handle) {
    if (!handler) return nullptr;

    WxdHandlerClientData* clientData = static_cast<WxdHandlerClientData*>(handler->GetClientData());
    WxdEventHandler* customHandler = nullptr;

    if (!clientData) {
        // Create the handler
        customHandler = new WxdEventHandler(c_handle, handler);
        // Create the client data wrapper to manage the handler's lifetime
        clientData = new WxdHandlerClientData(customHandler);
        // Associate client data with the wxEvtHandler
        handler->SetClientData(clientData);
        // wxLogDebug("Created WxdEventHandler %p and WxdHandlerClientData %p for wxEvtHandler %p (c_handle %p)", customHandler, clientData, handler, c_handle);
    } else {
        customHandler = clientData->handler;
        // Ensure c_handle is up-to-date (shouldn't change, but good practice)
        if (customHandler) {
            customHandler->c_handle = c_handle; // Update C handle if needed
            customHandler->ownerHandler = handler; // Update owner pointer
        }
    }

    return customHandler;
}

// --- C++ Closure Wrapper (Functor) ---

// A simple functor class to wrap the Rust callback and data pointer.
// Its lifetime is managed by wxWidgets when bound using `wxEvtHandler::Bind`.
// Based on wxRust2 CxxClosureVoid.
class CxxClosureVoid {
public:
    // Type alias for the Rust trampoline function signature
    // It takes the Rust closure data (as void*) and the event pointer (as void*)
    // Note: Argument order might differ from previous attempts, match Rust definition.
    typedef void (*RustTrampolineFn)(void* closure_data, void* event_ptr);

    RustTrampolineFn fn_ptr;    // Pointer to the Rust trampoline function
    void*            param_ptr; // Pointer to the Rust closure Box
    bool             owned_by_wx; // NEW: Flag to track ownership transfer

    // Constructor: Store the Rust pointers, initially not owned by wx
    CxxClosureVoid(void* fn, void* param) : 
        fn_ptr(reinterpret_cast<RustTrampolineFn>(fn)), 
        param_ptr(param), 
        owned_by_wx(false) // Initialize flag
    {
         // wxLogDebug("CxxClosureVoid %p created fn=%p, param=%p, owned=%d", this, fn, param, owned_by_wx);
    }
    
    // Copy Constructor: Also copies the ownership flag state.
    CxxClosureVoid(const CxxClosureVoid& other) : 
        fn_ptr(other.fn_ptr), 
        param_ptr(other.param_ptr), 
        owned_by_wx(other.owned_by_wx) // CORRECT: Copy should inherit ownership state (initially false)
    {
        // wxLogDebug("CxxClosureVoid %p copy constructed from %p (owned=%d)", this, &other, owned_by_wx);
    }
    
    // Destructor: Only the wxWidgets-managed copy should drop the Rust Box.
    ~CxxClosureVoid() {
        // wxLogDebug("CxxClosureVoid %p destroyed. Checking ownership (owned=%d) for param=%p", this, owned_by_wx, param_ptr);
        
        // If owned_by_wx is TRUE, this is the *original* stack-allocated functor
        // whose ownership was transferred to the wxWidgets copy. DO NOT DROP here.
        if (owned_by_wx) {
            // wxLogDebug("CxxClosureVoid %p: Original functor (%p) destroyed, NOT dropping param=%p as owned by wx", this, param_ptr);
            return; // Don't drop if ownership was transferred
        }

        // If owned_by_wx is FALSE, this is either:
        // 1. The wxWidgets-managed copy being destroyed (param_ptr should be valid).
        // 2. The original functor being destroyed *because binding failed* (param_ptr should be valid).
        // In both cases where owned_by_wx is false, we *should* drop the box if the pointer is valid.
        if (param_ptr) {
            // wxLogDebug("CxxClosureVoid %p: Dropping Rust box param=%p as NOT owned by wx", this, param_ptr);
            drop_rust_closure_box(param_ptr);
            param_ptr = nullptr; // Avoid potential double drop if destructor called again somehow
        }
        // else: Warning if param_ptr is null when not owned? Might indicate logic error elsewhere.
    }

    // operator(): This is called by wxWidgets when the event fires.
    // It must accept the specific wxEvent subclass corresponding to the event type
    // it was bound with (e.g., wxCommandEvent&, wxCloseEvent&).
    // We define multiple operator() overloads or use templates if needed,
    // but the trampoline simplifies this: we just need one that takes wxEvent&.
    void operator()(wxEvent& event) {
        if (fn_ptr && param_ptr) {
            fn_ptr(param_ptr, reinterpret_cast<void*>(&event)); 
        } else {
             wxLogWarning("CxxClosureVoid operator() called but fn_ptr or param_ptr is null!");
             event.Skip();
        }
    }
};

// --- C API Implementation ---

extern "C" void wxd_EvtHandler_Bind(
    wxd_EvtHandler_t* handler,
    WXDEventTypeCEnum eventTypeC, 
    void* rust_trampoline_fn, 
    void* rust_closure_ptr   
) {
    wxEvtHandler* wx_handler = reinterpret_cast<wxEvtHandler*>(handler);
    if (!wx_handler) {
         wxLogWarning("wxd_EvtHandler_Bind called with null handler."); 
         if (rust_closure_ptr) { drop_rust_closure_box(rust_closure_ptr); }
         return;
    }

    if (!rust_trampoline_fn || !rust_closure_ptr) {
        wxLogWarning("wxd_EvtHandler_Bind called with null trampoline (%p) or closure (%p).", rust_trampoline_fn, rust_closure_ptr);
        if (rust_closure_ptr) { drop_rust_closure_box(rust_closure_ptr); } // Drop if trampoline is null but closure isn't
        return;
    }

    // Get or create the custom event handler
    WxdEventHandler* customHandler = GetOrCreateEventHandler(wx_handler, handler);
    if (!customHandler) {
        wxLogWarning("wxd_EvtHandler_Bind: Failed to create custom handler.");
        if (rust_closure_ptr) { drop_rust_closure_box(rust_closure_ptr); }
        return;
    }

    // Convert C enum to wxEventType
    wxEventType wx_event_type_to_bind = get_wx_event_type_for_c_enum(eventTypeC);
    if (wx_event_type_to_bind == wxEVT_NULL) {
        wxLogWarning("wxd_EvtHandler_Bind: Unsupported WXDEventTypeCEnum value %d.", (int)eventTypeC);
        if (rust_closure_ptr) { drop_rust_closure_box(rust_closure_ptr); }
        return;
    }

    // Use wxID_ANY for wxd_EvtHandler_Bind (non-ID-specific binding)
    wxd_Id actual_id_for_map_key = wxID_ANY;
    
    // Create map key
    std::pair<wxEventType, wxd_Id> map_key = {wx_event_type_to_bind, actual_id_for_map_key};
    
    // Create Rust closure info
    RustClosureInfo new_rust_info = {rust_closure_ptr, reinterpret_cast<wxd_ClosureCallback>(rust_trampoline_fn)};
    
    // Check if we need to bind DispatchEvent to wxWidgets
    if (!customHandler->wx_bindings_made[map_key]) {
        // For vetable events, use Connect() with higher priority to intercept before default handlers
        if (IsVetableEventType(wx_event_type_to_bind)) {
            if (wx_event_type_to_bind == wxEVT_CLOSE_WINDOW) {
                // Close events need special handler signature
                wx_handler->Connect(wx_event_type_to_bind, 
                                  wxCloseEventHandler(WxdEventHandler::DispatchCloseEvent), 
                                  nullptr, 
                                  customHandler);
            } else {
                // Other vetable events can use general handler
                wx_handler->Connect(wx_event_type_to_bind, 
                                  wxEventHandler(WxdEventHandler::DispatchEvent), 
                                  nullptr, 
                                  customHandler);
            }
        } else {
            // Non-vetable events can use regular Bind()
            wx_handler->Bind(wx_event_type_to_bind, &WxdEventHandler::DispatchEvent, customHandler, wxID_ANY, wxID_ANY);
        }
        customHandler->wx_bindings_made[map_key] = true;
    }
    
    // Add the closure to the vector (do this after binding to ensure cleanup on failure)
    customHandler->closureMap[map_key].push_back(new_rust_info);
}

// ID-specific event binding implementation
extern "C" void wxd_EvtHandler_BindWithId(
    wxd_EvtHandler_t* handler,
    WXDEventTypeCEnum eventTypeC, 
    int id,
    void* rust_trampoline_fn, 
    void* rust_closure_ptr   
) {
    wxEvtHandler* wx_handler = reinterpret_cast<wxEvtHandler*>(handler);
    if (!wx_handler) {
         wxLogWarning("wxd_EvtHandler_BindWithId called with null handler."); 
         if (rust_closure_ptr) { drop_rust_closure_box(rust_closure_ptr); }
         return;
    }

    if (!rust_trampoline_fn || !rust_closure_ptr) {
        wxLogWarning("wxd_EvtHandler_BindWithId called with null trampoline (%p) or closure (%p).", rust_trampoline_fn, rust_closure_ptr);
        if (rust_closure_ptr) { drop_rust_closure_box(rust_closure_ptr); }
        return;
    }

    // Get or create the custom event handler
    WxdEventHandler* customHandler = GetOrCreateEventHandler(wx_handler, handler);
    if (!customHandler) {
        wxLogWarning("wxd_EvtHandler_BindWithId: Failed to create custom handler.");
        if (rust_closure_ptr) { drop_rust_closure_box(rust_closure_ptr); }
        return;
    }

    // Convert C enum to wxEventType
    wxEventType wx_event_type_to_bind = get_wx_event_type_for_c_enum(eventTypeC);
    if (wx_event_type_to_bind == wxEVT_NULL) {
        wxLogWarning("wxd_EvtHandler_BindWithId: Unsupported WXDEventTypeCEnum value %d.", (int)eventTypeC);
        if (rust_closure_ptr) { drop_rust_closure_box(rust_closure_ptr); }
        return;
    }

    // Use the specific ID for ID-specific binding
    wxd_Id actual_id_for_map_key = id;
    
    // Create map key
    std::pair<wxEventType, wxd_Id> map_key = {wx_event_type_to_bind, actual_id_for_map_key};
    
    // Create Rust closure info
    RustClosureInfo new_rust_info = {rust_closure_ptr, reinterpret_cast<wxd_ClosureCallback>(rust_trampoline_fn)};
    
    // Check if we need to bind DispatchEvent to wxWidgets
    if (!customHandler->wx_bindings_made[map_key]) {
        // For vetable events, use Connect() with higher priority to intercept before default handlers
        if (IsVetableEventType(wx_event_type_to_bind)) {
            if (wx_event_type_to_bind == wxEVT_CLOSE_WINDOW) {
                // Close events need special handler signature
                wx_handler->Connect(wx_event_type_to_bind, 
                                  wxCloseEventHandler(WxdEventHandler::DispatchCloseEvent), 
                                  nullptr, 
                                  customHandler);
            } else {
                // Other vetable events can use general handler
                wx_handler->Connect(wx_event_type_to_bind, 
                                  wxEventHandler(WxdEventHandler::DispatchEvent), 
                                  nullptr, 
                                  customHandler);
            }
        } else {
            // Non-vetable events can use regular Bind() with specific ID
            wx_handler->Bind(wx_event_type_to_bind, &WxdEventHandler::DispatchEvent, customHandler, id, id);
        }
        customHandler->wx_bindings_made[map_key] = true;
    }
    
    // Add the closure to the vector (do this after binding to ensure cleanup on failure)
    customHandler->closureMap[map_key].push_back(new_rust_info);
}

// --- Event Accessors (Unchanged) ---

// Implementation for wxd_Event_GetId
wxd_Id wxd_Event_GetId(wxd_Event_t* event) {
    if (!event) return wxID_ANY;
    return ((wxEvent*)event)->GetId();
}

// Implementation for wxd_Event_GetEventObject
wxd_Window_t* wxd_Event_GetEventObject(wxd_Event_t* event) {
    if (!event) return nullptr;
    // GetEventObject returns wxObject*. We need to check if it's a window.
    wxObject* obj = ((wxEvent*)event)->GetEventObject();
    wxWindow* win = wxDynamicCast(obj, wxWindow);
    return reinterpret_cast<wxd_Window_t*>(win);
}

// ADDED: Correct signature for Skip
extern "C" void wxd_Event_Skip(wxd_Event_t* event, bool skip) {
    if (!event) return;
    ((wxEvent*)event)->Skip(skip);
}

// --- NEW: Event Data Accessors Implementation ---

// Accessors for specific event types
WXD_EXPORTED int wxd_CommandEvent_GetString(wxd_Event_t* event, char* buffer, int buffer_len) {
    if (!event || !buffer || buffer_len <= 0) return -1;
    wxCommandEvent* cmdEvent = wxDynamicCast((wxEvent*)event, wxCommandEvent);
    if (!cmdEvent) return -1; // Return -1 if not a command event
    wxString str = cmdEvent->GetString();
    return wxd_cpp_utils::copy_wxstring_to_buffer(str, buffer, (size_t)buffer_len);
}

WXD_EXPORTED bool wxd_CommandEvent_IsChecked(wxd_Event_t* event) {
    if (!event) return false;
    wxEvent* baseEvent = static_cast<wxEvent*>(static_cast<void*>(event)); // Cast via void*
    wxCommandEvent* cmdEvent = dynamic_cast<wxCommandEvent*>(baseEvent);
    if (!cmdEvent) return false; // Not a command event or derived

    return cmdEvent->IsChecked();
}

WXD_EXPORTED wxd_Point wxd_MouseEvent_GetPosition(wxd_Event_t* event) {
    wxd_Point defaultPos = { -1, -1 };
    if (!event) return defaultPos;
    wxEvent* baseEvent = static_cast<wxEvent*>(static_cast<void*>(event)); // Cast via void*
    wxMouseEvent* mouseEvent = dynamic_cast<wxMouseEvent*>(baseEvent);
    if (!mouseEvent) return defaultPos; // Not a mouse event or derived
    
    wxPoint wxPos = mouseEvent->GetPosition();
    wxd_Point pos = { wxPos.x, wxPos.y };
    return pos;
}

WXD_EXPORTED int wxd_KeyEvent_GetKeyCode(wxd_Event_t* event) {
    if (!event) return 0; 
    wxEvent* baseEvent = static_cast<wxEvent*>(static_cast<void*>(event)); // Cast via void*
    wxKeyEvent* keyEvent = dynamic_cast<wxKeyEvent*>(baseEvent);
    if (!keyEvent) return 0; // Not a key event or derived

    return keyEvent->GetKeyCode();
}

WXD_EXPORTED int wxd_KeyEvent_GetUnicodeKey(wxd_Event_t* event) {
    if (!event) return 0; 
    wxEvent* baseEvent = static_cast<wxEvent*>(static_cast<void*>(event)); // Cast via void*
    wxKeyEvent* keyEvent = dynamic_cast<wxKeyEvent*>(baseEvent);
    if (!keyEvent) return 0; // Not a key event or derived

    return keyEvent->GetUnicodeKey();
}

// ADDED: Implementation for wxd_CommandEvent_GetInt
WXD_EXPORTED int wxd_CommandEvent_GetInt(wxd_Event_t* event) {
    if (!event) return 0;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxCommandEvent* command_event = wxDynamicCast(wx_event, wxCommandEvent);
    if (!command_event) return 0;
    
    return command_event->GetInt();
}

// ADDED: Scroll Event Data Accessors
WXD_EXPORTED int wxd_ScrollEvent_GetPosition(wxd_Event_t* event) {
    if (!event) return -1; 
    wxScrollEvent* scrollEvent = wxDynamicCast((wxEvent*)event, wxScrollEvent);
    if (!scrollEvent) {
        wxScrollWinEvent* scrollWinEvent = wxDynamicCast((wxEvent*)event, wxScrollWinEvent);
        if (!scrollWinEvent) return -1;
        return scrollWinEvent->GetPosition();
    }
    return scrollEvent->GetPosition();
}

WXD_EXPORTED int wxd_ScrollEvent_GetOrientation(wxd_Event_t* event) {
    if (!event) return -1; 
    wxScrollEvent* scrollEvent = wxDynamicCast((wxEvent*)event, wxScrollEvent);
    if (!scrollEvent) {
        wxScrollWinEvent* scrollWinEvent = wxDynamicCast((wxEvent*)event, wxScrollWinEvent);
        if (!scrollWinEvent) return -1;
        return scrollWinEvent->GetOrientation();
    }
    return scrollEvent->GetOrientation();
}

// Forward declaration
static wxEventType get_wx_event_type_for_c_enum(WXDEventTypeCEnum c_enum_val);

static WXDEventTypeCEnum get_c_enum_for_wx_event_type(wxEventType wx_event_type) {
    // This is a bit inefficient, but it's simple and should be correct
    for (int i = WXD_EVENT_TYPE_NULL; i < WXD_EVENT_TYPE_MAX; i++) {
        WXDEventTypeCEnum c_enum = static_cast<WXDEventTypeCEnum>(i);
        wxEventType wx_type = get_wx_event_type_for_c_enum(c_enum);
        if (wx_type == wx_event_type) {
            return c_enum;
        }
    }
    // If we can't find a matching C enum, return NULL
    return WXD_EVENT_TYPE_NULL;
}

// Add the implementation of wxd_Event_GetEventType
wxEventType wxd_Event_GetEventType(wxd_Event_t* event) {
    if (!event || !event->event) {
        wxLogWarning("wxd_Event_GetEventType called with null event or null event->event");
        return wxEVT_NULL;
    }
    wxEventType eventType = event->event->GetEventType();
    WXDEventTypeCEnum c_enum_val = get_c_enum_for_wx_event_type(eventType);
    return eventType;
}

// Implement get_wx_event_type_for_c_enum to handle the mapping
static wxEventType get_wx_event_type_for_c_enum(WXDEventTypeCEnum c_enum_val) {
    switch (c_enum_val) {
        case WXD_EVENT_TYPE_COMMAND_BUTTON_CLICKED: return wxEVT_BUTTON;
        case WXD_EVENT_TYPE_CLOSE_WINDOW: return wxEVT_CLOSE_WINDOW;
        case WXD_EVENT_TYPE_CHECKBOX: return wxEVT_CHECKBOX;
        case WXD_EVENT_TYPE_TEXT: return wxEVT_TEXT;
        case WXD_EVENT_TYPE_TEXT_ENTER: return wxEVT_TEXT_ENTER;
        case WXD_EVENT_TYPE_SIZE: return wxEVT_SIZE;
        case WXD_EVENT_TYPE_MENU: return wxEVT_MENU;
        case WXD_EVENT_TYPE_LEFT_DOWN: return wxEVT_LEFT_DOWN;
        case WXD_EVENT_TYPE_LEFT_UP: return wxEVT_LEFT_UP;
        case WXD_EVENT_TYPE_RIGHT_DOWN: return wxEVT_RIGHT_DOWN;
        case WXD_EVENT_TYPE_RIGHT_UP: return wxEVT_RIGHT_UP;
        case WXD_EVENT_TYPE_MIDDLE_DOWN: return wxEVT_MIDDLE_DOWN;
        case WXD_EVENT_TYPE_MIDDLE_UP: return wxEVT_MIDDLE_UP;
        case WXD_EVENT_TYPE_MOTION: return wxEVT_MOTION;
        case WXD_EVENT_TYPE_MOUSEWHEEL: return wxEVT_MOUSEWHEEL;
        case WXD_EVENT_TYPE_ENTER_WINDOW: return wxEVT_ENTER_WINDOW;
        case WXD_EVENT_TYPE_LEAVE_WINDOW: return wxEVT_LEAVE_WINDOW;
        case WXD_EVENT_TYPE_KEY_DOWN: return wxEVT_KEY_DOWN;
        case WXD_EVENT_TYPE_KEY_UP: return wxEVT_KEY_UP;
        case WXD_EVENT_TYPE_CHAR: return wxEVT_CHAR;
        case WXD_EVENT_TYPE_COMMAND_RADIOBUTTON_SELECTED: return wxEVT_RADIOBUTTON;
        case WXD_EVENT_TYPE_COMMAND_RADIOBOX_SELECTED: return wxEVT_RADIOBOX;
        case WXD_EVENT_TYPE_COMMAND_LISTBOX_SELECTED: return wxEVT_LISTBOX;
        case WXD_EVENT_TYPE_COMMAND_CHOICE_SELECTED: return wxEVT_CHOICE;
        case WXD_EVENT_TYPE_COMMAND_COMBOBOX_SELECTED: return wxEVT_COMBOBOX;
        case WXD_EVENT_TYPE_COMMAND_CHECKLISTBOX_SELECTED: return wxEVT_CHECKLISTBOX;
        case WXD_EVENT_TYPE_COMMAND_TOGGLEBUTTON_CLICKED: return wxEVT_TOGGLEBUTTON;
        
        // Tree control events
        case WXD_EVENT_TYPE_TREE_BEGIN_LABEL_EDIT: return wxEVT_TREE_BEGIN_LABEL_EDIT;
        case WXD_EVENT_TYPE_TREE_END_LABEL_EDIT: return wxEVT_TREE_END_LABEL_EDIT;
        case WXD_EVENT_TYPE_TREE_SEL_CHANGED: return wxEVT_TREE_SEL_CHANGED;
        case WXD_EVENT_TYPE_TREE_ITEM_ACTIVATED: return wxEVT_TREE_ITEM_ACTIVATED;
        
        // Slider and spin control events
        case WXD_EVENT_TYPE_SLIDER: return wxEVT_SLIDER;
        case WXD_EVENT_TYPE_SPINCTRL: return wxEVT_SPINCTRL;
        case WXD_EVENT_TYPE_SPIN_UP: return wxEVT_SPIN_UP;
        case WXD_EVENT_TYPE_SPIN_DOWN: return wxEVT_SPIN_DOWN;
        case WXD_EVENT_TYPE_SPIN: return wxEVT_SPIN;
        case WXD_EVENT_TYPE_SPINCTRLDOUBLE: return wxEVT_SPINCTRLDOUBLE;
        
        // Notebook events
        case WXD_EVENT_TYPE_NOTEBOOK_PAGE_CHANGED: return wxEVT_NOTEBOOK_PAGE_CHANGED;
        
        // Splitter events
        case WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGED: return wxEVT_SPLITTER_SASH_POS_CHANGED;
        case WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGING: return wxEVT_SPLITTER_SASH_POS_CHANGING;
        case WXD_EVENT_TYPE_SPLITTER_DOUBLECLICKED: return wxEVT_SPLITTER_DOUBLECLICKED;
        case WXD_EVENT_TYPE_SPLITTER_UNSPLIT: return wxEVT_SPLITTER_UNSPLIT;
        
        // List control events
        case WXD_EVENT_TYPE_LIST_ITEM_SELECTED: return wxEVT_LIST_ITEM_SELECTED;
        case WXD_EVENT_TYPE_LIST_ITEM_ACTIVATED: return wxEVT_LIST_ITEM_ACTIVATED;
        case WXD_EVENT_TYPE_LIST_COL_CLICK: return wxEVT_LIST_COL_CLICK;
        case WXD_EVENT_TYPE_LIST_BEGIN_LABEL_EDIT: return wxEVT_LIST_BEGIN_LABEL_EDIT;
        case WXD_EVENT_TYPE_LIST_END_LABEL_EDIT: return wxEVT_LIST_END_LABEL_EDIT;
        case WXD_EVENT_TYPE_COMMAND_LISTBOX_DOUBLECLICKED: return wxEVT_LISTBOX_DCLICK;
        
        // Picker control events
        case WXD_EVENT_TYPE_COLOURPICKER_CHANGED: return wxEVT_COLOURPICKER_CHANGED;
        case WXD_EVENT_TYPE_DATE_CHANGED: return wxEVT_DATE_CHANGED;
        case WXD_EVENT_TYPE_TIME_CHANGED: return wxEVT_TIME_CHANGED;
        case WXD_EVENT_TYPE_FILEPICKER_CHANGED: return wxEVT_FILEPICKER_CHANGED;
        case WXD_EVENT_TYPE_DIRPICKER_CHANGED: return wxEVT_DIRPICKER_CHANGED;
        case WXD_EVENT_TYPE_FONTPICKER_CHANGED: return wxEVT_FONTPICKER_CHANGED;
        
        // Treebook events
        case WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGED: return wxEVT_TREEBOOK_PAGE_CHANGED;
        case WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGING: return wxEVT_TREEBOOK_PAGE_CHANGING;
        case WXD_EVENT_TYPE_TREEBOOK_NODE_EXPANDED: return wxEVT_TREEBOOK_NODE_EXPANDED;
        case WXD_EVENT_TYPE_TREEBOOK_NODE_COLLAPSED: return wxEVT_TREEBOOK_NODE_COLLAPSED;
        
        // Search control events
        case WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_SEARCH_BTN: return wxEVT_SEARCHCTRL_SEARCH_BTN;
        case WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_CANCEL_BTN: return wxEVT_SEARCHCTRL_CANCEL_BTN;
        
        // Hyperlink events
        case WXD_EVENT_TYPE_COMMAND_HYPERLINK: return wxEVT_HYPERLINK;
        
        // Calendar events
        case WXD_EVENT_TYPE_CALENDAR_SEL_CHANGED: return wxEVT_CALENDAR_SEL_CHANGED;
        case WXD_EVENT_TYPE_CALENDAR_DOUBLECLICKED: return wxEVT_CALENDAR_DOUBLECLICKED;
        case WXD_EVENT_TYPE_CALENDAR_MONTH_CHANGED: return wxEVT_CALENDAR_MONTH_CHANGED;
        case WXD_EVENT_TYPE_CALENDAR_YEAR_CHANGED: return wxEVT_CALENDAR_YEAR_CHANGED;
        case WXD_EVENT_TYPE_CALENDAR_WEEKDAY_CLICKED: return wxEVT_CALENDAR_WEEKDAY_CLICKED;
        
        // Scroll events
        case WXD_EVENT_TYPE_SCROLL_TOP: return wxEVT_SCROLL_TOP;
        case WXD_EVENT_TYPE_SCROLL_BOTTOM: return wxEVT_SCROLL_BOTTOM;
        case WXD_EVENT_TYPE_SCROLL_LINEUP: return wxEVT_SCROLL_LINEUP;
        case WXD_EVENT_TYPE_SCROLL_LINEDOWN: return wxEVT_SCROLL_LINEDOWN;
        case WXD_EVENT_TYPE_SCROLL_PAGEUP: return wxEVT_SCROLL_PAGEUP;
        case WXD_EVENT_TYPE_SCROLL_PAGEDOWN: return wxEVT_SCROLL_PAGEDOWN;
        case WXD_EVENT_TYPE_SCROLL_THUMBTRACK: return wxEVT_SCROLL_THUMBTRACK;
        case WXD_EVENT_TYPE_SCROLL_THUMBRELEASE: return wxEVT_SCROLL_THUMBRELEASE;
        case WXD_EVENT_TYPE_SCROLL_CHANGED: return wxEVT_SCROLL_CHANGED;
        
        // Window events
        case WXD_EVENT_TYPE_DESTROY: return wxEVT_DESTROY;
        case WXD_EVENT_TYPE_MOVE: return wxEVT_MOVE;
        case WXD_EVENT_TYPE_ERASE: return wxEVT_ERASE_BACKGROUND;
        case WXD_EVENT_TYPE_SET_FOCUS: return wxEVT_SET_FOCUS;
        case WXD_EVENT_TYPE_KILL_FOCUS: return wxEVT_KILL_FOCUS;
        case WXD_EVENT_TYPE_PAINT: return wxEVT_PAINT;
        
        // Notification message events
        case WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_CLICK: return wxEVT_NOTIFICATION_MESSAGE_CLICK;
        case WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_DISMISSED: return wxEVT_NOTIFICATION_MESSAGE_DISMISSED;
        case WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_ACTION: return wxEVT_NOTIFICATION_MESSAGE_ACTION;
        
        // Idle event
        case WXD_EVENT_TYPE_IDLE: return wxEVT_IDLE;
        
        // Drag and drop events (some may not exist in all wxWidgets versions)
        // case WXD_EVENT_TYPE_BEGIN_DRAG: return wxEVT_BEGIN_DRAG;  // Not a standard wxWidgets event
        case WXD_EVENT_TYPE_DROP_FILES: return wxEVT_DROP_FILES;
        // case WXD_EVENT_TYPE_DROP_TEXT: return wxEVT_DROP_TEXT;    // Not a standard wxWidgets event
        // case WXD_EVENT_TYPE_END_DRAG: return wxEVT_END_DRAG;      // Not a standard wxWidgets event
        
        // Additional ListCtrl events
        case WXD_EVENT_TYPE_LIST_BEGIN_DRAG: return wxEVT_LIST_BEGIN_DRAG;
        case WXD_EVENT_TYPE_LIST_BEGIN_RDRAG: return wxEVT_LIST_BEGIN_RDRAG;
        case WXD_EVENT_TYPE_LIST_DELETE_ITEM: return wxEVT_LIST_DELETE_ITEM;
        case WXD_EVENT_TYPE_LIST_DELETE_ALL_ITEMS: return wxEVT_LIST_DELETE_ALL_ITEMS;
        case WXD_EVENT_TYPE_LIST_ITEM_DESELECTED: return wxEVT_LIST_ITEM_DESELECTED;
        case WXD_EVENT_TYPE_LIST_ITEM_FOCUSED: return wxEVT_LIST_ITEM_FOCUSED;
        case WXD_EVENT_TYPE_LIST_ITEM_MIDDLE_CLICK: return wxEVT_LIST_ITEM_MIDDLE_CLICK;
        case WXD_EVENT_TYPE_LIST_ITEM_RIGHT_CLICK: return wxEVT_LIST_ITEM_RIGHT_CLICK;
        case WXD_EVENT_TYPE_LIST_KEY_DOWN: return wxEVT_LIST_KEY_DOWN;
        case WXD_EVENT_TYPE_LIST_INSERT_ITEM: return wxEVT_LIST_INSERT_ITEM;
        case WXD_EVENT_TYPE_LIST_COL_RIGHT_CLICK: return wxEVT_LIST_COL_RIGHT_CLICK;
        case WXD_EVENT_TYPE_LIST_COL_BEGIN_DRAG: return wxEVT_LIST_COL_BEGIN_DRAG;
        
        // Media events
        #if wxdUSE_MEDIACTRL
        case WXD_EVENT_TYPE_MEDIA_LOADED: return wxEVT_MEDIA_LOADED;
        case WXD_EVENT_TYPE_MEDIA_STOP: return wxEVT_MEDIA_STOP;
        case WXD_EVENT_TYPE_MEDIA_FINISHED: return wxEVT_MEDIA_FINISHED;
        case WXD_EVENT_TYPE_MEDIA_STATECHANGED: return wxEVT_MEDIA_STATECHANGED;
        case WXD_EVENT_TYPE_MEDIA_PLAY: return wxEVT_MEDIA_PLAY;
        case WXD_EVENT_TYPE_MEDIA_PAUSE: return wxEVT_MEDIA_PAUSE;
        #endif
             
        // DataView events
        case WXD_EVENT_TYPE_DATAVIEW_SELECTION_CHANGED: return wxEVT_DATAVIEW_SELECTION_CHANGED;
        case WXD_EVENT_TYPE_DATAVIEW_ITEM_ACTIVATED: return wxEVT_DATAVIEW_ITEM_ACTIVATED;
        case WXD_EVENT_TYPE_DATAVIEW_ITEM_EDITING_STARTED: return wxEVT_DATAVIEW_ITEM_EDITING_STARTED;
        case WXD_EVENT_TYPE_DATAVIEW_ITEM_EDITING_DONE: return wxEVT_DATAVIEW_ITEM_EDITING_DONE;
        case WXD_EVENT_TYPE_DATAVIEW_ITEM_COLLAPSING: return wxEVT_DATAVIEW_ITEM_COLLAPSING;
        case WXD_EVENT_TYPE_DATAVIEW_ITEM_COLLAPSED: return wxEVT_DATAVIEW_ITEM_COLLAPSED;
        case WXD_EVENT_TYPE_DATAVIEW_ITEM_EXPANDING: return wxEVT_DATAVIEW_ITEM_EXPANDING;
        case WXD_EVENT_TYPE_DATAVIEW_ITEM_EXPANDED: return wxEVT_DATAVIEW_ITEM_EXPANDED;
        case WXD_EVENT_TYPE_DATAVIEW_COLUMN_HEADER_CLICK: return wxEVT_DATAVIEW_COLUMN_HEADER_CLICK;
        case WXD_EVENT_TYPE_DATAVIEW_COLUMN_HEADER_RIGHT_CLICK: return wxEVT_DATAVIEW_COLUMN_HEADER_RIGHT_CLICK;
        case WXD_EVENT_TYPE_DATAVIEW_COLUMN_SORTED: return wxEVT_DATAVIEW_COLUMN_SORTED;
        case WXD_EVENT_TYPE_DATAVIEW_COLUMN_REORDERED: return wxEVT_DATAVIEW_COLUMN_REORDERED;
        
        // Additional TreeCtrl events
        case WXD_EVENT_TYPE_TREE_SEL_CHANGING: return wxEVT_TREE_SEL_CHANGING;
        case WXD_EVENT_TYPE_TREE_ITEM_COLLAPSING: return wxEVT_TREE_ITEM_COLLAPSING;
        case WXD_EVENT_TYPE_TREE_ITEM_COLLAPSED: return wxEVT_TREE_ITEM_COLLAPSED;
        case WXD_EVENT_TYPE_TREE_ITEM_EXPANDING: return wxEVT_TREE_ITEM_EXPANDING;
        case WXD_EVENT_TYPE_TREE_ITEM_EXPANDED: return wxEVT_TREE_ITEM_EXPANDED;
        case WXD_EVENT_TYPE_TREE_ITEM_RIGHT_CLICK: return wxEVT_TREE_ITEM_RIGHT_CLICK;
        case WXD_EVENT_TYPE_TREE_ITEM_MIDDLE_CLICK: return wxEVT_TREE_ITEM_MIDDLE_CLICK;
        case WXD_EVENT_TYPE_TREE_KEY_DOWN: return wxEVT_TREE_KEY_DOWN;
        case WXD_EVENT_TYPE_TREE_DELETE_ITEM: return wxEVT_TREE_DELETE_ITEM;
        case WXD_EVENT_TYPE_TREE_ITEM_MENU: return wxEVT_TREE_ITEM_MENU;
        case WXD_EVENT_TYPE_TREE_BEGIN_DRAG: return wxEVT_TREE_BEGIN_DRAG;
        case WXD_EVENT_TYPE_TREE_BEGIN_RDRAG: return wxEVT_TREE_BEGIN_RDRAG;
        case WXD_EVENT_TYPE_TREE_END_DRAG: return wxEVT_TREE_END_DRAG;
        case WXD_EVENT_TYPE_TREE_STATE_IMAGE_CLICK: return wxEVT_TREE_STATE_IMAGE_CLICK;
        case WXD_EVENT_TYPE_TREE_ITEM_GETTOOLTIP: return wxEVT_TREE_ITEM_GETTOOLTIP;
        
        // Tool events
        // case WXD_EVENT_TYPE_TOOL: return wxEVT_TOOL;  // Conflicts with WXD_EVENT_TYPE_CALENDAR_WEEKDAY_CLICKED (both = 123)
        case WXD_EVENT_TYPE_TOOL_ENTER: return wxEVT_TOOL_ENTER;
        
        // Timer event
        case WXD_EVENT_TYPE_TIMER: return wxEVT_TIMER;
        
        // Special events
        case WXD_EVENT_TYPE_ANY: return wxEVT_ANY;
        
        // AUI Manager event types
        #if wxdUSE_AUI
        case WXD_EVENT_TYPE_AUI_PANE_BUTTON: return wxEVT_AUI_PANE_BUTTON;
        case WXD_EVENT_TYPE_AUI_PANE_CLOSE: return wxEVT_AUI_PANE_CLOSE;
        case WXD_EVENT_TYPE_AUI_PANE_MAXIMIZE: return wxEVT_AUI_PANE_MAXIMIZE;
        case WXD_EVENT_TYPE_AUI_PANE_RESTORE: return wxEVT_AUI_PANE_RESTORE;
        case WXD_EVENT_TYPE_AUI_PANE_ACTIVATED: return wxEVT_AUI_PANE_ACTIVATED;
        case WXD_EVENT_TYPE_AUI_RENDER: return wxEVT_AUI_RENDER;
        #endif
        
        // RearrangeList event
        case WXD_EVENT_TYPE_COMMAND_REARRANGE_LIST: return wxEVT_COMMAND_LISTBOX_SELECTED;
        
        // CollapsiblePane event
        case WXD_EVENT_TYPE_COLLAPSIBLEPANE_CHANGED: return wxEVT_COLLAPSIBLEPANE_CHANGED;
        
        // StyledTextCtrl events - only available when stc feature is enabled
        #if wxdUSE_STC
        case WXD_EVENT_TYPE_STC_CHANGE: return wxEVT_STC_CHANGE;
        case WXD_EVENT_TYPE_STC_STYLENEEDED: return wxEVT_STC_STYLENEEDED;
        case WXD_EVENT_TYPE_STC_CHARADDED: return wxEVT_STC_CHARADDED;
        case WXD_EVENT_TYPE_STC_SAVEPOINTREACHED: return wxEVT_STC_SAVEPOINTREACHED;
        case WXD_EVENT_TYPE_STC_SAVEPOINTLEFT: return wxEVT_STC_SAVEPOINTLEFT;
        case WXD_EVENT_TYPE_STC_ROMODIFYATTEMPT: return wxEVT_STC_ROMODIFYATTEMPT;
        case WXD_EVENT_TYPE_STC_DOUBLECLICK: return wxEVT_STC_DOUBLECLICK;
        case WXD_EVENT_TYPE_STC_UPDATEUI: return wxEVT_STC_UPDATEUI;
        case WXD_EVENT_TYPE_STC_MODIFIED: return wxEVT_STC_MODIFIED;
        case WXD_EVENT_TYPE_STC_MACRORECORD: return wxEVT_STC_MACRORECORD;
        case WXD_EVENT_TYPE_STC_MARGINCLICK: return wxEVT_STC_MARGINCLICK;
        case WXD_EVENT_TYPE_STC_NEEDSHOWN: return wxEVT_STC_NEEDSHOWN;
        case WXD_EVENT_TYPE_STC_PAINTED: return wxEVT_STC_PAINTED;
        case WXD_EVENT_TYPE_STC_USERLISTSELECTION: return wxEVT_STC_USERLISTSELECTION;
        case WXD_EVENT_TYPE_STC_DWELLSTART: return wxEVT_STC_DWELLSTART;
        case WXD_EVENT_TYPE_STC_DWELLEND: return wxEVT_STC_DWELLEND;
        case WXD_EVENT_TYPE_STC_START_DRAG: return wxEVT_STC_START_DRAG;
        case WXD_EVENT_TYPE_STC_DRAG_OVER: return wxEVT_STC_DRAG_OVER;
        case WXD_EVENT_TYPE_STC_DO_DROP: return wxEVT_STC_DO_DROP;
        case WXD_EVENT_TYPE_STC_ZOOM: return wxEVT_STC_ZOOM;
        case WXD_EVENT_TYPE_STC_HOTSPOT_CLICK: return wxEVT_STC_HOTSPOT_CLICK;
        case WXD_EVENT_TYPE_STC_HOTSPOT_DCLICK: return wxEVT_STC_HOTSPOT_DCLICK;
        case WXD_EVENT_TYPE_STC_CALLTIP_CLICK: return wxEVT_STC_CALLTIP_CLICK;
        case WXD_EVENT_TYPE_STC_AUTOCOMP_SELECTION: return wxEVT_STC_AUTOCOMP_SELECTION;
        case WXD_EVENT_TYPE_STC_INDICATOR_CLICK: return wxEVT_STC_INDICATOR_CLICK;
        case WXD_EVENT_TYPE_STC_INDICATOR_RELEASE: return wxEVT_STC_INDICATOR_RELEASE;
        case WXD_EVENT_TYPE_STC_AUTOCOMP_CANCELLED: return wxEVT_STC_AUTOCOMP_CANCELLED;
        case WXD_EVENT_TYPE_STC_AUTOCOMP_CHAR_DELETED: return wxEVT_STC_AUTOCOMP_CHAR_DELETED;
        #endif
        
        // RichText events - only available when richtext feature is enabled
        #if wxdUSE_RICHTEXT
        case WXD_EVENT_TYPE_RICHTEXT_LEFT_CLICK: return wxEVT_RICHTEXT_LEFT_CLICK;
        case WXD_EVENT_TYPE_RICHTEXT_RIGHT_CLICK: return wxEVT_RICHTEXT_RIGHT_CLICK;
        case WXD_EVENT_TYPE_RICHTEXT_MIDDLE_CLICK: return wxEVT_RICHTEXT_MIDDLE_CLICK;
        case WXD_EVENT_TYPE_RICHTEXT_LEFT_DCLICK: return wxEVT_RICHTEXT_LEFT_DCLICK;
        case WXD_EVENT_TYPE_RICHTEXT_RETURN: return wxEVT_RICHTEXT_RETURN;
        case WXD_EVENT_TYPE_RICHTEXT_CHARACTER: return wxEVT_RICHTEXT_CHARACTER;
        case WXD_EVENT_TYPE_RICHTEXT_DELETE: return wxEVT_RICHTEXT_DELETE;
        case WXD_EVENT_TYPE_RICHTEXT_CONTENT_INSERTED: return wxEVT_RICHTEXT_CONTENT_INSERTED;
        case WXD_EVENT_TYPE_RICHTEXT_CONTENT_DELETED: return wxEVT_RICHTEXT_CONTENT_DELETED;
        case WXD_EVENT_TYPE_RICHTEXT_STYLE_CHANGED: return wxEVT_RICHTEXT_STYLE_CHANGED;
        case WXD_EVENT_TYPE_RICHTEXT_SELECTION_CHANGED: return wxEVT_RICHTEXT_SELECTION_CHANGED;
        case WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_CHANGING: return wxEVT_RICHTEXT_STYLESHEET_CHANGING;
        case WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_CHANGED: return wxEVT_RICHTEXT_STYLESHEET_CHANGED;
        case WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_REPLACING: return wxEVT_RICHTEXT_STYLESHEET_REPLACING;
        case WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_REPLACED: return wxEVT_RICHTEXT_STYLESHEET_REPLACED;
        #endif
        
        // TaskBarIcon events - platform-specific support
        #if wxUSE_TASKBARICON
        // Basic mouse events - available on Windows and Linux (constants defined but may not fire on macOS)
        case WXD_EVENT_TYPE_TASKBAR_LEFT_DOWN: 
            #ifdef wxEVT_TASKBAR_LEFT_DOWN
            return wxEVT_TASKBAR_LEFT_DOWN;
            #else
            return wxEVT_NULL; // Fallback for platforms without this event
            #endif
        case WXD_EVENT_TYPE_TASKBAR_LEFT_DCLICK: 
            #ifdef wxEVT_TASKBAR_LEFT_DCLICK
            return wxEVT_TASKBAR_LEFT_DCLICK;
            #else
            return wxEVT_NULL; // Fallback for platforms without this event
            #endif
        
        // Windows-only events - check each constant individually
        case WXD_EVENT_TYPE_TASKBAR_MOVE: 
            #ifdef wxEVT_TASKBAR_MOVE
            return wxEVT_TASKBAR_MOVE;
            #else
            return wxEVT_NULL; // Event not available on this platform
            #endif
        case WXD_EVENT_TYPE_TASKBAR_LEFT_UP: 
            #ifdef wxEVT_TASKBAR_LEFT_UP
            return wxEVT_TASKBAR_LEFT_UP;
            #else
            return wxEVT_NULL; // Event not available on this platform
            #endif
        case WXD_EVENT_TYPE_TASKBAR_RIGHT_DOWN: 
            #ifdef wxEVT_TASKBAR_RIGHT_DOWN
            return wxEVT_TASKBAR_RIGHT_DOWN;
            #else
            return wxEVT_NULL; // Event not available on this platform
            #endif
        case WXD_EVENT_TYPE_TASKBAR_RIGHT_UP: 
            #ifdef wxEVT_TASKBAR_RIGHT_UP
            return wxEVT_TASKBAR_RIGHT_UP;
            #else
            return wxEVT_NULL; // Event not available on this platform
            #endif
        case WXD_EVENT_TYPE_TASKBAR_RIGHT_DCLICK: 
            #ifdef wxEVT_TASKBAR_RIGHT_DCLICK
            return wxEVT_TASKBAR_RIGHT_DCLICK;
            #else
            return wxEVT_NULL; // Event not available on this platform
            #endif
        case WXD_EVENT_TYPE_TASKBAR_BALLOON_TIMEOUT: 
            #ifdef wxEVT_TASKBAR_BALLOON_TIMEOUT
            return wxEVT_TASKBAR_BALLOON_TIMEOUT;
            #else
            return wxEVT_NULL; // Event not available on this platform
            #endif
        case WXD_EVENT_TYPE_TASKBAR_BALLOON_CLICK: 
            #ifdef wxEVT_TASKBAR_BALLOON_CLICK
            return wxEVT_TASKBAR_BALLOON_CLICK;
            #else
            return wxEVT_NULL; // Event not available on this platform
            #endif
        #endif
        
        default: return wxEVT_NULL;
    }
}

// Type checking helper for casting
template<typename T>
T* wxEvent_SafeDynamicCast(wxd_Event_t* event) {
    if (!event) return nullptr;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    return dynamic_cast<T*>(wx_event);
}

// --- Event Type Checking Functions ---

extern "C" int wxd_IsMouseButtonEvent(wxd_Event_t* event) {
    if (!event) return 0;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxEventType type = wx_event->GetEventType();
    
    // Check if it's any of the mouse button event types
    return (type == wxEVT_LEFT_DOWN || 
            type == wxEVT_LEFT_UP || 
            type == wxEVT_RIGHT_DOWN || 
            type == wxEVT_RIGHT_UP || 
            type == wxEVT_MIDDLE_DOWN || 
            type == wxEVT_MIDDLE_UP || 
            type == wxEVT_MOUSEWHEEL) ? 1 : 0;
}

extern "C" int wxd_IsMouseMotionEvent(wxd_Event_t* event) {
    if (!event) return 0;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxEventType type = wx_event->GetEventType();
    
    // Check if it's a mouse motion event
    return (type == wxEVT_MOTION) ? 1 : 0;
}

extern "C" int wxd_IsKeyboardEvent(wxd_Event_t* event) {
    if (!event) return 0;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxEventType type = wx_event->GetEventType();
    
    // Check if it's any of the keyboard event types
    return (type == wxEVT_KEY_DOWN || 
            type == wxEVT_KEY_UP || 
            type == wxEVT_CHAR) ? 1 : 0;
}

extern "C" int wxd_IsSizeEvent(wxd_Event_t* event) {
    if (!event) return 0;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    
    // Check if it's a size event
    return (wx_event->GetEventType() == wxEVT_SIZE) ? 1 : 0;
}

extern "C" int wxd_Event_GetRawType(wxd_Event_t* event) {
    if (!event) return -1;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    return static_cast<int>(wx_event->GetEventType());
}

// --- CommandEvent specific ---
WXD_EXPORTED void* wxd_CommandEvent_GetClientData(wxd_Event_t* self) {
    if (!self) return nullptr;
    wxEvent* baseEvent = reinterpret_cast<wxEvent*>(self);
    wxCommandEvent* cmdEvent = wxDynamicCast(baseEvent, wxCommandEvent);
    if (!cmdEvent) return nullptr;
    return cmdEvent->GetClientData();
}

// --- CheckListBox specific --- 
WXD_EXPORTED int32_t wxd_CheckListBoxEvent_GetSelection(wxd_Event_t* self) {
    if (!self) return -1;
    wxEvent* baseEvent = reinterpret_cast<wxEvent*>(self);
    wxCommandEvent* cmdEvent = wxDynamicCast(baseEvent, wxCommandEvent);
    if (!cmdEvent) return -1;
    // For list-like controls, GetInt() often returns the selection index.
    // wxCheckListBox emits wxEVT_LISTBOX, which uses GetInt() for selection.
    return cmdEvent->GetInt(); 
}

// --- Notebook specific ---
// This is already implemented in notebook.cpp
/*
WXD_EXPORTED int32_t wxd_NotebookEvent_GetSelection(wxd_Event_t* self) {
    if (!self) return -1;
    wxNotebookEvent* notebookEvent = static_cast<wxNotebookEvent*>(self);
    return notebookEvent->GetSelection();
}
*/

// --- DataView Event Accessors ---

// Header: WXD_EXPORTED bool wxd_DataViewEvent_GetColumn(wxd_Event_t* event, int32_t* column);
WXD_EXPORTED bool wxd_DataViewEvent_GetColumn(wxd_Event_t* event, int32_t* column)
{
    if (!event || !column) return false;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxDataViewEvent* dve = dynamic_cast<wxDataViewEvent*>(wx_event);
    if (!dve) return false;
    
    *column = dve->GetColumn();
    return true;
}

// Header: WXD_EXPORTED bool wxd_DataViewEvent_GetRow(wxd_Event_t* event, int64_t* row);
WXD_EXPORTED bool wxd_DataViewEvent_GetRow(wxd_Event_t* event, int64_t* row) {
    if (!event || !row) return false;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxDataViewEvent* dve = dynamic_cast<wxDataViewEvent*>(wx_event);
    if (!dve) return false;
    
    wxDataViewItem item = dve->GetItem();
    if (!item.IsOk()) return false;
    
    // Get the model from the event - this is the correct API in wxWidgets 3.2.8
    wxDataViewModel* model = dve->GetModel();
    if (model) {
        // Try casting to virtual list model first
        wxDataViewVirtualListModel* vmodel = dynamic_cast<wxDataViewVirtualListModel*>(model);
        if (vmodel) {
            // For virtual list models, use the model's GetRow method
            *row = static_cast<int64_t>(vmodel->GetRow(item));
            return true;
        }
        
        // Try casting to regular list model
        wxDataViewIndexListModel* listModel = dynamic_cast<wxDataViewIndexListModel*>(model);
        if (listModel) {
            // For regular list models, use the model's GetRow method
            *row = static_cast<int64_t>(listModel->GetRow(item));
            return true;
        }
    }
    
    // Fall back to manual decoding for other model types
    // Regular models encode row index as (row + 1) in the item ID
    if (item.GetID()) {
        *row = static_cast<int64_t>(reinterpret_cast<uintptr_t>(item.GetID()) - 1);
        return true;
    }
    
    // If we can't determine the row, return false instead of 0
    return false;
}

// Header: WXD_EXPORTED bool wxd_DataViewEvent_GetValue(wxd_Event_t* event, wxd_Variant_t* value);
WXD_EXPORTED bool wxd_DataViewEvent_GetValue(wxd_Event_t* event, wxd_Variant_t* value)
{
    if (!event || !value) return false;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxDataViewEvent* dve = dynamic_cast<wxDataViewEvent*>(wx_event);
    if (!dve) return false;
    
    wxVariant var = dve->GetValue();
    // Copy variant data - this needs proper implementation based on wxd_Variant_t structure
    // For now, just mark as valid
    return true;
}

// Header: WXD_EXPORTED bool wxd_DataViewEvent_SetValue(wxd_Event_t* event, const wxd_Variant_t* value);
WXD_EXPORTED bool wxd_DataViewEvent_SetValue(wxd_Event_t* event, const wxd_Variant_t* value) {
    if (!event || !value) return false;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxDataViewEvent* dve = dynamic_cast<wxDataViewEvent*>(wx_event);
    if (!dve) return false;
    
    // Convert wxd_Variant_t to wxVariant - needs proper implementation
    // For now, just return success
    return true;
}

// Header: WXD_EXPORTED bool wxd_DataViewEvent_IsEditCancelled(wxd_Event_t* event);
WXD_EXPORTED bool wxd_DataViewEvent_IsEditCancelled(wxd_Event_t* event)
{
    if (!event) return true;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxDataViewEvent* dve = dynamic_cast<wxDataViewEvent*>(wx_event);
    if (!dve) return true;
    
    return dve->IsEditCancelled();
}

// --- Idle Event Implementation ---

WXD_EXPORTED void wxd_IdleEvent_RequestMore(wxd_Event_t* event, bool needMore) {
    if (!event) return;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxIdleEvent* idle_event = dynamic_cast<wxIdleEvent*>(wx_event);
    if (!idle_event) return;
    
    idle_event->RequestMore(needMore);
}

WXD_EXPORTED bool wxd_IdleEvent_MoreRequested(wxd_Event_t* event) {
    if (!event) return false;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxIdleEvent* idle_event = dynamic_cast<wxIdleEvent*>(wx_event);
    if (!idle_event) return false;
    
    return idle_event->MoreRequested();
}

WXD_EXPORTED void wxd_IdleEvent_SetMode(int mode) {
    wxIdleEvent::SetMode(static_cast<wxIdleMode>(mode));
}

WXD_EXPORTED int wxd_IdleEvent_GetMode() {
    return static_cast<int>(wxIdleEvent::GetMode());
}

// Mouse wheel event functions
WXD_EXPORTED int wxd_MouseEvent_GetWheelRotation(wxd_Event_t* event) {
    if (!event) return 0;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxMouseEvent* mouse_event = wxDynamicCast(wx_event, wxMouseEvent);
    if (!mouse_event) return 0;
    
    return mouse_event->GetWheelRotation();
}

WXD_EXPORTED int wxd_MouseEvent_GetWheelDelta(wxd_Event_t* event) {
    if (!event) return 120; // Default wheel delta
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    wxMouseEvent* mouse_event = wxDynamicCast(wx_event, wxMouseEvent);
    if (!mouse_event) return 120;
    
    return mouse_event->GetWheelDelta();
}

// General veto support for all event types (replaces old close event specific functions)
WXD_EXPORTED bool wxd_Event_CanVeto(wxd_Event_t* event) {
    if (!event) return false;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    
    // Try wxCloseEvent first
    wxCloseEvent* close_event = wxDynamicCast(wx_event, wxCloseEvent);
    if (close_event) {
        return close_event->CanVeto();
    }
    
    // Try wxNotifyEvent (most other vetable events derive from this)
    wxNotifyEvent* notify_event = wxDynamicCast(wx_event, wxNotifyEvent);
    if (notify_event) {
        return true; // wxNotifyEvent always supports veto
    }
    
    return false; // Event doesn't support veto
}

WXD_EXPORTED void wxd_Event_Veto(wxd_Event_t* event) {
    if (!event) return;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    
    // Try wxCloseEvent first
    wxCloseEvent* close_event = wxDynamicCast(wx_event, wxCloseEvent);
    if (close_event) {
        close_event->Veto();
        return;
    }
    
    // Try wxNotifyEvent
    wxNotifyEvent* notify_event = wxDynamicCast(wx_event, wxNotifyEvent);
    if (notify_event) {
        notify_event->Veto();
        return;
    }
}

WXD_EXPORTED bool wxd_Event_IsVetoed(wxd_Event_t* event) {
    if (!event) return false;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    
    return IsEventVetoed(*wx_event);
}

WXD_EXPORTED void wxd_Event_SetCanVeto(wxd_Event_t* event, bool can_veto) {
    if (!event) return;
    wxEvent* wx_event = reinterpret_cast<wxEvent*>(event);
    
    // Try wxCloseEvent first
    wxCloseEvent* close_event = wxDynamicCast(wx_event, wxCloseEvent);
    if (close_event) {
        close_event->SetCanVeto(can_veto);
        return;
    }
    
    // Try wxNotifyEvent - note that wxNotifyEvent doesn't have SetCanVeto
    // but it's always vetable, so we don't need to do anything for it
    wxNotifyEvent* notify_event = wxDynamicCast(wx_event, wxNotifyEvent);
    if (notify_event) {
        // wxNotifyEvent doesn't have SetCanVeto method, it's always vetable
        // No action needed for wxNotifyEvent-derived events
        return;
    }
}