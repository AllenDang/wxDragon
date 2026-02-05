#include <wx/wxprec.h>
#include <wx/wx.h>
#include <wx/grid.h>
#include "../include/wxdragon.h"
#include "../src/wxd_utils.h"

extern "C" {

// --- Grid Creation ---

WXD_EXPORTED wxd_Grid_t*
wxd_Grid_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                wxd_Style_t style)
{
    if (!parent) return nullptr;
    wxWindow* p = reinterpret_cast<wxWindow*>(parent);
    wxGrid* grid = new wxGrid(p, id, wxPoint(pos.x, pos.y),
                              wxSize(size.width, size.height), style);
    return reinterpret_cast<wxd_Grid_t*>(grid);
}

WXD_EXPORTED bool
wxd_Grid_CreateGrid(wxd_Grid_t* self, int numRows, int numCols, int selectionMode)
{
    if (!self) return false;
    wxGrid* grid = reinterpret_cast<wxGrid*>(self);
    return grid->CreateGrid(numRows, numCols,
                           static_cast<wxGrid::wxGridSelectionModes>(selectionMode));
}

// --- Grid Dimensions ---

WXD_EXPORTED int
wxd_Grid_GetNumberRows(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetNumberRows();
}

WXD_EXPORTED int
wxd_Grid_GetNumberCols(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetNumberCols();
}

// --- Row and Column Management ---

WXD_EXPORTED bool
wxd_Grid_InsertRows(wxd_Grid_t* self, int pos, int numRows, bool updateLabels)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->InsertRows(pos, numRows, updateLabels);
}

WXD_EXPORTED bool
wxd_Grid_AppendRows(wxd_Grid_t* self, int numRows, bool updateLabels)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->AppendRows(numRows, updateLabels);
}

WXD_EXPORTED bool
wxd_Grid_DeleteRows(wxd_Grid_t* self, int pos, int numRows, bool updateLabels)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->DeleteRows(pos, numRows, updateLabels);
}

WXD_EXPORTED bool
wxd_Grid_InsertCols(wxd_Grid_t* self, int pos, int numCols, bool updateLabels)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->InsertCols(pos, numCols, updateLabels);
}

WXD_EXPORTED bool
wxd_Grid_AppendCols(wxd_Grid_t* self, int numCols, bool updateLabels)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->AppendCols(numCols, updateLabels);
}

WXD_EXPORTED bool
wxd_Grid_DeleteCols(wxd_Grid_t* self, int pos, int numCols, bool updateLabels)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->DeleteCols(pos, numCols, updateLabels);
}

// --- Cell Value Accessors ---

WXD_EXPORTED int
wxd_Grid_GetCellValue(wxd_Grid_t* self, int row, int col, char* buffer, int buffer_len)
{
    if (!self) return 0;
    wxString value = reinterpret_cast<wxGrid*>(self)->GetCellValue(row, col);
    return static_cast<int>(wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, buffer_len));
}

WXD_EXPORTED void
wxd_Grid_SetCellValue(wxd_Grid_t* self, int row, int col, const char* value)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetCellValue(row, col,
                                                   wxString::FromUTF8(value ? value : ""));
}

// --- Label Functions ---

WXD_EXPORTED int
wxd_Grid_GetRowLabelValue(wxd_Grid_t* self, int row, char* buffer, int buffer_len)
{
    if (!self) return 0;
    wxString value = reinterpret_cast<wxGrid*>(self)->GetRowLabelValue(row);
    return static_cast<int>(wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, buffer_len));
}

WXD_EXPORTED void
wxd_Grid_SetRowLabelValue(wxd_Grid_t* self, int row, const char* value)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetRowLabelValue(row,
                                                       wxString::FromUTF8(value ? value : ""));
}

WXD_EXPORTED int
wxd_Grid_GetColLabelValue(wxd_Grid_t* self, int col, char* buffer, int buffer_len)
{
    if (!self) return 0;
    wxString value = reinterpret_cast<wxGrid*>(self)->GetColLabelValue(col);
    return static_cast<int>(wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, buffer_len));
}

WXD_EXPORTED void
wxd_Grid_SetColLabelValue(wxd_Grid_t* self, int col, const char* value)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetColLabelValue(col,
                                                       wxString::FromUTF8(value ? value : ""));
}

