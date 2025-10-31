//! wxMenu wrapper

use crate::id::Id;
use crate::menus::menuitem::{ItemKind, MenuItem};
#[cfg(feature = "xrc")]
use crate::window::Window;
#[cfg(feature = "xrc")]
use crate::xrc::XmlResource;
use crate::{CommandEventData, Event, EventType};
use std::ffi::CString;
use std::marker::PhantomData;
use wxdragon_sys as ffi;

/// Represents a wxMenu.
pub struct Menu {
    ptr: *mut ffi::wxd_Menu_t,
    owned: bool,
}

impl Drop for Menu {
    fn drop(&mut self) {
        self.destroy_meun();
    }
}

impl From<*mut ffi::wxd_Menu_t> for Menu {
    fn from(ptr: *mut ffi::wxd_Menu_t) -> Self {
        Menu { ptr, owned: true }
    }
}

impl From<*const ffi::wxd_Menu_t> for Menu {
    fn from(ptr: *const ffi::wxd_Menu_t) -> Self {
        Menu {
            ptr: ptr as *mut ffi::wxd_Menu_t,
            owned: false,
        }
    }
}

impl AsRef<*mut ffi::wxd_Menu_t> for Menu {
    fn as_ref(&self) -> &*mut ffi::wxd_Menu_t {
        &self.ptr
    }
}

impl std::ops::Deref for Menu {
    type Target = *mut ffi::wxd_Menu_t;
    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl Menu {
    /// Creates a new, empty menu using the builder pattern.
    pub fn builder() -> MenuBuilder {
        MenuBuilder::new()
    }

    /// Gets the number of items in the menu.
    pub fn get_item_count(&self) -> usize {
        unsafe { ffi::wxd_Menu_GetMenuItemCount(self.ptr) }
    }

    /// Gets the title of the menu.
    pub fn get_title(&self) -> String {
        // First, get the required buffer size
        let size = unsafe { ffi::wxd_Menu_GetTitle(self.ptr, std::ptr::null_mut(), 0) };
        if size == 0 {
            return String::new();
        }

        // Allocate buffer
        let mut buffer: Vec<u8> = vec![0; size + 1]; // +1 for null terminator

        // Get the title
        unsafe { ffi::wxd_Menu_GetTitle(self.ptr, buffer.as_mut_ptr() as *mut i8, buffer.len()) };

        // Convert to String
        let cstr = unsafe { CString::from_vec_unchecked(buffer) };
        cstr.to_string_lossy().into_owned()
    }

    /// Explicitly destroy this Menu. Use this for standalone/popup menus that are not
    /// appended to a MenuBar. After calling this method, the Menu must not be used.
    ///
    /// Safety: Do NOT call this if the menu was appended to a MenuBar, as the menubar
    /// takes ownership and will delete it, leading to double free.
    pub fn destroy_meun(&mut self) {
        if self.owned && !self.ptr.is_null() {
            log::debug!("Menu '{}' destroyed", self.get_title());
            unsafe { ffi::wxd_Menu_Destroy(self.ptr) };
            self.ptr = std::ptr::null_mut();
        }
    }

    /// Appends a menu item.
    /// Returns a wrapper for the created item (for potential modification), but ownership remains with the menu.
    pub fn append(&self, id: Id, item: &str, help_string: &str, kind: ItemKind) -> Option<MenuItem> {
        self.append_raw(id, item, help_string, kind)
    }

    /// Appends a submenu.
    pub fn append_submenu(&self, submenu: &mut Menu, title: &str, help_string: &str) -> Option<MenuItem> {
        let title_c = CString::new(title).unwrap_or_default();
        let help_c = CString::new(help_string).unwrap_or_default();
        let item_ptr = unsafe { ffi::wxd_Menu_AppendSubMenu(self.ptr, **submenu, title_c.as_ptr(), help_c.as_ptr()) };
        if item_ptr.is_null() {
            return None;
        }
        submenu.relinquish_ownership();
        // Return a MenuItem wrapper, but don't give it ownership
        Some(MenuItem::from(item_ptr))
    }

    /// Appends a separator.
    pub fn append_separator(&self) {
        self.append_separator_raw();
    }

    /// Gets a menu item by its XRC name.
    /// Returns a MenuItem wrapper that can be used for event binding.
    #[cfg(feature = "xrc")]
    pub fn get_item_by_name(&self, parent_window: &Window, item_name: &str) -> Option<MenuItem> {
        MenuItem::from_xrc_name(parent_window, item_name)
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

    /// Relinquish ownership of the underlying wxMenu.
    ///
    /// Use this when transferring ownership to another native owner (e.g. MenuBar::append).
    /// After calling this, Drop will not delete the native wxMenu.
    pub(crate) fn relinquish_ownership(&mut self) {
        self.owned = false;
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
    _marker: PhantomData<()>,
}

impl MenuBuilder {
    /// Creates a new, default builder.
    pub fn new() -> Self {
        Default::default()
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
        // Pass default title (empty string) and default style (0)
        let title_c = CString::new("").unwrap();
        let style = 0i64;
        let ptr = unsafe { ffi::wxd_Menu_Create(title_c.as_ptr(), style as ffi::wxd_Style_t) };
        if ptr.is_null() {
            panic!("Failed to create Menu");
        }
        let menu = Menu { ptr, owned: true };

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
        Self { ptr, owned: false }
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
        **self as *mut ffi::wxd_EvtHandler_t
    }
}

// At the bottom of the file, use the local macro
crate::implement_widget_local_event_handlers!(
    Menu,
    MenuEvent,
    MenuEventData,
    Selected => selected, EventType::MENU
);
