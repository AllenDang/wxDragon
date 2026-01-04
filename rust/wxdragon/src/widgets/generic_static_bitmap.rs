//!
//! Safe wrapper for wxGenericStaticBitmap

use crate::bitmap::Bitmap;
use crate::bitmap_bundle::BitmapBundle;
use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::widgets::static_bitmap::ScaleMode; // Reuse existing ScaleMode
use crate::window::{WindowHandle, WxWidget};
// Window is used by new_from_composition for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
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
///
/// GenericStaticBitmap uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let bitmap = Bitmap::new_from_file("image.png");
/// let static_bmp = GenericStaticBitmap::builder(&panel)
///     .bitmap(bitmap)
///     .build();
///
/// // GenericStaticBitmap is Copy - no clone needed for closures!
/// // After parent destruction, operations are safe no-ops
/// panel.destroy();
/// assert!(!static_bmp.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct GenericStaticBitmap {
    /// Safe handle to the underlying wxGenericStaticBitmap - automatically invalidated on destroy
    handle: WindowHandle,
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
                bitmap.as_const_ptr(),
                ffi::wxd_Point { x: -1, y: -1 },         // DEFAULT_POSITION
                ffi::wxd_Size { width: -1, height: -1 }, // DEFAULT_SIZE
                0,                                       // Default style
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
            let ptr = ffi::wxd_GenericStaticBitmap_CreateWithBitmapBundle(parent.handle_ptr(), id as c_int, bundle.as_ptr());

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
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw generic_static_bitmap pointer, returns null if widget has been destroyed
    #[inline]
    fn generic_static_bitmap_ptr(&self) -> *mut ffi::wxd_GenericStaticBitmap_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_GenericStaticBitmap_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Sets or replaces the bitmap shown in the control.
    /// No-op if the widget has been destroyed.
    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        let ptr = self.generic_static_bitmap_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_GenericStaticBitmap_SetBitmap(ptr, bitmap.as_const_ptr()) };

        // Trigger refresh on parent to update the display
        if let Some(parent) = self.get_parent() {
            parent.refresh(true, None);
            parent.layout();
        }
    }

    /// Sets or replaces the bitmap bundle shown in the control.
    ///
    /// Using a bitmap bundle allows for better DPI scaling on high-resolution displays.
    /// No-op if the widget has been destroyed.
    pub fn set_bitmap_bundle(&self, bundle: &BitmapBundle) {
        let ptr = self.generic_static_bitmap_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_GenericStaticBitmap_SetBitmapBundle(ptr, bundle.as_ptr()) };

        // Trigger refresh on parent to update the display
        if let Some(parent) = self.get_parent() {
            parent.refresh(true, None);
            parent.layout();
        }
    }

    /// Gets the current bitmap from the control.
    /// Returns a new bitmap instance that the caller owns.
    /// Returns None if the widget has been destroyed.
    pub fn get_bitmap(&self) -> Option<Bitmap> {
        let ptr = self.generic_static_bitmap_ptr();
        if ptr.is_null() {
            return None;
        }
        unsafe {
            let bmp_ptr = ffi::wxd_GenericStaticBitmap_GetBitmap(ptr);
            if bmp_ptr.is_null() {
                None
            } else {
                Some(Bitmap::from(bmp_ptr))
            }
        }
    }

    /// Sets the scale mode for how the bitmap is displayed within the control.
    ///
    /// This determines how the bitmap is scaled to fit the control's size.
    /// No-op if the widget has been destroyed.
    ///
    /// # Arguments
    /// * `mode` - The scale mode to use
    pub fn set_scale_mode(&self, mode: ScaleMode) {
        let ptr = self.generic_static_bitmap_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_GenericStaticBitmap_SetScaleMode(ptr, mode.to_raw());
        }

        // Trigger refresh on parent to apply the new scale mode
        if let Some(parent) = self.get_parent() {
            parent.refresh(true, None);
            parent.layout();
        }
    }

    /// Gets the current scale mode of the control.
    ///
    /// Returns the scale mode that determines how the bitmap is scaled within the control.
    /// Returns ScaleMode::None if the widget has been destroyed.
    pub fn get_scale_mode(&self) -> ScaleMode {
        let ptr = self.generic_static_bitmap_ptr();
        if ptr.is_null() {
            return ScaleMode::None;
        }

        let raw_mode = unsafe { ffi::wxd_GenericStaticBitmap_GetScaleMode(ptr) };
        ScaleMode::from_raw(raw_mode)
    }

    /// Returns the underlying WindowHandle for this generic static bitmap.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
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
                    bmp.as_const_ptr(),
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

// Manual WxWidget implementation for GenericStaticBitmap (using WindowHandle)
impl WxWidget for GenericStaticBitmap {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Note: We don't implement Deref to Window because returning a reference
// to a temporary Window is unsound. Users can access window methods through
// the WxWidget trait methods directly.

// Implement WxEvtHandler for event binding
impl WxEvtHandler for GenericStaticBitmap {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for GenericStaticBitmap {}

// XRC Support - enables GenericStaticBitmap to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for GenericStaticBitmap {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        GenericStaticBitmap {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for GenericStaticBitmap
impl crate::window::FromWindowWithClassName for GenericStaticBitmap {
    fn class_name() -> &'static str {
        "wxGenericStaticBitmap"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        GenericStaticBitmap {
            handle: WindowHandle::new(ptr),
        }
    }
}
