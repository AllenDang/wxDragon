#ifndef WXD_TYPES_H
#define WXD_TYPES_H

// Use standard C types
#include <stdbool.h> 
#include <stdint.h> // For integer types if needed
#include <stddef.h> // For size_t

// --- Basic Types --- 
typedef int wxd_Id;

typedef struct {
    int x, y;
} wxd_Point;

typedef struct {
    int width, height;
} wxd_Size;

// GridBagSizer position and span types
typedef struct {
    int row, col;
} wxd_GBPosition;

typedef struct {
    int rowspan, colspan;
} wxd_GBSpan;

typedef struct wxd_Rect {
    int x;
    int y;
    int width;
    int height;
} wxd_Rect;

typedef struct {
    unsigned char r;
    unsigned char g;
    unsigned char b;
    unsigned char a;
} wxd_Colour_t;

// DateTime type for DataView
typedef struct {
    short day;
    unsigned short month;
    int year;
    short hour;
    short minute;
    short second;
} wxd_DateTime_t;

// Variant type codes for DataView
#define WXD_VARIANT_TYPE_INVALID 0
#define WXD_VARIANT_TYPE_BOOL 1
#define WXD_VARIANT_TYPE_INT32 2
#define WXD_VARIANT_TYPE_INT64 3
#define WXD_VARIANT_TYPE_DOUBLE 4
#define WXD_VARIANT_TYPE_STRING 5
#define WXD_VARIANT_TYPE_BITMAP 6
#define WXD_VARIANT_TYPE_DATETIME 7
#define WXD_VARIANT_TYPE_VOID_PTR 8
#define WXD_VARIANT_TYPE_BITMAP_RUST_BORROWED 9 // New type for bitmaps borrowed from Rust

// C-compatible variant type for DataView
typedef struct wxd_Variant_t {
    int32_t type;
    union {
        bool bool_val;
        int32_t int32_val;
        int64_t int64_val;
        double double_val;
        char* string_val;
        struct wxd_Bitmap_t* bitmap_val;
        wxd_DateTime_t datetime_val;
    } data;
} wxd_Variant_t;

