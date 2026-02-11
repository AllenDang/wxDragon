#ifndef WXD_ACCESSIBLE_H
#define WXD_ACCESSIBLE_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// --- Accessibility Enums ---

typedef enum {
    WXD_ACC_FAIL,
    WXD_ACC_FALSE,
    WXD_ACC_OK,
    WXD_ACC_NOT_IMPLEMENTED,
    WXD_ACC_NOT_SUPPORTED,
    WXD_ACC_INVALID_ARG
} wxd_AccStatus;

typedef enum {
    WXD_NAVDIR_DOWN,
    WXD_NAVDIR_FIRSTCHILD,
    WXD_NAVDIR_LASTCHILD,
    WXD_NAVDIR_LEFT,
    WXD_NAVDIR_NEXT,
    WXD_NAVDIR_PREVIOUS,
    WXD_NAVDIR_RIGHT,
    WXD_NAVDIR_UP
} wxd_NavDir;

// Selection flags
#define WXD_ACC_SEL_NONE            0
#define WXD_ACC_SEL_TAKEFOCUS       1
#define WXD_ACC_SEL_TAKESELECTION   2
#define WXD_ACC_SEL_EXTENDSELECTION 4
#define WXD_ACC_SEL_ADDSELECTION    8
#define WXD_ACC_SEL_REMOVESELECTION 16

// Object types for NotifyEvent
typedef enum {
    WXD_ACC_OBJ_WINDOW = 0x00000000,
    WXD_ACC_OBJ_SYSMENU = 0xFFFFFFFF,
    WXD_ACC_OBJ_TITLEBAR = 0xFFFFFFFE,
    WXD_ACC_OBJ_MENU = 0xFFFFFFFD,
    WXD_ACC_OBJ_CLIENT = 0xFFFFFFFC,
    WXD_ACC_OBJ_VSCROLL = 0xFFFFFFFB,
    WXD_ACC_OBJ_HSCROLL = 0xFFFFFFFA,
    WXD_ACC_OBJ_SIZEGRIP = 0xFFFFFFF9,
    WXD_ACC_OBJ_CARET = 0xFFFFFFF8,
    WXD_ACC_OBJ_CURSOR = 0xFFFFFFF7,
    WXD_ACC_OBJ_ALERT = 0xFFFFFFF6,
    WXD_ACC_OBJ_SOUND = 0xFFFFFFF5
} wxd_AccObjectType;

