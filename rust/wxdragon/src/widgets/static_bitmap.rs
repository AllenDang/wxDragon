//!
//! Safe wrapper for wxStaticBitmap

use crate::bitmap::Bitmap;
use crate::bitmap_bundle::BitmapBundle;
use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{Window, WindowHandle, WxWidget};
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
///
/// StaticBitmap uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct StaticBitmap {
    /// Safe handle to the underlying wxStaticBitmap - automatically invalidated on destroy
    handle: WindowHandle,
}

impl StaticBitmap {
    /// Creates a new StaticBitmap builder.
    pub fn builder<W: WxWidget>(parent: &W) -> StaticBitmapBuilder<'_> {
        StaticBitmapBuilder::new(parent)
    }

    /// Helper to get raw static bitmap pointer, returns null if widget has been destroyed
    #[inline]
    fn widget_ptr(&self) -> *mut ffi::wxd_StaticBitmap_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_StaticBitmap_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Creates a new StaticBitmap with a bitmap.
    pub fn new_with_bitmap(parent: &dyn WxWidget, id: Id, bitmap: &Bitmap) -> Self {
        let name_cstr = CString::new("StaticBitmap").unwrap_or_default();

        unsafe {
            let ptr = ffi::wxd_StaticBitmap_CreateWithBitmap(
                parent.handle_ptr(),
                id as c_int,
                bitmap.as_const_ptr(),
                ffi::wxd_Point { x: -1, y: -1 },         // DEFAULT_POSITION
                ffi::wxd_Size { width: -1, height: -1 }, // DEFAULT_SIZE
                0,                                       // Default style
                name_cstr.as_ptr(),
            );

            if ptr.is_null() {
                panic!("Failed to create StaticBitmap widget");
            }
            StaticBitmap {
                handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
            }
        }
    }

    /// Creates a new StaticBitmap with a bitmap bundle.
    pub fn new_with_bitmap_bundle(parent: &dyn WxWidget, id: Id, bundle: &BitmapBundle) -> Self {
        unsafe {
            let ptr = ffi::wxd_StaticBitmap_CreateWithBitmapBundle(parent.handle_ptr(), id as c_int, bundle.as_ptr());

            if ptr.is_null() {
                panic!("Failed to create StaticBitmap widget with BitmapBundle");
            }
            StaticBitmap {
                handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
            }
        }
    }

    /// Sets or replaces the bitmap shown in the control.
    ///
    /// Accepts a `&Bitmap`. Use `Bitmap::null_bitmap()` to clear the image.
    /// No-op if the widget has been destroyed.
    ///
    /// # Example
    /// ```rust
    /// # use wxdragon::prelude::*;
    /// # fn example(sb: &StaticBitmap, bmp: &Bitmap) {
    /// // Set a normal bitmap
    /// sb.set_bitmap(bmp);
    ///
    /// // Clear using a null/empty bitmap
    /// sb.set_bitmap(&Bitmap::null_bitmap());
    /// # }
    /// ```
    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_StaticBitmap_SetBitmap(ptr, bitmap.as_const_ptr()) };

        // Trigger refresh on parent to update the display
        if let Some(window_ptr) = self.handle.get_ptr() {
            let window = unsafe { Window::from_ptr(window_ptr) };
            if let Some(parent) = window.get_parent() {
                parent.refresh(true, None);
                parent.layout();
            }
        }
    }

    /// Sets or replaces the bitmap bundle shown in the control.
    ///
    /// Using a bitmap bundle allows for better DPI scaling on high-resolution displays.
    /// No-op if the widget has been destroyed.
    pub fn set_bitmap_bundle(&self, bundle: &BitmapBundle) {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_StaticBitmap_SetBitmapBundle(ptr, bundle.as_ptr()) };

        // Trigger refresh on parent to update the display
        if let Some(window_ptr) = self.handle.get_ptr() {
            let window = unsafe { Window::from_ptr(window_ptr) };
            if let Some(parent) = window.get_parent() {
                parent.refresh(true, None);
                parent.layout();
            }
        }
    }

    /// Gets the current bitmap from the control.
    /// Returns a new bitmap instance that the caller owns.
    /// Returns None if the widget has been destroyed.
    pub fn get_bitmap(&self) -> Option<Bitmap> {
        let widget_ptr = self.widget_ptr();
        if widget_ptr.is_null() {
            return None;
        }

        let ptr = unsafe { ffi::wxd_StaticBitmap_GetBitmap(widget_ptr) };

        if ptr.is_null() {
            None
        } else {
            // We get ownership of the bitmap from C++
            Some(Bitmap::from(ptr))
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
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_StaticBitmap_SetScaleMode(ptr, mode.to_raw()) };

        // Trigger refresh on parent to apply the new scale mode
        if let Some(window_ptr) = self.handle.get_ptr() {
            let window = unsafe { Window::from_ptr(window_ptr) };
            if let Some(parent) = window.get_parent() {
                parent.refresh(true, None);
                parent.layout();
            }
        }
    }

    /// Gets the current scale mode of the control.
    ///
    /// Returns the scale mode that determines how the bitmap is scaled within the control.
    /// Returns ScaleMode::None if the widget has been destroyed.
    pub fn get_scale_mode(&self) -> ScaleMode {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return ScaleMode::None;
        }
        let raw_mode = unsafe { ffi::wxd_StaticBitmap_GetScaleMode(ptr) };
        ScaleMode::from_raw(raw_mode)
    }

    /// Returns the underlying WindowHandle for this static bitmap.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
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
                StaticBitmap {
                    handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
                }
            }
        } else if let Some(bmp) = &slf.bitmap {
            unsafe {
                let ptr = ffi::wxd_StaticBitmap_CreateWithBitmap(
                    slf.parent.handle_ptr(),
                    slf.id as c_int,
                    bmp.as_const_ptr(),
                    slf.pos.into(),
                    slf.size.into(),
                    slf.style.bits() as ffi::wxd_Style_t,
                    name_cstr.as_ptr(),
                );

                if ptr.is_null() {
                    panic!("Failed to create StaticBitmap widget");
                }
                StaticBitmap {
                    handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
                }
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

// Manual WxWidget implementation for StaticBitmap (using WindowHandle)
impl WxWidget for StaticBitmap {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for StaticBitmap {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for StaticBitmap {}

// XRC Support - enables StaticBitmap to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for StaticBitmap {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        StaticBitmap {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for StaticBitmap
impl crate::window::FromWindowWithClassName for StaticBitmap {
    fn class_name() -> &'static str {
        "wxStaticBitmap"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        StaticBitmap {
            handle: WindowHandle::new(ptr),
        }
    }
}
