// wxdragon/src/menus/menuitem.rs
//! wxMenuItem wrapper and related types

use crate::event::{Event, EventType, WxEvtHandler};
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use wxdragon_sys as ffi;

// --- Standard Menu Item IDs ---
// Define explicitly as i32, casting from the ffi i64 type
pub const ID_EXIT: i32 = ffi::WXD_ID_EXIT as i32;
pub const ID_ABOUT: i32 = ffi::WXD_ID_ABOUT as i32;
pub const ITEM_NORMAL: i32 = ffi::WXD_ITEM_NORMAL as i32;
pub const ITEM_CHECK: i32 = ffi::WXD_ITEM_CHECK as i32;
pub const ITEM_RADIO: i32 = ffi::WXD_ITEM_RADIO as i32;
pub const ITEM_SEPARATOR: i32 = ffi::WXD_ITEM_SEPARATOR as i32;

// Often used ID for separators
pub const ID_SEPARATOR: i32 = ffi::WXD_ITEM_SEPARATOR as i32; // Use ITEM_SEPARATOR value

// --- Item Kind Enum ---
// Cast from ffi i64 constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ItemKind {
    Normal = ffi::WXD_ITEM_NORMAL as i32,
    Separator = ffi::WXD_ITEM_SEPARATOR as i32,
    Check = ffi::WXD_ITEM_CHECK as i32,
    Radio = ffi::WXD_ITEM_RADIO as i32,
}

impl From<ItemKind> for i32 {
    fn from(kind: ItemKind) -> Self {
        kind as i32
    }
}

/// Represents a wxMenuItem.
/// This can be either a wrapper around an existing menu item or loaded from XRC.
#[derive(Clone)]
pub struct MenuItem {
    ptr: *mut ffi::wxd_MenuItem_t, // Non-owning pointer
    /// Reference to the parent window that will receive menu events
    parent_window: Window,
    /// The menu item's ID for event handling
    item_id: i32,
    /// The menu item's XRC name for identification (if loaded from XRC)
    item_name: Option<String>,
}

impl From<*mut ffi::wxd_MenuItem_t> for MenuItem {
    fn from(ptr: *mut ffi::wxd_MenuItem_t) -> Self {
        MenuItem::from_ptr(ptr)
    }
}

impl From<*const ffi::wxd_MenuItem_t> for MenuItem {
    fn from(ptr: *const ffi::wxd_MenuItem_t) -> Self {
        MenuItem::from_ptr(ptr as *mut ffi::wxd_MenuItem_t)
    }
}

impl MenuItem {
    /// Creates a non-owning wrapper from a raw pointer.
    /// Note: This function does not dereference the pointer. The wrapper is
    /// non-owning; validity must be ensured by the creator (e.g., owned by wxMenu).
    pub(crate) fn from_ptr(ptr: *mut ffi::wxd_MenuItem_t) -> Self {
        // Try to resolve owning window (typically the frame) and real id
        let owner_ptr = unsafe { ffi::wxd_MenuItem_GetOwningWindow(ptr) };
        let item_id = unsafe { ffi::wxd_MenuItem_GetId(ptr) };

        let parent_window = unsafe { Window::from_ptr(owner_ptr as *mut ffi::wxd_Window_t) };

        MenuItem {
            ptr,
            parent_window,
            item_id,
            item_name: None,
        }
    }

    /// Creates a MenuItem wrapper from XRC information.
    /// This is typically called by the XRC loading system.
    #[cfg(feature = "xrc")]
    pub(crate) fn new(parent_window: Window, item_id: i32, item_name: String) -> Self {
        Self {
            ptr: std::ptr::null_mut(), // Not used for XRC items
            parent_window,
            item_id,
            item_name: Some(item_name),
        }
    }

    /// Gets the menu item's ID used for event handling.
    pub fn get_item_id(&self) -> i32 {
        self.item_id
    }

    /// Gets the menu item's XRC name.
    pub fn get_item_name(&self) -> Option<&str> {
        self.item_name.as_deref()
    }

