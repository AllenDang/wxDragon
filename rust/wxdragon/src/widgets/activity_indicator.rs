use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
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

// Opaque pointer type from FFI
pub type RawActivityIndicator = ffi::wxd_ActivityIndicator_t;

/// Represents a `wxActivityIndicator`, an animated control that shows
/// an animation to indicate a long-running process is occurring.
///
/// ActivityIndicator uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let indicator = ActivityIndicator::builder(&frame).build();
///
/// // ActivityIndicator is Copy - no clone needed for closures!
/// indicator.start();
///
/// // After parent destruction, indicator operations are safe no-ops
/// frame.destroy();
/// assert!(!indicator.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct ActivityIndicator {
    /// Safe handle to the underlying wxActivityIndicator - automatically invalidated on destroy
    handle: WindowHandle,
}

impl ActivityIndicator {
    /// Creates a new `ActivityIndicatorBuilder` for constructing an activity indicator.
    pub fn builder(parent: &dyn WxWidget) -> ActivityIndicatorBuilder<'_> {
        ActivityIndicatorBuilder::new(parent)
    }

    /// Helper to get raw activity indicator pointer, returns null if widget has been destroyed
    #[inline]
    fn activity_indicator_ptr(&self) -> *mut RawActivityIndicator {
        self.handle
            .get_ptr()
            .map(|p| p as *mut RawActivityIndicator)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Start the animation.
    /// No-op if the activity indicator has been destroyed.
    pub fn start(&self) {
        let ptr = self.activity_indicator_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ActivityIndicator_Start(ptr) }
    }

    /// Stop the animation.
    /// No-op if the activity indicator has been destroyed.
    pub fn stop(&self) {
        let ptr = self.activity_indicator_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ActivityIndicator_Stop(ptr) }
    }

    /// Check if the animation is currently running.
    /// Returns false if the activity indicator has been destroyed.
    pub fn is_running(&self) -> bool {
        let ptr = self.activity_indicator_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_ActivityIndicator_IsRunning(ptr) }
    }

    /// Creates an ActivityIndicator from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_ActivityIndicator_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut RawActivityIndicator) -> Self {
        assert!(!ptr.is_null());
        ActivityIndicator {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Returns the underlying WindowHandle for this activity indicator.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for ActivityIndicator (using WindowHandle)
impl WxWidget for ActivityIndicator {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for ActivityIndicator {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for ActivityIndicator {}

// Use the widget_builder macro to generate the ActivityIndicatorBuilder implementation
widget_builder!(
    name: ActivityIndicator,
    parent_type: &'a dyn WxWidget,
    style_type: ActivityIndicatorStyle,
    fields: {},
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        unsafe {
            let ctrl_ptr = ffi::wxd_ActivityIndicator_Create(
                parent_ptr,
                slf.id,
                slf.pos.x,
                slf.pos.y,
                slf.size.width,
                slf.size.height,
                slf.style.bits(),
            );
            assert!(!ctrl_ptr.is_null(), "wxd_ActivityIndicator_Create returned null");
            ActivityIndicator::from_ptr(ctrl_ptr)
        }
    }
);

// XRC Support - enables ActivityIndicator to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for ActivityIndicator {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ActivityIndicator {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for ActivityIndicator
impl crate::window::FromWindowWithClassName for ActivityIndicator {
    fn class_name() -> &'static str {
        "wxActivityIndicator"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ActivityIndicator {
            handle: WindowHandle::new(ptr),
        }
    }
}
