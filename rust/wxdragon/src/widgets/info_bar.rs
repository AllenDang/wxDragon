use crate::event::{EventType, WxEvtHandler};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::CString;
use wxdragon_sys as ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ShowEffect {
    #[default]
    None = 0,
    RollToLeft = 1,
    RollToRight = 2,
    RollToTop = 3,
    RollToBottom = 4,
    SlideToLeft = 5,
    SlideToRight = 6,
    SlideToTop = 7,
    SlideToBottom = 8,
    Blend = 9,
    Expand = 10,
}

impl ShowEffect {
    pub fn from_i32(val: i32) -> Self {
        match val {
            1 => ShowEffect::RollToLeft,
            2 => ShowEffect::RollToRight,
            3 => ShowEffect::RollToTop,
            4 => ShowEffect::RollToBottom,
            5 => ShowEffect::SlideToLeft,
            6 => ShowEffect::SlideToRight,
            7 => ShowEffect::SlideToTop,
            8 => ShowEffect::SlideToBottom,
            9 => ShowEffect::Blend,
            10 => ShowEffect::Expand,
            _ => ShowEffect::None,
        }
    }
}

widget_style_enum!(
    name: InfoBarStyle,
    doc: "Style flags for InfoBar.",
    variants: {
        None: 0, "No icon.",
        Information: ffi::WXD_ICON_INFORMATION, "Information icon.",
        Warning: ffi::WXD_ICON_WARNING, "Warning icon.",
        Error: ffi::WXD_ICON_ERROR, "Error icon.",
        Question: ffi::WXD_ICON_QUESTION, "Question icon."
    },
    default_variant: Information
);

pub type RawInfoBar = ffi::wxd_InfoBar_t;

#[derive(Clone, Copy)]
pub struct InfoBar {
    handle: WindowHandle,
}

impl InfoBar {
    pub fn builder(parent: &dyn WxWidget) -> InfoBarBuilder<'_> {
        InfoBarBuilder::new(parent)
    }

    #[inline]
    fn info_bar_ptr(&self) -> *mut RawInfoBar {
        self.handle
            .get_ptr()
            .map(|p| p as *mut RawInfoBar)
            .unwrap_or(std::ptr::null_mut())
    }

    pub fn show_message(&self, msg: &str, flags: InfoBarStyle) {
        let ptr = self.info_bar_ptr();
        if ptr.is_null() {
            return;
        }
        let c_msg = CString::new(msg).unwrap_or_default();
        unsafe {
            ffi::wxd_InfoBar_ShowMessage(ptr, c_msg.as_ptr(), flags.bits() as i32);
        }
    }

    pub fn dismiss(&self) {
        let ptr = self.info_bar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_InfoBar_Dismiss(ptr);
        }
    }

    pub fn add_button(&self, btn_id: i32, label: &str) {
        let ptr = self.info_bar_ptr();
        if ptr.is_null() {
            return;
        }
        let c_label = CString::new(label).unwrap_or_default();
        unsafe {
            ffi::wxd_InfoBar_AddButton(ptr, btn_id, c_label.as_ptr());
        }
    }

    pub fn remove_button(&self, btn_id: i32) {
        let ptr = self.info_bar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_InfoBar_RemoveButton(ptr, btn_id);
        }
    }

    pub fn set_show_hide_effects(&self, show_effect: ShowEffect, hide_effect: ShowEffect) {
        let ptr = self.info_bar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_InfoBar_SetShowHideEffects(ptr, show_effect as i32, hide_effect as i32);
        }
    }

    pub fn get_show_effect(&self) -> ShowEffect {
        let ptr = self.info_bar_ptr();
        if ptr.is_null() {
            return ShowEffect::None;
        }
        ShowEffect::from_i32(unsafe { ffi::wxd_InfoBar_GetShowEffect(ptr) })
    }

    pub fn get_hide_effect(&self) -> ShowEffect {
        let ptr = self.info_bar_ptr();
        if ptr.is_null() {
            return ShowEffect::None;
        }
        ShowEffect::from_i32(unsafe { ffi::wxd_InfoBar_GetHideEffect(ptr) })
    }

    pub fn set_effect_duration(&self, duration: i32) {
        let ptr = self.info_bar_ptr();
        if ptr.is_null() {
            return;
        }
        unsafe {
            ffi::wxd_InfoBar_SetEffectDuration(ptr, duration);
        }
    }

    pub fn get_effect_duration(&self) -> i32 {
        let ptr = self.info_bar_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_InfoBar_GetEffectDuration(ptr) }
    }

    pub fn get_button_count(&self) -> usize {
        let ptr = self.info_bar_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_InfoBar_GetButtonCount(ptr) }
    }

    pub fn has_button_id(&self, btn_id: i32) -> bool {
        let ptr = self.info_bar_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_InfoBar_HasButtonId(ptr, btn_id) }
    }

    pub fn on_button<F>(&self, handler: F)
    where
        F: FnMut(crate::event::Event) + 'static,
    {
        self.bind_internal(EventType::COMMAND_BUTTON_CLICKED, handler);
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut RawInfoBar) -> Self {
        assert!(!ptr.is_null());
        InfoBar {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

impl WxWidget for InfoBar {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

impl WxEvtHandler for InfoBar {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

impl crate::event::WindowEvents for InfoBar {}

pub struct InfoBarBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
}

impl<'a> InfoBarBuilder<'a> {
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: crate::id::ID_ANY as Id,
        }
    }

    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn build(self) -> InfoBar {
        let parent_ptr = self.parent.handle_ptr();
        unsafe {
            let ptr = ffi::wxd_InfoBar_Create(parent_ptr, self.id);
            assert!(!ptr.is_null(), "wxd_InfoBar_Create returned null");
            InfoBar::from_ptr(ptr)
        }
    }
}

#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for InfoBar {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        InfoBar {
            handle: WindowHandle::new(ptr),
        }
    }
}

impl crate::window::FromWindowWithClassName for InfoBar {
    fn class_name() -> &'static str {
        "wxInfoBar"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        InfoBar {
            handle: WindowHandle::new(ptr),
        }
    }
}