WXD_EXPORTED int
wxd_Grid_GetRowLabelSize(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetRowLabelSize();
}

WXD_EXPORTED void
wxd_Grid_SetRowLabelSize(wxd_Grid_t* self, int width)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetRowLabelSize(width);
}

WXD_EXPORTED int
wxd_Grid_GetColLabelSize(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetColLabelSize();
}

WXD_EXPORTED void
wxd_Grid_SetColLabelSize(wxd_Grid_t* self, int height)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetColLabelSize(height);
}

WXD_EXPORTED void
wxd_Grid_HideRowLabels(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->HideRowLabels();
}

WXD_EXPORTED void
wxd_Grid_HideColLabels(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->HideColLabels();
}

// --- Row and Column Sizes ---

WXD_EXPORTED int
wxd_Grid_GetDefaultRowSize(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetDefaultRowSize();
}

WXD_EXPORTED int
wxd_Grid_GetRowSize(wxd_Grid_t* self, int row)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetRowSize(row);
}

WXD_EXPORTED void
wxd_Grid_SetDefaultRowSize(wxd_Grid_t* self, int height, bool resizeExistingRows)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetDefaultRowSize(height, resizeExistingRows);
}

WXD_EXPORTED void
wxd_Grid_SetRowSize(wxd_Grid_t* self, int row, int height)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetRowSize(row, height);
}

WXD_EXPORTED int
wxd_Grid_GetDefaultColSize(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetDefaultColSize();
}

WXD_EXPORTED int
wxd_Grid_GetColSize(wxd_Grid_t* self, int col)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetColSize(col);
}

WXD_EXPORTED void
wxd_Grid_SetDefaultColSize(wxd_Grid_t* self, int width, bool resizeExistingCols)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetDefaultColSize(width, resizeExistingCols);
}

WXD_EXPORTED void
wxd_Grid_SetColSize(wxd_Grid_t* self, int col, int width)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetColSize(col, width);
}

WXD_EXPORTED void
wxd_Grid_AutoSizeColumn(wxd_Grid_t* self, int col, bool setAsMin)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->AutoSizeColumn(col, setAsMin);
}

WXD_EXPORTED void
wxd_Grid_AutoSizeRow(wxd_Grid_t* self, int row, bool setAsMin)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->AutoSizeRow(row, setAsMin);
}

WXD_EXPORTED void
wxd_Grid_AutoSizeColumns(wxd_Grid_t* self, bool setAsMin)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->AutoSizeColumns(setAsMin);
}

WXD_EXPORTED void
wxd_Grid_AutoSizeRows(wxd_Grid_t* self, bool setAsMin)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->AutoSizeRows(setAsMin);
}

WXD_EXPORTED void
wxd_Grid_AutoSize(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->AutoSize();
}

// --- Cell Formatting ---

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetCellBackgroundColour(wxd_Grid_t* self, int row, int col)
{
    wxd_Colour_t colour = {255, 255, 255, 255};
    if (!self) return colour;
    wxColour wxc = reinterpret_cast<wxGrid*>(self)->GetCellBackgroundColour(row, col);
    colour.r = wxc.Red();
    colour.g = wxc.Green();
    colour.b = wxc.Blue();
    colour.a = wxc.Alpha();
    return colour;
}

WXD_EXPORTED void
wxd_Grid_SetCellBackgroundColour(wxd_Grid_t* self, int row, int col, wxd_Colour_t colour)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetCellBackgroundColour(row, col,
        wxColour(colour.r, colour.g, colour.b, colour.a));
}

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetCellTextColour(wxd_Grid_t* self, int row, int col)
{
    wxd_Colour_t colour = {0, 0, 0, 255};
    if (!self) return colour;
    wxColour wxc = reinterpret_cast<wxGrid*>(self)->GetCellTextColour(row, col);
    colour.r = wxc.Red();
    colour.g = wxc.Green();
    colour.b = wxc.Blue();
    colour.a = wxc.Alpha();
    return colour;
}

WXD_EXPORTED void
wxd_Grid_SetCellTextColour(wxd_Grid_t* self, int row, int col, wxd_Colour_t colour)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetCellTextColour(row, col,
        wxColour(colour.r, colour.g, colour.b, colour.a));
}

