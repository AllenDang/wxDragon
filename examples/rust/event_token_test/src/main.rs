use std::cell::RefCell;
use std::rc::Rc;
use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Event Token Unbind Demo")
            .with_size(Size::new(400, 200))
            .build();

        let panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Button 1: Initially has no event bindings
        let button1 = Button::builder(&panel)
            .with_label("Button 1 (Popup Messagebox)")
            .build();

        // Button 2: Remove event binding
        let button2 = Button::builder(&panel)
            .with_label("Button 2 (Remove Event Binding)")
            .build();

        // Button 3: Add event binding
        let button3 = Button::builder(&panel)
            .with_label("Button 3 (Add Event Binding)")
            .build();

        sizer.add(&button1, 0, SizerFlag::Expand, 10);
        sizer.add(&button2, 0, SizerFlag::Expand, 10);
        sizer.add(&button3, 0, SizerFlag::Expand, 10);

        panel.set_sizer(sizer, true);

        // Shared state: Store tokens and next binding index
        let tokens = Rc::new(RefCell::new(Vec::new()));
        let next_index = Rc::new(RefCell::new(1));

        // Button 2: Remove the first event binding from Button 1
        {
            let tokens_clone = tokens.clone();
            let button1_clone = button1.clone();
            button2.on_click(move |_| {
                let mut tokens_vec = tokens_clone.borrow_mut();
                if tokens_vec.is_empty() {
                    MessageDialog::builder(&button1_clone, "No event bindings to remove!", "Info")
                        .build()
                        .show_modal();
                } else {
                    // Remove the first token
                    let token = tokens_vec.remove(0);
                    if button1_clone.unbind(token) {
                        MessageDialog::builder(
                            &button1_clone,
                            &format!(
                                "Removed event binding! {} binding(s) remaining.",
                                tokens_vec.len()
                            ),
                            "Success",
                        )
                        .build()
                        .show_modal();
                    } else {
                        MessageDialog::builder(&button1_clone, "Failed to unbind event!", "Error")
                            .build()
                            .show_modal();
                    }
                }
            });
        }

        // Button 3: Add a new event binding to Button 1
        {
            let tokens_clone = tokens.clone();
            let next_index_clone = next_index.clone();
            let button1_clone = button1.clone();
            let frame_clone = frame.clone();

            button3.on_click(move |_| {
                let mut index_val = next_index_clone.borrow_mut();
                let current_index = *index_val;
                *index_val += 1;

                // Add a new event binding to button1
                let frame_ref = frame_clone.clone();
                let token = button1_clone.on_click(move |_| {
                    MessageDialog::builder(
                        &frame_ref,
                        &format!("On click callback #{}", current_index),
                        "Button 1 Clicked",
                    )
                    .build()
                    .show_modal();
                });

                // Store the token
                tokens_clone.borrow_mut().push(token);

                MessageDialog::builder(
                    &button1_clone,
                    &format!(
                        "Added event binding #{}! Total bindings: {}",
                        current_index,
                        tokens_clone.borrow().len()
                    ),
                    "Binding Added",
                )
                .build()
                .show_modal();
            });
        }

        frame.show(true);
        frame.centre();
    });
}
