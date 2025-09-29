//! wxMenuBar wrapper

use crate::menus::Menu;
#[cfg(feature = "xrc")]
use crate::menus::MenuItem;
#[cfg(feature = "xrc")]
use crate::window::Window;
#[cfg(feature = "xrc")]
use crate::xrc::XmlResource;
use std::ffi::CString;
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
    pub fn get_item_by_name(&self, parent_window: &Window, item_name: &str) -> Option<MenuItem> {
        MenuItem::from_xrc_name(parent_window, item_name)
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
            let menu_ptr = unsafe { menu.as_ptr() };
            // MenuBar takes ownership of the menu pointer, but Menu doesn't implement Drop
            // so no need to forget it
            unsafe {
                ffi::wxd_MenuBar_Append(menubar.ptr, menu_ptr, title_c.as_ptr());
            }
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
