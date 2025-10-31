#ifndef WXD_AUI_H
#define WXD_AUI_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// --- wxAuiManager ---
WXD_EXPORTED wxd_AuiManager_t*
wxd_AuiManager_Create();

WXD_EXPORTED void
wxd_AuiManager_SetManagedWindow(wxd_AuiManager_t* self, wxd_Window_t* managed_window);

WXD_EXPORTED wxd_Window_t*
wxd_AuiManager_GetManagedWindow(wxd_AuiManager_t* self);

WXD_EXPORTED void
wxd_AuiManager_UnInit(wxd_AuiManager_t* self);

WXD_EXPORTED bool
wxd_AuiManager_AddPane(wxd_AuiManager_t* self, wxd_Window_t* window, int direction,
                       const char* caption);

WXD_EXPORTED bool
wxd_AuiManager_AddPaneWithInfo(wxd_AuiManager_t* self, wxd_Window_t* window,
                               wxd_AuiPaneInfo_t* pane_info);

WXD_EXPORTED bool
wxd_AuiManager_Update(wxd_AuiManager_t* self);

WXD_EXPORTED void
wxd_AuiManager_Delete(wxd_AuiManager_t* self);

/**
 * Serialize the current AUI layout to a string perspective.
 * The returned value is the required UTF-8 byte length (excluding the null terminator),
 * if any error, -1 returned.
 * If out is not null and out_len > 0, copies up to out_len - 1 bytes and null-terminates.
 * If out is null or out_len == 0, nothing is written.
 * @return Required UTF-8 byte length (excluding null terminator)
 */
WXD_EXPORTED int
wxd_AuiManager_SavePerspective(const wxd_AuiManager_t* self, char* out, size_t out_len);

WXD_EXPORTED bool
wxd_AuiManager_LoadPerspective(wxd_AuiManager_t* self, const char* perspective, bool update);

WXD_EXPORTED bool
wxd_AuiManager_DetachPane(wxd_AuiManager_t* self, wxd_Window_t* window);

// --- wxAuiPaneInfo ---
WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Create();

WXD_EXPORTED void
wxd_AuiPaneInfo_Delete(wxd_AuiPaneInfo_t* self);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Name(wxd_AuiPaneInfo_t* self, const char* name);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Caption(wxd_AuiPaneInfo_t* self, const char* caption);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Left(wxd_AuiPaneInfo_t* self);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Right(wxd_AuiPaneInfo_t* self);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Top(wxd_AuiPaneInfo_t* self);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Bottom(wxd_AuiPaneInfo_t* self);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Center(wxd_AuiPaneInfo_t* self);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Floatable(wxd_AuiPaneInfo_t* self, bool floatable);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Dockable(wxd_AuiPaneInfo_t* self, bool dockable);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Movable(wxd_AuiPaneInfo_t* self, bool movable);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Resizable(wxd_AuiPaneInfo_t* self, bool resizable);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_CloseButton(wxd_AuiPaneInfo_t* self, bool visible);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_MaximizeButton(wxd_AuiPaneInfo_t* self, bool visible);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_MinimizeButton(wxd_AuiPaneInfo_t* self, bool visible);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_PinButton(wxd_AuiPaneInfo_t* self, bool visible);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_PaneBorder(wxd_AuiPaneInfo_t* self, bool visible);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Gripper(wxd_AuiPaneInfo_t* self, bool visible);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_GripperTop(wxd_AuiPaneInfo_t* self, bool attop);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Layer(wxd_AuiPaneInfo_t* self, int layer);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_MinSize(wxd_AuiPaneInfo_t* self, int width, int height);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_MaxSize(wxd_AuiPaneInfo_t* self, int width, int height);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Position(wxd_AuiPaneInfo_t* self, int pos);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Row(wxd_AuiPaneInfo_t* self, int row);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_CenterPane(wxd_AuiPaneInfo_t* self);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_DefaultPane(wxd_AuiPaneInfo_t* self);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_ToolbarPane(wxd_AuiPaneInfo_t* self);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_BestSize(wxd_AuiPaneInfo_t* self, int width, int height);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Show(wxd_AuiPaneInfo_t* self, bool show);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_Hide(wxd_AuiPaneInfo_t* self);

WXD_EXPORTED wxd_AuiPaneInfo_t*
wxd_AuiPaneInfo_CaptionVisible(wxd_AuiPaneInfo_t* self, bool visible);

WXD_EXPORTED wxd_AuiMDIParentFrame_t*
wxd_AuiMDIParentFrame_Create(wxd_Window_t* parent, int id, const char* title, wxd_Point pos,
                             wxd_Size size, int64_t style, const char* name);

