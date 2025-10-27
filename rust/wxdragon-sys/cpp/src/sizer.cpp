#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/sizer.h>
#include <wx/statbox.h>
#include <wx/wrapsizer.h>
#include <wx/gbsizer.h>

extern "C" {

// --- Sizer Functions ---

WXD_EXPORTED wxd_Sizer_t*
wxd_BoxSizer_Create(wxd_Orientation_t orient)
{
    // Cast integer orientation to wxOrientation
    wxOrientation wx_orient = static_cast<wxOrientation>(orient);
    wxBoxSizer* sizer = new wxBoxSizer(wx_orient);
    return reinterpret_cast<wxd_Sizer_t*>(sizer);
}

WXD_EXPORTED void
wxd_Sizer_AddWindow(wxd_Sizer_t* sizer, wxd_Window_t* window, int proportion, wxd_SizerFlags_t flag,
                    int border)
{
    if (!sizer || !window)
        return;
    wxSizer* wx_sizer = reinterpret_cast<wxSizer*>(sizer);
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);

    // Ensure flag is treated as an int for wxWidgets flags (safe due to typedef)
    wx_sizer->Add(wx_window, proportion, flag, border);
}

WXD_EXPORTED void
wxd_Sizer_AddSpacer(wxd_Sizer_t* sizer, int size)
{
    if (!sizer)
        return;
    wxSizer* wx_sizer = reinterpret_cast<wxSizer*>(sizer);
    wx_sizer->AddSpacer(size);
}

WXD_EXPORTED void
wxd_Sizer_AddSizer(wxd_Sizer_t* sizer, wxd_Sizer_t* childSizer, int proportion,
                   wxd_SizerFlags_t flag, int border)
{
    if (!sizer || !childSizer)
        return;
    wxSizer* wx_sizer = reinterpret_cast<wxSizer*>(sizer);
    wxSizer* wx_child_sizer = reinterpret_cast<wxSizer*>(childSizer);

    // The parent sizer takes ownership of the child sizer pointer
    // when added. The Rust side should `std::mem::forget` the wrapper
    // for the child sizer to prevent a double free.
    wx_sizer->Add(wx_child_sizer, proportion, flag, border);
}

WXD_EXPORTED void
wxd_Sizer_AddStretchSpacer(wxd_Sizer_t* sizer, int prop)
{
    if (!sizer)
        return;
    wxSizer* wx_sizer = reinterpret_cast<wxSizer*>(sizer);
    wx_sizer->AddStretchSpacer(prop);
}

// --- ADDED: wxStaticBoxSizer Implementation ---

WXD_EXPORTED wxd_StaticBoxSizer_t*
wxd_StaticBoxSizer_Create_WithBox(wxd_StaticBox_t* box, wxd_Orientation_t orient)
{
    if (!box)
        return nullptr;
    wxStaticBox* wx_box = reinterpret_cast<wxStaticBox*>(box);
    wxOrientation wx_orient = static_cast<wxOrientation>(orient);
    wxStaticBoxSizer* sizer = new wxStaticBoxSizer(wx_box, wx_orient);
    return reinterpret_cast<wxd_StaticBoxSizer_t*>(sizer);
}

WXD_EXPORTED wxd_StaticBoxSizer_t*
wxd_StaticBoxSizer_Create_WithLabel(wxd_Orientation_t orient, wxd_Window_t* parent,
                                    const char* label)
{
    if (!parent)
        return nullptr;
    wxWindow* wx_parent = reinterpret_cast<wxWindow*>(parent);
    wxOrientation wx_orient = static_cast<wxOrientation>(orient);
    wxStaticBoxSizer* sizer =
        new wxStaticBoxSizer(wx_orient, wx_parent, wxString::FromUTF8(label ? label : ""));
    return reinterpret_cast<wxd_StaticBoxSizer_t*>(sizer);
}

wxd_StaticBox_t*
wxd_StaticBoxSizer_GetStaticBox(wxd_StaticBoxSizer_t* self)
{
    if (!self)
        return nullptr;
    wxStaticBoxSizer* wx_sizer = reinterpret_cast<wxStaticBoxSizer*>(self);
    return reinterpret_cast<wxd_StaticBox_t*>(wx_sizer->GetStaticBox());
}

// --- wxGridSizer ---
wxd_GridSizer_t*
wxd_GridSizer_Create(int rows, int cols, int vgap, int hgap)
{
    wxGridSizer* sizer = new wxGridSizer(rows, cols, vgap, hgap);
    return reinterpret_cast<wxd_GridSizer_t*>(sizer);
}

