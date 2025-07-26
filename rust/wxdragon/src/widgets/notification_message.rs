//!
//! Safe wrapper for wxNotificationMessage.

// use crate::window::Window; // For parent type, though it's just a handle - Unused, removing
// use crate::base::WxResult; // Will define locally for now
use crate::event::{Event, EventType};
use crate::window::WxWidget;
use std::ffi::{CString, NulError};
use std::os::raw::c_int;
use wxdragon_sys as ffi; // Import WxWidget trait
                         // use log; // REMOVED: Using println! instead
use crate::widget_style_enum;

// --- Temporary Error Handling --- TODO: Refactor to use a crate-wide error type
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
// --- End Temporary Error Handling ---

// wxNotificationMessage specific constants that might be useful
// These values are from wxWidgets documentation for wxNotificationMessage::Show
pub const TIMEOUT_AUTO: i32 = -1; // Automatically determine the timeout
pub const TIMEOUT_NEVER: i32 = 0; // Never hide the notification automatically (manual Close() needed)

// Define NotificationStyle using widget_style_enum macro
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

/// Events emitted by NotificationMessage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationMessageEvent {
    /// Emitted when the notification is clicked
    Click,
    /// Emitted when the notification is dismissed
    Dismissed,
    /// Emitted when an action button is clicked
    Action,
}

/// Event data for NotificationMessageEvent events
#[derive(Debug)]
pub struct NotificationMessageEventData {
    event: Event,
}

impl NotificationMessageEventData {
    /// Create a new NotificationMessageEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the ID of the notification or the action button that was clicked
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }
}

/// Represents a `wxNotificationMessage`.
///
/// This struct manages a pointer to the underlying C++ `wxNotificationMessage` object.
/// It is responsible for calling `wxd_NotificationMessage_Destroy` when it goes out of scope.
#[derive(Debug)] // wxNotificationMessage is not Cloneable as it has direct destructor
pub struct NotificationMessage {
    ptr: *mut ffi::wxd_NotificationMessage_t,
}

unsafe impl Send for NotificationMessage {}
unsafe impl Sync for NotificationMessage {}

impl NotificationMessage {
    /// Creates a new `NotificationMessageBuilder`.
    pub fn builder() -> NotificationMessageBuilder {
        NotificationMessageBuilder::new()
    }

    /// Shows the notification to the user.
    ///
    /// # Arguments
    /// * `timeout` - How long the notification is shown, in seconds.
    ///   Use `TIMEOUT_AUTO` to let the system decide, or `TIMEOUT_NEVER` if it shouldn't time out.
    ///   A positive value specifies the timeout in seconds.
    ///
    /// Returns `true` if it was possible to show the notification, `false` if an error occurred.
    pub fn show(&self, timeout: i32) -> bool {
        unsafe { ffi::wxd_NotificationMessage_Show(self.ptr, timeout as c_int) }
    }

    /// Hides the notification.
    ///
    /// Returns `true` if the notification was hidden or `false` if it couldn't be (e.g. it was already hidden).
    pub fn close(&self) -> bool {
        unsafe { ffi::wxd_NotificationMessage_Close(self.ptr) }
    }

    /// Sets the main text of the notification.
    /// Returns `true` on success.
    pub fn set_title(&self, title: &str) -> WxResult<()> {
        let c_title = CString::new(title)?;
        unsafe {
            ffi::wxd_NotificationMessage_SetTitle(self.ptr, c_title.as_ptr());
        }
        Ok(())
    }

    /// Sets the secondary, more detailed, text of the notification.
    /// Returns `true` on success.
    pub fn set_message(&self, message: &str) -> WxResult<()> {
        let c_message = CString::new(message)?;
        unsafe {
            ffi::wxd_NotificationMessage_SetMessage(self.ptr, c_message.as_ptr());
        }
        Ok(())
    }

    /// Sets the style for the notification message.
    /// These flags typically control the icon shown.
    /// Returns `true` on success.
    pub fn set_style(&self, style: NotificationStyle) -> WxResult<()> {
        unsafe {
            ffi::wxd_NotificationMessage_SetFlags(self.ptr, style.bits() as c_int);
        }
        Ok(())
    }

