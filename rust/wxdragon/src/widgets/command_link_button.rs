use crate::event::WxEvtHandler;
use crate::event::button_events::ButtonEvents;
use crate::prelude::*;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

/// Represents a wxCommandLinkButton.
///
/// CommandLinkButton uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let cmd_btn = CommandLinkButton::builder(&frame)
///     .label("Main Action")
///     .note("Additional details about this action")
///     .build();
///
/// // CommandLinkButton is Copy - no clone needed for closures!
/// cmd_btn.bind_click(move |_| {
///     // Safe: if cmd_btn was destroyed, this is a no-op
///     cmd_btn.set_note("Updated note");
/// });
///
/// // After parent destruction, operations are safe no-ops
/// frame.destroy();
/// assert!(!cmd_btn.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct CommandLinkButton {
    /// Safe handle to the underlying wxCommandLinkButton - automatically invalidated on destroy
    handle: WindowHandle,
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
        let parent_ptr = slf.parent.handle_ptr();
        assert!(!parent_ptr.is_null(), "CommandLinkButton requires a parent");

        let c_main_label = CString::new(&slf.label[..]).expect("CString::new for main_label failed");
        let c_note = CString::new(&slf.note[..]).expect("CString::new for note failed");

        let ptr = unsafe {
            ffi::wxd_CommandLinkButton_Create(
                parent_ptr,
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
        }

        // Create a WindowHandle which automatically registers for destroy events
        CommandLinkButton {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }
);

impl CommandLinkButton {
    /// Creates a new `CommandLinkButtonBuilder` for constructing a command link button.
    pub fn builder(parent: &dyn WxWidget) -> CommandLinkButtonBuilder<'_> {
        CommandLinkButtonBuilder::new(parent)
    }

    /// Helper to get raw command link button pointer, returns null if widget has been destroyed
    #[inline]
    fn cmd_link_button_ptr(&self) -> *mut ffi::wxd_CommandLinkButton_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_CommandLinkButton_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Sets the note displayed on the button.
    /// No-op if the button has been destroyed.
    pub fn set_note(&self, note: &str) {
        let ptr = self.cmd_link_button_ptr();
        if ptr.is_null() {
            return;
        }
        let c_note = CString::new(note).expect("CString::new for note failed");
        unsafe { ffi::wxd_CommandLinkButton_SetNote(ptr, c_note.as_ptr()) };
    }

    /// Sets the button's label (main label).
    /// No-op if the button has been destroyed.
    /// Since CommandLinkButton inherits from Button, we use the Button FFI.
    pub fn set_label(&self, label: &str) {
        let ptr = self.handle.get_ptr().unwrap_or(std::ptr::null_mut());
        if ptr.is_null() {
            return;
        }
        let c_label = CString::new(label).expect("CString::new failed");
        unsafe {
            ffi::wxd_Button_SetLabel(ptr as *mut ffi::wxd_Button_t, c_label.as_ptr());
        }
    }

    /// Gets the button's label (main label).
    /// Returns empty string if the button has been destroyed.
    /// Since CommandLinkButton inherits from Button, we use the Button FFI.
    pub fn get_label(&self) -> String {
        let ptr = self.handle.get_ptr().unwrap_or(std::ptr::null_mut());
        if ptr.is_null() {
            return String::new();
        }
        let button_ptr = ptr as *mut ffi::wxd_Button_t;
        let len = unsafe { ffi::wxd_Button_GetLabel(button_ptr, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_Button_GetLabel(button_ptr, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() }
    }

    /// Returns the underlying WindowHandle for this command link button.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Implement ButtonEvents trait for CommandLinkButton
impl ButtonEvents for CommandLinkButton {}

// Manual WxWidget implementation for CommandLinkButton (using WindowHandle)
impl WxWidget for CommandLinkButton {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for CommandLinkButton {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for CommandLinkButton {}

// XRC Support - enables CommandLinkButton to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for CommandLinkButton {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        CommandLinkButton {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for CommandLinkButton
impl crate::window::FromWindowWithClassName for CommandLinkButton {
    fn class_name() -> &'static str {
        "wxCommandLinkButton"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        CommandLinkButton {
            handle: WindowHandle::new(ptr),
        }
    }
}
