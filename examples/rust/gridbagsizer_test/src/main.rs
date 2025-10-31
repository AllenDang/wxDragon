use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("GridBagSizer Test")
            .with_size(Size::new(600, 500))
            .build();

        let panel = Panel::builder(&frame).build();

        // Create a main vertical sizer to hold everything
        let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Add a title
        let title = StaticText::builder(&panel)
            .with_label("GridBagSizer Demo - Layout with Positions and Spans")
            .build();
        main_sizer.add(&title, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);

        // Create the GridBagSizer with gaps
        let gb_sizer = GridBagSizer::builder().with_vgap(8).with_hgap(8).build();

        // Row 0: Form-like layout with labels and inputs
        let name_label = StaticText::builder(&panel).with_label("Name:").build();
        gb_sizer.add_at(
            &name_label,
            GBPosition::new(0, 0),
            DEFAULT_GB_SPAN,
            SizerFlag::AlignCenterVertical | SizerFlag::AlignRight,
            5,
        );

        let name_input = TextCtrl::builder(&panel).with_value("Enter your name").build();
        gb_sizer.add_at(
            &name_input,
            GBPosition::new(0, 1),
            GBSpan::new(1, 2), // Span 2 columns
            SizerFlag::Expand,
            5,
        );

        // Row 1: Email input spanning 3 columns
        let email_label = StaticText::builder(&panel).with_label("Email:").build();
        gb_sizer.add_at(
            &email_label,
            GBPosition::new(1, 0),
            DEFAULT_GB_SPAN,
            SizerFlag::AlignCenterVertical | SizerFlag::AlignRight,
            5,
        );

        let email_input = TextCtrl::builder(&panel).with_value("your.email@example.com").build();
        gb_sizer.add_at(
            &email_input,
            GBPosition::new(1, 1),
            GBSpan::new(1, 3), // Span 3 columns
            SizerFlag::Expand,
            5,
        );

        // Row 2: Address spanning multiple columns
        let address_label = StaticText::builder(&panel).with_label("Address:").build();
        gb_sizer.add_at(
            &address_label,
            GBPosition::new(2, 0),
            DEFAULT_GB_SPAN,
            SizerFlag::Top | SizerFlag::AlignRight,
            5,
        );

        let address_input = TextCtrl::builder(&panel)
            .with_value("123 Main Street\nAnytown, State 12345")
            .with_style(TextCtrlStyle::MultiLine)
            .build();
        gb_sizer.add_at(
            &address_input,
            GBPosition::new(2, 1),
            GBSpan::new(2, 3), // Span 2 rows and 3 columns
            SizerFlag::Expand,
            5,
        );

        // Row 3: Skip this row for address continuation

        // Row 4: Preferences section
        let preferences_label = StaticText::builder(&panel).with_label("Preferences:").build();
        gb_sizer.add_at(
            &preferences_label,
            GBPosition::new(4, 0),
            DEFAULT_GB_SPAN,
            SizerFlag::Top | SizerFlag::AlignRight,
            5,
        );

        let checkbox1 = CheckBox::builder(&panel).with_label("Subscribe to newsletter").build();
        gb_sizer.add_at(
            &checkbox1,
            GBPosition::new(4, 1),
            DEFAULT_GB_SPAN,
            SizerFlag::AlignCenterVertical,
            5,
        );

        let checkbox2 = CheckBox::builder(&panel).with_label("Enable notifications").build();
        gb_sizer.add_at(
            &checkbox2,
            GBPosition::new(4, 2),
            DEFAULT_GB_SPAN,
            SizerFlag::AlignCenterVertical,
            5,
        );

        let checkbox3 = CheckBox::builder(&panel).with_label("Auto-save").build();
        gb_sizer.add_at(
            &checkbox3,
            GBPosition::new(4, 3),
            DEFAULT_GB_SPAN,
            SizerFlag::AlignCenterVertical,
            5,
        );

        // Row 5: Buttons spanning across columns
        let button_cancel = Button::builder(&panel).with_label("Cancel").build();
        gb_sizer.add_at(&button_cancel, GBPosition::new(5, 1), DEFAULT_GB_SPAN, SizerFlag::Expand, 5);

        let button_save = Button::builder(&panel).with_label("Save").build();
        gb_sizer.add_at(&button_save, GBPosition::new(5, 2), DEFAULT_GB_SPAN, SizerFlag::Expand, 5);

        let button_submit = Button::builder(&panel).with_label("Submit").build();
        gb_sizer.add_at(&button_submit, GBPosition::new(5, 3), DEFAULT_GB_SPAN, SizerFlag::Expand, 5);

        // Add the GridBagSizer to the main sizer
        main_sizer.add_sizer(&gb_sizer, 1, SizerFlag::Expand | SizerFlag::All, 20);

        // Add some information about the grid
        let info_sizer = BoxSizer::builder(Orientation::Horizontal).build();

        let info_text = StaticText::builder(&panel)
            .with_label("GridBagSizer allows precise positioning and spanning of widgets in a grid.")
            .build();
        info_sizer.add(&info_text, 1, SizerFlag::AlignCenterVertical, 0);

        let demo_button = Button::builder(&panel).with_label("Move Items").build();
        info_sizer.add(&demo_button, 0, SizerFlag::AlignCenterVertical | SizerFlag::Left, 10);

        main_sizer.add_sizer(&info_sizer, 0, SizerFlag::Expand | SizerFlag::All, 10);

        // Set up event handling for the demo button
        demo_button.on_click(move |_| {
            // For now, just print a message
            // Dynamic layout changes would require more complex state management
            println!("GridBagSizer demo button clicked!");
        });

        panel.set_sizer(main_sizer, true);
        frame.show(true);
        frame.centre();
    });
}
