use crate::event::event_data::CommandEventData;
use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

// Special value returned by GetSelection when nothing is selected
pub const NOT_FOUND: i32 = -1; // wxNOT_FOUND is typically -1

// Create a proper style enum for Choice
widget_style_enum!(
    name: ChoiceStyle,
    doc: "Style flags for the Choice widget.",
    variants: {
        Default: 0, "Default style.",
        Sort: ffi::WXD_CB_SORT, "The items in the choice control are kept sorted alphabetically."
    },
    default_variant: Default
);

/// Represents a wxChoice control (dropdown list).
///
/// Choice uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
#[derive(Clone, Copy)]
pub struct Choice {
    /// Safe handle to the underlying wxChoice - automatically invalidated on destroy
    handle: WindowHandle,
}

impl Choice {
    /// Creates a new `ChoiceBuilder` for constructing a choice control.
    pub fn builder(parent: &dyn WxWidget) -> ChoiceBuilder<'_> {
        ChoiceBuilder::new(parent)
    }

    /// Helper to get raw choice pointer, returns null if widget has been destroyed
    #[inline]
    fn widget_ptr(&self) -> *mut ffi::wxd_Choice_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_Choice_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Appends an item to the choice control.
    /// No-op if the widget has been destroyed.
    pub fn append(&self, item: &str) {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return;
        }
        let c_item = CString::new(item).expect("Invalid CString for Choice item");
        unsafe {
            ffi::wxd_Choice_Append(ptr, c_item.as_ptr());
        }
    }

    /// Removes all items from the choice control.
    /// No-op if the widget has been destroyed.
    pub fn clear(&self) {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_Choice_Clear(ptr);
        }
    }

    /// Gets the index of the currently selected item.
    /// Returns `None` if no item is selected (matches `NOT_FOUND`) or if the widget has been destroyed.
    pub fn get_selection(&self) -> Option<u32> {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return None;
        }
        let selection = unsafe { ffi::wxd_Choice_GetSelection(ptr) };
        if selection == NOT_FOUND {
            None
        } else {
            Some(selection as u32)
        }
    }

    /// Gets the string value of the currently selected item.
    /// Returns `None` if no item is selected or if the widget has been destroyed.
    pub fn get_string_selection(&self) -> Option<String> {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return None;
        }
        let len = unsafe { ffi::wxd_Choice_GetStringSelection(ptr, std::ptr::null_mut(), 0) };

        if len < 0 {
            return None; // Error or no selection
        }

        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_Choice_GetStringSelection(ptr, buf.as_mut_ptr(), buf.len()) };
        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
    }

    /// Selects the item at the given index.
    /// No-op if the widget has been destroyed.
    pub fn set_selection(&self, index: u32) {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Choice_SetSelection(ptr, index as i32) };
    }

    /// Gets the string at the specified index.
    /// Returns `None` if the index is out of bounds or if the widget has been destroyed.
    pub fn get_string(&self, index: u32) -> Option<String> {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return None;
        }
        let len = unsafe { ffi::wxd_Choice_GetString(ptr, index as i32, std::ptr::null_mut(), 0) };
        if len < 0 {
            return None; // Error or invalid index
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_Choice_GetString(ptr, index as i32, buf.as_mut_ptr(), buf.len()) };
        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
    }

    /// Gets the number of items in the choice control.
    /// Returns 0 if the widget has been destroyed.
    pub fn get_count(&self) -> u32 {
        let ptr = self.widget_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Choice_GetCount(ptr) }
    }

    /// Returns the underlying WindowHandle for this choice.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

widget_builder!(
    name: Choice,
    parent_type: &'a dyn WxWidget,
    style_type: ChoiceStyle,
    fields: {
        choices: Vec<String> = Vec::new(),
        selection: Option<u32> = None
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let pos = slf.pos.into();
        let size = slf.size.into();

        // Create the choice control
        let ctrl_ptr = unsafe {
            ffi::wxd_Choice_Create(
                parent_ptr,
                slf.id,
                pos,
                size,
                slf.style.bits()
            )
        };

        if ctrl_ptr.is_null() {
            panic!("Failed to create Choice widget");
        }

        let choice = Choice {
            handle: WindowHandle::new(ctrl_ptr as *mut ffi::wxd_Window_t),
        };

        // Add initial choices
        for choice_str in &slf.choices {
            choice.append(choice_str);
        }

        // Set initial selection if provided
        if let Some(sel) = slf.selection {
            choice.set_selection(sel);
        }

        choice
    }
);

// Manual WxWidget implementation for Choice (using WindowHandle)
impl WxWidget for Choice {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for Choice {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for Choice {}

// --- Choice specific event enum ---
/// Events specific to Choice controls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoiceEvent {
    /// Fired when an item is selected
    Selected,
}

/// Event data for Choice events
#[derive(Debug)]
pub struct ChoiceEventData {
    pub event: CommandEventData,
}

impl ChoiceEventData {
    pub fn new(event: Event) -> Self {
        Self {
            event: CommandEventData::new(event),
        }
    }

    /// Get the widget ID that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Get the selected item's index
    pub fn get_selection(&self) -> Option<i32> {
        self.event.get_int()
    }

    /// Get the selected item's text (if available)
    pub fn get_string(&self) -> Option<String> {
        self.event.get_string()
    }
}

// At the bottom of the file, use the local macro
crate::implement_widget_local_event_handlers!(
    Choice,
    ChoiceEvent,
    ChoiceEventData,
    Selected => selection_changed, EventType::COMMAND_CHOICE_SELECTED
);

// Add XRC Support - enables Choice to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for Choice {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Choice {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Widget casting support for Choice
impl crate::window::FromWindowWithClassName for Choice {
    fn class_name() -> &'static str {
        "wxChoice"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Choice {
            handle: WindowHandle::new(ptr),
        }
    }
}
