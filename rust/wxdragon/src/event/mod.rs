//! Safe wrappers for wxWidgets events.

use crate::geometry::Point;
use crate::window::Window;
use std::boxed::Box;
use std::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;
use wxdragon_sys as ffi;
pub mod button_events;
pub mod event_data;
pub mod macros;
pub mod scroll_events;
pub mod text_events;
pub mod tree_events;
pub mod window_events;

// Re-export window events for easier access
pub use window_events::{
    KeyboardEvent, MouseButtonEvent, MouseEnterEvent, MouseLeaveEvent, MouseMotionEvent, WindowEvent, WindowEventData, WindowEvents,
    WindowSizeEvent,
};

// Re-export button events for easier access
pub use button_events::{ButtonEvent, ButtonEventData, ButtonEvents};

// Re-export text events for easier access
pub use text_events::{TextEvent, TextEventData, TextEvents};

// Re-export tree events for easier access
pub use tree_events::{TreeEvent, TreeEventData, TreeEvents};

// Re-export scroll events for easier access
pub use scroll_events::{ScrollEvent, ScrollEventData, ScrollEvents};

// Re-export the stable C enum for use in the safe wrapper
pub use ffi::WXDEventTypeCEnum;

// --- EventType Enum ---

/// Represents a wxDragon event type using stable C enum values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // Ensures memory layout matches the underlying C enum integer type
pub struct EventType(ffi::WXDEventTypeCEnum); // Use the generated C enum type

