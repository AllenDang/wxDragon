use crate::sizers::WxSizer as WxSizerTrait;
use crate::sizers::base::{Orientation, Sizer};
use std::marker::PhantomData;
use std::ops::Deref;
use wxdragon_sys as ffi;

// --- WrapSizer Flag Constants ---
widget_style_enum!(
    name: WrapSizerFlag,
    doc: "Style flags for WrapSizer.",
    variants: {
        ExtendLastOnEachLine: ffi::WXD_EXTEND_LAST_ON_EACH_LINE, "Extend the last item on each line to fill available space.",
        RemoveLeadingSpaces: ffi::WXD_REMOVE_LEADING_SPACES, "Remove leading spacers from the beginning of each line.",
        DefaultFlags: ffi::WXD_WRAPSIZER_DEFAULT_FLAGS, "Default flags (ExtendLastOnEachLine | RemoveLeadingSpaces)."
    },
    default_variant: DefaultFlags
);

/// Represents a wxWrapSizer.
#[derive(Clone, Copy)]
pub struct WrapSizer {
    raw_specific_ptr: *mut ffi::wxd_WrapSizer_t, // Specific pointer for WrapSizer FFI calls
    sizer_base: Sizer,                           // Base Sizer for common functionality and Deref
}

impl WrapSizer {
    /// Creates a new WrapSizer wrapper from a raw wxWrapSizer pointer.
    /// Unsafe because the caller must ensure the pointer is valid.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_WrapSizer_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            // The pointer for the base Sizer is the same as the specific WrapSizer pointer.
            let base_ptr = ptr as *mut ffi::wxd_Sizer_t;
            unsafe {
                Sizer::from_ptr(base_ptr).map(|sizer_base| WrapSizer {
                    raw_specific_ptr: ptr,
                    sizer_base,
                })
            }
        }
    }

    /// Get the raw WrapSizer pointer for potential future WrapSizer-specific operations.
    /// This method is currently unused but kept for future extensibility.
    #[allow(dead_code)]
    pub(crate) fn as_wrap_sizer_ptr(&self) -> *mut ffi::wxd_WrapSizer_t {
        self.raw_specific_ptr
    }

    pub fn builder(orientation: Orientation) -> WrapSizerBuilder {
        WrapSizerBuilder::new(orientation)
    }
}

// Implement WxSizer trait by delegating to sizer_base
impl WxSizerTrait for WrapSizer {
    fn as_sizer_ptr(&self) -> *mut ffi::wxd_Sizer_t {
        self.sizer_base.as_sizer_ptr()
    }
}

// Implement Deref to Sizer to access common sizer methods
impl Deref for WrapSizer {
    type Target = Sizer;
    fn deref(&self) -> &Self::Target {
        &self.sizer_base
    }
}

/// Builder for [`WrapSizer`].
pub struct WrapSizerBuilder {
    orientation: Orientation,
    flags: WrapSizerFlag,
    _marker: PhantomData<()>,
}

impl WrapSizerBuilder {
    fn new(orientation: Orientation) -> Self {
        Self {
            orientation,
            flags: WrapSizerFlag::DefaultFlags,
            _marker: PhantomData,
        }
    }

    /// Set the wrap sizer flags.
    pub fn with_flags(mut self, flags: WrapSizerFlag) -> Self {
        self.flags = flags;
        self
    }

    /// Build the WrapSizer.
    pub fn build(self) -> WrapSizer {
        let ptr = unsafe { ffi::wxd_WrapSizer_Create(self.orientation.bits() as i32, self.flags.bits() as i32) };
        unsafe { WrapSizer::from_ptr(ptr).expect("Failed to create wxWrapSizer") }
    }
}
