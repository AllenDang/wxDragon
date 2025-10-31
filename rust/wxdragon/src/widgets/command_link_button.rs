use crate::prelude::*;
use crate::widgets::button::Button; // Compose Button
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use wxdragon_sys as ffi;

/// Represents a wxCommandLinkButton.
#[derive(Clone)]
pub struct CommandLinkButton {
    button: Button, // Composition: CommandLinkButton IS a Button
}

// Create a style enum for CommandLinkButton
widget_style_enum!(
    name: CommandLinkButtonStyle,
    doc: "Style flags for CommandLinkButton.",
    variants: {
        Default: 0, "Default style with no special behavior."
    },
    default_variant: Default
);

widget_builder!(
    name: CommandLinkButton,
    parent_type: &'a dyn WxWidget,
    style_type: CommandLinkButtonStyle,
    fields: {
        label: String = String::new(),
        note: String = String::new()
    },
    build_impl: |slf| {
        assert!(!slf.parent.handle_ptr().is_null(), "CommandLinkButton requires a parent");

        let c_main_label = CString::new(&slf.label[..]).expect("CString::new for main_label failed");
        let c_note = CString::new(&slf.note[..]).expect("CString::new for note failed");

        let ptr = unsafe {
            ffi::wxd_CommandLinkButton_Create(
                slf.parent.handle_ptr(),
                slf.id,
                c_main_label.as_ptr(),
                c_note.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits(),
            )
        };

        if ptr.is_null() {
            panic!("Failed to create CommandLinkButton widget");
        } else {
            let window = unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) }; // Cast to Window pointer and wrap
            let button = Button::new_from_composition(window, slf.parent.handle_ptr());
            CommandLinkButton { button }
        }
    }
);

impl CommandLinkButton {
    /// Creates a new `CommandLinkButtonBuilder` for constructing a command link button.
    pub fn builder(parent: &dyn WxWidget) -> CommandLinkButtonBuilder<'_> {
        CommandLinkButtonBuilder::new(parent)
    }

    /// Sets the note displayed on the button.
    pub fn set_note(&self, note: &str) {
        let c_note = CString::new(note).expect("CString::new for note failed");
        let ptr = self.button.handle_ptr() as *mut ffi::wxd_CommandLinkButton_t;
        unsafe { ffi::wxd_CommandLinkButton_SetNote(ptr, c_note.as_ptr()) };
    }

    // Getters for main label and note could be added if wxCommandLinkButton provides them
    // or if we decide to store them in the Rust struct. For now, main label uses Button::get_label().
}

// Use the implement_widget_traits_with_target! macro to implement standard traits
// with Button as the target type for Deref
implement_widget_traits_with_target!(CommandLinkButton, button, Button);

// Manual XRC Support for CommandLinkButton - composition structure needs custom handling
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for CommandLinkButton {
    unsafe fn from_xrc_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
        let window = unsafe { Window::from_ptr(ptr) };
        let button = Button::new_from_composition(window, std::ptr::null_mut());
        CommandLinkButton { button }
    }
}

// Manual widget casting support for CommandLinkButton - composition structure needs custom handling
impl crate::window::FromWindowWithClassName for CommandLinkButton {
    fn class_name() -> &'static str {
        "wxCommandLinkButton"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        let window = unsafe { Window::from_ptr(ptr) };
        let button = Button::new_from_composition(window, std::ptr::null_mut());
        CommandLinkButton { button }
    }
}
