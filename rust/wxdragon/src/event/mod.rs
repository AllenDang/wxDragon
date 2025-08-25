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
pub mod taskbar_events;
pub mod text_events;
pub mod tree_events;
pub mod window_events;

// Re-export window events for easier access
pub use window_events::{
    IdleEventData, KeyboardEvent, MouseButtonEvent, MouseEnterEvent, MouseLeaveEvent,
    MouseMotionEvent, WindowEvent, WindowEventData, WindowEvents, WindowSizeEvent,
};

// Re-export button events for easier access
pub use button_events::{ButtonEvent, ButtonEventData, ButtonEvents};

// Re-export text events for easier access
pub use text_events::{TextEvent, TextEventData, TextEvents};

// Re-export tree events for easier access
pub use tree_events::{TreeEvent, TreeEventData, TreeEvents};

// Re-export scroll events for easier access
pub use scroll_events::{ScrollEvent, ScrollEventData, ScrollEvents};

// Re-export taskbar events for easier access
#[cfg(any(target_os = "windows", target_os = "linux"))]
pub use taskbar_events::{TaskBarIconEvent, TaskBarIconEventData};

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
    pub const ENTER_WINDOW: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_ENTER_WINDOW);
    pub const LEAVE_WINDOW: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LEAVE_WINDOW);
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

    // Media events - only available when media-ctrl feature is enabled
    #[cfg(feature = "media-ctrl")]
    pub const MEDIA_LOADED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_LOADED);
    #[cfg(feature = "media-ctrl")]
    pub const MEDIA_STOP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_STOP);
    #[cfg(feature = "media-ctrl")]
    pub const MEDIA_FINISHED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_FINISHED);
    #[cfg(feature = "media-ctrl")]
    pub const MEDIA_STATECHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_STATECHANGED);
    #[cfg(feature = "media-ctrl")]
    pub const MEDIA_PLAY: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_PLAY);
    #[cfg(feature = "media-ctrl")]
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
    #[cfg(feature = "aui")]
    pub const AUI_PANE_BUTTON: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_BUTTON);
    #[cfg(feature = "aui")]
    pub const AUI_PANE_CLOSE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_CLOSE);
    #[cfg(feature = "aui")]
    pub const AUI_PANE_MAXIMIZE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_MAXIMIZE);
    #[cfg(feature = "aui")]
    pub const AUI_PANE_RESTORE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_RESTORE);
    #[cfg(feature = "aui")]
    pub const AUI_PANE_ACTIVATED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_ACTIVATED);
    #[cfg(feature = "aui")]
    pub const AUI_RENDER: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_RENDER);

    // Timer event
    pub const TIMER: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TIMER);

    // StyledTextCtrl events - only available when stc feature is enabled
    #[cfg(feature = "stc")]
    pub const STC_CHANGE: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_CHANGE);
    #[cfg(feature = "stc")]
    pub const STC_STYLENEEDED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_STYLENEEDED);
    #[cfg(feature = "stc")]
    pub const STC_CHARADDED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_CHARADDED);
    #[cfg(feature = "stc")]
    pub const STC_SAVEPOINTREACHED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_SAVEPOINTREACHED);
    #[cfg(feature = "stc")]
    pub const STC_SAVEPOINTLEFT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_SAVEPOINTLEFT);
    #[cfg(feature = "stc")]
    pub const STC_ROMODIFYATTEMPT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_ROMODIFYATTEMPT);
    #[cfg(feature = "stc")]
    pub const STC_DOUBLECLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_DOUBLECLICK);
    #[cfg(feature = "stc")]
    pub const STC_UPDATEUI: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_UPDATEUI);
    #[cfg(feature = "stc")]
    pub const STC_MODIFIED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_MODIFIED);
    #[cfg(feature = "stc")]
    pub const STC_MACRORECORD: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_MACRORECORD);
    #[cfg(feature = "stc")]
    pub const STC_MARGINCLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_MARGINCLICK);
    #[cfg(feature = "stc")]
    pub const STC_NEEDSHOWN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_NEEDSHOWN);
    #[cfg(feature = "stc")]
    pub const STC_PAINTED: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_PAINTED);
    #[cfg(feature = "stc")]
    pub const STC_USERLISTSELECTION: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_USERLISTSELECTION);
    #[cfg(feature = "stc")]
    pub const STC_DWELLSTART: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_DWELLSTART);
    #[cfg(feature = "stc")]
    pub const STC_DWELLEND: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_DWELLEND);
    #[cfg(feature = "stc")]
    pub const STC_START_DRAG: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_START_DRAG);
    #[cfg(feature = "stc")]
    pub const STC_DRAG_OVER: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_DRAG_OVER);
    #[cfg(feature = "stc")]
    pub const STC_DO_DROP: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_DO_DROP);
    #[cfg(feature = "stc")]
    pub const STC_ZOOM: EventType = EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_ZOOM);
    #[cfg(feature = "stc")]
    pub const STC_HOTSPOT_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_HOTSPOT_CLICK);
    #[cfg(feature = "stc")]
    pub const STC_HOTSPOT_DCLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_HOTSPOT_DCLICK);
    #[cfg(feature = "stc")]
    pub const STC_CALLTIP_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_CALLTIP_CLICK);
    #[cfg(feature = "stc")]
    pub const STC_AUTOCOMP_SELECTION: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_AUTOCOMP_SELECTION);
    #[cfg(feature = "stc")]
    pub const STC_INDICATOR_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_INDICATOR_CLICK);
    #[cfg(feature = "stc")]
    pub const STC_INDICATOR_RELEASE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_INDICATOR_RELEASE);
    #[cfg(feature = "stc")]
    pub const STC_AUTOCOMP_CANCELLED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_AUTOCOMP_CANCELLED);
    #[cfg(feature = "stc")]
    pub const STC_AUTOCOMP_CHAR_DELETED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_AUTOCOMP_CHAR_DELETED);

    // RichText events
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_LEFT_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_LEFT_CLICK);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_RIGHT_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_RIGHT_CLICK);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_MIDDLE_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_MIDDLE_CLICK);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_LEFT_DCLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_LEFT_DCLICK);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_RETURN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_RETURN);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_CHARACTER: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_CHARACTER);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_DELETE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_DELETE);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_CONTENT_INSERTED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_CONTENT_INSERTED);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_CONTENT_DELETED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_CONTENT_DELETED);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_STYLE_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_STYLE_CHANGED);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_SELECTION_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_SELECTION_CHANGED);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_STYLESHEET_CHANGING: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_CHANGING);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_STYLESHEET_CHANGED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_CHANGED);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_STYLESHEET_REPLACING: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_REPLACING);
    #[cfg(feature = "richtext")]
    pub const RICHTEXT_STYLESHEET_REPLACED: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_REPLACED);

    // TaskBarIcon Event Types - platform-specific support

    // Common events supported on Windows and Linux
    pub const TASKBAR_LEFT_DOWN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_LEFT_DOWN);
    pub const TASKBAR_LEFT_DCLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_LEFT_DCLICK);

    // Windows-only events
    #[cfg(target_os = "windows")]
    pub const TASKBAR_MOVE: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_MOVE);
    #[cfg(target_os = "windows")]
    pub const TASKBAR_LEFT_UP: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_LEFT_UP);
    #[cfg(target_os = "windows")]
    pub const TASKBAR_RIGHT_DOWN: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_RIGHT_DOWN);
    #[cfg(target_os = "windows")]
    pub const TASKBAR_RIGHT_UP: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_RIGHT_UP);
    #[cfg(target_os = "windows")]
    pub const TASKBAR_RIGHT_DCLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_RIGHT_DCLICK);
    #[cfg(target_os = "windows")]
    pub const TASKBAR_BALLOON_TIMEOUT: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_BALLOON_TIMEOUT);
    #[cfg(target_os = "windows")]
    pub const TASKBAR_BALLOON_CLICK: EventType =
        EventType(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_BALLOON_CLICK);

    /// Get the underlying stable C enum value.
    pub(crate) fn as_c_enum(&self) -> ffi::WXDEventTypeCEnum {
        self.0
    }
}

