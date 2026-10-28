//! Native re-entry regressions, run on the OS main thread (desktop/Xvfb required).
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wxdragon::event::EventToken;
use wxdragon::prelude::*;

const CMD_ID: i32 = 4321;

fn verify_id_binding(panel: Panel) {
    let log = Rc::new(RefCell::new(Vec::new()));
    let recorded = log.clone();
    let mut invocations = 0;
    panel.bind_with_id_internal(EventType::MENU, CMD_ID, move |_| {
        // Captured state is mutated directly, so FnMut compatibility is exercised.
        invocations += 1;
        recorded.borrow_mut().push(format!("a{invocations}-start"));
        panel.process_menu_command(CMD_ID);
        recorded.borrow_mut().push(format!("a{invocations}-end"));
    });
    let other = log.clone();
    panel.bind_with_id_internal(EventType::MENU, CMD_ID, move |_| {
        other.borrow_mut().push("b".to_owned());
    });
    for invocation in 1..=2 {
        log.borrow_mut().clear();
        panel.process_menu_command(CMD_ID);
        assert_eq!(
            *log.borrow(),
            [
                format!("a{invocation}-start"),
                "b".to_owned(),
                format!("a{invocation}-end"),
                "b".to_owned()
            ]
        );
    }
    panel.unbind_all(); // Also removes the widget's internal lifecycle binding.
}

fn verify_any_id_binding(panel: Panel) {
    let calls = Rc::new(Cell::new(0));
    let recorded = calls.clone();
    // This is the issue's public API reproducer, without a user-supplied guard.
    panel.on_menu_selected(move |_| {
        recorded.set(recorded.get() + 1);
        panel.process_menu_command(CMD_ID);
    });
    panel.process_menu_command(CMD_ID);
    panel.process_menu_command(CMD_ID);
    assert_eq!(calls.get(), 2);
    panel.unbind_all();
}

struct DropProbe(Rc<Cell<usize>>);
impl Drop for DropProbe {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

fn verify_unbind_during_nested_dispatch(panel: Panel) {
    let drops = Rc::new(Cell::new(0));
    let probe = DropProbe(drops.clone());
    let a_token = Rc::new(Cell::new(EventToken::INVALID_TOKEN));
    let token_recorded = a_token.clone();
    let calls = Rc::new(Cell::new(0));
    let recorded = calls.clone();
    let alive_after_nested = Rc::new(Cell::new(false));
    let alive_recorded = alive_after_nested.clone();
    let drops_recorded = drops.clone();
    let token = panel.bind_with_id_internal(EventType::MENU, CMD_ID, move |_| {
        let _keep_capture = &probe;
        recorded.set(recorded.get() + 1);
        panel.process_menu_command(CMD_ID);
        alive_recorded.set(drops_recorded.get() == 0);
    });
    a_token.set(token);
    // The second handler unbinds A while A's outer invocation is on the stack.
    panel.bind_with_id_internal(EventType::MENU, CMD_ID, move |_| {
        panel.unbind(token_recorded.get());
    });
    panel.process_menu_command(CMD_ID);
    assert_eq!(calls.get(), 1);
    assert!(alive_after_nested.get());
    assert_eq!(drops.get(), 1);
    panel.process_menu_command(CMD_ID);
    assert_eq!(calls.get(), 1);
    assert_eq!(drops.get(), 1);
    panel.unbind_all();
}

fn verify_self_unbind_after_reentry(panel: Panel) {
    let drops = Rc::new(Cell::new(0));
    let probe = DropProbe(drops.clone());
    let token = Rc::new(Cell::new(EventToken::INVALID_TOKEN));
    let token_recorded = token.clone();
    let alive_after_unbind = Rc::new(Cell::new(false));
    let alive_recorded = alive_after_unbind.clone();
    let drops_recorded = drops.clone();
    let bound = panel.bind_with_id_internal(EventType::MENU, CMD_ID, move |_| {
        let _keep_capture = &probe;
        panel.process_menu_command(CMD_ID);
        let removed = panel.unbind(token_recorded.get());
        alive_recorded.set(removed && drops_recorded.get() == 0);
    });
    token.set(bound);
    panel.process_menu_command(CMD_ID);
    assert!(alive_after_unbind.get());
    assert_eq!(drops.get(), 1);
    panel.process_menu_command(CMD_ID);
    assert_eq!(drops.get(), 1);
    panel.unbind_all();
}

fn verify_outer_consumption_does_not_consume_nested_event(panel: Panel) {
    let other_calls = Rc::new(Cell::new(0));
    let recorded = other_calls.clone();
    panel.bind_with_id_internal(EventType::MENU, CMD_ID, move |event| {
        panel.process_menu_command(CMD_ID);
        event.skip(false);
    });
    panel.bind_with_id_internal(EventType::MENU, CMD_ID, move |_| {
        recorded.set(recorded.get() + 1);
    });
    panel.process_menu_command(CMD_ID);
    // B runs in the nested dispatch, but not in the consumed outer dispatch.
    assert_eq!(other_calls.get(), 1);
    panel.unbind_all();
}

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let finished = Rc::new(Cell::new(false));
    let finished_in_loop = finished.clone();
    let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
    let timer_in_loop = timer_store.clone();
    wxdragon::main(move |app| {
        let frame = Frame::builder().with_title("Event re-entry regressions").build();
        let panel = Panel::builder(&frame).build();
        // Install the exit timer first: caught assertion panics must not hang the test.
        let timer = Timer::new(&frame);
        let timer_cleanup = timer_in_loop.clone();
        timer.on_tick(move |_| {
            timer_cleanup.borrow_mut().take();
            app.exit_main_loop();
        });
        timer.start(100, true);
        timer_in_loop.borrow_mut().replace(timer);

        verify_id_binding(panel);
        verify_any_id_binding(panel);
        verify_unbind_during_nested_dispatch(panel);
        verify_self_unbind_after_reentry(panel);
        verify_outer_consumption_does_not_consume_nested_event(panel);
        finished_in_loop.set(true);
    })
    .expect("could not initialize wxWidgets for event re-entry regressions");
    assert!(finished.get(), "event re-entry assertions did not finish");
}
