use wxdragon::prelude::*;

mod dataview;

fn main() {
    wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("wxWidgets 3.3.4 accessibility")
            .with_size(Size::new(560, 900))
            .build();
        wxdragon::app::set_top_window(&frame);
        let panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        let flags = SizerFlag::Expand | SizerFlag::All;

        let instructions = StaticText::builder(&panel)
            .with_label("Use Tab / Shift+Tab and a screen reader to explore these controls.")
            .build();
        sizer.add(&instructions, 0, flags, 8);

        // Create each label immediately before its control, under the same parent.
        // wxWidgets associates these native controls with their label on macOS.
        let choice_label = StaticText::builder(&panel).with_label("Language").build();
        let choice = Choice::builder(&panel).build();
        choice.append("English");
        choice.append("Français");
        choice.set_selection(0);
        sizer.add(&choice_label, 0, flags, 8);
        sizer.add(&choice, 0, flags, 8);

        let combo_label = StaticText::builder(&panel).with_label("City").build();
        let combo = ComboBox::builder(&panel).build();
        combo.append_items(&["Paris", "Tokyo", "Montréal"]);
        combo.set_selection(0);
        sizer.add(&combo_label, 0, flags, 8);
        sizer.add(&combo, 0, flags, 8);

        let list_label = StaticText::builder(&panel).with_label("Destinations").build();
        let list = ListBox::builder(&panel).build();
        list.append_items(&["Paris", "Tokyo", "Montréal"]);
        sizer.add(&list_label, 0, flags, 8);
        sizer.add(&list, 1, flags, 8);

        let readonly_label = StaticText::builder(&panel).with_label("Reservation number").build();
        let readonly = TextCtrl::builder(&panel).with_style(TextCtrlStyle::ReadOnly).build();
        readonly.set_value("ABC-123");
        readonly.enable_focus_from_keyboard();
        sizer.add(&readonly_label, 0, flags, 8);
        sizer.add(&readonly, 0, flags, 8);

        let toggle_focus = Button::builder(&panel)
            .with_label("Toggle read-only field Tab access")
            .build();
        toggle_focus.on_click(move |_| {
            if readonly.accepts_focus_from_keyboard() {
                readonly.disable_focus_from_keyboard();
            } else {
                readonly.enable_focus_from_keyboard();
            }
        });
        sizer.add(&toggle_focus, 0, flags, 8);

        let named_button = Button::builder(&panel).with_label("Continue").build();
        named_button.set_accessibility_label("Continue to payment — 支払い");
        named_button.set_accessibility_description("Proceed with the selected reservation.");
        sizer.add(&named_button, 0, flags, 8);
        let clear_name = Button::builder(&panel)
            .with_label("Restore Continue button's default name")
            .build();
        clear_name.on_click(move |_| named_button.set_accessibility_label(""));
        sizer.add(&clear_name, 0, flags, 8);

        dataview::add_tree(&panel, &sizer);

        panel.set_sizer(sizer, true);
        let frame_sizer = BoxSizer::builder(Orientation::Vertical).build();
        frame_sizer.add(&panel, 1, SizerFlag::Expand, 0);
        frame.set_sizer(frame_sizer, true);
        frame.centre();
        frame.show(true);
        frame.raise();
        // Activate after startup, once the main event loop is running.
        #[cfg(target_os = "macos")]
        wxdragon::app::call_after(Box::new(wxdragon::app::activate_app));
    })
    .expect("could not start accessibility demo");
}
