use crate::prelude::Size;
use crate::sizers::WxSizer as WxSizerTrait;
use crate::sizers::base::Sizer;
use std::marker::PhantomData;
use std::ops::Deref;
use wxdragon_sys as ffi;

/// Represents a wxGridSizer.
#[derive(Clone, Copy)]
pub struct GridSizer {
    raw_specific_ptr: *mut ffi::wxd_GridSizer_t, // Specific pointer for GridSizer FFI calls
    sizer_base: Sizer,                           // Base Sizer for common functionality and Deref
}

impl GridSizer {
    /// Creates a new GridSizer wrapper from a raw wxGridSizer pointer.
    /// Unsafe because the caller must ensure the pointer is valid.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_GridSizer_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            // The pointer for the base Sizer is the same as the specific GridSizer pointer.
            let base_ptr = ptr as *mut ffi::wxd_Sizer_t;
            unsafe {
                Sizer::from_ptr(base_ptr).map(|sizer_base| GridSizer {
                    raw_specific_ptr: ptr,
                    sizer_base,
                })
            }
        }
    }

    /// Get the raw GridSizer pointer for GridSizer-specific operations.
    #[allow(dead_code)]
    pub(crate) fn as_grid_sizer_ptr(&self) -> *mut ffi::wxd_GridSizer_t {
        self.raw_specific_ptr
    }

    pub fn builder(rows: i32, cols: i32) -> GridSizerBuilder {
        GridSizerBuilder::new(rows, cols)
    }

    // GridSizer-specific methods
    pub fn set_cols(&self, cols: i32) {
        unsafe {
            ffi::wxd_GridSizer_SetCols(self.raw_specific_ptr, cols);
        }
    }

    pub fn set_rows(&self, rows: i32) {
        unsafe {
            ffi::wxd_GridSizer_SetRows(self.raw_specific_ptr, rows);
        }
    }

    pub fn set_vgap(&self, gap: i32) {
        unsafe {
            ffi::wxd_GridSizer_SetVGap(self.raw_specific_ptr, gap);
        }
    }

    pub fn set_hgap(&self, gap: i32) {
        unsafe {
            ffi::wxd_GridSizer_SetHGap(self.raw_specific_ptr, gap);
        }
    }

    pub fn get_cols(&self) -> i32 {
        unsafe { ffi::wxd_GridSizer_GetCols(self.raw_specific_ptr) }
    }

    pub fn get_rows(&self) -> i32 {
        unsafe { ffi::wxd_GridSizer_GetRows(self.raw_specific_ptr) }
    }

    pub fn get_vgap(&self) -> i32 {
        unsafe { ffi::wxd_GridSizer_GetVGap(self.raw_specific_ptr) }
    }

    pub fn get_hgap(&self) -> i32 {
        unsafe { ffi::wxd_GridSizer_GetHGap(self.raw_specific_ptr) }
    }
}

// Implement WxSizer trait by delegating to sizer_base
impl WxSizerTrait for GridSizer {
    fn as_sizer_ptr(&self) -> *mut ffi::wxd_Sizer_t {
        self.sizer_base.as_sizer_ptr()
    }
}

// Implement Deref to Sizer to access common sizer methods
impl Deref for GridSizer {
    type Target = Sizer;
    fn deref(&self) -> &Self::Target {
        &self.sizer_base
    }
}

/// Builder for [`GridSizer`].
pub struct GridSizerBuilder {
    rows: i32,
    cols: i32,
    vgap: i32,
    hgap: i32,
    gap: Option<Size>,
    _marker: PhantomData<()>,
}

impl GridSizerBuilder {
    fn new(rows: i32, cols: i32) -> Self {
        Self {
            rows,
            cols,
            vgap: 0,
            hgap: 0,
            gap: None,
            _marker: PhantomData,
        }
    }

    /// Set the vertical gap between rows.
    pub fn with_vgap(mut self, vgap: i32) -> Self {
        self.vgap = vgap;
        self.gap = None;
        self
    }

    /// Set the horizontal gap between columns.
    pub fn with_hgap(mut self, hgap: i32) -> Self {
        self.hgap = hgap;
        self.gap = None;
        self
    }

    /// Set both horizontal and vertical gaps using a Size.
    pub fn with_gap(mut self, gap: Size) -> Self {
        self.gap = Some(gap);
        self
    }

    /// Build the GridSizer.
    pub fn build(self) -> GridSizer {
        let ptr = unsafe {
            if let Some(g) = self.gap {
                ffi::wxd_GridSizer_CreateWithGap(self.rows, self.cols, g.width, g.height)
            } else {
                ffi::wxd_GridSizer_Create(self.rows, self.cols, self.vgap, self.hgap)
            }
        };
        unsafe { GridSizer::from_ptr(ptr).expect("Failed to create wxGridSizer") }
    }
}
