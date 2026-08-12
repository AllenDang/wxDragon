//! Access to native, platform-appropriate UI colours, fonts, and metrics.
//!
//! This wraps `wxSystemSettings`, which is the supported way to match the host
//! platform's look and feel when drawing custom controls, instead of hard-coding
//! colours or sizes that would look wrong (or break entirely) under a different
//! theme, DPI setting, or OS.
//!
//! # Example
//!
//! ```no_run
//! use wxdragon::prelude::*;
//! use wxdragon::syssettings::{SystemColour, SystemMetric, SystemSettings};
//!
//! let window_bg = SystemSettings::get_colour(SystemColour::Window);
//! let highlight = SystemSettings::get_colour(SystemColour::Highlight);
//! let border_width = SystemSettings::get_metric(SystemMetric::BorderX, None);
//! ```

use crate::color::Colour;
use crate::font::Font;
use crate::window::WxWidget;
use wxdragon_sys as ffi;

/// Identifies a standard system colour, as used by [`SystemSettings::get_colour`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum SystemColour {
    Scrollbar,
    Desktop,
    ActiveCaption,
    InactiveCaption,
    Menu,
    Window,
    WindowFrame,
    MenuText,
    WindowText,
    CaptionText,
    ActiveBorder,
    InactiveBorder,
    AppWorkspace,
    Highlight,
    HighlightText,
    BtnFace,
    BtnShadow,
    GrayText,
    BtnText,
    InactiveCaptionText,
    BtnHighlight,
    ThreeDDkShadow,
    ThreeDLight,
    InfoText,
    InfoBk,
    ListBox,
    HotLight,
    GradientActiveCaption,
    GradientInactiveCaption,
    MenuHilight,
    MenuBar,
    ListBoxText,
    ListBoxHighlightText,
    GridLines,
    ListBoxHighlight,
}

