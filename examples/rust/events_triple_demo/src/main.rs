use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wxdragon::event::{IdleEvent, IdleMode, WindowEvents};
use wxdragon::prelude::*;

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    let _ = wxdragon::main(|_| {
        // Configure idle: only windows that request will receive idle events
        IdleEvent::set_mode(IdleMode::ProcessSpecified);

        log::info!("Starting Events Triple Demo");

        let frame = Frame::builder()
            .with_title("Events Triple Demo - Veto | Background | Idle")
            .with_size(Size::new(520, 320))
            .build();

        // Request idle for this frame
        frame.set_extra_style(ExtraWindowStyle::ProcessIdle);

        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        let status = StaticText::builder(&frame).with_label("Status: Ready").build();

        let toggle_unsaved_btn = Button::builder(&frame).with_label("Toggle Unsaved").build();

        let bg_task_btn = Button::builder(&frame).with_label("Start Background Task").build();

        let idle_btn = Button::builder(&frame).with_label("Start Idle Work").build();

        // State
        let unsaved = Rc::new(RefCell::new(false));
        let idle_remaining = Rc::new(RefCell::new(0_i32));
        let bg_msg: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

        // Toggle unsaved
        // Widgets are Copy, Rc/Arc need .clone()
        let unsaved_toggle = unsaved.clone();
        toggle_unsaved_btn.on_click(move |_| {
            let mut u = unsaved_toggle.borrow_mut();
            *u = !*u;
            status.set_label(&format!("Status: unsaved = {}", *u));
        });

        // Background task: simulate work, notify start and completion via status updates
        let bg_msg_btn = bg_msg.clone();
        bg_task_btn.on_click(move |_| {
            {
                if let Ok(mut m) = bg_msg_btn.lock() {
                    *m = Some("Status: background task started...".to_string());
                }
            }
            wxdragon::call_after(Box::new(|| {
                // Wake the GUI event loop so the Idle handler runs promptly
                // and applies the pending "started" status update.
            }));
            let bg_msg = bg_msg_btn.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(1000));
                // Mark completion; Idle handler will pick this up and update the label
                if let Ok(mut m) = bg_msg.lock() {
                    *m = Some("Status: background task finished.".to_string());
                }
                wxdragon::call_after(Box::new(move || {
                    // Second tick to ensure GUI wakes up; idle will apply message if any.
                }));
            });
        });

        // Idle work: start a chunked job processed in idle handler
        let idle_remaining_btn = idle_remaining.clone();
        idle_btn.on_click(move |_| {
            *idle_remaining_btn.borrow_mut() = 100000; // units of work
            status.set_label("Status: idle work scheduled (100000 units)");
        });

        // Idle handler: process a few units each idle; also apply background messages
        let idle_remaining_handler = idle_remaining.clone();
        let bg_msg_handler = bg_msg.clone();
        frame.on_idle(move |e| {
            // Apply background message if any
            if let Ok(mut m) = bg_msg_handler.lock()
                && let Some(text) = m.take()
            {
                status.set_label(&text);
            }
            let mut left = idle_remaining_handler.borrow_mut();
            if *left > 0 {
                let step = 7;
                *left = (*left - step).max(0);
                status.set_label(&format!("Status: idle processing... left={}", *left));
                if let wxdragon::event::WindowEventData::Idle(idle) = e {
                    idle.event.request_more(true);
                }
            }
        });

        // Veto close if unsaved
        let unsaved_close = unsaved.clone();
        frame.on_close(move |e| {
            if let WindowEventData::General(ev) = e
                && ev.can_veto()
                && *unsaved_close.borrow()
            {
                ev.veto();
                status.set_label("Status: close vetoed (unsaved)");
            }
        });

        // Layout
        sizer.add(&status, 0, SizerFlag::All | SizerFlag::Expand, 8);

        let row = BoxSizer::builder(Orientation::Horizontal).build();
        row.add(&toggle_unsaved_btn, 0, SizerFlag::Right | SizerFlag::All, 4);
        row.add(&bg_task_btn, 0, SizerFlag::Right | SizerFlag::All, 4);
        row.add(&idle_btn, 0, SizerFlag::Right | SizerFlag::All, 4);

        sizer.add_sizer(&row, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 4);
        sizer.add_stretch_spacer(1);

        frame.set_sizer(sizer, true);
        frame.show(true);
        frame.centre();
    });
}
