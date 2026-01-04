use std::ffi::{CStr, CString};

use wxdragon_sys as ffi;

use crate::Id;
use crate::event::Event;
use crate::geometry::{Point, Size};
use crate::utils::ArrayString;
use crate::window::{WindowHandle, WxWidget};
// Window is used by impl_xrc_support for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;

/// An editable listbox is a listbox with buttons to add, remove, and reorder items in the list.
///
/// EditableListBox uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops.
///
/// # Example
/// ```ignore
/// let elb = EditableListBox::builder(&frame).with_label("Items").build();
///
/// // EditableListBox is Copy - no clone needed for closures!
/// elb.add_string("Item 1");
/// elb.add_string("Item 2");
///
/// // After parent destruction, operations are safe no-ops
/// frame.destroy();
/// assert!(!elb.is_valid());
/// ```
#[derive(Clone, Copy)]
pub struct EditableListBox {
    /// Safe handle to the underlying wxEditableListBox - automatically invalidated on destroy
    handle: WindowHandle,
}

// Style flags for EditableListBox
widget_style_enum!(
    name: EditableListBoxStyle,
    doc: "Style flags for EditableListBox widget.",
    variants: {
        Default: 0, "Default style with no special behavior.",
        AllowNew: ffi::WXD_EL_ALLOW_NEW, "Enable the New button.",
        AllowEdit: ffi::WXD_EL_ALLOW_EDIT, "Enable the Edit button.",
        AllowDelete: ffi::WXD_EL_ALLOW_DELETE, "Enable the Delete button.",
        NoReorder: ffi::WXD_EL_NO_REORDER, "Disable the Up/Down buttons.",
        DefaultStyle: ffi::WXD_EL_DEFAULT_STYLE, "Default style (AllowNew | AllowEdit | AllowDelete)."
    },
    default_variant: DefaultStyle
);

/// Events emitted by EditableListBox
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditableListBoxEvent {
    /// Emitted when an item is selected
    Selected,
    /// Emitted when an item is double-clicked (often triggers edit)
    DoubleClicked,
    /// Emitted when an item is about to be edited
    BeginLabelEdit,
    /// Emitted when an item edit is completed
    EndLabelEdit,
}

/// Event data for EditableListBox events
#[derive(Debug)]
pub struct EditableListBoxEventData {
    event: Event,
}

impl EditableListBoxEventData {
    /// Create a new EditableListBoxEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the ID of the control that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Skip this event (allow it to be processed by the parent window)
    pub fn skip(&self, skip: bool) {
        self.event.skip(skip);
    }

    /// Get the item index that was affected by this event
    pub fn get_item_index(&self) -> i32 {
        if self.event.is_null() {
            return -1;
        }
        unsafe { ffi::wxd_ListEvent_GetItemIndex(self.event.0) }
    }

    /// Get the item text (for label edit events)
    pub fn get_label(&self) -> Option<String> {
        if self.event.is_null() {
            return None;
        }
        let len = unsafe { ffi::wxd_ListEvent_GetLabel(self.event.0, std::ptr::null_mut(), 0) };
        if len < 0 {
            return None;
        }
        let mut buf = vec![0; len as usize + 1];
        unsafe { ffi::wxd_ListEvent_GetLabel(self.event.0, buf.as_mut_ptr(), buf.len()) };
        Some(unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() })
    }

    /// Check if editing was cancelled (for end edit events)
    pub fn is_edit_cancelled(&self) -> Option<bool> {
        if self.event.is_null() {
            return None;
        }
        // Boolean functions from C++ return int (0/1), already converted to Rust bool
        Some(unsafe { ffi::wxd_ListEvent_IsEditCancelled(self.event.0) })
    }
}

impl EditableListBox {
    /// Create a new EditableListBox with default settings.
    ///
    /// This is a convenience method that creates a builder, sets default values,
    /// and immediately builds the EditableListBox.
    ///
    /// # Arguments
    ///
    /// * `parent` - The parent window
    /// * `label` - The label shown at the top of the listbox
    pub fn new(parent: &dyn WxWidget, label: &str) -> Self {
        Self::builder(parent).with_label(label).build()
    }

