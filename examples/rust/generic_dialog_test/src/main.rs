use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Generic Dialog Test")
            .with_size(Size::new(400, 300))
            .build();

        // Create a button to show the generic dialog
        let button = Button::builder(&frame)
            .with_label("Show Generic Dialog")
            .build();

        // Clone frame for the event handler
        let frame_clone = frame.clone();
        button.on_click(move |_| {
            // Create a generic dialog using the new builder
            let dialog = Dialog::builder(&frame_clone, "My Generic Dialog")
                .with_style(DialogStyle::DefaultDialogStyle | DialogStyle::ResizeBorder)
                .with_size(300, 200)
                .build();

            // Add some content to the dialog
            let panel = Panel::builder(&dialog).build();

            let text = StaticText::builder(&panel)
                .with_label("This is a generic dialog created with wxDragon!")
                .build();

            let ok_button = Button::builder(&panel).with_label("OK").build();

            // Clone dialog for the OK button event handler
            let dialog_clone = dialog.clone();
            ok_button.on_click(move |_| {
                dialog_clone.end_modal(ID_OK);
            });

            // Layout the panel content
            let panel_sizer = BoxSizer::builder(Orientation::Vertical).build();
            panel_sizer.add(&text, 1, SizerFlag::Expand | SizerFlag::All, 10);
            panel_sizer.add(&ok_button, 0, SizerFlag::AlignCentre | SizerFlag::All, 10);
            panel.set_sizer(panel_sizer, true);

            // Layout the dialog
            let dialog_sizer = BoxSizer::builder(Orientation::Vertical).build();
            dialog_sizer.add(&panel, 1, SizerFlag::Expand, 0);
            dialog.set_sizer(dialog_sizer, true);

            // Show the dialog modally
            let result = dialog.show_modal();
            println!("Dialog returned: {}", result);

            // Dialog is automatically cleaned up when it goes out of scope
        });

        // Layout the main frame
        let frame_sizer = BoxSizer::builder(Orientation::Vertical).build();
        frame_sizer.add(&button, 0, SizerFlag::AlignCentre | SizerFlag::All, 20);
        frame.set_sizer(frame_sizer, true);

        frame.show(true);
        frame.centre();
    });
}