/// @brief Event types represented by stable integer values in C.
typedef enum {
    WXD_EVENT_TYPE_NULL = 0,
    WXD_EVENT_TYPE_COMMAND_BUTTON_CLICKED = 1,
    WXD_EVENT_TYPE_CLOSE_WINDOW = 2,
    WXD_EVENT_TYPE_CHECKBOX = 3,
    WXD_EVENT_TYPE_TEXT = 4,
    WXD_EVENT_TYPE_TEXT_ENTER = 5,
    WXD_EVENT_TYPE_SIZE = 6,
    WXD_EVENT_TYPE_MENU = 7,
    WXD_EVENT_TYPE_LEFT_DOWN = 8,
    WXD_EVENT_TYPE_LEFT_UP = 9,
    WXD_EVENT_TYPE_RIGHT_DOWN = 131,
    WXD_EVENT_TYPE_RIGHT_UP = 132,
    WXD_EVENT_TYPE_MIDDLE_DOWN = 133,
    WXD_EVENT_TYPE_MIDDLE_UP = 134,
    WXD_EVENT_TYPE_MOTION = 10,
    WXD_EVENT_TYPE_MOUSEWHEEL = 11,
    WXD_EVENT_TYPE_KEY_DOWN = 12,
    WXD_EVENT_TYPE_KEY_UP = 13,
    WXD_EVENT_TYPE_CHAR = 14,
    WXD_EVENT_TYPE_COMMAND_RADIOBUTTON_SELECTED = 15,
    WXD_EVENT_TYPE_COMMAND_RADIOBOX_SELECTED = 16,
    WXD_EVENT_TYPE_COMMAND_LISTBOX_SELECTED = 17,
    WXD_EVENT_TYPE_COMMAND_CHOICE_SELECTED = 18,
    WXD_EVENT_TYPE_COMMAND_COMBOBOX_SELECTED = 19,
    WXD_EVENT_TYPE_COMMAND_CHECKLISTBOX_SELECTED = 20,
    WXD_EVENT_TYPE_COMMAND_TOGGLEBUTTON_CLICKED = 21,
    WXD_EVENT_TYPE_TREE_BEGIN_LABEL_EDIT = 22,
    WXD_EVENT_TYPE_TREE_END_LABEL_EDIT = 23,
    WXD_EVENT_TYPE_TREE_SEL_CHANGED = 24,
    WXD_EVENT_TYPE_TREE_ITEM_ACTIVATED = 25,
    WXD_EVENT_TYPE_SLIDER = 26,
    WXD_EVENT_TYPE_SPINCTRL = 27,
    WXD_EVENT_TYPE_SPIN_UP = 28,
    WXD_EVENT_TYPE_SPIN_DOWN = 29,
    WXD_EVENT_TYPE_SPIN = 30,
    WXD_EVENT_TYPE_NOTEBOOK_PAGE_CHANGED = 31,
    WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGED = 32,
    WXD_EVENT_TYPE_SPLITTER_SASH_POS_CHANGING = 33,
    WXD_EVENT_TYPE_SPLITTER_DOUBLECLICKED = 34,
    WXD_EVENT_TYPE_SPLITTER_UNSPLIT = 35,
    WXD_EVENT_TYPE_LIST_ITEM_SELECTED = 36,
    WXD_EVENT_TYPE_LIST_ITEM_ACTIVATED = 37,
    WXD_EVENT_TYPE_LIST_COL_CLICK = 38,
    WXD_EVENT_TYPE_LIST_BEGIN_LABEL_EDIT = 39,
    WXD_EVENT_TYPE_LIST_END_LABEL_EDIT = 40,
    WXD_EVENT_TYPE_COLOURPICKER_CHANGED = 41,
    WXD_EVENT_TYPE_DATE_CHANGED = 42,
    WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGED = 43,
    WXD_EVENT_TYPE_TREEBOOK_PAGE_CHANGING = 44,
    WXD_EVENT_TYPE_TREEBOOK_NODE_EXPANDED = 45,
    WXD_EVENT_TYPE_TREEBOOK_NODE_COLLAPSED = 46,
    WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_SEARCH_BTN = 47,
    WXD_EVENT_TYPE_COMMAND_SEARCHCTRL_CANCEL_BTN = 48,
    WXD_EVENT_TYPE_COMMAND_HYPERLINK = 49,
    WXD_EVENT_TYPE_SPINCTRLDOUBLE = 50,
    WXD_EVENT_TYPE_CALENDAR_SEL_CHANGED = 51,
    WXD_EVENT_TYPE_CALENDAR_DOUBLECLICKED = 120,
    WXD_EVENT_TYPE_CALENDAR_MONTH_CHANGED = 121,
    WXD_EVENT_TYPE_CALENDAR_YEAR_CHANGED = 122,
    WXD_EVENT_TYPE_CALENDAR_WEEKDAY_CLICKED = 123,
    WXD_EVENT_TYPE_SCROLL_TOP = 52,
    WXD_EVENT_TYPE_SCROLL_BOTTOM = 53,
    WXD_EVENT_TYPE_SCROLL_LINEUP = 54,
    WXD_EVENT_TYPE_SCROLL_LINEDOWN = 55,
    WXD_EVENT_TYPE_SCROLL_PAGEUP = 56,
    WXD_EVENT_TYPE_SCROLL_PAGEDOWN = 57,
    WXD_EVENT_TYPE_SCROLL_THUMBTRACK = 58,
    WXD_EVENT_TYPE_SCROLL_THUMBRELEASE = 59,
    WXD_EVENT_TYPE_SCROLL_CHANGED = 60,
    WXD_EVENT_TYPE_FILEPICKER_CHANGED = 61,
    WXD_EVENT_TYPE_DIRPICKER_CHANGED = 62,
    WXD_EVENT_TYPE_FONTPICKER_CHANGED = 63,
    WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_CLICK = 64,
    WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_DISMISSED = 65,
    WXD_EVENT_TYPE_NOTIFICATION_MESSAGE_ACTION = 66,
    WXD_EVENT_TYPE_IDLE = 67,
    // Drag and drop events
    WXD_EVENT_TYPE_BEGIN_DRAG = 68,
    WXD_EVENT_TYPE_DROP_FILES = 69,
    WXD_EVENT_TYPE_DROP_TEXT = 70,
    WXD_EVENT_TYPE_END_DRAG = 71,
    WXD_EVENT_TYPE_PAINT = 72,
    WXD_EVENT_TYPE_TIME_CHANGED = 73,
    WXD_EVENT_TYPE_DESTROY = 74,
    // ADDED: Additional ListCtrl event types
    WXD_EVENT_TYPE_LIST_BEGIN_DRAG = 75,
    WXD_EVENT_TYPE_LIST_BEGIN_RDRAG = 76,
    WXD_EVENT_TYPE_LIST_DELETE_ITEM = 77,
    WXD_EVENT_TYPE_LIST_DELETE_ALL_ITEMS = 78,
    WXD_EVENT_TYPE_LIST_ITEM_DESELECTED = 79,
    WXD_EVENT_TYPE_LIST_ITEM_FOCUSED = 80,
    WXD_EVENT_TYPE_LIST_ITEM_MIDDLE_CLICK = 81,
    WXD_EVENT_TYPE_LIST_ITEM_RIGHT_CLICK = 82,
    WXD_EVENT_TYPE_LIST_KEY_DOWN = 83,
    WXD_EVENT_TYPE_LIST_INSERT_ITEM = 84,
    WXD_EVENT_TYPE_LIST_COL_RIGHT_CLICK = 85,
    WXD_EVENT_TYPE_LIST_COL_BEGIN_DRAG = 86,
    
    // Media events - only available when media-ctrl feature is enabled
    #if wxdUSE_MEDIACTRL
    WXD_EVENT_TYPE_MEDIA_LOADED = 87,
    WXD_EVENT_TYPE_MEDIA_STOP = 88,
    WXD_EVENT_TYPE_MEDIA_FINISHED = 89,
    WXD_EVENT_TYPE_MEDIA_STATECHANGED = 90,
    WXD_EVENT_TYPE_MEDIA_PLAY = 91,
    WXD_EVENT_TYPE_MEDIA_PAUSE = 92,
    #endif
    
    // DataView Events
    WXD_EVENT_TYPE_DATAVIEW_SELECTION_CHANGED = 93,
    WXD_EVENT_TYPE_DATAVIEW_ITEM_ACTIVATED = 94,
    WXD_EVENT_TYPE_DATAVIEW_ITEM_EDITING_STARTED = 95,
    WXD_EVENT_TYPE_DATAVIEW_ITEM_EDITING_DONE = 96,
    WXD_EVENT_TYPE_DATAVIEW_ITEM_COLLAPSING = 97,
    WXD_EVENT_TYPE_DATAVIEW_ITEM_COLLAPSED = 98,
    WXD_EVENT_TYPE_DATAVIEW_ITEM_EXPANDING = 99,
    WXD_EVENT_TYPE_DATAVIEW_ITEM_EXPANDED = 100,
    WXD_EVENT_TYPE_DATAVIEW_COLUMN_HEADER_CLICK = 101,
    WXD_EVENT_TYPE_DATAVIEW_COLUMN_HEADER_RIGHT_CLICK = 102,
    WXD_EVENT_TYPE_DATAVIEW_COLUMN_SORTED = 103,
    WXD_EVENT_TYPE_DATAVIEW_COLUMN_REORDERED = 104,

    // New TreeCtrl Event Types
    WXD_EVENT_TYPE_TREE_SEL_CHANGING = 105,      // wxEVT_TREE_SEL_CHANGING
    WXD_EVENT_TYPE_TREE_ITEM_COLLAPSING = 106,   // wxEVT_TREE_ITEM_COLLAPSING
    WXD_EVENT_TYPE_TREE_ITEM_COLLAPSED = 107,    // wxEVT_TREE_ITEM_COLLAPSED
    WXD_EVENT_TYPE_TREE_ITEM_EXPANDING = 108,    // wxEVT_TREE_ITEM_EXPANDING
    WXD_EVENT_TYPE_TREE_ITEM_EXPANDED = 109,     // wxEVT_TREE_ITEM_EXPANDED
    WXD_EVENT_TYPE_TREE_ITEM_RIGHT_CLICK = 110,  // wxEVT_TREE_ITEM_RIGHT_CLICK
    WXD_EVENT_TYPE_TREE_ITEM_MIDDLE_CLICK = 111, // wxEVT_TREE_ITEM_MIDDLE_CLICK
    WXD_EVENT_TYPE_TREE_KEY_DOWN = 112,          // wxEVT_TREE_KEY_DOWN
    WXD_EVENT_TYPE_TREE_DELETE_ITEM = 113,       // wxEVT_TREE_DELETE_ITEM
    WXD_EVENT_TYPE_TREE_ITEM_MENU = 114,         // wxEVT_TREE_ITEM_MENU
    WXD_EVENT_TYPE_TREE_BEGIN_DRAG = 115,        // wxEVT_TREE_BEGIN_DRAG
    WXD_EVENT_TYPE_TREE_BEGIN_RDRAG = 116,       // wxEVT_TREE_BEGIN_RDRAG
    WXD_EVENT_TYPE_TREE_END_DRAG = 117,          // wxEVT_TREE_END_DRAG
    WXD_EVENT_TYPE_TREE_STATE_IMAGE_CLICK = 118, // wxEVT_TREE_STATE_IMAGE_CLICK
    WXD_EVENT_TYPE_COMMAND_LISTBOX_DOUBLECLICKED = 119, // wxEVT_LISTBOX_DCLICK

    // ADDED: Additional event types missing from enum
    WXD_EVENT_TYPE_TOOL = 123, // ToolBar Tool clicked event (wxEVT_TOOL)
    WXD_EVENT_TYPE_TOOL_ENTER = 124, // ToolBar Enter event
    WXD_EVENT_TYPE_TREE_ITEM_GETTOOLTIP = 125, // TreeCtrl tooltip event
    WXD_EVENT_TYPE_ANY = 126, // Generic event type
    
    // ADDED: Window event types
    WXD_EVENT_TYPE_MOVE = 127, // Window move event
    WXD_EVENT_TYPE_ERASE = 128, // Window erase background event
    WXD_EVENT_TYPE_SET_FOCUS = 129, // Window set focus event
    WXD_EVENT_TYPE_KILL_FOCUS = 130, // Window kill focus event

    // AUI Manager event types
    #if wxdUSE_AUI
    WXD_EVENT_TYPE_AUI_PANE_BUTTON = 135, // wxEVT_AUI_PANE_BUTTON
    WXD_EVENT_TYPE_AUI_PANE_CLOSE = 136, // wxEVT_AUI_PANE_CLOSE
    WXD_EVENT_TYPE_AUI_PANE_MAXIMIZE = 137, // wxEVT_AUI_PANE_MAXIMIZE
    WXD_EVENT_TYPE_AUI_PANE_RESTORE = 138, // wxEVT_AUI_PANE_RESTORE
    WXD_EVENT_TYPE_AUI_PANE_ACTIVATED = 139, // wxEVT_AUI_PANE_ACTIVATED
    WXD_EVENT_TYPE_AUI_RENDER = 140, // wxEVT_AUI_RENDER
    #endif

    // ADDED: RearrangeList event
    WXD_EVENT_TYPE_COMMAND_REARRANGE_LIST = 141, // Event for RearrangeList when items are rearranged

    // ADDED: CollapsiblePane event
    WXD_EVENT_TYPE_COLLAPSIBLEPANE_CHANGED = 142, // Event for CollapsiblePane when expanded/collapsed

    // ADDED: TreeListCtrl event types
    WXD_EVENT_TYPE_TREELIST_SELECTION_CHANGED = 143, // Event for TreeListCtrl selection changed
    WXD_EVENT_TYPE_TREELIST_ITEM_CHECKED = 144,      // Event for TreeListCtrl item checked/unchecked
    WXD_EVENT_TYPE_TREELIST_ITEM_ACTIVATED = 145,    // Event for TreeListCtrl item activated (double-click)
    WXD_EVENT_TYPE_TREELIST_COLUMN_SORTED = 146,     // Event for TreeListCtrl column header clicked
    WXD_EVENT_TYPE_TREELIST_ITEM_EXPANDING = 147,    // Event for TreeListCtrl item expanding
    WXD_EVENT_TYPE_TREELIST_ITEM_EXPANDED = 148,     // Event for TreeListCtrl item expanded

    WXD_EVENT_TYPE_TIMER = 200, // Added wxTimer event

    // Mouse enter/leave events
    WXD_EVENT_TYPE_ENTER_WINDOW = 201, // wxEVT_ENTER_WINDOW
    WXD_EVENT_TYPE_LEAVE_WINDOW = 202, // wxEVT_LEAVE_WINDOW

    // RichText events
    #if wxdUSE_RICHTEXT
    WXD_EVENT_TYPE_RICHTEXT_LEFT_CLICK = 250,       // wxEVT_RICHTEXT_LEFT_CLICK
    WXD_EVENT_TYPE_RICHTEXT_RIGHT_CLICK = 251,      // wxEVT_RICHTEXT_RIGHT_CLICK
    WXD_EVENT_TYPE_RICHTEXT_MIDDLE_CLICK = 252,     // wxEVT_RICHTEXT_MIDDLE_CLICK
    WXD_EVENT_TYPE_RICHTEXT_LEFT_DCLICK = 253,      // wxEVT_RICHTEXT_LEFT_DCLICK
    WXD_EVENT_TYPE_RICHTEXT_RETURN = 254,           // wxEVT_RICHTEXT_RETURN
    WXD_EVENT_TYPE_RICHTEXT_CHARACTER = 255,        // wxEVT_RICHTEXT_CHARACTER
    WXD_EVENT_TYPE_RICHTEXT_DELETE = 256,           // wxEVT_RICHTEXT_DELETE
    WXD_EVENT_TYPE_RICHTEXT_CONTENT_INSERTED = 257, // wxEVT_RICHTEXT_CONTENT_INSERTED
    WXD_EVENT_TYPE_RICHTEXT_CONTENT_DELETED = 258,  // wxEVT_RICHTEXT_CONTENT_DELETED
    WXD_EVENT_TYPE_RICHTEXT_STYLE_CHANGED = 259,    // wxEVT_RICHTEXT_STYLE_CHANGED
    WXD_EVENT_TYPE_RICHTEXT_SELECTION_CHANGED = 260, // wxEVT_RICHTEXT_SELECTION_CHANGED
    WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_CHANGING = 261, // wxEVT_RICHTEXT_STYLESHEET_CHANGING
    WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_CHANGED = 262, // wxEVT_RICHTEXT_STYLESHEET_CHANGED
    WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_REPLACING = 263, // wxEVT_RICHTEXT_STYLESHEET_REPLACING
    WXD_EVENT_TYPE_RICHTEXT_STYLESHEET_REPLACED = 264, // wxEVT_RICHTEXT_STYLESHEET_REPLACED
    #endif

    // StyledTextCtrl events - only available when stc feature is enabled
    #if wxdUSE_STC
    WXD_EVENT_TYPE_STC_CHANGE = 300,                    // wxEVT_STC_CHANGE
    WXD_EVENT_TYPE_STC_STYLENEEDED = 301,               // wxEVT_STC_STYLENEEDED
    WXD_EVENT_TYPE_STC_CHARADDED = 302,                 // wxEVT_STC_CHARADDED
    WXD_EVENT_TYPE_STC_SAVEPOINTREACHED = 303,          // wxEVT_STC_SAVEPOINTREACHED
    WXD_EVENT_TYPE_STC_SAVEPOINTLEFT = 304,             // wxEVT_STC_SAVEPOINTLEFT
    WXD_EVENT_TYPE_STC_ROMODIFYATTEMPT = 305,           // wxEVT_STC_ROMODIFYATTEMPT
    WXD_EVENT_TYPE_STC_DOUBLECLICK = 306,               // wxEVT_STC_DOUBLECLICK
    WXD_EVENT_TYPE_STC_UPDATEUI = 307,                  // wxEVT_STC_UPDATEUI
    WXD_EVENT_TYPE_STC_MODIFIED = 308,                  // wxEVT_STC_MODIFIED
    WXD_EVENT_TYPE_STC_MACRORECORD = 309,               // wxEVT_STC_MACRORECORD
    WXD_EVENT_TYPE_STC_MARGINCLICK = 310,               // wxEVT_STC_MARGINCLICK
    WXD_EVENT_TYPE_STC_NEEDSHOWN = 311,                 // wxEVT_STC_NEEDSHOWN
    WXD_EVENT_TYPE_STC_PAINTED = 312,                   // wxEVT_STC_PAINTED
    WXD_EVENT_TYPE_STC_USERLISTSELECTION = 313,         // wxEVT_STC_USERLISTSELECTION
    WXD_EVENT_TYPE_STC_DWELLSTART = 314,                // wxEVT_STC_DWELLSTART
    WXD_EVENT_TYPE_STC_DWELLEND = 315,                  // wxEVT_STC_DWELLEND
    WXD_EVENT_TYPE_STC_START_DRAG = 316,                // wxEVT_STC_START_DRAG
    WXD_EVENT_TYPE_STC_DRAG_OVER = 317,                 // wxEVT_STC_DRAG_OVER
    WXD_EVENT_TYPE_STC_DO_DROP = 318,                   // wxEVT_STC_DO_DROP
    WXD_EVENT_TYPE_STC_ZOOM = 319,                      // wxEVT_STC_ZOOM
    WXD_EVENT_TYPE_STC_HOTSPOT_CLICK = 320,             // wxEVT_STC_HOTSPOT_CLICK
    WXD_EVENT_TYPE_STC_HOTSPOT_DCLICK = 321,            // wxEVT_STC_HOTSPOT_DCLICK
    WXD_EVENT_TYPE_STC_CALLTIP_CLICK = 322,             // wxEVT_STC_CALLTIP_CLICK
    WXD_EVENT_TYPE_STC_AUTOCOMP_SELECTION = 323,        // wxEVT_STC_AUTOCOMP_SELECTION
    WXD_EVENT_TYPE_STC_INDICATOR_CLICK = 324,           // wxEVT_STC_INDICATOR_CLICK
    WXD_EVENT_TYPE_STC_INDICATOR_RELEASE = 325,         // wxEVT_STC_INDICATOR_RELEASE
    WXD_EVENT_TYPE_STC_AUTOCOMP_CANCELLED = 326,        // wxEVT_STC_AUTOCOMP_CANCELLED
    WXD_EVENT_TYPE_STC_AUTOCOMP_CHAR_DELETED = 327,     // wxEVT_STC_AUTOCOMP_CHAR_DELETED
    #endif

    // TaskBarIcon event types
    WXD_EVENT_TYPE_TASKBAR_MOVE = 330,                  // wxEVT_TASKBAR_MOVE
    WXD_EVENT_TYPE_TASKBAR_LEFT_DOWN = 331,             // wxEVT_TASKBAR_LEFT_DOWN
    WXD_EVENT_TYPE_TASKBAR_LEFT_UP = 332,               // wxEVT_TASKBAR_LEFT_UP
    WXD_EVENT_TYPE_TASKBAR_RIGHT_DOWN = 333,            // wxEVT_TASKBAR_RIGHT_DOWN
    WXD_EVENT_TYPE_TASKBAR_RIGHT_UP = 334,              // wxEVT_TASKBAR_RIGHT_UP
    WXD_EVENT_TYPE_TASKBAR_LEFT_DCLICK = 335,           // wxEVT_TASKBAR_LEFT_DCLICK
    WXD_EVENT_TYPE_TASKBAR_RIGHT_DCLICK = 336,          // wxEVT_TASKBAR_RIGHT_DCLICK
    WXD_EVENT_TYPE_TASKBAR_BALLOON_TIMEOUT = 337,       // wxEVT_TASKBAR_BALLOON_TIMEOUT
    WXD_EVENT_TYPE_TASKBAR_BALLOON_CLICK = 338,         // wxEVT_TASKBAR_BALLOON_CLICK

    // NEW: Menu event types
    WXD_EVENT_TYPE_MENU_OPEN = 350,                     // wxEVT_MENU_OPEN
    WXD_EVENT_TYPE_MENU_CLOSE = 351,                    // wxEVT_MENU_CLOSE
    WXD_EVENT_TYPE_MENU_HIGHLIGHT = 352,                // wxEVT_MENU_HIGHLIGHT
    WXD_EVENT_TYPE_CONTEXT_MENU = 353,                  // wxEVT_CONTEXT_MENU

    WXD_EVENT_TYPE_MAX // Keep this last for count if needed, or remove if not used for iteration
} WXDEventTypeCEnum;

