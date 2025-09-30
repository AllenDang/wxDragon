use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use wxdragon_sys as ffi;

// Define a proper style enum for ActivityIndicator
widget_style_enum!(
    name: ActivityIndicatorStyle,
    doc: "Style flags for ActivityIndicator control.",
    variants: {
        Default: 0, "Default style."
    },
    default_variant: Default
);

/// Represents a `wxActivityIndicator`, an animated control that shows
/// an animation to indicate a long-running process is occurring.
#[derive(Clone)]
pub struct ActivityIndicator {
    window: Window,
}

impl ActivityIndicator {
    /// Creates a new `ActivityIndicatorBuilder` for constructing an activity indicator.
    pub fn builder(parent: &dyn WxWidget) -> ActivityIndicatorBuilder<'_> {
        ActivityIndicatorBuilder::new(parent)
    }

    /// Low-level constructor used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        assert!(!parent_ptr.is_null(), "ActivityIndicator requires a parent");

        let ptr = unsafe {
            ffi::wxd_ActivityIndicator_Create(
                parent_ptr,
                id,
                pos.x,
                pos.y,
                size.width,
                size.height,
                style,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create wxActivityIndicator");
        }

        unsafe {
            let window = Window::from_ptr(ptr as *mut ffi::wxd_Window_t);
            ActivityIndicator { window }
        }
    }

    /// Start the animation.
    pub fn start(&self) {
        unsafe { ffi::wxd_ActivityIndicator_Start(self.window.as_ptr() as *mut _) }
    }

    /// Stop the animation.
    pub fn stop(&self) {
        unsafe { ffi::wxd_ActivityIndicator_Stop(self.window.as_ptr() as *mut _) }
    }

    /// Check if the animation is currently running.
    pub fn is_running(&self) -> bool {
        unsafe { ffi::wxd_ActivityIndicator_IsRunning(self.window.as_ptr() as *mut _) }
    }
}

// Use the widget_builder macro for ActivityIndicator
widget_builder!(
    name: ActivityIndicator,
    parent_type: &'a dyn WxWidget,
    style_type: ActivityIndicatorStyle,
    fields: {},
    build_impl: |slf| {
        ActivityIndicator::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
);

// Apply common trait implementations for this widget
implement_widget_traits_with_target!(ActivityIndicator, window, Window);

// Add XRC Support - enables ActivityIndicator to be created from XRC-managed pointers
impl_xrc_support!(ActivityIndicator, { window });

// Widget casting support for ActivityIndicator
impl_widget_cast!(ActivityIndicator, "wxActivityIndicator", { window });
