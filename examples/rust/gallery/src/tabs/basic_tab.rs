use wxdragon::prelude::*;

#[allow(dead_code)]
pub struct BasicTabControls {
    pub panel: Panel,
    pub text_ctrl: TextCtrl,
    pub spin_button: SpinButton,
    pub checkbox: CheckBox,
    pub radio1: RadioButton,
    pub radio2: RadioButton,
    pub radio_status_label: StaticText,
    pub toggle_button: ToggleButton,
    pub toggle_status_label: StaticText,
    pub bitmap_button: BitmapButton,
    pub art_button: BitmapButton,
    pub radio_box: RadioBox,
    pub bitmap_combo_box: BitmapComboBox,
    pub colour_picker: ColourPickerCtrl,
    pub colour_label: StaticText,
    pub date_picker: DatePickerCtrl,
    pub date_picker_label: StaticText,
    pub time_picker: TimePickerCtrl,
    pub time_picker_label: StaticText,
    pub search_ctrl: SearchCtrl,
    pub hyperlink_ctrl: HyperlinkCtrl,
    pub activity_indicator: ActivityIndicator,
    pub activity_start_btn: Button,
    pub activity_stop_btn: Button,
    pub spinctrl_double: SpinCtrlDouble,
    pub spinctrl_double_label: StaticText,
    pub calendar_ctrl: CalendarCtrl,
    pub calendar_label: StaticText,
    pub scroll_bar: ScrollBar,
    pub scrollbar_status_label: StaticText,
    pub cmd_link_button: CommandLinkButton,
    pub collapsible_pane: CollapsiblePane,
}

