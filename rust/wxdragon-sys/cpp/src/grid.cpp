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

WXD_EXPORTED void
wxd_Grid_AutoSizeRowLabelSize(wxd_Grid_t* self, int row)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->AutoSizeRowLabelSize(row);
}

WXD_EXPORTED void
wxd_Grid_AutoSizeColLabelSize(wxd_Grid_t* self, int col)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->AutoSizeColLabelSize(col);
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

WXD_EXPORTED int
wxd_Grid_GetSelectedCells(wxd_Grid_t* self, wxd_GridCellCoords* buffer, int buffer_len)
{
    if (!self) return 0;
    wxGridCellCoordsArray cells = reinterpret_cast<wxGrid*>(self)->GetSelectedCells();
    int count = static_cast<int>(cells.GetCount());
    if (buffer && buffer_len > 0) {
        int copy_count = (count < buffer_len) ? count : buffer_len;
        for (int i = 0; i < copy_count; i++) {
            buffer[i].row = cells[i].GetRow();
            buffer[i].col = cells[i].GetCol();
        }
    }
    return count;
}

WXD_EXPORTED int
wxd_Grid_GetSelectedBlocks(wxd_Grid_t* self, wxd_GridBlockCoords* buffer, int buffer_len)
{
    if (!self) return 0;
    // GetSelectedBlocks() returns wxGridBlocks (an iterator range), not a vector.
    // We need to iterate it to collect the blocks.
    wxGridBlocks range = reinterpret_cast<wxGrid*>(self)->GetSelectedBlocks();
    std::vector<wxGridBlockCoords> blocks(range.begin(), range.end());
    int count = static_cast<int>(blocks.size());
    if (buffer && buffer_len > 0) {
        int copy_count = (count < buffer_len) ? count : buffer_len;
        for (int i = 0; i < copy_count; i++) {
            buffer[i].top_row = blocks[i].GetTopRow();
            buffer[i].left_col = blocks[i].GetLeftCol();
            buffer[i].bottom_row = blocks[i].GetBottomRow();
            buffer[i].right_col = blocks[i].GetRightCol();
        }
    }
    return count;
}

WXD_EXPORTED int
wxd_Grid_GetSelectedRowBlocks(wxd_Grid_t* self, wxd_GridBlockCoords* buffer, int buffer_len)
{
    if (!self) return 0;
    wxGridBlockCoordsVector blocks = reinterpret_cast<wxGrid*>(self)->GetSelectedRowBlocks();
    int count = static_cast<int>(blocks.size());
    if (buffer && buffer_len > 0) {
        int copy_count = (count < buffer_len) ? count : buffer_len;
        for (int i = 0; i < copy_count; i++) {
            buffer[i].top_row = blocks[i].GetTopRow();
            buffer[i].left_col = blocks[i].GetLeftCol();
            buffer[i].bottom_row = blocks[i].GetBottomRow();
            buffer[i].right_col = blocks[i].GetRightCol();
        }
    }
    return count;
}

