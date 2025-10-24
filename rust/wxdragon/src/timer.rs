//! Timer module for wxDragon.
//!
//! This module provides a safe wrapper around wxWidgets' wxTimer class.
//! Timers are used to generate events at regular intervals.

use crate::event::{Event, EventType, WxEvtHandler};
use std::marker::PhantomData;
use wxdragon_sys as ffi;

/// Represents a timer that triggers events at specified intervals.
///
/// A Timer is connected to an event handler (window, frame, etc.) and will
/// send timer events to it. You must bind a handler for timer events
/// directly on the timer.
///
/// # Example
///
/// ```rust,no_run
/// use wxdragon::prelude::*;
/// use wxdragon::timer::Timer;
///
/// let frame = Frame::builder()
///     .with_title("Timer Example")
///     .build();
///
/// // Create a timer connected to the frame
/// let timer = Timer::new(&frame);
///
/// // Bind an event handler directly on the timer
/// timer.on_tick(|_event| {
///     println!("Timer fired!");
/// });
///
/// // Start the timer to fire every 1000ms (1 second)
/// timer.start(1000, false);
/// ```
pub struct Timer<T: WxEvtHandler> {
    // Raw pointer to wxTimer
    ptr: *mut ffi::wxd_Timer_t,
    // Store the owner's pointer to use for event binding
    owner_ptr: *mut ffi::wxd_EvtHandler_t,
    // Phantom data to track the owner's type
    _owner: PhantomData<T>,
}

// Timer event handler methods
impl<T: WxEvtHandler> Timer<T> {
    /// Create a new timer associated with the given event handler.
    ///
    /// The handler will receive timer events when the timer fires.
    /// It must implement the WxEvtHandler trait (Windows, Frames, etc.)
    pub fn new(owner: &T) -> Self {
        let owner_ptr = unsafe { owner.get_event_handler_ptr() };
        let ptr = unsafe { ffi::wxd_Timer_Create(owner_ptr) };
        Self {
            ptr,
            owner_ptr,
            _owner: PhantomData,
        }
    }

    /// Bind an event handler for timer events.
    ///
    /// This method registers the callback to be called when the timer fires.
    pub fn on_tick<F>(&self, callback: F)
    where
        F: FnMut(Event) + 'static,
    {
        // We need to make sure the owner exists when using its pointer
        if !self.owner_ptr.is_null() {
            // Create a WxEvtHandler wrapper from the bare pointer
            let handler = TimerOwnerWrapper(self.owner_ptr);

            // Use bind_internal from the WxEvtHandler trait via the wrapper
            handler.bind_internal(EventType::TIMER, callback);
        }
    }

    /// Start the timer.
    ///
    /// # Arguments
    ///
    /// * `milliseconds` - The interval in milliseconds between timer events.
    /// * `one_shot` - If true, the timer will only fire once and then stop.
    ///   If false, the timer will keep firing at the specified interval.
    ///
    /// # Returns
    ///
    /// Returns true if the timer was successfully started, false otherwise.
    pub fn start(&self, milliseconds: i32, one_shot: bool) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Timer_Start(self.ptr, milliseconds, one_shot) }
    }

    /// Stop the timer.
    pub fn stop(&self) {
        if self.ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Timer_Stop(self.ptr) };
    }

    /// Check if the timer is currently running.
    pub fn is_running(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_Timer_IsRunning(self.ptr) }
    }

    /// Get the timer interval in milliseconds.
    pub fn get_interval(&self) -> i32 {
        if self.ptr.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_Timer_GetInterval(self.ptr) }
    }

    /// Set the timer interval in milliseconds.
    pub fn set_interval(&self, milliseconds: i32) {
        if self.ptr.is_null() {
            return;
        }
        unsafe { ffi::wxd_Timer_SetInterval(self.ptr, milliseconds) };
    }
}

impl<T: WxEvtHandler> Drop for Timer<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_Timer_Destroy(self.ptr) };
        }
    }
}

// This is a special wrapper to implement WxEvtHandler for the timer owner
// It allows us to call bind_internal on the owner from the Timer methods
struct TimerOwnerWrapper(*mut ffi::wxd_EvtHandler_t);

// Implement WxEvtHandler for the wrapper so we can register events
impl WxEvtHandler for TimerOwnerWrapper {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.0
    }
}
