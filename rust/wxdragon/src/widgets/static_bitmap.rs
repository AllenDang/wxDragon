//!
//! Safe wrapper for wxStaticBitmap

use crate::bitmap::Bitmap;
use crate::bitmap_bundle::BitmapBundle;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// Since there are no specific styles for StaticBitmap, we'll use a thin wrapper around i64
widget_style_enum!(
    name: StaticBitmapStyle,
    doc: "Style flags for the StaticBitmap widget.",
    variants: {
        Default: 0, "Default style with no special behavior."
    },
    default_variant: Default
);

/// Scale modes for how the bitmap is scaled within the StaticBitmap control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ScaleMode {
    /// No scaling - display the bitmap at its original size.
    None = ffi::WXD_StaticBitmap_Scale_None as i32,
    /// Scale the bitmap to fill the entire control, potentially changing aspect ratio.
    Fill = ffi::WXD_StaticBitmap_Scale_Fill as i32,
    /// Scale the bitmap to fit within the control while maintaining aspect ratio.
    AspectFit = ffi::WXD_StaticBitmap_Scale_AspectFit as i32,
    /// Scale the bitmap to fill the control while maintaining aspect ratio (may crop).
    AspectFill = ffi::WXD_StaticBitmap_Scale_AspectFill as i32,
}

impl ScaleMode {
    /// Convert from raw integer value to ScaleMode enum.
    pub fn from_raw(value: i32) -> Self {
        match value {
            x if x == ffi::WXD_StaticBitmap_Scale_None as i32 => ScaleMode::None,
            x if x == ffi::WXD_StaticBitmap_Scale_Fill as i32 => ScaleMode::Fill,
            x if x == ffi::WXD_StaticBitmap_Scale_AspectFit as i32 => ScaleMode::AspectFit,
            x if x == ffi::WXD_StaticBitmap_Scale_AspectFill as i32 => ScaleMode::AspectFill,
            _ => ScaleMode::None, // Default to None for unknown values
        }
    }

    /// Convert ScaleMode enum to raw integer value.
    pub fn to_raw(self) -> i32 {
        self as i32
    }
}

/// Represents a wxStaticBitmap widget, used to display a bitmap.
#[derive(Clone)]
pub struct StaticBitmap {
    window: Window,
}