impl EventType {
    // Constants map directly to the stable C enum values
    pub const COMMAND_BUTTON_CLICKED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_BUTTON_CLICKED);
    pub const CLOSE_WINDOW: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CLOSE_WINDOW);
    pub const CHECKBOX: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CHECKBOX);
    pub const TEXT: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TEXT);
    pub const TEXT_ENTER: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TEXT_ENTER);
    pub const SIZE: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SIZE);
    pub const MENU: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MENU);
    pub const LEFT_DOWN: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LEFT_DOWN);
    pub const LEFT_UP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LEFT_UP);
    pub const RIGHT_DOWN: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RIGHT_DOWN);
    pub const RIGHT_UP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RIGHT_UP);
    pub const MIDDLE_DOWN: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MIDDLE_DOWN);
    pub const MIDDLE_UP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MIDDLE_UP);
    pub const MOTION: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MOTION);
    pub const MOUSEWHEEL: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MOUSEWHEEL);
    pub const ENTER_WINDOW: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_ENTER_WINDOW);
    pub const LEAVE_WINDOW: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LEAVE_WINDOW);
    pub const KEY_DOWN: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_KEY_DOWN);
    pub const KEY_UP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_KEY_UP);
    pub const CHAR: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CHAR);
    pub const COMMAND_RADIOBUTTON_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_RADIOBUTTON_SELECTED);
    pub const COMMAND_RADIOBOX_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_RADIOBOX_SELECTED);
    pub const COMMAND_LISTBOX_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_LISTBOX_SELECTED);
    pub const COMMAND_CHOICE_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_CHOICE_SELECTED);
    pub const COMMAND_COMBOBOX_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_COMBOBOX_SELECTED);
    pub const COMMAND_CHECKLISTBOX_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_CHECKLISTBOX_SELECTED);
    pub const COMMAND_LISTBOX_DOUBLECLICKED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_LISTBOX_DOUBLECLICKED);
    pub const COMMAND_TOGGLEBUTTON_CLICKED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_TOGGLEBUTTON_CLICKED);
    // ADDED: RearrangeList event type
    pub const COMMAND_REARRANGE_LIST: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_REARRANGE_LIST);
    // ADDED: CollapsiblePane event type
    pub const COLLAPSIBLEPANE_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COLLAPSIBLEPANE_CHANGED);
    // ADDED: TreeCtrl event types
    pub const TREE_BEGIN_LABEL_EDIT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_BEGIN_LABEL_EDIT);
    pub const TREE_END_LABEL_EDIT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_END_LABEL_EDIT);
    pub const TREE_SEL_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_SEL_CHANGED);
    pub const TREE_ITEM_ACTIVATED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_ACTIVATED);
    // ADDED: Slider event type
    pub const SLIDER: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SLIDER);
    // ADDED: SpinCtrl event type
    pub const SPINCTRL: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPINCTRL);
    // ADDED: SpinButton event types
    pub const SPIN_UP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPIN_UP);
    pub const SPIN_DOWN: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPIN_DOWN);
    pub const SPIN: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPIN);
    // ADDED: Notebook event type
    pub const NOTEBOOK_PAGE_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NOTEBOOK_PAGE_CHANGED);
    // ADDED: Splitter event types
    pub const SPLITTER_SASH_POS_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGED);
    pub const SPLITTER_SASH_POS_CHANGING: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGING);
    pub const SPLITTER_DOUBLECLICKED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPLITTER_DOUBLECLICKED);
    pub const SPLITTER_UNSPLIT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPLITTER_UNSPLIT);
    // ADDED: ListCtrl event types
    pub const LIST_ITEM_SELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_SELECTED);
    pub const LIST_ITEM_ACTIVATED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_ACTIVATED);
    pub const LIST_COL_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_COL_CLICK);
    pub const LIST_BEGIN_LABEL_EDIT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_BEGIN_LABEL_EDIT);
    pub const LIST_END_LABEL_EDIT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_END_LABEL_EDIT);
    // ADDED: Additional ListCtrl event types
    pub const LIST_BEGIN_DRAG: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_BEGIN_DRAG);
    pub const LIST_BEGIN_RDRAG: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_BEGIN_RDRAG);
    pub const LIST_DELETE_ITEM: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_DELETE_ITEM);
    pub const LIST_DELETE_ALL_ITEMS: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_DELETE_ALL_ITEMS);
    pub const LIST_ITEM_DESELECTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_DESELECTED);
    pub const LIST_ITEM_FOCUSED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_FOCUSED);
    pub const LIST_ITEM_MIDDLE_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_MIDDLE_CLICK);
    pub const LIST_ITEM_RIGHT_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_RIGHT_CLICK);
    pub const LIST_KEY_DOWN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_KEY_DOWN);
    pub const LIST_INSERT_ITEM: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_INSERT_ITEM);
    pub const LIST_COL_RIGHT_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_COL_RIGHT_CLICK);
    pub const LIST_COL_BEGIN_DRAG: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_COL_BEGIN_DRAG);
    // ADDED: ColourPickerCtrl event type
    pub const COLOURPICKER_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COLOURPICKER_CHANGED);
    // DatePicker Event
    pub const DATE_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATE_CHANGED);
    // TimePicker Event
    pub const TIME_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TIME_CHANGED);
    // Treebook Events (match WXDEventTypeCEnum values)
    pub const TREEBOOK_PAGE_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGED);
    pub const TREEBOOK_PAGE_CHANGING: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGING);
    pub const TREEBOOK_NODE_EXPANDED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREEBOOK_NODE_EXPANDED);
    pub const TREEBOOK_NODE_COLLAPSED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREEBOOK_NODE_COLLAPSED);
    // ADDED: SearchCtrl Event Types
    pub const COMMAND_SEARCHCTRL_SEARCH_BTN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_SEARCH_BTN);
    pub const COMMAND_SEARCHCTRL_CANCEL_BTN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_CANCEL_BTN);
    pub const COMMAND_HYPERLINK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_HYPERLINK);
    pub const SPINCTRLDOUBLE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPINCTRLDOUBLE);
    // ADDED: Calendar Control Event Type
    pub const CALENDAR_SEL_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CALENDAR_SEL_CHANGED);
    // ADDED: Missing Calendar Control Event Types
    pub const CALENDAR_DOUBLECLICKED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CALENDAR_DOUBLECLICKED);
    pub const CALENDAR_MONTH_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CALENDAR_MONTH_CHANGED);
    pub const CALENDAR_YEAR_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CALENDAR_YEAR_CHANGED);
    pub const CALENDAR_WEEKDAY_CLICKED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CALENDAR_WEEKDAY_CLICKED);
    // ADDED: ScrollBar Events
    pub const SCROLL_TOP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_TOP);
    pub const SCROLL_BOTTOM: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_BOTTOM);
    pub const SCROLL_LINEUP: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_LINEUP);
    pub const SCROLL_LINEDOWN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_LINEDOWN);
    pub const SCROLL_PAGEUP: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_PAGEUP);
    pub const SCROLL_PAGEDOWN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_PAGEDOWN);
    pub const SCROLL_THUMBTRACK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_THUMBTRACK);
    pub const SCROLL_THUMBRELEASE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_THUMBRELEASE);
    pub const SCROLL_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_CHANGED);
    pub const FILE_PICKER_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_FILEPICKER_CHANGED);
    pub const DIR_PICKER_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DIRPICKER_CHANGED);
    pub const FONT_PICKER_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_FONTPICKER_CHANGED);

    pub const NOTIFICATION_MESSAGE_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_CLICK);
    pub const NOTIFICATION_MESSAGE_DISMISSED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_DISMISSED);
    pub const NOTIFICATION_MESSAGE_ACTION: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_ACTION);

    // Media events
    pub const MEDIA_LOADED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_LOADED);
    pub const MEDIA_STOP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_STOP);
    pub const MEDIA_FINISHED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_FINISHED);
    pub const MEDIA_STATECHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_STATECHANGED);
    pub const MEDIA_PLAY: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_PLAY);
    pub const MEDIA_PAUSE: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_PAUSE);

    pub const EVT_DATE_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATE_CHANGED);

    pub const IDLE: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_IDLE);

    // Drag and drop events
    pub const DROP_FILES: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DROP_FILES);

    pub const PAINT: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_PAINT);

    pub const DESTROY: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DESTROY);

    // Additional window events
    pub const MOVE: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MOVE);
    pub const ERASE: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_ERASE);
    pub const SET_FOCUS: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SET_FOCUS);
    pub const KILL_FOCUS: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_KILL_FOCUS);

    // DataView events
    pub const DATAVIEW_SELECTION_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_SELECTION_CHANGED);
    pub const DATAVIEW_ITEM_ACTIVATED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_ACTIVATED);
    pub const DATAVIEW_ITEM_EDITING_STARTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_EDITING_STARTED);
    pub const DATAVIEW_ITEM_EDITING_DONE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_EDITING_DONE);
    pub const DATAVIEW_ITEM_COLLAPSING: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_COLLAPSING);
    pub const DATAVIEW_ITEM_COLLAPSED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_COLLAPSED);
    pub const DATAVIEW_ITEM_EXPANDING: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_EXPANDING);
    pub const DATAVIEW_ITEM_EXPANDED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_EXPANDED);
    pub const DATAVIEW_COLUMN_HEADER_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_COLUMN_HEADER_CLICK);
    pub const DATAVIEW_COLUMN_HEADER_RIGHT_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_COLUMN_HEADER_RIGHT_CLICK);
    pub const DATAVIEW_COLUMN_SORTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_COLUMN_SORTED);
    pub const DATAVIEW_COLUMN_REORDERED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_COLUMN_REORDERED);

    // ADDED: New TreeCtrl Event Types (complementing 22-25)
    pub const TREE_SEL_CHANGING: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_SEL_CHANGING);
    pub const TREE_ITEM_COLLAPSING: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_COLLAPSING);
    pub const TREE_ITEM_COLLAPSED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_COLLAPSED);
    pub const TREE_ITEM_EXPANDING: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_EXPANDING);
    pub const TREE_ITEM_EXPANDED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_EXPANDED);
    pub const TREE_ITEM_RIGHT_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_RIGHT_CLICK);
    pub const TREE_ITEM_MIDDLE_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_MIDDLE_CLICK);
    pub const TREE_KEY_DOWN: EventType = // Specific to TreeCtrl
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_KEY_DOWN);
    pub const TREE_DELETE_ITEM: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_DELETE_ITEM);
    pub const TREE_ITEM_MENU: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_MENU);
    pub const TREE_BEGIN_DRAG: EventType = // Specific to TreeCtrl
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_BEGIN_DRAG);
    pub const TREE_BEGIN_RDRAG: EventType = // Specific to TreeCtrl
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_BEGIN_RDRAG);
    pub const TREE_END_DRAG: EventType = // Specific to TreeCtrl
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_END_DRAG);
    pub const TREE_STATE_IMAGE_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_STATE_IMAGE_CLICK);

    // ToolBar Events
    pub const TOOL: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TOOL);
    pub const TOOL_ENTER: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TOOL_ENTER);

    // TreeCtrl Events
    pub const TREE_ITEM_GETTOOLTIP: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_GETTOOLTIP);

    // Generic events that might not fit a specific category or are widely used
    pub const ANY: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_ANY);

    // Special event type for null/None, not a real wxWidgets event type
    pub const NONE: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NULL); // Assuming NULL is 0

    // AuiManager events
    pub const AUI_PANE_BUTTON: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_BUTTON);
    pub const AUI_PANE_CLOSE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_CLOSE);
    pub const AUI_PANE_MAXIMIZE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_MAXIMIZE);
    pub const AUI_PANE_RESTORE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_RESTORE);
    pub const AUI_PANE_ACTIVATED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_ACTIVATED);
    pub const AUI_RENDER: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_RENDER);

    // Timer event
    pub const TIMER: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TIMER);

    /// Get the underlying stable C enum value.
    pub(crate) fn as_c_enum(&self) -> ffi::WXDEventTypeCEnum {
        self.0
    }
}

