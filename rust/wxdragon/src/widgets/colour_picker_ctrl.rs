use crate::color::{colours, Colour};
use crate::event::{Event, EventType};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::default::Default;
use wxdragon_sys as ffi;

// --- ColourPickerCtrl Style Enum ---

widget_style_enum!(
    name: ColourPickerCtrlStyle,
    doc: "Style flags for the ColourPickerCtrl widget.",
    variants: {
        Default: ffi::WXD_CLRP_DEFAULT_STYLE, "Default style with no specific options.",
        UseTextCtrl: ffi::WXD_CLRP_USE_TEXTCTRL, "Creates a text control to the left of the picker button which can be used by the user to specify a colour.",
        ShowLabel: ffi::WXD_CLRP_SHOW_LABEL, "Shows the colour in HTML form (AABBCC) as colour button label.",
        ShowAlpha: ffi::WXD_CLRP_SHOW_ALPHA, "Allows selecting opacity in the colour-chooser (effective under wxGTK and wxOSX)."
    },
    default_variant: Default
);

/// Events emitted by ColourPickerCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColourPickerCtrlEvent {
    /// Emitted when the user selects a colour
    ColourChanged,
}

/// Event data for ColourPickerCtrl events
#[derive(Debug)]
pub struct ColourPickerCtrlEventData {
    event: Event,
}

impl ColourPickerCtrlEventData {
    /// Create a new ColourPickerCtrlEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the colour that was selected
    pub fn get_colour(&self) -> Option<Colour> {
        if self.event.is_null() {
            return None;
        }
        // Directly call the FFI function
        let c_colour = unsafe { ffi::wxd_ColourPickerEvent_GetColour(self.event.0) };
        Some(Colour::from(c_colour))
    }
}

// --- ColourPickerCtrl Widget ---

/// Represents a wxColourPickerCtrl, which allows the user to select a colour.
#[derive(Clone)]
pub struct ColourPickerCtrl {
    window: Window,
}

impl ColourPickerCtrl {
    /// Creates a new `ColourPickerCtrlBuilder` for constructing a colour picker control.
    pub fn builder(parent: &dyn WxWidget) -> ColourPickerCtrlBuilder<'_> {
        ColourPickerCtrlBuilder::new(parent)
    }

    /// Gets the currently selected colour.
    pub fn get_colour(&self) -> Colour {
        let c_colour =
            unsafe { ffi::wxd_ColourPickerCtrl_GetColour(self.window.as_ptr() as *mut _) };
        Colour::from(c_colour)
    }

    /// Sets the currently selected colour.
    pub fn set_colour(&self, colour: Colour) {
        unsafe {
            ffi::wxd_ColourPickerCtrl_SetColour(self.window.as_ptr() as *mut _, colour.into())
        };
    }
}

// Implement event handlers for ColourPickerCtrl
crate::implement_widget_local_event_handlers!(
    ColourPickerCtrl,
    ColourPickerCtrlEvent,
    ColourPickerCtrlEventData,
    ColourChanged => colour_changed, EventType::COLOURPICKER_CHANGED
);

// Add XRC Support - enables ColourPickerCtrl to be created from XRC-managed pointers
impl_xrc_support!(ColourPickerCtrl, { window });

// Widget casting support for ColourPickerCtrl
impl_widget_cast!(ColourPickerCtrl, "wxColourPickerCtrl", { window });

widget_builder!(
    name: ColourPickerCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: ColourPickerCtrlStyle,
    fields: {
        initial_colour: Colour = colours::BLACK
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let pos = slf.pos.into();
        let size = slf.size.into();
        let colour = slf.initial_colour.into();

        let ptr = unsafe {
            ffi::wxd_ColourPickerCtrl_Create(
                parent_ptr,
                slf.id,
                colour,
                pos,
                size,
                slf.style.bits(),
            )
        };

        if ptr.is_null() {
            panic!("Failed to create wxColourPickerCtrl");
        }

        ColourPickerCtrl {
            window: unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) },
        }
    }
);

implement_widget_traits_with_target!(ColourPickerCtrl, window, Window);
