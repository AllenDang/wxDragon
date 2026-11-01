//! GUI regression for `Window` values outliving the window they refer to. Requires a desktop.
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wxdragon::prelude::*;

/// Windows and flags carried from the setup phase to the checks that run once the deferred
/// destruction has happened.
struct Pending {
    notebook: Notebook,
    first: Panel,
    second: Panel,
    page: Window,
    destroyed_panel: Panel,
    destroy_seen: Rc<Cell<bool>>,
}

fn setup(frame: &Frame) -> Pending {
    let notebook = Notebook::builder(frame).build();
    let first = Panel::builder(&notebook).build();
    let second = Panel::builder(&notebook).build();
    assert!(notebook.add_page(&first, "First", true, None));
    assert!(notebook.add_page(&second, "Second", false, None));

    let page = notebook.get_page(0).expect("page 0 exists");
    assert!(page.is_valid());
    assert_eq!(page.as_ptr(), first.handle_ptr());

    // Looking the same window up again reuses its handle instead of registering a new one.
    assert_eq!(page.window_handle(), first.window_handle());
    assert_eq!(notebook.get_page(0), Some(page));

    // Take the page out of the notebook and destroy it while `page` is still held.
    assert!(notebook.remove_page(0));
    first.destroy();

    // A window's own destroy handler still runs, and the event object it gets is already
    // detached from the window that is going away.
    let destroyed_panel = Panel::builder(frame).build();
    let destroy_seen = Rc::new(Cell::new(false));
    let seen_in_handler = destroy_seen.clone();
    destroyed_panel.on_destroy(move |data| {
        let WindowEventData::General(event) = data else {
            panic!("destroy events carry general event data");
        };
        let object = event.get_event_object().expect("destroy event has a source");
        assert!(!object.is_valid());
        assert!(object.as_ptr().is_null());
        seen_in_handler.set(true);
    });
    destroyed_panel.destroy();

    Pending {
        notebook,
        first,
        second,
        page,
        destroyed_panel,
        destroy_seen,
    }
}

fn verify(pending: Pending) {
    let Pending {
        notebook,
        first,
        second,
        page,
        destroyed_panel,
        destroy_seen,
    } = pending;
    assert!(!first.is_valid(), "the removed page was not destroyed");

    // The stale Window must no longer reach the freed wxWindow.
    assert!(!page.is_valid());
    assert!(page.is_null());
    assert!(page.as_ptr().is_null());
    page.set_label("after destroy");
    assert_eq!(page.get_label(), None);
    assert!(page.get_parent().is_none());
    assert!(page.as_widget::<Panel>().is_none());

    // The remaining page is unaffected.
    let remaining = notebook.get_page(0).expect("second page is now at index 0");
    assert_eq!(remaining.window_handle(), second.window_handle());
    assert!(remaining.is_valid());

    assert!(destroy_seen.get(), "destroy handler did not run");
    assert!(!destroyed_panel.is_valid());
}

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let finished = Rc::new(Cell::new(false));
    let finished_in_loop = finished.clone();
    let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
    let timer_in_loop = timer_store.clone();
    wxdragon::main(move |app| {
        let frame = Frame::builder().with_title("Window after destroy").build();
        let pending = Rc::new(RefCell::new(None));
        // Child windows are destroyed at idle time, so the checks run from a timer.
        let timer = Timer::new(&frame);
        let timer_cleanup = timer_in_loop.clone();
        let pending_in_timer = pending.clone();
        let finished_in_timer = finished_in_loop.clone();
        timer.on_tick(move |_| {
            timer_cleanup.borrow_mut().take();
            // Ask the loop to exit before checking, so a failed assertion can't hang the test.
            app.exit_main_loop();
            if let Some(pending) = pending_in_timer.borrow_mut().take() {
                verify(pending);
                finished_in_timer.set(true);
            }
        });
        timer.start(100, true);
        timer_in_loop.borrow_mut().replace(timer);

        pending.borrow_mut().replace(setup(&frame));
    })
    .expect("could not initialize wxWidgets for window-after-destroy regressions");
    assert!(finished.get(), "window-after-destroy assertions did not finish");
}