wxd_GridSizer_t*
wxd_GridSizer_CreateWithGap(int rows, int cols, int gap_width, int gap_height)
{
    wxGridSizer* sizer = new wxGridSizer(rows, cols, wxSize(gap_width, gap_height));
    return reinterpret_cast<wxd_GridSizer_t*>(sizer);
}

void
wxd_GridSizer_SetCols(wxd_GridSizer_t* self, int cols)
{
    if (self) {
        wxGridSizer* wx_sizer = reinterpret_cast<wxGridSizer*>(self);
        wx_sizer->SetCols(cols);
    }
}

void
wxd_GridSizer_SetRows(wxd_GridSizer_t* self, int rows)
{
    if (self) {
        wxGridSizer* wx_sizer = reinterpret_cast<wxGridSizer*>(self);
        wx_sizer->SetRows(rows);
    }
}

void
wxd_GridSizer_SetVGap(wxd_GridSizer_t* self, int gap)
{
    if (self) {
        wxGridSizer* wx_sizer = reinterpret_cast<wxGridSizer*>(self);
        wx_sizer->SetVGap(gap);
    }
}

void
wxd_GridSizer_SetHGap(wxd_GridSizer_t* self, int gap)
{
    if (self) {
        wxGridSizer* wx_sizer = reinterpret_cast<wxGridSizer*>(self);
        wx_sizer->SetHGap(gap);
    }
}

int
wxd_GridSizer_GetCols(wxd_GridSizer_t* self)
{
    if (self) {
        wxGridSizer* wx_sizer = reinterpret_cast<wxGridSizer*>(self);
        return wx_sizer->GetCols();
    }
    return 0;
}

int
wxd_GridSizer_GetRows(wxd_GridSizer_t* self)
{
    if (self) {
        wxGridSizer* wx_sizer = reinterpret_cast<wxGridSizer*>(self);
        return wx_sizer->GetRows();
    }
    return 0;
}

int
wxd_GridSizer_GetVGap(wxd_GridSizer_t* self)
{
    if (self) {
        wxGridSizer* wx_sizer = reinterpret_cast<wxGridSizer*>(self);
        return wx_sizer->GetVGap();
    }
    return 0;
}

int
wxd_GridSizer_GetHGap(wxd_GridSizer_t* self)
{
    if (self) {
        wxGridSizer* wx_sizer = reinterpret_cast<wxGridSizer*>(self);
        return wx_sizer->GetHGap();
    }
    return 0;
}

// --- wxFlexGridSizer ---
wxd_FlexGridSizer_t*
wxd_FlexGridSizer_Create(int rows, int cols, int vgap, int hgap)
{
    wxFlexGridSizer* sizer = new wxFlexGridSizer(rows, cols, vgap, hgap);
    return reinterpret_cast<wxd_FlexGridSizer_t*>(sizer);
}

wxd_FlexGridSizer_t*
wxd_FlexGridSizer_CreateWithGap(int rows, int cols, int gap_width, int gap_height)
{
    wxFlexGridSizer* sizer = new wxFlexGridSizer(rows, cols, wxSize(gap_width, gap_height));
    return reinterpret_cast<wxd_FlexGridSizer_t*>(sizer);
}

void
wxd_FlexGridSizer_AddGrowableCol(wxd_FlexGridSizer_t* self, size_t idx, int proportion)
{
    if (self) {
        wxFlexGridSizer* wx_sizer = reinterpret_cast<wxFlexGridSizer*>(self);
        wx_sizer->AddGrowableCol(idx, proportion);
    }
}

void
wxd_FlexGridSizer_AddGrowableRow(wxd_FlexGridSizer_t* self, size_t idx, int proportion)
{
    if (self) {
        wxFlexGridSizer* wx_sizer = reinterpret_cast<wxFlexGridSizer*>(self);
        wx_sizer->AddGrowableRow(idx, proportion);
    }
}

void
wxd_FlexGridSizer_SetFlexibleDirection(wxd_FlexGridSizer_t* self, int direction)
{
    if (self) {
        wxFlexGridSizer* wx_sizer = reinterpret_cast<wxFlexGridSizer*>(self);
        wx_sizer->SetFlexibleDirection(direction);
    }
}

void
wxd_FlexGridSizer_SetNonFlexibleGrowMode(wxd_FlexGridSizer_t* self, int mode)
{
    if (self) {
        wxFlexGridSizer* wx_sizer = reinterpret_cast<wxFlexGridSizer*>(self);
        wx_sizer->SetNonFlexibleGrowMode(static_cast<wxFlexSizerGrowMode>(mode));
    }
}