// --- Simple Event Struct ---

/// Represents a wxWidgets event.
/// This struct is a lightweight wrapper around the raw `wxd_Event_t*` pointer.
/// It provides safe methods to access event details.
#[derive(Debug, Clone, Copy)] // Raw pointers are Copy
pub struct Event(pub(crate) *mut ffi::wxd_Event_t);

impl Event {
    /// Creates a new Event wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_Event_t` pointer obtained from wxWidgets.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Event_t) -> Self {
        Event(ptr)
    }

    /// Gets the raw pointer to the underlying wxWidgets event object.
    pub(crate) fn _as_ptr(&self) -> *mut ffi::wxd_Event_t {
        self.0
    }

    /// Checks if the underlying pointer is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Gets the ID of the event.
    pub fn get_id(&self) -> i32 {
        if self.0.is_null() {
            return ffi::WXD_ID_ANY as i32;
        }
        unsafe { ffi::wxd_Event_GetId(self.0) }
    }

    /// Gets the object (usually a window) that generated the event.
    pub fn get_event_object(&self) -> Option<Window> {
        if self.0.is_null() {
            return None;
        }
        let ptr = unsafe { ffi::wxd_Event_GetEventObject(self.0) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Window::from_ptr(ptr) })
        }
    }

    /// Gets the event type.
    pub fn get_event_type(&self) -> Option<EventType> {
        if self.0.is_null() {
            return None;
        }
        let event_type_c = unsafe { ffi::wxd_Event_GetEventType(self.0) };
        // If event_type_c is WXD_EVENT_TYPE_NULL or an invalid value, return None
        if event_type_c
            == ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NULL
                .try_into()
                .unwrap()
        {
            None
        } else {
            // Convert i32 to the C enum type
            let c_enum_val =
                unsafe { std::mem::transmute::<i32, ffi::WXDEventTypeCEnum>(event_type_c) };
            Some(EventType(c_enum_val))
        }
    }

    /// Controls whether the event is processed further.
    pub fn skip(&self, skip: bool) {
        if self.0.is_null() {
            return;
        }
        unsafe { ffi::wxd_Event_Skip(self.0, skip) };
    }

    // --- Common Event Data Accessors ---

    /// Gets the string associated with a command event.
    pub fn get_string(&self) -> Option<String> {
        if self.0.is_null() {
            return None;
        }
        unsafe {
            let mut buffer: [c_char; 1024] = [0; 1024];
            let len_needed =
                ffi::wxd_CommandEvent_GetString(self.0, buffer.as_mut_ptr(), buffer.len() as i32);
            if len_needed < 0 {
                return None;
            }
            let len_needed_usize = len_needed as usize;
            if len_needed_usize < buffer.len() {
                Some(
                    CStr::from_ptr(buffer.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                )
            } else {
                let mut vec_buffer: Vec<u8> = vec![0; len_needed_usize + 1];
                let len_copied = ffi::wxd_CommandEvent_GetString(
                    self.0,
                    vec_buffer.as_mut_ptr() as *mut c_char,
                    vec_buffer.len() as i32,
                );
                if len_copied == len_needed {
                    vec_buffer.pop();
                    String::from_utf8(vec_buffer).ok()
                } else {
                    None
                }
            }
        }
    }

    /// Checks if a command event represents a "checked" state.
    pub fn is_checked(&self) -> Option<bool> {
        if self.0.is_null() {
            return None;
        }
        Some(unsafe { ffi::wxd_CommandEvent_IsChecked(self.0) })
    }

    /// Gets the mouse position associated with a mouse event.
    pub fn get_position(&self) -> Option<Point> {
        if self.0.is_null() {
            return None;
        }
        let c_point = unsafe { ffi::wxd_MouseEvent_GetPosition(self.0) };
        if c_point.x == -1 && c_point.y == -1 {
            None
        } else {
            Some(Point {
                x: c_point.x,
                y: c_point.y,
            })
        }
    }

    /// Gets the key code associated with a key event.
    pub fn get_key_code(&self) -> Option<i32> {
        if self.0.is_null() {
            return None;
        }
        let key_code = unsafe { ffi::wxd_KeyEvent_GetKeyCode(self.0) };
        if key_code == 0 {
            None
        } else {
            Some(key_code)
        }
    }

    /// Gets the integer value associated with a command event.
    pub fn get_int(&self) -> Option<i32> {
        if self.0.is_null() {
            return None;
        }
        let int_val = unsafe { ffi::wxd_CommandEvent_GetInt(self.0) };
        if int_val == -1 {
            None
        } else {
            Some(int_val)
        }
    }
}

