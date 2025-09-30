use crate::implement_widget_traits_with_target;
use crate::prelude::*;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
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

#[derive(Clone)]
pub struct Gauge {
    window: Window, // Embed the generic Window
}

impl Gauge {
    /// Creates a new `GaugeBuilder` for constructing a gauge.
    pub fn builder(parent: &dyn WxWidget) -> GaugeBuilder<'_> {
        GaugeBuilder::new(parent)
    }

    /// Sets the range (maximum value) of the gauge.
    pub fn set_range(&self, range: i32) {
        unsafe {
            ffi::wxd_Gauge_SetRange(self.window.handle_ptr() as *mut RawGauge, range as c_int)
        }
    }

    /// Sets the current value of the gauge.
    pub fn set_value(&self, value: i32) {
        unsafe {
            ffi::wxd_Gauge_SetValue(self.window.handle_ptr() as *mut RawGauge, value as c_int)
        }
    }

    /// Gets the current value of the gauge.
    pub fn get_value(&self) -> i32 {
        unsafe { ffi::wxd_Gauge_GetValue(self.window.handle_ptr() as *mut RawGauge) as i32 }
    }

    /// Creates a Gauge from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_Gauge_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut RawGauge) -> Self {
        assert!(!ptr.is_null());
        Gauge {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

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

// Apply common trait implementations for Gauge
implement_widget_traits_with_target!(Gauge, window, Window);

// Add XRC Support - enables Gauge to be created from XRC-managed pointers
impl_xrc_support!(Gauge, { window });

// Widget casting support for Gauge
impl_widget_cast!(Gauge, "wxGauge", { window });
