use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by new_from_composition for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::CString;
use wxdragon_sys as ffi;

// Define a standard style enum for AnimationCtrl
widget_style_enum!(
    name: AnimationCtrlStyle,
    doc: "Style flags for AnimationCtrl widget.",
    variants: {
        Default: 0, "Default style."
    },
    default_variant: Default
);

/// Represents a `wxAnimationCtrl` control, which displays an animation.
///
/// AnimationCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct AnimationCtrl {
    /// Safe handle to the underlying wxAnimationCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl AnimationCtrl {
    /// Creates a new `AnimationCtrlBuilder` for constructing an animation control.
    pub fn builder(parent: &dyn WxWidget) -> AnimationCtrlBuilder<'_> {
        AnimationCtrlBuilder::new(parent)
    }

    /// Creates a new AnimationCtrl from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Creates a new AnimationCtrl from a raw window pointer.
    /// This is for backwards compatibility with widgets that compose AnimationCtrl.
    /// The parent_ptr parameter is ignored (kept for API compatibility).
    #[allow(dead_code)]
    pub(crate) fn new_from_composition(_window: Window, _parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        // Use the window's pointer to create a new WindowHandle
        Self {
            handle: WindowHandle::new(_window.as_ptr()),
        }
    }

    /// Helper to get raw animation control pointer, returns null if widget has been destroyed
    #[inline]
    fn animation_ctrl_ptr(&self) -> *mut ffi::wxd_AnimationCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_AnimationCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Play the animation from the beginning if it is not disabled.
    /// Returns false if the animation control has been destroyed.
    pub fn play(&self) -> bool {
        let ptr = self.animation_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_AnimationCtrl_Play(ptr) }
    }

    /// Stop the animation.
    /// No-op if the animation control has been destroyed.
    pub fn stop(&self) {
        let ptr = self.animation_ctrl_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_AnimationCtrl_Stop(ptr) }
    }

    /// Returns true if the animation is being played.
    /// Returns false if the animation control has been destroyed.
    pub fn is_playing(&self) -> bool {
        let ptr = self.animation_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_AnimationCtrl_IsPlaying(ptr) }
    }

    /// Load an animation from a file.
    /// Returns false if the animation control has been destroyed.
    pub fn load_file(&self, animation_file: &str) -> bool {
        let ptr = self.animation_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_animation_file = CString::new(animation_file).expect("CString::new failed for animation_file");
        unsafe { ffi::wxd_AnimationCtrl_LoadFile(ptr, c_animation_file.as_ptr()) }
    }

    /// Load an animation from bytes.
    /// Returns false if the animation control has been destroyed or if data is empty.
    pub fn load_from_bytes(&self, data: &[u8]) -> bool {
        if data.is_empty() {
            return false;
        }
        let ptr = self.animation_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_AnimationCtrl_LoadFromBytes(ptr, data.as_ptr(), data.len()) }
    }

    /// Returns the underlying WindowHandle for this animation control.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Use the widget_builder macro for AnimationCtrl
widget_builder!(
    name: AnimationCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: AnimationCtrlStyle,
    fields: {
        animation_file: String = String::new(),
        name: String = "AnimationCtrl".to_string()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let c_animation_file = CString::new(slf.animation_file.as_str())
            .expect("CString::new failed for animation_file");
        let c_name = CString::new(slf.name.as_str())
            .expect("CString::new failed for name");

        let ptr = unsafe {
            ffi::wxd_AnimationCtrl_Create(
                parent_ptr,
                slf.id,
                c_animation_file.as_ptr(),
                slf.pos.x,
                slf.pos.y,
                slf.size.width,
                slf.size.height,
                slf.style.bits(),
                c_name.as_ptr()
            )
        };

        if ptr.is_null() {
            panic!("Failed to create wxAnimationCtrl");
        }

        // Create a WindowHandle which automatically registers for destroy events
        AnimationCtrl {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// Manual WxWidget implementation for AnimationCtrl (using WindowHandle)
impl WxWidget for AnimationCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for AnimationCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for AnimationCtrl {}

// XRC Support - enables AnimationCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for AnimationCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        AnimationCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for AnimationCtrl
impl crate::window::FromWindowWithClassName for AnimationCtrl {
    fn class_name() -> &'static str {
        "wxAnimationCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        AnimationCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