impl SystemColour {
    fn to_raw(self) -> ffi::wxd_SystemColour {
        match self {
            SystemColour::Scrollbar => ffi::wxd_SystemColour_WXD_SYS_COLOUR_SCROLLBAR,
            SystemColour::Desktop => ffi::wxd_SystemColour_WXD_SYS_COLOUR_DESKTOP,
            SystemColour::ActiveCaption => ffi::wxd_SystemColour_WXD_SYS_COLOUR_ACTIVECAPTION,
            SystemColour::InactiveCaption => ffi::wxd_SystemColour_WXD_SYS_COLOUR_INACTIVECAPTION,
            SystemColour::Menu => ffi::wxd_SystemColour_WXD_SYS_COLOUR_MENU,
            SystemColour::Window => ffi::wxd_SystemColour_WXD_SYS_COLOUR_WINDOW,
            SystemColour::WindowFrame => ffi::wxd_SystemColour_WXD_SYS_COLOUR_WINDOWFRAME,
            SystemColour::MenuText => ffi::wxd_SystemColour_WXD_SYS_COLOUR_MENUTEXT,
            SystemColour::WindowText => ffi::wxd_SystemColour_WXD_SYS_COLOUR_WINDOWTEXT,
            SystemColour::CaptionText => ffi::wxd_SystemColour_WXD_SYS_COLOUR_CAPTIONTEXT,
            SystemColour::ActiveBorder => ffi::wxd_SystemColour_WXD_SYS_COLOUR_ACTIVEBORDER,
            SystemColour::InactiveBorder => ffi::wxd_SystemColour_WXD_SYS_COLOUR_INACTIVEBORDER,
            SystemColour::AppWorkspace => ffi::wxd_SystemColour_WXD_SYS_COLOUR_APPWORKSPACE,
            SystemColour::Highlight => ffi::wxd_SystemColour_WXD_SYS_COLOUR_HIGHLIGHT,
            SystemColour::HighlightText => ffi::wxd_SystemColour_WXD_SYS_COLOUR_HIGHLIGHTTEXT,
            SystemColour::BtnFace => ffi::wxd_SystemColour_WXD_SYS_COLOUR_BTNFACE,
            SystemColour::BtnShadow => ffi::wxd_SystemColour_WXD_SYS_COLOUR_BTNSHADOW,
            SystemColour::GrayText => ffi::wxd_SystemColour_WXD_SYS_COLOUR_GRAYTEXT,
            SystemColour::BtnText => ffi::wxd_SystemColour_WXD_SYS_COLOUR_BTNTEXT,
            SystemColour::InactiveCaptionText => ffi::wxd_SystemColour_WXD_SYS_COLOUR_INACTIVECAPTIONTEXT,
            SystemColour::BtnHighlight => ffi::wxd_SystemColour_WXD_SYS_COLOUR_BTNHIGHLIGHT,
            SystemColour::ThreeDDkShadow => ffi::wxd_SystemColour_WXD_SYS_COLOUR_3DDKSHADOW,
            SystemColour::ThreeDLight => ffi::wxd_SystemColour_WXD_SYS_COLOUR_3DLIGHT,
            SystemColour::InfoText => ffi::wxd_SystemColour_WXD_SYS_COLOUR_INFOTEXT,
            SystemColour::InfoBk => ffi::wxd_SystemColour_WXD_SYS_COLOUR_INFOBK,
            SystemColour::ListBox => ffi::wxd_SystemColour_WXD_SYS_COLOUR_LISTBOX,
            SystemColour::HotLight => ffi::wxd_SystemColour_WXD_SYS_COLOUR_HOTLIGHT,
            SystemColour::GradientActiveCaption => ffi::wxd_SystemColour_WXD_SYS_COLOUR_GRADIENTACTIVECAPTION,
            SystemColour::GradientInactiveCaption => ffi::wxd_SystemColour_WXD_SYS_COLOUR_GRADIENTINACTIVECAPTION,
            SystemColour::MenuHilight => ffi::wxd_SystemColour_WXD_SYS_COLOUR_MENUHILIGHT,
            SystemColour::MenuBar => ffi::wxd_SystemColour_WXD_SYS_COLOUR_MENUBAR,
            SystemColour::ListBoxText => ffi::wxd_SystemColour_WXD_SYS_COLOUR_LISTBOXTEXT,
            SystemColour::ListBoxHighlightText => ffi::wxd_SystemColour_WXD_SYS_COLOUR_LISTBOXHIGHLIGHTTEXT,
            SystemColour::GridLines => ffi::wxd_SystemColour_WXD_SYS_COLOUR_GRIDLINES,
            SystemColour::ListBoxHighlight => ffi::wxd_SystemColour_WXD_SYS_COLOUR_LISTBOXHIGHLIGHT,
        }
    }
}

/// Identifies a standard system font, as used by [`SystemSettings::get_font`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum SystemFont {
    OemFixedFont,
    AnsiFixedFont,
    AnsiVarFont,
    SystemFont,
    DeviceDefaultFont,
    SystemFixedFont,
    /// The default font used for controls and dialogs; this is almost always the one you want.
    DefaultGuiFont,
}

impl SystemFont {
    fn to_raw(self) -> ffi::wxd_SystemFont {
        match self {
            SystemFont::OemFixedFont => ffi::wxd_SystemFont_WXD_SYS_OEM_FIXED_FONT,
            SystemFont::AnsiFixedFont => ffi::wxd_SystemFont_WXD_SYS_ANSI_FIXED_FONT,
            SystemFont::AnsiVarFont => ffi::wxd_SystemFont_WXD_SYS_ANSI_VAR_FONT,
            SystemFont::SystemFont => ffi::wxd_SystemFont_WXD_SYS_SYSTEM_FONT,
            SystemFont::DeviceDefaultFont => ffi::wxd_SystemFont_WXD_SYS_DEVICE_DEFAULT_FONT,
            SystemFont::SystemFixedFont => ffi::wxd_SystemFont_WXD_SYS_SYSTEM_FIXED_FONT,
            SystemFont::DefaultGuiFont => ffi::wxd_SystemFont_WXD_SYS_DEFAULT_GUI_FONT,
        }
    }
}

