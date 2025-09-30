use crate::event::event_data::CommandEventData;
use crate::event::{Event, EventType};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use wxdragon_sys as ffi;

// Re-export specific CheckBox constants if needed later

/// Represents a wxCheckBox.
#[derive(Clone)]
pub struct CheckBox {
    window: Window,
}

impl CheckBox {
    /// Creates a new `CheckBoxBuilder` for constructing a checkbox.
    pub fn builder(parent: &dyn WxWidget) -> CheckBoxBuilder<'_> {
        CheckBoxBuilder::new(parent)
    }

    /// Low-level constructor used by the builder.
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        label: &str,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
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
        unsafe { Self::from_ptr(ctrl_ptr) }
    }

    /// Returns `true` if the checkbox is checked, `false` otherwise.
    pub fn is_checked(&self) -> bool {
        unsafe { ffi::wxd_CheckBox_IsChecked(self.window.as_ptr() as *mut ffi::wxd_CheckBox_t) }
    }

    /// Sets the checkbox to the given state.
    pub fn set_value(&self, value: bool) {
        unsafe {
            ffi::wxd_CheckBox_SetValue(self.window.as_ptr() as *mut ffi::wxd_CheckBox_t, value);
        }
    }

    /// Returns `true` if the checkbox is checked, `false` otherwise.
    /// Alias for `is_checked()` to match common widget patterns.
    pub fn get_value(&self) -> bool {
        self.is_checked()
    }

    // Private unsafe constructor from raw pointer
    unsafe fn from_ptr(ptr: *mut ffi::wxd_CheckBox_t) -> Self {
        assert!(!ptr.is_null());
        CheckBox {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
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

// Implement common widget traits
implement_widget_traits_with_target!(CheckBox, window, Window);

// XRC Support - enables CheckBox to be created from XRC-managed pointers
impl_xrc_support!(CheckBox, { window });

// Widget casting support for CheckBox
impl_widget_cast!(CheckBox, "wxCheckBox", { window });
