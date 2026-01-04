//! wxMenu wrapper

use crate::id::Id;
use crate::menus::menuitem::{ItemKind, MenuItem};
#[cfg(feature = "xrc")]
use crate::window::WindowHandle;
#[cfg(feature = "xrc")]
use crate::xrc::XmlResource;
use crate::{CommandEventData, Event, EventType};
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::rc::Rc;
use wxdragon_sys as ffi;

/// Safe Rust wrapper over a wxMenu pointer (wxd_Menu_t).
///
/// By default, instances created via builder or From<*mut> own the underlying native resource
/// and will destroy it on drop. Instances created from a const pointer are treated as borrowed
/// and will not destroy the underlying object.
pub struct Menu {
    ptr: *mut ffi::wxd_Menu_t,
    /// Whether this wrapper owns the underlying wxMenu and should destroy it on drop.
    owned: bool,
    /// Prevent Send/Sync: wxWidgets objects are not thread-safe and must stay on UI thread.
    _nosend_nosync: PhantomData<Rc<()>>,
}

impl Drop for Menu {
    fn drop(&mut self) {
        self.destroy_menu();
    }
}

impl From<*mut ffi::wxd_Menu_t> for Menu {
    fn from(ptr: *mut ffi::wxd_Menu_t) -> Self {
        assert!(!ptr.is_null(), "invalid null pointer passed to Menu::from");
        Menu {
            ptr,
            owned: true,
            _nosend_nosync: PhantomData,
        }
    }
}

impl From<*const ffi::wxd_Menu_t> for Menu {
    fn from(ptr: *const ffi::wxd_Menu_t) -> Self {
        assert!(!ptr.is_null(), "invalid null pointer passed to Menu::from");
        let ptr = ptr as *mut ffi::wxd_Menu_t;
        Menu {
            ptr,
            owned: false,
            _nosend_nosync: PhantomData,
        }
    }
}

// Pointer conversions mirroring Variant
impl TryFrom<Menu> for *const ffi::wxd_Menu_t {
    type Error = std::io::Error;
    fn try_from(value: Menu) -> Result<Self, Self::Error> {
        use std::io::{Error, ErrorKind::InvalidData, ErrorKind::InvalidInput};
        if value.ptr.is_null() {
            return Err(Error::new(InvalidInput, "Menu pointer is null"));
        }
        if value.is_owned() {
            return Err(Error::new(
                InvalidData,
                "Menu owns the pointer, use into_raw_mut or mutable version",
            ));
        }
        let ptr = value.ptr as *const _;
        std::mem::forget(value);
        Ok(ptr)
    }
}

impl TryFrom<Menu> for *mut ffi::wxd_Menu_t {
    type Error = std::io::Error;
    fn try_from(value: Menu) -> Result<Self, Self::Error> {
        use std::io::{Error, ErrorKind::InvalidData, ErrorKind::InvalidInput};
        if value.ptr.is_null() {
            return Err(Error::new(InvalidInput, "Menu pointer is null"));
        }
        if !value.is_owned() {
            return Err(Error::new(InvalidData, "Menu does not own the pointer, use const version"));
        }
        let ptr = value.ptr;
        std::mem::forget(value);
        Ok(ptr)
    }
}

impl Menu {
    /// Creates a new, empty menu using the builder pattern.
    pub fn builder() -> MenuBuilder {
        MenuBuilder::new()
    }

    /// Returns whether this wrapper owns the underlying wxMenu.
    pub fn is_owned(&self) -> bool {
        self.owned
    }

    /// Gets the number of items in the menu.
    pub fn get_item_count(&self) -> usize {
        unsafe { ffi::wxd_Menu_GetMenuItemCount(self.ptr) }
    }

