use super::*;
use std::cell::Cell;
use std::rc::Rc;

fn text_callbacks() -> TextDropTargetCallbacks {
    TextDropTargetCallbacks {
        on_enter: None,
        on_drag_over: None,
        on_leave: None,
        on_drop: None,
        on_data: None,
        on_drop_text: ReentrantCallback::new(Box::new(|_, _, _| true)),
    }
}

fn file_callbacks() -> FileDropTargetCallbacks {
    FileDropTargetCallbacks {
        on_enter: None,
        on_drag_over: None,
        on_leave: None,
        on_drop: None,
        on_data: None,
        on_drop_files: ReentrantCallback::new(Box::new(|_, _, _| true)),
    }
}

// Exercise each drag-result trampoline while its own callback is already borrowed.
macro_rules! drag_reentry_test {
    ($name:ident, $callbacks:ident, $container:ty, $field:ident, $trampoline:ident, $free:ident) => {
        #[test]
        fn $name() {
            let calls = Rc::new(Cell::new(0));
            let recorded = calls.clone();
            let mut callbacks = $callbacks();
            callbacks.$field = Some(ReentrantCallback::new(Box::new(move |_, _, result| {
                recorded.set(recorded.get() + 1);
                result
            })));
            let ptr = FfiCallbackData::into_raw(callbacks);
            let data = unsafe { FfiCallbackData::<$container>::retain(ptr) };
            // Bindgen's C enum representation can be signed or unsigned by target.
            let nested = data
                .$field
                .as_ref()
                .unwrap()
                .call("outer", ffi::wxd_DragResult::MAX, |_| {
                    $trampoline(1, 2, ffi::wxd_DragResult_WXD_DRAG_COPY, ptr)
                });
            assert_eq!(nested, ffi::wxd_DragResult_WXD_DRAG_NONE);
            assert_eq!(calls.get(), 0);
            assert_eq!(
                $trampoline(1, 2, ffi::wxd_DragResult_WXD_DRAG_COPY, ptr),
                ffi::wxd_DragResult_WXD_DRAG_COPY
            );
            assert_eq!(calls.get(), 1);
            $free(ptr);
        }
    };
}

drag_reentry_test!(
    reentrant_text_enter,
    text_callbacks,
    TextDropTargetCallbacks,
    on_enter,
    text_on_enter_trampoline,
    free_text_drop_target_userdata
);
drag_reentry_test!(
    reentrant_text_drag_over,
    text_callbacks,
    TextDropTargetCallbacks,
    on_drag_over,
    text_on_drag_over_trampoline,
    free_text_drop_target_userdata
);
drag_reentry_test!(
    reentrant_text_data,
    text_callbacks,
    TextDropTargetCallbacks,
    on_data,
    text_on_data_trampoline,
    free_text_drop_target_userdata
);
drag_reentry_test!(
    reentrant_file_enter,
    file_callbacks,
    FileDropTargetCallbacks,
    on_enter,
    file_on_enter_trampoline,
    free_file_drop_target_userdata
);
drag_reentry_test!(
    reentrant_file_drag_over,
    file_callbacks,
    FileDropTargetCallbacks,
    on_drag_over,
    file_on_drag_over_trampoline,
    free_file_drop_target_userdata
);
drag_reentry_test!(
    reentrant_file_data,
    file_callbacks,
    FileDropTargetCallbacks,
    on_data,
    file_on_data_trampoline,
    free_file_drop_target_userdata
);

macro_rules! drop_reentry_test {
    ($name:ident, $callbacks:ident, $container:ty, $trampoline:ident, $free:ident) => {
        #[test]
        fn $name() {
            let mut callbacks = $callbacks();
            callbacks.on_drop = Some(ReentrantCallback::new(Box::new(|_, _| true)));
            let ptr = FfiCallbackData::into_raw(callbacks);
            let data = unsafe { FfiCallbackData::<$container>::retain(ptr) };
            assert!(
                !data
                    .on_drop
                    .as_ref()
                    .unwrap()
                    .call("outer", true, |_| $trampoline(1, 2, ptr))
            );
            assert!($trampoline(1, 2, ptr));
            $free(ptr);
        }
    };
}
drop_reentry_test!(
    reentrant_text_drop,
    text_callbacks,
    TextDropTargetCallbacks,
    text_on_drop_trampoline,
    free_text_drop_target_userdata
);
drop_reentry_test!(
    reentrant_file_drop,
    file_callbacks,
    FileDropTargetCallbacks,
    file_on_drop_trampoline,
    free_file_drop_target_userdata
);

