use wxdragon::prelude::*;

struct TabOrderDemo {
    frame: Frame,
    main_panel: Panel,
    button1: Button,
    button2: Button,
    button3: Button,
    text_ctrl1: TextCtrl,
    text_ctrl2: TextCtrl,
    checkbox: CheckBox,
    radio1: RadioButton,
    radio2: RadioButton,
    combo_box: ComboBox,
    status_text: StaticText,

    // Control buttons for testing
    reset_order_btn: Button,
    reverse_order_btn: Button,
    custom_order_btn: Button,
    test_navigate_btn: Button,
}

impl TabOrderDemo {
    fn new() -> Self {
        let frame = Frame::builder()
            .with_title("Tab Order Demo - wxdragon")
            .with_size(Size::new(600, 500))
            .build();

        let main_panel = Panel::builder(&frame)
            .with_style(PanelStyle::TabTraversal)
            .build();

        // Create main controls to demonstrate tab order
        let button1 = Button::builder(&main_panel).with_label("Button 1").build();

        let button2 = Button::builder(&main_panel).with_label("Button 2").build();

        let button3 = Button::builder(&main_panel).with_label("Button 3").build();

        let text_ctrl1 = TextCtrl::builder(&main_panel)
            .with_value("Text Control 1")
            .build();

        let text_ctrl2 = TextCtrl::builder(&main_panel)
            .with_value("Text Control 2")
            .build();

        let checkbox = CheckBox::builder(&main_panel)
            .with_label("Checkbox Control")
            .build();

        let radio1 = RadioButton::builder(&main_panel)
            .with_label("Radio Option 1")
            .first_in_group()
            .build();

        let radio2 = RadioButton::builder(&main_panel)
            .with_label("Radio Option 2")
            .build();

        let combo_box = ComboBox::builder(&main_panel)
            .with_value("Combo Box")
            .build();
        combo_box.append("Option 1");
        combo_box.append("Option 2");
        combo_box.append("Option 3");

        let status_text = StaticText::builder(&main_panel)
            .with_label("Status: Use Tab/Shift+Tab to navigate between controls")
            .build();

        // Control buttons for testing tab order functionality
        let reset_order_btn = Button::builder(&main_panel)
            .with_label("Reset to Default Order")
            .build();

        let reverse_order_btn = Button::builder(&main_panel)
            .with_label("Reverse Tab Order")
            .build();

        let custom_order_btn = Button::builder(&main_panel)
            .with_label("Custom Tab Order")
            .build();

        let test_navigate_btn = Button::builder(&main_panel)
            .with_label("Test Navigate() Function")
            .build();

        Self {
            frame,
            main_panel,
            button1,
            button2,
            button3,
            text_ctrl1,
            text_ctrl2,
            checkbox,
            radio1,
            radio2,
            combo_box,
            status_text,
            reset_order_btn,
            reverse_order_btn,
            custom_order_btn,
            test_navigate_btn,
        }
    }