    /// Gets the title of the menu.
    pub fn get_title(&self) -> Option<String> {
        // First, get the required buffer size
        let size = unsafe { ffi::wxd_Menu_GetTitle(self.ptr, std::ptr::null_mut(), 0) };
        if size < 0 {
            return None;
        }

        let mut buffer = vec![0; size as usize + 1]; // +1 for null terminator
        unsafe { ffi::wxd_Menu_GetTitle(self.ptr, buffer.as_mut_ptr(), buffer.len()) };
        Some(unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().to_string() })
    }

    pub fn set_title(&mut self, title: &str) {
        let title_c = CString::new(title).unwrap_or_default();
        unsafe { ffi::wxd_Menu_SetTitle(self.as_mut_ptr(), title_c.as_ptr()) };
    }

    /// Explicitly destroy this Menu. Use this for standalone/popup menus that are not
    /// appended to a MenuBar. After calling this method, the Menu must not be used.
    ///
    /// Safety: Do NOT call this if the menu was appended to a MenuBar, as the menubar
    /// takes ownership and will delete it, leading to double free.
    pub fn destroy_menu(&mut self) {
        if self.owned && !self.ptr.is_null() {
            log::debug!("Menu '{:?}' destroyed", self.get_title());
            unsafe { ffi::wxd_Menu_Destroy(self.ptr) };
            self.ptr = std::ptr::null_mut();
        }
    }

    /// Returns a const raw pointer to the underlying wxMenu.
    /// This does not transfer ownership and is only valid while self is alive.
    pub fn as_const_ptr(&self) -> *const ffi::wxd_Menu_t {
        self.ptr as *const _
    }

    /// Returns a mutable raw pointer to the underlying wxMenu.
    /// This does not transfer ownership.
    pub fn as_mut_ptr(&mut self) -> *mut ffi::wxd_Menu_t {
        self.ptr
    }

    /// Consumes self and returns a raw mutable pointer, transferring ownership to the caller.
    /// After calling this, you must destroy the pointer exactly once using wxd_Menu_Destroy.
    pub fn into_raw_mut(self) -> *mut ffi::wxd_Menu_t {
        self.try_into().expect("into_raw_mut must only be used on owning wrappers")
    }

    /// Consumes a borrowed (non-owning) wrapper and returns a raw const pointer without taking ownership.
    /// Panics if called on an owning wrapper to avoid leaking the owned resource.
    pub fn into_raw_const(self) -> *const ffi::wxd_Menu_t {
        self.try_into()
            .expect("into_raw_const must only be used on non-owning wrappers")
    }

    /// Appends a menu item.
    /// Returns a wrapper for the created item (for potential modification), but ownership remains with the menu.
    pub fn append(&self, id: Id, item: &str, help_string: &str, kind: ItemKind) -> Option<MenuItem> {
        self.append_raw(id, item, help_string, kind)
    }

    /// Appends a submenu.
    pub fn append_submenu(&self, submenu: Menu, title: &str, help_string: &str) -> Option<MenuItem> {
        let title = CString::new(title).unwrap_or_default();
        let help_c = CString::new(help_string).unwrap_or_default();
        let item_ptr = unsafe { ffi::wxd_Menu_AppendSubMenu(self.ptr, submenu.into_raw_mut(), title.as_ptr(), help_c.as_ptr()) };
        if item_ptr.is_null() {
            return None;
        }
        // Return a MenuItem wrapper, but don't give it ownership
        Some(MenuItem::from(item_ptr))
    }

    /// Enables or disables a menu item by its ID.
    /// Returns true if the operation was successful.
    pub fn enable_item(&self, id: Id, enable: bool) -> bool {
        unsafe { ffi::wxd_Menu_ItemEnable(self.ptr, id, enable) }
    }

    /// Checks if a menu item is enabled.
    pub fn is_item_enabled(&self, id: Id) -> bool {
        unsafe { ffi::wxd_Menu_IsItemEnabled(self.ptr, id) }
    }

    /// Appends a separator.
    pub fn append_separator(&self) {
        self.append_separator_raw();
    }

    /// Gets a menu item by its XRC name.
    /// Returns a MenuItem wrapper that can be used for event binding.
    #[cfg(feature = "xrc")]
    pub fn get_item_by_name(&self, parent_handle: WindowHandle, item_name: &str) -> Option<MenuItem> {
        MenuItem::from_xrc_name(parent_handle, item_name)
    }

    /// Special XRC loading method for menus.
    /// This looks up the menu by name and creates a Menu wrapper.
    #[cfg(feature = "xrc")]
    pub fn from_xrc_name(menu_name: &str) -> Option<Self> {
        // For now, we'll assume menus are loaded as part of menubar
        // This might need to be extended if we support standalone menu loading
        // Get the XRC resource to check if the menu exists
        let menu_id = XmlResource::get_xrc_id(menu_name);

        if menu_id != -1 {
            // This is a placeholder - in practice, menus are usually loaded as part of menubars
            // We might need to extend XRC support for standalone menus if needed
            None
        } else {
            None
        }
    }

    // Make append private as it's called by builder
    fn append_raw(&self, id: Id, item: &str, help_string: &str, kind: ItemKind) -> Option<MenuItem> {
        let item_c = CString::new(item).unwrap_or_default();
        let help_c = CString::new(help_string).unwrap_or_default();
        let item_ptr = unsafe { ffi::wxd_Menu_Append(self.ptr, id, item_c.as_ptr(), help_c.as_ptr(), kind.into()) };
        if item_ptr.is_null() {
            None
        } else {
            // Return a MenuItem wrapper, but don't give it ownership
            Some(MenuItem::from_ptr(item_ptr))
        }
    }

    // Make append_separator private as it's called by builder
    fn append_separator_raw(&self) {
        unsafe { ffi::wxd_Menu_AppendSeparator(self.ptr) };
    }
}