typedef int64_t wxd_Style_t;
typedef int wxd_Direction_t;
typedef int wxd_Orientation_t;
typedef int wxd_SizerFlags_t;

// --- Opaque Struct Typedefs --- 
typedef struct wxd_App_t wxd_App_t;
typedef struct wxd_Window_t wxd_Window_t;
typedef struct wxd_Event_t wxd_Event_t;
typedef struct wxd_EvtHandler_t wxd_EvtHandler_t;
typedef struct wxd_Control_t wxd_Control_t;
typedef struct wxd_Frame_t wxd_Frame_t;
typedef struct wxd_Button_t wxd_Button_t;
typedef struct wxd_Sizer_t wxd_Sizer_t;
typedef struct wxd_StaticText_t wxd_StaticText_t;
typedef struct wxd_TextCtrl_t wxd_TextCtrl_t;
typedef struct wxd_CheckBox_t wxd_CheckBox_t;
typedef struct wxd_Panel_t wxd_Panel_t;
typedef struct wxd_RadioButton_t wxd_RadioButton_t;
typedef struct wxd_ListBox_t wxd_ListBox_t;
typedef struct wxd_Choice_t wxd_Choice_t;
typedef struct wxd_ComboBox_t wxd_ComboBox_t;
typedef struct wxd_CheckListBox_t wxd_CheckListBox_t;
typedef struct wxd_ToggleButton_t wxd_ToggleButton_t;
typedef struct wxd_RadioBox_t wxd_RadioBox_t;
typedef struct wxd_BitmapComboBox_t wxd_BitmapComboBox_t;
typedef struct wxd_TreeCtrl_t wxd_TreeCtrl_t;
typedef struct wxd_TreeItemData_t wxd_TreeItemData_t;
typedef struct wxd_StaticBox_t wxd_StaticBox_t;
typedef struct wxd_Gauge_t wxd_Gauge_t;
typedef struct wxd_Slider_t wxd_Slider_t;
typedef struct wxd_SpinCtrl_t wxd_SpinCtrl_t;
typedef struct wxd_SpinButton_t wxd_SpinButton_t;
typedef struct wxd_Notebook_t wxd_Notebook_t;
typedef struct wxd_SimpleBook_t wxd_SimpleBook_t;
typedef struct wxd_SplitterWindow_t wxd_SplitterWindow_t;
typedef struct wxd_CollapsiblePane_t wxd_CollapsiblePane_t;
typedef struct wxd_Bitmap_t wxd_Bitmap_t;
typedef struct wxd_BitmapButton_t wxd_BitmapButton_t;
typedef struct wxd_ScrolledWindow_t wxd_ScrolledWindow_t;
typedef struct wxd_StatusBar_t wxd_StatusBar_t;
typedef struct wxd_ToolBar_t wxd_ToolBar_t;
typedef struct wxd_MenuBar_t wxd_MenuBar_t;
typedef struct wxd_Menu_t wxd_Menu_t;
typedef struct wxd_MenuItem_t wxd_MenuItem_t;
typedef struct wxd_ListCtrl_t wxd_ListCtrl_t;
typedef struct wxd_ColourPickerCtrl_t wxd_ColourPickerCtrl_t;
typedef struct wxd_DatePickerCtrl_t wxd_DatePickerCtrl_t;
typedef struct wxd_TimePickerCtrl_t wxd_TimePickerCtrl_t;
typedef struct wxd_Treebook wxd_Treebook_t;
typedef struct wxd_SearchCtrl wxd_SearchCtrl_t;
typedef struct wxd_HyperlinkCtrl wxd_HyperlinkCtrl_t;
typedef struct wxd_ActivityIndicator wxd_ActivityIndicator_t;
typedef struct wxd_SpinCtrlDouble wxd_SpinCtrlDouble_t;
typedef struct wxd_CalendarCtrl_t wxd_CalendarCtrl_t;
typedef struct wxd_FlexGridSizer_t wxd_FlexGridSizer_t;
typedef struct wxd_GridSizer_t wxd_GridSizer_t;
typedef struct wxd_GridBagSizer_t wxd_GridBagSizer_t;
typedef struct wxd_StaticBoxSizer_t wxd_StaticBoxSizer_t;
typedef struct wxd_WrapSizer_t wxd_WrapSizer_t;
typedef struct wxd_StaticBitmap_t wxd_StaticBitmap_t;
typedef struct wxd_GenericStaticBitmap_t wxd_GenericStaticBitmap_t;
typedef struct wxd_StaticLine_t wxd_StaticLine_t;
typedef struct wxd_ScrollBar_t wxd_ScrollBar_t;
typedef struct wxd_CommandLinkButton_t wxd_CommandLinkButton_t;
typedef struct wxd_Dialog wxd_Dialog_t;
typedef struct wxd_MessageDialog wxd_MessageDialog_t;
typedef struct wxd_ArrayString { void* internal_data; } wxd_ArrayString_t;
typedef struct wxd_FileDialog wxd_FileDialog_t;
typedef struct wxd_ColourData wxd_ColourData_t;
typedef struct wxd_ColourDialog wxd_ColourDialog_t;
typedef struct wxd_FontData wxd_FontData_t;
typedef struct wxd_Font_t wxd_Font_t;
typedef struct wxd_FontDialog wxd_FontDialog_t;
typedef struct wxd_TextEntryDialog wxd_TextEntryDialog_t;
typedef struct wxd_ProgressDialog wxd_ProgressDialog_t;
typedef struct wxd_SingleChoiceDialog wxd_SingleChoiceDialog_t;
typedef struct wxd_MultiChoiceDialog wxd_MultiChoiceDialog_t;
typedef struct wxd_DirDialog wxd_DirDialog_t;
typedef struct wxd_AnimationCtrl wxd_AnimationCtrl_t;
typedef struct wxd_FilePickerCtrl_t wxd_FilePickerCtrl_t;
typedef struct wxd_DirPickerCtrl_t wxd_DirPickerCtrl_t;
typedef struct wxd_FontPickerCtrl_t wxd_FontPickerCtrl_t;
typedef struct wxd_NotificationMessage_t wxd_NotificationMessage_t;
typedef struct wxd_FileCtrl_t wxd_FileCtrl_t;
typedef struct wxd_MediaCtrl_t wxd_MediaCtrl_t;
typedef struct wxd_RearrangeList_t wxd_RearrangeList_t;
typedef struct wxd_EditableListBox_t wxd_EditableListBox_t;
typedef struct wxd_Clipboard_t wxd_Clipboard_t;

