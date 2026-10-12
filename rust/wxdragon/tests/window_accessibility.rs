//! GUI regressions run on the OS main thread. Requires a desktop (Xvfb on Linux).
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wxdragon::prelude::*;

struct NullWindow;
impl WxWidget for NullWindow {
    fn handle_ptr(&self) -> *mut wxdragon_sys::wxd_Window_t {
        std::ptr::null_mut()
    }
}

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let null_window = NullWindow;
    null_window.set_accessibility_label("名前");
    null_window.set_accessibility_label("");
    null_window.enable_focus_from_keyboard();
    null_window.disable_focus_from_keyboard();
    assert!(!null_window.accepts_focus_from_keyboard());

    let finished = Rc::new(Cell::new(false));
    let finished_in_loop = finished.clone();
    let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
    let timer_store_in_loop = timer_store.clone();
    wxdragon::main(move |app| {
        let frame = Frame::builder().with_title("Accessibility regressions").build();
        let panel = Panel::builder(&frame).build();
        let text = TextCtrl::builder(&panel).with_style(TextCtrlStyle::ReadOnly).build();

        // Install the exit timer before assertions: a panic caught at the FFI
        // boundary must still allow the main loop to terminate and report failure.
        let timer = Timer::new(&frame);
        let timer_cleanup = timer_store_in_loop.clone();
        timer.on_tick(move |_| {
            timer_cleanup.borrow_mut().take();
            app.exit_main_loop();
            // Destroy() can defer deletion until idle. Check after the event loop
            // has processed it, not immediately after requesting destruction.
            assert!(!text.is_valid());
            text.set_accessibility_label("Destroyed");
            text.set_accessibility_label("");
            text.enable_focus_from_keyboard();
            text.disable_focus_from_keyboard();
            assert!(!text.accepts_focus_from_keyboard());
            assert!(panel.is_valid());
            finished_in_loop.set(true);
        });
        timer.start(100, true);
        timer_store_in_loop.borrow_mut().replace(timer);

        #[cfg(target_os = "windows")]
        assert!(!text.accepts_focus_from_keyboard());
        text.enable_focus_from_keyboard();
        assert!(text.accepts_focus_from_keyboard());
        text.disable_focus_from_keyboard();
        assert!(!text.accepts_focus_from_keyboard());
        text.enable_focus_from_keyboard();
        assert!(text.accepts_focus_from_keyboard());
        text.set_accessibility_label("予約番号 — Réservation");
        text.set_accessibility_description("Reservation number");
        text.set_accessibility_label("");
        text.destroy();
    })
    .expect("could not run accessibility regressions");
    assert!(finished.get(), "accessibility assertions did not finish");
}