    fn setup_layout(&self) {
        // Main vertical sizer
        let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Title
        let title = StaticText::builder(&self.main_panel)
            .with_label("Tab Order Demonstration")
            .build();

        // Make title bold and larger
        if let Some(mut font) = title.get_font() {
            font.make_bold();
            font.set_point_size(font.get_point_size() + 2);
            title.set_font(&font);
        }

        main_sizer.add(&title, 0, SizerFlag::All | SizerFlag::AlignCentre, 10);

        // Instructions
        let instructions = StaticText::builder(&self.main_panel)
            .with_label("Instructions:\n• Use Tab/Shift+Tab to navigate between controls\n• Try the buttons below to change tab order\n• Watch the status bar for feedback")
            .build();
        main_sizer.add(&instructions, 0, SizerFlag::All | SizerFlag::Expand, 10);

        // Separator
        let separator = StaticLine::builder(&self.main_panel)
            .with_style(StaticLineStyle::Default)
            .build();
        main_sizer.add(&separator, 0, SizerFlag::Expand | SizerFlag::All, 5);

        // Controls grid
        let controls_sizer = FlexGridSizer::builder(0, 3)
            .with_vgap(10)
            .with_hgap(10)
            .build();

        // Add all the main controls to the grid
        controls_sizer.add(&self.button1, 0, SizerFlag::Expand, 0);
        controls_sizer.add(&self.button2, 0, SizerFlag::Expand, 0);
        controls_sizer.add(&self.button3, 0, SizerFlag::Expand, 0);
        controls_sizer.add(&self.text_ctrl1, 0, SizerFlag::Expand, 0);
        controls_sizer.add(&self.text_ctrl2, 0, SizerFlag::Expand, 0);
        controls_sizer.add(&self.checkbox, 0, SizerFlag::Expand, 0);
        controls_sizer.add(&self.radio1, 0, SizerFlag::Expand, 0);
        controls_sizer.add(&self.radio2, 0, SizerFlag::Expand, 0);
        controls_sizer.add(&self.combo_box, 0, SizerFlag::Expand, 0);

        controls_sizer.add_growable_col(0, 1);
        controls_sizer.add_growable_col(1, 1);
        controls_sizer.add_growable_col(2, 1);

        main_sizer.add_sizer(&controls_sizer, 1, SizerFlag::Expand | SizerFlag::All, 10);

        // Another separator
        let separator2 = StaticLine::builder(&self.main_panel)
            .with_style(StaticLineStyle::Default)
            .build();
        main_sizer.add(&separator2, 0, SizerFlag::Expand | SizerFlag::All, 5);

        // Control buttons
        let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        button_sizer.add(&self.reset_order_btn, 0, SizerFlag::All, 5);
        button_sizer.add(&self.reverse_order_btn, 0, SizerFlag::All, 5);
        button_sizer.add(&self.custom_order_btn, 0, SizerFlag::All, 5);
        button_sizer.add(&self.test_navigate_btn, 0, SizerFlag::All, 5);

        main_sizer.add_sizer(
            &button_sizer,
            0,
            SizerFlag::AlignCentre | SizerFlag::All,
            10,
        );

        // Status text at bottom
        main_sizer.add(&self.status_text, 0, SizerFlag::Expand | SizerFlag::All, 10);

        self.main_panel.set_sizer_and_fit(main_sizer, true);
    }

    fn bind_events(&self) {
        // Focus events for all main controls to show tab order
        let status_clone = self.status_text.clone();
        self.button1.on_set_focus(move |_| {
            status_clone.set_label("Focus: Button 1");
        });

        let status_clone = self.status_text.clone();
        self.button2.on_set_focus(move |_| {
            status_clone.set_label("Focus: Button 2");
        });

        let status_clone = self.status_text.clone();
        self.button3.on_set_focus(move |_| {
            status_clone.set_label("Focus: Button 3");
        });

        let status_clone = self.status_text.clone();
        self.text_ctrl1.on_set_focus(move |_| {
            status_clone.set_label("Focus: Text Control 1");
        });

        let status_clone = self.status_text.clone();
        self.text_ctrl2.on_set_focus(move |_| {
            status_clone.set_label("Focus: Text Control 2");
        });

        let status_clone = self.status_text.clone();
        self.checkbox.on_set_focus(move |_| {
            status_clone.set_label("Focus: Checkbox");
        });

        let status_clone = self.status_text.clone();
        self.radio1.on_set_focus(move |_| {
            status_clone.set_label("Focus: Radio Option 1");
        });

        let status_clone = self.status_text.clone();
        self.radio2.on_set_focus(move |_| {
            status_clone.set_label("Focus: Radio Option 2");
        });

        let status_clone = self.status_text.clone();
        self.combo_box.on_set_focus(move |_| {
            status_clone.set_label("Focus: Combo Box");
        });

        // Tab order control buttons
        self.setup_tab_order_controls();
    }