// XRC-related opaque types
typedef struct wxd_XmlResource_t wxd_XmlResource_t;

typedef struct wxd_AuiMDIParentFrame_t wxd_AuiMDIParentFrame_t;
typedef struct wxd_AuiMDIChildFrame_t wxd_AuiMDIChildFrame_t;
typedef struct wxd_AuiManager_t wxd_AuiManager_t;
typedef struct wxd_AuiManagerEvent_t wxd_AuiManagerEvent_t;
typedef struct wxd_AuiNotebook_t wxd_AuiNotebook_t;
typedef struct wxd_AuiNotebookEvent_t wxd_AuiNotebookEvent_t;
typedef struct wxd_AuiPaneInfo_t wxd_AuiPaneInfo_t;

// wxAuiToolBar
typedef struct wxd_AuiToolBar_t wxd_AuiToolBar_t;

// Drag and drop related typedefs (opaque pointers)
typedef struct wxd_DataObject_t wxd_DataObject_t;
typedef struct wxd_TextDataObject_t wxd_TextDataObject_t;
typedef struct wxd_FileDataObject_t wxd_FileDataObject_t;
typedef struct wxd_BitmapDataObject_t wxd_BitmapDataObject_t;
typedef struct wxd_DropSource_t wxd_DropSource_t;
typedef struct wxd_DropTarget_t wxd_DropTarget_t;
typedef struct wxd_TextDropTarget_t wxd_TextDropTarget_t;
typedef struct wxd_FileDropTarget_t wxd_FileDropTarget_t;

