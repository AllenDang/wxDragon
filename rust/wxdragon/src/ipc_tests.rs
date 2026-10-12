use super::*;
use std::cell::Cell;
use std::rc::Rc;

const FORMAT: ffi::wxd_IPCFormat = ffi::wxd_IPCFormat_WXD_IPC_TEXT;

macro_rules! boolean_reentry_test {
    ($name:ident, $field:ident, [$($arg:pat),*], $invoke:expr) => {
        #[test]
        fn $name() {
            let invoke: fn(*mut c_void) -> bool = $invoke;
            let ptr_cell = Rc::new(Cell::new(ptr::null_mut()));
            let ptr_in_callback = ptr_cell.clone();
            let calls = Rc::new(Cell::new(0));
            let recorded = calls.clone();
            let nested = Rc::new(Cell::new(true));
            let nested_recorded = nested.clone();
            let builder = IPCConnectionBuilder::new().$field(move |$($arg),*| {
                recorded.set(recorded.get() + 1);
                nested_recorded.set(invoke(ptr_in_callback.get()));
                true
            });
            let ptr = FfiCallbackData::into_raw(builder.callbacks);
            ptr_cell.set(ptr);
            assert!(invoke(ptr));
            assert!(!nested.get());
            assert_eq!(calls.get(), 1);
            assert!(invoke(ptr));
            assert_eq!(calls.get(), 2);
            unsafe { free_connection_callbacks(ptr) };
        }
    };
}

boolean_reentry_test!(reentrant_ipc_execute, on_execute, [_, _, _], |ptr| unsafe {
    on_execute_trampoline(ptr, c"topic".as_ptr(), ptr::null(), 0, FORMAT)
});
boolean_reentry_test!(reentrant_ipc_poke, on_poke, [_, _, _, _], |ptr| unsafe {
    on_poke_trampoline(ptr, c"topic".as_ptr(), c"item".as_ptr(), ptr::null(), 0, FORMAT)
});
boolean_reentry_test!(reentrant_ipc_start_advise, on_start_advise, [_, _], |ptr| unsafe {
    on_start_advise_trampoline(ptr, c"topic".as_ptr(), c"item".as_ptr())
});
boolean_reentry_test!(reentrant_ipc_stop_advise, on_stop_advise, [_, _], |ptr| unsafe {
    on_stop_advise_trampoline(ptr, c"topic".as_ptr(), c"item".as_ptr())
});
boolean_reentry_test!(reentrant_ipc_advise, on_advise, [_, _, _, _], |ptr| unsafe {
    on_advise_trampoline(ptr, c"topic".as_ptr(), c"item".as_ptr(), ptr::null(), 0, FORMAT)
});
boolean_reentry_test!(reentrant_ipc_disconnect, on_disconnect, [], |ptr| unsafe {
    on_disconnect_trampoline(ptr)
});

#[test]
fn reentrant_ipc_request_preserves_buffer_and_returns_empty_nested_response() {
    let ptr_cell = Rc::new(Cell::new(ptr::null_mut()));
    let ptr_recorded = ptr_cell.clone();
    let calls = Rc::new(Cell::new(0));
    let calls_recorded = calls.clone();
    let nested = Rc::new(Cell::new((false, usize::MAX)));
    let nested_recorded = nested.clone();
    let buffer_unchanged = Rc::new(Cell::new(false));
    let buffer_recorded = buffer_unchanged.clone();
    let builder = IPCConnectionBuilder::new().on_request(move |_, _, _| {
        calls_recorded.set(calls_recorded.get() + 1);
        let data = unsafe { FfiCallbackData::<ConnectionCallbacks>::retain(ptr_recorded.get()) };
        let previous = data.request_buffer.borrow().clone();
        let mut size = usize::MAX;
        let response =
            unsafe { on_request_trampoline(ptr_recorded.get(), c"topic".as_ptr(), c"item".as_ptr(), &mut size, FORMAT) };
        nested_recorded.set((response.is_null(), size));
        buffer_recorded.set(*data.request_buffer.borrow() == previous);
        Some(b"response".to_vec())
    });
    *builder.callbacks.request_buffer.borrow_mut() = b"previous".to_vec();
    let ptr = FfiCallbackData::into_raw(builder.callbacks);
    ptr_cell.set(ptr);
    for expected_calls in 1..=2 {
        let mut size = usize::MAX;
        let response = unsafe { on_request_trampoline(ptr, c"topic".as_ptr(), c"item".as_ptr(), &mut size, FORMAT) };
        assert!(!response.is_null());
        assert_eq!(size, 8);
        assert_eq!(
            unsafe { std::slice::from_raw_parts(response as *const u8, size) },
            b"response"
        );
        assert_eq!(nested.get(), (true, 0));
        assert!(buffer_unchanged.get());
        assert_eq!(calls.get(), expected_calls);
    }
    unsafe { free_connection_callbacks(ptr) };
}