pub fn create_basic_tab(notebook: &Notebook, _frame: &Frame) -> BasicTabControls {
    let basic_panel = Panel::builder(notebook)
        .with_style(PanelStyle::TabTraversal)
        .build();

    let static_text_label = StaticText::builder(&basic_panel)
        .with_label("Text Input:")
        .build();
    static_text_label.set_tooltip("This is a label for the text input field.");
    let text_ctrl = TextCtrl::builder(&basic_panel)
        .with_value("Edit me!")
        .with_style(TextCtrlStyle::ProcessEnter)
        .build();
    text_ctrl.set_tooltip("Enter text here.");
    let spin_button_label = StaticText::builder(&basic_panel)
        .with_label("Spin Button:")
        .build();
    let spin_button = SpinButton::builder(&basic_panel).build();
    spin_button.set_tooltip("Click arrows or use keys to change the value (wraps around).");
    let spinctrl_double_label_widget = StaticText::builder(&basic_panel)
        .with_label("Spin Double:")
        .build();
    let spinctrl_double = SpinCtrlDouble::builder(&basic_panel)
        .with_range(0.0, 100.0)
        .with_initial_value(50.5)
        .with_increment(0.1)
        .build();
    spinctrl_double.set_digits(2);
    let spinctrl_double_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!("{:.2}", spinctrl_double.get_value()))
        .build();

    let checkbox_label_widget = StaticText::builder(&basic_panel)
        .with_label("Checkbox:")
        .build();
    let checkbox = CheckBox::builder(&basic_panel)
        .with_label("Enable Feature")
        .build();
    checkbox.set_tooltip("Toggle this checkbox.");
    let radio_box_label = StaticText::builder(&basic_panel)
        .with_label("Radio Box:")
        .build();
    let radio_box_choices = ["Option 1", "Option 2", "Option 3"];
    let radio_box = RadioBox::builder(&basic_panel, &radio_box_choices)
        .with_label("RadioBox Title")
        .with_major_dimension(1) // 1 column
        .with_style(RadioBoxStyle::SpecifyCols)
        .build();
    radio_box.set_selection(0);
    radio_box.set_tooltip("Select one option from the radio box.");
    let radio_label = StaticText::builder(&basic_panel)
        .with_label("Radio Buttons:")
        .build();
    let radio1 = RadioButton::builder(&basic_panel)
        .with_label("Option 1")
        .first_in_group()
        .build();
    radio1.set_tooltip("Select Option 1.");
    let radio2 = RadioButton::builder(&basic_panel)
        .with_label("Option 2")
        .build();
    radio2.set_tooltip("Select Option 2.");
    radio1.set_value(true);
    let radio_status_label = StaticText::builder(&basic_panel)
        .with_label("Selected: Option 1")
        .build();

    let toggle_button_label = StaticText::builder(&basic_panel)
        .with_label("Toggle Button:")
        .build();
    let toggle_button = ToggleButton::builder(&basic_panel)
        .with_label("Toggle Status")
        .build();
    toggle_button.set_value(true);
    toggle_button.set_tooltip("Click to toggle ON/OFF.");
    let toggle_status_label = StaticText::builder(&basic_panel).with_label("ON").build();

    let cmd_link_button_label = StaticText::builder(&basic_panel)
        .with_label("Cmd Link Btn:")
        .build();
    let cmd_link_button = CommandLinkButton::builder(&basic_panel)
        .with_label("Open System Settings")
        .with_note("Configure your display, network, and other system preferences.")
        .build();
    cmd_link_button.set_tooltip("Click to open system settings (simulated).");

    const BMP_WIDTH: u32 = 16;
    const BMP_HEIGHT: u32 = 16;
    let mut bmp_data = vec![0u8; (BMP_WIDTH * BMP_HEIGHT * 4) as usize];
    for y in 0..BMP_HEIGHT {
        for x in 0..BMP_WIDTH {
            let idx = ((y * BMP_WIDTH + x) * 4) as usize;
            if !(2..BMP_WIDTH - 2).contains(&x) || !(2..BMP_HEIGHT - 2).contains(&y) {
                // Set border pixels to transparent
                bmp_data[idx + 3] = 0;
            } else {
                bmp_data[idx] = 255;
                bmp_data[idx + 3] = 255;
            }
        }
    }
    let red_bitmap =
        Bitmap::from_rgba(&bmp_data, BMP_WIDTH, BMP_HEIGHT).expect("Failed to create test bitmap");
    let bitmap_button_label = StaticText::builder(&basic_panel)
        .with_label("Bitmap Button:")
        .build();
    let bitmap_button = BitmapButton::builder(&basic_panel)
        .with_bitmap(Some(red_bitmap))
        .build();
    bitmap_button.set_tooltip("A button with a custom red square bitmap.");

    let open_icon_bitmap = ArtProvider::get_bitmap(ArtId::FileOpen, ArtClient::Button, None)
        .or_else(|| ArtProvider::get_bitmap(ArtId::Error, ArtClient::Button, None))
        .expect("Failed to get ART_FILE_OPEN or ART_ERROR icon");
    let art_button_label = StaticText::builder(&basic_panel)
        .with_label("Art Button:")
        .build();
    let art_button = BitmapButton::builder(&basic_panel)
        .with_bitmap(Some(open_icon_bitmap))
        .build();
    art_button.set_tooltip("A button using an icon from the ArtProvider.");

    let activity_label = StaticText::builder(&basic_panel)
        .with_label("Activity:")
        .build();
    let activity_indicator = ActivityIndicator::builder(&basic_panel).build();
    let activity_start_btn = Button::builder(&basic_panel).with_label("Start").build();
    activity_start_btn.set_tooltip("Start the activity indicator.");
    let activity_stop_btn = Button::builder(&basic_panel).with_label("Stop").build();
    activity_stop_btn.set_tooltip("Stop the activity indicator.");

    let colour_picker_label = StaticText::builder(&basic_panel)
        .with_label("Colour Picker:")
        .build();
    let colour_picker = ColourPickerCtrl::builder(&basic_panel)
        .with_initial_colour(colours::RED)
        .build();
    colour_picker.set_tooltip("Click to choose a colour.");
    let colour_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!("{:?}", colours::RED))
        .build();
    let date_picker_label_widget = StaticText::builder(&basic_panel)
        .with_label("Date Picker:")
        .build();
    let date_picker = DatePickerCtrl::builder(&basic_panel).build();
    date_picker.set_tooltip("Click to choose a date.");
    let initial_selected_date = date_picker.get_value();
    let date_picker_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!(
            "{:04}-{:02}-{:02}",
            initial_selected_date.year(),
            initial_selected_date.month(),
            initial_selected_date.day()
        ))
        .build();
    let calendar_label_widget = StaticText::builder(&basic_panel)
        .with_label("Calendar:")
        .build();
    let calendar_ctrl = CalendarCtrl::builder(&basic_panel).build();
    calendar_ctrl.set_tooltip("Select a date from the calendar.");

    // Set a default date to avoid null pointer issues
    let today = DateTime::now();
    calendar_ctrl.set_date(&today);

    let initial_calendar_date = calendar_ctrl.get_date().unwrap_or_else(DateTime::now);
    let calendar_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!(
            "{:04}-{:02}-{:02}",
            initial_calendar_date.year(),
            initial_calendar_date.month(),
            initial_calendar_date.day()
        ))
        .build();

    let search_ctrl_label = StaticText::builder(&basic_panel)
        .with_label("Search Ctrl:")
        .build();
    let search_ctrl = SearchCtrl::builder(&basic_panel)
        .with_value("Search...")
        .with_style(SearchCtrlStyle::ProcessEnter)
        .build();
    search_ctrl.show_search_button(true);
    search_ctrl.show_cancel_button(true);
    search_ctrl.set_tooltip("Enter search text here.");
    let bitmap_combo_box_label = StaticText::builder(&basic_panel)
        .with_label("Bitmap Combo:")
        .build();
    let open_bmp =
        ArtProvider::get_bitmap(ArtId::FileOpen, ArtClient::Menu, None).expect("Failed art");
    let save_bmp =
        ArtProvider::get_bitmap(ArtId::FileSave, ArtClient::Menu, None).expect("Failed art");
    let new_bmp = ArtProvider::get_bitmap(ArtId::New, ArtClient::Menu, None).expect("Failed art");
    let bitmap_combo_box = BitmapComboBox::builder(&basic_panel)
        .with_size(Size::new(200, -1))
        .with_value("Default Value")
        .build();
    bitmap_combo_box.append("Open", Some(&open_bmp));
    bitmap_combo_box.append("Save", Some(&save_bmp));
    bitmap_combo_box.append("New", Some(&new_bmp));
    bitmap_combo_box.append("No Icon", None);
    bitmap_combo_box.set_selection(0);
    bitmap_combo_box.set_tooltip("Select an item from the list with icons.");

    let hyperlink_label = StaticText::builder(&basic_panel)
        .with_label("Hyperlink:")
        .build();
    let hyperlink_ctrl = HyperlinkCtrl::builder(&basic_panel)
        .with_label("wxWidgets Official Website")
        .with_url("https://www.wxwidgets.org")
        .build();
    hyperlink_ctrl.set_tooltip("Click to visit the wxWidgets website.");

    let scrollbar_label = StaticText::builder(&basic_panel)
        .with_label("Scroll Bar:")
        .build();
    let scroll_bar = ScrollBar::builder(&basic_panel)
        .with_style(ScrollBarStyle::Default) // Use Default for Horizontal
        .build();
    scroll_bar.set_scrollbar(0, 10, 100, 10, true); // position, thumb_size, range, page_size, refresh
    let scrollbar_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!("{}", scroll_bar.thumb_position()))
        .build();

    // CollapsiblePane
    let collapsible_pane_label = StaticText::builder(&basic_panel)
        .with_label("Collapsible:")
        .build();
    let collapsible_pane = CollapsiblePane::builder(&basic_panel)
        .with_label("Click to expand/collapse")
        .with_style(CollapsiblePaneStyle::Default)
        .build();
    collapsible_pane.set_tooltip("Click to expand or collapse the pane content.");

    // Add content to the collapsible pane
    if let Some(pane_window) = collapsible_pane.get_pane() {
        let pane_content_text = StaticText::builder(&pane_window)
            .with_label(
                "This content is inside the collapsible pane!\nYou can add any widgets here.",
            )
            .build();

        let pane_button = Button::builder(&pane_window)
            .with_label("Button in pane")
            .build();

        // Layout the content in the pane
        let pane_sizer = BoxSizer::builder(Orientation::Vertical).build();
        pane_sizer.add(&pane_content_text, 0, SizerFlag::All, 5);
        pane_sizer.add(&pane_button, 0, SizerFlag::All, 5);
        pane_window.set_sizer(pane_sizer, true);
    }

    // --- Layout using Main Vertical BoxSizer and child FlexGridSizers ---
    let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
    let label_flags = SizerFlag::AlignRight | SizerFlag::AlignCenterVertical;
    let control_flags = SizerFlag::Expand;

    let grid_sizer_group1 = FlexGridSizer::builder(0, 2)
        .with_vgap(5)
        .with_hgap(5)
        .build();
    grid_sizer_group1.add_growable_col(0, 1);
    grid_sizer_group1.add_growable_col(1, 3);
    grid_sizer_group1.add(&static_text_label, 0, label_flags, 0);
    grid_sizer_group1.add(&text_ctrl, 1, control_flags, 0);
    grid_sizer_group1.add(&spin_button_label, 0, label_flags, 0);
    grid_sizer_group1.add(
        &spin_button,
        0,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );
    grid_sizer_group1.add(&spinctrl_double_label_widget, 0, label_flags, 0);
    let spin_double_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    spin_double_sizer.add(&spinctrl_double, 0, SizerFlag::AlignCenterVertical, 0);
    spin_double_sizer.add_spacer(5);
    spin_double_sizer.add(&spinctrl_double_status_label, 1, SizerFlag::Expand, 0);
    grid_sizer_group1.add_sizer(&spin_double_sizer, 1, control_flags, 0);
    main_sizer.add_sizer(
        &grid_sizer_group1,
        0,
        SizerFlag::Expand | SizerFlag::All,
        10,
    );

    let static_line_sep1 = StaticLine::builder(&basic_panel)
        .with_style(StaticLineStyle::Default) // Use Default for Horizontal
        .build();
    main_sizer.add(&static_line_sep1, 0, SizerFlag::Expand | SizerFlag::All, 5);

    let grid_sizer_therest = FlexGridSizer::builder(0, 2)
        .with_vgap(5)
        .with_hgap(5)
        .build();
    grid_sizer_therest.add_growable_col(0, 1);
    grid_sizer_therest.add_growable_col(1, 3);

    grid_sizer_therest.add(&checkbox_label_widget, 0, label_flags, 0);
    grid_sizer_therest.add(
        &checkbox,
        1,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );
    grid_sizer_therest.add(&radio_label, 0, label_flags, 0);
    let radio_button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    radio_button_sizer.add(&radio1, 0, SizerFlag::AlignCenterVertical, 0);
    radio_button_sizer.add_spacer(5);
    radio_button_sizer.add(&radio2, 0, SizerFlag::AlignCenterVertical, 0);
    radio_button_sizer.add_spacer(10);
    radio_button_sizer.add(&radio_status_label, 1, SizerFlag::Expand, 0);
    grid_sizer_therest.add_sizer(&radio_button_sizer, 1, control_flags, 0);
    grid_sizer_therest.add(&radio_box_label, 0, label_flags, 0);
    grid_sizer_therest.add(&radio_box, 1, control_flags, 0);

    grid_sizer_therest.add(&toggle_button_label, 0, label_flags, 0);
    let toggle_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    toggle_sizer.add(&toggle_button, 0, SizerFlag::AlignCenterVertical, 0);
    toggle_sizer.add_spacer(5);
    toggle_sizer.add(&toggle_status_label, 1, SizerFlag::Expand, 0);
    grid_sizer_therest.add_sizer(
        &toggle_sizer,
        1,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );
    grid_sizer_therest.add(&bitmap_button_label, 0, label_flags, 0);
    grid_sizer_therest.add(
        &bitmap_button,
        0,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );
    grid_sizer_therest.add(&art_button_label, 0, label_flags, 0);
    grid_sizer_therest.add(
        &art_button,
        0,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );
    grid_sizer_therest.add(&activity_label, 0, label_flags, 0);
    let activity_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    activity_sizer.add(&activity_indicator, 0, SizerFlag::AlignCenterVertical, 0);
    activity_sizer.add_spacer(5);
    activity_sizer.add(&activity_start_btn, 0, SizerFlag::AlignCenterVertical, 5);
    activity_sizer.add_spacer(5);
    activity_sizer.add(&activity_stop_btn, 0, SizerFlag::AlignCenterVertical, 5);
    grid_sizer_therest.add_sizer(
        &activity_sizer,
        1,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );
    grid_sizer_therest.add(&scrollbar_label, 0, label_flags, 0);
    let scrollbar_h_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    scrollbar_h_sizer.add(&scroll_bar, 1, SizerFlag::Expand, 0);
    scrollbar_h_sizer.add_spacer(5);
    scrollbar_h_sizer.add(
        &scrollbar_status_label,
        0,
        SizerFlag::AlignCenterVertical,
        0,
    );
    grid_sizer_therest.add_sizer(&scrollbar_h_sizer, 1, control_flags, 0);

    grid_sizer_therest.add(&colour_picker_label, 0, label_flags, 0);
    let colour_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    colour_sizer.add(&colour_picker, 0, SizerFlag::AlignCenterVertical, 0);
    colour_sizer.add_spacer(5);
    colour_sizer.add(&colour_status_label, 1, SizerFlag::Expand, 0);
    grid_sizer_therest.add_sizer(
        &colour_sizer,
        1,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );
    grid_sizer_therest.add(&date_picker_label_widget, 0, label_flags, 0);
    let date_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    date_sizer.add(&date_picker, 0, SizerFlag::AlignCenterVertical, 0);
    date_sizer.add_spacer(5);
    date_sizer.add(&date_picker_status_label, 1, SizerFlag::Expand, 0);
    grid_sizer_therest.add_sizer(
        &date_sizer,
        1,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );

    // Add Time Picker
    let time_picker_label_widget = StaticText::builder(&basic_panel)
        .with_label("Time Picker:")
        .build();
    let time_picker = TimePickerCtrl::builder(&basic_panel).build();
    time_picker.set_tooltip("Click to choose a time.");
    let initial_time = time_picker.get_value();
    let time_picker_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!(
            "{:02}:{:02}:{:02}",
            initial_time.hour(),
            initial_time.minute(),
            initial_time.second()
        ))
        .build();
    grid_sizer_therest.add(&time_picker_label_widget, 0, label_flags, 0);
    let time_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    time_sizer.add(&time_picker, 0, SizerFlag::AlignCenterVertical, 0);
    time_sizer.add_spacer(5);
    time_sizer.add(&time_picker_status_label, 1, SizerFlag::Expand, 0);
    grid_sizer_therest.add_sizer(
        &time_sizer,
        1,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );

    grid_sizer_therest.add(&calendar_label_widget, 0, label_flags, 0);
    let calendar_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    calendar_sizer.add(&calendar_ctrl, 1, SizerFlag::Expand, 0);
    calendar_sizer.add_spacer(5);
    calendar_sizer.add(&calendar_status_label, 1, SizerFlag::Expand, 0);
    grid_sizer_therest.add_sizer(
        &calendar_sizer,
        1,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );

    grid_sizer_therest.add(&search_ctrl_label, 0, label_flags, 0);
    grid_sizer_therest.add(&search_ctrl, 1, control_flags, 0);
    grid_sizer_therest.add(&bitmap_combo_box_label, 0, label_flags, 0);
    grid_sizer_therest.add(&bitmap_combo_box, 1, control_flags, 0);
    grid_sizer_therest.add(&hyperlink_label, 0, label_flags, 0);
    grid_sizer_therest.add(
        &hyperlink_ctrl,
        1,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );
    grid_sizer_therest.add(&collapsible_pane_label, 0, label_flags, 0);
    grid_sizer_therest.add(&collapsible_pane, 1, control_flags, 0);
    grid_sizer_therest.add(&cmd_link_button_label, 0, label_flags, 0);
    grid_sizer_therest.add(&cmd_link_button, 1, control_flags, 0);

    main_sizer.add_sizer(
        &grid_sizer_therest,
        1,
        SizerFlag::Expand | SizerFlag::All,
        10,
    );
    basic_panel.set_sizer_and_fit(main_sizer, true);

    // Return all the created controls
    BasicTabControls {
        panel: basic_panel,
        text_ctrl,
        spin_button,
        checkbox,
        radio1,
        radio2,
        radio_status_label,
        toggle_button,
        toggle_status_label,
        bitmap_button,
        art_button,
        radio_box,
        bitmap_combo_box,
        colour_picker,
        colour_label: colour_status_label,
        date_picker,
        date_picker_label: date_picker_status_label,
        time_picker,
        time_picker_label: time_picker_status_label,
        search_ctrl,
        hyperlink_ctrl,
        activity_indicator,
        activity_start_btn,
        activity_stop_btn,
        spinctrl_double,
        spinctrl_double_label: spinctrl_double_status_label,
        calendar_ctrl,
        calendar_label: calendar_status_label,
        scroll_bar,
        scrollbar_status_label,
        cmd_link_button,
        collapsible_pane,
    }
}

