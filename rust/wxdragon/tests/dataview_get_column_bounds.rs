//! GUI regression for asking a DataView control for a column it doesn't have. Requires a desktop.
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wxdragon::prelude::*;

fn verify_get_column_bounds(frame: &Frame) {
    let tree = DataViewTreeCtrl::builder(frame).build();
    assert!(tree.append_text_column("Extra", 0, 80, DataViewAlign::Left, DataViewColumnFlags::Resizable));

    // wxDataViewTreeCtrl makes its own icon-text column, so there are two now.
    assert!(tree.get_column(0).is_some());
    assert!(tree.get_column(1).is_some());

    // Positions past the end are refused instead of reading past the column vector.
    assert!(tree.get_column(2).is_none());
    assert!(tree.get_column(1000).is_none());
    // A position that doesn't fit the C ABI's u32 must not wrap around to column 0.
    assert!(tree.get_column(u32::MAX as usize + 1).is_none());

    let ctrl = DataViewCtrl::builder(frame).build();
    assert!(ctrl.get_column(0).is_none());
}

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let finished = Rc::new(Cell::new(false));
    let finished_in_loop = finished.clone();
    let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
    let timer_in_loop = timer_store.clone();
    wxdragon::main(move |app| {
        let frame = Frame::builder().with_title("DataView column bounds").build();
        // Install the exit timer first: caught assertion panics must not hang the test.
        let timer = Timer::new(&frame);
        let timer_cleanup = timer_in_loop.clone();
        timer.on_tick(move |_| {
            timer_cleanup.borrow_mut().take();
            app.exit_main_loop();
        });
        timer.start(100, true);
        timer_in_loop.borrow_mut().replace(timer);

        verify_get_column_bounds(&frame);
        finished_in_loop.set(true);
    })
    .expect("could not initialize wxWidgets for DataView column bounds regressions");
    assert!(finished.get(), "DataView column bounds assertions did not finish");
}
