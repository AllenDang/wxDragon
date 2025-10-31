//!
//! Provides access to wxArtProvider for stock art (icons, bitmaps).

use crate::bitmap::Bitmap;
use crate::bitmap_bundle::BitmapBundle;
use crate::geometry::Size;
use crate::window::WxWidget;
use std::ffi::CString;
use wxdragon_sys as ffi;

// --- ArtId Enum ---
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ArtId {
    Error,
    Information,
    Warning,
    Question,
    HelpSidePanel,
    HelpSettings,
    HelpBook,
    HelpFolder,
    HelpPage,
    AddBookmark,
    DeleteBookmark,
    GoBack,
    GoForward,
    GoUp,
    GoDown,
    GoToParent,
    GoHome,
    GoToFirst,
    GoToLast,
    FileOpen,
    FileSave,
    FileSaveAs,
    Print,
    Help,
    Tip,
    ReportView,
    ListView,
    NewDir,
    HardDisk,
    Floppy,
    CDRom,
    Removable,
    Folder,
    FolderOpen,
    GoDirUp,
    ExecutableFile,
    NormalFile,
    TickMark,
    CrossMark,
    MissingImage,
    New,
    Edit,
    Undo,
    Redo,
    Delete,
    Copy,
    Cut,
    Paste,
    Find,
    FindAndReplace,
    Quit,
    // Add any other specific IDs if they appear in ffi or wxWidgets docs
}

impl ArtId {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArtId::Error => "wxART_ERROR",
            ArtId::Information => "wxART_INFORMATION",
            ArtId::Warning => "wxART_WARNING",
            ArtId::Question => "wxART_QUESTION",
            ArtId::HelpSidePanel => "wxART_HELP_SIDE_PANEL",
            ArtId::HelpSettings => "wxART_HELP_SETTINGS",
            ArtId::HelpBook => "wxART_HELP_BOOK",
            ArtId::HelpFolder => "wxART_HELP_FOLDER",
            ArtId::HelpPage => "wxART_HELP_PAGE",
            ArtId::AddBookmark => "wxART_ADD_BOOKMARK",
            ArtId::DeleteBookmark => "wxART_DEL_BOOKMARK",
            ArtId::GoBack => "wxART_GO_BACK",
            ArtId::GoForward => "wxART_GO_FORWARD",
            ArtId::GoUp => "wxART_GO_UP",
            ArtId::GoDown => "wxART_GO_DOWN",
            ArtId::GoToParent => "wxART_GO_TO_PARENT",
            ArtId::GoHome => "wxART_GO_HOME",
            ArtId::GoToFirst => "wxART_GOTO_FIRST",
            ArtId::GoToLast => "wxART_GOTO_LAST",
            ArtId::FileOpen => "wxART_FILE_OPEN",
            ArtId::FileSave => "wxART_FILE_SAVE",
            ArtId::FileSaveAs => "wxART_FILE_SAVE_AS",
            ArtId::Print => "wxART_PRINT",
            ArtId::Help => "wxART_HELP",
            ArtId::Tip => "wxART_TIP",
            ArtId::ReportView => "wxART_REPORT_VIEW",
            ArtId::ListView => "wxART_LIST_VIEW",
            ArtId::NewDir => "wxART_NEW_DIR",
            ArtId::HardDisk => "wxART_HARDDISK",
            ArtId::Floppy => "wxART_FLOPPY",
            ArtId::CDRom => "wxART_CDROM",
            ArtId::Removable => "wxART_REMOVABLE",
            ArtId::Folder => "wxART_FOLDER",
            ArtId::FolderOpen => "wxART_FOLDER_OPEN",
            ArtId::GoDirUp => "wxART_GO_DIR_UP",
            ArtId::ExecutableFile => "wxART_EXECUTABLE_FILE",
            ArtId::NormalFile => "wxART_NORMAL_FILE",
            ArtId::TickMark => "wxART_TICK_MARK",
            ArtId::CrossMark => "wxART_CROSS_MARK",
            ArtId::MissingImage => "wxART_MISSING_IMAGE",
            ArtId::New => "wxART_NEW",
            ArtId::Edit => "wxART_EDIT",
            ArtId::Undo => "wxART_UNDO",
            ArtId::Redo => "wxART_REDO",
            ArtId::Delete => "wxART_DELETE",
            ArtId::Copy => "wxART_COPY",
            ArtId::Cut => "wxART_CUT",
            ArtId::Paste => "wxART_PASTE",
            ArtId::Find => "wxART_FIND",
            ArtId::FindAndReplace => "wxART_FIND_AND_REPLACE",
            ArtId::Quit => "wxART_QUIT",
        }
    }
}

