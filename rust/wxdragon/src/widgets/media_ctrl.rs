use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
// Window is used by MediaCtrlEventData for backwards compatibility
#[allow(unused_imports)]
use crate::window::Window;
use std::ffi::CString;
use wxdragon_sys as ffi;

// Define a style enum for MediaCtrl
widget_style_enum!(
    name: MediaCtrlStyle,
    doc: "Style flags for MediaCtrl widget.",
    variants: {
        NoAutoResize: ffi::WXD_MC_NO_AUTORESIZE, "Don't automatically resize the media to match the control size."
    },
    default_variant: NoAutoResize
);

widget_style_enum!(
    name: MediaState,
    doc: "State of the media player.",
    variants: {
        Stopped: ffi::WXD_MEDIASTATE_STOPPED, "Media is stopped.",
        Paused: ffi::WXD_MEDIASTATE_PAUSED, "Media is paused.",
        Playing: ffi::WXD_MEDIASTATE_PLAYING, "Media is playing."
    },
    default_variant: Stopped
);

widget_style_enum!(
    name: MediaCtrlPlayerControls,
    doc: "Player controls for the media player.",
    variants: {
        None: ffi::WXD_MEDIACTRLPLAYERCONTROLS_NONE, "No player controls.",
        Step: ffi::WXD_MEDIACTRLPLAYERCONTROLS_STEP, "Step player controls.",
        Volume: ffi::WXD_MEDIACTRLPLAYERCONTROLS_VOLUME, "Volume player controls.",
        Default: ffi::WXD_MEDIACTRLPLAYERCONTROLS_DEFAULT, "Default player controls."
    },
    default_variant: Default
);

/// Events emitted by MediaCtrl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaCtrlEvent {
    /// Emitted when media is successfully loaded
    Loaded,
    /// Emitted when media is stopped
    Stop,
    /// Emitted when media playback has finished
    Finished,
    /// Emitted when media state changes
    StateChanged,
    /// Emitted when media starts playing
    Play,
    /// Emitted when media is paused
    Pause,
}

/// Event data for MediaCtrl events
#[derive(Debug)]
pub struct MediaCtrlEventData {
    event: Event,
}

impl MediaCtrlEventData {
    /// Create a new MediaCtrlEventData from a generic Event
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Get the current state of the media player
    pub fn get_state(&self) -> Option<MediaState> {
        // Since the event doesn't provide state information directly,
        // we can get the mediaCtrl from the event source and query it
        if let Some(window_obj) = self.event.get_event_object() {
            let media_ctrl = MediaCtrl {
                handle: WindowHandle::new(window_obj.as_ptr()),
            };
            return Some(media_ctrl.get_state());
        }
        None
    }
}

/// Represents a seek mode for media controls and similar use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum SeekMode {
    /// Seek from start of media (offset is positive from the beginning)
    #[default]
    FromStart = 0, // wxFromStart
    /// Seek from current position (offset can be negative or positive)
    FromCurrent = 1, // wxFromCurrent
    /// Seek from end of media (offset is usually negative from the end)
    FromEnd = 2, // wxFromEnd
}

/// A wxWidgets media player control
///
/// MediaCtrl uses `WindowHandle` internally for safe memory management.
/// When the underlying window is destroyed (by calling `destroy()` or when
/// its parent is destroyed), the handle becomes invalid and all operations
/// become safe no-ops or return default values.
#[derive(Clone, Copy)]
pub struct MediaCtrl {
    /// Safe handle to the underlying wxMediaCtrl - automatically invalidated on destroy
    handle: WindowHandle,
}

