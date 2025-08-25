#ifndef WXD_WINDOW_BASE_H
#define WXD_WINDOW_BASE_H

#include "../wxd_types.h"

// --- Common Window Functions ---
WXD_EXPORTED void wxd_Window_SetSizer(wxd_Window_t* window, wxd_Sizer_t* sizer, bool deleteOldSizer);
WXD_EXPORTED void wxd_Window_SetSizerAndFit(wxd_Window_t* window, wxd_Sizer_t* sizer, bool deleteOldSizer);
WXD_EXPORTED wxd_Sizer_t* wxd_Window_GetSizer(wxd_Window_t* window);
WXD_EXPORTED int wxd_Window_GetId(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_Fit(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_Layout(wxd_Window_t* window);
WXD_EXPORTED wxd_Size wxd_Window_GetBestSize(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_Destroy(wxd_Window_t* window); // Generic destroy
WXD_EXPORTED void wxd_Window_SetBackgroundColor(wxd_Window_t* window, wxd_Colour_t color);
WXD_EXPORTED void wxd_Window_SetMinSize(wxd_Window_t* window, wxd_Size size);
WXD_EXPORTED void wxd_Window_Refresh(wxd_Window_t* window, int eraseBackground, const wxd_Rect* rect);
WXD_EXPORTED void wxd_Window_SetToolTip(wxd_Window_t* window, const char* tipString);

// Size and position related functions
WXD_EXPORTED wxd_Size wxd_Window_GetSize(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_SetSize(wxd_Window_t* window, wxd_Size size);
WXD_EXPORTED void wxd_Window_SetSizeWithPos(wxd_Window_t* window, int x, int y, int width, int height, int sizeFlags);
WXD_EXPORTED wxd_Size wxd_Window_GetClientSize(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_SetClientSize(wxd_Window_t* window, wxd_Size size);
WXD_EXPORTED wxd_Size wxd_Window_GetMinSize(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_Move(wxd_Window_t* window, int x, int y);
WXD_EXPORTED void wxd_Window_Center(wxd_Window_t* window);
WXD_EXPORTED wxd_Point wxd_Window_ClientToScreen(wxd_Window_t* window, wxd_Point pt);
WXD_EXPORTED wxd_Point wxd_Window_ScreenToClient(wxd_Window_t* window, wxd_Point pt);

// Declarations for functions that were previously in wxdragon.h directly
WXD_EXPORTED void wxd_Window_Show(wxd_Window_t* self, bool show);
WXD_EXPORTED bool wxd_Window_Close(wxd_Window_t* self, bool force);
WXD_EXPORTED void wxd_Window_SetId(wxd_Window_t* self, int id);
WXD_EXPORTED void wxd_Window_SetLabel(wxd_Window_t* self, const char* label);
WXD_EXPORTED char* wxd_Window_GetLabel(wxd_Window_t* self); // Caller must free with wxd_free_string

WXD_EXPORTED bool wxd_Window_IsEnabled(wxd_Window_t *self);
WXD_EXPORTED void wxd_Window_Enable(wxd_Window_t *self, bool enable);

WXD_EXPORTED wxd_Window_t* wxd_Window_GetParent(wxd_Window_t* self);
WXD_EXPORTED wxd_Window_t* wxd_Window_GetGrandParent(wxd_Window_t* self);

WXD_EXPORTED void wxd_Window_SetFont(wxd_Window_t* self, const wxd_Font_t* font);
WXD_EXPORTED wxd_Font_t* wxd_Window_GetFont(wxd_Window_t* self);
WXD_EXPORTED wxd_Point wxd_Window_GetPosition(wxd_Window_t* self);

// Background style functions
WXD_EXPORTED void wxd_Window_SetBackgroundStyle(wxd_Window_t* window, int style);
WXD_EXPORTED int wxd_Window_GetBackgroundStyle(wxd_Window_t* window);

// Extra window style functions  
WXD_EXPORTED void wxd_Window_SetExtraStyle(wxd_Window_t* window, int64_t exStyle);
WXD_EXPORTED int64_t wxd_Window_GetExtraStyle(wxd_Window_t* window);

// Color management functions
WXD_EXPORTED void wxd_Window_SetForegroundColor(wxd_Window_t* window, wxd_Colour_t color);
WXD_EXPORTED wxd_Colour_t wxd_Window_GetForegroundColor(wxd_Window_t* window);
WXD_EXPORTED wxd_Colour_t wxd_Window_GetBackgroundColor(wxd_Window_t* window);

// Focus management functions
WXD_EXPORTED void wxd_Window_SetFocus(wxd_Window_t* window);
WXD_EXPORTED bool wxd_Window_HasFocus(wxd_Window_t* window);
WXD_EXPORTED bool wxd_Window_CanAcceptFocus(wxd_Window_t* window);

// Visibility functions
WXD_EXPORTED bool wxd_Window_IsShown(wxd_Window_t* window);

// Size constraint functions
WXD_EXPORTED void wxd_Window_SetMaxSize(wxd_Window_t* window, wxd_Size size);
WXD_EXPORTED wxd_Size wxd_Window_GetMaxSize(wxd_Window_t* window);

// Window properties functions
WXD_EXPORTED void wxd_Window_SetName(wxd_Window_t* window, const char* name);
WXD_EXPORTED char* wxd_Window_GetName(wxd_Window_t* window); // Caller must free with wxd_free_string

// Window finding functions
WXD_EXPORTED wxd_Window_t* wxd_Window_FindWindowByName(wxd_Window_t* window, const char* name);
WXD_EXPORTED wxd_Window_t* wxd_Window_FindWindowById(wxd_Window_t* window, int id);

// --- Cursor Management Functions ---
WXD_EXPORTED void wxd_Window_SetCursor(wxd_Window_t* window, wxd_Cursor_t* cursor);
WXD_EXPORTED wxd_Cursor_t* wxd_Window_GetCursor(wxd_Window_t* window);

// --- Z-Order Management Functions ---
WXD_EXPORTED void wxd_Window_Raise(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_Lower(wxd_Window_t* window);

// --- Mouse Capture Functions ---
WXD_EXPORTED void wxd_Window_CaptureMouse(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_ReleaseMouse(wxd_Window_t* window);
WXD_EXPORTED bool wxd_Window_HasCapture(wxd_Window_t* window);
WXD_EXPORTED wxd_Window_t* wxd_Window_GetCapture(); // Static function

// --- Text Measurement Functions ---
WXD_EXPORTED wxd_Size wxd_Window_GetTextExtent(wxd_Window_t* window, const char* text);
WXD_EXPORTED void wxd_Window_GetFullTextExtent(wxd_Window_t* window, const char* text, wxd_Size* size, int* descent, int* external_leading, wxd_Font_t* font);
WXD_EXPORTED int wxd_Window_GetCharHeight(wxd_Window_t* window);
WXD_EXPORTED int wxd_Window_GetCharWidth(wxd_Window_t* window);

// --- Window Style Functions ---
WXD_EXPORTED void wxd_Window_SetWindowStyle(wxd_Window_t* window, int64_t style);
WXD_EXPORTED int64_t wxd_Window_GetWindowStyle(wxd_Window_t* window);

// --- Scrolling Functions ---
WXD_EXPORTED void wxd_Window_ShowPosition(wxd_Window_t* window, int64_t position);
WXD_EXPORTED void wxd_Window_ScrollIntoView(wxd_Window_t* window, int64_t position, int keyCode);
WXD_EXPORTED bool wxd_Window_IsPositionVisible(wxd_Window_t* window, int64_t position);
WXD_EXPORTED int64_t wxd_Window_GetLastPosition(wxd_Window_t* window);

// Widget type identification using wxWidgets' built-in RTTI
WXD_EXPORTED const char* wxd_Window_GetClassName(wxd_Window_t* window);

// --- Platform-specific Functions ---
/// Gets the native handle of the window (platform-specific)
WXD_EXPORTED void* wxd_Window_GetHandle(wxd_Window_t* window);

#endif // WXD_WINDOW_BASE_H 