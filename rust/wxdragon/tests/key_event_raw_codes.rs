//! GUI regression for reading a key event's raw code and flags. Requires a desktop.
//!
//! Raw values are platform-specific and need a real key event, which only Windows can post to a
//! window without taking focus, so the check runs there and other platforms only build it.

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
    const VK_OEM_COMMA: u32 = 0xBC;
    /// A repeat count of 1 with the comma key's scan code in bits 16 to 23.
    const LPARAM: u32 = 1 | (0x33 << 16);

    pub fn run() {
        SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
        let seen = Rc::new(Cell::new(None));
        let seen_in_loop = seen.clone();
        let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
        let timer_in_loop = timer_store.clone();
        wxdragon::main(move |app| {
            let frame = Frame::builder().with_title("Key event raw codes").build();
            let text = TextCtrl::builder(&frame).build();
            frame.show(true);
            let seen_in_handler = seen_in_loop.clone();
            text.on_key_down(move |event| {
                if let WindowEventData::Keyboard(key) = event {
                    seen_in_handler.set(Some((key.get_raw_key_code(), key.get_raw_key_flags())));
                    key.event.skip(true);
                }
            });
            // Install the exit timer first: a key that never arrives must not hang the test.
            let timer = Timer::new(&frame);
            let timer_cleanup = timer_in_loop.clone();
            timer.on_tick(move |_| {
                timer_cleanup.borrow_mut().take();
                app.exit_main_loop();
            });
            timer.start(300, true);
            timer_in_loop.borrow_mut().replace(timer);
            let posted = unsafe { PostMessageW(text.get_handle(), WM_KEYDOWN, VK_OEM_COMMA as usize, LPARAM as isize) };
            assert_ne!(posted, 0, "could not post the key to the text control");
        })
        .expect("could not initialize wxWidgets for key event regressions");
        assert_eq!(seen.get(), Some((Some(VK_OEM_COMMA), Some(LPARAM))));
    }
}

fn main() {
    #[cfg(windows)]
    windows::run();
}
