//! GUI regression for sliders whose keys raise the value for Up and Page Up. Requires a desktop.
//!
//! The keys need real key events, which only Windows can post to a window without taking focus,
//! so the check runs there and other platforms only build it.

#[cfg(windows)]
mod windows {
    use std::cell::{Cell, RefCell};
    use std::ffi::c_void;
    use std::rc::Rc;
    use wxdragon::prelude::*;

    #[link(name = "user32")]
    unsafe extern "system" {
        fn PostMessageW(window: *mut c_void, message: u32, wparam: usize, lparam: isize) -> i32;
    }

    const WM_KEYDOWN: u32 = 0x0100;
    const VK_PRIOR: usize = 0x21;
    const VK_NEXT: usize = 0x22;
    const VK_END: usize = 0x23;
    const VK_HOME: usize = 0x24;
    const VK_LEFT: usize = 0x25;
    const VK_UP: usize = 0x26;
    const VK_RIGHT: usize = 0x27;

    fn post(slider: Slider, key: usize) {
        let posted = unsafe { PostMessageW(slider.get_handle(), WM_KEYDOWN, key, 1) };
        assert_ne!(posted, 0, "could not post a key to the slider");
    }

    /// What the natural slider's value was after each key, and what each `on_slider` event said.
    #[derive(Default)]
    struct Seen {
        values: Vec<i32>,
        events: Vec<i32>,
        plain_after_up: Option<i32>,
    }

    pub fn run() {
        SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
        let seen = Rc::new(RefCell::new(Seen::default()));
        let seen_in_loop = seen.clone();
        let finished = Rc::new(Cell::new(false));
        let finished_in_loop = finished.clone();
        let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
        let timer_in_loop = timer_store.clone();
        wxdragon::main(move |app| {
            let frame = Frame::builder().with_title("Slider natural keys").build();
            let panel = Panel::builder(&frame).build();
            let natural = Slider::builder(&panel).with_value(50).with_natural_keys(true).build();
            let plain = Slider::builder(&panel).with_value(50).build();
            frame.show(true);
            let seen_events = seen_in_loop.clone();
            natural.on_slider(move |event| seen_events.borrow_mut().events.push(event.get_value()));
            // Record the value after each key by posting a marker behind it: a key the slider
            // ignores, which wxWidgets still delivers to this handler.
            let seen_values = seen_in_loop.clone();
            natural.on_key_up(move |event| {
                if let WindowEventData::Keyboard(key) = event {
                    seen_values.borrow_mut().values.push(natural.value());
                    key.event.skip(true);
                }
            });
            // Install the exit timer first: keys that never arrive must not hang the test.
            let timer = Timer::new(&frame);
            let timer_cleanup = timer_in_loop.clone();
            let seen_plain = seen_in_loop.clone();
            let finished_in_tick = finished_in_loop.clone();
            timer.on_tick(move |_| {
                seen_plain.borrow_mut().plain_after_up = Some(plain.value());
                finished_in_tick.set(true);
                timer_cleanup.borrow_mut().take();
                app.exit_main_loop();
            });
            timer.start(500, true);
            timer_in_loop.borrow_mut().replace(timer);
            const WM_KEYUP: u32 = 0x0101;
            for key in [VK_UP, VK_RIGHT, VK_PRIOR, VK_LEFT, VK_NEXT, VK_END, VK_UP, VK_HOME] {
                post(natural, key);
                let posted = unsafe { PostMessageW(natural.get_handle(), WM_KEYUP, key, 0xC000_0001u32 as isize) };
                assert_ne!(posted, 0, "could not post a key release to the slider");
            }
            post(plain, VK_UP);
        })
        .expect("could not initialize wxWidgets for slider natural key regressions");
        assert!(finished.get(), "slider natural key checks did not finish");
        let seen = seen.borrow();
        // Up, Right, Page Up, Left, Page Down, End, Up at the maximum, Home.
        assert_eq!(seen.values, [51, 52, 62, 61, 51, 100, 100, 0]);
        // Up at the maximum changes nothing, so it sends nothing.
        assert_eq!(seen.events, [51, 52, 62, 61, 51, 100, 0]);
        // Without natural keys, the Windows trackbar lowers the value for Up.
        assert_eq!(seen.plain_after_up, Some(49));
    }
}

fn main() {
    #[cfg(windows)]
    windows::run();
}
