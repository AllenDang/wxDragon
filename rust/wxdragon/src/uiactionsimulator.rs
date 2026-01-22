//! UI Action Simulator module for wxDragon.
//!
//! This module provides a safe wrapper around wxWidgets' wxUIActionSimulator class.
//! The UIActionSimulator is used to simulate user interface actions such as mouse
//! clicks or key presses. Common usage includes playback/record (macro) functionality
//! and automated UI testing.
//!
//! # Example
//!
//! ```rust,no_run
//! use wxdragon::uiactionsimulator::{UIActionSimulator, MouseButton, KeyModifier};
//!
//! let sim = UIActionSimulator::new();
//!
//! // Move mouse to position (100, 100)
//! sim.mouse_move(100, 100);
//!
//! // Click the left mouse button
//! sim.mouse_click(MouseButton::Left);
//!
//! // Type some text
//! sim.text("Hello, World!");
//!
//! // Press Ctrl+S
//! sim.char_with_modifiers('s' as i32, KeyModifier::CONTROL);
//! ```
//!
//! # Note
//!
//! This class currently doesn't work when using Wayland with wxGTK.

use std::ffi::CString;
use wxdragon_sys as ffi;

/// Mouse button constants for UIActionSimulator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MouseButton {
    /// Any mouse button.
    Any = ffi::wxd_MouseButton_WXD_MOUSE_BTN_ANY,
    /// No mouse button.
    None = ffi::wxd_MouseButton_WXD_MOUSE_BTN_NONE,
    /// Left mouse button.
    Left = ffi::wxd_MouseButton_WXD_MOUSE_BTN_LEFT,
    /// Middle mouse button.
    Middle = ffi::wxd_MouseButton_WXD_MOUSE_BTN_MIDDLE,
    /// Right mouse button.
    Right = ffi::wxd_MouseButton_WXD_MOUSE_BTN_RIGHT,
    /// Auxiliary button 1.
    Aux1 = ffi::wxd_MouseButton_WXD_MOUSE_BTN_AUX1,
    /// Auxiliary button 2.
    Aux2 = ffi::wxd_MouseButton_WXD_MOUSE_BTN_AUX2,
}

impl Default for MouseButton {
    fn default() -> Self {
        MouseButton::Left
    }
}

/// Key modifier flags for UIActionSimulator.
///
/// These can be combined using bitwise OR operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyModifier(i32);

impl KeyModifier {
    /// No modifier keys.
    pub const NONE: KeyModifier = KeyModifier(ffi::wxd_KeyModifier_WXD_MOD_NONE as i32);
    /// Alt key.
    pub const ALT: KeyModifier = KeyModifier(ffi::wxd_KeyModifier_WXD_MOD_ALT as i32);
    /// Control key.
    pub const CONTROL: KeyModifier = KeyModifier(ffi::wxd_KeyModifier_WXD_MOD_CONTROL as i32);
    /// Alt+Ctrl combination (AltGr on some keyboards).
    pub const ALTGR: KeyModifier = KeyModifier(ffi::wxd_KeyModifier_WXD_MOD_ALTGR as i32);
    /// Shift key.
    pub const SHIFT: KeyModifier = KeyModifier(ffi::wxd_KeyModifier_WXD_MOD_SHIFT as i32);
    /// Meta/Windows/Command key.
    pub const META: KeyModifier = KeyModifier(ffi::wxd_KeyModifier_WXD_MOD_META as i32);
    /// Windows key (alias for META).
    pub const WIN: KeyModifier = KeyModifier(ffi::wxd_KeyModifier_WXD_MOD_WIN as i32);
    /// All modifier keys.
    pub const ALL: KeyModifier = KeyModifier(ffi::wxd_KeyModifier_WXD_MOD_ALL as i32);

    /// Create a KeyModifier from a raw value.
    pub fn from_raw(value: i32) -> Self {
        KeyModifier(value)
    }

    /// Get the raw value.
    pub fn to_raw(self) -> i32 {
        self.0
    }
}

impl Default for KeyModifier {
    fn default() -> Self {
        KeyModifier::NONE
    }
}

impl std::ops::BitOr for KeyModifier {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        KeyModifier(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for KeyModifier {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// UI Action Simulator for simulating user input.
///
/// This class is used to simulate user interface actions such as mouse clicks
/// or key presses. Common usage would be to provide playback and record (macro)
/// functionality for users, or to drive unit tests by simulating user sessions.
///
/// # Example
///
/// ```rust,no_run
/// use wxdragon::uiactionsimulator::{UIActionSimulator, MouseButton};
///
/// let sim = UIActionSimulator::new();
///
/// // Move mouse and click
/// sim.mouse_move(100, 100);
/// sim.mouse_click(MouseButton::Left);
///
/// // Type some text
/// sim.text("Hello!");
/// ```
pub struct UIActionSimulator {
    ptr: *mut ffi::wxd_UIActionSimulator_t,
}

impl UIActionSimulator {
    /// Creates a new UIActionSimulator.
    ///
    /// Returns a new instance of UIActionSimulator that can be used to simulate
    /// user interface actions.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_UIActionSimulator_Create() };
        Self { ptr }
    }

    /// Returns true if the simulator was created successfully.
    pub fn is_ok(&self) -> bool {
        !self.ptr.is_null()
    }

    // --- Mouse Simulation ---

    /// Move the mouse to the specified screen coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate in screen coordinates.
    /// * `y` - Y coordinate in screen coordinates.
    ///
    /// # Returns
    ///
    /// Returns true if the operation was successful.
    pub fn mouse_move(&self, x: i32, y: i32) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_UIActionSimulator_MouseMove(self.ptr, x as i64, y as i64) }
    }