impl MediaCtrl {
    /// Creates a new `MediaCtrlBuilder` for constructing a media control.
    pub fn builder(parent: &dyn WxWidget) -> MediaCtrlBuilder<'_> {
        MediaCtrlBuilder::new(parent)
    }

    /// Helper to get raw media ctrl pointer, returns null if widget has been destroyed
    #[inline]
    fn media_ctrl_ptr(&self) -> *mut ffi::wxd_MediaCtrl_t {
        self.handle
            .get_ptr()
            .map(|p| p as *mut ffi::wxd_MediaCtrl_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Play the media.
    /// Returns false if the widget has been destroyed.
    pub fn play(&self) -> bool {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_MediaCtrl_Play(ptr) }
    }

    /// Pause the media.
    /// Returns false if the widget has been destroyed.
    pub fn pause(&self) -> bool {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_MediaCtrl_Pause(ptr) }
    }

    /// Stop the media.
    /// Returns false if the widget has been destroyed.
    pub fn stop(&self) -> bool {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_MediaCtrl_Stop(ptr) }
    }

    /// Load media from a file path.
    /// Returns false if the widget has been destroyed.
    pub fn load(&self, file_name: &str) -> bool {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_file_name = CString::new(file_name).expect("CString::new failed for file_name");
        unsafe { ffi::wxd_MediaCtrl_Load(ptr, c_file_name.as_ptr()) }
    }

    /// Load media from a URI.
    /// Returns false if the widget has been destroyed.
    pub fn load_uri(&self, uri: &str) -> bool {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_uri = CString::new(uri).expect("CString::new failed for uri");
        unsafe { ffi::wxd_MediaCtrl_LoadURI(ptr, c_uri.as_ptr()) }
    }

    /// Load media from a URI using a proxy.
    /// Returns false if the widget has been destroyed.
    pub fn load_uri_with_proxy(&self, uri: &str, proxy: &str) -> bool {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        let c_uri = CString::new(uri).expect("CString::new failed for uri");
        let c_proxy = CString::new(proxy).expect("CString::new failed for proxy");
        unsafe { ffi::wxd_MediaCtrl_LoadURIWithProxy(ptr, c_uri.as_ptr(), c_proxy.as_ptr()) }
    }

    /// Get the current state of the media.
    /// Returns Stopped if the widget has been destroyed.
    pub fn get_state(&self) -> MediaState {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return MediaState::Stopped;
        }
        let state = unsafe { ffi::wxd_MediaCtrl_GetState(ptr) };

        match state as u32 {
            0 => MediaState::Stopped,
            1 => MediaState::Paused,
            2 => MediaState::Playing,
            _ => MediaState::Stopped, // Default to Stopped for unknown values
        }
    }

    /// Seek to a position in the media.
    /// Returns 0 if the widget has been destroyed.
    pub fn seek(&self, where_: i64, mode: SeekMode) -> i64 {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_MediaCtrl_Seek(ptr, where_, mode as i32) }
    }

    /// Get the current position in the media.
    /// Returns 0 if the widget has been destroyed.
    pub fn tell(&self) -> i64 {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_MediaCtrl_Tell(ptr) }
    }

    /// Get the length of the media.
    /// Returns 0 if the widget has been destroyed.
    pub fn length(&self) -> i64 {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_MediaCtrl_Length(ptr) }
    }

    /// Get the current playback rate.
    /// Returns 0.0 if the widget has been destroyed.
    pub fn get_playback_rate(&self) -> f64 {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return 0.0;
        }
        unsafe { ffi::wxd_MediaCtrl_GetPlaybackRate(ptr) }
    }

    /// Set the playback rate.
    /// Returns false if the widget has been destroyed.
    pub fn set_playback_rate(&self, rate: f64) -> bool {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_MediaCtrl_SetPlaybackRate(ptr, rate) }
    }

    /// Get the download progress (DirectShow only).
    /// Returns 0 if the widget has been destroyed.
    pub fn get_download_progress(&self) -> i64 {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_MediaCtrl_GetDownloadProgress(ptr) }
    }

    /// Get the download total (DirectShow only).
    /// Returns 0 if the widget has been destroyed.
    pub fn get_download_total(&self) -> i64 {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_MediaCtrl_GetDownloadTotal(ptr) }
    }

    /// Get the current volume.
    /// Returns 0.0 if the widget has been destroyed.
    pub fn get_volume(&self) -> f64 {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return 0.0;
        }
        unsafe { ffi::wxd_MediaCtrl_GetVolume(ptr) }
    }

    /// Set the volume.
    /// Returns false if the widget has been destroyed.
    pub fn set_volume(&self, volume: f64) -> bool {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_MediaCtrl_SetVolume(ptr, volume) }
    }

    /// Show player controls.
    /// Returns false if the widget has been destroyed.
    pub fn show_player_controls(&self, controls: MediaCtrlPlayerControls) -> bool {
        let ptr = self.media_ctrl_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_MediaCtrl_ShowPlayerControls(ptr, controls.bits() as u32 as i32) }
    }

    /// Returns the underlying WindowHandle for this media control.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

// Manual WxWidget implementation for MediaCtrl (using WindowHandle)
impl WxWidget for MediaCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

// Implement WxEvtHandler for event binding
impl WxEvtHandler for MediaCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

// Implement common event traits that all Window-based widgets support
impl crate::event::WindowEvents for MediaCtrl {}

// Implement event handlers for MediaCtrl
crate::implement_widget_local_event_handlers!(
    MediaCtrl,
    MediaCtrlEvent,
    MediaCtrlEventData,
    Loaded => loaded, EventType::MEDIA_LOADED,
    Stop => stop, EventType::MEDIA_STOP,
    Finished => finished, EventType::MEDIA_FINISHED,
    StateChanged => state_changed, EventType::MEDIA_STATECHANGED,
    Play => play, EventType::MEDIA_PLAY,
    Pause => pause, EventType::MEDIA_PAUSE
);

// XRC Support - enables MediaCtrl to be created from XRC-managed pointers
#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for MediaCtrl {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        MediaCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}

// Create the builder for MediaCtrl
widget_builder!(
    name: MediaCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: MediaCtrlStyle,
    fields: {
        file_name: String = String::new(),
        backend_name: String = String::new()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        let c_file_name = CString::new(slf.file_name.clone()).unwrap();
        let c_backend_name = CString::new(slf.backend_name.clone()).unwrap();

        let ptr = unsafe {
            ffi::wxd_MediaCtrl_Create(
                parent_ptr,
                slf.id,
                c_file_name.as_ptr(),
                slf.pos.x, slf.pos.y,
                slf.size.width, slf.size.height,
                slf.style.bits(),
                c_backend_name.as_ptr(),
            )
        };

        assert!(!ptr.is_null(), "Failed to create MediaCtrl");

        MediaCtrl {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }
);

// Enable widget casting for MediaCtrl
impl crate::window::FromWindowWithClassName for MediaCtrl {
    fn class_name() -> &'static str {
        "wxMediaCtrl"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        MediaCtrl {
            handle: WindowHandle::new(ptr),
        }
    }
}
