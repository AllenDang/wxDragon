#ifndef WXD_GRID_H
#define WXD_GRID_H

#include "../wxd_types.h"

// --- Grid Selection Modes ---
#define WXD_GRID_SELECT_CELLS 0
#define WXD_GRID_SELECT_ROWS 1
#define WXD_GRID_SELECT_COLUMNS 2
#define WXD_GRID_SELECT_ROWS_OR_COLUMNS 3
#define WXD_GRID_SELECT_NONE 4

// --- Grid Render Styles ---
#define WXD_GRID_DRAW_ROWS_HEADER 0x001
#define WXD_GRID_DRAW_COLS_HEADER 0x002
#define WXD_GRID_DRAW_CELL_LINES 0x004
#define WXD_GRID_DRAW_BOX_RECT 0x008
#define WXD_GRID_DRAW_SELECTION 0x010
#define WXD_GRID_DRAW_DEFAULT 0x00F

// --- Grid Cell Coordinates ---
typedef struct {
    int row;
    int col;
} wxd_GridCellCoords;

// --- Grid Block Coordinates ---
typedef struct {
    int top_row;
    int left_col;
    int bottom_row;
    int right_col;
} wxd_GridBlockCoords;

// --- Grid Functions ---

// Creation and destruction
WXD_EXPORTED wxd_Grid_t*
wxd_Grid_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                wxd_Style_t style);

// Initialize grid with specified number of rows and columns
WXD_EXPORTED bool
wxd_Grid_CreateGrid(wxd_Grid_t* self, int numRows, int numCols, int selectionMode);

// --- Grid Dimensions ---
WXD_EXPORTED int
wxd_Grid_GetNumberRows(wxd_Grid_t* self);

WXD_EXPORTED int
wxd_Grid_GetNumberCols(wxd_Grid_t* self);

// --- Row and Column Management ---
WXD_EXPORTED bool
wxd_Grid_InsertRows(wxd_Grid_t* self, int pos, int numRows, bool updateLabels);

WXD_EXPORTED bool
wxd_Grid_AppendRows(wxd_Grid_t* self, int numRows, bool updateLabels);

WXD_EXPORTED bool
wxd_Grid_DeleteRows(wxd_Grid_t* self, int pos, int numRows, bool updateLabels);

WXD_EXPORTED bool
wxd_Grid_InsertCols(wxd_Grid_t* self, int pos, int numCols, bool updateLabels);

WXD_EXPORTED bool
wxd_Grid_AppendCols(wxd_Grid_t* self, int numCols, bool updateLabels);

WXD_EXPORTED bool
wxd_Grid_DeleteCols(wxd_Grid_t* self, int pos, int numCols, bool updateLabels);

// --- Cell Value Accessors ---
WXD_EXPORTED int
wxd_Grid_GetCellValue(wxd_Grid_t* self, int row, int col, char* buffer, int buffer_len);

WXD_EXPORTED void
wxd_Grid_SetCellValue(wxd_Grid_t* self, int row, int col, const char* value);

// --- Label Functions ---
WXD_EXPORTED int
wxd_Grid_GetRowLabelValue(wxd_Grid_t* self, int row, char* buffer, int buffer_len);

WXD_EXPORTED void
wxd_Grid_SetRowLabelValue(wxd_Grid_t* self, int row, const char* value);

WXD_EXPORTED int
wxd_Grid_GetColLabelValue(wxd_Grid_t* self, int col, char* buffer, int buffer_len);

WXD_EXPORTED void
wxd_Grid_SetColLabelValue(wxd_Grid_t* self, int col, const char* value);

