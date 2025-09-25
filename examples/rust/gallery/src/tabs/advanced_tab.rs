use wxdragon::prelude::*;

pub struct AdvancedTabControls {
    pub tree_ctrl: TreeCtrl,
    pub tree_status_label: StaticText,
    pub gauge: Gauge,
    pub gauge_increase_btn: Button,
    pub gauge_reset_btn: Button,
    pub gauge_status_label: StaticText,
    pub slider: Slider,
    pub spin_ctrl: SpinCtrl,
    pub spin_ctrl_label: StaticText,
}

pub fn create_advanced_tab(notebook: &Notebook) -> (SplitterWindow, AdvancedTabControls) {
    // Create a SplitterWindow instead of a Panel for this tab's main container
    let splitter = SplitterWindow::builder(notebook)
        .with_id(200) // Give splitter an ID
        .with_style(SplitterWindowStyle::LiveUpdate | SplitterWindowStyle::Default)
        .with_size(Size::new(400, 200))
        .build();

    // Create Panel 1 (Left: Tree)
    let tree_panel = Panel::builder(&splitter).build();
    let tree_ctrl = TreeCtrl::builder(&tree_panel)
        .with_id(111)
        .with_style(TreeCtrlStyle::Default | TreeCtrlStyle::HasButtons | TreeCtrlStyle::LinesAtRoot)
        .build();

    // Add items to the tree
    if let Some(root_id) = tree_ctrl.add_root("Root Node", None, None) {
        if let Some(child1_id) = tree_ctrl.append_item(&root_id, "Child 1", None, None) {
            let _grandchild1 = tree_ctrl.append_item(&child1_id, "Grandchild 1.1", None, None);
            let _grandchild2 = tree_ctrl.append_item(&child1_id, "Grandchild 1.2", None, None);
        }
        let _child2 = tree_ctrl.append_item(&root_id, "Child 2", None, None);
    }

    let tree_status_label = StaticText::builder(&tree_panel)
        .with_label("Tree Selection: None")
        .build();

    // Sizer for Tree Panel
    let tree_sizer = BoxSizer::builder(Orientation::Vertical).build();
    tree_sizer.add(&tree_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 5);
    tree_sizer.add(&tree_status_label, 0, SizerFlag::Expand | SizerFlag::All, 5);
    tree_panel.set_sizer(tree_sizer, true);

    // Create Panel 2 (Right: Controls)
    let controls_panel = Panel::builder(&splitter).build();
    let controls_sizer = BoxSizer::builder(Orientation::Vertical).build();

    // --- Gauge Controls section ---
    let gauge_panel = Panel::builder(&controls_panel).build();
    let gauge_sizer = BoxSizer::builder(Orientation::Vertical).build();

    // Add a title
    let gauge_title = StaticText::builder(&gauge_panel)
        .with_label("Gauge Demo")
        .build();
    gauge_sizer.add_spacer(10);
    gauge_sizer.add(
        &gauge_title,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );

    // Create the gauge
    let gauge = Gauge::builder(&gauge_panel)
        .with_range(100)
        .with_size(Size::new(200, -1))
        .build();

    // Set initial value after creation since the builder doesn't have with_value
    gauge.set_value(25);

    gauge_sizer.add(&gauge, 0, SizerFlag::Expand | SizerFlag::All, 5);

    // Create gauge control buttons
    let gauge_button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    let gauge_increase_btn = Button::builder(&gauge_panel)
        .with_label("Increase (+10)")
        .build();
    let gauge_reset_btn = Button::builder(&gauge_panel).with_label("Reset").build();

    gauge_button_sizer.add(&gauge_increase_btn, 0, SizerFlag::All, 5);
    gauge_button_sizer.add(&gauge_reset_btn, 0, SizerFlag::All, 5);
    gauge_sizer.add_sizer(&gauge_button_sizer, 0, SizerFlag::AlignCenterHorizontal, 0);

    // Gauge status label
    let gauge_status_label = StaticText::builder(&gauge_panel)
        .with_label("Gauge Value: 25%")
        .build();
    gauge_sizer.add(
        &gauge_status_label,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );

    gauge_panel.set_sizer(gauge_sizer, true);

    // --- Slider section ---
    let slider_panel = Panel::builder(&controls_panel).build();
    let slider_sizer = BoxSizer::builder(Orientation::Vertical).build();

    let slider_title = StaticText::builder(&slider_panel)
        .with_label("Slider Demo (Controls Gauge)")
        .build();
    slider_sizer.add_spacer(10);
    slider_sizer.add(
        &slider_title,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );

    // Create the slider
    let slider = Slider::builder(&slider_panel)
        .with_value(25)
        .with_min_value(0)
        .with_max_value(100)
        .with_size(Size::new(200, -1))
        .build();
    slider_sizer.add(&slider, 0, SizerFlag::Expand | SizerFlag::All, 5);
    slider_panel.set_sizer(slider_sizer, true);

    // --- SpinCtrl section ---
    let spin_panel = Panel::builder(&controls_panel).build();
    let spin_sizer = BoxSizer::builder(Orientation::Vertical).build();

    let spin_title = StaticText::builder(&spin_panel)
        .with_label("SpinCtrl Demo")
        .build();
    spin_sizer.add_spacer(10);
    spin_sizer.add(
        &spin_title,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );

    const INITIAL_VALUE: i32 = 42;
    // Create the spin control
    let spin_ctrl = SpinCtrl::builder(&spin_panel)
        // .with_value_str(&INITIAL_VALUE.to_string())
        .with_initial_value(INITIAL_VALUE)
        .with_min_value(0)
        .with_max_value(1000)
        .build();
    spin_sizer.add(
        &spin_ctrl,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );

    // SpinCtrl status label
    let spin_ctrl_label = StaticText::builder(&spin_panel)
        .with_label(&format!("Spin Value: {INITIAL_VALUE}"))
        .build();
    spin_sizer.add(
        &spin_ctrl_label,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );

    spin_panel.set_sizer(spin_sizer, true);

    // --- Drag and Drop Demo section ---
    let dnd_panel = Panel::builder(&controls_panel).build();
    let dnd_sizer = BoxSizer::builder(Orientation::Vertical).build();
    add_dnd_demo(&dnd_panel, &dnd_sizer);
    dnd_panel.set_sizer(dnd_sizer, true);

    // Add all sections to the controls panel
    controls_sizer.add(&gauge_panel, 0, SizerFlag::Expand | SizerFlag::All, 5);
    controls_sizer.add(&slider_panel, 0, SizerFlag::Expand | SizerFlag::All, 5);
    controls_sizer.add(&spin_panel, 0, SizerFlag::Expand | SizerFlag::All, 5);
    controls_sizer.add(&dnd_panel, 1, SizerFlag::Expand | SizerFlag::All, 5);

    controls_panel.set_sizer(controls_sizer, true);

    // Split the window
    splitter.split_vertically(&tree_panel, &controls_panel, 250);
    splitter.set_minimum_pane_size(50);

    // Return the splitter AND the controls struct
    (
        splitter,
        AdvancedTabControls {
            tree_ctrl,
            tree_status_label,
            gauge,
            gauge_increase_btn,
            gauge_reset_btn,
            gauge_status_label,
            slider,
            spin_ctrl,
            spin_ctrl_label,
        },
    )
}