    /// Sets the parent window for this notification.
    /// Returns `true` on success.
    pub fn set_parent<W: WxWidget>(&self, parent: Option<&W>) -> WxResult<()> {
        let parent_ptr = parent.map_or(std::ptr::null_mut(), |p| p.handle_ptr());
        unsafe {
            ffi::wxd_NotificationMessage_SetParent(self.ptr, parent_ptr);
        }
        Ok(())
    }

    /// Adds an action button to the notification.
    ///
    /// This method should be called after the `NotificationMessage` has been created
    /// but before `show()` is called for the actions to appear.
    /// The `action_id` will be returned by `event.get_id()` if the corresponding
    /// action button is clicked and an `EventType::NotificationMessageAction` is caught.
    ///
    /// # Arguments
    /// * `action_id` - An integer ID for this action. Must be > 0.
    /// * `label` - The text to display on the action button.
    ///
    /// Returns `true` if the action was added successfully, `false` otherwise (e.g., too many actions).
    pub fn add_action(&self, action_id: i32, label: &str) -> WxResult<bool> {
        if self.ptr.is_null() {
            return Err(Error::FfiCreation(
                "NotificationMessage pointer is null".to_string(),
            ));
        }
        if action_id <= 0 {
            // wxWidgets requires action IDs to be > 0
            // Consider returning a specific error type here
            println!("Warning: NotificationMessage action_id must be > 0.");
            return Ok(false);
        }
        let c_label = CString::new(label)?; // This can return NulError, which converts to Error::NulError
        let result = unsafe {
            ffi::wxd_NotificationMessage_AddAction(self.ptr, action_id, c_label.as_ptr())
        };
        Ok(result)
    }

    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_NotificationMessage_Destroy(self.ptr) };
            self.ptr = std::ptr::null_mut();
        }
    }

    /// Gets the raw pointer to the notification message
    #[allow(dead_code)]
    pub(crate) fn get_ptr(&self) -> *mut ffi::wxd_NotificationMessage_t {
        self.ptr
    }
}

// Implement event handlers for NotificationMessage
crate::implement_widget_local_event_handlers!(
    NotificationMessage,
    NotificationMessageEvent,
    NotificationMessageEventData,
    Click => click, EventType::NOTIFICATION_MESSAGE_CLICK,
    Dismissed => dismissed, EventType::NOTIFICATION_MESSAGE_DISMISSED,
    Action => action, EventType::NOTIFICATION_MESSAGE_ACTION
);

// Special implementation of WxEvtHandler for NotificationMessage
impl crate::event::WxEvtHandler for NotificationMessage {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t
    }
}

// Since NotificationMessage is not a true window, we don't implement WindowEvents
// However, we can still bind specific events defined above

impl Drop for NotificationMessage {
    fn drop(&mut self) {
        self.destroy();
    }
}

/// Builder for `NotificationMessage` instances.
#[derive(Default)]
pub struct NotificationMessageBuilder {
    title: String,
    message: String,
    parent: Option<*mut ffi::wxd_Window_t>,
    style: NotificationStyle,
}

impl NotificationMessageBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        NotificationMessageBuilder {
            title: String::new(),
            message: String::new(),
            parent: None,
            style: NotificationStyle::None,
        }
    }

    /// Sets the main title of the notification.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the detailed message of the notification.
    pub fn with_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    /// Sets the parent window.
    pub fn with_parent<W: WxWidget>(mut self, parent: &W) -> Self {
        self.parent = Some(parent.handle_ptr());
        self
    }

    /// Sets the style (e.g., for icons like `NotificationStyle::Information`).
    pub fn with_style(mut self, style: NotificationStyle) -> Self {
        self.style = style;
        self
    }

    /// Builds the `NotificationMessage`.
    ///
    /// Returns `Some(NotificationMessage)` on success, or `None` if creation failed.
    pub fn build(self) -> WxResult<NotificationMessage> {
        let c_title = CString::new(self.title)?;
        let c_message = CString::new(self.message)?;
        let parent_ptr = self.parent.unwrap_or(std::ptr::null_mut());

        let ptr = unsafe {
            ffi::wxd_NotificationMessage_Create(
                c_title.as_ptr(),
                c_message.as_ptr(),
                parent_ptr,
                self.style.bits() as c_int,
            )
        };

        if ptr.is_null() {
            return Err(Error::FfiCreation(
                "Failed to create notification message".to_string(),
            ));
        }

        Ok(NotificationMessage { ptr })
    }
}
