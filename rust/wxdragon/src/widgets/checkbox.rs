use crate::event::event_data::CommandEventData;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::CString;
use wxdragon_sys as ffi;

// Re-export specific CheckBox constants if needed later

/// Represents a wxCheckBox.
///
/// CheckBox uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct CheckBox {
    /// Safe handle to the underlying wxCheckBox - automatically invalidated on destroy
    handle: WindowHandle,
}

impl CheckBox {
    /// Creates a new `CheckBoxBuilder` for constructing a checkbox.
    pub fn builder(parent: &dyn WxWidget) -> CheckBoxBuilder<'_> {
        CheckBoxBuilder::new(parent)
    }

    /// Low-level constructor used by the builder.
    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: Id, label: &str, pos: Point, size: Size, style: i64) -> Self {
        let label_c = CString::new(label).unwrap_or_default();
        let ctrl_ptr = unsafe {
            ffi::wxd_CheckBox_Create(
                parent_ptr,
                id,
                label_c.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };
        assert!(!ctrl_ptr.is_null(), "wxd_CheckBox_Create returned null");
        CheckBox {
            handle: WindowHandle::new(ctrl_ptr as *mut ffi::wxd_Window_t),
        }
    }

    /// Helper to get raw checkbox pointer, returns null if widget has been destroyed
    #[inline]
    fn widget_ptr(&self) -> *mut ffi::wxd_CheckBox_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_CheckBox_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Returns `true` if the checkbox is checked, `false` otherwise.
    /// Returns `false` if the widget has been destroyed.
    pub fn is_checked(&self) -> bool {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_CheckBox_IsChecked(ptr) }
    }

    /// Sets the checkbox to the given state.
    /// No-op if the widget has been destroyed.
    pub fn set_value(&self, value: bool) {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_CheckBox_SetValue(ptr, value);
        }
    }

    /// Returns `true` if the checkbox is checked, `false` otherwise.
    /// Alias for `is_checked()` to match common widget patterns.
    /// Returns `false` if the widget has been destroyed.
    pub fn get_value(&self) -> bool {
        self.is_checked()
    }

    /// Returns the underlying WindowHandle for this checkbox.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// --- CheckBox Event Handling ---

/// Event types specific to `CheckBox`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckBoxEvent {
    /// The checkbox state has changed (checked or unchecked).
    /// Corresponds to `EventType::CHECKBOX` (`wxEVT_CHECKBOX`).
    Toggled,
}

/// Event data for `CheckBox` events.
#[derive(Debug)]
pub struct CheckBoxEventData {
    base: CommandEventData,
}

impl CheckBoxEventData {
    /// Creates new `CheckBoxEventData` from base `CommandEventData`.
    pub(crate) fn new(event: Event) -> Self {
        Self {
            base: CommandEventData::new(event),
        }
    }

    /// Returns `true` if the checkbox is currently checked, `false` otherwise.
    /// This reflects the state of the checkbox when the event occurred.
    pub fn is_checked(&self) -> bool {
        self.base.is_checked().unwrap_or(false) // CHECKBOX event should always provide this
    }

    /// Returns the ID of the checkbox that generated the event.
    pub fn get_id(&self) -> i32 {
        self.base.get_id()
    }
}

// Use the implement_widget_local_event_handlers macro
crate::implement_widget_local_event_handlers!(
    CheckBox, CheckBoxEvent, CheckBoxEventData,
    Toggled => toggled, EventType::CHECKBOX
);

// Define the CheckBoxStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: CheckBoxStyle,
    doc: "Style flags for `CheckBox`.",
    variants: {
        Default: ffi::WXD_CHK_2STATE, "Default style (2-state, label on the right).",
        ThreeState: ffi::WXD_CHK_3STATE, "Three-state checkbox. The third state is \"undetermined\".",
        AllowUserThirdState: ffi::WXD_CHK_ALLOW_3RD_STATE_FOR_USER, "Allows the user to set the checkbox to the third state (undetermined). Only applicable if `ThreeState` is also used.",
        AlignLeft: 0, "Align label to the right of the checkbox (checkbox on the left). This is usually the default layout.",
        AlignRight: ffi::WXD_ALIGN_RIGHT, "Align label to the left of the checkbox (checkbox on the right)."
    },
    default_variant: Default
);

// Use the widget_builder macro to generate the CheckBoxBuilder implementation
widget_builder!(
    name: CheckBox,
    parent_type: &'a dyn WxWidget,
    style_type: CheckBoxStyle,
    fields: {
        label: String = String::new(),
        value: bool = false
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let checkbox = CheckBox::new_impl(
            parent_ptr,
            slf.id,
            &slf.label,
            slf.pos,
            slf.size,
            slf.style.bits(),
        );
        checkbox.set_value(slf.value);
        checkbox
    }
);

// Manual WxWidget implementation for CheckBox (using WindowHandle)
impl WxWidget for CheckBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for CheckBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for CheckBox {}

// XRC Support - enables CheckBox to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for CheckBox {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        CheckBox {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Widget casting support for CheckBox
impl crate::window::FromWindowWithClassName for CheckBox {
    fn class_name() -> &'static str {
        "wxCheckBox"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        CheckBox {
            handle: WindowHandle::new(ptr),
        }
    }
}
