use crate::color::{Colour, colours};
use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by impl_xrc_support for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
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
///
/// ColourPickerCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let picker = ColourPickerCtrl::builder(&frame).initial_colour(colours::RED).build();
///
/// // ColourPickerCtrl is Copy - no clone needed for closures!
/// picker.bind_colour_changed(move |_| {
///     // Safe: if picker was destroyed, this is a no-op
///     let colour = picker.get_colour();
///     println!("Selected colour: {:?}", colour);
/// });
///
/// // After parent destruction, picker operations are safe no-ops
/// frame.destroy();
/// assert!(!picker.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct ColourPickerCtrl {
    /// Safe handle to the underlying wxColourPickerCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl ColourPickerCtrl {
    /// Creates a new `ColourPickerCtrlBuilder` for constructing a colour picker control.
    pub fn builder(parent: &dyn WxWidget) -> ColourPickerCtrlBuilder<'_> {
        ColourPickerCtrlBuilder::new(parent)
    }

    /// Helper to get raw colour picker pointer, returns null if widget has been destroyed
    #[inline]
    fn picker_ptr(&self) -> *mut ffi::wxd_ColourPickerCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_ColourPickerCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Gets the currently selected colour.
    /// Returns black if the picker has been destroyed.
    pub fn get_colour(&self) -> Colour {
        let ptr = self.picker_ptr();
        if ptr.is_null() {
            return colours::BLACK;
        }
        let c_colour = unsafe { ffi::wxd_ColourPickerCtrl_GetColour(ptr) };
        Colour::from(c_colour)
    }

    /// Sets the currently selected colour.
    /// No-op if the picker has been destroyed.
    pub fn set_colour(&self, colour: Colour) {
        let ptr = self.picker_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_ColourPickerCtrl_SetColour(ptr, colour.into()) };
    }

    /// Returns the underlying WindowHandle for this colour picker.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Implement event handlers for ColourPickerCtrl
crate::implement_widget_local_event_handlers!(
    ColourPickerCtrl,
    ColourPickerCtrlEvent,
    ColourPickerCtrlEventData,
    ColourChanged => colour_changed, EventType::COLOURPICKER_CHANGED
);

// Manual WxWidget implementation for ColourPickerCtrl (using WindowHandle)
impl WxWidget for ColourPickerCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for ColourPickerCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for ColourPickerCtrl {}

// XRC Support - enables ColourPickerCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for ColourPickerCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ColourPickerCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for ColourPickerCtrl
impl crate::window::FromWindowWithClassName for ColourPickerCtrl {
    fn class_name() -> &'static str {
        "wxColourPickerCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        ColourPickerCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

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
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }
);