impl AdvancedTabControls {
    pub fn bind_events(&self) {
        // TreeCtrl Selection Changed event
        let tree_status_label_clone = self.tree_status_label.clone();
        self.tree_ctrl.on_selection_changed(move |event_data| {
            if let Some(selected_item) = event_data.get_item() {
                let mut status = String::new();
                std::fmt::Write::write_fmt(
                    &mut status,
                    format_args!("Tree Selection: Item {selected_item:?}"),
                )
                .unwrap();
                tree_status_label_clone.set_label(&status);
            } else {
                tree_status_label_clone.set_label("Tree Selection: None");
            }
        });

        // Gauge button events
        let gauge_clone_for_inc = self.gauge.clone();
        let gauge_status_label_clone_for_inc = self.gauge_status_label.clone();
        self.gauge_increase_btn.on_click(move |_event| {
            let current_value = gauge_clone_for_inc.get_value();
            let new_value = std::cmp::min(current_value + 10, 100);
            gauge_clone_for_inc.set_value(new_value);
            gauge_status_label_clone_for_inc.set_label(&format!("Gauge Value: {new_value}%"));
        });

        let gauge_clone_for_reset = self.gauge.clone();
        let gauge_status_label_clone_for_reset = self.gauge_status_label.clone();
        self.gauge_reset_btn.on_click(move |_event| {
            gauge_clone_for_reset.set_value(0);
            gauge_status_label_clone_for_reset.set_label("Gauge Value: 0%");
        });

        // Slider Event Binding
        let gauge_clone = self.gauge.clone();
        let gauge_status_label_clone = self.gauge_status_label.clone();
        self.slider.on_thumb_track(move |event_data| {
            let value = event_data.get_position().unwrap_or(0);
            gauge_clone.set_value(value);
            gauge_status_label_clone.set_label(&format!("Gauge Value: {value}"));
        });

        // Timer for Gauge Pulse
        let gauge_status_label_clone_timer = self.gauge_status_label.clone();
        self.slider.on_scroll_changed(move |event_data| {
            let value = event_data.get_position().unwrap_or(0);
            gauge_status_label_clone_timer.set_label(&format!("Gauge Value: {value}"));
        });

        // SpinCtrl Event Binding
        let spin_ctrl_label_clone = self.spin_ctrl_label.clone();
        self.spin_ctrl.on_value_changed(move |event| {
            let value = event.get_value();
            spin_ctrl_label_clone.set_label(&format!("Spin Value: {value}"));
            println!(
                "SPINCTRL Event (Advanced Tab): ID: {}, Value: {}",
                event.base.get_id(),
                value
            );
        });
    }
}