    /// Press a mouse button.
    ///
    /// # Arguments
    ///
    /// * `button` - The mouse button to press.
    ///
    /// # Returns
    ///
    /// Returns true if the operation was successful.
    pub fn mouse_down(&self, button: MouseButton) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_UIActionSimulator_MouseDown(self.ptr, button as i32) }
    }

    /// Release a mouse button.
    ///
    /// # Arguments
    ///
    /// * `button` - The mouse button to release.
    ///
    /// # Returns
    ///
    /// Returns true if the operation was successful.
    pub fn mouse_up(&self, button: MouseButton) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_UIActionSimulator_MouseUp(self.ptr, button as i32) }
    }

    /// Click a mouse button (press and release).
    ///
    /// # Arguments
    ///
    /// * `button` - The mouse button to click.
    ///
    /// # Returns
    ///
    /// Returns true if the operation was successful.
    pub fn mouse_click(&self, button: MouseButton) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_UIActionSimulator_MouseClick(self.ptr, button as i32) }
    }

    /// Double-click a mouse button.
    ///
    /// # Arguments
    ///
    /// * `button` - The mouse button to double-click.
    ///
    /// # Returns
    ///
    /// Returns true if the operation was successful.
    pub fn mouse_dbl_click(&self, button: MouseButton) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_UIActionSimulator_MouseDblClick(self.ptr, button as i32) }
    }

    /// Perform a drag and drop operation.
    ///
    /// # Arguments
    ///
    /// * `x1` - Starting X coordinate in screen coordinates.
    /// * `y1` - Starting Y coordinate in screen coordinates.
    /// * `x2` - Ending X coordinate in screen coordinates.
    /// * `y2` - Ending Y coordinate in screen coordinates.
    /// * `button` - The mouse button to use for dragging.
    ///
    /// # Returns
    ///
    /// Returns true if the operation was successful.
    pub fn mouse_drag_drop(&self, x1: i32, y1: i32, x2: i32, y2: i32, button: MouseButton) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_UIActionSimulator_MouseDragDrop(self.ptr, x1 as i64, y1 as i64, x2 as i64, y2 as i64, button as i32) }
    }

    // --- Keyboard Simulation ---

    /// Press a key.
    ///
    /// If you are using modifiers, this needs to be paired with an identical
    /// `key_up` call or the modifiers will not be released.
    ///
    /// # Arguments
    ///
    /// * `keycode` - The key code (wxKeyCode).
    /// * `modifiers` - Modifier keys to press with the key.
    ///
    /// # Returns
    ///
    /// Returns true if the operation was successful.
    pub fn key_down(&self, keycode: i32, modifiers: KeyModifier) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_UIActionSimulator_KeyDown(self.ptr, keycode, modifiers.to_raw()) }
    }

    /// Release a key.
    ///
    /// # Arguments
    ///
    /// * `keycode` - The key code (wxKeyCode).
    /// * `modifiers` - Modifier keys that were pressed with the key.
    ///
    /// # Returns
    ///
    /// Returns true if the operation was successful.
    pub fn key_up(&self, keycode: i32, modifiers: KeyModifier) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_UIActionSimulator_KeyUp(self.ptr, keycode, modifiers.to_raw()) }
    }

    /// Press and release a key.
    ///
    /// # Arguments
    ///
    /// * `keycode` - The key code (wxKeyCode).
    ///
    /// # Returns
    ///
    /// Returns true if the operation was successful.
    pub fn char_key(&self, keycode: i32) -> bool {
        self.char_with_modifiers(keycode, KeyModifier::NONE)
    }

    /// Press and release a key with modifiers.
    ///
    /// # Arguments
    ///
    /// * `keycode` - The key code (wxKeyCode).
    /// * `modifiers` - Modifier keys to press with the key.
    ///
    /// # Returns
    ///
    /// Returns true if the operation was successful.
    pub fn char_with_modifiers(&self, keycode: i32, modifiers: KeyModifier) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_UIActionSimulator_Char(self.ptr, keycode, modifiers.to_raw()) }
    }

    /// Emulate typing the given string.
    ///
    /// Currently only ASCII letters are universally supported. Digits and
    /// punctuation characters can be used with the standard QWERTY (US)
    /// keyboard layout but may not work with other layouts.
    ///
    /// # Arguments
    ///
    /// * `text` - The string to type (ASCII characters).
    ///
    /// # Returns
    ///
    /// Returns true if the operation was successful.
    pub fn text(&self, text: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_text = match CString::new(text) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_UIActionSimulator_Text(self.ptr, c_text.as_ptr()) }
    }

    /// Select an item with the given text in the currently focused control.
    ///
    /// This method selects an item in the currently focused wxChoice, wxComboBox,
    /// wxListBox and similar controls. It does it by simulating keyboard events,
    /// so the behaviour should be the same as if the item was really selected
    /// by the user.
    ///
    /// # Arguments
    ///
    /// * `text` - The text of the item to select.
    ///
    /// # Returns
    ///
    /// Returns true if the item was successfully selected, false if the currently
    /// focused window is not one of the controls allowing item selection or if
    /// the item with the given text was not found in it.
    pub fn select(&self, text: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_text = match CString::new(text) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_UIActionSimulator_Select(self.ptr, c_text.as_ptr()) }
    }
}

impl Default for UIActionSimulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for UIActionSimulator {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_UIActionSimulator_Destroy(self.ptr) };
        }
    }
}
