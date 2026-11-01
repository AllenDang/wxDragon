//! GUI regression for item data surviving a child window's destroy. Requires a desktop.
use std::cell::RefCell;
use std::rc::Rc;
use wxdragon::ffi;
use wxdragon::prelude::*;

/// Creates and destroys a child window that has no wxDragon event handler of its own, as the
/// label editor that wxWidgets deletes after every in-place edit has none. Its `wxEVT_DESTROY`
/// is a command event, so it propagates up to the parent's handlers. The destroy itself runs
/// on the next idle.
fn destroy_bare_child(parent: &dyn WxWidget) {
    let panel = unsafe {
        ffi::wxd_Panel_Create(
            parent.handle_ptr(),
            ffi::WXD_ID_ANY as _,
            ffi::wxd_Point { x: 0, y: 0 },
            ffi::wxd_Size { width: 10, height: 10 },
            0,
        )
    };
    assert!(!panel.is_null());
    unsafe { ffi::wxd_Window_Destroy(panel as *mut ffi::wxd_Window_t) };
}

type Check = Box<dyn Fn() -> Result<(), String>>;
type Results = Rc<RefCell<Option<Vec<Result<(), String>>>>>;

fn setup_tree(frame: &Frame) -> Check {
    let tree = TreeCtrl::builder(frame).build();
    let root = tree.add_root_with_data("Root", 7_i32, None, None).unwrap();
    let child = tree
        .append_item_with_data(&root, "Child", String::from("kept"), None, None)
        .unwrap();

    destroy_bare_child(&tree);

    Box::new(move || {
        let root_data = tree.get_custom_data_direct(&root).ok_or("tree root data was freed")?;
        if root_data.downcast_ref::<i32>() != Some(&7) {
            return Err("tree root data changed".into());
        }
        let child_data = tree.get_custom_data_direct(&child).ok_or("tree child data was freed")?;
        if child_data.downcast_ref::<String>().map(String::as_str) != Some("kept") {
            return Err("tree child data changed".into());
        }
        Ok(())
    })
}

fn setup_list(frame: &Frame) -> Check {
    let list = ListCtrl::builder(frame).with_style(ListCtrlStyle::Report).build();
    assert_eq!(list.insert_column(0, "Name", ListColumnFormat::Left, 100), 0);
    assert_eq!(list.insert_item(0, "First", None), 0);
    assert_ne!(list.set_custom_data(0u64, 42_u32), 0);

    destroy_bare_child(&list);

    Box::new(move || {
        let data = list.get_custom_data(0u64).ok_or("list data was freed")?;
        if data.downcast_ref::<u32>() != Some(&42) {
            return Err("list data changed".into());
        }
        Ok(())
    })
}

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let results: Results = Rc::new(RefCell::new(None));
    let results_in_loop = results.clone();
    let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
    let timer_in_loop = timer_store.clone();
    wxdragon::main(move |app| {
        let frame = Frame::builder().with_title("Tree and list child destroy").build();
        let checks = [setup_tree(&frame), setup_list(&frame)];

        // The checks run after the deferred destroys, from the timer. They report failures
        // instead of panicking, as a panic caught in an event handler would hang the test.
        let timer = Timer::new(&frame);
        let timer_cleanup = timer_in_loop.clone();
        let results_in_tick = results_in_loop.clone();
        timer.on_tick(move |_| {
            results_in_tick
                .borrow_mut()
                .replace(checks.iter().map(|check| check()).collect());
            timer_cleanup.borrow_mut().take();
            app.exit_main_loop();
        });
        timer.start(100, true);
        timer_in_loop.borrow_mut().replace(timer);
    })
    .expect("could not initialize wxWidgets for tree and list child destroy regressions");

    let results = results
        .borrow_mut()
        .take()
        .expect("tree and list child destroy checks did not run");
    for result in results {
        if let Err(message) = result {
            panic!("{message}");
        }
    }
}
