use crate::prelude::Size;
use crate::sizers::WxSizer as WxSizerTrait;
use crate::sizers::base::Sizer;
use std::ops::Deref;
use wxdragon_sys as ffi;

// --- FlexGridSizer Grow Mode ---
widget_style_enum!(
    name: FlexGrowMode,
    doc: "Grow mode flags for FlexGridSizer.",
    variants: {
        None: ffi::WXD_FLEX_GROWMODE_NONE, "Don't resize the cells in non-flexible direction at all.",
        Specified: ffi::WXD_FLEX_GROWMODE_SPECIFIED, "Uniformly resize only the specified ones (default).",
        All: ffi::WXD_FLEX_GROWMODE_ALL, "Uniformly resize all cells."
    },
    default_variant: Specified
);

#[derive(Clone, Copy)]
pub struct FlexGridSizer {
    raw_specific_ptr: *mut ffi::wxd_FlexGridSizer_t, // Specific pointer for FlexGridSizer FFI calls
    sizer_base: Sizer,                               // Base Sizer for common functionality and Deref
}

impl FlexGridSizer {
    /// Creates a new FlexGridSizer wrapper from a raw wxFlexGridSizer pointer.
    /// Unsafe because the caller must ensure the pointer is valid.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_FlexGridSizer_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            // The pointer for the base Sizer is the same as the specific FlexGridSizer pointer.
            let base_ptr = ptr as *mut ffi::wxd_Sizer_t;
            unsafe {
                Sizer::from_ptr(base_ptr).map(|sizer_base| FlexGridSizer {
                    raw_specific_ptr: ptr,
                    sizer_base,
                })
            }
        }
    }

    pub fn builder(rows: i32, cols: i32) -> FlexGridSizerBuilder {
        FlexGridSizerBuilder::new(rows, cols)
    }

    // FFI calls now use raw_specific_ptr
    pub fn add_growable_col(&self, idx: usize, proportion: i32) {
        unsafe { ffi::wxd_FlexGridSizer_AddGrowableCol(self.raw_specific_ptr, idx, proportion) };
    }

    pub fn add_growable_row(&self, idx: usize, proportion: i32) {
        unsafe { ffi::wxd_FlexGridSizer_AddGrowableRow(self.raw_specific_ptr, idx, proportion) };
    }

    pub fn set_flexible_direction(&self, direction: i32) {
        unsafe { ffi::wxd_FlexGridSizer_SetFlexibleDirection(self.raw_specific_ptr, direction) };
    }

    pub fn set_non_flexible_grow_mode(&self, mode: FlexGrowMode) {
        unsafe { ffi::wxd_FlexGridSizer_SetNonFlexibleGrowMode(self.raw_specific_ptr, mode.bits() as i32) };
    }

    // REMOVED sizer_ptr method, use as_sizer_ptr() from trait or Deref
    // pub fn sizer_ptr(&self) -> *mut ffi::wxd_Sizer_t {
    //     self.sizer_base.as_sizer_ptr() // Assuming Sizer has as_sizer_ptr from WxSizer trait
    // }
}

// Implement WxSizer trait by delegating to sizer_base
impl WxSizerTrait for FlexGridSizer {
    fn as_sizer_ptr(&self) -> *mut ffi::wxd_Sizer_t {
        self.sizer_base.as_sizer_ptr()
    }
}

// Implement Deref to Sizer to access common sizer methods
impl Deref for FlexGridSizer {
    type Target = Sizer;
    fn deref(&self) -> &Self::Target {
        &self.sizer_base
    }
}

pub struct FlexGridSizerBuilder {
    rows: i32,
    cols: i32,
    vgap: i32,
    hgap: i32,
    gap: Option<Size>,
}

impl FlexGridSizerBuilder {
    fn new(rows: i32, cols: i32) -> Self {
        FlexGridSizerBuilder {
            rows,
            cols,
            vgap: 0,
            hgap: 0,
            gap: None,
        }
    }

    pub fn with_vgap(mut self, vgap: i32) -> Self {
        self.vgap = vgap;
        self.gap = None;
        self
    }

    pub fn with_hgap(mut self, hgap: i32) -> Self {
        self.hgap = hgap;
        self.gap = None;
        self
    }

    pub fn with_gap(mut self, gap: Size) -> Self {
        self.gap = Some(gap);
        self
    }

    pub fn build(self) -> FlexGridSizer {
        let ptr = if let Some(g) = self.gap {
            unsafe { ffi::wxd_FlexGridSizer_CreateWithGap(self.rows, self.cols, g.width, g.height) }
        } else {
            unsafe { ffi::wxd_FlexGridSizer_Create(self.rows, self.cols, self.vgap, self.hgap) }
        };
        // Use the unsafe from_ptr constructor
        unsafe { FlexGridSizer::from_ptr(ptr).expect("Failed to create wxFlexGridSizer") }
    }
}

// Ensure FlexGridSizer implements WxSizer trait if it exists and is appropriate.
// For now, relying on Deref to Sizer for common methods.
// We might need to implement `WxSizer` trait for `FlexGridSizer` explicitly.

// Example of how WxSizer methods would be available via Deref:
// let flex_sizer = FlexGridSizer::builder(2,2).build();
// flex_sizer.add_window(some_window, 0, SizerFlag::wxEXPAND(), 0);

// TODO: Implement Drop carefully if FlexGridSizer has ownership distinct from Sizer
// Currently, Sizer handles destruction.
