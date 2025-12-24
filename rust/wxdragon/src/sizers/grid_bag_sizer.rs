use crate::prelude::Size;
use crate::sizers::WxSizer as WxSizerTrait;
use crate::sizers::base::Sizer;
use crate::window::WxWidget;
use std::ops::Deref;
use wxdragon_sys as ffi;

// --- GBPosition ---
/// Represents a position in a GridBagSizer grid (row, column).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GBPosition {
    pub row: i32,
    pub col: i32,
}

impl GBPosition {
    /// Create a new grid position.
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    /// Get the row.
    pub fn get_row(&self) -> i32 {
        self.row
    }

    /// Get the column.
    pub fn get_col(&self) -> i32 {
        self.col
    }

    /// Set the row.
    pub fn set_row(&mut self, row: i32) {
        self.row = row;
    }

    /// Set the column.
    pub fn set_col(&mut self, col: i32) {
        self.col = col;
    }

    /// Convert to FFI type.
    pub(crate) fn to_ffi(self) -> ffi::wxd_GBPosition {
        ffi::wxd_GBPosition {
            row: self.row,
            col: self.col,
        }
    }

    /// Convert from FFI type.
    pub(crate) fn from_ffi(pos: ffi::wxd_GBPosition) -> Self {
        Self {
            row: pos.row,
            col: pos.col,
        }
    }
}

// --- GBSpan ---
/// Represents how many rows and columns an item spans in a GridBagSizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GBSpan {
    pub rowspan: i32,
    pub colspan: i32,
}

impl GBSpan {
    /// Create a new grid span.
    pub fn new(rowspan: i32, colspan: i32) -> Self {
        Self { rowspan, colspan }
    }

    /// Get the row span.
    pub fn get_rowspan(&self) -> i32 {
        self.rowspan
    }

    /// Get the column span.
    pub fn get_colspan(&self) -> i32 {
        self.colspan
    }

    /// Set the row span.
    pub fn set_rowspan(&mut self, rowspan: i32) {
        self.rowspan = rowspan;
    }

    /// Set the column span.
    pub fn set_colspan(&mut self, colspan: i32) {
        self.colspan = colspan;
    }

    /// Convert to FFI type.
    pub(crate) fn to_ffi(self) -> ffi::wxd_GBSpan {
        ffi::wxd_GBSpan {
            rowspan: self.rowspan,
            colspan: self.colspan,
        }
    }

    /// Convert from FFI type.
    pub(crate) fn from_ffi(span: ffi::wxd_GBSpan) -> Self {
        Self {
            rowspan: span.rowspan,
            colspan: span.colspan,
        }
    }
}

// --- Constants ---
/// Default position (0, 0).
pub const DEFAULT_GB_POSITION: GBPosition = GBPosition { row: 0, col: 0 };

/// Default span (1, 1).
pub const DEFAULT_GB_SPAN: GBSpan = GBSpan { rowspan: 1, colspan: 1 };

// --- GridBagSizer ---
/// A sizer that can lay out items in a grid with items at specified cells,
/// and with the option of row and/or column spanning.
#[derive(Clone, Copy)]
pub struct GridBagSizer {
    raw_specific_ptr: *mut ffi::wxd_GridBagSizer_t, // Specific pointer for GridBagSizer FFI calls
    sizer_base: Sizer,                              // Base Sizer for common functionality and Deref
}