    /// Binds a click event handler to this menu item.
    /// This binds a menu event on the parent window for this item's ID.
    pub fn on_click<F>(&self, handler: F)
    where
        F: FnMut(Event) + 'static,
    {
        // Use ID-specific binding for MENU events
        self.parent_window
            .bind_with_id_internal(EventType::MENU, self.item_id, handler);
    }

    /// Special XRC loading method for menu items.
    /// This looks up the menu item by name and creates a MenuItem wrapper.
    #[cfg(feature = "xrc")]
    pub fn from_xrc_name(parent_window: &Window, item_name: &str) -> Option<Self> {
        use crate::xrc::XmlResource;

        // Get the XRC ID for this menu item name
        let item_id = XmlResource::get_xrc_id(item_name);

        if item_id != -1 {
            Some(MenuItem::new(
                *parent_window,
                item_id,
                item_name.to_string(),
            ))
        } else {
            None
        }
    }

    // --- Methods to modify MenuItem state (if needed later) ---
    /// Sets the menu item's label text.
    pub fn set_label(&self, label: &str) {
        if self.ptr.is_null() {
            // This is an XRC item - we can't modify it directly through a pointer
            // For XRC items, modification typically happens through the parent window
            // or by re-loading the resource, which is not supported in this implementation
            return;
        }
        let c_label = CString::new(label).unwrap_or_default();
        unsafe {
            ffi::wxd_MenuItem_SetLabel(self.ptr, c_label.as_ptr());
        }
    }

    /// Gets the menu item's label text.
    pub fn get_label(&self) -> String {
        if self.ptr.is_null() {
            // For XRC items, we can't get the label directly
            return String::new();
        }
        unsafe {
            let c_str = ffi::wxd_MenuItem_GetLabel(self.ptr);
            if c_str.is_null() {
                return String::new();
            }
            let rust_str = CStr::from_ptr(c_str).to_string_lossy().into_owned();
            // Free the C string returned by the FFI function using wxd_free_string
            ffi::wxd_free_string(c_str);
            rust_str
        }
    }

    /// Enables or disables the menu item.
    pub fn enable(&self, enable: bool) {
        if self.ptr.is_null() {
            // For XRC items, we can't modify them directly
            return;
        }
        unsafe {
            ffi::wxd_MenuItem_Enable(self.ptr, enable);
        }
    }

    /// Returns true if the menu item is enabled.
    pub fn is_enabled(&self) -> bool {
        if self.ptr.is_null() {
            // For XRC items, assume enabled by default
            return true;
        }
        unsafe { ffi::wxd_MenuItem_IsEnabled(self.ptr) }
    }

    /// Checks or unchecks the menu item (for Check/Radio items).
    /// This only works for menu items that were created with `ItemKind::Check` or `ItemKind::Radio`.
    pub fn check(&self, check: bool) {
        if self.ptr.is_null() {
            // For XRC items, we can't modify them directly
            return;
        }
        unsafe {
            ffi::wxd_MenuItem_Check(self.ptr, check);
        }
    }

    /// Returns true if the menu item is checked (for Check/Radio items).
    pub fn is_checked(&self) -> bool {
        if self.ptr.is_null() {
            // For XRC items, assume unchecked by default
            return false;
        }
        unsafe { ffi::wxd_MenuItem_IsChecked(self.ptr) }
    }
}

/// Implement WxWidget for MenuItem (delegating to parent window)
impl WxWidget for MenuItem {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        // Menu items don't have their own window handle - they're part of the menu system
        // Return the parent window's handle for XRC compatibility
        self.parent_window.as_ptr()
    }

    fn get_id(&self) -> i32 {
        self.item_id
    }
}

/// Event handler implementation for MenuItem (delegates to parent window)
impl WxEvtHandler for MenuItem {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.parent_window.as_ptr() as *mut ffi::wxd_EvtHandler_t
    }
}

// Note: No Drop impl here, as wxMenu takes ownership.