// --- WxEvtHandler Trait (Updated for Simple Event Handling) ---

pub trait WxEvtHandler {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t;

    // Internal implementation with crate visibility
    #[doc(hidden)]
    fn bind_internal<F>(&self, event_type: EventType, callback: F)
    where
        F: FnMut(Event) + 'static,
    {
        let handler_ptr = unsafe { self.get_event_handler_ptr() };
        if handler_ptr.is_null() {
            /* ... error handling ... */
            return;
        }

        // Double-box the callback to match trampoline expectations
        let boxed_callback: Box<dyn FnMut(Event) + 'static> = Box::new(callback);
        let double_boxed = Box::new(boxed_callback);
        let user_data = Box::into_raw(double_boxed) as *mut c_void;

        type TrampolineFn = unsafe extern "C" fn(*mut c_void, *mut c_void);
        let trampoline_ptr: TrampolineFn = rust_event_handler_trampoline;
        let trampoline_c_void = trampoline_ptr as *mut c_void;

        unsafe {
            ffi::wxd_EvtHandler_Bind(
                handler_ptr,
                event_type.as_c_enum(),
                trampoline_c_void,
                user_data,
            );
        }
    }

    // Internal implementation with ID support for tools and menu items
    #[doc(hidden)]
    fn bind_with_id_internal<F>(&self, event_type: EventType, id: i32, callback: F)
    where
        F: FnMut(Event) + 'static,
    {
        let handler_ptr = unsafe { self.get_event_handler_ptr() };
        if handler_ptr.is_null() {
            /* ... error handling ... */
            return;
        }

        // Double-box the callback to match trampoline expectations
        let boxed_callback: Box<dyn FnMut(Event) + 'static> = Box::new(callback);
        let double_boxed = Box::new(boxed_callback);
        let user_data = Box::into_raw(double_boxed) as *mut c_void;

        type TrampolineFn = unsafe extern "C" fn(*mut c_void, *mut c_void);
        let trampoline_ptr: TrampolineFn = rust_event_handler_trampoline;
        let trampoline_c_void = trampoline_ptr as *mut c_void;

        unsafe {
            ffi::wxd_EvtHandler_BindWithId(
                handler_ptr,
                event_type.as_c_enum(),
                id,
                trampoline_c_void,
                user_data,
            );
        }
    }
}

