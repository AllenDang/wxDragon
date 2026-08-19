use crate::bitmap_bundle::BitmapBundle;
use crate::color::Colour;
use crate::font::Font;
use crate::geometry::Rect;
use crate::window::WxWidget;
use std::ffi::CString;
use wxdragon_sys as ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum TipKind {
    None = 0,
    TopLeft = 1,
    Top = 2,
    TopRight = 3,
    BottomLeft = 4,
    Bottom = 5,
    BottomRight = 6,
    #[default]
    Auto = 7,
}

pub struct RichToolTip {
    ptr: *mut ffi::wxd_RichToolTip_t,
}

impl RichToolTip {
    pub fn new(title: &str, message: &str) -> Self {
        let c_title = CString::new(title).unwrap_or_default();
        let c_msg = CString::new(message).unwrap_or_default();
        let ptr = unsafe { ffi::wxd_RichToolTip_Create(c_title.as_ptr(), c_msg.as_ptr()) };
        assert!(!ptr.is_null(), "wxd_RichToolTip_Create returned null");
        RichToolTip { ptr }
    }

    pub fn builder(title: &str, message: &str) -> RichToolTipBuilder {
        RichToolTipBuilder::new(title, message)
    }

    pub fn set_background_colour(&self, col: &Colour, col_end: Option<&Colour>) {
        let raw_col = col.to_raw();
        let raw_col_end = col_end.map(|c| c.to_raw());
        let end_ptr = raw_col_end
            .as_ref()
            .map_or(std::ptr::null(), |c| c as *const ffi::wxd_Colour_t);
        unsafe {
            ffi::wxd_RichToolTip_SetBackgroundColour(self.ptr, &raw_col, end_ptr);
        }
    }

    pub fn set_title_font(&self, font: &Font) {
        unsafe {
            ffi::wxd_RichToolTip_SetTitleFont(self.ptr, font.as_ptr());
        }
    }

    pub fn set_icon(&self, icon: i32) {
        unsafe {
            ffi::wxd_RichToolTip_SetIcon(self.ptr, icon);
        }
    }

    pub fn set_custom_icon(&self, icon: &BitmapBundle) {
        unsafe {
            ffi::wxd_RichToolTip_SetCustomIcon(self.ptr, icon.as_ptr());
        }
    }

    pub fn set_timeout(&self, milliseconds: u32, show_delay_ms: u32) {
        unsafe {
            ffi::wxd_RichToolTip_SetTimeout(self.ptr, milliseconds, show_delay_ms);
        }
    }

    pub fn set_tip_kind(&self, tip_kind: TipKind) {
        unsafe {
            ffi::wxd_RichToolTip_SetTipKind(self.ptr, tip_kind as i32);
        }
    }

    pub fn show_for(&self, window: &impl WxWidget) {
        let win_ptr = window.handle_ptr();
        unsafe {
            ffi::wxd_RichToolTip_ShowFor(self.ptr, win_ptr, std::ptr::null());
        }
    }

    pub fn show_for_rect(&self, window: &impl WxWidget, rect: &Rect) {
        let win_ptr = window.handle_ptr();
        let raw_rect: ffi::wxd_Rect = (*rect).into();
        unsafe {
            ffi::wxd_RichToolTip_ShowFor(self.ptr, win_ptr, &raw_rect);
        }
    }
}

impl Drop for RichToolTip {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::wxd_RichToolTip_Destroy(self.ptr);
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}

pub struct RichToolTipBuilder {
    title: String,
    message: String,
    bg_col: Option<Colour>,
    bg_col_end: Option<Colour>,
    title_font: Option<Font>,
    icon_flag: Option<i32>,
    custom_icon: Option<BitmapBundle>,
    timeout_ms: Option<(u32, u32)>,
    tip_kind: Option<TipKind>,
}

impl RichToolTipBuilder {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            bg_col: None,
            bg_col_end: None,
            title_font: None,
            icon_flag: None,
            custom_icon: None,
            timeout_ms: None,
            tip_kind: None,
        }
    }

    pub fn with_background_colour(mut self, col: Colour, col_end: Option<Colour>) -> Self {
        self.bg_col = Some(col);
        self.bg_col_end = col_end;
        self
    }

    pub fn with_title_font(mut self, font: Font) -> Self {
        self.title_font = Some(font);
        self
    }

    pub fn with_icon(mut self, icon: i32) -> Self {
        self.icon_flag = Some(icon);
        self
    }

    pub fn with_custom_icon(mut self, icon: BitmapBundle) -> Self {
        self.custom_icon = Some(icon);
        self
    }

    pub fn with_timeout(mut self, milliseconds: u32, show_delay_ms: u32) -> Self {
        self.timeout_ms = Some((milliseconds, show_delay_ms));
        self
    }

    pub fn with_tip_kind(mut self, tip_kind: TipKind) -> Self {
        self.tip_kind = Some(tip_kind);
        self
    }

    pub fn build(self) -> RichToolTip {
        let tip = RichToolTip::new(&self.title, &self.message);
        if let Some(bg) = &self.bg_col {
            tip.set_background_colour(bg, self.bg_col_end.as_ref());
        }
        if let Some(font) = &self.title_font {
            tip.set_title_font(font);
        }
        if let Some(flag) = self.icon_flag {
            tip.set_icon(flag);
        }
        if let Some(icon) = &self.custom_icon {
            tip.set_custom_icon(icon);
        }
        if let Some((ms, delay)) = self.timeout_ms {
            tip.set_timeout(ms, delay);
        }
        if let Some(kind) = self.tip_kind {
            tip.set_tip_kind(kind);
        }
        tip
    }
}