#[test]
fn reentrant_ipc_callbacks_allow_distinct_callbacks_to_nest() {
    let ptr_cell = Rc::new(Cell::new(ptr::null_mut()));
    let ptr_recorded = ptr_cell.clone();
    let disconnect_calls = Rc::new(Cell::new(0));
    let recorded = disconnect_calls.clone();
    let builder = IPCConnectionBuilder::new()
        .on_execute(move |_, _, _| unsafe { on_disconnect_trampoline(ptr_recorded.get()) })
        .on_disconnect(move || {
            recorded.set(recorded.get() + 1);
            true
        });
    let ptr = FfiCallbackData::into_raw(builder.callbacks);
    ptr_cell.set(ptr);
    assert!(unsafe { on_execute_trampoline(ptr, ptr::null(), ptr::null(), 0, FORMAT) });
    assert_eq!(disconnect_calls.get(), 1);
    unsafe { free_connection_callbacks(ptr) };
}

#[test]
fn reentrant_ipc_accept_connection_returns_null() {
    let ptr_cell = Rc::new(Cell::new(ptr::null_mut()));
    let ptr_recorded = ptr_cell.clone();
    let calls = Rc::new(Cell::new(0));
    let recorded = calls.clone();
    let nested_is_null = Rc::new(Cell::new(false));
    let nested_recorded = nested_is_null.clone();
    let callbacks = ServerCallbacks {
        on_accept: ReentrantCallback::new(Box::new(move |_| {
            recorded.set(recorded.get() + 1);
            nested_recorded.set(unsafe { on_accept_connection_trampoline(ptr_recorded.get(), c"topic".as_ptr()).is_null() });
            None
        })),
    };
    let ptr = FfiCallbackData::into_raw(callbacks);
    ptr_cell.set(ptr);
    for expected in 1..=2 {
        assert!(unsafe { on_accept_connection_trampoline(ptr, c"topic".as_ptr()).is_null() });
        assert!(nested_is_null.get());
        assert_eq!(calls.get(), expected);
    }
    unsafe { free_server_callbacks(ptr) };
}

#[test]
fn reentrant_ipc_request_panic_releases_borrow_and_zeroes_size() {
    let calls = Rc::new(Cell::new(0));
    let recorded = calls.clone();
    let builder = IPCConnectionBuilder::new().on_request(move |_, _, _| {
        recorded.set(recorded.get() + 1);
        if recorded.get() == 1 {
            panic!("test panic");
        }
        Some(vec![42])
    });
    let ptr = FfiCallbackData::into_raw(builder.callbacks);
    let mut size = usize::MAX;
    assert!(unsafe { on_request_trampoline(ptr, ptr::null(), ptr::null(), &mut size, FORMAT) }.is_null());
    assert_eq!(size, 0);
    let response = unsafe { on_request_trampoline(ptr, ptr::null(), ptr::null(), &mut size, FORMAT) };
    assert!(!response.is_null());
    assert_eq!(size, 1);
    assert_eq!(unsafe { *(response as *const u8) }, 42);
    unsafe { free_connection_callbacks(ptr) };
    size = usize::MAX;
    assert!(unsafe { on_request_trampoline(ptr::null_mut(), ptr::null(), ptr::null(), &mut size, FORMAT) }.is_null());
    assert_eq!(size, 0);
}

#[test]
fn reentrant_ipc_request_cannot_return_storage_after_owner_is_freed() {
    let ptr_cell = Rc::new(Cell::new(ptr::null_mut()));
    let ptr_recorded = ptr_cell.clone();
    let captured = Rc::new(());
    let weak = Rc::downgrade(&captured);
    let builder = IPCConnectionBuilder::new().on_request(move |_, _, _| {
        let _keep_capture = &captured;
        unsafe { free_connection_callbacks(ptr_recorded.get()) };
        Some(vec![42])
    });
    let ptr = FfiCallbackData::into_raw(builder.callbacks);
    ptr_cell.set(ptr);
    let mut size = usize::MAX;
    assert!(unsafe { on_request_trampoline(ptr, ptr::null(), ptr::null(), &mut size, FORMAT) }.is_null());
    assert_eq!(size, 0);
    assert!(weak.upgrade().is_none());
}