WXD_EXPORTED int
wxd_Grid_GetSelectedColBlocks(wxd_Grid_t* self, wxd_GridBlockCoords* buffer, int buffer_len)
{
    if (!self) return 0;
    wxGridBlockCoordsVector blocks = reinterpret_cast<wxGrid*>(self)->GetSelectedColBlocks();
    int count = static_cast<int>(blocks.size());
    if (buffer && buffer_len > 0) {
        int copy_count = (count < buffer_len) ? count : buffer_len;
        for (int i = 0; i < copy_count; i++) {
            buffer[i].top_row = blocks[i].GetTopRow();
            buffer[i].left_col = blocks[i].GetLeftCol();
            buffer[i].bottom_row = blocks[i].GetBottomRow();
            buffer[i].right_col = blocks[i].GetRightCol();
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

// --- Cell Font ---

WXD_EXPORTED wxd_Font_t*
wxd_Grid_GetCellFont(wxd_Grid_t* self, int row, int col)
{
    if (!self) return nullptr;
    wxFont font = reinterpret_cast<wxGrid*>(self)->GetCellFont(row, col);
    return reinterpret_cast<wxd_Font_t*>(new wxFont(font));
}

WXD_EXPORTED void
wxd_Grid_SetCellFont(wxd_Grid_t* self, int row, int col, const wxd_Font_t* font)
{
    if (!self || !font) return;
    reinterpret_cast<wxGrid*>(self)->SetCellFont(row, col, *reinterpret_cast<const wxFont*>(font));
}

WXD_EXPORTED wxd_Font_t*
wxd_Grid_GetDefaultCellFont(wxd_Grid_t* self)
{
    if (!self) return nullptr;
    wxFont font = reinterpret_cast<wxGrid*>(self)->GetDefaultCellFont();
    return reinterpret_cast<wxd_Font_t*>(new wxFont(font));
}

WXD_EXPORTED void
wxd_Grid_SetDefaultCellFont(wxd_Grid_t* self, const wxd_Font_t* font)
{
    if (!self || !font) return;
    reinterpret_cast<wxGrid*>(self)->SetDefaultCellFont(*reinterpret_cast<const wxFont*>(font));
}

// --- Label Font ---

WXD_EXPORTED wxd_Font_t*
wxd_Grid_GetLabelFont(wxd_Grid_t* self)
{
    if (!self) return nullptr;
    wxFont font = reinterpret_cast<wxGrid*>(self)->GetLabelFont();
    return reinterpret_cast<wxd_Font_t*>(new wxFont(font));
}

WXD_EXPORTED void
wxd_Grid_SetLabelFont(wxd_Grid_t* self, const wxd_Font_t* font)
{
    if (!self || !font) return;
    reinterpret_cast<wxGrid*>(self)->SetLabelFont(*reinterpret_cast<const wxFont*>(font));
}

// --- Label Alignment ---

WXD_EXPORTED void
wxd_Grid_GetColLabelAlignment(wxd_Grid_t* self, int* horiz, int* vert)
{
    if (!self || !horiz || !vert) return;
    reinterpret_cast<wxGrid*>(self)->GetColLabelAlignment(horiz, vert);
}

WXD_EXPORTED void
wxd_Grid_SetColLabelAlignment(wxd_Grid_t* self, int horiz, int vert)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetColLabelAlignment(horiz, vert);
}

WXD_EXPORTED void
wxd_Grid_GetRowLabelAlignment(wxd_Grid_t* self, int* horiz, int* vert)
{
    if (!self || !horiz || !vert) return;
    reinterpret_cast<wxGrid*>(self)->GetRowLabelAlignment(horiz, vert);
}

WXD_EXPORTED void
wxd_Grid_SetRowLabelAlignment(wxd_Grid_t* self, int horiz, int vert)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetRowLabelAlignment(horiz, vert);
}

WXD_EXPORTED int
wxd_Grid_GetColLabelTextOrientation(wxd_Grid_t* self)
{
    if (!self) return wxHORIZONTAL;
    return reinterpret_cast<wxGrid*>(self)->GetColLabelTextOrientation();
}

WXD_EXPORTED void
wxd_Grid_SetColLabelTextOrientation(wxd_Grid_t* self, int textOrientation)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetColLabelTextOrientation(textOrientation);
}

// --- Corner Label ---

WXD_EXPORTED int
wxd_Grid_GetCornerLabelValue(wxd_Grid_t* self, char* buffer, int buffer_len)
{
    if (!self) return 0;
    wxString value = reinterpret_cast<wxGrid*>(self)->GetCornerLabelValue();
    return static_cast<int>(wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, buffer_len));
}

WXD_EXPORTED void
wxd_Grid_SetCornerLabelValue(wxd_Grid_t* self, const char* value)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetCornerLabelValue(wxString::FromUTF8(value ? value : ""));
}

WXD_EXPORTED void
wxd_Grid_GetCornerLabelAlignment(wxd_Grid_t* self, int* horiz, int* vert)
{
    if (!self || !horiz || !vert) return;
    reinterpret_cast<wxGrid*>(self)->GetCornerLabelAlignment(horiz, vert);
}

