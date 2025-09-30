use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
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
#[derive(Clone)]
pub struct AnimationCtrl {
    window: Window,
}

impl AnimationCtrl {
    /// Creates a new `AnimationCtrlBuilder` for constructing an animation control.
    pub fn builder(parent: &dyn WxWidget) -> AnimationCtrlBuilder<'_> {
        AnimationCtrlBuilder::new(parent)
    }

    /// Play the animation from the beginning if it is not disabled.
    pub fn play(&self) -> bool {
        unsafe { ffi::wxd_AnimationCtrl_Play(self.window.as_ptr() as *mut _) }
    }

    /// Stop the animation.
    pub fn stop(&self) {
        unsafe { ffi::wxd_AnimationCtrl_Stop(self.window.as_ptr() as *mut _) }
    }

    /// Returns true if the animation is being played.
    pub fn is_playing(&self) -> bool {
        unsafe { ffi::wxd_AnimationCtrl_IsPlaying(self.window.as_ptr() as *mut _) }
    }

    /// Load an animation from a file.
    pub fn load_file(&self, animation_file: &str) -> bool {
        let c_animation_file =
            CString::new(animation_file).expect("CString::new failed for animation_file");
        unsafe {
            ffi::wxd_AnimationCtrl_LoadFile(
                self.window.as_ptr() as *mut _,
                c_animation_file.as_ptr(),
            )
        }
    }

    /// Load an animation from bytes.
    pub fn load_from_bytes(&self, data: &[u8]) -> bool {
        if data.is_empty() {
            return false;
        }
        unsafe {
            ffi::wxd_AnimationCtrl_LoadFromBytes(
                self.window.as_ptr() as *mut _,
                data.as_ptr(),
                data.len(),
            )
        }
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

        let handle = unsafe {
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

        if handle.is_null() {
            panic!("Failed to create wxAnimationCtrl");
        }

        AnimationCtrl {
            window: unsafe { Window::from_ptr(handle as *mut ffi::wxd_Window_t) },
        }
    }
);

// Implement common widget traits
implement_widget_traits_with_target!(AnimationCtrl, window, Window);

// After implement_widget_traits_with_target

// Add XRC Support - enables AnimationCtrl to be created from XRC-managed pointers
impl_xrc_support!(AnimationCtrl, { window });

// Widget casting support for AnimationCtrl
impl_widget_cast!(AnimationCtrl, "wxAnimationCtrl", { window });