WXD_EXPORTED void
wxd_Grid_GetCellAlignment(wxd_Grid_t* self, int row, int col, int* horiz, int* vert)
{
    if (!self || !horiz || !vert) return;
    reinterpret_cast<wxGrid*>(self)->GetCellAlignment(row, col, horiz, vert);
}

WXD_EXPORTED void
wxd_Grid_SetCellAlignment(wxd_Grid_t* self, int row, int col, int horiz, int vert)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetCellAlignment(row, col, horiz, vert);
}

// --- Default Cell Formatting ---

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetDefaultCellBackgroundColour(wxd_Grid_t* self)
{
    wxd_Colour_t colour = {255, 255, 255, 255};
    if (!self) return colour;
    wxColour wxc = reinterpret_cast<wxGrid*>(self)->GetDefaultCellBackgroundColour();
    colour.r = wxc.Red();
    colour.g = wxc.Green();
    colour.b = wxc.Blue();
    colour.a = wxc.Alpha();
    return colour;
}

WXD_EXPORTED void
wxd_Grid_SetDefaultCellBackgroundColour(wxd_Grid_t* self, wxd_Colour_t colour)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetDefaultCellBackgroundColour(
        wxColour(colour.r, colour.g, colour.b, colour.a));
}

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetDefaultCellTextColour(wxd_Grid_t* self)
{
    wxd_Colour_t colour = {0, 0, 0, 255};
    if (!self) return colour;
    wxColour wxc = reinterpret_cast<wxGrid*>(self)->GetDefaultCellTextColour();
    colour.r = wxc.Red();
    colour.g = wxc.Green();
    colour.b = wxc.Blue();
    colour.a = wxc.Alpha();
    return colour;
}

WXD_EXPORTED void
wxd_Grid_SetDefaultCellTextColour(wxd_Grid_t* self, wxd_Colour_t colour)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetDefaultCellTextColour(
        wxColour(colour.r, colour.g, colour.b, colour.a));
}

WXD_EXPORTED void
wxd_Grid_GetDefaultCellAlignment(wxd_Grid_t* self, int* horiz, int* vert)
{
    if (!self || !horiz || !vert) return;
    reinterpret_cast<wxGrid*>(self)->GetDefaultCellAlignment(horiz, vert);
}

WXD_EXPORTED void
wxd_Grid_SetDefaultCellAlignment(wxd_Grid_t* self, int horiz, int vert)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetDefaultCellAlignment(horiz, vert);
}

// --- Read-Only Cells ---

WXD_EXPORTED bool
wxd_Grid_IsReadOnly(wxd_Grid_t* self, int row, int col)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->IsReadOnly(row, col);
}

WXD_EXPORTED void
wxd_Grid_SetReadOnly(wxd_Grid_t* self, int row, int col, bool isReadOnly)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetReadOnly(row, col, isReadOnly);
}

// --- Selection ---

WXD_EXPORTED void
wxd_Grid_SelectRow(wxd_Grid_t* self, int row, bool addToSelected)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SelectRow(row, addToSelected);
}

WXD_EXPORTED void
wxd_Grid_SelectCol(wxd_Grid_t* self, int col, bool addToSelected)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SelectCol(col, addToSelected);
}

WXD_EXPORTED void
wxd_Grid_SelectBlock(wxd_Grid_t* self, int topRow, int leftCol, int bottomRow, int rightCol,
                     bool addToSelected)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SelectBlock(topRow, leftCol, bottomRow, rightCol, addToSelected);
}

WXD_EXPORTED void
wxd_Grid_SelectAll(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SelectAll();
}

WXD_EXPORTED bool
wxd_Grid_IsSelection(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->IsSelection();
}

WXD_EXPORTED void
wxd_Grid_DeselectRow(wxd_Grid_t* self, int row)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->DeselectRow(row);
}

WXD_EXPORTED void
wxd_Grid_DeselectCol(wxd_Grid_t* self, int col)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->DeselectCol(col);
}

WXD_EXPORTED void
wxd_Grid_DeselectCell(wxd_Grid_t* self, int row, int col)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->DeselectCell(row, col);
}

WXD_EXPORTED void
wxd_Grid_ClearSelection(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->ClearSelection();
}

