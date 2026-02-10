use std::ffi::CString;
use wxdragon_sys as ffi;

widget_style_enum!(
    name: SoundFlags,
    doc: "Flags for playing sounds.",
    variants: {
        Sync: ffi::wxd_SoundFlags_WXD_SOUND_SYNC as i64, "Play sound synchronously (waits for sound to finish).",
        Async: ffi::wxd_SoundFlags_WXD_SOUND_ASYNC as i64, "Play sound asynchronously (doesn't wait).",
        Loop: ffi::wxd_SoundFlags_WXD_SOUND_LOOP as i64, "Loop the sound until stopped."
    },
    default_variant: Async
);

/// Represents a sound that can be played.
///
/// wxSound is typically limited to WAV files.
pub struct Sound {
    ptr: *mut ffi::wxd_Sound_t,
}

impl Sound {
    /// Creates a new sound from a file.
    ///
    /// # Arguments
    /// * `file_name` - Path to the sound file (usually .wav).
    /// * `is_resource` - If true, file_name is a resource name (Windows only).
    pub fn new(file_name: &str, is_resource: bool) -> Self {
        let c_file = CString::new(file_name).expect("CString::new failed");
        let ptr = unsafe { ffi::wxd_Sound_Create(c_file.as_ptr(), is_resource) };
        Self { ptr }
    }

    /// Returns true if the sound was created successfully.
    pub fn is_ok(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Sound_IsOk(self.ptr) }
    }

    /// Plays the sound with given flags.
    pub fn play(&self, flags: SoundFlags) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Sound_Play(self.ptr, flags.bits() as u32) }
    }

    /// Plays a sound file directly without creating a Sound object.
    pub fn play_file(file_name: &str, flags: SoundFlags) -> bool {
        let c_file = match CString::new(file_name) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_Sound_PlayFile(c_file.as_ptr(), flags.bits() as u32) }
    }

    /// Stops any currently playing sound.
    pub fn stop() {
        unsafe { ffi::wxd_Sound_Stop() }
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_Sound_Destroy(self.ptr) };
        }
    }
}

unsafe impl Send for Sound {}