/// Identifies a system-dependent metric, as used by [`SystemSettings::get_metric`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum SystemMetric {
    MouseButtons,
    BorderX,
    BorderY,
    /// Width/height of a cursor; cursors are always square, so a single value covers both.
    CursorSize,
    DClickX,
    DClickY,
    DragX,
    DragY,
    EdgeX,
    EdgeY,
    HScrollArrowX,
    HScrollArrowY,
    HThumbX,
    IconX,
    IconY,
    IconSpacingX,
    IconSpacingY,
    WindowMinX,
    WindowMinY,
    ScreenX,
    ScreenY,
    FrameSizeX,
    FrameSizeY,
    SmallIconX,
    SmallIconY,
    HScrollY,
    VScrollX,
    VScrollArrowX,
    VScrollArrowY,
    VThumbY,
    CaptionY,
    MenuY,
    /// Non-zero if a network is present.
    NetworkPresent,
    PenWindowsPresent,
    /// Non-zero if the user prefers visual accompaniments for sounds.
    ShowSounds,
    /// Non-zero if the primary/secondary mouse buttons are swapped.
    SwapButtons,
    DClickMsec,
    CaretOnMsec,
    CaretOffMsec,
    CaretTimeoutMsec,
}

impl SystemMetric {
    fn to_raw(self) -> ffi::wxd_SystemMetric {
        match self {
            SystemMetric::MouseButtons => ffi::wxd_SystemMetric_WXD_SYS_MOUSE_BUTTONS,
            SystemMetric::BorderX => ffi::wxd_SystemMetric_WXD_SYS_BORDER_X,
            SystemMetric::BorderY => ffi::wxd_SystemMetric_WXD_SYS_BORDER_Y,
            SystemMetric::CursorSize => ffi::wxd_SystemMetric_WXD_SYS_CURSOR_SIZE,
            SystemMetric::DClickX => ffi::wxd_SystemMetric_WXD_SYS_DCLICK_X,
            SystemMetric::DClickY => ffi::wxd_SystemMetric_WXD_SYS_DCLICK_Y,
            SystemMetric::DragX => ffi::wxd_SystemMetric_WXD_SYS_DRAG_X,
            SystemMetric::DragY => ffi::wxd_SystemMetric_WXD_SYS_DRAG_Y,
            SystemMetric::EdgeX => ffi::wxd_SystemMetric_WXD_SYS_EDGE_X,
            SystemMetric::EdgeY => ffi::wxd_SystemMetric_WXD_SYS_EDGE_Y,
            SystemMetric::HScrollArrowX => ffi::wxd_SystemMetric_WXD_SYS_HSCROLL_ARROW_X,
            SystemMetric::HScrollArrowY => ffi::wxd_SystemMetric_WXD_SYS_HSCROLL_ARROW_Y,
            SystemMetric::HThumbX => ffi::wxd_SystemMetric_WXD_SYS_HTHUMB_X,
            SystemMetric::IconX => ffi::wxd_SystemMetric_WXD_SYS_ICON_X,
            SystemMetric::IconY => ffi::wxd_SystemMetric_WXD_SYS_ICON_Y,
            SystemMetric::IconSpacingX => ffi::wxd_SystemMetric_WXD_SYS_ICONSPACING_X,
            SystemMetric::IconSpacingY => ffi::wxd_SystemMetric_WXD_SYS_ICONSPACING_Y,
            SystemMetric::WindowMinX => ffi::wxd_SystemMetric_WXD_SYS_WINDOWMIN_X,
            SystemMetric::WindowMinY => ffi::wxd_SystemMetric_WXD_SYS_WINDOWMIN_Y,
            SystemMetric::ScreenX => ffi::wxd_SystemMetric_WXD_SYS_SCREEN_X,
            SystemMetric::ScreenY => ffi::wxd_SystemMetric_WXD_SYS_SCREEN_Y,
            SystemMetric::FrameSizeX => ffi::wxd_SystemMetric_WXD_SYS_FRAMESIZE_X,
            SystemMetric::FrameSizeY => ffi::wxd_SystemMetric_WXD_SYS_FRAMESIZE_Y,
            SystemMetric::SmallIconX => ffi::wxd_SystemMetric_WXD_SYS_SMALLICON_X,
            SystemMetric::SmallIconY => ffi::wxd_SystemMetric_WXD_SYS_SMALLICON_Y,
            SystemMetric::HScrollY => ffi::wxd_SystemMetric_WXD_SYS_HSCROLL_Y,
            SystemMetric::VScrollX => ffi::wxd_SystemMetric_WXD_SYS_VSCROLL_X,
            SystemMetric::VScrollArrowX => ffi::wxd_SystemMetric_WXD_SYS_VSCROLL_ARROW_X,
            SystemMetric::VScrollArrowY => ffi::wxd_SystemMetric_WXD_SYS_VSCROLL_ARROW_Y,
            SystemMetric::VThumbY => ffi::wxd_SystemMetric_WXD_SYS_VTHUMB_Y,
            SystemMetric::CaptionY => ffi::wxd_SystemMetric_WXD_SYS_CAPTION_Y,
            SystemMetric::MenuY => ffi::wxd_SystemMetric_WXD_SYS_MENU_Y,
            SystemMetric::NetworkPresent => ffi::wxd_SystemMetric_WXD_SYS_NETWORK_PRESENT,
            SystemMetric::PenWindowsPresent => ffi::wxd_SystemMetric_WXD_SYS_PENWINDOWS_PRESENT,
            SystemMetric::ShowSounds => ffi::wxd_SystemMetric_WXD_SYS_SHOW_SOUNDS,
            SystemMetric::SwapButtons => ffi::wxd_SystemMetric_WXD_SYS_SWAP_BUTTONS,
            SystemMetric::DClickMsec => ffi::wxd_SystemMetric_WXD_SYS_DCLICK_MSEC,
            SystemMetric::CaretOnMsec => ffi::wxd_SystemMetric_WXD_SYS_CARET_ON_MSEC,
            SystemMetric::CaretOffMsec => ffi::wxd_SystemMetric_WXD_SYS_CARET_OFF_MSEC,
            SystemMetric::CaretTimeoutMsec => ffi::wxd_SystemMetric_WXD_SYS_CARET_TIMEOUT_MSEC,
        }
    }
}