// --- ArtClient Enum ---
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ArtClient {
    Button,
    Menu,
    Toolbar,
    FrameIcon,
    MessageBox,
    Dialog,
    Other, // Represents wxART_OTHER, typically an empty string
}

impl ArtClient {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArtClient::Button => "wxART_BUTTON_C",
            ArtClient::Menu => "wxART_MENU_C",
            ArtClient::Toolbar => "wxART_TOOLBAR_C",
            ArtClient::FrameIcon => "wxART_FRAME_ICON_C",
            ArtClient::MessageBox => "wxART_MESSAGE_BOX_C",
            ArtClient::Dialog => "wxART_DIALOG_C",
            ArtClient::Other => "", // Empty string for wxART_OTHER
        }
    }
}

/// Provides static methods to access stock art (bitmaps/icons).
pub struct ArtProvider;

impl ArtProvider {
    /// Retrieves a stock bitmap.
    ///
    /// # Arguments
    /// * `id` - The art ID (e.g., `ART_FILE_OPEN`, `ART_ERROR`). These are string constants.
    /// * `client_id` - The client for which the bitmap is being requested.
    ///   This might affect the expected size or resolution.
    ///   An empty string can be used for `wxART_OTHER`.
    /// * `size` - Optional desired size. If `None`, default size is requested (`-1, -1`).
    ///
    /// Returns `Option<Bitmap>`. The bitmap is `None` if not found or an error occurs.
    /// The caller owns the returned `Bitmap` and is responsible for its `Drop`.
    pub fn get_bitmap(id: ArtId, client: ArtClient, size: Option<Size>) -> Option<Bitmap> {
        let c_id = match CString::new(id.as_str()) {
            Ok(s) => s,
            Err(_) => return None, // Should not happen with enum if as_str is correct
        };
        let c_client = match CString::new(client.as_str()) {
            Ok(s) => s,
            Err(_) => return None, // Should not happen with enum
        };

        let ffi_size = size.map_or_else(|| ffi::wxd_Size { width: -1, height: -1 }, |s| s.into());

        let bitmap_ptr = unsafe { ffi::wxd_ArtProvider_GetBitmap(c_id.as_ptr(), c_client.as_ptr(), ffi_size) };

        if bitmap_ptr.is_null() {
            None
        } else {
            // The bitmap is created by wxWidgets and ownership is transferred to Rust
            Some(Bitmap::from(bitmap_ptr))
        }
    }

    /// Retrieves a stock bitmap bundle with multiple resolutions.
    ///
    /// This is preferred over get_bitmap for better high-DPI support.
    ///
    /// # Arguments
    /// * `id` - The art ID (e.g., `ART_FILE_OPEN`, `ART_ERROR`).
    /// * `client` - The art client ID (e.g., `ART_BUTTON`, `ART_MENU`).
    /// * `size` - Optional desired size. If `None`, default size is requested (`-1, -1`).
    ///
    /// Returns `Option<BitmapBundle>` which can render the icon/bitmap at various DPI scales.
    /// Returns `None` if no matching art is found or an error occurs.
    pub fn get_bitmap_bundle(id: ArtId, client: ArtClient, size: Option<Size>) -> Option<BitmapBundle> {
        let c_id = match CString::new(id.as_str()) {
            Ok(s) => s,
            Err(_) => return None,
        };
        let c_client = match CString::new(client.as_str()) {
            Ok(s) => s,
            Err(_) => return None,
        };

        let ffi_size = size.map_or_else(|| ffi::wxd_Size { width: -1, height: -1 }, |s| s.into());

        let bundle_ptr = unsafe { ffi::wxd_ArtProvider_GetBitmapBundle(c_id.as_ptr(), c_client.as_ptr(), ffi_size) };

        if bundle_ptr.is_null() {
            None
        } else {
            Some(BitmapBundle::from_ptr_owned(bundle_ptr))
        }
    }