// Common Roles (subset of wxAccRole)
typedef enum {
    WXD_ROLE_NONE,
    WXD_ROLE_SYSTEM_TITLEBAR,
    WXD_ROLE_SYSTEM_MENUBAR,
    WXD_ROLE_SYSTEM_SCROLLBAR,
    WXD_ROLE_SYSTEM_GRIP,
    WXD_ROLE_SYSTEM_SOUND,
    WXD_ROLE_SYSTEM_CURSOR,
    WXD_ROLE_SYSTEM_CARET,
    WXD_ROLE_SYSTEM_ALERT,
    WXD_ROLE_SYSTEM_WINDOW,
    WXD_ROLE_SYSTEM_CLIENT,
    WXD_ROLE_SYSTEM_MENUPOPUP,
    WXD_ROLE_SYSTEM_MENUITEM,
    WXD_ROLE_SYSTEM_TOOLTIP,
    WXD_ROLE_SYSTEM_APPLICATION,
    WXD_ROLE_SYSTEM_DOCUMENT,
    WXD_ROLE_SYSTEM_PANE,
    WXD_ROLE_SYSTEM_CHART,
    WXD_ROLE_SYSTEM_DIALOG,
    WXD_ROLE_SYSTEM_BORDER,
    WXD_ROLE_SYSTEM_GROUPING,
    WXD_ROLE_SYSTEM_SEPARATOR,
    WXD_ROLE_SYSTEM_TOOLBAR,
    WXD_ROLE_SYSTEM_STATUSBAR,
    WXD_ROLE_SYSTEM_TABLE,
    WXD_ROLE_SYSTEM_COLUMNHEADER,
    WXD_ROLE_SYSTEM_ROWHEADER,
    WXD_ROLE_SYSTEM_COLUMN,
    WXD_ROLE_SYSTEM_ROW,
    WXD_ROLE_SYSTEM_CELL,
    WXD_ROLE_SYSTEM_LINK,
    WXD_ROLE_SYSTEM_HELPBALLOON,
    WXD_ROLE_SYSTEM_CHARACTER,
    WXD_ROLE_SYSTEM_LIST,
    WXD_ROLE_SYSTEM_LISTITEM,
    WXD_ROLE_SYSTEM_OUTLINE,
    WXD_ROLE_SYSTEM_OUTLINEITEM,
    WXD_ROLE_SYSTEM_PAGETAB,
    WXD_ROLE_SYSTEM_PROPERTYPAGE,
    WXD_ROLE_SYSTEM_INDICATOR,
    WXD_ROLE_SYSTEM_GRAPHIC,
    WXD_ROLE_SYSTEM_STATICTEXT,
    WXD_ROLE_SYSTEM_TEXT,
    WXD_ROLE_SYSTEM_PUSHBUTTON,
    WXD_ROLE_SYSTEM_CHECKBUTTON,
    WXD_ROLE_SYSTEM_RADIOBUTTON,
    WXD_ROLE_SYSTEM_COMBOBOX,
    WXD_ROLE_SYSTEM_DROPLIST,
    WXD_ROLE_SYSTEM_PROGRESSBAR,
    WXD_ROLE_SYSTEM_DIAL,
    WXD_ROLE_SYSTEM_HOTKEYFIELD,
    WXD_ROLE_SYSTEM_SLIDER,
    WXD_ROLE_SYSTEM_SPINBUTTON,
    WXD_ROLE_SYSTEM_DIAGRAM,
    WXD_ROLE_SYSTEM_ANIMATION,
    WXD_ROLE_SYSTEM_EQUATION,
    WXD_ROLE_SYSTEM_BUTTONDROPDOWN,
    WXD_ROLE_SYSTEM_BUTTONMENU,
    WXD_ROLE_SYSTEM_BUTTONDROPDOWNGRID,
    WXD_ROLE_SYSTEM_WHITESPACE,
    WXD_ROLE_SYSTEM_PAGETABLIST,
    WXD_ROLE_SYSTEM_CLOCK,
    WXD_ROLE_SYSTEM_SPLITBUTTON,
    WXD_ROLE_SYSTEM_IPADDRESS,
    WXD_ROLE_SYSTEM_OUTLINEBUTTON
} wxd_AccRole;

// Common States (subset of wxAccStatus)
#define WXD_ACC_STATE_SYSTEM_UNAVAILABLE     0x00000001
#define WXD_ACC_STATE_SYSTEM_SELECTED        0x00000002
#define WXD_ACC_STATE_SYSTEM_FOCUSED         0x00000004
#define WXD_ACC_STATE_SYSTEM_PRESSED         0x00000008
#define WXD_ACC_STATE_SYSTEM_CHECKED         0x00000010
#define WXD_ACC_STATE_SYSTEM_MIXED           0x00000020
#define WXD_ACC_STATE_SYSTEM_INDETERMINATE   WXD_ACC_STATE_SYSTEM_MIXED
#define WXD_ACC_STATE_SYSTEM_READONLY        0x00000040
#define WXD_ACC_STATE_SYSTEM_HOTTRACKED      0x00000080
#define WXD_ACC_STATE_SYSTEM_DEFAULT         0x00000100
#define WXD_ACC_STATE_SYSTEM_EXPANDED        0x00000200
#define WXD_ACC_STATE_SYSTEM_COLLAPSED       0x00000400
#define WXD_ACC_STATE_SYSTEM_BUSY            0x00000800
#define WXD_ACC_STATE_SYSTEM_FLOATING        0x00001000
#define WXD_ACC_STATE_SYSTEM_MARQUEED        0x00002000
#define WXD_ACC_STATE_SYSTEM_ANIMATED        0x00004000
#define WXD_ACC_STATE_SYSTEM_INVISIBLE       0x00008000
#define WXD_ACC_STATE_SYSTEM_OFFSCREEN       0x00010000
#define WXD_ACC_STATE_SYSTEM_SIZEABLE        0x00020000
#define WXD_ACC_STATE_SYSTEM_MOVEABLE        0x00040000
#define WXD_ACC_STATE_SYSTEM_SELFVOICING     0x00080000
#define WXD_ACC_STATE_SYSTEM_FOCUSABLE       0x00100000
#define WXD_ACC_STATE_SYSTEM_SELECTABLE      0x00200000
#define WXD_ACC_STATE_SYSTEM_LINKED          0x00400000
#define WXD_ACC_STATE_SYSTEM_TRAVERSED       0x00800000
#define WXD_ACC_STATE_SYSTEM_MULTISELECTABLE 0x01000000
#define WXD_ACC_STATE_SYSTEM_EXTSELECTABLE   0x02000000
#define WXD_ACC_STATE_SYSTEM_ALERT_LOW       0x04000000
#define WXD_ACC_STATE_SYSTEM_ALERT_MEDIUM    0x08000000
#define WXD_ACC_STATE_SYSTEM_ALERT_HIGH      0x10000000
#define WXD_ACC_STATE_SYSTEM_PROTECTED       0x20000000
#define WXD_ACC_STATE_SYSTEM_HASPOPUP        0x40000000