// DC related typedefs (opaque pointers)
typedef struct wxd_DC_t wxd_DC_t;
typedef struct wxd_WindowDC_t wxd_WindowDC_t;
typedef struct wxd_ClientDC_t wxd_ClientDC_t;
typedef struct wxd_PaintDC_t wxd_PaintDC_t;
typedef struct wxd_MemoryDC_t wxd_MemoryDC_t;
typedef struct wxd_ScreenDC_t wxd_ScreenDC_t;
typedef struct wxd_AutoBufferedPaintDC_t wxd_AutoBufferedPaintDC_t;

// wxBitmap (placeholder for future use)
typedef struct wxd_Bitmap_t wxd_Bitmap_t;

// wxItemKind C Enum (for wxAuiToolBar, wxMenu, etc.)
typedef enum {
    WXD_ITEM_NORMAL = 0,     // wxITEM_NORMAL
    WXD_ITEM_CHECK = 1,      // wxITEM_CHECK
    WXD_ITEM_RADIO = 2,      // wxITEM_RADIO
    WXD_ITEM_SEPARATOR = 3,  // wxITEM_SEPARATOR
    // wxITEM_DROPDOWN (specific to wxToolBar and wxRibbonBar, might add later if needed)
    // wxITEM_MAX is not typically used directly as a kind
} WXDItemKindCEnum;

