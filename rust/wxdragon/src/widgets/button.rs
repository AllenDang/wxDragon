use crate::bitmap::Bitmap;
use crate::bitmap_bundle::BitmapBundle;
use crate::event::button_events::ButtonEvents; // Added ButtonEvents import
use crate::implement_widget_traits_with_target;
use crate::prelude::*; // Use prelude
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget}; // Make sure WxEvtHandler is imported // Added
                                       // Remove specific imports covered by prelude
                                       // use crate::{Id, Point, Size};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi; // ADDED for enum bitwise operations

/// Enum for specifying bitmap position on a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)] // Matches wxd_ButtonBitmapPosition_t which is an enum
#[derive(Default)]
pub enum ButtonBitmapPosition {
    #[default]
    Left = ffi::wxd_ButtonBitmapPosition_t_WXD_BUTTON_BITMAP_LEFT,
    Right = ffi::wxd_ButtonBitmapPosition_t_WXD_BUTTON_BITMAP_RIGHT,
    Top = ffi::wxd_ButtonBitmapPosition_t_WXD_BUTTON_BITMAP_TOP,
    Bottom = ffi::wxd_ButtonBitmapPosition_t_WXD_BUTTON_BITMAP_BOTTOM,
}

/// Represents a wxButton.
#[derive(Clone)]
pub struct Button {
    window: Window, // Composition: Button IS a Window
    // Store parent pointer to manage drop behavior
    // Allow dead_code because it's used implicitly by the Drop logic.
    #[allow(dead_code)]
    parent_ptr: *mut ffi::wxd_Window_t,
}

