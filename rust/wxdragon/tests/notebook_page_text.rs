//! GUI regression for changing a Notebook tab's text. Requires a desktop.
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wxdragon::prelude::*;

fn verify_page_text(notebook: Notebook) {
    let first = Panel::builder(&notebook).build();
    let second = Panel::builder(&notebook).build();
    assert!(notebook.add_page(&first, "Inbox", true, None));
    assert!(notebook.add_page(&second, "Sent", false, None));

    assert_eq!(notebook.get_page_text(0).as_deref(), Some("Inbox"));
    assert_eq!(notebook.get_page_text(1).as_deref(), Some("Sent"));

    // Text of more bytes than a fixed buffer holds comes back whole. It is kept to 255 characters,
    // because wxMSW reads a tab's text through a buffer of 256 characters.
    let long = "受信トレイ".repeat(40);
    assert!(long.chars().count() <= 255 && long.len() > 256);
    assert!(notebook.set_page_text(0, &long));
    assert_eq!(notebook.get_page_text(0), Some(long));
    assert!(notebook.set_page_text(1, ""));
    assert_eq!(notebook.get_page_text(1).as_deref(), Some(""));

    // Changing the text doesn't add, remove or select pages.
    assert_eq!(notebook.get_page_count(), 2);
    assert_eq!(notebook.selection(), 0);

    // An index with no page is refused, without the wxWidgets assertion.
    assert!(!notebook.set_page_text(2, "Nowhere"));
    assert_eq!(notebook.get_page_text(2), None);
    assert!(!notebook.set_page_text(0, "Bad\0text"));
    assert_eq!(notebook.get_page_text(1).as_deref(), Some(""));
}

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let finished = Rc::new(Cell::new(false));
    let finished_in_loop = finished.clone();
    let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
    let timer_in_loop = timer_store.clone();
    wxdragon::main(move |app| {
        let frame = Frame::builder().with_title("Notebook page text").build();
        let notebook = Notebook::builder(&frame).build();
        // Install the exit timer first: caught assertion panics must not hang the test.
        let timer = Timer::new(&frame);
        let timer_cleanup = timer_in_loop.clone();
        timer.on_tick(move |_| {
            timer_cleanup.borrow_mut().take();
            app.exit_main_loop();
        });
        timer.start(100, true);
        timer_in_loop.borrow_mut().replace(timer);

        verify_page_text(notebook);
        finished_in_loop.set(true);
    })
    .expect("could not initialize wxWidgets for notebook page text regressions");
    assert!(finished.get(), "notebook page text assertions did not finish");
}
