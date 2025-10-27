#ifndef WXD_FRAME_H
#define WXD_FRAME_H

#include "../wxd_types.h"

// --- Frame Functions ---
WXD_EXPORTED wxd_Frame_t*
wxd_Frame_Create(wxd_Window_t* parent, wxd_Id id, const char* title, wxd_Point pos, wxd_Size size,
                 wxd_Style_t style);

WXD_EXPORTED void
wxd_Frame_Destroy(wxd_Frame_t* frame);

WXD_EXPORTED void
wxd_Frame_Show(wxd_Frame_t* frame, bool show);

WXD_EXPORTED void
wxd_Frame_SetTitle(wxd_Frame_t* frame, const char* title);

WXD_EXPORTED void
wxd_Frame_Centre(wxd_Frame_t* frame, wxd_Direction_t direction);

WXD_EXPORTED void
wxd_Frame_Close(wxd_Frame_t* frame, bool force);

WXD_EXPORTED void
wxd_Frame_SetMenuBar(wxd_Frame_t* frame, wxd_MenuBar_t* menubar);

WXD_EXPORTED wxd_MenuBar_t*
wxd_Frame_GetMenuBar(wxd_Frame_t* frame);

WXD_EXPORTED void
wxd_Frame_SetStatusBar(wxd_Frame_t* frame, wxd_StatusBar_t* statusBar);

WXD_EXPORTED void
wxd_Frame_SetToolBar(wxd_Frame_t* frame, wxd_ToolBar_t* toolBar);

WXD_EXPORTED wxd_ToolBar_t*
wxd_Frame_CreateToolBar(wxd_Frame_t* frame, wxd_Style_t style, wxd_Id id);

WXD_EXPORTED wxd_StatusBar_t*
wxd_Frame_CreateStatusBar(wxd_Frame_t* frame, int number, int64_t style, int id, const char* name);

WXD_EXPORTED void
wxd_Frame_CenterOnScreen(wxd_Frame_t* frame);

WXD_EXPORTED void
wxd_Frame_SetStatusText(wxd_Frame_t* frame, const char* text, int number);

WXD_EXPORTED char*
wxd_Frame_GetTitle(wxd_Frame_t* frame);

WXD_EXPORTED void
wxd_Frame_Iconize(wxd_Frame_t* frame, bool iconize);

WXD_EXPORTED bool
wxd_Frame_IsIconized(wxd_Frame_t* frame);

WXD_EXPORTED void
wxd_Frame_Maximize(wxd_Frame_t* frame, bool maximize);

WXD_EXPORTED bool
wxd_Frame_IsMaximized(wxd_Frame_t* frame);

WXD_EXPORTED void
wxd_Frame_SetIconFromBitmap(wxd_Frame_t* frame, const wxd_Bitmap_t* bitmap);

#endif // WXD_FRAME_H