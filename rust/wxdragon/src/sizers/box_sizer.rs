// Rust wrappers for wxWidgets Sizers (moved to sizers/box_sizer.rs)

use crate::sizers::WxSizer as WxSizerTrait;
use crate::sizers::base::Sizer;
use std::marker::PhantomData;
use std::ops::Deref;
use wxdragon_sys as ffi;

use super::base::Orientation;

/// Represents a wxBoxSizer.
#[derive(Clone, Copy)]
pub struct BoxSizer {
    sizer_base: Sizer,
}

// Implement WxSizer for BoxSizer (delegates to Sizer)
impl WxSizerTrait for BoxSizer {
    fn as_sizer_ptr(&self) -> *mut ffi::wxd_Sizer_t {
        self.sizer_base.as_sizer_ptr()
    }
}

// Implement Deref to access base Sizer methods directly
impl Deref for BoxSizer {
    type Target = Sizer;
    fn deref(&self) -> &Self::Target {
        &self.sizer_base
    }
}

impl BoxSizer {
    pub fn builder(orientation: Orientation) -> BoxSizerBuilder {
        BoxSizerBuilder::new(orientation)
    }
}

/// Builder for [`BoxSizer`].
pub struct BoxSizerBuilder {
    orientation: Orientation,
    _marker: PhantomData<()>,
}

impl BoxSizerBuilder {
    fn new(orientation: Orientation) -> Self {
        Self {
            orientation,
            _marker: PhantomData,
        }
    }

    pub fn build(self) -> BoxSizer {
        let ptr = unsafe { ffi::wxd_BoxSizer_Create(self.orientation.bits().try_into().unwrap()) };
        let sizer_base = unsafe { Sizer::from_ptr(ptr).expect("Failed to create base Sizer for BoxSizer") };
        BoxSizer { sizer_base }
    }
}