WXD_EXPORTED wxd_AuiMDIChildFrame_t*
wxd_AuiMDIChildFrame_Create(wxd_AuiMDIParentFrame_t* parent, int id, const char* title,
                            wxd_Point pos, wxd_Size size, int64_t style, const char* name);

// Potentially other wxAuiMDIChildFrame specific functions will be added here.

// --- wxAuiNotebook ---
WXD_EXPORTED wxd_AuiNotebook_t*
wxd_AuiNotebook_Create(wxd_Window_t* parent, int id, wxd_Point pos, wxd_Size size, int64_t style);

WXD_EXPORTED bool
wxd_AuiNotebook_AddPage(wxd_AuiNotebook_t* self, wxd_Window_t* page, const char* caption,
                        bool select, int bitmap_id);

WXD_EXPORTED size_t
wxd_AuiNotebook_GetPageCount(wxd_AuiNotebook_t* self);

WXD_EXPORTED size_t
wxd_AuiNotebook_SetSelection(wxd_AuiNotebook_t* self, size_t new_page);
// WXD_EXPORTED wxd_Window_t* wxd_AuiNotebook_GetPage(wxd_AuiNotebook_t* self, size_t page_idx); // TODO: decide if GetPage is needed and how to manage its lifecycle

// --- wxAuiToolBar ---
WXD_EXPORTED wxd_AuiToolBar_t*
wxd_AuiToolBar_Create(wxd_Window_t* parent, int id, wxd_Point pos, wxd_Size size, int64_t style);

WXD_EXPORTED void
wxd_AuiToolBar_AddTool(
    wxd_AuiToolBar_t* self, int tool_id, const char* label,
    /*wxd_Bitmap_t* bitmap, wxd_Bitmap_t* disabled_bitmap,*/ const char* short_help_string,
    WXDItemKindCEnum kind);

WXD_EXPORTED void
wxd_AuiToolBar_AddLabel(wxd_AuiToolBar_t* self, int tool_id, const char* label, int width);

WXD_EXPORTED void
wxd_AuiToolBar_AddControl(wxd_AuiToolBar_t* self, wxd_Control_t* control, const char* label);

WXD_EXPORTED void
wxd_AuiToolBar_AddSeparator(wxd_AuiToolBar_t* self);

WXD_EXPORTED void
wxd_AuiToolBar_AddSpacer(wxd_AuiToolBar_t* self, int pixels);

WXD_EXPORTED void
wxd_AuiToolBar_AddStretchSpacer(wxd_AuiToolBar_t* self, int proportion);

WXD_EXPORTED void
wxd_AuiToolBar_Realize(wxd_AuiToolBar_t* self);

WXD_EXPORTED void
wxd_AuiToolBar_SetToolBitmapSize(wxd_AuiToolBar_t* self, wxd_Size size);

WXD_EXPORTED wxd_Size
wxd_AuiToolBar_GetToolBitmapSize(wxd_AuiToolBar_t* self);

WXD_EXPORTED void
wxd_AuiToolBar_SetOverflowVisible(wxd_AuiToolBar_t* self, bool visible);

WXD_EXPORTED bool
wxd_AuiToolBar_GetOverflowVisible(wxd_AuiToolBar_t* self);

WXD_EXPORTED void
wxd_AuiToolBar_SetGripperVisible(wxd_AuiToolBar_t* self, bool visible);

WXD_EXPORTED bool
wxd_AuiToolBar_GetGripperVisible(wxd_AuiToolBar_t* self);

WXD_EXPORTED void
wxd_AuiToolBar_SetToolDropDown(wxd_AuiToolBar_t* self, int tool_id, bool dropdown);

WXD_EXPORTED bool
wxd_AuiToolBar_GetToolDropDown(wxd_AuiToolBar_t* self, int tool_id);

WXD_EXPORTED void
wxd_AuiToolBar_EnableTool(wxd_AuiToolBar_t* self, int tool_id, bool enable);

WXD_EXPORTED bool
wxd_AuiToolBar_GetToolEnabled(wxd_AuiToolBar_t* self, int tool_id);

WXD_EXPORTED int
wxd_AuiToolBar_GetToolCount(wxd_AuiToolBar_t* self);

WXD_EXPORTED void
wxd_AuiToolBar_ClearTools(wxd_AuiToolBar_t* self);

WXD_EXPORTED bool
wxd_AuiToolBar_DeleteTool(wxd_AuiToolBar_t* self, int tool_id);

#ifdef __cplusplus
}
#endif

#endif // WXD_AUI_H