    /// Create a builder for configuring and creating an EditableListBox.
    pub fn builder(parent: &dyn WxWidget) -> EditableListBoxBuilder<'_> {
        EditableListBoxBuilder::new(parent)
    }

    /// Creates a new EditableListBox from a raw pointer.
    /// This is intended for internal use by other widget wrappers.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Internal implementation for creating an EditableListBox directly.
    fn new_impl(parent: *mut ffi::wxd_Window_t, id: i32, label: &str, pos: Point, size: Size, style: i64) -> Self {
        assert!(!parent.is_null(), "EditableListBox requires a parent");
        let lab = CString::new(label).unwrap_or_default();

        let ptr = unsafe { ffi::wxd_EditableListBox_New(parent, id, lab.as_ptr(), pos.x, pos.y, size.width, size.height, style) };

        if ptr.is_null() {
            panic!("Failed to create EditableListBox widget");
        }

        // Create a WindowHandle which automatically registers for destroy events
        EditableListBox {
            handle: WindowHandle::new(ptr),
        }
    }

    /// Helper to get raw editable listbox pointer, returns null if widget has been destroyed
    #[inline]
    fn elb_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    /// Get all strings in the listbox.
    /// Returns empty vector if the widget has been destroyed.
    pub fn get_strings(&self) -> Vec<String> {
        let ptr = self.elb_ptr();
        if ptr.is_null() {
            return Vec::new();
        }

        let array_str_ptr = unsafe { ffi::wxd_EditableListBox_CopyStringsToArrayString(ptr) };

        if array_str_ptr.is_null() {
            return Vec::new();
        }

        let wxd_array_string = ArrayString::from(array_str_ptr);
        wxd_array_string.get_strings()
    }

    /// Set all strings in the listbox.
    /// No-op if the widget has been destroyed.
    pub fn set_strings(&self, strings: &[&str]) {
        let ptr = self.elb_ptr();
        if ptr.is_null() {
            return;
        }

        let c_strings: Vec<CString> = strings.iter().map(|s| CString::new(*s).unwrap_or_default()).collect();

        let mut c_ptrs: Vec<*const i8> = c_strings.iter().map(|s| s.as_ptr()).collect();

        unsafe { ffi::wxd_EditableListBox_SetStrings(ptr, c_ptrs.as_mut_ptr(), c_strings.len() as i32) }
    }

    /// Add a string to the listbox.
    /// No-op if the widget has been destroyed.
    pub fn add_string(&self, string: &str) {
        let ptr = self.elb_ptr();
        if ptr.is_null() {
            return;
        }

        let c_string = CString::new(string).unwrap_or_default();

        unsafe { ffi::wxd_EditableListBox_AddString(ptr, c_string.as_ptr()) }
    }

    /// Get the underlying ListBox control.
    /// Returns an invalid Window if the widget has been destroyed.
    pub fn get_list_ctrl(&self) -> Window {
        let ptr = self.elb_ptr();
        if ptr.is_null() {
            return unsafe { Window::from_ptr(std::ptr::null_mut()) };
        }

        let list_ptr = unsafe { ffi::wxd_EditableListBox_GetListCtrl(ptr) };

        // We don't take ownership, just a reference
        unsafe { Window::from_ptr(list_ptr) }
    }

    /// Returns the underlying WindowHandle for this editable listbox.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for EditableListBox (using WindowHandle)
impl WxWidget for EditableListBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Note: We don't implement Deref to Window because returning a reference
// to a temporary Window is unsound. Users can access window methods through
// the WxWidget trait methods directly.

// Implement WxEvtHandler for event binding
impl crate::event::WxEvtHandler for EditableListBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Builder for EditableListBox
widget_builder!(
    name: EditableListBox,
    parent_type: &'a dyn WxWidget,
    style_type: EditableListBoxStyle,
    fields: {
        label: String = String::new()
    },
    build_impl: |slf| {
        EditableListBox::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            &slf.label,
            slf.pos,
            slf.size,
            slf.style.bits()
        )
    }
);

// Implement event handlers for EditableListBox
crate::implement_widget_local_event_handlers!(
    EditableListBox,
    EditableListBoxEvent,
    EditableListBoxEventData,
    Selected => selection_changed, crate::event::EventType::COMMAND_LISTBOX_SELECTED,
    DoubleClicked => item_double_clicked, crate::event::EventType::COMMAND_LISTBOX_DOUBLECLICKED,
    BeginLabelEdit => begin_label_edit, crate::event::EventType::LIST_BEGIN_LABEL_EDIT,
    EndLabelEdit => end_label_edit, crate::event::EventType::LIST_END_LABEL_EDIT
);

// XRC Support - enables EditableListBox to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for EditableListBox {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        EditableListBox {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Enable widget casting for EditableListBox
impl crate::window::FromWindowWithClassName for EditableListBox {
    fn class_name() -> &'static str {
        "wxEditableListBox"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        EditableListBox {
            handle: WindowHandle::new(ptr),
        }
    }
}
