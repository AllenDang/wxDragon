#ifndef WXD_SIZERS_H
#define WXD_SIZERS_H

#include "../wxd_types.h"

// --- BoxSizer (and base Sizer operations) ---
WXD_EXPORTED wxd_Sizer_t*
wxd_BoxSizer_Create(wxd_Orientation_t orient);
WXD_EXPORTED void
wxd_Sizer_AddWindow(wxd_Sizer_t* sizer, wxd_Window_t* window, int proportion, wxd_SizerFlags_t flag,
                    int border);
WXD_EXPORTED void
wxd_Sizer_AddSpacer(wxd_Sizer_t* sizer, int size);
WXD_EXPORTED void
wxd_Sizer_AddSizer(wxd_Sizer_t* sizer, wxd_Sizer_t* childSizer, int proportion,
                   wxd_SizerFlags_t flag, int border);
WXD_EXPORTED void
wxd_Sizer_AddStretchSpacer(wxd_Sizer_t* sizer, int prop);

// --- GridSizer ---
WXD_EXPORTED wxd_GridSizer_t*
wxd_GridSizer_Create(int rows, int cols, int vgap, int hgap);
WXD_EXPORTED wxd_GridSizer_t*
wxd_GridSizer_CreateWithGap(int rows, int cols, int gap_width, int gap_height);
WXD_EXPORTED void
wxd_GridSizer_SetCols(wxd_GridSizer_t* self, int cols);
WXD_EXPORTED void
wxd_GridSizer_SetRows(wxd_GridSizer_t* self, int rows);
WXD_EXPORTED void
wxd_GridSizer_SetVGap(wxd_GridSizer_t* self, int gap);
WXD_EXPORTED void
wxd_GridSizer_SetHGap(wxd_GridSizer_t* self, int gap);
WXD_EXPORTED int
wxd_GridSizer_GetCols(wxd_GridSizer_t* self);
WXD_EXPORTED int
wxd_GridSizer_GetRows(wxd_GridSizer_t* self);
WXD_EXPORTED int
wxd_GridSizer_GetVGap(wxd_GridSizer_t* self);
WXD_EXPORTED int
wxd_GridSizer_GetHGap(wxd_GridSizer_t* self);

// --- FlexGridSizer ---
WXD_EXPORTED wxd_FlexGridSizer_t*
wxd_FlexGridSizer_Create(int rows, int cols, int vgap, int hgap);
WXD_EXPORTED wxd_FlexGridSizer_t*
wxd_FlexGridSizer_CreateWithGap(int rows, int cols, int gap_width, int gap_height);
WXD_EXPORTED void
wxd_FlexGridSizer_AddGrowableCol(wxd_FlexGridSizer_t* self, size_t idx, int proportion);
WXD_EXPORTED void
wxd_FlexGridSizer_AddGrowableRow(wxd_FlexGridSizer_t* self, size_t idx, int proportion);
WXD_EXPORTED void
wxd_FlexGridSizer_SetFlexibleDirection(wxd_FlexGridSizer_t* self, int direction);
WXD_EXPORTED void
wxd_FlexGridSizer_SetNonFlexibleGrowMode(wxd_FlexGridSizer_t* self, int mode);

// --- StaticBoxSizer ---
WXD_EXPORTED wxd_StaticBoxSizer_t*
wxd_StaticBoxSizer_Create_WithBox(wxd_StaticBox_t* box, wxd_Orientation_t orient);
WXD_EXPORTED wxd_StaticBoxSizer_t*
wxd_StaticBoxSizer_Create_WithLabel(wxd_Orientation_t orient, wxd_Window_t* parent,
                                    const char* label);
WXD_EXPORTED wxd_StaticBox_t*
wxd_StaticBoxSizer_GetStaticBox(wxd_StaticBoxSizer_t* self);

// --- WrapSizer ---
WXD_EXPORTED wxd_WrapSizer_t*
wxd_WrapSizer_Create(wxd_Orientation_t orient, int flags);

// --- GridBagSizer ---
WXD_EXPORTED wxd_GridBagSizer_t*
wxd_GridBagSizer_Create(int vgap, int hgap);
WXD_EXPORTED void
wxd_GridBagSizer_AddWindow(wxd_GridBagSizer_t* sizer, wxd_Window_t* window, wxd_GBPosition pos,
                           wxd_GBSpan span, int flag, int border);
WXD_EXPORTED void
wxd_GridBagSizer_AddSizer(wxd_GridBagSizer_t* sizer, wxd_Sizer_t* child_sizer, wxd_GBPosition pos,
                          wxd_GBSpan span, int flag, int border);
WXD_EXPORTED void
wxd_GridBagSizer_AddSpacer(wxd_GridBagSizer_t* sizer, int width, int height, wxd_GBPosition pos,
                           wxd_GBSpan span, int flag, int border);
WXD_EXPORTED wxd_GBPosition
wxd_GridBagSizer_GetItemPosition_Window(wxd_GridBagSizer_t* sizer, wxd_Window_t* window);
WXD_EXPORTED wxd_GBPosition
wxd_GridBagSizer_GetItemPosition_Sizer(wxd_GridBagSizer_t* sizer, wxd_Sizer_t* child_sizer);
WXD_EXPORTED wxd_GBPosition
wxd_GridBagSizer_GetItemPosition_Index(wxd_GridBagSizer_t* sizer, size_t index);
WXD_EXPORTED bool
wxd_GridBagSizer_SetItemPosition_Window(wxd_GridBagSizer_t* sizer, wxd_Window_t* window,
                                        wxd_GBPosition pos);
WXD_EXPORTED bool
wxd_GridBagSizer_SetItemPosition_Sizer(wxd_GridBagSizer_t* sizer, wxd_Sizer_t* child_sizer,
                                       wxd_GBPosition pos);
WXD_EXPORTED bool
wxd_GridBagSizer_SetItemPosition_Index(wxd_GridBagSizer_t* sizer, size_t index, wxd_GBPosition pos);
WXD_EXPORTED wxd_GBSpan
wxd_GridBagSizer_GetItemSpan_Window(wxd_GridBagSizer_t* sizer, wxd_Window_t* window);
WXD_EXPORTED wxd_GBSpan
wxd_GridBagSizer_GetItemSpan_Sizer(wxd_GridBagSizer_t* sizer, wxd_Sizer_t* child_sizer);
WXD_EXPORTED wxd_GBSpan
wxd_GridBagSizer_GetItemSpan_Index(wxd_GridBagSizer_t* sizer, size_t index);
WXD_EXPORTED bool
wxd_GridBagSizer_SetItemSpan_Window(wxd_GridBagSizer_t* sizer, wxd_Window_t* window,
                                    wxd_GBSpan span);
WXD_EXPORTED bool
wxd_GridBagSizer_SetItemSpan_Sizer(wxd_GridBagSizer_t* sizer, wxd_Sizer_t* child_sizer,
                                   wxd_GBSpan span);
WXD_EXPORTED bool
wxd_GridBagSizer_SetItemSpan_Index(wxd_GridBagSizer_t* sizer, size_t index, wxd_GBSpan span);
WXD_EXPORTED wxd_Size
wxd_GridBagSizer_GetEmptyCellSize(wxd_GridBagSizer_t* sizer);
WXD_EXPORTED void
wxd_GridBagSizer_SetEmptyCellSize(wxd_GridBagSizer_t* sizer, wxd_Size size);
WXD_EXPORTED wxd_Size
wxd_GridBagSizer_GetCellSize(wxd_GridBagSizer_t* sizer, int row, int col);

#endif // WXD_SIZERS_H