impl Button {
    /// Creates a new `ButtonBuilder` for constructing a button.
    pub fn builder(parent: &dyn WxWidget) -> ButtonBuilder<'_> {
        ButtonBuilder::new(parent)
    }

    /// Creates a new Button from a raw window and parent pointer.
    /// This is intended for internal use by other widget wrappers that compose Button.
    pub(crate) fn new_from_composition(window: Window, parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        Self { window, parent_ptr }
    }

    /// Creates a new Button (low-level constructor used by the builder)
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        label: &str,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        assert!(!parent_ptr.is_null(), "Button requires a parent");
        let c_label = CString::new(label).expect("CString::new failed");

        let ptr = unsafe {
            ffi::wxd_Button_Create(
                parent_ptr,
                id,
                c_label.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create Button widget");
        } else {
            // Cast the concrete Button pointer to the base Window pointer for the wrapper
            let window = unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) };
            Button { window, parent_ptr }
        }
    }

    /// Sets the button's label.
    pub fn set_label(&self, label: &str) {
        let c_label = CString::new(label).expect("CString::new failed");
        unsafe {
            ffi::wxd_Button_SetLabel(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                c_label.as_ptr(),
            );
        }
    }

    /// Gets the button's label.
    pub fn get_label(&self) -> String {
        let mut buffer: [c_char; 256] = [0; 256]; // Reasonable buffer size
        let len_needed = unsafe {
            ffi::wxd_Button_GetLabel(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            )
        };

        if len_needed > 0 && (len_needed as usize) <= buffer.len() {
            unsafe {
                CStr::from_ptr(buffer.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            }
        } else if len_needed > (buffer.len() as i32) {
            // Buffer too small, try again with required size
            let mut vec_buffer: Vec<c_char> = vec![0; len_needed as usize];
            let len_needed_2 = unsafe {
                ffi::wxd_Button_GetLabel(
                    self.window.as_ptr() as *mut ffi::wxd_Button_t,
                    vec_buffer.as_mut_ptr(),
                    vec_buffer.len() as i32,
                )
            };
            if len_needed_2 == len_needed {
                unsafe {
                    CStr::from_ptr(vec_buffer.as_ptr())
                        .to_string_lossy()
                        .into_owned()
                }
            } else {
                // Something went wrong
                String::new()
            }
        } else {
            // Error or empty label
            String::new()
        }
    }

    // --- Bitmap Methods ---

    /// Sets the bitmap to be displayed by the button.
    pub fn set_bitmap(&self, bitmap: &Bitmap, dir: ButtonBitmapPosition) {
        unsafe {
            ffi::wxd_Button_SetBitmap(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                bitmap.as_ptr(),
                dir as ffi::wxd_ButtonBitmapPosition_t,
            );
        }
    }

    /// Sets the bitmap for the label (main bitmap, default position Left).
    pub fn set_bitmap_label(&self, bitmap: &Bitmap) {
        self.set_bitmap(bitmap, ButtonBitmapPosition::Left); // wxButton::SetBitmapLabel is often an alias for SetBitmap with wxLEFT
    }

    /// Sets the bitmap for the disabled state.
    pub fn set_bitmap_disabled(&self, bitmap: &Bitmap) {
        unsafe {
            ffi::wxd_Button_SetBitmapDisabled(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                bitmap.as_ptr(),
            );
        }
    }

    /// Sets the bitmap for the focused state.
    pub fn set_bitmap_focus(&self, bitmap: &Bitmap) {
        unsafe {
            ffi::wxd_Button_SetBitmapFocus(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                bitmap.as_ptr(),
            );
        }
    }

    /// Sets the bitmap for the current (hover) state.
    pub fn set_bitmap_current(&self, bitmap: &Bitmap) {
        unsafe {
            ffi::wxd_Button_SetBitmapCurrent(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                bitmap.as_ptr(),
            );
        }
    }

    /// Sets the bitmap for the pressed state.
    pub fn set_bitmap_pressed(&self, bitmap: &Bitmap) {
        unsafe {
            ffi::wxd_Button_SetBitmapPressed(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                bitmap.as_ptr(),
            );
        }
    }

    // --- BitmapBundle Methods ---

    /// Sets the bitmap bundle to be displayed by the button.
    pub fn set_bitmap_bundle(&self, bundle: &BitmapBundle, dir: ButtonBitmapPosition) {
        unsafe {
            ffi::wxd_Button_SetBitmapBundle(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                bundle.as_ptr(),
                dir as i32,
            );
        }
    }

    /// Sets the bitmap bundle for the label (main bitmap, default position Left).
    pub fn set_bitmap_bundle_label(&self, bundle: &BitmapBundle) {
        self.set_bitmap_bundle(bundle, ButtonBitmapPosition::Left);
    }

    /// Sets the bitmap bundle for the disabled state.
    pub fn set_bitmap_bundle_disabled(&self, bundle: &BitmapBundle) {
        unsafe {
            ffi::wxd_Button_SetBitmapBundleDisabled(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                bundle.as_ptr(),
            );
        }
    }

    /// Sets the bitmap bundle for the focused state.
    pub fn set_bitmap_bundle_focus(&self, bundle: &BitmapBundle) {
        unsafe {
            ffi::wxd_Button_SetBitmapBundleFocus(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                bundle.as_ptr(),
            );
        }
    }

    /// Sets the bitmap bundle for the hover state.
    pub fn set_bitmap_bundle_hover(&self, bundle: &BitmapBundle) {
        unsafe {
            ffi::wxd_Button_SetBitmapBundleHover(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                bundle.as_ptr(),
            );
        }
    }

    /// Sets the bitmap bundle for the pressed state.
    pub fn set_bitmap_bundle_pressed(&self, bundle: &BitmapBundle) {
        unsafe {
            ffi::wxd_Button_SetBitmapBundlePressed(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                bundle.as_ptr(),
            );
        }
    }

    // Getters return Option<Bitmap> and are unowned due to C++ FFI returning direct or null pointers.
    // The C++ FFI getters are currently placeholders returning nullptr.
    // When implemented, they might return unowned pointers, requiring Bitmap::from_ptr_unowned.

    pub fn get_bitmap(&self) -> Option<Bitmap> {
        let ptr =
            unsafe { ffi::wxd_Button_GetBitmap(self.window.as_ptr() as *mut ffi::wxd_Button_t) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Bitmap::from_ptr_unowned(ptr) })
        }
    }
    pub fn get_bitmap_disabled(&self) -> Option<Bitmap> {
        let ptr = unsafe {
            ffi::wxd_Button_GetBitmapDisabled(self.window.as_ptr() as *mut ffi::wxd_Button_t)
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Bitmap::from_ptr_unowned(ptr) })
        }
    }
    pub fn get_bitmap_focus(&self) -> Option<Bitmap> {
        let ptr = unsafe {
            ffi::wxd_Button_GetBitmapFocus(self.window.as_ptr() as *mut ffi::wxd_Button_t)
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Bitmap::from_ptr_unowned(ptr) })
        }
    }
    pub fn get_bitmap_current(&self) -> Option<Bitmap> {
        let ptr = unsafe {
            ffi::wxd_Button_GetBitmapCurrent(self.window.as_ptr() as *mut ffi::wxd_Button_t)
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Bitmap::from_ptr_unowned(ptr) })
        }
    }
    pub fn get_bitmap_pressed(&self) -> Option<Bitmap> {
        let ptr = unsafe {
            ffi::wxd_Button_GetBitmapPressed(self.window.as_ptr() as *mut ffi::wxd_Button_t)
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Bitmap::from_ptr_unowned(ptr) })
        }
    }
}