fn add_dnd_demo(panel: &Panel, sizer: &BoxSizer) {
    // Add a title
    let title = StaticText::builder(panel)
        .with_label("Drag and Drop Demo")
        .build();
    sizer.add_spacer(10);
    sizer.add(
        &title,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );
    sizer.add_spacer(10);

    // Create a horizontal sizer for the text drag source and drop target
    let h_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    // Create a drag source panel
    let source_panel = Panel::builder(panel).with_size(Size::new(200, 150)).build();
    source_panel.set_background_color(Colour::new(173, 216, 230, 255)); // Light blue color

    // Create a vertical sizer for source panel contents
    let source_sizer = BoxSizer::builder(Orientation::Vertical).build();

    // Add text to the source panel
    let source_text = StaticText::builder(&source_panel)
        .with_label("Drag from here")
        .build();

    // Center the text in the panel
    source_sizer.add_spacer(60); // Push the text down for vertical centering
    source_sizer.add(&source_text, 0, SizerFlag::AlignCenterHorizontal, 0);
    source_panel.set_sizer(source_sizer, true);

    // Create a drop target panel
    let target_panel = Panel::builder(panel).with_size(Size::new(200, 150)).build();
    target_panel.set_background_color(Colour::new(144, 238, 144, 255)); // Light green color

    // Create a vertical sizer for target panel contents
    let target_sizer = BoxSizer::builder(Orientation::Vertical).build();

    // Add text to the target panel
    let target_label = StaticText::builder(&target_panel)
        .with_label("Drop here")
        .build();

    // Create a text control to display the dropped text
    let dropped_text = TextCtrl::builder(&target_panel)
        .with_size(Size::new(-1, 60)) // Width will be set automatically by the sizer
        .with_style(TextCtrlStyle::MultiLine)
        .build();

    // Add widgets to the target sizer with proper spacing
    target_sizer.add_spacer(20);
    target_sizer.add(&target_label, 0, SizerFlag::AlignCenterHorizontal, 0);
    target_sizer.add_spacer(10);
    target_sizer.add(
        &dropped_text,
        1,
        SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right,
        10,
    ); // Add expand and margins
    target_panel.set_sizer(target_sizer, true);

    // Set up the drag source
    source_panel.on_mouse_left_down({
        let source_panel_ptr = source_panel.clone();
        move |_| {
            // Create the data object
            let data = TextDataObject::new("Text dragged from wxDragon!");

            // Create the drop source
            let drop_source = DropSource::new(&source_panel_ptr);
            drop_source.set_data(&data);

            // Start the drag operation
            let result = drop_source.do_drag_drop(true);
            println!("Drag result: {result}");
        }
    });

    // Set up the text drop target using the builder pattern
    let _text_drop_target = TextDropTarget::builder(&target_panel)
        .with_on_drop_text({
            let dropped_text = dropped_text.clone();
            move |text, x, y| {
                println!("Text dropped at ({x}, {y}): {text}");
                dropped_text.set_value(text);
                true // Accept the drop
            }
        })
        .build();

    // Add the panels to the horizontal sizer
    h_sizer.add(&source_panel, 1, SizerFlag::Expand | SizerFlag::All, 10);
    h_sizer.add(&target_panel, 1, SizerFlag::Expand | SizerFlag::All, 10);

    // Add the horizontal sizer to the main sizer
    sizer.add_sizer(&h_sizer, 0, SizerFlag::Expand | SizerFlag::All, 5);

    // Add instructions for text drag and drop
    let instructions = StaticText::builder(panel)
        .with_label("Click and drag from the light blue panel to the light green panel")
        .build();
    sizer.add(
        &instructions,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );

    // Add file drop target demo
    let file_drop_title = StaticText::builder(panel)
        .with_label("File Drop Demo")
        .build();
    sizer.add_spacer(10);
    sizer.add(
        &file_drop_title,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );

    // Create a file drop target panel
    let file_target_panel = Panel::builder(panel).with_size(Size::new(-1, 150)).build();
    file_target_panel.set_background_color(Colour::new(230, 230, 250, 255)); // Lavender color

    // Create a vertical sizer for file panel contents
    let file_sizer = BoxSizer::builder(Orientation::Vertical).build();

    // Add text to the file drop target panel
    let file_target_label = StaticText::builder(&file_target_panel)
        .with_label("Drop files here")
        .build();

    // Create a text control to display the dropped files
    let file_list = TextCtrl::builder(&file_target_panel)
        .with_size(Size::new(-1, 90))
        .with_style(TextCtrlStyle::MultiLine)
        .build();

    // Add widgets to the file sizer with proper spacing
    file_sizer.add_spacer(15);
    file_sizer.add(&file_target_label, 0, SizerFlag::AlignCenterHorizontal, 0);
    file_sizer.add_spacer(10);
    file_sizer.add(
        &file_list,
        1,
        SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right,
        20,
    ); // Add expand and larger margins
    file_target_panel.set_sizer(file_sizer, true);

    // Set up the file drop target using the builder pattern
    let _file_drop_target = FileDropTarget::builder(&file_target_panel)
        .with_on_drop_files({
            let file_list = file_list.clone();
            move |files, x, y| {
                println!("Files dropped at ({}, {}): {} files", x, y, files.len());

                // Clear the text field
                file_list.set_value("");

                // Add each file path to the text field
                for file in files {
                    let current_text = file_list.get_value();
                    let new_text = if current_text.is_empty() {
                        file
                    } else {
                        format!("{current_text}\n{file}")
                    };
                    file_list.set_value(&new_text);
                }
                true // Accept the drop
            }
        })
        .build();

    // Add the file drop target panel to the main sizer
    sizer.add(
        &file_target_panel,
        0,
        SizerFlag::Expand | SizerFlag::All,
        10,
    );

    // Add instructions for file drag and drop
    let file_instructions = StaticText::builder(panel)
        .with_label("Drag and drop files from your file explorer onto the lavender panel")
        .build();
    sizer.add(
        &file_instructions,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );
}