impl StaticBitmap {
    /// Creates a new StaticBitmap builder.
    pub fn builder<W: WxWidget>(parent: &W) -> StaticBitmapBuilder<'_> {
        StaticBitmapBuilder::new(parent)
    }

    /// Creates a new StaticBitmap with a bitmap.
    pub fn new_with_bitmap(parent: &dyn WxWidget, id: Id, bitmap: &Bitmap) -> Self {
        let name_cstr = CString::new("StaticBitmap").unwrap_or_default();

        unsafe {
            let ptr = ffi::wxd_StaticBitmap_CreateWithBitmap(
                parent.handle_ptr(),
                id as c_int,
                bitmap.as_ptr(),
                ffi::wxd_Point { x: -1, y: -1 }, // DEFAULT_POSITION
                ffi::wxd_Size {
                    width: -1,
                    height: -1,
                }, // DEFAULT_SIZE
                0,                               // Default style
                name_cstr.as_ptr(),
            );

            if ptr.is_null() {
                panic!("Failed to create StaticBitmap widget");
            }
            Self::from_ptr(ptr)
        }
    }

    /// Creates a new StaticBitmap with a bitmap bundle.
    pub fn new_with_bitmap_bundle(parent: &dyn WxWidget, id: Id, bundle: &BitmapBundle) -> Self {
        unsafe {
            let ptr = ffi::wxd_StaticBitmap_CreateWithBitmapBundle(
                parent.handle_ptr(),
                id as c_int,
                bundle.as_ptr(),
            );

            if ptr.is_null() {
                panic!("Failed to create StaticBitmap widget with BitmapBundle");
            }
            Self::from_ptr(ptr)
        }
    }

    /// Creates a StaticBitmap from a raw wxStaticBitmap pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_StaticBitmap_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_StaticBitmap_t) -> Self {
        StaticBitmap {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Sets or replaces the bitmap shown in the control.
    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        unsafe {
            ffi::wxd_StaticBitmap_SetBitmap(
                self.window.handle_ptr() as *mut ffi::wxd_StaticBitmap_t,
                bitmap.as_ptr(),
            );
        }

        // Trigger refresh on parent to update the display
        if let Some(parent) = self.window.get_parent() {
            parent.refresh(true, None);
            parent.layout();
        }
    }

    /// Sets or replaces the bitmap bundle shown in the control.
    ///
    /// Using a bitmap bundle allows for better DPI scaling on high-resolution displays.
    pub fn set_bitmap_bundle(&self, bundle: &BitmapBundle) {
        unsafe {
            ffi::wxd_StaticBitmap_SetBitmapBundle(
                self.window.handle_ptr() as *mut ffi::wxd_StaticBitmap_t,
                bundle.as_ptr(),
            );
        }

        // Trigger refresh on parent to update the display
        if let Some(parent) = self.window.get_parent() {
            parent.refresh(true, None);
            parent.layout();
        }
    }

    /// Gets the current bitmap from the control.
    /// Returns a new bitmap instance that the caller owns.
    pub fn get_bitmap(&self) -> Option<Bitmap> {
        unsafe {
            let ptr = ffi::wxd_StaticBitmap_GetBitmap(
                self.window.handle_ptr() as *mut ffi::wxd_StaticBitmap_t
            );

            if ptr.is_null() {
                None
            } else {
                // We get ownership of the bitmap from C++
                Some(Bitmap::from_ptr_owned(ptr))
            }
        }
    }

    /// Sets the scale mode for how the bitmap is displayed within the control.
    ///
    /// This determines how the bitmap is scaled to fit the control's size.
    ///
    /// # Arguments
    /// * `mode` - The scale mode to use
    pub fn set_scale_mode(&self, mode: ScaleMode) {
        unsafe {
            ffi::wxd_StaticBitmap_SetScaleMode(
                self.window.handle_ptr() as *mut ffi::wxd_StaticBitmap_t,
                mode.to_raw(),
            );
        }

        // Trigger refresh on parent to apply the new scale mode
        if let Some(parent) = self.window.get_parent() {
            parent.refresh(true, None);
            parent.layout();
        }
    }

    /// Gets the current scale mode of the control.
    ///
    /// Returns the scale mode that determines how the bitmap is scaled within the control.
    pub fn get_scale_mode(&self) -> ScaleMode {
        unsafe {
            let raw_mode = ffi::wxd_StaticBitmap_GetScaleMode(
                self.window.handle_ptr() as *mut ffi::wxd_StaticBitmap_t
            );
            ScaleMode::from_raw(raw_mode)
        }
    }
}

widget_builder!(
    name: StaticBitmap,
    parent_type: &'a dyn WxWidget,
    style_type: StaticBitmapStyle,
    fields: {
        bitmap: Option<Bitmap> = None,
        bitmap_bundle: Option<BitmapBundle> = None,
        scale_mode: Option<ScaleMode> = None,
        name: String = "StaticBitmap".to_string()
    },
    build_impl: |slf| {
        let name_cstr = CString::new(&slf.name[..]).unwrap_or_default();

        // Prioritize BitmapBundle if both are set
        let static_bitmap = if let Some(bundle) = &slf.bitmap_bundle {
            unsafe {
                let ptr = ffi::wxd_StaticBitmap_CreateWithBitmapBundle(
                    slf.parent.handle_ptr(),
                    slf.id as c_int,
                    bundle.as_ptr(),
                );

                if ptr.is_null() {
                    panic!("Failed to create StaticBitmap widget with BitmapBundle");
                }
                StaticBitmap::from_ptr(ptr)
            }
        } else if let Some(bmp) = &slf.bitmap {
            unsafe {
                let ptr = ffi::wxd_StaticBitmap_CreateWithBitmap(
                    slf.parent.handle_ptr(),
                    slf.id as c_int,
                    bmp.as_ptr(),
                    slf.pos.into(),
                    slf.size.into(),
                    slf.style.bits() as ffi::wxd_Style_t,
                    name_cstr.as_ptr(),
                );

                if ptr.is_null() {
                    panic!("Failed to create StaticBitmap widget");
                }
                StaticBitmap::from_ptr(ptr)
            }
        } else {
        panic!("Either bitmap or bitmap_bundle must be set for StaticBitmap");
        };

        // Set scale mode if specified
        if let Some(mode) = slf.scale_mode {
            static_bitmap.set_scale_mode(mode);
        }

        static_bitmap
    }
);

// Use the macro to implement all the standard traits
implement_widget_traits_with_target!(StaticBitmap, window, Window);

// Add XRC Support - enables StaticBitmap to be created from XRC-managed pointers
impl_xrc_support!(StaticBitmap, { window });

// Widget casting support for StaticBitmap
impl_widget_cast!(StaticBitmap, "wxStaticBitmap", { window });
