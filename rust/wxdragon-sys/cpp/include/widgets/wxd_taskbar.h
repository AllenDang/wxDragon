#pragma once

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Forward declarations
typedef struct wxd_TaskBarIcon_t wxd_TaskBarIcon_t;
typedef struct wxd_Menu_t wxd_Menu_t;
typedef struct wxd_Bitmap_t wxd_Bitmap_t;
typedef struct wxd_BitmapBundle_t wxd_BitmapBundle_t;

// TaskBarIcon type enumeration
typedef enum {
    WXD_TASKBAR_ICON_DEFAULT = 0,
    WXD_TASKBAR_ICON_DOCK = 1,
    WXD_TASKBAR_ICON_CUSTOM_STATUSITEM = 2
} wxd_TaskBarIconType_t;

// TaskBarIcon creation and destruction
wxd_TaskBarIcon_t*
wxd_TaskBarIcon_Create(wxd_TaskBarIconType_t iconType);

void
wxd_TaskBarIcon_Destroy(wxd_TaskBarIcon_t* taskbar);

// Icon operations
bool
wxd_TaskBarIcon_SetIcon(wxd_TaskBarIcon_t* taskbar, const wxd_Bitmap_t* icon, const char* tooltip);

bool
wxd_TaskBarIcon_SetIconBundle(wxd_TaskBarIcon_t* taskbar, const wxd_BitmapBundle_t* iconBundle,
                              const char* tooltip);

bool
wxd_TaskBarIcon_RemoveIcon(wxd_TaskBarIcon_t* taskbar);

bool
wxd_TaskBarIcon_IsIconInstalled(wxd_TaskBarIcon_t* taskbar);

// Balloon tooltip operations (Windows only)
bool
wxd_TaskBarIcon_ShowBalloon(wxd_TaskBarIcon_t* taskbar, const char* title, const char* text,
                            unsigned int timeout, int flags, const wxd_BitmapBundle_t* icon);

// Menu operations
bool
wxd_TaskBarIcon_PopupMenu(wxd_TaskBarIcon_t* taskbar, wxd_Menu_t* menu);

// Automatic popup menu operations
void
wxd_TaskBarIcon_SetPopupMenu(wxd_TaskBarIcon_t* taskbar, wxd_Menu_t* menu);

const wxd_Menu_t*
wxd_TaskBarIcon_GetPopupMenu(wxd_TaskBarIcon_t* taskbar);

// Event handler pointer for binding events
wxd_EvtHandler_t*
wxd_TaskBarIcon_GetEvtHandler(wxd_TaskBarIcon_t* taskbar);

#ifdef __cplusplus
}
#endif