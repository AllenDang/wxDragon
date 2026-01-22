//! Configuration management module for wxDragon.
//!
//! This module provides a safe wrapper around wxWidgets' wxConfigBase class.
//! wxConfig is used to store and retrieve application configuration data
//! in a platform-appropriate manner (registry on Windows, config files elsewhere).
//!
//! # Example
//!
//! ```rust,no_run
//! use wxdragon::config::{Config, ConfigStyle};
//!
//! // Create a config object for your application
//! let config = Config::new("MyApp", Some("MyVendor"), None, None, ConfigStyle::empty());
//!
//! // Write some values
//! config.write_string("LastUser", "John");
//! config.write_long("WindowWidth", 800);
//! config.write_bool("Maximized", false);
//!
//! // Read values back
//! let user = config.read_string("LastUser", "Guest");
//! let width = config.read_long("WindowWidth", 640);
//! let maximized = config.read_bool("Maximized", false);
//!
//! // Use groups/paths to organize settings
//! config.set_path("/UI/Colors");
//! config.write_string("Background", "#FFFFFF");
//! config.write_string("Foreground", "#000000");
//!
//! // Don't forget to flush if you want immediate persistence
//! config.flush(false);
//! ```

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_long};
use wxdragon_sys as ffi;

/// Configuration style flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ConfigStyle(c_long);

impl ConfigStyle {
    /// Empty style (default behavior).
    pub const fn empty() -> Self {
        ConfigStyle(0)
    }

    /// Use local configuration file.
    pub const USE_LOCAL_FILE: ConfigStyle = ConfigStyle(ffi::wxd_ConfigStyle_WXD_CONFIG_USE_LOCAL_FILE as c_long);

    /// Use global configuration file.
    pub const USE_GLOBAL_FILE: ConfigStyle = ConfigStyle(ffi::wxd_ConfigStyle_WXD_CONFIG_USE_GLOBAL_FILE as c_long);

    /// Use relative paths.
    pub const USE_RELATIVE_PATH: ConfigStyle = ConfigStyle(ffi::wxd_ConfigStyle_WXD_CONFIG_USE_RELATIVE_PATH as c_long);

    /// Don't escape special characters.
    pub const USE_NO_ESCAPE_CHARACTERS: ConfigStyle =
        ConfigStyle(ffi::wxd_ConfigStyle_WXD_CONFIG_USE_NO_ESCAPE_CHARACTERS as c_long);

    /// Use subdirectory for config file.
    pub const USE_SUBDIR: ConfigStyle = ConfigStyle(ffi::wxd_ConfigStyle_WXD_CONFIG_USE_SUBDIR as c_long);

    /// Get the raw value.
    pub fn to_raw(self) -> c_long {
        self.0
    }
}