WXD_EXPORTED bool
wxd_Grid_IsInSelection(wxd_Grid_t* self, int row, int col)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->IsInSelection(row, col);
}

WXD_EXPORTED int
wxd_Grid_GetSelectedRows(wxd_Grid_t* self, int* buffer, int buffer_len)
{
    if (!self) return 0;
    wxArrayInt rows = reinterpret_cast<wxGrid*>(self)->GetSelectedRows();
    int count = static_cast<int>(rows.GetCount());
    if (buffer && buffer_len > 0) {
        int copy_count = (count < buffer_len) ? count : buffer_len;
        for (int i = 0; i < copy_count; i++) {
            buffer[i] = rows[i];
        }
    }
    return count;
}

WXD_EXPORTED int
wxd_Grid_GetSelectedCols(wxd_Grid_t* self, int* buffer, int buffer_len)
{
    if (!self) return 0;
    wxArrayInt cols = reinterpret_cast<wxGrid*>(self)->GetSelectedCols();
    int count = static_cast<int>(cols.GetCount());
    if (buffer && buffer_len > 0) {
        int copy_count = (count < buffer_len) ? count : buffer_len;
        for (int i = 0; i < copy_count; i++) {
            buffer[i] = cols[i];
        }
    }
    return count;
}

// --- Grid Cursor ---

WXD_EXPORTED int
wxd_Grid_GetGridCursorRow(wxd_Grid_t* self)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->GetGridCursorRow();
}

WXD_EXPORTED int
wxd_Grid_GetGridCursorCol(wxd_Grid_t* self)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->GetGridCursorCol();
}

WXD_EXPORTED void
wxd_Grid_SetGridCursor(wxd_Grid_t* self, int row, int col)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetGridCursor(row, col);
}

WXD_EXPORTED void
wxd_Grid_GoToCell(wxd_Grid_t* self, int row, int col)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->GoToCell(row, col);
}

// --- Cell Visibility ---

WXD_EXPORTED bool
wxd_Grid_IsVisible(wxd_Grid_t* self, int row, int col, bool wholeCellVisible)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->IsVisible(row, col, wholeCellVisible);
}

WXD_EXPORTED void
wxd_Grid_MakeCellVisible(wxd_Grid_t* self, int row, int col)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->MakeCellVisible(row, col);
}

// --- Editing ---

WXD_EXPORTED bool
wxd_Grid_IsEditable(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->IsEditable();
}

WXD_EXPORTED void
wxd_Grid_EnableEditing(wxd_Grid_t* self, bool edit)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->EnableEditing(edit);
}

WXD_EXPORTED void
wxd_Grid_EnableCellEditControl(wxd_Grid_t* self, bool enable)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->EnableCellEditControl(enable);
}

WXD_EXPORTED void
wxd_Grid_DisableCellEditControl(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->DisableCellEditControl();
}

WXD_EXPORTED bool
wxd_Grid_IsCellEditControlEnabled(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->IsCellEditControlEnabled();
}

// --- Grid Lines ---

WXD_EXPORTED void
wxd_Grid_EnableGridLines(wxd_Grid_t* self, bool enable)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->EnableGridLines(enable);
}

WXD_EXPORTED bool
wxd_Grid_GridLinesEnabled(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->GridLinesEnabled();
}

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetGridLineColour(wxd_Grid_t* self)
{
    wxd_Colour_t colour = {0, 0, 0, 255};
    if (!self) return colour;
    wxColour wxc = reinterpret_cast<wxGrid*>(self)->GetGridLineColour();
    colour.r = wxc.Red();
    colour.g = wxc.Green();
    colour.b = wxc.Blue();
    colour.a = wxc.Alpha();
    return colour;
}

WXD_EXPORTED void
wxd_Grid_SetGridLineColour(wxd_Grid_t* self, wxd_Colour_t colour)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetGridLineColour(
        wxColour(colour.r, colour.g, colour.b, colour.a));
}

// --- Label Appearance ---

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetLabelBackgroundColour(wxd_Grid_t* self)
{
    wxd_Colour_t colour = {192, 192, 192, 255};
    if (!self) return colour;
    wxColour wxc = reinterpret_cast<wxGrid*>(self)->GetLabelBackgroundColour();
    colour.r = wxc.Red();
    colour.g = wxc.Green();
    colour.b = wxc.Blue();
    colour.a = wxc.Alpha();
    return colour;
}