impl GridBagSizer {
    /// Creates a new GridBagSizer wrapper from a raw wxGridBagSizer pointer.
    /// Unsafe because the caller must ensure the pointer is valid.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_GridBagSizer_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            // The pointer for the base Sizer is the same as the specific GridBagSizer pointer.
            let base_ptr = ptr as *mut ffi::wxd_Sizer_t;
            unsafe {
                Sizer::from_ptr(base_ptr).map(|sizer_base| GridBagSizer {
                    raw_specific_ptr: ptr,
                    sizer_base,
                })
            }
        }
    }

    /// Get the raw GridBagSizer pointer for GridBagSizer-specific operations.
    #[allow(dead_code)]
    pub(crate) fn as_grid_bag_sizer_ptr(&self) -> *mut ffi::wxd_GridBagSizer_t {
        self.raw_specific_ptr
    }

    /// Create a new GridBagSizer builder.
    pub fn builder() -> GridBagSizerBuilder {
        GridBagSizerBuilder::new()
    }

    // --- Add methods with position and span ---

    /// Add a window at the specified position with optional span.
    pub fn add_at<W: WxWidget>(&self, window: &W, pos: GBPosition, span: GBSpan, flag: crate::sizers::SizerFlag, border: i32) {
        unsafe {
            ffi::wxd_GridBagSizer_AddWindow(
                self.raw_specific_ptr,
                window.handle_ptr(),
                pos.to_ffi(),
                span.to_ffi(),
                flag.bits() as i32,
                border,
            );
        }
    }

    /// Add a sizer at the specified position with optional span.
    pub fn add_sizer_at<S: WxSizerTrait>(
        &self,
        sizer: &S,
        pos: GBPosition,
        span: GBSpan,
        flag: crate::sizers::SizerFlag,
        border: i32,
    ) {
        unsafe {
            ffi::wxd_GridBagSizer_AddSizer(
                self.raw_specific_ptr,
                sizer.as_sizer_ptr(),
                pos.to_ffi(),
                span.to_ffi(),
                flag.bits() as i32,
                border,
            );
        }
    }

    /// Add a spacer at the specified position with optional span.
    pub fn add_spacer_at(
        &self,
        width: i32,
        height: i32,
        pos: GBPosition,
        span: GBSpan,
        flag: crate::sizers::SizerFlag,
        border: i32,
    ) {
        unsafe {
            ffi::wxd_GridBagSizer_AddSpacer(
                self.raw_specific_ptr,
                width,
                height,
                pos.to_ffi(),
                span.to_ffi(),
                flag.bits() as i32,
                border,
            );
        }
    }

    // --- Position management ---

    /// Get the position of a window in the grid.
    pub fn get_item_position<W: WxWidget>(&self, window: &W) -> GBPosition {
        let pos = unsafe { ffi::wxd_GridBagSizer_GetItemPosition_Window(self.raw_specific_ptr, window.handle_ptr()) };
        GBPosition::from_ffi(pos)
    }

    /// Get the position of a sizer in the grid.
    pub fn get_sizer_position<S: WxSizerTrait>(&self, sizer: &S) -> GBPosition {
        let pos = unsafe { ffi::wxd_GridBagSizer_GetItemPosition_Sizer(self.raw_specific_ptr, sizer.as_sizer_ptr()) };
        GBPosition::from_ffi(pos)
    }

    /// Get the position of an item by index.
    pub fn get_item_position_by_index(&self, index: usize) -> GBPosition {
        let pos = unsafe { ffi::wxd_GridBagSizer_GetItemPosition_Index(self.raw_specific_ptr, index) };
        GBPosition::from_ffi(pos)
    }

    /// Set the position of a window in the grid.
    pub fn set_item_position<W: WxWidget>(&self, window: &W, pos: GBPosition) -> bool {
        unsafe { ffi::wxd_GridBagSizer_SetItemPosition_Window(self.raw_specific_ptr, window.handle_ptr(), pos.to_ffi()) }
    }

    /// Set the position of a sizer in the grid.
    pub fn set_sizer_position<S: WxSizerTrait>(&self, sizer: &S, pos: GBPosition) -> bool {
        unsafe { ffi::wxd_GridBagSizer_SetItemPosition_Sizer(self.raw_specific_ptr, sizer.as_sizer_ptr(), pos.to_ffi()) }
    }

    /// Set the position of an item by index.
    pub fn set_item_position_by_index(&self, index: usize, pos: GBPosition) -> bool {
        unsafe { ffi::wxd_GridBagSizer_SetItemPosition_Index(self.raw_specific_ptr, index, pos.to_ffi()) }
    }

    // --- Span management ---

    /// Get the span of a window in the grid.
    pub fn get_item_span<W: WxWidget>(&self, window: &W) -> GBSpan {
        let span = unsafe { ffi::wxd_GridBagSizer_GetItemSpan_Window(self.raw_specific_ptr, window.handle_ptr()) };
        GBSpan::from_ffi(span)
    }

    /// Get the span of a sizer in the grid.
    pub fn get_sizer_span<S: WxSizerTrait>(&self, sizer: &S) -> GBSpan {
        let span = unsafe { ffi::wxd_GridBagSizer_GetItemSpan_Sizer(self.raw_specific_ptr, sizer.as_sizer_ptr()) };
        GBSpan::from_ffi(span)
    }

    /// Get the span of an item by index.
    pub fn get_item_span_by_index(&self, index: usize) -> GBSpan {
        let span = unsafe { ffi::wxd_GridBagSizer_GetItemSpan_Index(self.raw_specific_ptr, index) };
        GBSpan::from_ffi(span)
    }

    /// Set the span of a window in the grid.
    pub fn set_item_span<W: WxWidget>(&self, window: &W, span: GBSpan) -> bool {
        unsafe { ffi::wxd_GridBagSizer_SetItemSpan_Window(self.raw_specific_ptr, window.handle_ptr(), span.to_ffi()) }
    }

    /// Set the span of a sizer in the grid.
    pub fn set_sizer_span<S: WxSizerTrait>(&self, sizer: &S, span: GBSpan) -> bool {
        unsafe { ffi::wxd_GridBagSizer_SetItemSpan_Sizer(self.raw_specific_ptr, sizer.as_sizer_ptr(), span.to_ffi()) }
    }

    /// Set the span of an item by index.
    pub fn set_item_span_by_index(&self, index: usize, span: GBSpan) -> bool {
        unsafe { ffi::wxd_GridBagSizer_SetItemSpan_Index(self.raw_specific_ptr, index, span.to_ffi()) }
    }

    // --- Cell size management ---

    /// Get the size used for cells in the grid with no item.
    pub fn get_empty_cell_size(&self) -> Size {
        let size = unsafe { ffi::wxd_GridBagSizer_GetEmptyCellSize(self.raw_specific_ptr) };
        Size::new(size.width, size.height)
    }

    /// Set the size used for cells in the grid with no item.
    pub fn set_empty_cell_size(&self, size: Size) {
        let ffi_size = ffi::wxd_Size {
            width: size.width,
            height: size.height,
        };
        unsafe { ffi::wxd_GridBagSizer_SetEmptyCellSize(self.raw_specific_ptr, ffi_size) };
    }

    /// Get the size of the specified cell, including hgap and vgap.
    /// Only valid after a Layout.
    pub fn get_cell_size(&self, row: i32, col: i32) -> Size {
        let size = unsafe { ffi::wxd_GridBagSizer_GetCellSize(self.raw_specific_ptr, row, col) };
        Size::new(size.width, size.height)
    }
}

