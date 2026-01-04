use crate::event::WxEvtHandler;
use crate::prelude::*;
use crate::window::{WindowHandle, WxWidget};
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// --- Style enum using macro ---
widget_style_enum!(
    name: GaugeStyle,
    doc: "Style flags for Gauge.",
    variants: {
        Default: ffi::WXD_GA_HORIZONTAL, "Default style (horizontal bar).",
        Vertical: ffi::WXD_GA_VERTICAL, "Vertical gauge.",
        Smooth: ffi::WXD_GA_SMOOTH, "Use smooth progress indication (typically native look and feel determines this).",
        ShowProgress: ffi::WXD_GA_PROGRESS, "Show textual progress (e.g., \"50%\"). On some platforms, this might be the default or combined with non-smooth."
    },
    default_variant: Default
);

// Opaque pointer type from FFI
pub type RawGauge = ffi::wxd_Gauge_t;

/// Represents a wxGauge widget.
///
/// Gauge uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let gauge = Gauge::builder(&frame).range(100).build();
///
/// // Gauge is Copy - no clone needed for closures!
/// gauge.set_value(50);
///
/// // After parent destruction, gauge operations are safe no-ops
/// frame.destroy();
/// assert!(!gauge.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct Gauge {
    /// Safe handle to the underlying wxGauge - automatically invalidated on destroy
    handle: WindowHandle,
}

impl Gauge {
    /// Creates a new `GaugeBuilder` for constructing a gauge.
    pub fn builder(parent: &dyn WxWidget) -> GaugeBuilder<'_> {
        GaugeBuilder::new(parent)
    }

    /// Helper to get raw gauge pointer, returns null if widget has been destroyed
    #[inline]
    fn gauge_ptr(&self) -> *mut RawGauge {
        self.handle
            .get_ptr()
            .map(|p| p as *mut RawGauge)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Sets the range (maximum value) of the gauge.
    /// No-op if the gauge has been destroyed.
    pub fn set_range(&self, range: i32) {
        let ptr = self.gauge_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Gauge_SetRange(ptr, range as c_int) }
    }

    /// Sets the current value of the gauge.
    /// No-op if the gauge has been destroyed.
    pub fn set_value(&self, value: i32) {
        let ptr = self.gauge_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Gauge_SetValue(ptr, value as c_int) }
    }

    /// Gets the current value of the gauge.
    /// Returns 0 if the gauge has been destroyed.
    pub fn get_value(&self) -> i32 {
        let ptr = self.gauge_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Gauge_GetValue(ptr) as i32 }
    }

    /// Creates a Gauge from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_Gauge_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut RawGauge) -> Self {
        assert!(!ptr.is_null());
        Gauge {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the underlying WindowHandle for this gauge.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for Gauge (using WindowHandle)
impl WxWidget for Gauge {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for Gauge {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for Gauge {}

// Use the widget_builder macro to generate the GaugeBuilder implementation
widget_builder!(
    name: Gauge,
    parent_type: &'a dyn WxWidget,
    style_type: GaugeStyle,
    fields: {
        range: i32 = 100
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        unsafe {
            let ctrl_ptr = ffi::wxd_Gauge_Create(
                parent_ptr,
                slf.id,
                slf.range as c_int,
                slf.pos.x,
                slf.pos.y,
                slf.size.width,
                slf.size.height,
                slf.style.bits() as ffi::wxd_Style_t,
            );
            assert!(!ctrl_ptr.is_null(), "wxd_Gauge_Create returned null");
            Gauge::from_ptr(ctrl_ptr)
        }
    }
);

// XRC Support - enables Gauge to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for Gauge {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Gauge {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for Gauge
impl crate::window::FromWindowWithClassName for Gauge {
    fn class_name() -> &'static str {
        "wxGauge"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Gauge {
            handle: WindowHandle::new(ptr),
        }
    }
}
