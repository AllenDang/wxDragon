#ifndef WXD_TOOLBAR_H
#define WXD_TOOLBAR_H

#include "../wxd_types.h"

// --- ToolBar Functions ---
WXD_EXPORTED wxd_ToolBar_t*
wxd_ToolBar_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                   wxd_Style_t style);

WXD_EXPORTED void*
wxd_ToolBar_AddTool(wxd_ToolBar_t* self, wxd_Id toolId, const char* label,
                    const wxd_Bitmap_t* bitmap, const wxd_Bitmap_t* bitmapDisabled, int kind,
                    const char* shortHelp, const char* longHelp);

WXD_EXPORTED void
wxd_ToolBar_AddSeparator(wxd_ToolBar_t* self);

WXD_EXPORTED void
wxd_ToolBar_AddControl(wxd_ToolBar_t* self, wxd_Window_t* control);

WXD_EXPORTED bool
wxd_ToolBar_Realize(wxd_ToolBar_t* self);

WXD_EXPORTED void
wxd_ToolBar_EnableTool(wxd_ToolBar_t* self, wxd_Id toolId, bool enable);

WXD_EXPORTED void
wxd_ToolBar_ToggleTool(wxd_ToolBar_t* self, wxd_Id toolId, bool toggle);

WXD_EXPORTED bool
wxd_ToolBar_IsToolEnabled(wxd_ToolBar_t* self, wxd_Id toolId);

WXD_EXPORTED bool
wxd_ToolBar_GetToolState(wxd_ToolBar_t* self, wxd_Id toolId);

WXD_EXPORTED void
wxd_ToolBar_SetToolShortHelp(wxd_ToolBar_t* self, wxd_Id toolId, const char* helpString);

// BitmapBundle support
WXD_EXPORTED bool
wxd_ToolBar_AddToolWithBundle(wxd_ToolBar_t* toolbar, wxd_Id id, const char* label,
                              wxd_BitmapBundle_t* bitmap);

WXD_EXPORTED bool
wxd_ToolBar_AddToolWithBundles(wxd_ToolBar_t* toolbar, wxd_Id id, const char* label,
                               wxd_BitmapBundle_t* bitmap, wxd_BitmapBundle_t* bitmapDisabled,
                               const char* shortHelp, const char* longHelp);

#endif // WXD_TOOLBAR_H