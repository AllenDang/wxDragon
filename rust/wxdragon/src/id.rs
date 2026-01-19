//! Module for handling wxWidgets IDs.

use wxdragon_sys as ffi;

// Define Id as a type alias for i32
pub type Id = i32;

// Re-export the WXD_ID_ANY constant from ffi
pub use ffi::WXD_ID_ANY as ID_ANY; // Export as ID_ANY

/// Special ID meaning "no ID" or "disable this feature".
/// For dialogs, this is used with `set_escape_id()` to disable ESC key handling.
pub const ID_NONE: Id = -3;

/// The highest standard ID.
/// Used as a base for custom IDs (e.g., `ID_HIGHEST + 1`).
pub const ID_HIGHEST: Id = ffi::WXD_ID_HIGHEST as Id;

/// Standard ID for Exit commands.
pub const ID_EXIT: Id = ffi::WXD_ID_EXIT as Id;

/// Standard ID for About commands.
pub const ID_ABOUT: Id = ffi::WXD_ID_ABOUT as Id;

// --- Standard Dialog IDs (Manually Defined) ---
// Values based on typical wxWidgets assignments, verify if necessary.
pub const ID_OK: Id = ffi::WXD_ID_OK as Id; // Typically wxID_OK
pub const ID_CANCEL: Id = ffi::WXD_ID_CANCEL as Id; // Typically wxID_CANCEL
pub const ID_YES: Id = ffi::WXD_ID_YES as Id; // Typically wxID_YES
pub const ID_NO: Id = ffi::WXD_ID_NO as Id; // Typically wxID_NO

// Other standard IDs that might be useful (can be added if needed)
// pub const ID_HELP: Id = ...;
// pub const ID_CLOSE: Id = ...;
// pub const ID_ABOUT: Id = ...;
// pub const ID_EXIT: Id = ...;

// ... Any other constants ...
