use std::ffi::CString;
use std::ops::Deref;

use crate::StaticBox;
use crate::sizers::WxSizer as WxSizerTrait;
use crate::sizers::base::{Orientation, Sizer};
use crate::window::WxWidget;
use wxdragon_sys as ffi;

/// Represents the wxStaticBoxSizer.
#[derive(Clone, Copy)]
pub struct StaticBoxSizer {
    raw_specific_ptr: *mut ffi::wxd_StaticBoxSizer_t, // Pointer to the specific wxStaticBoxSizer
    sizer_base: Sizer,                                // Base sizer functionality
}

impl StaticBoxSizer {
    /// Creates a new StaticBoxSizer wrapper from a raw wxStaticBoxSizer pointer.
    /// Unsafe because the caller must ensure the pointer is valid.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StaticBoxSizer_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            // The pointer for the base Sizer is the same as the specific StaticBoxSizer pointer.
            let base_ptr = ptr as *mut ffi::wxd_Sizer_t;
            unsafe {
                Sizer::from_ptr(base_ptr).map(|sizer_base| StaticBoxSizer {
                    raw_specific_ptr: ptr,
                    sizer_base,
                })
            }
        }
    }

    /// Returns the wxStaticBox associated with this sizer, if any.
    pub fn get_static_box(&self) -> Option<StaticBox> {
        unsafe {
            let box_ptr = ffi::wxd_StaticBoxSizer_GetStaticBox(self.raw_specific_ptr);
            if box_ptr.is_null() {
                None
            } else {
                Some(StaticBox::from_ptr(box_ptr))
            }
        }
    }
}

// Implement the WxSizer trait to provide base sizer functionality
impl WxSizerTrait for StaticBoxSizer {
    fn as_sizer_ptr(&self) -> *mut ffi::wxd_Sizer_t {
        self.sizer_base.as_sizer_ptr()
    }
}

// Allow dereferencing to the base Sizer struct to access its methods
impl Deref for StaticBoxSizer {
    type Target = Sizer;
    fn deref(&self) -> &Self::Target {
        &self.sizer_base
    }
}

/// Builder for [`StaticBoxSizer`].
pub struct StaticBoxSizerBuilder {
    parent_window_ptr: Option<*mut ffi::wxd_Window_t>, // For creating StaticBox internally
    orientation: Orientation,
    source: StaticBoxSource, // Mandatory: either an existing box or info to create one
}

/// Defines the source for the StaticBox in the StaticBoxSizer.
enum StaticBoxSource {
    /// Use an existing wxStaticBox.
    Box(*mut ffi::wxd_StaticBox_t),
    /// Create a new wxStaticBox with a label.
    Label(String),
}

impl StaticBoxSizerBuilder {
    /// Creates a builder that uses an existing `StaticBox`.
    pub fn new_with_box(static_box: &StaticBox, orientation: Orientation) -> Self {
        StaticBoxSizerBuilder {
            parent_window_ptr: None,
            orientation,
            source: StaticBoxSource::Box(static_box.handle_ptr() as *mut ffi::wxd_StaticBox_t),
        }
    }

    /// Creates a builder that will create a new `StaticBox` internally.
    /// The `parent` is the window where the new `StaticBox` will be created.
    pub fn new_with_label<W: WxWidget>(orientation: Orientation, parent: &W, label: &str) -> Self {
        StaticBoxSizerBuilder {
            parent_window_ptr: Some(parent.handle_ptr()),
            orientation,
            source: StaticBoxSource::Label(label.to_string()),
        }
    }

    /// Builds the `StaticBoxSizer`.
    ///
    /// # Panics
    /// Panics if the underlying FFI call fails to create the sizer.
    pub fn build(self) -> StaticBoxSizer {
        let orient = self.orientation.bits() as i32;
        let s_ptr = match self.source {
            StaticBoxSource::Box(box_ptr) => unsafe { ffi::wxd_StaticBoxSizer_Create_WithBox(box_ptr, orient) },
            StaticBoxSource::Label(label) => {
                let parent_ptr = self
                    .parent_window_ptr
                    .expect("Parent window pointer must be Some when creating StaticBoxSizer with a label");
                let label_cstring = CString::new(label).unwrap_or_else(|_| CString::new("").unwrap());
                unsafe { ffi::wxd_StaticBoxSizer_Create_WithLabel(orient, parent_ptr, label_cstring.as_ptr()) }
            }
        };
        unsafe { StaticBoxSizer::from_ptr(s_ptr).expect("Failed to create wxStaticBoxSizer from pointer") }
    }
}