    fn setup_tab_order_controls(&self) {
        // Reset to default order button
        let status = self.status_text.clone();

        self.reset_order_btn.on_click(move |_| {
            // Reset to creation order (which should be the default tab order)
            // This demonstrates that the order can be restored
            status.set_label("Status: Reset to default tab order (restart app to see effect)");
            println!(
                "Resetting to default tab order - restart application to see the default order"
            );
        });

        // Reverse order button
        let button1 = self.button1.clone();
        let button2 = self.button2.clone();
        let button3 = self.button3.clone();
        let text1 = self.text_ctrl1.clone();
        let text2 = self.text_ctrl2.clone();
        let checkbox = self.checkbox.clone();
        let radio1 = self.radio1.clone();
        let radio2 = self.radio2.clone();
        let combo = self.combo_box.clone();
        let status = self.status_text.clone();

        self.reverse_order_btn.on_click(move |_| {
            // Reverse the tab order: combo -> radio2 -> radio1 -> checkbox -> text2 -> text1 -> button3 -> button2 -> button1
            combo.move_before_in_tab_order(&radio2);
            radio2.move_before_in_tab_order(&radio1);
            radio1.move_before_in_tab_order(&checkbox);
            checkbox.move_before_in_tab_order(&text2);
            text2.move_before_in_tab_order(&text1);
            text1.move_before_in_tab_order(&button3);
            button3.move_before_in_tab_order(&button2);
            button2.move_before_in_tab_order(&button1);

            status.set_label("Status: Reversed tab order - try tabbing now!");
            println!("Reversed tab order");
        });

        // Custom order button
        let button1 = self.button1.clone();
        let button2 = self.button2.clone();
        let button3 = self.button3.clone();
        let text1 = self.text_ctrl1.clone();
        let text2 = self.text_ctrl2.clone();
        let checkbox = self.checkbox.clone();
        let radio1 = self.radio1.clone();
        let radio2 = self.radio2.clone();
        let combo = self.combo_box.clone();
        let status = self.status_text.clone();

        self.custom_order_btn.on_click(move |_| {
            // Custom order: text controls first, then buttons, then other controls
            // Order: text1 -> text2 -> button1 -> button2 -> button3 -> checkbox -> radio1 -> radio2 -> combo
            text2.move_after_in_tab_order(&text1);
            button1.move_after_in_tab_order(&text2);
            button2.move_after_in_tab_order(&button1);
            button3.move_after_in_tab_order(&button2);
            checkbox.move_after_in_tab_order(&button3);
            radio1.move_after_in_tab_order(&checkbox);
            radio2.move_after_in_tab_order(&radio1);
            combo.move_after_in_tab_order(&radio2);

            status.set_label("Status: Custom tab order - Text controls first, then buttons!");
            println!("Applied custom tab order");
        });

        // Test navigate function
        let button1 = self.button1.clone();
        let status = self.status_text.clone();

        self.test_navigate_btn.on_click(move |_| {
            // Set focus to button1 and then programmatically navigate
            button1.set_focus();
            if button1.navigate(true) {
                status.set_label("Status: Programmatically navigated to next control");
                println!("Successfully navigated to next control");
            } else {
                status.set_label("Status: Navigation failed");
                println!("Navigation failed");
            }
        });

        // Test sibling navigation
        let button1 = self.button1.clone();
        let status = self.status_text.clone();

        self.button1.on_click(move |_| {
            let mut info = String::from("Button 1 clicked! ");

            if let Some(_next) = button1.get_next_sibling() {
                info.push_str("Has next sibling. ");
            } else {
                info.push_str("No next sibling. ");
            }

            if let Some(_prev) = button1.get_prev_sibling() {
                info.push_str("Has previous sibling.");
            } else {
                info.push_str("No previous sibling.");
            }

            status.set_label(&format!("Status: {}", info));
            println!("{}", info);
        });
    }

    fn show(&self) {
        self.frame.show(true);
        self.frame.centre();

        // Set initial focus
        self.button1.set_focus();
    }
}

fn main() {
    let _ = wxdragon::main(|_| {
        let demo = TabOrderDemo::new();
        demo.setup_layout();
        demo.bind_events();
        demo.show();

        println!("Tab Order Demo started!");
        println!("Features demonstrated:");
        println!("- move_after_in_tab_order()");
        println!("- move_before_in_tab_order()");
        println!("- get_next_sibling() / get_prev_sibling()");
        println!("- navigate() function");
        println!("- TabStop window style");
    });
}
