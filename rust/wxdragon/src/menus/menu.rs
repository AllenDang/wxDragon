//! wxMenu wrapper

use crate::id::Id;
use crate::menus::menuitem::{ItemKind, MenuItem};
#[cfg(feature = "xrc")]
use crate::window::Window;
#[cfg(feature = "xrc")]
use crate::xrc::XmlResource;
use std::ffi::CString;
use std::marker::PhantomData;
use wxdragon_sys as ffi;

/// Represents a wxMenu.
/// Note: Ownership is typically transferred to the MenuBar when Append is called.
pub struct Menu {
    ptr: *mut ffi::wxd_Menu_t,
}

impl Menu {
    /// Creates a new, empty menu using the builder pattern.
    pub fn builder() -> MenuBuilder {
        MenuBuilder::new()
    }

    /// Appends a menu item.
    /// Returns a wrapper for the created item (for potential modification), but ownership remains with the menu.
    pub fn append(
        &self,
        id: Id,
        item: &str,
        help_string: &str,
        kind: ItemKind,
    ) -> Option<MenuItem> {
        self.append_raw(id, item, help_string, kind)
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

    /// Creates a Menu wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid wxMenu pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Menu_t) -> Self {
        Self { ptr }
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

    /// Returns the raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is used correctly.
    pub(crate) unsafe fn as_ptr(&self) -> *mut ffi::wxd_Menu_t {
        self.ptr
    }

    // Make append private as it's called by builder
    fn append_raw(
        &self,
        id: Id,
        item: &str,
        help_string: &str,
        kind: ItemKind,
    ) -> Option<MenuItem> {
        let item_c = CString::new(item).unwrap_or_default();
        let help_c = CString::new(help_string).unwrap_or_default();
        let item_ptr = unsafe {
            ffi::wxd_Menu_Append(self.ptr, id, item_c.as_ptr(), help_c.as_ptr(), kind.into())
        };
        if item_ptr.is_null() {
            None
        } else {
            // Return a MenuItem wrapper, but don't give it ownership
            Some(unsafe { MenuItem::from_ptr(item_ptr) })
        }
    }

    // Make append_separator private as it's called by builder
    fn append_separator_raw(&self) {
        unsafe {
            ffi::wxd_Menu_AppendSeparator(self.ptr);
        }
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
        let menu = Menu { ptr };

        // Perform actions
        for action in self.actions {
            match action {
                MenuAction::AppendItem {
                    id,
                    item,
                    help,
                    kind,
                } => {
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
        let menu_ptr = ptr as *mut wxdragon_sys::wxd_Menu_t;
        Self { ptr: menu_ptr }
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
