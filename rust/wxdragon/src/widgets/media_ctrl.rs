use crate::event::{Event, EventType, WindowEvents};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
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
            let media_ctrl = MediaCtrl { window: window_obj };
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
#[derive(Clone)]
pub struct MediaCtrl {
    window: Window,
}

impl MediaCtrl {
    /// Creates a new `MediaCtrlBuilder` for constructing a media control.
    pub fn builder(parent: &dyn WxWidget) -> MediaCtrlBuilder<'_> {
        MediaCtrlBuilder::new(parent)
    }

    /// Play the media.
    pub fn play(&self) -> bool {
        unsafe { ffi::wxd_MediaCtrl_Play(self.window.as_ptr() as *mut _) }
    }

    /// Pause the media.
    pub fn pause(&self) -> bool {
        unsafe { ffi::wxd_MediaCtrl_Pause(self.window.as_ptr() as *mut _) }
    }

    /// Stop the media.
    pub fn stop(&self) -> bool {
        unsafe { ffi::wxd_MediaCtrl_Stop(self.window.as_ptr() as *mut _) }
    }

    /// Load media from a file path.
    pub fn load(&self, file_name: &str) -> bool {
        let c_file_name = CString::new(file_name).expect("CString::new failed for file_name");
        unsafe { ffi::wxd_MediaCtrl_Load(self.window.as_ptr() as *mut _, c_file_name.as_ptr()) }
    }

    /// Load media from a URI.
    pub fn load_uri(&self, uri: &str) -> bool {
        let c_uri = CString::new(uri).expect("CString::new failed for uri");
        unsafe { ffi::wxd_MediaCtrl_LoadURI(self.window.as_ptr() as *mut _, c_uri.as_ptr()) }
    }

    /// Load media from a URI using a proxy.
    pub fn load_uri_with_proxy(&self, uri: &str, proxy: &str) -> bool {
        let c_uri = CString::new(uri).expect("CString::new failed for uri");
        let c_proxy = CString::new(proxy).expect("CString::new failed for proxy");
        unsafe {
            ffi::wxd_MediaCtrl_LoadURIWithProxy(
                self.window.as_ptr() as *mut _,
                c_uri.as_ptr(),
                c_proxy.as_ptr(),
            )
        }
    }

    /// Get the current state of the media.
    pub fn get_state(&self) -> MediaState {
        let state = unsafe { ffi::wxd_MediaCtrl_GetState(self.window.as_ptr() as *mut _) };

        match state as u32 {
            0 => MediaState::Stopped,
            1 => MediaState::Paused,
            2 => MediaState::Playing,
            _ => MediaState::Stopped, // Default to Stopped for unknown values
        }
    }

    /// Seek to a position in the media.
    pub fn seek(&self, where_: i64, mode: SeekMode) -> i64 {
        unsafe { ffi::wxd_MediaCtrl_Seek(self.window.as_ptr() as *mut _, where_, mode as i32) }
    }

    /// Get the current position in the media.
    pub fn tell(&self) -> i64 {
        unsafe { ffi::wxd_MediaCtrl_Tell(self.window.as_ptr() as *mut _) }
    }

    /// Get the length of the media.
    pub fn length(&self) -> i64 {
        unsafe { ffi::wxd_MediaCtrl_Length(self.window.as_ptr() as *mut _) }
    }

    /// Get the current playback rate.
    pub fn get_playback_rate(&self) -> f64 {
        unsafe { ffi::wxd_MediaCtrl_GetPlaybackRate(self.window.as_ptr() as *mut _) }
    }

    /// Set the playback rate.
    pub fn set_playback_rate(&self, rate: f64) -> bool {
        unsafe { ffi::wxd_MediaCtrl_SetPlaybackRate(self.window.as_ptr() as *mut _, rate) }
    }

    /// Get the download progress (DirectShow only).
    pub fn get_download_progress(&self) -> i64 {
        unsafe { ffi::wxd_MediaCtrl_GetDownloadProgress(self.window.as_ptr() as *mut _) }
    }

    /// Get the download total (DirectShow only).
    pub fn get_download_total(&self) -> i64 {
        unsafe { ffi::wxd_MediaCtrl_GetDownloadTotal(self.window.as_ptr() as *mut _) }
    }

    /// Get the current volume.
    pub fn get_volume(&self) -> f64 {
        unsafe { ffi::wxd_MediaCtrl_GetVolume(self.window.as_ptr() as *mut _) }
    }

    /// Set the volume.
    pub fn set_volume(&self, volume: f64) -> bool {
        unsafe { ffi::wxd_MediaCtrl_SetVolume(self.window.as_ptr() as *mut _, volume) }
    }

    /// Show player controls.
    pub fn show_player_controls(&self, controls: MediaCtrlPlayerControls) -> bool {
        unsafe {
            ffi::wxd_MediaCtrl_ShowPlayerControls(
                self.window.as_ptr() as *mut _,
                controls.bits() as u32 as i32,
            )
        }
    }
}

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

// Add XRC Support - enables MediaCtrl to be created from XRC-managed pointers
impl_xrc_support!(MediaCtrl, { window });

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
            window: unsafe { Window::from_ptr(ptr as *mut _) },
        }
    }
);

// Implement standard widget traits
implement_widget_traits_with_target!(MediaCtrl, window, Window);