WXD_EXPORTED void
wxd_Grid_SetCornerLabelAlignment(wxd_Grid_t* self, int horiz, int vert)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetCornerLabelAlignment(horiz, vert);
}

WXD_EXPORTED int
wxd_Grid_GetCornerLabelTextOrientation(wxd_Grid_t* self)
{
    if (!self) return wxHORIZONTAL;
    return reinterpret_cast<wxGrid*>(self)->GetCornerLabelTextOrientation();
}

WXD_EXPORTED void
wxd_Grid_SetCornerLabelTextOrientation(wxd_Grid_t* self, int textOrientation)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetCornerLabelTextOrientation(textOrientation);
}

// --- Native Column Header ---

WXD_EXPORTED void
wxd_Grid_SetUseNativeColLabels(wxd_Grid_t* self, bool native_labels)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetUseNativeColLabels(native_labels);
}

WXD_EXPORTED bool
wxd_Grid_UseNativeColHeader(wxd_Grid_t* self, bool native_header)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->UseNativeColHeader(native_header);
}

WXD_EXPORTED bool
wxd_Grid_IsUsingNativeHeader(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->IsUsingNativeHeader();
}

// --- Cell Spanning ---

WXD_EXPORTED void
wxd_Grid_SetCellSize(wxd_Grid_t* self, int row, int col, int num_rows, int num_cols)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetCellSize(row, col, num_rows, num_cols);
}

WXD_EXPORTED int
wxd_Grid_GetCellSize(wxd_Grid_t* self, int row, int col, int* num_rows, int* num_cols)
{
    if (!self || !num_rows || !num_cols) return 0; // CellSpan_None
    return static_cast<int>(reinterpret_cast<wxGrid*>(self)->GetCellSize(row, col, num_rows, num_cols));
}

// --- Cell Overflow ---

WXD_EXPORTED bool
wxd_Grid_GetCellOverflow(wxd_Grid_t* self, int row, int col)
{
    if (!self) return true;
    return reinterpret_cast<wxGrid*>(self)->GetCellOverflow(row, col);
}

WXD_EXPORTED void
wxd_Grid_SetCellOverflow(wxd_Grid_t* self, int row, int col, bool allow)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetCellOverflow(row, col, allow);
}

WXD_EXPORTED bool
wxd_Grid_GetDefaultCellOverflow(wxd_Grid_t* self)
{
    if (!self) return true;
    return reinterpret_cast<wxGrid*>(self)->GetDefaultCellOverflow();
}

WXD_EXPORTED void
wxd_Grid_SetDefaultCellOverflow(wxd_Grid_t* self, bool allow)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetDefaultCellOverflow(allow);
}

// --- Column Format ---

WXD_EXPORTED void
wxd_Grid_SetColFormatBool(wxd_Grid_t* self, int col)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetColFormatBool(col);
}

WXD_EXPORTED void
wxd_Grid_SetColFormatNumber(wxd_Grid_t* self, int col)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetColFormatNumber(col);
}

WXD_EXPORTED void
wxd_Grid_SetColFormatFloat(wxd_Grid_t* self, int col, int width, int precision)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetColFormatFloat(col, width, precision);
}

WXD_EXPORTED void
wxd_Grid_SetColFormatDate(wxd_Grid_t* self, int col, const char* format)
{
    if (!self) return;
    wxString fmt = format ? wxString::FromUTF8(format) : wxString();
    reinterpret_cast<wxGrid*>(self)->SetColFormatDate(col, fmt);
}

WXD_EXPORTED void
wxd_Grid_SetColFormatCustom(wxd_Grid_t* self, int col, const char* typeName)
{
    if (!self || !typeName) return;
    reinterpret_cast<wxGrid*>(self)->SetColFormatCustom(col, wxString::FromUTF8(typeName));
}

// --- Sorting ---

WXD_EXPORTED int
wxd_Grid_GetSortingColumn(wxd_Grid_t* self)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->GetSortingColumn();
}