/// Idle event processing modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdleMode {
    /// Send idle events to all windows
    ProcessAll = 0,
    /// Send idle events only to windows that explicitly request them
    ProcessSpecified = 1,
}

/// Static methods for controlling idle event behavior
pub struct IdleEvent;

impl IdleEvent {
    /// Sets how wxWidgets will send idle events.
    ///
    /// # Arguments
    /// * `mode` - The idle processing mode
    ///
    /// # Example
    /// ```ignore
    /// use wxdragon::event::{IdleEvent, IdleMode};
    ///
    /// // Only send idle events to windows that request them
    /// IdleEvent::set_mode(IdleMode::ProcessSpecified);
    /// ```
    pub fn set_mode(mode: IdleMode) {
        unsafe {
            ffi::wxd_IdleEvent_SetMode(mode as i32);
        }
    }

    /// Gets the current idle event processing mode.
    pub fn get_mode() -> IdleMode {
        let mode = unsafe { ffi::wxd_IdleEvent_GetMode() };
        match mode {
            1 => IdleMode::ProcessSpecified,
            _ => IdleMode::ProcessAll,
        }
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
            let c_enum_val = event_type_c
                .try_into()
                .unwrap_or(ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NULL);
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

    /// Gets the wheel rotation value associated with a mouse wheel event.
    /// Returns the wheel rotation amount in multiples of wheel delta.
    /// Positive values indicate forward/up scrolling, negative values indicate backward/down scrolling.
    pub fn get_wheel_rotation(&self) -> i32 {
        if self.0.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_MouseEvent_GetWheelRotation(self.0) }
    }