WXD_EXPORTED void
wxd_Grid_SetLabelBackgroundColour(wxd_Grid_t* self, wxd_Colour_t colour)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetLabelBackgroundColour(
        wxColour(colour.r, colour.g, colour.b, colour.a));
}

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetLabelTextColour(wxd_Grid_t* self)
{
    wxd_Colour_t colour = {0, 0, 0, 255};
    if (!self) return colour;
    wxColour wxc = reinterpret_cast<wxGrid*>(self)->GetLabelTextColour();
    colour.r = wxc.Red();
    colour.g = wxc.Green();
    colour.b = wxc.Blue();
    colour.a = wxc.Alpha();
    return colour;
}

WXD_EXPORTED void
wxd_Grid_SetLabelTextColour(wxd_Grid_t* self, wxd_Colour_t colour)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetLabelTextColour(
        wxColour(colour.r, colour.g, colour.b, colour.a));
}

// --- Batch Updates ---

WXD_EXPORTED void
wxd_Grid_BeginBatch(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->BeginBatch();
}

WXD_EXPORTED void
wxd_Grid_EndBatch(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->EndBatch();
}

WXD_EXPORTED int
wxd_Grid_GetBatchCount(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetBatchCount();
}

WXD_EXPORTED void
wxd_Grid_ForceRefresh(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->ForceRefresh();
}

// --- Clear Grid ---

WXD_EXPORTED void
wxd_Grid_ClearGrid(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->ClearGrid();
}

// --- Drag Operations ---

WXD_EXPORTED void
wxd_Grid_EnableDragRowSize(wxd_Grid_t* self, bool enable)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->EnableDragRowSize(enable);
}

WXD_EXPORTED void
wxd_Grid_EnableDragColSize(wxd_Grid_t* self, bool enable)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->EnableDragColSize(enable);
}

WXD_EXPORTED void
wxd_Grid_EnableDragGridSize(wxd_Grid_t* self, bool enable)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->EnableDragGridSize(enable);
}

WXD_EXPORTED void
wxd_Grid_EnableDragCell(wxd_Grid_t* self, bool enable)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->EnableDragCell(enable);
}

WXD_EXPORTED bool
wxd_Grid_CanDragRowSize(wxd_Grid_t* self, int row)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->CanDragRowSize(row);
}

WXD_EXPORTED bool
wxd_Grid_CanDragColSize(wxd_Grid_t* self, int col)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->CanDragColSize(col);
}

// --- Selection Mode ---

WXD_EXPORTED void
wxd_Grid_SetSelectionMode(wxd_Grid_t* self, int selmode)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetSelectionMode(
        static_cast<wxGrid::wxGridSelectionModes>(selmode));
}

WXD_EXPORTED int
wxd_Grid_GetSelectionMode(wxd_Grid_t* self)
{
    if (!self) return 0;
    return static_cast<int>(reinterpret_cast<wxGrid*>(self)->GetSelectionMode());
}

// --- Selection Colors ---

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetSelectionBackground(wxd_Grid_t* self)
{
    wxd_Colour_t colour = {0, 0, 128, 255};
    if (!self) return colour;
    wxColour wxc = reinterpret_cast<wxGrid*>(self)->GetSelectionBackground();
    colour.r = wxc.Red();
    colour.g = wxc.Green();
    colour.b = wxc.Blue();
    colour.a = wxc.Alpha();
    return colour;
}

WXD_EXPORTED void
wxd_Grid_SetSelectionBackground(wxd_Grid_t* self, wxd_Colour_t colour)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetSelectionBackground(
        wxColour(colour.r, colour.g, colour.b, colour.a));
}

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetSelectionForeground(wxd_Grid_t* self)
{
    wxd_Colour_t colour = {255, 255, 255, 255};
    if (!self) return colour;
    wxColour wxc = reinterpret_cast<wxGrid*>(self)->GetSelectionForeground();
    colour.r = wxc.Red();
    colour.g = wxc.Green();
    colour.b = wxc.Blue();
    colour.a = wxc.Alpha();
    return colour;
}

WXD_EXPORTED void
wxd_Grid_SetSelectionForeground(wxd_Grid_t* self, wxd_Colour_t colour)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetSelectionForeground(
        wxColour(colour.r, colour.g, colour.b, colour.a));
}