WXD_EXPORTED bool
wxd_Grid_IsSortingBy(wxd_Grid_t* self, int col)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->IsSortingBy(col);
}

WXD_EXPORTED bool
wxd_Grid_IsSortOrderAscending(wxd_Grid_t* self)
{
    if (!self) return true;
    return reinterpret_cast<wxGrid*>(self)->IsSortOrderAscending();
}

WXD_EXPORTED void
wxd_Grid_SetSortingColumn(wxd_Grid_t* self, int col, bool ascending)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetSortingColumn(col, ascending);
}

WXD_EXPORTED void
wxd_Grid_UnsetSortingColumn(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->UnsetSortingColumn();
}

// --- Tab Behaviour ---

WXD_EXPORTED void
wxd_Grid_SetTabBehaviour(wxd_Grid_t* self, int behaviour)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetTabBehaviour(static_cast<wxGrid::TabBehaviour>(behaviour));
}

// --- Frozen Rows/Cols ---

WXD_EXPORTED bool
wxd_Grid_FreezeTo(wxd_Grid_t* self, int row, int col)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->FreezeTo(row, col);
}

WXD_EXPORTED int
wxd_Grid_GetNumberFrozenRows(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetNumberFrozenRows();
}

WXD_EXPORTED int
wxd_Grid_GetNumberFrozenCols(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetNumberFrozenCols();
}

// --- Row/Col Minimal Sizes ---

WXD_EXPORTED int
wxd_Grid_GetColMinimalAcceptableWidth(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetColMinimalAcceptableWidth();
}

WXD_EXPORTED void
wxd_Grid_SetColMinimalAcceptableWidth(wxd_Grid_t* self, int width)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetColMinimalAcceptableWidth(width);
}

WXD_EXPORTED void
wxd_Grid_SetColMinimalWidth(wxd_Grid_t* self, int col, int width)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetColMinimalWidth(col, width);
}

WXD_EXPORTED int
wxd_Grid_GetRowMinimalAcceptableHeight(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetRowMinimalAcceptableHeight();
}

WXD_EXPORTED void
wxd_Grid_SetRowMinimalAcceptableHeight(wxd_Grid_t* self, int height)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetRowMinimalAcceptableHeight(height);
}

WXD_EXPORTED void
wxd_Grid_SetRowMinimalHeight(wxd_Grid_t* self, int row, int height)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetRowMinimalHeight(row, height);
}

// --- Default Label Sizes ---

WXD_EXPORTED int
wxd_Grid_GetDefaultRowLabelSize(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetDefaultRowLabelSize();
}

WXD_EXPORTED int
wxd_Grid_GetDefaultColLabelSize(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetDefaultColLabelSize();
}

// --- Cell Edit Control ---

WXD_EXPORTED bool
wxd_Grid_CanEnableCellControl(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->CanEnableCellControl();
}

WXD_EXPORTED bool
wxd_Grid_IsCellEditControlShown(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->IsCellEditControlShown();
}

WXD_EXPORTED bool
wxd_Grid_IsCurrentCellReadOnly(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->IsCurrentCellReadOnly();
}

WXD_EXPORTED void
wxd_Grid_HideCellEditControl(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->HideCellEditControl();
}

WXD_EXPORTED void
wxd_Grid_ShowCellEditControl(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->ShowCellEditControl();
}

WXD_EXPORTED void
wxd_Grid_SaveEditControlValue(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SaveEditControlValue();
}

// --- Cell Highlight ---

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetCellHighlightColour(wxd_Grid_t* self)
{
    wxd_Colour_t colour = {0, 0, 0, 255};
    if (!self) return colour;
    wxColour wxc = reinterpret_cast<wxGrid*>(self)->GetCellHighlightColour();
    colour.r = wxc.Red(); colour.g = wxc.Green(); colour.b = wxc.Blue(); colour.a = wxc.Alpha();
    return colour;
}

