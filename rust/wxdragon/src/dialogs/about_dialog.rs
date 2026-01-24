use crate::bitmap::Bitmap;
use crate::window::WxWidget;
use std::ffi::CString;
use wxdragon_sys as ffi;

/// Information to be shown in the about dialog.
///
/// This struct holds all the information that can be displayed in an about dialog,
/// including the application name, version, description, copyright notice,
/// license text, developers, artists, translators, and more.
///
/// # Example
/// ```ignore
/// use wxdragon::{AboutDialogInfo, show_about_box};
///
/// let mut info = AboutDialogInfo::new();
/// info.set_name("My Application");
/// info.set_version("1.0.0");
/// info.set_description("A wonderful application.");
/// info.set_copyright("(c) 2024 My Company");
/// info.add_developer("John Doe");
/// info.add_developer("Jane Smith");
///
/// show_about_box(&info, Some(&frame));
/// ```
pub struct AboutDialogInfo {
    ptr: *mut ffi::wxd_AboutDialogInfo_t,
}

impl AboutDialogInfo {
    /// Creates a new empty AboutDialogInfo.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_AboutDialogInfo_Create() };
        if ptr.is_null() {
            panic!("Failed to create AboutDialogInfo");
        }
        AboutDialogInfo { ptr }
    }

    /// Sets the name of the application.
    pub fn set_name(&mut self, name: &str) {
        let c_name = CString::new(name).expect("CString::new failed");
        unsafe {
            ffi::wxd_AboutDialogInfo_SetName(self.ptr, c_name.as_ptr());
        }
    }

    /// Sets the version string of the application.
    pub fn set_version(&mut self, version: &str) {
        let c_version = CString::new(version).expect("CString::new failed");
        unsafe {
            ffi::wxd_AboutDialogInfo_SetVersion(self.ptr, c_version.as_ptr());
        }
    }

    /// Sets both short and long version strings.
    ///
    /// The short version is typically just the version number (e.g., "1.0.0"),
    /// while the long version can include additional information
    /// (e.g., "Version 1.0.0 (Build 1234)").
    pub fn set_version_ex(&mut self, version: &str, long_version: &str) {
        let c_version = CString::new(version).expect("CString::new failed");
        let c_long = CString::new(long_version).expect("CString::new failed");
        unsafe {
            ffi::wxd_AboutDialogInfo_SetVersionEx(self.ptr, c_version.as_ptr(), c_long.as_ptr());
        }
    }

    /// Sets the description of the application.
    ///
    /// This should be a brief description of what the application does.
    pub fn set_description(&mut self, description: &str) {
        let c_desc = CString::new(description).expect("CString::new failed");
        unsafe {
            ffi::wxd_AboutDialogInfo_SetDescription(self.ptr, c_desc.as_ptr());
        }
    }

    /// Sets the copyright notice.
    ///
    /// Example: "(c) 2024 My Company"
    pub fn set_copyright(&mut self, copyright: &str) {
        let c_copyright = CString::new(copyright).expect("CString::new failed");
        unsafe {
            ffi::wxd_AboutDialogInfo_SetCopyright(self.ptr, c_copyright.as_ptr());
        }
    }

    /// Sets the license text.
    ///
    /// This can be the full license text or a summary of the license.
    /// Note: This method is also available as `set_license` (American spelling).
    pub fn set_licence(&mut self, licence: &str) {
        let c_licence = CString::new(licence).expect("CString::new failed");
        unsafe {
            ffi::wxd_AboutDialogInfo_SetLicence(self.ptr, c_licence.as_ptr());
        }
    }

    /// Sets the license text (American spelling).
    ///
    /// This is an alias for `set_licence`.
    pub fn set_license(&mut self, license: &str) {
        self.set_licence(license);
    }

    /// Sets the icon for the about dialog.
    pub fn set_icon(&mut self, bitmap: &Bitmap) {
        unsafe {
            ffi::wxd_AboutDialogInfo_SetIcon(self.ptr, bitmap.as_const_ptr());
        }
    }

    /// Sets the website URL.
    pub fn set_website(&mut self, url: &str) {
        let c_url = CString::new(url).expect("CString::new failed");
        unsafe {
            ffi::wxd_AboutDialogInfo_SetWebSite(self.ptr, c_url.as_ptr());
        }
    }

    /// Sets the website URL with a custom description.
    ///
    /// The description is the text that will be shown as the link text.
    pub fn set_website_ex(&mut self, url: &str, description: &str) {
        let c_url = CString::new(url).expect("CString::new failed");
        let c_desc = CString::new(description).expect("CString::new failed");
        unsafe {
            ffi::wxd_AboutDialogInfo_SetWebSiteEx(self.ptr, c_url.as_ptr(), c_desc.as_ptr());
        }
    }

    /// Adds a developer to the list of developers.
    pub fn add_developer(&mut self, developer: &str) {
        let c_dev = CString::new(developer).expect("CString::new failed");
        unsafe {
            ffi::wxd_AboutDialogInfo_AddDeveloper(self.ptr, c_dev.as_ptr());
        }
    }

    /// Adds a documentation writer to the list.
    pub fn add_doc_writer(&mut self, doc_writer: &str) {
        let c_doc = CString::new(doc_writer).expect("CString::new failed");
        unsafe {
            ffi::wxd_AboutDialogInfo_AddDocWriter(self.ptr, c_doc.as_ptr());
        }
    }

    /// Adds an artist to the list of artists.
    pub fn add_artist(&mut self, artist: &str) {
        let c_artist = CString::new(artist).expect("CString::new failed");
        unsafe {
            ffi::wxd_AboutDialogInfo_AddArtist(self.ptr, c_artist.as_ptr());
        }
    }

    /// Adds a translator to the list of translators.
    pub fn add_translator(&mut self, translator: &str) {
        let c_trans = CString::new(translator).expect("CString::new failed");
        unsafe {
            ffi::wxd_AboutDialogInfo_AddTranslator(self.ptr, c_trans.as_ptr());
        }
    }

    /// Returns the raw pointer to the underlying wxAboutDialogInfo.
    pub fn as_ptr(&self) -> *const ffi::wxd_AboutDialogInfo_t {
        self.ptr
    }
}

impl Default for AboutDialogInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AboutDialogInfo {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::wxd_AboutDialogInfo_Destroy(self.ptr);
            }
        }
    }
}

/// Shows an about dialog with the given information.
///
/// This function displays a standard about dialog with the information
/// provided in the `AboutDialogInfo` struct.
///
/// # Arguments
/// * `info` - The information to display in the dialog.
/// * `parent` - Optional parent window. If `None`, the dialog will be shown
///   without a parent (top-level).
///
/// # Example
/// ```ignore
/// use wxdragon::{AboutDialogInfo, show_about_box};
///
/// let mut info = AboutDialogInfo::new();
/// info.set_name("My App");
/// info.set_version("1.0");
///
/// show_about_box(&info, Some(&frame));
/// ```
pub fn show_about_box(info: &AboutDialogInfo, parent: Option<&dyn WxWidget>) {
    let parent_ptr = parent.map(|p| p.handle_ptr()).unwrap_or(std::ptr::null_mut());
    unsafe {
        ffi::wxd_AboutBox(info.as_ptr(), parent_ptr);
    }
}