// Note: No Drop impl here, as wxMenuBar takes ownership via Append.

// --- Menu Builder ---

// Enum to represent actions to perform on the menu during build
enum MenuAction {
    AppendItem {
        id: Id,
        item: String,
        help: String,
        kind: ItemKind,
    },
    AppendSeparator,
    // TODO: Add AppendSubMenu if needed
}

/// Builder for [`Menu`].
#[derive(Default)]
pub struct MenuBuilder {
    actions: Vec<MenuAction>,
    title: String,
    _marker: PhantomData<()>,
}

impl MenuBuilder {
    /// Creates a new, default builder.
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Adds an item to be appended to the menu.
    pub fn append_item(mut self, id: Id, item: &str, help: &str) -> Self {
        self.actions.push(MenuAction::AppendItem {
            id,
            item: item.to_string(),
            help: help.to_string(),
            kind: ItemKind::Normal,
        });
        self
    }

    /// Adds a check item to be appended to the menu.
    pub fn append_check_item(mut self, id: Id, item: &str, help: &str) -> Self {
        self.actions.push(MenuAction::AppendItem {
            id,
            item: item.to_string(),
            help: help.to_string(),
            kind: ItemKind::Check,
        });
        self
    }

    /// Adds a radio item to be appended to the menu.
    pub fn append_radio_item(mut self, id: Id, item: &str, help: &str) -> Self {
        self.actions.push(MenuAction::AppendItem {
            id,
            item: item.to_string(),
            help: help.to_string(),
            kind: ItemKind::Radio,
        });
        self
    }

    /// Adds a separator to be appended to the menu.
    pub fn append_separator(mut self) -> Self {
        self.actions.push(MenuAction::AppendSeparator);
        self
    }

    /// Builds the `Menu`.
    ///
    /// # Panics
    /// Panics if the menu cannot be created.
    pub fn build(self) -> Menu {
        // Pass default style (0)
        let title_c = CString::new(self.title).unwrap();
        let style = 0i64;
        let ptr = unsafe { ffi::wxd_Menu_Create(title_c.as_ptr(), style as ffi::wxd_Style_t) };
        if ptr.is_null() {
            panic!("Failed to create Menu");
        }
        let menu = Menu {
            ptr,
            owned: true,
            _nosend_nosync: PhantomData,
        };

        // Perform actions
        for action in self.actions {
            match action {
                MenuAction::AppendItem { id, item, help, kind } => {
                    // We might ignore the returned MenuItem here, as the builder doesn't expose it.
                    let _ = menu.append_raw(id, &item, &help, kind);
                }
                MenuAction::AppendSeparator => {
                    menu.append_separator_raw();
                }
            }
        }
        menu
    }
}

// Add XRC support
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for Menu {
    unsafe fn from_xrc_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
        let ptr = ptr as *mut wxdragon_sys::wxd_Menu_t;
        // Menus loaded via XRC are owned by their parent (e.g., MenuBar/Frame),
        // so this wrapper must NOT destroy them on drop.
        Self {
            ptr,
            owned: false,
            _nosend_nosync: PhantomData,
        }
    }
}

// Implement WxWidget for Menu (needed for XRC support)
impl crate::window::WxWidget for Menu {
    fn handle_ptr(&self) -> *mut wxdragon_sys::wxd_Window_t {
        self.ptr as *mut wxdragon_sys::wxd_Window_t
    }

    fn get_id(&self) -> i32 {
        -1 // Menus don't typically have IDs
    }
}

// --- Menu specific event enum ---
/// Events specific to Menu controls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuEvent {
    /// Fired when an item is selected
    Selected,
}

/// Event data for Menu events
#[derive(Debug)]
pub struct MenuEventData {
    pub event: CommandEventData,
}

impl MenuEventData {
    pub fn new(event: Event) -> Self {
        Self {
            event: CommandEventData::new(event),
        }
    }

    /// Get the widget ID that generated the event
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }
}

impl crate::event::WxEvtHandler for Menu {
    unsafe fn get_event_handler_ptr(&self) -> *mut wxdragon_sys::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

// At the bottom of the file, use the local macro
crate::implement_widget_local_event_handlers!(
    Menu,
    MenuEvent,
    MenuEventData,
    Selected => selected, EventType::MENU
);
