//!
//! Safe wrapper for wxGenericStaticBitmap

use crate::bitmap::Bitmap;
use crate::bitmap_bundle::BitmapBundle;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::widgets::static_bitmap::ScaleMode; // Reuse existing ScaleMode
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// Since there are no specific styles for GenericStaticBitmap, we'll use a thin wrapper around i64
widget_style_enum!(
    name: GenericStaticBitmapStyle,
    doc: "Style flags for the GenericStaticBitmap widget.",
    variants: {
        Default: 0, "Default style with no special behavior."
    },
    default_variant: Default
);

// ScaleMode is imported from static_bitmap module - no need to redefine

/// Represents a wxGenericStaticBitmap widget, used to display a bitmap.
/// This is a platform-independent implementation that properly handles scaling on all platforms.
#[derive(Clone)]
pub struct GenericStaticBitmap {
    window: Window,
}

impl GenericStaticBitmap {
    /// Creates a new GenericStaticBitmap builder.
    pub fn builder<W: WxWidget>(parent: &W) -> GenericStaticBitmapBuilder<'_> {
        GenericStaticBitmapBuilder::new(parent)
    }

    /// Creates a new GenericStaticBitmap with a bitmap.
    pub fn new_with_bitmap(parent: &dyn WxWidget, id: Id, bitmap: &Bitmap) -> Self {
        let name_cstr = CString::new("GenericStaticBitmap").unwrap_or_default();

        unsafe {
            let ptr = ffi::wxd_GenericStaticBitmap_CreateWithBitmap(
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
                panic!("Failed to create GenericStaticBitmap widget");
            }
            Self::from_ptr(ptr)
        }
    }

    /// Creates a new GenericStaticBitmap with a bitmap bundle.
    pub fn new_with_bitmap_bundle(parent: &dyn WxWidget, id: Id, bundle: &BitmapBundle) -> Self {
        unsafe {
            let ptr = ffi::wxd_GenericStaticBitmap_CreateWithBitmapBundle(
                parent.handle_ptr(),
                id as c_int,
                bundle.as_ptr(),
            );

            if ptr.is_null() {
                panic!("Failed to create GenericStaticBitmap widget with BitmapBundle");
            }
            Self::from_ptr(ptr)
        }
    }

    /// Creates a GenericStaticBitmap from a raw wxGenericStaticBitmap pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_GenericStaticBitmap_t` pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_GenericStaticBitmap_t) -> Self {
        GenericStaticBitmap {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Sets or replaces the bitmap shown in the control.
    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        unsafe {
            ffi::wxd_GenericStaticBitmap_SetBitmap(
                self.window.handle_ptr() as *mut ffi::wxd_GenericStaticBitmap_t,
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
            ffi::wxd_GenericStaticBitmap_SetBitmapBundle(
                self.window.handle_ptr() as *mut ffi::wxd_GenericStaticBitmap_t,
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
            let ptr = ffi::wxd_GenericStaticBitmap_GetBitmap(
                self.window.handle_ptr() as *mut ffi::wxd_GenericStaticBitmap_t
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
            ffi::wxd_GenericStaticBitmap_SetScaleMode(
                self.window.handle_ptr() as *mut ffi::wxd_GenericStaticBitmap_t,
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
            let raw_mode = ffi::wxd_GenericStaticBitmap_GetScaleMode(
                self.window.handle_ptr() as *mut ffi::wxd_GenericStaticBitmap_t
            );
            ScaleMode::from_raw(raw_mode)
        }
    }
}

widget_builder!(
    name: GenericStaticBitmap,
    parent_type: &'a dyn WxWidget,
    style_type: GenericStaticBitmapStyle,
    fields: {
        bitmap: Option<Bitmap> = None,
        bitmap_bundle: Option<BitmapBundle> = None,
        scale_mode: Option<ScaleMode> = None,
        name: String = "GenericStaticBitmap".to_string()
    },
    build_impl: |slf| {
        let name_cstr = CString::new(&slf.name[..]).unwrap_or_default();

        // Prioritize BitmapBundle if both are set
        let static_bitmap = if let Some(bundle) = &slf.bitmap_bundle {
            unsafe {
                let ptr = ffi::wxd_GenericStaticBitmap_CreateWithBitmapBundle(
                    slf.parent.handle_ptr(),
                    slf.id as c_int,
                    bundle.as_ptr(),
                );

                if ptr.is_null() {
                    panic!("Failed to create GenericStaticBitmap widget with BitmapBundle");
                }
                GenericStaticBitmap::from_ptr(ptr)
            }
        } else if let Some(bmp) = &slf.bitmap {
            unsafe {
                let ptr = ffi::wxd_GenericStaticBitmap_CreateWithBitmap(
                    slf.parent.handle_ptr(),
                    slf.id as c_int,
                    bmp.as_ptr(),
                    slf.pos.into(),
                    slf.size.into(),
                    slf.style.bits() as ffi::wxd_Style_t,
                    name_cstr.as_ptr(),
                );

                if ptr.is_null() {
                    panic!("Failed to create GenericStaticBitmap widget");
                }
                GenericStaticBitmap::from_ptr(ptr)
            }
        } else {
        panic!("Either bitmap or bitmap_bundle must be set for GenericStaticBitmap");
        };

        // Set scale mode if specified
        if let Some(mode) = slf.scale_mode {
            static_bitmap.set_scale_mode(mode);
        }

        static_bitmap
    }
);

// Use the macro to implement all the standard traits
implement_widget_traits_with_target!(GenericStaticBitmap, window, Window);

// Add XRC Support - enables GenericStaticBitmap to be created from XRC-managed pointers
impl_xrc_support!(GenericStaticBitmap, { window });

// Widget casting support for GenericStaticBitmap
impl_widget_cast!(GenericStaticBitmap, "wxGenericStaticBitmap", { window });