// --- Function Pointer Typedefs --- 
typedef bool (*wxd_OnInitCallback)(void* userData);
typedef void (*wxd_ClosureCallback)(void* closure_ptr, wxd_Event_t* event);

// wxDragResult C Enum (for drag and drop operations)
typedef enum {
    WXD_DRAG_NONE = 0,      // wxDragNone - No drag operation
    WXD_DRAG_COPY = 1,      // wxDragCopy - Copy the data
    WXD_DRAG_MOVE = 2,      // wxDragMove - Move the data
    WXD_DRAG_LINK = 3,      // wxDragLink - Link to the data
    WXD_DRAG_CANCEL = 4,    // wxDragCancel - Cancel the drag
    WXD_DRAG_ERROR = 5      // wxDragError - Error in drag operation
} wxd_DragResult;

// Define a long integer type for positions, lengths, etc.
typedef long long wxd_Long_t;

// ListCtrl constants
#define WXD_LIST_MASK_STATE         0x0001
#define WXD_LIST_MASK_TEXT          0x0002
#define WXD_LIST_MASK_IMAGE         0x0004
#define WXD_LIST_MASK_DATA          0x0008
#define WXD_LIST_MASK_WIDTH         0x0010
#define WXD_LIST_MASK_FORMAT        0x0020