macro_rules! leave_reentry_test {
    ($name:ident, $callbacks:ident, $container:ty, $trampoline:ident, $free:ident) => {
        #[test]
        fn $name() {
            let calls = Rc::new(Cell::new(0));
            let recorded = calls.clone();
            let mut callbacks = $callbacks();
            callbacks.on_leave = Some(ReentrantCallback::new(Box::new(move || {
                recorded.set(recorded.get() + 1)
            })));
            let ptr = FfiCallbackData::into_raw(callbacks);
            let data = unsafe { FfiCallbackData::<$container>::retain(ptr) };
            data.on_leave.as_ref().unwrap().call("outer", (), |_| $trampoline(ptr));
            assert_eq!(calls.get(), 0);
            $trampoline(ptr);
            assert_eq!(calls.get(), 1);
            $free(ptr);
        }
    };
}
leave_reentry_test!(
    reentrant_text_leave,
    text_callbacks,
    TextDropTargetCallbacks,
    text_on_leave_trampoline,
    free_text_drop_target_userdata
);
leave_reentry_test!(
    reentrant_file_leave,
    file_callbacks,
    FileDropTargetCallbacks,
    file_on_leave_trampoline,
    free_file_drop_target_userdata
);

#[test]
fn reentrant_drop_text_rejects_nested_call_but_allows_other_callbacks() {
    let ptr_cell = Rc::new(Cell::new(std::ptr::null_mut()));
    let ptr_in_callback = ptr_cell.clone();
    let nested_result = Rc::new(Cell::new(true));
    let nested_recorded = nested_result.clone();
    let leave_calls = Rc::new(Cell::new(0));
    let leave_recorded = leave_calls.clone();
    let mut callbacks = text_callbacks();
    callbacks.on_leave = Some(ReentrantCallback::new(Box::new(move || {
        leave_recorded.set(leave_recorded.get() + 1)
    })));
    callbacks.on_drop_text = ReentrantCallback::new(Box::new(move |_, _, _| {
        nested_recorded.set(text_on_drop_text_trampoline(c"nested".as_ptr(), 1, 2, ptr_in_callback.get()));
        text_on_leave_trampoline(ptr_in_callback.get());
        true
    }));
    let ptr = FfiCallbackData::into_raw(callbacks);
    ptr_cell.set(ptr);
    assert!(text_on_drop_text_trampoline(c"outer".as_ptr(), 1, 2, ptr));
    assert!(!nested_result.get());
    assert_eq!(leave_calls.get(), 1);
    assert!(text_on_drop_text_trampoline(c"later".as_ptr(), 1, 2, ptr));
    assert_eq!(leave_calls.get(), 2);
    free_text_drop_target_userdata(ptr);
}

#[test]
fn reentrant_drop_files_rejects_nested_call() {
    let ptr = FfiCallbackData::into_raw(file_callbacks());
    let data = unsafe { FfiCallbackData::<FileDropTargetCallbacks>::retain(ptr) };
    // wxArrayString is a value container and needs no GUI initialization.
    let filenames = unsafe { ffi::wxd_ArrayString_Create() };
    assert!(!filenames.is_null());
    assert!(
        !data
            .on_drop_files
            .call("outer", true, |_| file_on_drop_files_trampoline(filenames, 1, 2, ptr))
    );
    assert!(file_on_drop_files_trampoline(filenames, 1, 2, ptr));
    unsafe { ffi::wxd_ArrayString_Free(filenames) };
    free_file_drop_target_userdata(ptr);
}

#[test]
fn reentrant_drop_text_releases_borrow_after_panic() {
    let calls = Rc::new(Cell::new(0));
    let recorded = calls.clone();
    let mut callbacks = text_callbacks();
    callbacks.on_drop_text = ReentrantCallback::new(Box::new(move |_, _, _| {
        recorded.set(recorded.get() + 1);
        if recorded.get() == 1 {
            panic!("test panic");
        }
        true
    }));
    let ptr = FfiCallbackData::into_raw(callbacks);
    assert!(!text_on_drop_text_trampoline(c"text".as_ptr(), 1, 2, ptr));
    assert!(text_on_drop_text_trampoline(c"text".as_ptr(), 1, 2, ptr));
    free_text_drop_target_userdata(ptr);
}

#[test]
fn reentrant_drop_target_cleanup_waits_for_active_callback() {
    let captured = Rc::new(());
    let weak = Rc::downgrade(&captured);
    let ptr_cell = Rc::new(Cell::new(std::ptr::null_mut()));
    let ptr_recorded = ptr_cell.clone();
    let alive_after_release = Rc::new(Cell::new(false));
    let alive_recorded = alive_after_release.clone();
    let weak_recorded = weak.clone();
    let mut callbacks = text_callbacks();
    callbacks.on_drop_text = ReentrantCallback::new(Box::new(move |_, _, _| {
        let _keep_capture = &captured;
        free_text_drop_target_userdata(ptr_recorded.get());
        alive_recorded.set(weak_recorded.upgrade().is_some());
        true
    }));
    let ptr = FfiCallbackData::into_raw(callbacks);
    ptr_cell.set(ptr);
    assert!(text_on_drop_text_trampoline(c"text".as_ptr(), 1, 2, ptr));
    assert!(alive_after_release.get());
    assert!(weak.upgrade().is_none());
}