/// Provides access to native, platform-appropriate UI colours, fonts, and metrics.
pub struct SystemSettings;

impl SystemSettings {
    /// Returns a standard system colour, such as the native window background or highlight colour.
    pub fn get_colour(index: SystemColour) -> Colour {
        let raw = unsafe { ffi::wxd_SystemSettings_GetColour(index.to_raw()) };
        Colour::from(raw)
    }

    /// Returns a standard system font, such as the default GUI font used by controls and dialogs.
    ///
    /// Returns `None` if the requested font is not available on this platform.
    pub fn get_font(index: SystemFont) -> Option<Font> {
        let ptr = unsafe { ffi::wxd_SystemSettings_GetFont(index.to_raw()) };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { Font::from_ptr(ptr, true) })
    }

    /// Returns a system-dependent metric, such as a scrollbar's width or the double-click distance.
    ///
    /// If `window` is given, the metric is scaled for the display that window is on, which
    /// matters on multi-monitor setups where displays can have different DPI settings.
    pub fn get_metric(index: SystemMetric, window: Option<&dyn WxWidget>) -> i32 {
        let window_ptr = window.map_or(std::ptr::null_mut(), |w| w.handle_ptr());
        unsafe { ffi::wxd_SystemSettings_GetMetric(index.to_raw(), window_ptr) }
    }
}
