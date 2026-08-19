use crate::bitmap_bundle::BitmapBundle;
use crate::event::{Event, EventType};
use crate::widgets::taskbar_icon::TaskBarIcon;
use crate::window::WxWidget;
use std::ffi::{CString, NulError};
use std::os::raw::c_int;
use wxdragon_sys as ffi;

#[derive(Debug)]
pub enum Error {
    NulError(NulError),
    FfiCreation(String),
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Self {
        Error::NulError(err)
    }
}

pub type WxResult<T> = Result<T, Error>;

pub const TIMEOUT_AUTO: i32 = -1;
pub const TIMEOUT_NEVER: i32 = 0;

widget_style_enum!(
    name: NotificationStyle,
    doc: "Style flags for NotificationMessage.",
    variants: {
        None: 0, "No icon. This is the default style.",
        Information: ffi::WXD_ICON_INFORMATION, "Show an information icon.",
        Warning: ffi::WXD_ICON_WARNING, "Show a warning icon.",
        Error: ffi::WXD_ICON_ERROR, "Show an error icon.",
        Question: ffi::WXD_ICON_QUESTION, "Show a question icon."
    },
    default_variant: None
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationMessageEvent {
    Click,
    Dismissed,
    Action,
}

#[derive(Debug)]
pub struct NotificationMessageEventData {
    event: Event,
}

impl NotificationMessageEventData {
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }
}

#[derive(Debug)]
pub struct NotificationMessage {
    ptr: *mut ffi::wxd_NotificationMessage_t,
}

impl NotificationMessage {
    pub fn builder() -> NotificationMessageBuilder {
        NotificationMessageBuilder::new()
    }

    pub fn msw_use_toasts(shortcut_path: &str, app_id: &str) -> bool {
        let c_path = CString::new(shortcut_path).unwrap_or_default();
        let c_id = CString::new(app_id).unwrap_or_default();
        unsafe { ffi::wxd_NotificationMessage_MSWUseToasts(c_path.as_ptr(), c_id.as_ptr()) }
    }

    pub fn use_taskbar_icon(&self, icon: &TaskBarIcon) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::wxd_NotificationMessage_UseTaskBarIcon(self.ptr, icon.as_ptr());
            }
        }
    }

    pub fn set_icon_bundle(&self, icon: &BitmapBundle) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::wxd_NotificationMessage_SetIcon(self.ptr, icon.as_ptr());
            }
        }
    }

    pub fn show(&self, timeout: i32) -> bool {
        unsafe { ffi::wxd_NotificationMessage_Show(self.ptr, timeout as c_int) }
    }

    pub fn close(&self) -> bool {
        unsafe { ffi::wxd_NotificationMessage_Close(self.ptr) }
    }

    pub fn set_title(&self, title: &str) -> WxResult<()> {
        let c_title = CString::new(title)?;
        unsafe {
            ffi::wxd_NotificationMessage_SetTitle(self.ptr, c_title.as_ptr());
        }
        Ok(())
    }

    pub fn set_message(&self, message: &str) -> WxResult<()> {
        let c_message = CString::new(message)?;
        unsafe {
            ffi::wxd_NotificationMessage_SetMessage(self.ptr, c_message.as_ptr());
        }
        Ok(())
    }

    pub fn set_style(&self, style: NotificationStyle) -> WxResult<()> {
        unsafe {
            ffi::wxd_NotificationMessage_SetFlags(self.ptr, style.bits() as c_int);
        }
        Ok(())
    }

    pub fn set_parent<W: WxWidget>(&self, parent: Option<&W>) -> WxResult<()> {
        let parent_ptr = parent.map_or(std::ptr::null_mut(), |p| p.handle_ptr());
        unsafe {
            ffi::wxd_NotificationMessage_SetParent(self.ptr, parent_ptr);
        }
        Ok(())
    }

    pub fn add_action(&self, action_id: i32, label: &str) -> WxResult<bool> {
        if self.ptr.is_null() {
            return Err(Error::FfiCreation("NotificationMessage pointer is null".to_string()));
        }
        if action_id <= 0 {
            log::warn!("NotificationMessage action_id must be > 0.");
            return Ok(false);
        }
        let c_label = CString::new(label)?;
        let result = unsafe { ffi::wxd_NotificationMessage_AddAction(self.ptr, action_id, c_label.as_ptr()) };
        Ok(result)
    }

    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_NotificationMessage_Destroy(self.ptr) };
            self.ptr = std::ptr::null_mut();
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_ptr(&self) -> *mut ffi::wxd_NotificationMessage_t {
        self.ptr
    }
}

crate::implement_widget_local_event_handlers!(
    NotificationMessage,
    NotificationMessageEvent,
    NotificationMessageEventData,
    Click => click, EventType::NOTIFICATION_MESSAGE_CLICK,
    Dismissed => dismissed, EventType::NOTIFICATION_MESSAGE_DISMISSED,
    Action => action, EventType::NOTIFICATION_MESSAGE_ACTION
);

impl crate::event::WxEvtHandler for NotificationMessage {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

impl Drop for NotificationMessage {
    fn drop(&mut self) {
        self.destroy();
    }
}

pub struct NotificationMessageBuilder {
    title: String,
    message: String,
    parent: Option<*mut ffi::wxd_Window_t>,
    style: NotificationStyle,
    use_generic: bool,
}

impl Default for NotificationMessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationMessageBuilder {
    pub fn new() -> Self {
        NotificationMessageBuilder {
            title: String::new(),
            message: String::new(),
            parent: None,
            style: NotificationStyle::None,
            // Default to the generic backend on Windows: the native backend has a history of
            // causing app hangs and leaving persistent taskbar icons. Callers who want the
            // native (toast-capable) backend on Windows can opt in via `.with_generic(false)`.
            use_generic: cfg!(target_os = "windows"),
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn with_parent<W: WxWidget>(mut self, parent: &W) -> Self {
        self.parent = Some(parent.handle_ptr());
        self
    }

    pub fn with_style(mut self, style: NotificationStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_generic(mut self, generic: bool) -> Self {
        self.use_generic = generic;
        self
    }

    pub fn build(self) -> WxResult<NotificationMessage> {
        let c_title = CString::new(self.title)?;
        let msg = CString::new(self.message)?;
        let prt = self.parent.unwrap_or(std::ptr::null_mut());

        let ptr = unsafe {
            if self.use_generic {
                ffi::wxd_NotificationMessage_CreateGeneric(c_title.as_ptr(), msg.as_ptr(), prt, self.style.bits() as c_int)
            } else {
                ffi::wxd_NotificationMessage_Create(c_title.as_ptr(), msg.as_ptr(), prt, self.style.bits() as c_int)
            }
        };

        if ptr.is_null() {
            return Err(Error::FfiCreation("Failed to create notification message".to_string()));
        }

        Ok(NotificationMessage { ptr })
    }
}
