//! Safe wrapper for wxSlider.

use crate::CommandEventData;
use crate::EventType;
use crate::event::{Event, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_local_event_handlers;
use crate::window::{WindowHandle, WxWidget};
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
///
/// Slider uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let slider = Slider::builder(&frame).min_value(0).max_value(100).build();
///
/// // Slider is Copy - no clone needed for closures!
/// slider.bind_slider(move |_| {
///     // Safe: if slider was destroyed, this is a no-op
///     let value = slider.value();
/// });
///
/// // After parent destruction, slider operations are safe no-ops
/// frame.destroy();
/// assert!(!slider.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct Slider {
    /// Safe handle to the underlying wxSlider - automatically invalidated on destroy
    handle: WindowHandle,
}

impl Slider {
    /// Creates a new Slider builder.
    pub fn builder(parent: &dyn WxWidget) -> SliderBuilder<'_> {
        SliderBuilder::new(parent)
    }

    // Internal constructor
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Slider_t) -> Self {
        Slider {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw slider pointer, returns null if widget has been destroyed
    #[inline]
    fn slider_ptr(&self) -> *mut ffi::wxd_Slider_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_Slider_t)
            .unwrap_or(std::ptr::null_mut())
    }

    // --- Methods specific to Slider ---

    /// Gets the current slider value.
    /// Returns 0 if the slider has been destroyed.
    pub fn value(&self) -> i32 {
        let ptr = self.slider_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Slider_GetValue(ptr) }
    }

    /// Sets the slider value.
    /// No-op if the slider has been destroyed.
    pub fn set_value(&self, value: i32) {
        let ptr = self.slider_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Slider_SetValue(ptr, value) }
    }

    /// Sets the slider range (minimum and maximum values).
    /// No-op if the slider has been destroyed.
    pub fn set_range(&self, min_value: i32, max_value: i32) {
        let ptr = self.slider_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Slider_SetRange(ptr, min_value as c_int, max_value as c_int) };
    }

    /// Gets the minimum slider value.
    /// Returns 0 if the slider has been destroyed.
    pub fn min(&self) -> i32 {
        let ptr = self.slider_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Slider_GetMin(ptr) }
    }

    /// Gets the maximum slider value.
    /// Returns 0 if the slider has been destroyed.
    pub fn max(&self) -> i32 {
        let ptr = self.slider_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Slider_GetMax(ptr) }
    }

    /// Gets the current slider value.
    /// Returns 0 if the slider has been destroyed.
    pub fn get_value(&self) -> i32 {
        let ptr = self.slider_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Slider_GetValue(ptr) }
    }

    /// Returns the underlying WindowHandle for this slider.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for Slider (using WindowHandle)
impl WxWidget for Slider {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for Slider {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for Slider {}

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

// XRC Support - enables Slider to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for Slider {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Slider {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for Slider
impl crate::window::FromWindowWithClassName for Slider {
    fn class_name() -> &'static str {
        "wxSlider"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Slider {
            handle: WindowHandle::new(ptr),
        }
    }
}

/// Events that can be emitted by a [`Slider`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderEventType {
    /// The slider value has changed.
    /// Corresponds to `EventType::SLIDER` (`wxEVT_SLIDER`).
    Slider,
}

/// Event data for a `SpinCtrl::ValueChanged` event.
#[derive(Debug)]
pub struct SliderEvent {
    /// The base command event data.
    pub base: CommandEventData,
}

impl SliderEvent {
    /// Creates a new [`SliderEvent`] from a generic [`Event`].
    pub fn new(event: Event) -> Self {
        Self {
            base: CommandEventData::new(event),
        }
    }

    /// Gets the current value of the Slider from the event.
    pub fn get_value(&self) -> i32 {
        self.base.get_int().unwrap_or(0)
    }
}

implement_widget_local_event_handlers!(Slider, SliderEventType, SliderEvent,
    Slider => slider, EventType::SLIDER
);