// --- Column Position Functions ---

WXD_EXPORTED int
wxd_Grid_GetColAt(wxd_Grid_t* self, int pos)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->GetColAt(pos);
}

WXD_EXPORTED int
wxd_Grid_GetColPos(wxd_Grid_t* self, int idx)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->GetColPos(idx);
}

WXD_EXPORTED void
wxd_Grid_SetColPos(wxd_Grid_t* self, int idx, int pos)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetColPos(idx, pos);
}

WXD_EXPORTED void
wxd_Grid_ResetColPos(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->ResetColPos();
}

// --- Row/Column Hiding ---

WXD_EXPORTED void
wxd_Grid_HideRow(wxd_Grid_t* self, int row)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->HideRow(row);
}

WXD_EXPORTED void
wxd_Grid_ShowRow(wxd_Grid_t* self, int row)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->ShowRow(row);
}

WXD_EXPORTED bool
wxd_Grid_IsRowShown(wxd_Grid_t* self, int row)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->IsRowShown(row);
}

WXD_EXPORTED void
wxd_Grid_HideCol(wxd_Grid_t* self, int col)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->HideCol(col);
}

WXD_EXPORTED void
wxd_Grid_ShowCol(wxd_Grid_t* self, int col)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->ShowCol(col);
}

WXD_EXPORTED bool
wxd_Grid_IsColShown(wxd_Grid_t* self, int col)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->IsColShown(col);
}

// --- Grid Event Data Accessors ---

WXD_EXPORTED int
wxd_GridEvent_GetRow(wxd_Event_t* event)
{
    if (!event) return -1;
    wxGridEvent* evt = dynamic_cast<wxGridEvent*>(reinterpret_cast<wxEvent*>(event));
    if (!evt) return -1;
    return evt->GetRow();
}

WXD_EXPORTED int
wxd_GridEvent_GetCol(wxd_Event_t* event)
{
    if (!event) return -1;
    wxGridEvent* evt = dynamic_cast<wxGridEvent*>(reinterpret_cast<wxEvent*>(event));
    if (!evt) return -1;
    return evt->GetCol();
}

WXD_EXPORTED wxd_Point
wxd_GridEvent_GetPosition(wxd_Event_t* event)
{
    wxd_Point pos = {0, 0};
    if (!event) return pos;
    wxGridEvent* evt = dynamic_cast<wxGridEvent*>(reinterpret_cast<wxEvent*>(event));
    if (!evt) return pos;
    wxPoint p = evt->GetPosition();
    pos.x = p.x;
    pos.y = p.y;
    return pos;
}

WXD_EXPORTED bool
wxd_GridEvent_Selecting(wxd_Event_t* event)
{
    if (!event) return false;
    wxGridEvent* evt = dynamic_cast<wxGridEvent*>(reinterpret_cast<wxEvent*>(event));
    if (!evt) return false;
    return evt->Selecting();
}

WXD_EXPORTED bool
wxd_GridEvent_ControlDown(wxd_Event_t* event)
{
    if (!event) return false;
    wxGridEvent* evt = dynamic_cast<wxGridEvent*>(reinterpret_cast<wxEvent*>(event));
    if (!evt) return false;
    return evt->ControlDown();
}

WXD_EXPORTED bool
wxd_GridEvent_ShiftDown(wxd_Event_t* event)
{
    if (!event) return false;
    wxGridEvent* evt = dynamic_cast<wxGridEvent*>(reinterpret_cast<wxEvent*>(event));
    if (!evt) return false;
    return evt->ShiftDown();
}

WXD_EXPORTED bool
wxd_GridEvent_AltDown(wxd_Event_t* event)
{
    if (!event) return false;
    wxGridEvent* evt = dynamic_cast<wxGridEvent*>(reinterpret_cast<wxEvent*>(event));
    if (!evt) return false;
    return evt->AltDown();
}

WXD_EXPORTED bool
wxd_GridEvent_MetaDown(wxd_Event_t* event)
{
    if (!event) return false;
    wxGridEvent* evt = dynamic_cast<wxGridEvent*>(reinterpret_cast<wxEvent*>(event));
    if (!evt) return false;
    return evt->MetaDown();
}

} // extern "C"