// Add this method to bind all events for basic tab controls
impl BasicTabControls {
    pub fn bind_events(&self) {
        // BitmapButton click event
        self.bitmap_button.on_click(move |event| {
            println!("Bitmap Button clicked: {}", event.get_id());
        });

        // ArtProvider BitmapButton click event
        self.art_button.on_click(move |event| {
            println!(
                "ArtProvider Button (ID: {}) clicked! Icon from ArtProvider works.",
                event.get_id()
            );
        });

        // Checkbox Event Binding
        self.checkbox.on_toggled(move |event_data| {
            println!(
                "Checkbox Event: ID: {}, Checked: {}",
                event_data.get_id(),
                event_data.is_checked()
            );
        });

        // TextCtrl events
        self.text_ctrl.on_text_changed(move |_event| {
            // Example: log text changes if needed
            // if let Some(text) = _event.get_string() {
            //     println!("Text: {}", text);
            // }
        });

        // TextCtrl Enter key event
        let spin_button_clone_bind_events = self.spin_button.clone();
        let text_ctrl_clone_bind_events = self.text_ctrl.clone();
        self.text_ctrl.on_text_enter(move |event| {
            if let Some(text) = event.get_string() {
                println!("Text Enter (from bind_events): {text}");
                if let Ok(num) = text.parse::<i32>() {
                    let min = spin_button_clone_bind_events.min();
                    let max = spin_button_clone_bind_events.max();
                    spin_button_clone_bind_events.set_value(num.clamp(min, max));
                } else {
                    text_ctrl_clone_bind_events
                        .set_value(&spin_button_clone_bind_events.value().to_string());
                }
            }
        });

        // RadioButton events
        let radio_status_label_clone1 = self.radio_status_label.clone();
        let radio1_clone = self.radio1.clone();
        self.radio1.on_selected(move |event_data| {
            let radio1_label_str = radio1_clone
                .get_label()
                .unwrap_or_else(|| "<unknown>".to_string());
            println!(
                "Radio Button Selected: ID {}, Label: {}",
                event_data.get_id(),
                radio1_label_str
            );
            radio_status_label_clone1.set_label(&format!("Selected: {radio1_label_str}"));
        });

        let radio_status_label_clone2 = self.radio_status_label.clone();
        let radio2_clone = self.radio2.clone();
        self.radio2.on_selected(move |event_data| {
            let radio2_label_str = radio2_clone
                .get_label()
                .unwrap_or_else(|| "<unknown>".to_string());
            println!(
                "Radio Button Selected: ID {}, Label: {}",
                event_data.get_id(),
                radio2_label_str
            );
            radio_status_label_clone2.set_label(&format!("Selected: {radio2_label_str}"));
        });

        // ToggleButton Event Binding
        let toggle_status_label_clone = self.toggle_status_label.clone();
        self.toggle_button.on_toggle(move |event| {
            let is_on = event.is_checked().unwrap_or(false);
            toggle_status_label_clone.set_label(if is_on { "ON" } else { "OFF" });
            println!("ToggleButton clicked, is_on: {is_on}");
        });

        // SpinButton Event Bindings
        let text_ctrl_clone = self.text_ctrl.clone();
        self.spin_button.on_spin(move |event| {
            let value = event.get_int().unwrap_or(0);
            text_ctrl_clone.set_value(&value.to_string());
            println!("SpinButton changed to: {value}");
        });

        // RadioBox Event binding
        let radio_box_clone = self.radio_box.clone();
        self.radio_box.on_selected(move |event| {
            let selection = event.get_selection().unwrap_or(0);
            let selection_string = radio_box_clone.get_string(selection);
            println!("RadioBox selected: {selection_string} ({selection})");
        });

        // TimePicker event
        let time_picker_clone = self.time_picker.clone();
        let time_picker_label_clone = self.time_picker_label.clone();
        self.time_picker.on_time_changed(move |_event| {
            let selected_time = time_picker_clone.get_value();
            time_picker_label_clone.set_label(&format!(
                "{:02}:{:02}:{:02}",
                selected_time.hour(),
                selected_time.minute(),
                selected_time.second()
            ));
            println!("Time picker value changed!");
        });

        // BitmapComboBox Event
        let bitmap_combo_box_clone = self.bitmap_combo_box.clone();
        self.bitmap_combo_box.on_selection_changed(move |event| {
            let selection_idx = event.get_selection().unwrap_or(0);
            let mut log_output = format!("BitmapComboBox selected item {selection_idx}");

            let item_str = bitmap_combo_box_clone.get_string(selection_idx as u32);
            log_output.push_str(&format!(", Selected Item [{selection_idx}]: '{item_str}'"));

            if let Some(bitmap) = bitmap_combo_box_clone.get_item_bitmap(selection_idx as u32) {
                log_output.push_str(&format!(
                    ", Item has bitmap (w:{}, h:{})",
                    bitmap.get_width(),
                    bitmap.get_height()
                ));
            } else {
                log_output.push_str(", Item has no bitmap or bitmap is invalid");
            }

            println!("{log_output}");
        });

        // SearchCtrl Event Handlers
        let search_ctrl_clone_search = self.search_ctrl.clone();
        self.search_ctrl.on_search_button_clicked(move |event| {
            println!(
                "SEARCH_CTRL Event: Search Button Clicked! ID: {}, Value: \"{}\"",
                event.get_id(),
                search_ctrl_clone_search.get_value()
            );
        });

        let search_ctrl_clone_cancel = self.search_ctrl.clone();
        self.search_ctrl.on_cancel_button_clicked(move |event| {
            let value_before_clear = search_ctrl_clone_cancel.get_value();
            search_ctrl_clone_cancel.set_value("");
            println!(
                "SEARCH_CTRL Event: Cancel Button Clicked! ID: {}, Value was: \"{}\"",
                event.get_id(),
                value_before_clear
            );
        });

        // TextCtrl Enter for SearchCtrl
        self.search_ctrl.on_text_updated(move |event| {
            println!(
                "SEARCH_CTRL Event: Text Entered! Value: \"{}\"",
                event.get_string().unwrap_or_default()
            );
        });

        // DatePicker event
        let date_picker_clone = self.date_picker.clone();
        let date_picker_label_clone = self.date_picker_label.clone();
        self.date_picker.on_date_changed(move |_event| {
            let selected_date = date_picker_clone.get_value();
            date_picker_label_clone.set_label(&format!(
                "{:04}-{:02}-{:02}",
                selected_date.year(),
                selected_date.month(),
                selected_date.day()
            ));
            println!("Date picker value changed!");
        });

        // Calendar event
        let calendar_ctrl_clone = self.calendar_ctrl.clone();
        let calendar_label_clone = self.calendar_label.clone();
        self.calendar_ctrl.on_selection_changed(move |_event| {
            if let Some(selected_date) = calendar_ctrl_clone.get_date() {
                let date_str = format!(
                    "{:04}-{:02}-{:02}",
                    selected_date.year(),
                    selected_date.month(),
                    selected_date.day()
                );
                calendar_label_clone.set_label(&date_str);
                println!("CALENDAR_SEL_CHANGED: Date: {date_str}");
            }
        });

        // Activity indicator controls
        let activity_indicator_clone = self.activity_indicator.clone();
        self.activity_start_btn.on_click(move |_event| {
            activity_indicator_clone.start();
            println!("Activity indicator started");
        });

        let activity_indicator_clone_stop = self.activity_indicator.clone();
        self.activity_stop_btn.on_click(move |_event| {
            activity_indicator_clone_stop.stop();
            println!("Activity indicator stopped");
        });

        // SpinCtrlDouble event
        let spinctrld_label_clone = self.spinctrl_double_label.clone();
        let spinctrld_clone = self.spinctrl_double.clone();
        self.spinctrl_double.on_value_changed(move |_event| {
            let current_value = spinctrld_clone.get_value();
            spinctrld_label_clone.set_label(&format!("{current_value:.2}"));
            println!("SpinCtrlDouble Value: {current_value:.2}");
        });

        // ScrollBar event
        let scrollbar_status_label_clone = self.scrollbar_status_label.clone();
        let scroll_bar_clone = self.scroll_bar.clone();
        self.scroll_bar.on_thumb_track(move |_event| {
            let pos = scroll_bar_clone.thumb_position();
            scrollbar_status_label_clone.set_label(&format!("{pos}"));
            println!("Scroll Pos: {pos}");
        });

        // Colour picker event
        let colour_label_clone = self.colour_label.clone();
        self.colour_picker.on_colour_changed(move |event| {
            let colour = event.get_colour();
            let label_text = format!("{colour:?}");
            colour_label_clone.set_label(&label_text);
            println!("ColourPicker changed: {colour:?}");
        });

        // Hyperlink event
        let hyperlink_ctrl_clone = self.hyperlink_ctrl.clone();
        self.hyperlink_ctrl.on_clicked(move |_event| {
            let url = hyperlink_ctrl_clone.get_url();
            println!(
                "HYPERLINK Clicked! URL: \"{}\", Visited: {}",
                url,
                hyperlink_ctrl_clone.get_visited(),
            );
        });

        // Command link button
        let cmd_link_button_clone = self.cmd_link_button.clone();
        self.cmd_link_button.on_click(move |_event| {
            MessageDialog::builder(
                &cmd_link_button_clone,
                "CommandLinkButton was clicked!\n(Simulating opening system settings)",
                "Command Link Action",
            )
            .with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation)
            .build()
            .show_modal();
        });

        // CollapsiblePane event - trigger layout update when expanded/collapsed
        let collapsible_pane_clone = self.collapsible_pane.clone();
        let panel_clone = self.panel.clone();
        self.collapsible_pane.on_changed(move |_event| {
            if collapsible_pane_clone.is_expanded() {
                println!("CollapsiblePane expanded!");
            } else {
                println!("CollapsiblePane collapsed!");
            }
            // Update the layout to reposition other widgets
            panel_clone.layout();
        });
    }
}