    /// Gets the wheel delta value associated with a mouse wheel event.
    /// This is the basic unit of wheel rotation, typically 120 on most systems.
    /// The actual rotation can be calculated as get_wheel_rotation() / get_wheel_delta().
    pub fn get_wheel_delta(&self) -> i32 {
        if self.0.is_null() {
            return 120; // Default wheel delta
        }
        unsafe { ffi::wxd_MouseEvent_GetWheelDelta(self.0) }
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

    /// Gets the key unicode value associated with a key event.
    pub fn get_unicode_key(&self) -> Option<i32> {
        if self.0.is_null() {
            return None;
        }
        let key_code = unsafe { ffi::wxd_KeyEvent_GetUnicodeKey(self.0) };
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

    /// Requests more idle events to be sent.
    /// This should only be called from an idle event handler.
    /// When `need_more` is true, the system will continue sending idle events.
    /// When false, idle events will stop until triggered by other activity.
    pub fn request_more(&self, need_more: bool) {
        if self.0.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_IdleEvent_RequestMore(self.0, need_more);
        }
    }

    /// Returns true if more idle events have been requested.
    /// This can be used to check if the idle event handler requested more processing.
    pub fn more_requested(&self) -> bool {
        if self.0.is_null() {
            return false;
        }
        unsafe { ffi::wxd_IdleEvent_MoreRequested(self.0) }
    }

    /// Checks if an event can be vetoed.
    /// This works with all vetable events (close events, tree events, list events, etc.)
    /// to determine if the application can prevent the event's default action.
    /// Note: This method now uses the general veto system and works with all vetable events.
    pub fn can_veto(&self) -> bool {
        if self.0.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Event_CanVeto(self.0) }
    }

    /// Vetos an event, preventing its default action.
    /// This should only be called if `can_veto()` returns true.
    /// Works with all vetable events (close events, tree events, list events, etc.).
    /// When called on a close event, it prevents the window from being closed.
    /// When called on other events, it prevents their respective default actions.
    /// The event handler should provide feedback to the user about why the action was cancelled.
    /// Note: This method now uses the general veto system and works with all vetable events.
    pub fn veto(&self) {
        if self.0.is_null() {
            return;
        }
        unsafe { ffi::wxd_Event_Veto(self.0) }
    }

    /// General method to check if any event was vetoed.
    /// Works with all vetable events (wxCloseEvent, wxNotifyEvent derivatives, etc.)
    pub fn is_vetoed(&self) -> bool {
        if self.0.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Event_IsVetoed(self.0) }
    }

    /// Sets whether an event can be vetoed.
    /// This method only applies to events that support veto functionality.
    /// For wxCloseEvent: controls whether the close event can be cancelled
    /// For other vetable events: this method exists for API completeness but may not have effect
    /// as most other vetable events (derived from wxNotifyEvent) are always vetable
    pub fn set_can_veto(&self, can_veto: bool) {
        if self.0.is_null() {
            return;
        }
        unsafe { ffi::wxd_Event_SetCanVeto(self.0, can_veto) }
    }
}

// --- WxEvtHandler Trait (Updated for Simple Event Handling) ---

pub trait WxEvtHandler {
    /// Returns the raw event handler pointer for this widget.
    ///
    /// # Safety
    /// The caller must ensure the returned pointer is valid and not null.
    /// The pointer must point to a valid wxEvtHandler object that remains valid
    /// for the lifetime of this widget.
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
///
/// # Safety
/// This function is called from C++ code and must maintain the following invariants:
/// - `user_data` must be a valid pointer to a `Box<Box<dyn FnMut(Event) + 'static>>`
/// - `event_ptr_cvoid` must be a valid pointer to a `wxd_Event_t` object
/// - The pointers must remain valid for the duration of this function call
/// - This function must not be called from multiple threads simultaneously
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
///
/// # Safety
/// This function is called from C++ code to clean up Rust callbacks.
/// - `ptr` must be a valid pointer to a `Box<Box<dyn FnMut(Event) + 'static>>`
///   that was previously allocated by Rust
/// - The pointer must not be used after this function returns
/// - This function must only be called once per pointer
#[no_mangle]
pub unsafe extern "C" fn drop_rust_closure_box(ptr: *mut c_void) {
    if !ptr.is_null() {
        // Drop the Box<dyn FnMut(Event)>
        let _ = Box::from_raw(ptr as *mut Box<dyn FnMut(Event) + 'static>);
    }
}