    /// Returns a suitable size hint for the given client in device-independent pixels (DIPs).
    ///
    /// # Arguments
    /// * `client` - The art client to get the size hint for.
    ///
    /// Returns the size in DIPs that the topmost art provider recommends for the given client.
    pub fn get_dip_size_hint(client: ArtClient) -> Size {
        let c_client = match CString::new(client.as_str()) {
            Ok(s) => s,
            Err(_) => return Size::new(0, 0),
        };

        let size = unsafe { ffi::wxd_ArtProvider_GetDIPSizeHint(c_client.as_ptr()) };
        Size::from(size)
    }

    /// Returns a suitable size hint for the given client.
    ///
    /// # Arguments
    /// * `client` - The art client to get the size hint for.
    /// * `window` - Optional window to scale the size for. If `None`, uses the default display scale.
    ///
    /// Returns the size (scaled for the display or window) that the art provider recommends.
    pub fn get_size_hint(client: ArtClient, window: Option<&dyn WxWidget>) -> Size {
        let c_client = match CString::new(client.as_str()) {
            Ok(s) => s,
            Err(_) => return Size::new(0, 0),
        };

        let window_ptr = window.map_or(std::ptr::null_mut(), |w| w.handle_ptr());

        let size = unsafe { ffi::wxd_ArtProvider_GetSizeHint(c_client.as_ptr(), window_ptr) };
        Size::from(size)
    }

    /// Returns the native icon size for the specified client in DIPs.
    ///
    /// # Arguments
    /// * `client` - The art client to get the native size for.
    ///
    /// Returns the platform's native size for the specified client in DIPs, or a default size
    /// if there is no commonly used default or the client is not recognized.
    pub fn get_native_dip_size_hint(client: ArtClient) -> Size {
        let c_client = match CString::new(client.as_str()) {
            Ok(s) => s,
            Err(_) => return Size::new(0, 0),
        };

        let size = unsafe { ffi::wxd_ArtProvider_GetNativeDIPSizeHint(c_client.as_ptr()) };
        Size::from(size)
    }

    /// Returns the native icon size for the specified client, scaled for a window.
    ///
    /// # Arguments
    /// * `client` - The art client to get the native size for.
    /// * `window` - Optional window to scale the size for. If `None`, uses the default display scale.
    ///
    /// Returns the platform's native size for the specified client, scaled appropriately for
    /// the display or window.
    pub fn get_native_size_hint(client: ArtClient, window: Option<&dyn WxWidget>) -> Size {
        let c_client = match CString::new(client.as_str()) {
            Ok(s) => s,
            Err(_) => return Size::new(0, 0),
        };

        let window_ptr = window.map_or(std::ptr::null_mut(), |w| w.handle_ptr());

        let size = unsafe { ffi::wxd_ArtProvider_GetNativeSizeHint(c_client.as_ptr(), window_ptr) };
        Size::from(size)
    }

    /// Returns true if the platform uses native icons provider that should take
    /// precedence over any customizations.
    ///
    /// This is true for any platform that has user-customizable icon themes,
    /// such as GTK.
    pub fn has_native_provider() -> bool {
        unsafe { ffi::wxd_ArtProvider_HasNativeProvider() }
    }
}

// All old pub const ART_XXX definitions below this point should be removed.
// The edit tool started this process but didn't complete it.
// The following is a placeholder to indicate all subsequent lines of old constants are gone.

// End of file (effectively)