WXD_EXPORTED void
wxd_Grid_SetCellHighlightColour(wxd_Grid_t* self, wxd_Colour_t colour)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetCellHighlightColour(wxColour(colour.r, colour.g, colour.b, colour.a));
}

WXD_EXPORTED int
wxd_Grid_GetCellHighlightPenWidth(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetCellHighlightPenWidth();
}

WXD_EXPORTED void
wxd_Grid_SetCellHighlightPenWidth(wxd_Grid_t* self, int width)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetCellHighlightPenWidth(width);
}

WXD_EXPORTED int
wxd_Grid_GetCellHighlightROPenWidth(wxd_Grid_t* self)
{
    if (!self) return 0;
    return reinterpret_cast<wxGrid*>(self)->GetCellHighlightROPenWidth();
}

WXD_EXPORTED void
wxd_Grid_SetCellHighlightROPenWidth(wxd_Grid_t* self, int width)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetCellHighlightROPenWidth(width);
}

// --- Grid Frozen Border ---

WXD_EXPORTED void
wxd_Grid_SetGridFrozenBorderColour(wxd_Grid_t* self, wxd_Colour_t colour)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetGridFrozenBorderColour(wxColour(colour.r, colour.g, colour.b, colour.a));
}

WXD_EXPORTED void
wxd_Grid_SetGridFrozenBorderPenWidth(wxd_Grid_t* self, int width)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetGridFrozenBorderPenWidth(width);
}

// --- Cursor Movement ---

WXD_EXPORTED bool
wxd_Grid_MoveCursorUp(wxd_Grid_t* self, bool expandSelection)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->MoveCursorUp(expandSelection);
}

WXD_EXPORTED bool
wxd_Grid_MoveCursorDown(wxd_Grid_t* self, bool expandSelection)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->MoveCursorDown(expandSelection);
}

WXD_EXPORTED bool
wxd_Grid_MoveCursorLeft(wxd_Grid_t* self, bool expandSelection)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->MoveCursorLeft(expandSelection);
}

WXD_EXPORTED bool
wxd_Grid_MoveCursorRight(wxd_Grid_t* self, bool expandSelection)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->MoveCursorRight(expandSelection);
}

WXD_EXPORTED bool
wxd_Grid_MoveCursorUpBlock(wxd_Grid_t* self, bool expandSelection)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->MoveCursorUpBlock(expandSelection);
}

WXD_EXPORTED bool
wxd_Grid_MoveCursorDownBlock(wxd_Grid_t* self, bool expandSelection)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->MoveCursorDownBlock(expandSelection);
}

WXD_EXPORTED bool
wxd_Grid_MoveCursorLeftBlock(wxd_Grid_t* self, bool expandSelection)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->MoveCursorLeftBlock(expandSelection);
}

WXD_EXPORTED bool
wxd_Grid_MoveCursorRightBlock(wxd_Grid_t* self, bool expandSelection)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->MoveCursorRightBlock(expandSelection);
}

WXD_EXPORTED bool
wxd_Grid_MovePageUp(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->MovePageUp();
}

WXD_EXPORTED bool
wxd_Grid_MovePageDown(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->MovePageDown();
}

WXD_EXPORTED wxd_GridCellCoords
wxd_Grid_GetGridCursorCoords(wxd_Grid_t* self)
{
    wxd_GridCellCoords coords = {-1, -1};
    if (!self) return coords;
    const wxGridCellCoords& c = reinterpret_cast<wxGrid*>(self)->GetGridCursorCoords();
    coords.row = c.GetRow();
    coords.col = c.GetCol();
    return coords;
}

// --- Scrolling ---

WXD_EXPORTED int
wxd_Grid_GetScrollLineX(wxd_Grid_t* self)
{
    if (!self) return 15;
    return reinterpret_cast<wxGrid*>(self)->GetScrollLineX();
}

WXD_EXPORTED int
wxd_Grid_GetScrollLineY(wxd_Grid_t* self)
{
    if (!self) return 15;
    return reinterpret_cast<wxGrid*>(self)->GetScrollLineY();
}