// --- WrapSizer ---
WXD_EXPORTED wxd_WrapSizer_t*
wxd_WrapSizer_Create(wxd_Orientation_t orient, int flags)
{
    wxOrientation wx_orient = static_cast<wxOrientation>(orient);
    wxWrapSizer* sizer = new wxWrapSizer(wx_orient, flags);
    return reinterpret_cast<wxd_WrapSizer_t*>(sizer);
}

// --- GridBagSizer ---
WXD_EXPORTED wxd_GridBagSizer_t*
wxd_GridBagSizer_Create(int vgap, int hgap)
{
    wxGridBagSizer* sizer = new wxGridBagSizer(vgap, hgap);
    return reinterpret_cast<wxd_GridBagSizer_t*>(sizer);
}

WXD_EXPORTED void
wxd_GridBagSizer_AddWindow(wxd_GridBagSizer_t* sizer, wxd_Window_t* window, wxd_GBPosition pos,
                           wxd_GBSpan span, int flag, int border)
{
    if (!sizer || !window)
        return;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wxGBPosition wx_pos(pos.row, pos.col);
    wxGBSpan wx_span(span.rowspan, span.colspan);
    wx_sizer->Add(wx_window, wx_pos, wx_span, flag, border);
}

WXD_EXPORTED void
wxd_GridBagSizer_AddSizer(wxd_GridBagSizer_t* sizer, wxd_Sizer_t* child_sizer, wxd_GBPosition pos,
                          wxd_GBSpan span, int flag, int border)
{
    if (!sizer || !child_sizer)
        return;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxSizer* wx_child_sizer = reinterpret_cast<wxSizer*>(child_sizer);
    wxGBPosition wx_pos(pos.row, pos.col);
    wxGBSpan wx_span(span.rowspan, span.colspan);
    wx_sizer->Add(wx_child_sizer, wx_pos, wx_span, flag, border);
}

WXD_EXPORTED void
wxd_GridBagSizer_AddSpacer(wxd_GridBagSizer_t* sizer, int width, int height, wxd_GBPosition pos,
                           wxd_GBSpan span, int flag, int border)
{
    if (!sizer)
        return;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxGBPosition wx_pos(pos.row, pos.col);
    wxGBSpan wx_span(span.rowspan, span.colspan);
    wx_sizer->Add(width, height, wx_pos, wx_span, flag, border);
}

WXD_EXPORTED wxd_GBPosition
wxd_GridBagSizer_GetItemPosition_Window(wxd_GridBagSizer_t* sizer, wxd_Window_t* window)
{
    wxd_GBPosition result = { 0, 0 };
    if (!sizer || !window)
        return result;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wxGBPosition wx_pos = wx_sizer->GetItemPosition(wx_window);
    result.row = wx_pos.GetRow();
    result.col = wx_pos.GetCol();
    return result;
}

WXD_EXPORTED wxd_GBPosition
wxd_GridBagSizer_GetItemPosition_Sizer(wxd_GridBagSizer_t* sizer, wxd_Sizer_t* child_sizer)
{
    wxd_GBPosition result = { 0, 0 };
    if (!sizer || !child_sizer)
        return result;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxSizer* wx_child_sizer = reinterpret_cast<wxSizer*>(child_sizer);
    wxGBPosition wx_pos = wx_sizer->GetItemPosition(wx_child_sizer);
    result.row = wx_pos.GetRow();
    result.col = wx_pos.GetCol();
    return result;
}

WXD_EXPORTED wxd_GBPosition
wxd_GridBagSizer_GetItemPosition_Index(wxd_GridBagSizer_t* sizer, size_t index)
{
    wxd_GBPosition result = { 0, 0 };
    if (!sizer)
        return result;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxGBPosition wx_pos = wx_sizer->GetItemPosition(index);
    result.row = wx_pos.GetRow();
    result.col = wx_pos.GetCol();
    return result;
}

WXD_EXPORTED bool
wxd_GridBagSizer_SetItemPosition_Window(wxd_GridBagSizer_t* sizer, wxd_Window_t* window,
                                        wxd_GBPosition pos)
{
    if (!sizer || !window)
        return false;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wxGBPosition wx_pos(pos.row, pos.col);
    return wx_sizer->SetItemPosition(wx_window, wx_pos);
}

WXD_EXPORTED bool
wxd_GridBagSizer_SetItemPosition_Sizer(wxd_GridBagSizer_t* sizer, wxd_Sizer_t* child_sizer,
                                       wxd_GBPosition pos)
{
    if (!sizer || !child_sizer)
        return false;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxSizer* wx_child_sizer = reinterpret_cast<wxSizer*>(child_sizer);
    wxGBPosition wx_pos(pos.row, pos.col);
    return wx_sizer->SetItemPosition(wx_child_sizer, wx_pos);
}