// Implement ButtonEvents trait for Button
impl ButtonEvents for Button {}

// Use the widget_builder macro to generate the ButtonBuilder implementation
widget_builder!(
    name: Button,
    parent_type: &'a dyn WxWidget,
    style_type: ButtonStyle,
    fields: {
        label: String = String::new(),
        bitmap_label: Option<Bitmap> = None, // Renamed from bitmap
        bitmap_position: Option<ButtonBitmapPosition> = None,
        bitmap_disabled: Option<Bitmap> = None,
        bitmap_focus: Option<Bitmap> = None,
        bitmap_current: Option<Bitmap> = None,
        bitmap_pressed: Option<Bitmap> = None,
        // BitmapBundle fields
        bitmap_bundle_label: Option<BitmapBundle> = None,
        bitmap_bundle_disabled: Option<BitmapBundle> = None,
        bitmap_bundle_focus: Option<BitmapBundle> = None,
        bitmap_bundle_hover: Option<BitmapBundle> = None,
        bitmap_bundle_pressed: Option<BitmapBundle> = None
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let button = Button::new_impl(
            parent_ptr,
            slf.id,
            &slf.label,
            slf.pos,
            slf.size,
            slf.style.bits(),
        );

        // Prioritize BitmapBundle over Bitmap if both are set
        if let Some(ref bundle) = slf.bitmap_bundle_label {
            button.set_bitmap_bundle(bundle, slf.bitmap_position.unwrap_or_default());
        } else if let Some(ref bmp) = slf.bitmap_label {
            button.set_bitmap(bmp, slf.bitmap_position.unwrap_or_default());
        }

        if let Some(ref bundle) = slf.bitmap_bundle_disabled {
            button.set_bitmap_bundle_disabled(bundle);
        } else if let Some(ref bmp) = slf.bitmap_disabled {
            button.set_bitmap_disabled(bmp);
        }

        if let Some(ref bundle) = slf.bitmap_bundle_focus {
            button.set_bitmap_bundle_focus(bundle);
        } else if let Some(ref bmp) = slf.bitmap_focus {
            button.set_bitmap_focus(bmp);
        }

        if let Some(ref bundle) = slf.bitmap_bundle_hover {
            button.set_bitmap_bundle_hover(bundle);
        } else if let Some(ref bmp) = slf.bitmap_current {
            button.set_bitmap_current(bmp);
        }

        if let Some(ref bundle) = slf.bitmap_bundle_pressed {
            button.set_bitmap_bundle_pressed(bundle);
        } else if let Some(ref bmp) = slf.bitmap_pressed {
            button.set_bitmap_pressed(bmp);
        }

        button
    }
);

// Apply common trait implementations for Button
implement_widget_traits_with_target!(Button, window, Window);

// XRC Support - enables Button to be created from XRC-managed pointers
impl_xrc_support!(Button, {
    window,
    parent_ptr: std::ptr::null_mut()
});

// Define the ButtonStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: ButtonStyle,
    doc: "Style flags for `Button`.",
    variants: {
        Default: 0, "Default style (no specific alignment, standard border).",
        Left: ffi::WXD_BU_LEFT, "Align label to the left.",
        Top: ffi::WXD_BU_TOP, "Align label to the top.",
        Right: ffi::WXD_BU_RIGHT, "Align label to the right.",
        Bottom: ffi::WXD_BU_BOTTOM, "Align label to the bottom.",
        ExactFit: ffi::WXD_BU_EXACTFIT, "Button size will be adjusted to exactly fit the label.",
        NoText: ffi::WXD_BU_NOTEXT, "Do not display the label string (useful for buttons with only an image).",
        BorderNone: ffi::WXD_BORDER_NONE, "No border.",
        BorderSimple: ffi::WXD_BORDER_SIMPLE, "A simple border (rarely used for buttons, which have a default look)."
    },
    default_variant: Default
);

// Enable widget casting for Button
impl_widget_cast!(Button, "wxButton", composition);