WXD_EXPORTED void
wxd_Grid_SetScrollLineX(wxd_Grid_t* self, int x)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetScrollLineX(x);
}

WXD_EXPORTED void
wxd_Grid_SetScrollLineY(wxd_Grid_t* self, int y)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetScrollLineY(y);
}

WXD_EXPORTED int
wxd_Grid_GetFirstFullyVisibleRow(wxd_Grid_t* self)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->GetFirstFullyVisibleRow();
}

WXD_EXPORTED int
wxd_Grid_GetFirstFullyVisibleColumn(wxd_Grid_t* self)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->GetFirstFullyVisibleColumn();
}

// --- Coordinate Conversion ---

WXD_EXPORTED int
wxd_Grid_XToCol(wxd_Grid_t* self, int x, bool clipToMinMax)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->XToCol(x, clipToMinMax);
}

WXD_EXPORTED int
wxd_Grid_YToRow(wxd_Grid_t* self, int y, bool clipToMinMax)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->YToRow(y, clipToMinMax);
}

WXD_EXPORTED int
wxd_Grid_XToEdgeOfCol(wxd_Grid_t* self, int x)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->XToEdgeOfCol(x);
}

WXD_EXPORTED int
wxd_Grid_YToEdgeOfRow(wxd_Grid_t* self, int y)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->YToEdgeOfRow(y);
}

WXD_EXPORTED wxd_GridCellCoords
wxd_Grid_XYToCell(wxd_Grid_t* self, int x, int y)
{
    wxd_GridCellCoords coords = {-1, -1};
    if (!self) return coords;
    wxGridCellCoords c = reinterpret_cast<wxGrid*>(self)->XYToCell(x, y);
    coords.row = c.GetRow();
    coords.col = c.GetCol();
    return coords;
}

WXD_EXPORTED wxd_Rect
wxd_Grid_CellToRect(wxd_Grid_t* self, int row, int col)
{
    wxd_Rect rect = {0, 0, 0, 0};
    if (!self) return rect;
    wxRect r = reinterpret_cast<wxGrid*>(self)->CellToRect(row, col);
    rect.x = r.x; rect.y = r.y; rect.width = r.width; rect.height = r.height;
    return rect;
}

WXD_EXPORTED wxd_Rect
wxd_Grid_BlockToDeviceRect(wxd_Grid_t* self, int topRow, int leftCol, int bottomRow, int rightCol)
{
    wxd_Rect rect = {0, 0, 0, 0};
    if (!self) return rect;
    wxGridCellCoords tl(topRow, leftCol);
    wxGridCellCoords br(bottomRow, rightCol);
    wxRect r = reinterpret_cast<wxGrid*>(self)->BlockToDeviceRect(tl, br);
    rect.x = r.x; rect.y = r.y; rect.width = r.width; rect.height = r.height;
    return rect;
}

// --- Grid Clipping ---

WXD_EXPORTED bool
wxd_Grid_AreHorzGridLinesClipped(wxd_Grid_t* self)
{
    if (!self) return true;
    return reinterpret_cast<wxGrid*>(self)->AreHorzGridLinesClipped();
}

WXD_EXPORTED bool
wxd_Grid_AreVertGridLinesClipped(wxd_Grid_t* self)
{
    if (!self) return true;
    return reinterpret_cast<wxGrid*>(self)->AreVertGridLinesClipped();
}

WXD_EXPORTED void
wxd_Grid_ClipHorzGridLines(wxd_Grid_t* self, bool clip)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->ClipHorzGridLines(clip);
}

WXD_EXPORTED void
wxd_Grid_ClipVertGridLines(wxd_Grid_t* self, bool clip)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->ClipVertGridLines(clip);
}

// --- Extra Drag/Move Operations ---

WXD_EXPORTED bool
wxd_Grid_CanDragCell(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->CanDragCell();
}

WXD_EXPORTED bool
wxd_Grid_CanDragColMove(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->CanDragColMove();
}

WXD_EXPORTED bool
wxd_Grid_CanDragGridSize(wxd_Grid_t* self)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->CanDragGridSize();
}

