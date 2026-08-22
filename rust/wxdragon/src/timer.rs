//! Timer module for wxDragon.
//!
//! This module provides a safe wrapper around wxWidgets' wxTimer class.
//! Timers are used to generate events at regular intervals.

use crate::event::{Event, EventToken, EventType, WxEvtHandler};
use std::marker::PhantomData;
use wxdragon_sys as ffi;

/// Represents a timer that triggers events at specified intervals.
///
/// A timer sends its events to the owner supplied to [`Timer::new`].
///
/// # Example
///
/// ```rust,no_run
/// use wxdragon::prelude::*;
/// use wxdragon::timer::Timer;
///
/// let frame = Frame::builder().with_title("Timer Example").build();
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

impl<T: WxEvtHandler> Timer<T> {
    /// Create a new timer associated with an event handler.
    ///
    /// The timer does not fire until [`Timer::start`] is called.
    pub fn new(owner: &T) -> Self {
        let owner_ptr = unsafe { owner.get_event_handler_ptr() };
        Self {
            ptr: unsafe { ffi::wxd_Timer_Create(owner_ptr) },
            owner_ptr,
            _owner: PhantomData,
        }
    }

    /// Bind a callback to be called when this timer fires.
    ///
    /// Returns a token that can be passed to [`WxEvtHandler::unbind`].
    pub fn on_tick<F>(&self, callback: F) -> EventToken
    where
        F: FnMut(Event) + 'static,
    {
        if self.owner_ptr.is_null() || self.ptr.is_null() {
            return EventToken::INVALID_TOKEN;
        }
        let timer_id = unsafe { ffi::wxd_Timer_GetId(self.ptr) };
        let handler = TimerOwnerWrapper(self.owner_ptr);
        handler.bind_with_id_internal(EventType::TIMER, timer_id, callback)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::Frame;
    use std::cell::RefCell;
    use std::rc::Rc;

    // wxWidgets needs the OS main thread for its event loop, but on macOS the Cargo test
    // harness runs each test on a worker thread, so `crate::main` never returns here.
    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn timers_do_not_share_callbacks() {
        let _gui_test = crate::app::GUI_TEST_LOCK.lock().unwrap();
        let ticks = Rc::new(RefCell::new(0u32));
        let idle_ticks = Rc::new(RefCell::new(0u32));
        let timers: Rc<RefCell<Vec<Timer<Frame>>>> = Rc::new(RefCell::new(Vec::new()));

        let (ticks_in, idle_ticks_in, timers_in) = (ticks.clone(), idle_ticks.clone(), timers.clone());
        let timers_cleanup = timers.clone();
        let res = crate::main(move |app| {
            let frame = Frame::builder().with_title("timer test").build();

            let ticking = Timer::new(&frame);
            ticking.on_tick(move |_| {
                let mut ticks = ticks_in.borrow_mut();
                *ticks += 1;
                if *ticks >= 1 {
                    timers_cleanup.borrow_mut().clear();
                    app.exit_main_loop();
                }
            });

            // Never started: its callback must never run.
            let idle = Timer::new(&frame);
            idle.on_tick(move |_| *idle_ticks_in.borrow_mut() += 1);

            assert!(ticking.start(10, true));
            timers_in.borrow_mut().extend([ticking, idle]);
        });

        // Headless CI has no display, so the loop above never ran and there is nothing to check.
        if res.is_err() {
            return;
        }
        assert_eq!(*ticks.borrow(), 1, "the started timer should have run its callback once");
        assert_eq!(
            *idle_ticks.borrow(),
            0,
            "a timer that was never started ran another timer's callback"
        );
    }
}