WXD_EXPORTED bool
wxd_GridBagSizer_SetItemPosition_Index(wxd_GridBagSizer_t* sizer, size_t index, wxd_GBPosition pos)
{
    if (!sizer)
        return false;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxGBPosition wx_pos(pos.row, pos.col);
    return wx_sizer->SetItemPosition(index, wx_pos);
}

WXD_EXPORTED wxd_GBSpan
wxd_GridBagSizer_GetItemSpan_Window(wxd_GridBagSizer_t* sizer, wxd_Window_t* window)
{
    wxd_GBSpan result = { 1, 1 };
    if (!sizer || !window)
        return result;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wxGBSpan wx_span = wx_sizer->GetItemSpan(wx_window);
    result.rowspan = wx_span.GetRowspan();
    result.colspan = wx_span.GetColspan();
    return result;
}

WXD_EXPORTED wxd_GBSpan
wxd_GridBagSizer_GetItemSpan_Sizer(wxd_GridBagSizer_t* sizer, wxd_Sizer_t* child_sizer)
{
    wxd_GBSpan result = { 1, 1 };
    if (!sizer || !child_sizer)
        return result;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxSizer* wx_child_sizer = reinterpret_cast<wxSizer*>(child_sizer);
    wxGBSpan wx_span = wx_sizer->GetItemSpan(wx_child_sizer);
    result.rowspan = wx_span.GetRowspan();
    result.colspan = wx_span.GetColspan();
    return result;
}

WXD_EXPORTED wxd_GBSpan
wxd_GridBagSizer_GetItemSpan_Index(wxd_GridBagSizer_t* sizer, size_t index)
{
    wxd_GBSpan result = { 1, 1 };
    if (!sizer)
        return result;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxGBSpan wx_span = wx_sizer->GetItemSpan(index);
    result.rowspan = wx_span.GetRowspan();
    result.colspan = wx_span.GetColspan();
    return result;
}

WXD_EXPORTED bool
wxd_GridBagSizer_SetItemSpan_Window(wxd_GridBagSizer_t* sizer, wxd_Window_t* window,
                                    wxd_GBSpan span)
{
    if (!sizer || !window)
        return false;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wxGBSpan wx_span(span.rowspan, span.colspan);
    return wx_sizer->SetItemSpan(wx_window, wx_span);
}

WXD_EXPORTED bool
wxd_GridBagSizer_SetItemSpan_Sizer(wxd_GridBagSizer_t* sizer, wxd_Sizer_t* child_sizer,
                                   wxd_GBSpan span)
{
    if (!sizer || !child_sizer)
        return false;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxSizer* wx_child_sizer = reinterpret_cast<wxSizer*>(child_sizer);
    wxGBSpan wx_span(span.rowspan, span.colspan);
    return wx_sizer->SetItemSpan(wx_child_sizer, wx_span);
}

WXD_EXPORTED bool
wxd_GridBagSizer_SetItemSpan_Index(wxd_GridBagSizer_t* sizer, size_t index, wxd_GBSpan span)
{
    if (!sizer)
        return false;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxGBSpan wx_span(span.rowspan, span.colspan);
    return wx_sizer->SetItemSpan(index, wx_span);
}

WXD_EXPORTED wxd_Size
wxd_GridBagSizer_GetEmptyCellSize(wxd_GridBagSizer_t* sizer)
{
    wxd_Size result = { 0, 0 };
    if (!sizer)
        return result;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxSize wx_size = wx_sizer->GetEmptyCellSize();
    result.width = wx_size.GetWidth();
    result.height = wx_size.GetHeight();
    return result;
}

WXD_EXPORTED void
wxd_GridBagSizer_SetEmptyCellSize(wxd_GridBagSizer_t* sizer, wxd_Size size)
{
    if (!sizer)
        return;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxSize wx_size(size.width, size.height);
    wx_sizer->SetEmptyCellSize(wx_size);
}

WXD_EXPORTED wxd_Size
wxd_GridBagSizer_GetCellSize(wxd_GridBagSizer_t* sizer, int row, int col)
{
    wxd_Size result = { 0, 0 };
    if (!sizer)
        return result;
    wxGridBagSizer* wx_sizer = reinterpret_cast<wxGridBagSizer*>(sizer);
    wxSize wx_size = wx_sizer->GetCellSize(row, col);
    result.width = wx_size.GetWidth();
    result.height = wx_size.GetHeight();
    return result;
}

// --- Treebook ---

} // extern "C"