WXD_EXPORTED bool
wxd_Grid_EnableDragColMove(wxd_Grid_t* self, bool enable)
{
    if (!self) return false;
    return reinterpret_cast<wxGrid*>(self)->EnableDragColMove(enable);
}

WXD_EXPORTED void
wxd_Grid_DisableDragColMove(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->DisableDragColMove();
}

WXD_EXPORTED void
wxd_Grid_DisableDragColSize(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->DisableDragColSize();
}

WXD_EXPORTED void
wxd_Grid_DisableDragRowSize(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->DisableDragRowSize();
}

WXD_EXPORTED void
wxd_Grid_DisableDragGridSize(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->DisableDragGridSize();
}

WXD_EXPORTED void
wxd_Grid_DisableColResize(wxd_Grid_t* self, int col)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->DisableColResize(col);
}

WXD_EXPORTED void
wxd_Grid_DisableRowResize(wxd_Grid_t* self, int row)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->DisableRowResize(row);
}

// --- Row Position/Move ---

WXD_EXPORTED int
wxd_Grid_GetRowAt(wxd_Grid_t* self, int pos)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->GetRowAt(pos);
}

WXD_EXPORTED int
wxd_Grid_GetRowPos(wxd_Grid_t* self, int idx)
{
    if (!self) return -1;
    return reinterpret_cast<wxGrid*>(self)->GetRowPos(idx);
}

WXD_EXPORTED void
wxd_Grid_SetRowPos(wxd_Grid_t* self, int idx, int pos)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetRowPos(idx, pos);
}

WXD_EXPORTED void
wxd_Grid_ResetRowPos(wxd_Grid_t* self)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->ResetRowPos();
}

// --- Margins ---

WXD_EXPORTED void
wxd_Grid_SetMargins(wxd_Grid_t* self, int extraWidth, int extraHeight)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->SetMargins(extraWidth, extraHeight);
}

// --- Refresh ---

WXD_EXPORTED void
wxd_Grid_RefreshAttr(wxd_Grid_t* self, int row, int col)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->RefreshAttr(row, col);
}

WXD_EXPORTED void
wxd_Grid_RefreshBlock(wxd_Grid_t* self, int topRow, int leftCol, int bottomRow, int rightCol)
{
    if (!self) return;
    reinterpret_cast<wxGrid*>(self)->RefreshBlock(topRow, leftCol, bottomRow, rightCol);
}

// --- Grid Event Data Accessors ---
//
// wxGrid uses several event classes:
//   wxGridEvent          - cell clicks, label clicks, select, edit, drag
//   wxGridSizeEvent      - row/col resize (has GetRowOrCol(), not GetRow/GetCol)
//   wxGridRangeSelectEvent - range selection (has GetTopRow/GetLeftCol etc.)
//   wxGridEditorCreatedEvent - editor created

WXD_EXPORTED int
wxd_GridEvent_GetRow(wxd_Event_t* event)
{
    if (!event) return -1;
    wxEvent* raw = reinterpret_cast<wxEvent*>(event);

    // Try wxGridEvent first (most common)
    if (auto* evt = dynamic_cast<wxGridEvent*>(raw))
        return evt->GetRow();

    // wxGridSizeEvent: GetRowOrCol() is the row for ROW_SIZE events
    if (auto* evt = dynamic_cast<wxGridSizeEvent*>(raw)) {
        if (evt->GetEventType() == wxEVT_GRID_ROW_SIZE)
            return evt->GetRowOrCol();
        return -1; // COL_SIZE event has no meaningful row
    }

    // wxGridRangeSelectEvent
    if (auto* evt = dynamic_cast<wxGridRangeSelectEvent*>(raw))
        return evt->GetTopRow();

    return -1;
}

