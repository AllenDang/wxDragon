//! GUI regression for telling the GUI thread from other threads. Requires a desktop.
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::mpsc;
use wxdragon::prelude::*;

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let finished = Rc::new(Cell::new(false));
    let finished_in_loop = finished.clone();
    let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
    let timer_in_loop = timer_store.clone();
    wxdragon::main(move |app| {
        let frame = Frame::builder().with_title("Is main thread").build();
        let timer = Timer::new(&frame);
        let timer_cleanup = timer_in_loop.clone();
        timer.on_tick(move |_| {
            timer_cleanup.borrow_mut().take();
            app.exit_main_loop();
        });
        timer.start(100, true);
        timer_in_loop.borrow_mut().replace(timer);

        assert!(is_main_thread(), "the GUI thread is not the main thread");
        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || sender.send(is_main_thread()).unwrap())
            .join()
            .unwrap();
        assert!(!receiver.recv().unwrap(), "a spawned thread counts as the main thread");
        finished_in_loop.set(true);
    })
    .expect("could not initialize wxWidgets for the main thread regression");
    assert!(finished.get(), "main thread assertions did not finish");
}