// --- FFI Trampoline & Drop Functions (Updated for Simple Event) ---

/// Trampoline function: Called by C++.
/// `user_data` is a raw pointer to `Box<dyn FnMut(Event) + 'static>`.
#[no_mangle]
pub unsafe extern "C" fn rust_event_handler_trampoline(
    user_data: *mut c_void,
    event_ptr_cvoid: *mut c_void,
) {
    if user_data.is_null() {
        /* ... error handling ... */
        return;
    }

    // Cast to Box<dyn FnMut(Event)> directly
    let closure_box = &mut *(user_data as *mut Box<dyn FnMut(Event) + 'static>);
    let event_ptr = event_ptr_cvoid as *mut ffi::wxd_Event_t;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // UPDATED: Create simple Event
        let safe_event = Event::from_ptr(event_ptr);
        (*closure_box)(safe_event);
    }));

    if result.is_err() { /* ... error handling ... */ }
}

/// Function called by C++ to drop the Rust closure Box.
/// `ptr` is a raw pointer to `Box<dyn FnMut(Event) + 'static>`.
#[no_mangle]
pub unsafe extern "C" fn drop_rust_closure_box(ptr: *mut c_void) {
    if !ptr.is_null() {
        // Drop the Box<dyn FnMut(Event)>
        let _ = Box::from_raw(ptr as *mut Box<dyn FnMut(Event) + 'static>);
    } else { /* ... */
    }
}
