//! wxMenuBar wrapper

use crate::id::Id;
use crate::menus::{Menu, MenuItem};
#[cfg(feature = "xrc")]
use crate::window::WindowHandle;
#[cfg(feature = "xrc")]
use crate::xrc::XmlResource;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use wxdragon_sys as ffi;

/// Represents a wxMenuBar.
/// Note: Ownership is typically transferred to the Frame when SetMenuBar is called.
pub struct MenuBar {
    ptr: *mut ffi::wxd_MenuBar_t,
}

impl MenuBar {
    /// Creates a new menu bar using the builder pattern.
    pub fn builder() -> MenuBarBuilder {
        MenuBarBuilder::new()
    }

    /// Creates a MenuBar wrapper from a raw pointer (for XRC loading).
    /// # Safety
    /// The pointer must be a valid wxMenuBar pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_MenuBar_t) -> Self {
        Self { ptr }
    }

    /// Gets a menu item by its XRC name from any menu in this menubar.
    /// Returns a MenuItem wrapper that can be used for event binding.
    #[cfg(feature = "xrc")]
    pub fn get_item_by_name(&self, parent_handle: WindowHandle, item_name: &str) -> Option<MenuItem> {
        MenuItem::from_xrc_name(parent_handle, item_name)
    }

    /// Special XRC loading method for menubars.
    /// This looks up the menubar by name and creates a MenuBar wrapper.
    #[cfg(feature = "xrc")]
    pub fn from_xrc_name(menubar_name: &str) -> Option<Self> {
        // Get the XRC resource and try to load the menubar
        let xml_resource = XmlResource::get();

        // Try to load the menubar from XRC
        let name_c = CString::new(menubar_name).unwrap_or_default();
        let ptr = unsafe {
            ffi::wxd_XmlResource_LoadMenuBar(
                xml_resource.as_ptr(),
                std::ptr::null_mut(), // parent (null for menubar)
                name_c.as_ptr(),
            )
        };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { MenuBar::from_ptr(ptr) })
        }
    }

    /// Returns the raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is used correctly.
    /// NOTE: This is needed internally by Frame::set_menu_bar.
    pub(crate) unsafe fn as_ptr(&self) -> *mut ffi::wxd_MenuBar_t {
        self.ptr
    }

    /// Enables or disables a menu item by its ID across this menu bar.
    /// Returns true if the item was found and its state changed.
    pub fn enable_item(&self, id: Id, enable: bool) -> bool {
        unsafe { ffi::wxd_MenuBar_EnableItem(self.ptr, id, enable) }
    }

    /// Checks if a menu item is enabled via this menu bar.
    pub fn is_item_enabled(&self, id: Id) -> bool {
        unsafe { ffi::wxd_MenuBar_IsItemEnabled(self.ptr, id) }
    }

    /// Checks or unchecks a menu item by its ID across this menu bar.
    pub fn check_item(&self, id: Id, check: bool) {
        unsafe { ffi::wxd_MenuBar_CheckItem(self.ptr, id, check) }
    }

    /// Checks if a menu item is checked via this menu bar.
    pub fn is_item_checked(&self, id: Id) -> bool {
        unsafe { ffi::wxd_MenuBar_IsItemChecked(self.ptr, id) }
    }

    /// Finds a menu item by its ID.
    /// Returns the found MenuItem or None if not found.
    pub fn find_item(&self, id: Id) -> Option<MenuItem> {
        let item_ptr = unsafe { ffi::wxd_MenuBar_FindItem(self.ptr, id, std::ptr::null_mut()) };

        if item_ptr.is_null() {
            None
        } else {
            // MenuItem::from_ptr creates a non-owning wrapper
            Some(MenuItem::from_ptr(item_ptr))
        }
    }

    /// Finds a menu item by its ID and returns both the item and the menu it belongs to.
    /// Returns (MenuItem, Menu) or None if not found.
    pub fn find_item_and_menu(&self, id: Id) -> Option<(MenuItem, Menu)> {
        let mut menu_ptr: *mut ffi::wxd_Menu_t = std::ptr::null_mut();
        let item_ptr = unsafe { ffi::wxd_MenuBar_FindItem(self.ptr, id, &mut menu_ptr) };

        if item_ptr.is_null() || menu_ptr.is_null() {
            None
        } else {
            // Both MenuItem and Menu are returned as non-owning wrappers here
            // because they are owned by the MenuBar structure.
            Some((MenuItem::from_ptr(item_ptr), Menu::from(menu_ptr as *const ffi::wxd_Menu_t)))
        }
    }

    /// Returns the menu at the specified index.
    pub fn get_menu(&self, index: usize) -> Option<Menu> {
        let ptr = unsafe { ffi::wxd_MenuBar_GetMenu(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            // Return non-owning wrapper
            Some(Menu::from(ptr as *const ffi::wxd_Menu_t))
        }
    }

    /// Returns the number of menus in the menu bar.
    pub fn get_menu_count(&self) -> usize {
        unsafe { ffi::wxd_MenuBar_GetMenuCount(self.ptr) }
    }

    /// Returns the index of the menu with the given title or NOT_FOUND if not found.
    pub fn find_menu(&self, title: &str) -> i32 {
        let c_title = CString::new(title).unwrap_or_default();
        unsafe { ffi::wxd_MenuBar_FindMenu(self.ptr, c_title.as_ptr()) }
    }

    /// Enables or disables the menu at the given position.
    pub fn enable_top(&self, pos: usize, enable: bool) {
        unsafe { ffi::wxd_MenuBar_EnableTop(self.ptr, pos, enable) }
    }

    /// Gets the label of the menu at the given position.
    pub fn get_menu_label(&self, pos: usize) -> String {
        let len = unsafe { ffi::wxd_MenuBar_GetMenuLabel(self.ptr, pos, std::ptr::null_mut(), 0) };
        if len <= 0 {
            return String::new();
        }
        let mut buffer = vec![0u8; len as usize + 1];
        unsafe { ffi::wxd_MenuBar_GetMenuLabel(self.ptr, pos, buffer.as_mut_ptr() as *mut _, buffer.len()) };
        unsafe { CStr::from_ptr(buffer.as_ptr() as *const _).to_string_lossy().into_owned() }
    }

    /// Sets the label of the menu at the given position.
    pub fn set_menu_label(&self, pos: usize, label: &str) {
        let c_label = CString::new(label).unwrap_or_default();
        unsafe { ffi::wxd_MenuBar_SetMenuLabel(self.ptr, pos, c_label.as_ptr()) }
    }

    /// Replaces the menu at the given position with another one.
    /// The MenuBar takes ownership of the new menu.
    /// Returns the old menu (which is now owned by the caller) or None on failure.
    pub fn replace(&self, pos: usize, menu: Menu, title: &str) -> Option<Menu> {
        let c_title = CString::new(title).unwrap_or_default();
        // MenuBar takes ownership, so we use into_raw_mut
        let ptr = unsafe { ffi::wxd_MenuBar_Replace(self.ptr, pos, menu.into_raw_mut(), c_title.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            // The old menu is detached, so we own it now.
            // Menu::from(*mut) sets owned=true.
            Some(Menu::from(ptr))
        }
    }
}

// Note: No Drop impl here, as wxFrame takes ownership via SetMenuBar.

// --- MenuBar Builder ---

/// Builder for [`MenuBar`].
#[derive(Default)]
pub struct MenuBarBuilder {
    style: i64,
    items: Vec<(Menu, String)>, // Store menu and title pairs
    _marker: PhantomData<()>,   // Optional: for generics later?
}

impl MenuBarBuilder {
    /// Creates a new, default builder.
    pub fn new() -> Self {
        Default::default() // style = 0, items = empty vec
    }

    /// Sets the style flags for the menu bar.
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Appends a menu to be added to the menu bar.
    /// Takes ownership of the `Menu` object.
    pub fn append(mut self, menu: Menu, title: &str) -> Self {
        self.items.push((menu, title.to_string()));
        self
    }

    /// Builds the `MenuBar`.
    ///
    /// # Panics
    /// Panics if the menu bar cannot be created.
    pub fn build(self) -> MenuBar {
        let ptr = unsafe { ffi::wxd_MenuBar_Create(self.style as ffi::wxd_Style_t) };
        if ptr.is_null() {
            panic!("Failed to create MenuBar");
        }
        let menubar = MenuBar { ptr };

        // Append all collected menus
        for (menu, title) in self.items {
            let title_c = CString::new(title).unwrap_or_default();
            // MenuBar takes ownership of the menu pointer
            unsafe { ffi::wxd_MenuBar_Append(menubar.ptr, menu.into_raw_mut(), title_c.as_ptr()) };
        }

        menubar
    }
}

// Add XRC support
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for MenuBar {
    unsafe fn from_xrc_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
        let menubar_ptr = ptr as *mut wxdragon_sys::wxd_MenuBar_t;
        Self { ptr: menubar_ptr }
    }
}

// Implement WxWidget for MenuBar (needed for XRC support)
impl crate::window::WxWidget for MenuBar {
    fn handle_ptr(&self) -> *mut wxdragon_sys::wxd_Window_t {
        self.ptr as *mut wxdragon_sys::wxd_Window_t
    }

    fn get_id(&self) -> i32 {
        -1 // MenuBars don't typically have IDs
    }
}

// Implement WxEvtHandler for MenuBar to enable event handling
impl crate::event::WxEvtHandler for MenuBar {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement MenuEvents trait for MenuBar
impl crate::event::MenuEvents for MenuBar {}
