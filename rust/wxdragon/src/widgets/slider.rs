//! Safe wrapper for wxSlider.

use crate::event::ScrollEvents;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// --- Style enum using macro ---
widget_style_enum!(
    name: SliderStyle,
    doc: "Style flags for Slider",
    variants: {
        Default: ffi::WXD_SL_HORIZONTAL, "Default style (horizontal slider with no labels or ticks).",
        Vertical: ffi::WXD_SL_VERTICAL, "Vertical slider.",
        AutoTicks: ffi::WXD_SL_AUTOTICKS, "Display tick marks.",
        Labels: ffi::WXD_SL_LABELS, "Display labels (min, max, and current value).",
        MinMaxLabels: ffi::WXD_SL_MIN_MAX_LABELS, "Display min and max labels only.",
        ValueLabel: ffi::WXD_SL_VALUE_LABEL, "Display the current value as a label.",
        BothSides: ffi::WXD_SL_BOTH, "Show ticks on both sides of the slider (not always supported or visually distinct)."
    },
    default_variant: Default
);

/// Represents a wxSlider widget.
#[derive(Clone)]
pub struct Slider {
    window: Window,
}

impl Slider {
    /// Creates a new Slider builder.
    pub fn builder(parent: &dyn WxWidget) -> SliderBuilder<'_> {
        SliderBuilder::new(parent)
    }

    // Internal constructor
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Slider_t) -> Self {
        Slider {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
        }
    }

    /// Returns the raw underlying slider pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_Slider_t {
        self.window.handle_ptr() as *mut ffi::wxd_Slider_t
    }

    // --- Methods specific to Slider ---

    /// Gets the current slider value.
    pub fn value(&self) -> i32 {
        unsafe { ffi::wxd_Slider_GetValue(self.as_ptr()) }
    }

    /// Sets the slider value.
    pub fn set_value(&self, value: i32) {
        unsafe { ffi::wxd_Slider_SetValue(self.as_ptr(), value) }
    }

    /// Sets the slider range (minimum and maximum values).
    pub fn set_range(&self, min_value: i32, max_value: i32) {
        unsafe { ffi::wxd_Slider_SetRange(self.as_ptr(), min_value as c_int, max_value as c_int) };
    }

    /// Gets the minimum slider value.
    pub fn min(&self) -> i32 {
        unsafe { ffi::wxd_Slider_GetMin(self.as_ptr()) }
    }

    /// Gets the maximum slider value.
    pub fn max(&self) -> i32 {
        unsafe { ffi::wxd_Slider_GetMax(self.as_ptr()) }
    }

    pub fn get_value(&self) -> i32 {
        unsafe { ffi::wxd_Slider_GetValue(self.as_ptr()) }
    }
}

// Apply common trait implementations
implement_widget_traits_with_target!(Slider, window, Window);

// Use the widget_builder macro to generate the SliderBuilder implementation
widget_builder!(
    name: Slider,
    parent_type: &'a dyn WxWidget,
    style_type: SliderStyle,
    fields: {
        value: i32 = 0,
        min_value: i32 = 0,
        max_value: i32 = 100
    },
    build_impl: |slf| {
        let slider_ptr = unsafe {
            ffi::wxd_Slider_Create(
                slf.parent.handle_ptr(),
                slf.id,
                slf.value as c_int,
                slf.min_value as c_int,
                slf.max_value as c_int,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            )
        };

        if slider_ptr.is_null() {
            panic!("Failed to create Slider");
        }

        unsafe { Slider::from_ptr(slider_ptr) }
    }
);

// At the bottom of the file, add the ScrollEvents trait implementation
impl ScrollEvents for Slider {}

// Add XRC Support - enables Slider to be created from XRC-managed pointers
impl_xrc_support!(Slider, { window });

// Widget casting support for Slider
impl_widget_cast!(Slider, "wxSlider", { window });
