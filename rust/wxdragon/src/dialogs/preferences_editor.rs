//! The platform's own preferences window, wrapping `wxPreferencesEditor`.

use crate::bitmap_bundle::BitmapBundle;
use crate::window::{Window, WxWidget};
use std::ffi::{CString, c_void};
use wxdragon_sys as ffi;

/// A page whose name and icon come from the platform, such as "General" with the macOS gear.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StockPreferencesPage {
    /// The page for the most common settings.
    General,
    /// The page for settings most people don't need.
    Advanced,
}

impl StockPreferencesPage {
    fn to_raw(self) -> i32 {
        match self {
            StockPreferencesPage::General => ffi::WXD_PREFERENCES_PAGE_GENERAL as i32,
            StockPreferencesPage::Advanced => ffi::WXD_PREFERENCES_PAGE_ADVANCED as i32,
        }
    }
}

/// The platform's own preferences window.
///
/// On macOS this is a modeless window with a toolbar of icons, one per page, that is sized to the
/// page shown. On GTK it's a modeless dialog with tabs, and on Windows a modal dialog with tabs and
/// OK and Cancel buttons. Use [`PreferencesEditor::should_apply_changes_immediately`] to know
/// whether a page should save each change as it's made, or wait for the dialog to close.
///
/// Each page's window is built by a closure, which wxWidgets calls with the parent the window
/// must be created in. macOS builds each page once and keeps the window, hidden, when the editor
/// is dismissed, so it reopens as the user left it. The other platforms build a new dialog, and
/// call every closure again, each time the editor is shown.
///
/// The window lives as long as the editor, so where it's modeless, keep the editor somewhere that
/// outlives [`PreferencesEditor::show`], such as the main window's state. Dropping it closes the
/// window and, once wxWidgets deletes the window, drops the closures.
///
/// # Example
/// ```ignore
/// use wxdragon::prelude::*;
///
/// let editor = PreferencesEditor::new("");
/// editor.add_stock_page(StockPreferencesPage::General, |parent| {
///     let panel = Panel::builder(&parent).build();
///     CheckBox::builder(&panel).with_label("Open the last document").build();
///     panel
/// });
/// editor.show(&frame);
/// ```
pub struct PreferencesEditor {
    ptr: *mut ffi::wxd_PreferencesEditor_t,
}

impl PreferencesEditor {
    /// Creates an editor with no pages. An empty title uses wxWidgets' default, which is
    /// "Preferences" on macOS, where macOS 13 and later call the window "Settings".
    pub fn new(title: &str) -> Self {
        let title = CString::new(title).unwrap_or_default();
        let ptr = unsafe { ffi::wxd_PreferencesEditor_Create(title.as_ptr()) };
        assert!(!ptr.is_null(), "Failed to create PreferencesEditor");
        Self { ptr }
    }

    /// Adds a page with its own name and an icon for macOS's toolbar. macOS needs the icon, and
    /// wxWidgets asserts without it; the other platforms don't show it.
    ///
    /// Add every page before the first [`PreferencesEditor::show`]: macOS can't add a page to a
    /// window that's been shown.
    pub fn add_page<F, W>(&self, name: &str, icon: Option<&BitmapBundle>, create: F)
    where
        F: FnMut(Window) -> W + 'static,
        W: WxWidget,
    {
        let name = CString::new(name).unwrap_or_default();
        let icon = icon.map_or(std::ptr::null(), |icon| icon.as_ptr() as *const _);
        let user_data = Box::into_raw(Box::new(create)) as *mut c_void;
        unsafe {
            ffi::wxd_PreferencesEditor_AddPage(
                self.ptr,
                name.as_ptr(),
                icon,
                Some(create_page_trampoline::<F, W>),
                Some(drop_page::<F>),
                user_data,
            )
        };
    }

    /// Adds a page with the platform's name and icon for `kind`.
    pub fn add_stock_page<F, W>(&self, kind: StockPreferencesPage, create: F)
    where
        F: FnMut(Window) -> W + 'static,
        W: WxWidget,
    {
        let user_data = Box::into_raw(Box::new(create)) as *mut c_void;
        unsafe {
            ffi::wxd_PreferencesEditor_AddStockPage(
                self.ptr,
                kind.to_raw(),
                Some(create_page_trampoline::<F, W>),
                Some(drop_page::<F>),
                user_data,
            )
        };
    }

    /// Shows the editor, or brings it to the front if it's already shown. Where it's modal, this
    /// returns when the user closes it.
    pub fn show(&self, parent: &dyn WxWidget) {
        unsafe { ffi::wxd_PreferencesEditor_Show(self.ptr, parent.handle_ptr()) };
    }

    /// Closes the editor if it's shown.
    pub fn dismiss(&self) {
        unsafe { ffi::wxd_PreferencesEditor_Dismiss(self.ptr) };
    }

    /// Whether pages should apply each change as it's made, as on macOS and GTK, rather than when
    /// the user clicks OK.
    pub fn should_apply_changes_immediately() -> bool {
        unsafe { ffi::wxd_PreferencesEditor_ShouldApplyChangesImmediately() }
    }

    /// Whether [`PreferencesEditor::show`] is modal, as on Windows.
    pub fn shown_modally() -> bool {
        unsafe { ffi::wxd_PreferencesEditor_ShownModally() }
    }
}

impl Drop for PreferencesEditor {
    fn drop(&mut self) {
        unsafe { ffi::wxd_PreferencesEditor_Destroy(self.ptr) };
    }
}

unsafe extern "C" fn create_page_trampoline<F, W>(
    user_data: *mut c_void,
    parent: *mut ffi::wxd_Window_t,
) -> *mut ffi::wxd_Window_t
where
    F: FnMut(Window) -> W + 'static,
    W: WxWidget,
{
    crate::utils::guard_ffi_callback("create_page_trampoline", std::ptr::null_mut(), || {
        if user_data.is_null() || parent.is_null() {
            return std::ptr::null_mut();
        }
        let create = unsafe { &mut *(user_data as *mut F) };
        create(unsafe { Window::from_ptr(parent) }).handle_ptr()
    })
}

/// Frees a page's closure. C++ calls this when the editor deletes the page, or right away if the
/// page could not be added.
unsafe extern "C" fn drop_page<F>(user_data: *mut c_void) {
    crate::utils::guard_ffi_callback("drop_page", (), || {
        if !user_data.is_null() {
            let _ = unsafe { Box::from_raw(user_data as *mut F) };
        }
    })
}
