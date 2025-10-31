use std::cell::RefCell;
use std::rc::Rc;
use wxdragon::prelude::*;

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Event Token Unbind Demo")
            .with_size(Size::new(520, 360))
            .build();

        let panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Button 1: Initially has no event bindings
        let button1 = Button::builder(&panel).with_label("Button 1 (Append Log Entry)").build();

        // Button 2: Remove event binding
        let button2 = Button::builder(&panel).with_label("Button 2 (Remove Event Binding)").build();

        // Button 3: Add event binding
        let button3 = Button::builder(&panel).with_label("Button 3 (Add Event Binding)").build();

        sizer.add(&button1, 0, SizerFlag::Expand, 8);
        sizer.add(&button2, 0, SizerFlag::Expand, 8);
        sizer.add(&button3, 0, SizerFlag::Expand, 8);

        // Simple multiline log area using a StaticText (supports '\n').
        // If you need scrolling, consider adding a TextCtrl wrapper later.
        let log_buffer = Rc::new(RefCell::new(String::from("Log:\n")));
        let log_view = StaticText::builder(&panel).with_label(&log_buffer.borrow()).build();
        sizer.add(&log_view, 1, SizerFlag::Expand, 8);

        panel.set_sizer(sizer, true);

        // Shared state: Store tokens and next binding index
        let tokens = Rc::new(RefCell::new(Vec::new()));
        let next_index = Rc::new(RefCell::new(1));

        // Helper to append a line to the log
        fn append_line(log: &Rc<RefCell<String>>, view: &StaticText, line: &str) {
            let mut buf = log.borrow_mut();
            buf.push_str(line);
            buf.push('\n');
            view.set_label(&buf);
            log::info!("{line}");
        }

        // Button 2: Remove the first event binding from Button 1
        {
            let tokens_clone = tokens.clone();
            let button1_clone = button1.clone();
            let log_buffer_clone = log_buffer.clone();
            let log_view_clone = log_view.clone();
            button2.on_click(move |_| {
                let mut tokens_vec = tokens_clone.borrow_mut();
                if tokens_vec.is_empty() {
                    append_line(&log_buffer_clone, &log_view_clone, "No event bindings to remove!");
                } else {
                    // Remove the first token
                    let token = tokens_vec.remove(0);
                    if button1_clone.unbind(token) {
                        let line = format!("Removed event binding! {} binding(s) remaining.", tokens_vec.len());
                        append_line(&log_buffer_clone, &log_view_clone, &line);
                    } else {
                        append_line(&log_buffer_clone, &log_view_clone, "Failed to unbind event!");
                    }
                }
            });
        }

        // Button 3: Add a new event binding to Button 1
        {
            let tokens_clone = tokens.clone();
            let next_index_clone = next_index.clone();
            let button1_clone = button1.clone();
            let log_buffer_clone = log_buffer.clone();
            let log_view_clone = log_view.clone();

            button3.on_click(move |_| {
                let mut index_val = next_index_clone.borrow_mut();
                let current_index = *index_val;
                *index_val += 1;

                // Add a new event binding to button1
                let log_buffer_clicked = log_buffer_clone.clone();
                let log_view_clicked = log_view_clone.clone();
                let token = button1_clone.on_click(move |_| {
                    let line = format!("On click callback #{current_index}");
                    append_line(&log_buffer_clicked, &log_view_clicked, &line);
                });

                // Store the token
                tokens_clone.borrow_mut().push(token);

                let line = format!(
                    "Added event binding #{current_index}! Total bindings: {}",
                    tokens_clone.borrow().len()
                );
                append_line(&log_buffer_clone, &log_view_clone, &line);
            });
        }

        frame.show(true);
        frame.centre();
    });
}