// DataView item type
typedef struct {
    void* id;  // Internally, wxDataViewItem wraps a void* as an id
} wxd_DataViewItemWithID_t;

// DataView types
typedef void wxd_DataViewModel_t;
//typedef void wxd_DataViewItem_t;  // Already defined above as a struct
typedef void wxd_DataViewColumn_t;

// DataViewCell mode enum (for cell renderers)
typedef enum {
    WXD_DATAVIEW_CELL_INERT,
    WXD_DATAVIEW_CELL_ACTIVATABLE, 
    WXD_DATAVIEW_CELL_EDITABLE
} wxd_DataViewCellModeCEnum;

// Image related opaque types
typedef void wxd_Image_t;          // Added if not present
typedef void wxd_ImageList_t;      // Added
typedef struct wxd_Icon_t wxd_Icon_t; // Assuming this pattern from Bitmap_t

// --- Cursor type ---
typedef struct wxd_Cursor_t wxd_Cursor_t;

// --- Cursor Stock Types ---
typedef enum {
    WXD_CURSOR_NONE = 0,
    WXD_CURSOR_ARROW = 1,           // Standard arrow cursor
    WXD_CURSOR_RIGHT_ARROW = 2,     // Standard arrow pointing right
    WXD_CURSOR_BULLSEYE = 3,        // Bullseye cursor
    WXD_CURSOR_CHAR = 4,            // Rectangular character cursor
    WXD_CURSOR_CROSS = 5,           // Cross cursor
    WXD_CURSOR_HAND = 6,            // Hand cursor
    WXD_CURSOR_IBEAM = 7,           // I-beam cursor for text
    WXD_CURSOR_LEFT_BUTTON = 8,     // Represents a left mouse button
    WXD_CURSOR_MAGNIFIER = 9,       // Magnifying glass cursor
    WXD_CURSOR_MIDDLE_BUTTON = 10,  // Represents a middle mouse button
    WXD_CURSOR_NO_ENTRY = 11,       // No entry sign cursor
    WXD_CURSOR_PAINT_BRUSH = 12,    // Paint brush cursor
    WXD_CURSOR_PENCIL = 13,         // Pencil cursor
    WXD_CURSOR_POINT_LEFT = 14,     // Cursor pointing left
    WXD_CURSOR_POINT_RIGHT = 15,    // Cursor pointing right
    WXD_CURSOR_QUESTION_ARROW = 16, // Question mark cursor
    WXD_CURSOR_RIGHT_BUTTON = 17,   // Represents a right mouse button
    WXD_CURSOR_SIZENESW = 18,       // Resize cursor NE-SW
    WXD_CURSOR_SIZENS = 19,         // Resize cursor N-S
    WXD_CURSOR_SIZENWSE = 20,       // Resize cursor NW-SE
    WXD_CURSOR_SIZEWE = 21,         // Resize cursor W-E
    WXD_CURSOR_SIZING = 22,         // General sizing cursor
    WXD_CURSOR_SPRAYCAN = 23,       // Spray can cursor
    WXD_CURSOR_WAIT = 24,           // Hourglass/spinning wheel cursor
    WXD_CURSOR_WATCH = 25,          // Watch cursor
    WXD_CURSOR_BLANK = 26,          // Invisible cursor
    WXD_CURSOR_DEFAULT = 27,        // Default cursor for the platform
    WXD_CURSOR_ARROWWAIT = 28,      // Arrow with small hourglass
    WXD_CURSOR_MAX                  // Number of stock cursors
} wxd_StockCursor;