WXD_EXPORTED int
wxd_GridEvent_GetCol(wxd_Event_t* event)
{
    if (!event) return -1;
    wxEvent* raw = reinterpret_cast<wxEvent*>(event);

    if (auto* evt = dynamic_cast<wxGridEvent*>(raw))
        return evt->GetCol();

    // wxGridSizeEvent: GetRowOrCol() is the col for COL_SIZE events
    if (auto* evt = dynamic_cast<wxGridSizeEvent*>(raw)) {
        if (evt->GetEventType() == wxEVT_GRID_COL_SIZE)
            return evt->GetRowOrCol();
        return -1; // ROW_SIZE event has no meaningful col
    }

    if (auto* evt = dynamic_cast<wxGridRangeSelectEvent*>(raw))
        return evt->GetLeftCol();

    return -1;
}

WXD_EXPORTED wxd_Point
wxd_GridEvent_GetPosition(wxd_Event_t* event)
{
    wxd_Point pos = {0, 0};
    if (!event) return pos;
    wxEvent* raw = reinterpret_cast<wxEvent*>(event);

    if (auto* evt = dynamic_cast<wxGridEvent*>(raw)) {
        wxPoint p = evt->GetPosition();
        pos.x = p.x; pos.y = p.y;
        return pos;
    }
    if (auto* evt = dynamic_cast<wxGridSizeEvent*>(raw)) {
        wxPoint p = evt->GetPosition();
        pos.x = p.x; pos.y = p.y;
        return pos;
    }
    return pos;
}

WXD_EXPORTED bool
wxd_GridEvent_Selecting(wxd_Event_t* event)
{
    if (!event) return false;
    wxEvent* raw = reinterpret_cast<wxEvent*>(event);

    if (auto* evt = dynamic_cast<wxGridEvent*>(raw))
        return evt->Selecting();
    if (auto* evt = dynamic_cast<wxGridRangeSelectEvent*>(raw))
        return evt->Selecting();
    return false;
}

WXD_EXPORTED bool
wxd_GridEvent_ControlDown(wxd_Event_t* event)
{
    if (!event) return false;
    wxEvent* raw = reinterpret_cast<wxEvent*>(event);

    if (auto* evt = dynamic_cast<wxGridEvent*>(raw))
        return evt->ControlDown();
    if (auto* evt = dynamic_cast<wxGridSizeEvent*>(raw))
        return evt->ControlDown();
    if (auto* evt = dynamic_cast<wxGridRangeSelectEvent*>(raw))
        return evt->ControlDown();
    return false;
}

WXD_EXPORTED bool
wxd_GridEvent_ShiftDown(wxd_Event_t* event)
{
    if (!event) return false;
    wxEvent* raw = reinterpret_cast<wxEvent*>(event);

    if (auto* evt = dynamic_cast<wxGridEvent*>(raw))
        return evt->ShiftDown();
    if (auto* evt = dynamic_cast<wxGridSizeEvent*>(raw))
        return evt->ShiftDown();
    if (auto* evt = dynamic_cast<wxGridRangeSelectEvent*>(raw))
        return evt->ShiftDown();
    return false;
}

WXD_EXPORTED bool
wxd_GridEvent_AltDown(wxd_Event_t* event)
{
    if (!event) return false;
    wxEvent* raw = reinterpret_cast<wxEvent*>(event);

    if (auto* evt = dynamic_cast<wxGridEvent*>(raw))
        return evt->AltDown();
    if (auto* evt = dynamic_cast<wxGridSizeEvent*>(raw))
        return evt->AltDown();
    if (auto* evt = dynamic_cast<wxGridRangeSelectEvent*>(raw))
        return evt->AltDown();
    return false;
}

WXD_EXPORTED bool
wxd_GridEvent_MetaDown(wxd_Event_t* event)
{
    if (!event) return false;
    wxEvent* raw = reinterpret_cast<wxEvent*>(event);

    if (auto* evt = dynamic_cast<wxGridEvent*>(raw))
        return evt->MetaDown();
    if (auto* evt = dynamic_cast<wxGridSizeEvent*>(raw))
        return evt->MetaDown();
    if (auto* evt = dynamic_cast<wxGridRangeSelectEvent*>(raw))
        return evt->MetaDown();
    return false;
}

} // extern "C"
