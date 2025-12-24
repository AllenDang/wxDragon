use crate::window::WxWidget; // Needed for add method
// Needed for add method, ensure this is correctly pathed if SizerFlag type alias is in box_sizer.rs or mod.rs
// Assuming SizerFlag type and its constants (ALIGN_LEFT etc.) are available in the crate::sizers scope
use wxdragon_sys as ffi;

// --- Sizer Orientation Constants ---
widget_style_enum!(
    name: Orientation,
    doc: "Orientation flags for sizers.",
    variants: {
        Vertical: ffi::WXD_VERTICAL, "Vertical orientation.",
        Horizontal: ffi::WXD_HORIZONTAL, "Horizontal orientation.",
        Both: ffi::WXD_BOTH, "Both horizontal and vertical orientation."
    },
    default_variant: Vertical
);

// --- Sizer Flag Constants ---
widget_style_enum!(
    name: SizerFlag,
    doc: "Style flags for sizers.",
    variants: {
        Left: ffi::WXD_LEFT, "Left alignment or border.",
        Right: ffi::WXD_RIGHT, "Right alignment or border.",
        Top: ffi::WXD_TOP, "Top alignment or border.",
        Bottom: ffi::WXD_BOTTOM, "Bottom alignment or border.",
        All: ffi::WXD_ALL, "All sides border.",
        Expand: ffi::WXD_EXPAND, "Expand to fill available space.",
        AlignLeft: ffi::WXD_ALIGN_LEFT, "Align to the left.",
        AlignRight: ffi::WXD_ALIGN_RIGHT, "Align to the right.",
        AlignCenterVertical: ffi::WXD_ALIGN_CENTRE_VERTICAL, "Center vertically.",
        AlignCenterHorizontal: ffi::WXD_ALIGN_CENTRE_HORIZONTAL, "Center horizontally.",
        AlignCentre: ffi::WXD_ALIGN_CENTRE, "Center in both directions.",
        Shaped: ffi::WXD_SHAPED, "Shaped sizer behavior.",
        FixedMinsize: ffi::WXD_FIXED_MINSIZE, "Fixed minimum size.",
        ReserveSpaceEvenIfHidden: ffi::WXD_RESERVE_SPACE_EVEN_IF_HIDDEN, "Reserve space even if hidden.",
        Shrink: ffi::WXD_SHRINK, "Shrink to fit contents.",
        Tile: ffi::WXD_TILE, "Tile to fill available space.",
        StretchMask: ffi::WXD_STRETCH_MASK, "Stretch mask to fill available space."
    },
    default_variant: AlignLeft
);

// --- WxSizer Trait ---
// Common trait for all sizer types
pub trait WxSizer {
    // Returns the raw underlying sizer pointer.
    // Unsafe because the lifetime is not tied to self.
    // Primarily for internal use or passing back to FFI.
    // Note: This returns the base wxd_Sizer_t pointer.
    fn as_sizer_ptr(&self) -> *mut ffi::wxd_Sizer_t;

    // Add common sizer methods here? Or keep them on concrete types?
    // Keeping them on concrete types for now seems simpler,
    // as they might need specific logic or return types (&Self).
}

/// Opaque wrapper for a base wxSizer pointer.
#[derive(Clone, Copy)]
pub struct Sizer {
    pub(crate) ptr: *mut ffi::wxd_Sizer_t,
}

impl Sizer {
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Sizer_t) -> Option<Self> {
        if ptr.is_null() { None } else { Some(Sizer { ptr }) }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    // --- Moved methods from BoxSizer to base Sizer ---
    pub fn add<W: WxWidget>(
        &self,
        widget: &W,
        proportion: i32,
        flag: SizerFlag, // Ensure SizerFlag type is accessible
        border: i32,
    ) -> &Self {
        unsafe {
            ffi::wxd_Sizer_AddWindow(
                self.as_sizer_ptr(), // Uses WxSizer trait method
                widget.handle_ptr(),
                proportion,
                flag.bits() as i32,
                border,
            );
        }
        self
    }

    pub fn add_stretch_spacer(&self, proportion: i32) -> &Self {
        unsafe {
            ffi::wxd_Sizer_AddStretchSpacer(self.as_sizer_ptr(), proportion);
        }
        self
    }

    pub fn add_spacer(&self, size: i32) -> &Self {
        if size > 0 {
            unsafe {
                ffi::wxd_Sizer_AddSpacer(self.as_sizer_ptr(), size);
            }
        }
        self
    }

    pub fn add_sizer(
        &self,
        child_sizer: &impl WxSizer, // Use WxSizer trait directly
        proportion: i32,
        flag: SizerFlag, // Ensure SizerFlag type is accessible
        border: i32,
    ) -> &Self {
        let child_ptr = child_sizer.as_sizer_ptr();
        unsafe { ffi::wxd_Sizer_AddSizer(self.as_sizer_ptr(), child_ptr, proportion, flag.bits() as i32, border) };
        self
    }
}

impl WxSizer for Sizer {
    fn as_sizer_ptr(&self) -> *mut ffi::wxd_Sizer_t {
        self.ptr
    }
}