// --- Bitmap Types for Cursor Creation ---
typedef enum {
    WXD_BITMAP_TYPE_INVALID = 0,
    WXD_BITMAP_TYPE_BMP = 1,
    WXD_BITMAP_TYPE_ICO = 2,
    WXD_BITMAP_TYPE_CUR = 3,
    WXD_BITMAP_TYPE_XBM = 4,
    WXD_BITMAP_TYPE_XPM = 5,
    WXD_BITMAP_TYPE_PNG = 6,
    WXD_BITMAP_TYPE_JPEG = 7,
    WXD_BITMAP_TYPE_GIF = 8,
    WXD_BITMAP_TYPE_ANI = 9,
    WXD_BITMAP_TYPE_ANY = 50
} wxd_BitmapType;

// For wxd_TreeItemId_t, wxTreeItemId internally holds a void* m_pItem.
// We will pass this void* directly as an opaque pointer type for Rust.
// struct wxd_TreeItemId_s; // Remove old forward declaration
// typedef struct wxd_TreeItemId_s wxd_TreeItemId_t; // Remove old typedef
typedef struct wxd_TreeItemId_Opaque_ForBindgen wxd_TreeItemId_t; // Define as opaque struct for bindgen

// Define wxd_DropResult enum (ensure this is defined if used)
// ... existing code ...

// --- Opaque and FFI Struct Definitions ---
// Moved from wxd_event_api.h to ensure it's defined before use in event.cpp
typedef struct wxd_Event_t wxd_Event_t;

typedef struct wxd_App_t wxd_App_t;
typedef struct wxd_Window_t wxd_Window_t;
// typedef struct wxd_Event_t wxd_Event_t; // Now defined above
typedef struct wxd_EvtHandler_t wxd_EvtHandler_t;
typedef struct wxd_Control_t wxd_Control_t;
// ... existing code ...

typedef struct wxd_BitmapBundle_t wxd_BitmapBundle_t;

/// Opaque pointer to wxFont
typedef struct wxd_Font_t wxd_Font_t;

/// Opaque pointer to wxWindow
typedef struct wxd_Window_t wxd_Window_t;

/// Opaque pointer to wxTimer
typedef struct wxd_Timer_t wxd_Timer_t;

/// Window ID type (must match wxWidgets window ID type)
typedef int wxd_Id;

// StyledTextCtrl type
typedef struct wxd_StyledTextCtrl_t wxd_StyledTextCtrl_t;

// AppProgressIndicator type
typedef struct wxd_AppProgressIndicator_t wxd_AppProgressIndicator_t;

// --- Appearance Support (wxWidgets 3.3.0+) ---

// Appearance modes for dark mode support
typedef enum {
    WXD_APPEARANCE_LIGHT = 0,   // Force light mode
    WXD_APPEARANCE_DARK = 1,    // Force dark mode
    WXD_APPEARANCE_SYSTEM = 2   // Follow system appearance
} wxd_Appearance;

// AppearanceResult for SetAppearance method
typedef enum {
    WXD_APPEARANCE_RESULT_OK = 0,           // Success
    WXD_APPEARANCE_RESULT_FAILURE = 1,      // Failed to set appearance
    WXD_APPEARANCE_RESULT_CANNOT_CHANGE = 2 // Cannot change (e.g., windows already exist)
} wxd_AppearanceResult;

// Forward declarations for appearance classes
typedef struct wxd_SystemAppearance_t wxd_SystemAppearance_t;

// --- End of Appearance Support ---

// RichText types
typedef struct wxd_RichTextCtrl_t wxd_RichTextCtrl_t;
typedef struct wxd_TreeListCtrl_t wxd_TreeListCtrl_t;

#endif // WXD_TYPES_H 