WXD_EXPORTED int
wxd_Grid_GetRowLabelSize(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetRowLabelSize(wxd_Grid_t* self, int width);

WXD_EXPORTED int
wxd_Grid_GetColLabelSize(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetColLabelSize(wxd_Grid_t* self, int height);

WXD_EXPORTED void
wxd_Grid_HideRowLabels(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_HideColLabels(wxd_Grid_t* self);

// --- Row and Column Sizes ---
WXD_EXPORTED int
wxd_Grid_GetDefaultRowSize(wxd_Grid_t* self);

WXD_EXPORTED int
wxd_Grid_GetRowSize(wxd_Grid_t* self, int row);

WXD_EXPORTED void
wxd_Grid_SetDefaultRowSize(wxd_Grid_t* self, int height, bool resizeExistingRows);

WXD_EXPORTED void
wxd_Grid_SetRowSize(wxd_Grid_t* self, int row, int height);

WXD_EXPORTED int
wxd_Grid_GetDefaultColSize(wxd_Grid_t* self);

WXD_EXPORTED int
wxd_Grid_GetColSize(wxd_Grid_t* self, int col);

WXD_EXPORTED void
wxd_Grid_SetDefaultColSize(wxd_Grid_t* self, int width, bool resizeExistingCols);

WXD_EXPORTED void
wxd_Grid_SetColSize(wxd_Grid_t* self, int col, int width);

WXD_EXPORTED void
wxd_Grid_AutoSizeColumn(wxd_Grid_t* self, int col, bool setAsMin);

WXD_EXPORTED void
wxd_Grid_AutoSizeRow(wxd_Grid_t* self, int row, bool setAsMin);

WXD_EXPORTED void
wxd_Grid_AutoSizeColumns(wxd_Grid_t* self, bool setAsMin);

WXD_EXPORTED void
wxd_Grid_AutoSizeRows(wxd_Grid_t* self, bool setAsMin);

WXD_EXPORTED void
wxd_Grid_AutoSize(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_AutoSizeRowLabelSize(wxd_Grid_t* self, int row);

WXD_EXPORTED void
wxd_Grid_AutoSizeColLabelSize(wxd_Grid_t* self, int col);

// --- Cell Formatting ---
WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetCellBackgroundColour(wxd_Grid_t* self, int row, int col);

WXD_EXPORTED void
wxd_Grid_SetCellBackgroundColour(wxd_Grid_t* self, int row, int col, wxd_Colour_t colour);

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetCellTextColour(wxd_Grid_t* self, int row, int col);

WXD_EXPORTED void
wxd_Grid_SetCellTextColour(wxd_Grid_t* self, int row, int col, wxd_Colour_t colour);

WXD_EXPORTED void
wxd_Grid_GetCellAlignment(wxd_Grid_t* self, int row, int col, int* horiz, int* vert);

WXD_EXPORTED void
wxd_Grid_SetCellAlignment(wxd_Grid_t* self, int row, int col, int horiz, int vert);

// --- Default Cell Formatting ---
WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetDefaultCellBackgroundColour(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetDefaultCellBackgroundColour(wxd_Grid_t* self, wxd_Colour_t colour);

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetDefaultCellTextColour(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetDefaultCellTextColour(wxd_Grid_t* self, wxd_Colour_t colour);

WXD_EXPORTED void
wxd_Grid_GetDefaultCellAlignment(wxd_Grid_t* self, int* horiz, int* vert);

WXD_EXPORTED void
wxd_Grid_SetDefaultCellAlignment(wxd_Grid_t* self, int horiz, int vert);

// --- Read-Only Cells ---
WXD_EXPORTED bool
wxd_Grid_IsReadOnly(wxd_Grid_t* self, int row, int col);

WXD_EXPORTED void
wxd_Grid_SetReadOnly(wxd_Grid_t* self, int row, int col, bool isReadOnly);

// --- Selection ---
WXD_EXPORTED void
wxd_Grid_SelectRow(wxd_Grid_t* self, int row, bool addToSelected);

WXD_EXPORTED void
wxd_Grid_SelectCol(wxd_Grid_t* self, int col, bool addToSelected);

WXD_EXPORTED void
wxd_Grid_SelectBlock(wxd_Grid_t* self, int topRow, int leftCol, int bottomRow, int rightCol,
                     bool addToSelected);

WXD_EXPORTED void
wxd_Grid_SelectAll(wxd_Grid_t* self);

WXD_EXPORTED bool
wxd_Grid_IsSelection(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_DeselectRow(wxd_Grid_t* self, int row);

WXD_EXPORTED void
wxd_Grid_DeselectCol(wxd_Grid_t* self, int col);

WXD_EXPORTED void
wxd_Grid_DeselectCell(wxd_Grid_t* self, int row, int col);

WXD_EXPORTED void
wxd_Grid_ClearSelection(wxd_Grid_t* self);

WXD_EXPORTED bool
wxd_Grid_IsInSelection(wxd_Grid_t* self, int row, int col);

// Get selected rows/columns (returns count, fills buffer)
WXD_EXPORTED int
wxd_Grid_GetSelectedRows(wxd_Grid_t* self, int* buffer, int buffer_len);

WXD_EXPORTED int
wxd_Grid_GetSelectedCols(wxd_Grid_t* self, int* buffer, int buffer_len);

// Get selected cells (returns count, fills buffer with cell coords)
WXD_EXPORTED int
wxd_Grid_GetSelectedCells(wxd_Grid_t* self, wxd_GridCellCoords* buffer, int buffer_len);

// Get selected blocks (returns count, fills buffer with block coords)
WXD_EXPORTED int
wxd_Grid_GetSelectedBlocks(wxd_Grid_t* self, wxd_GridBlockCoords* buffer, int buffer_len);

// Get selected row blocks (returns count, fills buffer with block coords)
WXD_EXPORTED int
wxd_Grid_GetSelectedRowBlocks(wxd_Grid_t* self, wxd_GridBlockCoords* buffer, int buffer_len);

// Get selected col blocks (returns count, fills buffer with block coords)
WXD_EXPORTED int
wxd_Grid_GetSelectedColBlocks(wxd_Grid_t* self, wxd_GridBlockCoords* buffer, int buffer_len);

// --- Grid Cursor ---
WXD_EXPORTED int
wxd_Grid_GetGridCursorRow(wxd_Grid_t* self);

WXD_EXPORTED int
wxd_Grid_GetGridCursorCol(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetGridCursor(wxd_Grid_t* self, int row, int col);

WXD_EXPORTED void
wxd_Grid_GoToCell(wxd_Grid_t* self, int row, int col);

// --- Cell Visibility ---
WXD_EXPORTED bool
wxd_Grid_IsVisible(wxd_Grid_t* self, int row, int col, bool wholeCellVisible);

WXD_EXPORTED void
wxd_Grid_MakeCellVisible(wxd_Grid_t* self, int row, int col);

// --- Editing ---
WXD_EXPORTED bool
wxd_Grid_IsEditable(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_EnableEditing(wxd_Grid_t* self, bool edit);

WXD_EXPORTED void
wxd_Grid_EnableCellEditControl(wxd_Grid_t* self, bool enable);

WXD_EXPORTED void
wxd_Grid_DisableCellEditControl(wxd_Grid_t* self);

WXD_EXPORTED bool
wxd_Grid_IsCellEditControlEnabled(wxd_Grid_t* self);

// --- Grid Lines ---
WXD_EXPORTED void
wxd_Grid_EnableGridLines(wxd_Grid_t* self, bool enable);

WXD_EXPORTED bool
wxd_Grid_GridLinesEnabled(wxd_Grid_t* self);

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetGridLineColour(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetGridLineColour(wxd_Grid_t* self, wxd_Colour_t colour);

// --- Label Appearance ---
WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetLabelBackgroundColour(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetLabelBackgroundColour(wxd_Grid_t* self, wxd_Colour_t colour);

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetLabelTextColour(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetLabelTextColour(wxd_Grid_t* self, wxd_Colour_t colour);

// --- Batch Updates ---
WXD_EXPORTED void
wxd_Grid_BeginBatch(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_EndBatch(wxd_Grid_t* self);

WXD_EXPORTED int
wxd_Grid_GetBatchCount(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_ForceRefresh(wxd_Grid_t* self);

// --- Clear Grid ---
WXD_EXPORTED void
wxd_Grid_ClearGrid(wxd_Grid_t* self);

// --- Drag and Drop Operations ---
WXD_EXPORTED void
wxd_Grid_EnableDragRowSize(wxd_Grid_t* self, bool enable);

WXD_EXPORTED void
wxd_Grid_EnableDragColSize(wxd_Grid_t* self, bool enable);

WXD_EXPORTED void
wxd_Grid_EnableDragGridSize(wxd_Grid_t* self, bool enable);

WXD_EXPORTED void
wxd_Grid_EnableDragCell(wxd_Grid_t* self, bool enable);

WXD_EXPORTED bool
wxd_Grid_CanDragRowSize(wxd_Grid_t* self, int row);

WXD_EXPORTED bool
wxd_Grid_CanDragColSize(wxd_Grid_t* self, int col);

// --- Selection Mode ---
WXD_EXPORTED void
wxd_Grid_SetSelectionMode(wxd_Grid_t* self, int selmode);

WXD_EXPORTED int
wxd_Grid_GetSelectionMode(wxd_Grid_t* self);

// --- Selection Colors ---
WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetSelectionBackground(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetSelectionBackground(wxd_Grid_t* self, wxd_Colour_t colour);

WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetSelectionForeground(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetSelectionForeground(wxd_Grid_t* self, wxd_Colour_t colour);

// --- Row/Column Position Functions ---
WXD_EXPORTED int
wxd_Grid_GetColAt(wxd_Grid_t* self, int pos);

WXD_EXPORTED int
wxd_Grid_GetColPos(wxd_Grid_t* self, int idx);

WXD_EXPORTED void
wxd_Grid_SetColPos(wxd_Grid_t* self, int idx, int pos);

WXD_EXPORTED void
wxd_Grid_ResetColPos(wxd_Grid_t* self);

// --- Row Hiding ---
WXD_EXPORTED void
wxd_Grid_HideRow(wxd_Grid_t* self, int row);

WXD_EXPORTED void
wxd_Grid_ShowRow(wxd_Grid_t* self, int row);

WXD_EXPORTED bool
wxd_Grid_IsRowShown(wxd_Grid_t* self, int row);

// --- Column Hiding ---
WXD_EXPORTED void
wxd_Grid_HideCol(wxd_Grid_t* self, int col);

WXD_EXPORTED void
wxd_Grid_ShowCol(wxd_Grid_t* self, int col);

WXD_EXPORTED bool
wxd_Grid_IsColShown(wxd_Grid_t* self, int col);

// --- Cell Font ---
WXD_EXPORTED wxd_Font_t*
wxd_Grid_GetCellFont(wxd_Grid_t* self, int row, int col);

WXD_EXPORTED void
wxd_Grid_SetCellFont(wxd_Grid_t* self, int row, int col, const wxd_Font_t* font);

WXD_EXPORTED wxd_Font_t*
wxd_Grid_GetDefaultCellFont(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetDefaultCellFont(wxd_Grid_t* self, const wxd_Font_t* font);

// --- Label Font ---
WXD_EXPORTED wxd_Font_t*
wxd_Grid_GetLabelFont(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetLabelFont(wxd_Grid_t* self, const wxd_Font_t* font);

// --- Label Alignment ---
WXD_EXPORTED void
wxd_Grid_GetColLabelAlignment(wxd_Grid_t* self, int* horiz, int* vert);

WXD_EXPORTED void
wxd_Grid_SetColLabelAlignment(wxd_Grid_t* self, int horiz, int vert);

WXD_EXPORTED void
wxd_Grid_GetRowLabelAlignment(wxd_Grid_t* self, int* horiz, int* vert);

WXD_EXPORTED void
wxd_Grid_SetRowLabelAlignment(wxd_Grid_t* self, int horiz, int vert);

WXD_EXPORTED int
wxd_Grid_GetColLabelTextOrientation(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetColLabelTextOrientation(wxd_Grid_t* self, int textOrientation);

// --- Corner Label ---
WXD_EXPORTED int
wxd_Grid_GetCornerLabelValue(wxd_Grid_t* self, char* buffer, int buffer_len);

WXD_EXPORTED void
wxd_Grid_SetCornerLabelValue(wxd_Grid_t* self, const char* value);

WXD_EXPORTED void
wxd_Grid_GetCornerLabelAlignment(wxd_Grid_t* self, int* horiz, int* vert);

WXD_EXPORTED void
wxd_Grid_SetCornerLabelAlignment(wxd_Grid_t* self, int horiz, int vert);

WXD_EXPORTED int
wxd_Grid_GetCornerLabelTextOrientation(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetCornerLabelTextOrientation(wxd_Grid_t* self, int textOrientation);

// --- Native Column Header ---
WXD_EXPORTED void
wxd_Grid_SetUseNativeColLabels(wxd_Grid_t* self, bool native_labels);

WXD_EXPORTED bool
wxd_Grid_UseNativeColHeader(wxd_Grid_t* self, bool native_header);

WXD_EXPORTED bool
wxd_Grid_IsUsingNativeHeader(wxd_Grid_t* self);

// --- Cell Spanning ---
WXD_EXPORTED void
wxd_Grid_SetCellSize(wxd_Grid_t* self, int row, int col, int num_rows, int num_cols);

WXD_EXPORTED int
wxd_Grid_GetCellSize(wxd_Grid_t* self, int row, int col, int* num_rows, int* num_cols);

// --- Cell Overflow ---
WXD_EXPORTED bool
wxd_Grid_GetCellOverflow(wxd_Grid_t* self, int row, int col);

WXD_EXPORTED void
wxd_Grid_SetCellOverflow(wxd_Grid_t* self, int row, int col, bool allow);

WXD_EXPORTED bool
wxd_Grid_GetDefaultCellOverflow(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetDefaultCellOverflow(wxd_Grid_t* self, bool allow);

// --- Column Format ---
WXD_EXPORTED void
wxd_Grid_SetColFormatBool(wxd_Grid_t* self, int col);

WXD_EXPORTED void
wxd_Grid_SetColFormatNumber(wxd_Grid_t* self, int col);

WXD_EXPORTED void
wxd_Grid_SetColFormatFloat(wxd_Grid_t* self, int col, int width, int precision);

WXD_EXPORTED void
wxd_Grid_SetColFormatDate(wxd_Grid_t* self, int col, const char* format);

WXD_EXPORTED void
wxd_Grid_SetColFormatCustom(wxd_Grid_t* self, int col, const char* typeName);

// --- Sorting ---
WXD_EXPORTED int
wxd_Grid_GetSortingColumn(wxd_Grid_t* self);

WXD_EXPORTED bool
wxd_Grid_IsSortingBy(wxd_Grid_t* self, int col);

WXD_EXPORTED bool
wxd_Grid_IsSortOrderAscending(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetSortingColumn(wxd_Grid_t* self, int col, bool ascending);

WXD_EXPORTED void
wxd_Grid_UnsetSortingColumn(wxd_Grid_t* self);

// --- Tab Behaviour ---
// 0=Tab_Stop, 1=Tab_Wrap, 2=Tab_Leave
WXD_EXPORTED void
wxd_Grid_SetTabBehaviour(wxd_Grid_t* self, int behaviour);

// --- Frozen Rows/Cols ---
WXD_EXPORTED bool
wxd_Grid_FreezeTo(wxd_Grid_t* self, int row, int col);

WXD_EXPORTED int
wxd_Grid_GetNumberFrozenRows(wxd_Grid_t* self);

WXD_EXPORTED int
wxd_Grid_GetNumberFrozenCols(wxd_Grid_t* self);

// --- Row/Col Minimal Sizes ---
WXD_EXPORTED int
wxd_Grid_GetColMinimalAcceptableWidth(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetColMinimalAcceptableWidth(wxd_Grid_t* self, int width);

WXD_EXPORTED void
wxd_Grid_SetColMinimalWidth(wxd_Grid_t* self, int col, int width);

WXD_EXPORTED int
wxd_Grid_GetRowMinimalAcceptableHeight(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetRowMinimalAcceptableHeight(wxd_Grid_t* self, int height);

WXD_EXPORTED void
wxd_Grid_SetRowMinimalHeight(wxd_Grid_t* self, int row, int height);

// --- Default Label Sizes ---
WXD_EXPORTED int
wxd_Grid_GetDefaultRowLabelSize(wxd_Grid_t* self);

WXD_EXPORTED int
wxd_Grid_GetDefaultColLabelSize(wxd_Grid_t* self);

// --- Cell Edit Control ---
WXD_EXPORTED bool
wxd_Grid_CanEnableCellControl(wxd_Grid_t* self);

WXD_EXPORTED bool
wxd_Grid_IsCellEditControlShown(wxd_Grid_t* self);

WXD_EXPORTED bool
wxd_Grid_IsCurrentCellReadOnly(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_HideCellEditControl(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_ShowCellEditControl(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SaveEditControlValue(wxd_Grid_t* self);

// --- Cell Highlight ---
WXD_EXPORTED wxd_Colour_t
wxd_Grid_GetCellHighlightColour(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetCellHighlightColour(wxd_Grid_t* self, wxd_Colour_t colour);

WXD_EXPORTED int
wxd_Grid_GetCellHighlightPenWidth(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetCellHighlightPenWidth(wxd_Grid_t* self, int width);

WXD_EXPORTED int
wxd_Grid_GetCellHighlightROPenWidth(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetCellHighlightROPenWidth(wxd_Grid_t* self, int width);

// --- Grid Frozen Border ---
WXD_EXPORTED void
wxd_Grid_SetGridFrozenBorderColour(wxd_Grid_t* self, wxd_Colour_t colour);

WXD_EXPORTED void
wxd_Grid_SetGridFrozenBorderPenWidth(wxd_Grid_t* self, int width);

// --- Cursor Movement ---
WXD_EXPORTED bool
wxd_Grid_MoveCursorUp(wxd_Grid_t* self, bool expandSelection);

WXD_EXPORTED bool
wxd_Grid_MoveCursorDown(wxd_Grid_t* self, bool expandSelection);

WXD_EXPORTED bool
wxd_Grid_MoveCursorLeft(wxd_Grid_t* self, bool expandSelection);

WXD_EXPORTED bool
wxd_Grid_MoveCursorRight(wxd_Grid_t* self, bool expandSelection);

WXD_EXPORTED bool
wxd_Grid_MoveCursorUpBlock(wxd_Grid_t* self, bool expandSelection);

WXD_EXPORTED bool
wxd_Grid_MoveCursorDownBlock(wxd_Grid_t* self, bool expandSelection);

WXD_EXPORTED bool
wxd_Grid_MoveCursorLeftBlock(wxd_Grid_t* self, bool expandSelection);

WXD_EXPORTED bool
wxd_Grid_MoveCursorRightBlock(wxd_Grid_t* self, bool expandSelection);

WXD_EXPORTED bool
wxd_Grid_MovePageUp(wxd_Grid_t* self);

WXD_EXPORTED bool
wxd_Grid_MovePageDown(wxd_Grid_t* self);

WXD_EXPORTED wxd_GridCellCoords
wxd_Grid_GetGridCursorCoords(wxd_Grid_t* self);

// --- Scrolling ---
WXD_EXPORTED int
wxd_Grid_GetScrollLineX(wxd_Grid_t* self);

WXD_EXPORTED int
wxd_Grid_GetScrollLineY(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_SetScrollLineX(wxd_Grid_t* self, int x);

WXD_EXPORTED void
wxd_Grid_SetScrollLineY(wxd_Grid_t* self, int y);

WXD_EXPORTED int
wxd_Grid_GetFirstFullyVisibleRow(wxd_Grid_t* self);

WXD_EXPORTED int
wxd_Grid_GetFirstFullyVisibleColumn(wxd_Grid_t* self);

// --- Coordinate Conversion ---
WXD_EXPORTED int
wxd_Grid_XToCol(wxd_Grid_t* self, int x, bool clipToMinMax);

WXD_EXPORTED int
wxd_Grid_YToRow(wxd_Grid_t* self, int y, bool clipToMinMax);

WXD_EXPORTED int
wxd_Grid_XToEdgeOfCol(wxd_Grid_t* self, int x);

WXD_EXPORTED int
wxd_Grid_YToEdgeOfRow(wxd_Grid_t* self, int y);

WXD_EXPORTED wxd_GridCellCoords
wxd_Grid_XYToCell(wxd_Grid_t* self, int x, int y);

WXD_EXPORTED wxd_Rect
wxd_Grid_CellToRect(wxd_Grid_t* self, int row, int col);

WXD_EXPORTED wxd_Rect
wxd_Grid_BlockToDeviceRect(wxd_Grid_t* self, int topRow, int leftCol, int bottomRow, int rightCol);

// --- Grid Clipping ---
WXD_EXPORTED bool
wxd_Grid_AreHorzGridLinesClipped(wxd_Grid_t* self);

WXD_EXPORTED bool
wxd_Grid_AreVertGridLinesClipped(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_ClipHorzGridLines(wxd_Grid_t* self, bool clip);

WXD_EXPORTED void
wxd_Grid_ClipVertGridLines(wxd_Grid_t* self, bool clip);

// --- Extra Drag/Move Operations ---
WXD_EXPORTED bool
wxd_Grid_CanDragCell(wxd_Grid_t* self);

WXD_EXPORTED bool
wxd_Grid_CanDragColMove(wxd_Grid_t* self);

WXD_EXPORTED bool
wxd_Grid_CanDragGridSize(wxd_Grid_t* self);

WXD_EXPORTED bool
wxd_Grid_EnableDragColMove(wxd_Grid_t* self, bool enable);

WXD_EXPORTED void
wxd_Grid_DisableDragColMove(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_DisableDragColSize(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_DisableDragRowSize(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_DisableDragGridSize(wxd_Grid_t* self);

WXD_EXPORTED void
wxd_Grid_DisableColResize(wxd_Grid_t* self, int col);

WXD_EXPORTED void
wxd_Grid_DisableRowResize(wxd_Grid_t* self, int row);

// --- Row Position/Move ---
WXD_EXPORTED int
wxd_Grid_GetRowAt(wxd_Grid_t* self, int pos);

WXD_EXPORTED int
wxd_Grid_GetRowPos(wxd_Grid_t* self, int idx);

WXD_EXPORTED void
wxd_Grid_SetRowPos(wxd_Grid_t* self, int idx, int pos);

WXD_EXPORTED void
wxd_Grid_ResetRowPos(wxd_Grid_t* self);

// --- Margins ---
WXD_EXPORTED void
wxd_Grid_SetMargins(wxd_Grid_t* self, int extraWidth, int extraHeight);

// --- Refresh ---
WXD_EXPORTED void
wxd_Grid_RefreshAttr(wxd_Grid_t* self, int row, int col);

WXD_EXPORTED void
wxd_Grid_RefreshBlock(wxd_Grid_t* self, int topRow, int leftCol, int bottomRow, int rightCol);

// --- Grid Event Data Accessors ---
WXD_EXPORTED int
wxd_GridEvent_GetRow(wxd_Event_t* event);

WXD_EXPORTED int
wxd_GridEvent_GetCol(wxd_Event_t* event);

WXD_EXPORTED wxd_Point
wxd_GridEvent_GetPosition(wxd_Event_t* event);

WXD_EXPORTED bool
wxd_GridEvent_Selecting(wxd_Event_t* event);

WXD_EXPORTED bool
wxd_GridEvent_ControlDown(wxd_Event_t* event);

WXD_EXPORTED bool
wxd_GridEvent_ShiftDown(wxd_Event_t* event);

WXD_EXPORTED bool
wxd_GridEvent_AltDown(wxd_Event_t* event);

WXD_EXPORTED bool
wxd_GridEvent_MetaDown(wxd_Event_t* event);

#endif // WXD_GRID_H