// Implement WxSizer trait by delegating to sizer_base
impl WxSizerTrait for GridBagSizer {
    fn as_sizer_ptr(&self) -> *mut ffi::wxd_Sizer_t {
        self.sizer_base.as_sizer_ptr()
    }
}

// Implement Deref to Sizer to access common sizer methods
impl Deref for GridBagSizer {
    type Target = Sizer;
    fn deref(&self) -> &Self::Target {
        &self.sizer_base
    }
}

// --- GridBagSizerBuilder ---
/// Builder for [`GridBagSizer`].
pub struct GridBagSizerBuilder {
    vgap: i32,
    hgap: i32,
}

impl GridBagSizerBuilder {
    fn new() -> Self {
        Self { vgap: 0, hgap: 0 }
    }

    /// Set the vertical gap between rows.
    pub fn with_vgap(mut self, vgap: i32) -> Self {
        self.vgap = vgap;
        self
    }

    /// Set the horizontal gap between columns.
    pub fn with_hgap(mut self, hgap: i32) -> Self {
        self.hgap = hgap;
        self
    }

    /// Set both horizontal and vertical gaps.
    pub fn with_gap(mut self, gap: i32) -> Self {
        self.vgap = gap;
        self.hgap = gap;
        self
    }

    /// Build the GridBagSizer.
    pub fn build(self) -> GridBagSizer {
        let ptr = unsafe { ffi::wxd_GridBagSizer_Create(self.vgap, self.hgap) };
        unsafe { GridBagSizer::from_ptr(ptr).expect("Failed to create wxGridBagSizer") }
    }
}
