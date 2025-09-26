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

bitflags::bitflags! {
/// Represents a wxDragon event type using stable C enum values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // Ensures memory layout matches the underlying C enum integer type
pub struct EventType: ffi::WXDEventTypeCEnum { // Use the generated C enum type
    // Constants map directly to the stable C enum values
    const COMMAND_BUTTON_CLICKED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_BUTTON_CLICKED;
    const CLOSE_WINDOW = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CLOSE_WINDOW;
    const CHECKBOX = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CHECKBOX;
    const TEXT = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TEXT;
    const TEXT_ENTER = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TEXT_ENTER;
    const SIZE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SIZE;
    const MENU = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MENU;
    const LEFT_DOWN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LEFT_DOWN;
    const LEFT_UP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LEFT_UP;
    const RIGHT_DOWN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RIGHT_DOWN;
    const RIGHT_UP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RIGHT_UP;
    const MIDDLE_DOWN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MIDDLE_DOWN;
    const MIDDLE_UP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MIDDLE_UP;
    const MOTION = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MOTION;
    const MOUSEWHEEL = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MOUSEWHEEL;
    const ENTER_WINDOW = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_ENTER_WINDOW;
    const LEAVE_WINDOW = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LEAVE_WINDOW;
    const KEY_DOWN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_KEY_DOWN;
    const KEY_UP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_KEY_UP;
    const CHAR = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CHAR;
    const COMMAND_RADIOBUTTON_SELECTED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_RADIOBUTTON_SELECTED;
    const COMMAND_RADIOBOX_SELECTED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_RADIOBOX_SELECTED;
    const COMMAND_LISTBOX_SELECTED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_LISTBOX_SELECTED;
    const COMMAND_CHOICE_SELECTED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_CHOICE_SELECTED;
    const COMMAND_COMBOBOX_SELECTED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_COMBOBOX_SELECTED;
    const COMMAND_CHECKLISTBOX_SELECTED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_CHECKLISTBOX_SELECTED;
    const COMMAND_LISTBOX_DOUBLECLICKED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_LISTBOX_DOUBLECLICKED;
    const COMMAND_TOGGLEBUTTON_CLICKED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_TOGGLEBUTTON_CLICKED;
    // ADDED: RearrangeList event type
    const COMMAND_REARRANGE_LIST = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_REARRANGE_LIST;
    // ADDED: CollapsiblePane event type
    const COLLAPSIBLEPANE_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COLLAPSIBLEPANE_CHANGED;
    // ADDED: TreeCtrl event types
    const TREE_BEGIN_LABEL_EDIT = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_BEGIN_LABEL_EDIT;
    const TREE_END_LABEL_EDIT = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_END_LABEL_EDIT;
    const TREE_SEL_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_SEL_CHANGED;
    const TREE_ITEM_ACTIVATED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_ACTIVATED;
    // ADDED: Slider event type
    const SLIDER = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SLIDER;
    // ADDED: SpinCtrl event type
    const SPINCTRL = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPINCTRL;
    // ADDED: SpinButton event types
    const SPIN_UP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPIN_UP;
    const SPIN_DOWN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPIN_DOWN;
    const SPIN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPIN;
    // ADDED: Notebook event type
    const NOTEBOOK_PAGE_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NOTEBOOK_PAGE_CHANGED;
    // ADDED: Splitter event types
    const SPLITTER_SASH_POS_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGED;
    const SPLITTER_SASH_POS_CHANGING = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGING;
    const SPLITTER_DOUBLECLICKED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPLITTER_DOUBLECLICKED;
    const SPLITTER_UNSPLIT = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPLITTER_UNSPLIT;
    // ADDED: ListCtrl event types
    const LIST_ITEM_SELECTED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_SELECTED;
    const LIST_ITEM_ACTIVATED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_ACTIVATED;
    const LIST_COL_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_COL_CLICK;
    const LIST_BEGIN_LABEL_EDIT = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_BEGIN_LABEL_EDIT;
    const LIST_END_LABEL_EDIT = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_END_LABEL_EDIT;
    // ADDED: Additional ListCtrl event types
    const LIST_BEGIN_DRAG = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_BEGIN_DRAG;
    const LIST_BEGIN_RDRAG = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_BEGIN_RDRAG;
    const LIST_DELETE_ITEM = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_DELETE_ITEM;
    const LIST_DELETE_ALL_ITEMS = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_DELETE_ALL_ITEMS;
    const LIST_ITEM_DESELECTED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_DESELECTED;
    const LIST_ITEM_FOCUSED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_FOCUSED;
    const LIST_ITEM_MIDDLE_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_MIDDLE_CLICK;
    const LIST_ITEM_RIGHT_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_ITEM_RIGHT_CLICK;
    const LIST_KEY_DOWN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_KEY_DOWN;
    const LIST_INSERT_ITEM = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_INSERT_ITEM;
    const LIST_COL_RIGHT_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_COL_RIGHT_CLICK;
    const LIST_COL_BEGIN_DRAG = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_LIST_COL_BEGIN_DRAG;
    // ADDED: ColourPickerCtrl event type
    const COLOURPICKER_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COLOURPICKER_CHANGED;
    // DatePicker Event
    const DATE_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATE_CHANGED;
    // TimePicker Event
    const TIME_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TIME_CHANGED;
    // Treebook Events (match WXDEventTypeCEnum values)
    const TREEBOOK_PAGE_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGED;
    const TREEBOOK_PAGE_CHANGING = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGING;
    const TREEBOOK_NODE_EXPANDED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREEBOOK_NODE_EXPANDED;
    const TREEBOOK_NODE_COLLAPSED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREEBOOK_NODE_COLLAPSED;
    // ADDED: SearchCtrl Event Types
    const COMMAND_SEARCHCTRL_SEARCH_BTN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_SEARCH_BTN;
    const COMMAND_SEARCHCTRL_CANCEL_BTN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_CANCEL_BTN;
    const COMMAND_HYPERLINK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_COMMAND_HYPERLINK;
    const SPINCTRLDOUBLE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SPINCTRLDOUBLE;
    // ADDED: Calendar Control Event Type
    const CALENDAR_SEL_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CALENDAR_SEL_CHANGED;
    // ADDED: Missing Calendar Control Event Types
    const CALENDAR_DOUBLECLICKED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CALENDAR_DOUBLECLICKED;
    const CALENDAR_MONTH_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CALENDAR_MONTH_CHANGED;
    const CALENDAR_YEAR_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CALENDAR_YEAR_CHANGED;
    const CALENDAR_WEEKDAY_CLICKED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_CALENDAR_WEEKDAY_CLICKED;
    // ADDED: ScrollBar Events
    const SCROLL_TOP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_TOP;
    const SCROLL_BOTTOM = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_BOTTOM;
    const SCROLL_LINEUP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_LINEUP;
    const SCROLL_LINEDOWN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_LINEDOWN;
    const SCROLL_PAGEUP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_PAGEUP;
    const SCROLL_PAGEDOWN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_PAGEDOWN;
    const SCROLL_THUMBTRACK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_THUMBTRACK;
    const SCROLL_THUMBRELEASE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_THUMBRELEASE;
    const SCROLL_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SCROLL_CHANGED;
    const FILE_PICKER_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_FILEPICKER_CHANGED;
    const DIR_PICKER_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DIRPICKER_CHANGED;
    const FONT_PICKER_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_FONTPICKER_CHANGED;

    const NOTIFICATION_MESSAGE_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_CLICK;
    const NOTIFICATION_MESSAGE_DISMISSED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_DISMISSED;
    const NOTIFICATION_MESSAGE_ACTION = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_ACTION;

    // Media events - only available when media-ctrl feature is enabled
    #[cfg(feature = "media-ctrl")]
    const MEDIA_LOADED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_LOADED;
    #[cfg(feature = "media-ctrl")]
    const MEDIA_STOP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_STOP;
    #[cfg(feature = "media-ctrl")]
    const MEDIA_FINISHED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_FINISHED;
    #[cfg(feature = "media-ctrl")]
    const MEDIA_STATECHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_STATECHANGED;
    #[cfg(feature = "media-ctrl")]
    const MEDIA_PLAY = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_PLAY;
    #[cfg(feature = "media-ctrl")]
    const MEDIA_PAUSE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MEDIA_PAUSE;

    const EVT_DATE_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATE_CHANGED;

    const IDLE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_IDLE;

    // Drag and drop events
    const DROP_FILES = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DROP_FILES;

    const PAINT = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_PAINT;

    const DESTROY = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DESTROY;

    // Additional window events
    const MOVE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_MOVE;
    const ERASE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_ERASE;
    const SET_FOCUS = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_SET_FOCUS;
    const KILL_FOCUS = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_KILL_FOCUS;

    // DataView events
    const DATAVIEW_SELECTION_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_SELECTION_CHANGED;
    const DATAVIEW_ITEM_ACTIVATED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_ACTIVATED;
    const DATAVIEW_ITEM_EDITING_STARTED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_EDITING_STARTED;
    const DATAVIEW_ITEM_EDITING_DONE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_EDITING_DONE;
    const DATAVIEW_ITEM_COLLAPSING = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_COLLAPSING;
    const DATAVIEW_ITEM_COLLAPSED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_COLLAPSED;
    const DATAVIEW_ITEM_EXPANDING = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_EXPANDING;
    const DATAVIEW_ITEM_EXPANDED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_ITEM_EXPANDED;
    const DATAVIEW_COLUMN_HEADER_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_COLUMN_HEADER_CLICK;
    const DATAVIEW_COLUMN_HEADER_RIGHT_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_COLUMN_HEADER_RIGHT_CLICK;
    const DATAVIEW_COLUMN_SORTED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_COLUMN_SORTED;
    const DATAVIEW_COLUMN_REORDERED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATAVIEW_COLUMN_REORDERED;

    // ADDED: New TreeCtrl Event Types (complementing 22-25)
    const TREE_SEL_CHANGING = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_SEL_CHANGING;
    const TREE_ITEM_COLLAPSING = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_COLLAPSING;
    const TREE_ITEM_COLLAPSED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_COLLAPSED;
    const TREE_ITEM_EXPANDING = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_EXPANDING;
    const TREE_ITEM_EXPANDED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_EXPANDED;
    const TREE_ITEM_RIGHT_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_RIGHT_CLICK;
    const TREE_ITEM_MIDDLE_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_MIDDLE_CLICK;
    const TREE_KEY_DOWN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_KEY_DOWN;
    const TREE_DELETE_ITEM = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_DELETE_ITEM;
    const TREE_ITEM_MENU = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_MENU;
    const TREE_BEGIN_DRAG = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_BEGIN_DRAG;
    const TREE_BEGIN_RDRAG = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_BEGIN_RDRAG;
    const TREE_END_DRAG = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_END_DRAG;
    const TREE_STATE_IMAGE_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_STATE_IMAGE_CLICK;

    // ToolBar Events
    const TOOL = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TOOL;
    const TOOL_ENTER = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TOOL_ENTER;

    // TreeCtrl Events
    const TREE_ITEM_GETTOOLTIP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TREE_ITEM_GETTOOLTIP;

    // Generic events that might not fit a specific category or are widely used
    const ANY = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_ANY;

    // Special event type for null/None, not a real wxWidgets event type
    const NONE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NULL; // Assuming NULL is 0

    // AuiManager events
    #[cfg(feature = "aui")]
    const AUI_PANE_BUTTON = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_BUTTON;
    #[cfg(feature = "aui")]
    const AUI_PANE_CLOSE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_CLOSE;
    #[cfg(feature = "aui")]
    const AUI_PANE_MAXIMIZE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_MAXIMIZE;
    #[cfg(feature = "aui")]
    const AUI_PANE_RESTORE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_RESTORE;
    #[cfg(feature = "aui")]
    const AUI_PANE_ACTIVATED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_PANE_ACTIVATED;
    #[cfg(feature = "aui")]
    const AUI_RENDER = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_AUI_RENDER;

    // Timer event
    const TIMER = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TIMER;

    // StyledTextCtrl events - only available when stc feature is enabled
    #[cfg(feature = "stc")]
    const STC_CHANGE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_CHANGE;
    #[cfg(feature = "stc")]
    const STC_STYLENEEDED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_STYLENEEDED;
    #[cfg(feature = "stc")]
    const STC_CHARADDED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_CHARADDED;
    #[cfg(feature = "stc")]
    const STC_SAVEPOINTREACHED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_SAVEPOINTREACHED;
    #[cfg(feature = "stc")]
    const STC_SAVEPOINTLEFT = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_SAVEPOINTLEFT;
    #[cfg(feature = "stc")]
    const STC_ROMODIFYATTEMPT = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_ROMODIFYATTEMPT;
    #[cfg(feature = "stc")]
    const STC_DOUBLECLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_DOUBLECLICK;
    #[cfg(feature = "stc")]
    const STC_UPDATEUI = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_UPDATEUI;
    #[cfg(feature = "stc")]
    const STC_MODIFIED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_MODIFIED;
    #[cfg(feature = "stc")]
    const STC_MACRORECORD = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_MACRORECORD;
    #[cfg(feature = "stc")]
    const STC_MARGINCLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_MARGINCLICK;
    #[cfg(feature = "stc")]
    const STC_NEEDSHOWN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_NEEDSHOWN;
    #[cfg(feature = "stc")]
    const STC_PAINTED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_PAINTED;
    #[cfg(feature = "stc")]
    const STC_USERLISTSELECTION = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_USERLISTSELECTION;
    #[cfg(feature = "stc")]
    const STC_DWELLSTART = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_DWELLSTART;
    #[cfg(feature = "stc")]
    const STC_DWELLEND = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_DWELLEND;
    #[cfg(feature = "stc")]
    const STC_START_DRAG = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_START_DRAG;
    #[cfg(feature = "stc")]
    const STC_DRAG_OVER = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_DRAG_OVER;
    #[cfg(feature = "stc")]
    const STC_DO_DROP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_DO_DROP;
    #[cfg(feature = "stc")]
    const STC_ZOOM = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_ZOOM;
    #[cfg(feature = "stc")]
    const STC_HOTSPOT_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_HOTSPOT_CLICK;
    #[cfg(feature = "stc")]
    const STC_HOTSPOT_DCLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_HOTSPOT_DCLICK;
    #[cfg(feature = "stc")]
    const STC_CALLTIP_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_CALLTIP_CLICK;
    #[cfg(feature = "stc")]
    const STC_AUTOCOMP_SELECTION = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_AUTOCOMP_SELECTION;
    #[cfg(feature = "stc")]
    const STC_INDICATOR_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_INDICATOR_CLICK;
    #[cfg(feature = "stc")]
    const STC_INDICATOR_RELEASE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_INDICATOR_RELEASE;
    #[cfg(feature = "stc")]
    const STC_AUTOCOMP_CANCELLED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_AUTOCOMP_CANCELLED;
    #[cfg(feature = "stc")]
    const STC_AUTOCOMP_CHAR_DELETED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_STC_AUTOCOMP_CHAR_DELETED;

    // RichText events
    #[cfg(feature = "richtext")]
    const RICHTEXT_LEFT_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_LEFT_CLICK;
    #[cfg(feature = "richtext")]
    const RICHTEXT_RIGHT_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_RIGHT_CLICK;
    #[cfg(feature = "richtext")]
    const RICHTEXT_MIDDLE_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_MIDDLE_CLICK;
    #[cfg(feature = "richtext")]
    const RICHTEXT_LEFT_DCLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_LEFT_DCLICK;
    #[cfg(feature = "richtext")]
    const RICHTEXT_RETURN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_RETURN;
    #[cfg(feature = "richtext")]
    const RICHTEXT_CHARACTER = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_CHARACTER;
    #[cfg(feature = "richtext")]
    const RICHTEXT_DELETE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_DELETE;
    #[cfg(feature = "richtext")]
    const RICHTEXT_CONTENT_INSERTED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_CONTENT_INSERTED;
    #[cfg(feature = "richtext")]
    const RICHTEXT_CONTENT_DELETED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_CONTENT_DELETED;
    #[cfg(feature = "richtext")]
    const RICHTEXT_STYLE_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_STYLE_CHANGED;
    #[cfg(feature = "richtext")]
    const RICHTEXT_SELECTION_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_SELECTION_CHANGED;
    #[cfg(feature = "richtext")]
    const RICHTEXT_STYLESHEET_CHANGING = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_CHANGING;
    #[cfg(feature = "richtext")]
    const RICHTEXT_STYLESHEET_CHANGED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_CHANGED;
    #[cfg(feature = "richtext")]
    const RICHTEXT_STYLESHEET_REPLACING = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_REPLACING;
    #[cfg(feature = "richtext")]
    const RICHTEXT_STYLESHEET_REPLACED = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_REPLACED;

    // TaskBarIcon Event Types - platform-specific support

    // Common events supported on Windows and Linux
    const TASKBAR_LEFT_DOWN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_LEFT_DOWN;
    const TASKBAR_LEFT_DCLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_LEFT_DCLICK;

    // Windows-only events
    #[cfg(target_os = "windows")]
    const TASKBAR_MOVE = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_MOVE;
    #[cfg(target_os = "windows")]
    const TASKBAR_LEFT_UP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_LEFT_UP;
    #[cfg(target_os = "windows")]
    const TASKBAR_RIGHT_DOWN = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_RIGHT_DOWN;
    #[cfg(target_os = "windows")]
    const TASKBAR_RIGHT_UP = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_RIGHT_UP;
    #[cfg(target_os = "windows")]
    const TASKBAR_RIGHT_DCLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_RIGHT_DCLICK;
    #[cfg(target_os = "windows")]
    const TASKBAR_BALLOON_TIMEOUT = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_BALLOON_TIMEOUT;
    #[cfg(target_os = "windows")]
    const TASKBAR_BALLOON_CLICK = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_TASKBAR_BALLOON_CLICK;
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
        #[allow(clippy::useless_conversion)]
        let event_type_c = unsafe { ffi::wxd_Event_GetEventType(self.0) } as WXDEventTypeCEnum;
        // If event_type_c is WXD_EVENT_TYPE_NULL or an invalid value, return None
        if event_type_c == ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_NULL {
            None
        } else {
            EventType::from_bits(event_type_c)
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

    /// Check if the Control key is pressed during this key event
    pub fn control_down(&self) -> bool {
        if self.0.is_null() {
            return false;
        }
        unsafe { ffi::wxd_KeyEvent_ControlDown(self.0) }
    }

    /// Check if the Shift key is pressed during this key event
    pub fn shift_down(&self) -> bool {
        if self.0.is_null() {
            return false;
        }
        unsafe { ffi::wxd_KeyEvent_ShiftDown(self.0) }
    }

    /// Check if the Alt key is pressed during this key event
    pub fn alt_down(&self) -> bool {
        if self.0.is_null() {
            return false;
        }
        unsafe { ffi::wxd_KeyEvent_AltDown(self.0) }
    }

    /// Check if the Meta key is pressed during this key event (Cmd on macOS, Windows key on Windows)
    pub fn meta_down(&self) -> bool {
        if self.0.is_null() {
            return false;
        }
        unsafe { ffi::wxd_KeyEvent_MetaDown(self.0) }
    }

    /// Check if the platform-specific command key is pressed (Cmd on macOS, Ctrl on Windows/Linux)
    pub fn cmd_down(&self) -> bool {
        if self.0.is_null() {
            return false;
        }
        unsafe { ffi::wxd_KeyEvent_CmdDown(self.0) }
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

        let et = event_type.bits();
        unsafe { ffi::wxd_EvtHandler_Bind(handler_ptr, et, trampoline_c_void, user_data) };
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
        let trampo_c_void = trampoline_ptr as *mut c_void;

        let et = event_type.bits();
        unsafe { ffi::wxd_EvtHandler_BindWithId(handler_ptr, et, id, trampo_c_void, user_data) };
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
