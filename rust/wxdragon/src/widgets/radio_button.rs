//! Safe wrapper for wxRadioButton.

use crate::event::event_data::CommandEventData;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::CString;
use wxdragon_sys as ffi;

// --- Style enum using macro ---
widget_style_enum!(
    name: RadioButtonStyle,
    doc: "Style flags for RadioButton",
    variants: {
        Default: 0, "Default style. Represents a standalone radio button or a subsequent button in a group.",
        GroupStart: ffi::WXD_RB_GROUP, "Marks this radio button as the first in a new group. Subsequent radio buttons (until the next `GroupStart` or end of dialog) belong to the same group, where only one can be selected."
    },
    default_variant: Default
);

/// Represents a wxRadioButton control.
///
/// RadioButton uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct RadioButton {
    handle: WindowHandle,
}

impl RadioButton {
    /// Creates a new `RadioButtonBuilder`.
    pub fn builder(parent: &dyn WxWidget) -> RadioButtonBuilder<'_> {
        RadioButtonBuilder::new(parent)
    }

    /// Helper to get raw radio button pointer, returns null if widget has been destroyed
    #[inline]
    fn radiobutton_ptr(&self) -> *mut ffi::wxd_RadioButton_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_RadioButton_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Private constructor from raw pointer
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_RadioButton_t) -> Self {
        assert!(!ptr.is_null());
        RadioButton {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Gets the state of the radio button.
    /// Returns `false` if the radio button has been destroyed.
    pub fn get_value(&self) -> bool {
        let ptr = self.radiobutton_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_RadioButton_GetValue(ptr) }
    }

    /// Sets the state of the radio button.
    /// Note: Setting a radio button to `true` will implicitly set others in the same group to `false`.
    /// No-op if the radio button has been destroyed.
    pub fn set_value(&self, value: bool) {
        let ptr = self.radiobutton_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_RadioButton_SetValue(ptr, value) };
    }
}

// Manual WxWidget implementation for RadioButton (using WindowHandle)
impl WxWidget for RadioButton {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for RadioButton {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for RadioButton {}

// Use the widget_builder macro to generate the RadioButtonBuilder implementation
widget_builder!(
    name: RadioButton,
    parent_type: &'a dyn WxWidget,
    style_type: RadioButtonStyle,
    fields: {
        label: String = String::new()
    },
    build_impl: |slf| {
        let label_c = CString::new(slf.label.as_str()).unwrap_or_default();
        let ctrl_ptr = unsafe {
            ffi::wxd_RadioButton_Create(
                slf.parent.handle_ptr(),
                slf.id,
                label_c.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as ffi::wxd_Style_t,
            )
        };

        if ctrl_ptr.is_null() {
            panic!("Failed to create RadioButton: FFI returned null pointer");
        }

        unsafe { RadioButton::from_ptr(ctrl_ptr) }
    }
);

// Extension to RadioButtonBuilder to add the first_in_group method
impl<'a> RadioButtonBuilder<'a> {
    /// Marks this radio button as the first in a group.
    /// This typically means subsequent radio buttons (until the next `GroupStart` or end of dialog)
    /// belong to the same group.
    pub fn first_in_group(mut self) -> Self {
        self.style = RadioButtonStyle::GroupStart;
        self
    }
}

// --- RadioButton Event Handling ---

/// Event types specific to `RadioButton`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioButtonEvent {
    /// The radio button was selected.
    /// Corresponds to `EventType::COMMAND_RADIOBUTTON_SELECTED` (`wxEVT_RADIOBUTTON`).
    Selected,
}

/// Event data for `RadioButton` events.
#[derive(Debug)]
pub struct RadioButtonEventData {
    base: CommandEventData,
}

impl RadioButtonEventData {
    /// Creates new `RadioButtonEventData` from base `CommandEventData`.
    pub(crate) fn new(event: Event) -> Self {
        Self {
            base: CommandEventData::new(event),
        }
    }

    /// Returns the ID of the radio button that was selected.
    pub fn get_id(&self) -> i32 {
        self.base.get_id()
    }
}

// Use the implement_widget_local_event_handlers macro
crate::implement_widget_local_event_handlers!(
    RadioButton, RadioButtonEvent, RadioButtonEventData,
    Selected => selected, EventType::COMMAND_RADIOBUTTON_SELECTED
);

// Add XRC Support - enables RadioButton to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for RadioButton {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        RadioButton {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Widget casting support for RadioButton
impl crate::window::FromWindowWithClassName for RadioButton {
    fn class_name() -> &'static str {
        "wxRadioButton"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        RadioButton {
            handle: WindowHandle::new(ptr),
        }
    }
}