// --- Callback Structure for Custom Accessible ---

typedef struct {
    wxd_AccStatus (*GetChildCount)(void* userData, int* count);
    wxd_AccStatus (*GetChild)(void* userData, int childId, wxd_Accessible_t** child);
    wxd_AccStatus (*GetParent)(void* userData, wxd_Accessible_t** parent);
    wxd_AccStatus (*GetRole)(void* userData, int childId, wxd_AccRole* role);
    wxd_AccStatus (*GetState)(void* userData, int childId, long* state);
    wxd_AccStatus (*GetName)(void* userData, int childId, char* outName, size_t maxLen);
    wxd_AccStatus (*GetDescription)(void* userData, int childId, char* outDescription, size_t maxLen);
    wxd_AccStatus (*GetHelpText)(void* userData, int childId, char* outHelpText, size_t maxLen);
    wxd_AccStatus (*GetKeyboardShortcut)(void* userData, int childId, char* outShortcut, size_t maxLen);
    wxd_AccStatus (*GetDefaultAction)(void* userData, int childId, char* outAction, size_t maxLen);
    wxd_AccStatus (*GetValue)(void* userData, int childId, char* outValue, size_t maxLen);
    wxd_AccStatus (*Select)(void* userData, int childId, int selectFlags);
    wxd_AccStatus (*GetSelections)(void* userData, wxd_Variant_t* selections);
    wxd_AccStatus (*GetFocus)(void* userData, int* childId, wxd_Accessible_t** child);
    wxd_AccStatus (*DoDefaultAction)(void* userData, int childId);
    wxd_AccStatus (*GetLocation)(void* userData, int childId, wxd_Rect* rect);
    wxd_AccStatus (*HitTest)(void* userData, wxd_Point pt, int* childId, wxd_Accessible_t** childObject);
    wxd_AccStatus (*Navigate)(void* userData, wxd_NavDir navDir, int fromId, int* toId, wxd_Accessible_t** toObject);
} wxd_AccessibleCallbacks;

// --- Accessible Functions ---

WXD_EXPORTED wxd_Accessible_t*
wxd_Accessible_Create(wxd_Window_t* window, wxd_AccessibleCallbacks callbacks, void* userData);

WXD_EXPORTED void
wxd_Accessible_Destroy(wxd_Accessible_t* self);

WXD_EXPORTED void
wxd_Accessible_NotifyEvent(uint32_t eventType, wxd_Window_t* window, int objectType, int objectId);

// --- Window Accessibility Functions ---

/**
 * @brief Set the accessible object for the window.
 * The window takes ownership of the accessible object.
 */
WXD_EXPORTED void
wxd_Window_SetAccessible(wxd_Window_t* self, wxd_Accessible_t* accessible);

/**
 * @brief Get the accessible object for the window.
 * The returned pointer is owned by the window.
 */
WXD_EXPORTED wxd_Accessible_t*
wxd_Window_GetAccessible(wxd_Window_t* self);

#ifdef __cplusplus
}
#endif

#endif // WXD_ACCESSIBLE_H