impl std::ops::BitOr for ConfigStyle {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ConfigStyle(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for ConfigStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Configuration entry type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigEntryType {
    /// Unknown type.
    Unknown,
    /// String type.
    String,
    /// Boolean type.
    Boolean,
    /// Integer type.
    Integer,
    /// Float type.
    Float,
}

impl From<i32> for ConfigEntryType {
    fn from(value: i32) -> Self {
        if value == ffi::wxd_ConfigEntryType_WXD_CONFIG_TYPE_STRING as i32 {
            ConfigEntryType::String
        } else if value == ffi::wxd_ConfigEntryType_WXD_CONFIG_TYPE_BOOLEAN as i32 {
            ConfigEntryType::Boolean
        } else if value == ffi::wxd_ConfigEntryType_WXD_CONFIG_TYPE_INTEGER as i32 {
            ConfigEntryType::Integer
        } else if value == ffi::wxd_ConfigEntryType_WXD_CONFIG_TYPE_FLOAT as i32 {
            ConfigEntryType::Float
        } else {
            ConfigEntryType::Unknown
        }
    }
}

/// Configuration object for storing application settings.
///
/// This provides a platform-appropriate way to store configuration data:
/// - On Windows: Uses the registry
/// - On other platforms: Uses configuration files
///
/// The configuration is organized hierarchically using paths similar to
/// a file system, with groups (directories) and entries (files).
pub struct Config {
    ptr: *mut ffi::wxd_ConfigBase_t,
    owned: bool,
}

impl Config {
    /// Creates a new configuration object.
    ///
    /// # Arguments
    ///
    /// * `app_name` - The application name. If empty, uses wxApp::GetAppName().
    /// * `vendor_name` - The vendor name (optional).
    /// * `local_filename` - Local config filename (optional).
    /// * `global_filename` - Global config filename (optional).
    /// * `style` - Configuration style flags.
    pub fn new(
        app_name: &str,
        vendor_name: Option<&str>,
        local_filename: Option<&str>,
        global_filename: Option<&str>,
        style: ConfigStyle,
    ) -> Self {
        let c_app = CString::new(app_name).unwrap_or_default();
        let c_vendor = vendor_name.map(|s| CString::new(s).unwrap_or_default());
        let c_local = local_filename.map(|s| CString::new(s).unwrap_or_default());
        let c_global = global_filename.map(|s| CString::new(s).unwrap_or_default());

        let ptr = unsafe {
            ffi::wxd_Config_Create(
                c_app.as_ptr(),
                c_vendor.as_ref().map_or(std::ptr::null(), |s| s.as_ptr()),
                c_local.as_ref().map_or(std::ptr::null(), |s| s.as_ptr()),
                c_global.as_ref().map_or(std::ptr::null(), |s| s.as_ptr()),
                style.to_raw(),
            )
        };

        Self { ptr, owned: true }
    }

    /// Gets the global configuration object.
    ///
    /// If no global config exists and `create_on_demand` is true, one will be created.
    pub fn get(create_on_demand: bool) -> Option<Self> {
        let ptr = unsafe { ffi::wxd_Config_Get(create_on_demand) };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr, owned: false })
        }
    }

    /// Sets the global configuration object.
    ///
    /// Returns the previous global config if there was one.
    pub fn set(config: Option<Config>) -> Option<Config> {
        let ptr = config.map_or(std::ptr::null_mut(), |c| {
            let p = c.ptr;
            std::mem::forget(c); // Don't drop, ownership transfers
            p
        });

        let prev = unsafe { ffi::wxd_Config_Set(ptr) };
        if prev.is_null() {
            None
        } else {
            Some(Self { ptr: prev, owned: true })
        }
    }

    /// Returns true if the config object is valid.
    pub fn is_ok(&self) -> bool {
        !self.ptr.is_null()
    }

    // --- Path Management ---

    /// Gets the current path.
    pub fn get_path(&self) -> String {
        if self.ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_Config_GetPath(self.ptr, std::ptr::null_mut(), 0) };
        if len < 0 {
            return String::new();
        }
        let mut buf: Vec<c_char> = vec![0; len as usize + 1];
        unsafe { ffi::wxd_Config_GetPath(self.ptr, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() }
    }

    /// Sets the current path.
    ///
    /// If the path starts with '/', it's absolute. Otherwise, it's relative to
    /// the current path. Use ".." to go up one level.
    pub fn set_path(&self, path: &str) {
        if self.ptr.is_null() {
            return;
        }
        let c_path = match CString::new(path) {
            Ok(s) => s,
            Err(_) => return,
        };
        unsafe { ffi::wxd_Config_SetPath(self.ptr, c_path.as_ptr()) };
    }

    // --- Read Operations ---

    /// Reads a string value.
    pub fn read_string(&self, key: &str, default: &str) -> String {
        if self.ptr.is_null() {
            return default.to_string();
        }
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default.to_string(),
        };
        let c_default = CString::new(default).unwrap_or_default();

        // First get the length
        let len = unsafe { ffi::wxd_Config_ReadString(self.ptr, c_key.as_ptr(), std::ptr::null_mut(), 0, c_default.as_ptr()) };
        if len < 0 {
            return default.to_string();
        }
        let mut buf: Vec<c_char> = vec![0; len as usize + 1];
        unsafe { ffi::wxd_Config_ReadString(self.ptr, c_key.as_ptr(), buf.as_mut_ptr(), buf.len(), c_default.as_ptr()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() }
    }

    /// Reads a long integer value.
    pub fn read_long(&self, key: &str, default: i64) -> i64 {
        if self.ptr.is_null() {
            return default;
        }
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        let mut value: c_long = default as c_long;
        unsafe { ffi::wxd_Config_ReadLong(self.ptr, c_key.as_ptr(), &mut value, default as c_long) };
        value as i64
    }

    /// Reads a double value.
    pub fn read_double(&self, key: &str, default: f64) -> f64 {
        if self.ptr.is_null() {
            return default;
        }
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        let mut value: f64 = default;
        unsafe { ffi::wxd_Config_ReadDouble(self.ptr, c_key.as_ptr(), &mut value, default) };
        value
    }

    /// Reads a boolean value.
    pub fn read_bool(&self, key: &str, default: bool) -> bool {
        if self.ptr.is_null() {
            return default;
        }
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        let mut value: bool = default;
        unsafe { ffi::wxd_Config_ReadBool(self.ptr, c_key.as_ptr(), &mut value, default) };
        value
    }

    // --- Write Operations ---

    /// Writes a string value.
    pub fn write_string(&self, key: &str, value: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let c_value = match CString::new(value) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Config_WriteString(self.ptr, c_key.as_ptr(), c_value.as_ptr()) }
    }

    /// Writes a long integer value.
    pub fn write_long(&self, key: &str, value: i64) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Config_WriteLong(self.ptr, c_key.as_ptr(), value as c_long) }
    }

    /// Writes a double value.
    pub fn write_double(&self, key: &str, value: f64) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Config_WriteDouble(self.ptr, c_key.as_ptr(), value) }
    }

    /// Writes a boolean value.
    pub fn write_bool(&self, key: &str, value: bool) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Config_WriteBool(self.ptr, c_key.as_ptr(), value) }
    }

    // --- Existence Tests ---

    /// Checks if an entry or group exists.
    pub fn exists(&self, name: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_name = match CString::new(name) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Config_Exists(self.ptr, c_name.as_ptr()) }
    }

    /// Checks if an entry exists.
    pub fn has_entry(&self, name: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_name = match CString::new(name) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Config_HasEntry(self.ptr, c_name.as_ptr()) }
    }

    /// Checks if a group exists.
    pub fn has_group(&self, name: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_name = match CString::new(name) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Config_HasGroup(self.ptr, c_name.as_ptr()) }
    }

    /// Gets the type of an entry.
    pub fn get_entry_type(&self, name: &str) -> ConfigEntryType {
        if self.ptr.is_null() {
            return ConfigEntryType::Unknown;
        }
        let c_name = match CString::new(name) {
            Ok(s) => s,
            Err(_) => return ConfigEntryType::Unknown,
        };
        let entry_type = unsafe { ffi::wxd_Config_GetEntryType(self.ptr, c_name.as_ptr()) };
        ConfigEntryType::from(entry_type)
    }

    // --- Delete Operations ---

    /// Deletes an entry.
    ///
    /// If `delete_group_if_empty` is true, the containing group will be deleted
    /// if it becomes empty after removing the entry.
    pub fn delete_entry(&self, key: &str, delete_group_if_empty: bool) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Config_DeleteEntry(self.ptr, c_key.as_ptr(), delete_group_if_empty) }
    }

    /// Deletes a group and all its contents.
    pub fn delete_group(&self, key: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Config_DeleteGroup(self.ptr, c_key.as_ptr()) }
    }

    /// Deletes all entries and groups.
    pub fn delete_all(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Config_DeleteAll(self.ptr) }
    }

    // --- Enumeration ---

    /// Gets all entries in the current group.
    pub fn get_entries(&self) -> Vec<String> {
        if self.ptr.is_null() {
            return Vec::new();
        }

        let mut entries = Vec::new();
        let mut index: c_long = 0;
        let mut buf: Vec<c_char> = vec![0; 256];

        let mut has_more = unsafe { ffi::wxd_Config_GetFirstEntry(self.ptr, buf.as_mut_ptr(), buf.len(), &mut index) };

        while has_more {
            let name = unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() };
            entries.push(name);
            buf.fill(0);
            has_more = unsafe { ffi::wxd_Config_GetNextEntry(self.ptr, buf.as_mut_ptr(), buf.len(), &mut index) };
        }

        entries
    }

    /// Gets all groups in the current group.
    pub fn get_groups(&self) -> Vec<String> {
        if self.ptr.is_null() {
            return Vec::new();
        }

        let mut groups = Vec::new();
        let mut index: c_long = 0;
        let mut buf: Vec<c_char> = vec![0; 256];

        let mut has_more = unsafe { ffi::wxd_Config_GetFirstGroup(self.ptr, buf.as_mut_ptr(), buf.len(), &mut index) };

        while has_more {
            let name = unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() };
            groups.push(name);
            buf.fill(0);
            has_more = unsafe { ffi::wxd_Config_GetNextGroup(self.ptr, buf.as_mut_ptr(), buf.len(), &mut index) };
        }

        groups
    }

    /// Gets the number of entries in the current group.
    pub fn get_number_of_entries(&self, recursive: bool) -> usize {
        if self.ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Config_GetNumberOfEntries(self.ptr, recursive) }
    }

    /// Gets the number of groups in the current group.
    pub fn get_number_of_groups(&self, recursive: bool) -> usize {
        if self.ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Config_GetNumberOfGroups(self.ptr, recursive) }
    }

    // --- Rename Operations ---

    /// Renames an entry.
    pub fn rename_entry(&self, old_name: &str, new_name: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_old = match CString::new(old_name) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let c_new = match CString::new(new_name) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Config_RenameEntry(self.ptr, c_old.as_ptr(), c_new.as_ptr()) }
    }

    /// Renames a group.
    pub fn rename_group(&self, old_name: &str, new_name: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_old = match CString::new(old_name) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let c_new = match CString::new(new_name) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Config_RenameGroup(self.ptr, c_old.as_ptr(), c_new.as_ptr()) }
    }

    // --- Miscellaneous ---

    /// Flushes all changes to storage.
    ///
    /// If `current_only` is true, only flushes the current group.
    pub fn flush(&self, current_only: bool) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Config_Flush(self.ptr, current_only) }
    }

    /// Gets the application name.
    pub fn get_app_name(&self) -> String {
        if self.ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_Config_GetAppName(self.ptr, std::ptr::null_mut(), 0) };
        if len < 0 {
            return String::new();
        }
        let mut buf: Vec<c_char> = vec![0; len as usize + 1];
        unsafe { ffi::wxd_Config_GetAppName(self.ptr, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() }
    }

    /// Gets the vendor name.
    pub fn get_vendor_name(&self) -> String {
        if self.ptr.is_null() {
            return String::new();
        }
        let len = unsafe { ffi::wxd_Config_GetVendorName(self.ptr, std::ptr::null_mut(), 0) };
        if len < 0 {
            return String::new();
        }
        let mut buf: Vec<c_char> = vec![0; len as usize + 1];
        unsafe { ffi::wxd_Config_GetVendorName(self.ptr, buf.as_mut_ptr(), buf.len()) };
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string() }
    }

    /// Checks if environment variable expansion is enabled.
    pub fn is_expanding_env_vars(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Config_IsExpandingEnvVars(self.ptr) }
    }

    /// Sets whether to expand environment variables in values.
    pub fn set_expand_env_vars(&self, expand: bool) {
        if self.ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Config_SetExpandEnvVars(self.ptr, expand) };
    }

    /// Checks if recording defaults is enabled.
    pub fn is_recording_defaults(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Config_IsRecordingDefaults(self.ptr) }
    }

    /// Sets whether to record default values when reading.
    pub fn set_record_defaults(&self, record: bool) {
        if self.ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Config_SetRecordDefaults(self.ptr, record) };
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            unsafe { ffi::wxd_Config_Destroy(self.ptr) };
        }
    }
}

/// RAII guard that saves and restores the config path.
///
/// When created, it saves the current path. When dropped, it restores
/// the saved path. This is useful for functions that need to temporarily
/// change the config path.
///
/// # Example
///
/// ```rust,no_run
/// use wxdragon::config::{Config, ConfigPathGuard};
///
/// fn read_ui_settings(config: &Config) {
///     let _guard = ConfigPathGuard::new(config, "/UI/Settings");
///     // Now in /UI/Settings path
///     let color = config.read_string("BackgroundColor", "#FFFFFF");
///     // When _guard is dropped, path is restored to what it was before
/// }
/// ```
pub struct ConfigPathGuard<'a> {
    config: &'a Config,
    old_path: String,
}

impl<'a> ConfigPathGuard<'a> {
    /// Creates a new path guard, changing to the specified path.
    pub fn new(config: &'a Config, new_path: &str) -> Self {
        let old_path = config.get_path();
        config.set_path(new_path);
        Self { config, old_path }
    }
}

impl<'a> Drop for ConfigPathGuard<'a> {
    fn drop(&mut self) {
        self.config.set_path(&self.old_path);
    }
}
