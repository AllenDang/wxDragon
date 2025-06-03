use wxdragon::prelude::*;

// Load the frame XRC file
wxdragon::include_xrc!("../ui/frame.xrc", FrameUI);

// Load the panel XRC file
wxdragon::include_xrc!("../ui/panel.xrc", PanelUI);

// Load the dialog XRC file
wxdragon::include_xrc!("../ui/dialog.xrc", DialogUI);

fn main() {
    let _ = wxdragon::main(|_handle| {
        // 1. Load frame.xrc
        let frame_ui = FrameUI::new(None);

        println!("✓ Loaded frame.xrc");

        // Access StatusBar from the frame and configure it
        let statusbar = &frame_ui.main_statusbar;
        statusbar.set_fields_count(3);
        statusbar.set_status_text("Ready", 0);
        statusbar.set_status_text("XRC Demo", 1);
        statusbar.set_status_text("v1.0", 2);

        println!("✓ Configured StatusBar from XRC");

        // Get the main panel from the frame to embed content
        let _main_panel = &frame_ui.main_panel;
        let content_panel = &frame_ui.content_panel;

        // Set minimum size on main_panel to ensure proper space allocation
        _main_panel.set_min_size(wxdragon::geometry::Size {
            width: 450,
            height: 250,
        });

        // 2. Load panel.xrc and embed the panel in main frame
        let panel_ui = PanelUI::new(Some(content_panel));

        println!("✓ Loaded panel.xrc and embedded in frame");

        // IMPORTANT: Set a minimum size for content_panel to prevent it from shrinking to zero
        content_panel.set_min_size(wxdragon::geometry::Size {
            width: 400,
            height: 200,
        });

        // Use a sizer to properly layout the embedded panel
        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        sizer.add(
            &panel_ui.widget_panel,
            1,
            SizerFlag::Expand | SizerFlag::All,
            5,
        );
        content_panel.set_sizer(sizer, true);

        // Debug: Print sizes before layout
        println!("🔧 Debug sizes before layout:");
        let content_size = content_panel.get_size();
        println!(
            "  content_panel size: {}x{}",
            content_size.width, content_size.height
        );
        let widget_size = panel_ui.widget_panel.get_size();
        println!(
            "  widget_panel size: {}x{}",
            widget_size.width, widget_size.height
        );

        // Force layout on the content panel specifically
        content_panel.layout();

        // Show the widget panel explicitly
        panel_ui.widget_panel.show(true);

        // Debug: Print sizes after initial layout
        println!("🔧 Debug sizes after content_panel.layout():");
        let content_size = content_panel.get_size();
        println!(
            "  content_panel size: {}x{}",
            content_size.width, content_size.height
        );
        let widget_size = panel_ui.widget_panel.get_size();
        println!(
            "  widget_panel size: {}x{}",
            widget_size.width, widget_size.height
        );

        // 3. Load dialog but don't show it initially (pass frame as parent)
        let dialog_ui = DialogUI::new(Some(&frame_ui.main_frame));

        println!("✓ Loaded dialog.xrc");

        // Frame automatically handles layout with toolbar/statusbar

        // Tools are now auto-generated fields from XRC
        let up_tool = &frame_ui.m_tool_up;
        let down_tool = &frame_ui.m_tool_down;
        let help_tool = &frame_ui.m_tool_help;

        println!("✓ Using auto-generated XRC tool fields");

        // Bind events directly to auto-generated tool fields
        let statusbar_for_up = statusbar.clone();
        let up_tool_clone = up_tool.clone();
        up_tool_clone.on_click(move |_event| {
            println!("Up tool clicked!");
            statusbar_for_up.set_status_text("Up tool clicked", 0);
        });

        let statusbar_for_down = statusbar.clone();
        let down_tool_clone = down_tool.clone();
        down_tool_clone.on_click(move |_event| {
            println!("Down tool clicked!");
            statusbar_for_down.set_status_text("Down tool clicked", 0);
        });

        let statusbar_for_down = statusbar.clone();
        let help_tool_clone = help_tool.clone();
        help_tool_clone.on_click(move |_event| {
            println!("Help tool clicked!");
            statusbar_for_down.set_status_text("Help tool clicked", 0);
        });

        // 4. Handle menu items from auto-generated fields
        let menu_new = &frame_ui.menu_new;
        let menu_open = &frame_ui.menu_open;
        let menu_exit = &frame_ui.menu_exit;
        let menu_about = &frame_ui.menu_about;

        println!("✓ Using auto-generated XRC menu item fields");

        // Bind events to menu items
        let statusbar_for_new = statusbar.clone();
        let menu_new_clone = menu_new.clone();
        menu_new_clone.on_click(move |_event| {
            println!("New menu item clicked!");
            statusbar_for_new.set_status_text("New file created", 0);
        });

        let statusbar_for_open = statusbar.clone();
        let menu_open_clone = menu_open.clone();
        menu_open_clone.on_click(move |_event| {
            println!("Open menu item clicked!");
            statusbar_for_open.set_status_text("File opened", 0);
        });

        let frame_clone_for_exit = frame_ui.main_frame.clone();
        let menu_exit_clone = menu_exit.clone();
        menu_exit_clone.on_click(move |_event| {
            println!("Exit menu item clicked!");
            frame_clone_for_exit.close();
        });

        let statusbar_for_about = statusbar.clone();
        let menu_about_clone = menu_about.clone();
        menu_about_clone.on_click(move |_event| {
            println!("About menu item clicked!");
            statusbar_for_about.set_status_text("About dialog would open", 0);
        });

        // 5. Get widgets from the panel by name and bind events
        let show_dialog_btn = &panel_ui.show_dialog_btn;
        let action_btn = &panel_ui.action_btn;
        let enable_check = &panel_ui.enable_check;
        let option1_radio = &panel_ui.option1_radio;
        let option2_radio = &panel_ui.option2_radio;
        let choice_combo = &panel_ui.choice_combo;
        let items_list = &panel_ui.items_list;
        let input_text = &panel_ui.input_text;
        let multiline_text = &panel_ui.multiline_text;
        let search_ctrl = &panel_ui.search_ctrl;
        let toggle_btn = &panel_ui.toggle_btn;
        let value_slider = &panel_ui.value_slider;
        let progress_gauge = &panel_ui.progress_gauge;
        let number_spin = &panel_ui.number_spin;
        let status_label = &panel_ui.status_label;

        // Bind button events

        // Show dialog button
        let dialog_clone = dialog_ui.test_dialog.clone();
        show_dialog_btn.on_click(move |_event_data| {
            println!("Show dialog button clicked!");
            let result = dialog_clone.show_modal();
            println!("Dialog closed with result: {}", result);
        });

        // Action button
        let status_clone = status_label.clone();
        let statusbar_clone = statusbar.clone();
        action_btn.on_click(move |_event_data| {
            println!("Action button clicked!");
            status_clone.set_label("Action button was clicked!");
            statusbar_clone.set_status_text("Action button clicked", 0);
        });

        // Toggle button - fixed method name
        let status_clone2 = status_label.clone();
        let statusbar_clone2 = statusbar.clone();
        toggle_btn.on_toggle(move |event_data| {
            let is_pressed = event_data.is_checked().unwrap_or(false);
            println!("Toggle button: {}", if is_pressed { "ON" } else { "OFF" });
            status_clone2.set_label(&format!(
                "Toggle: {}",
                if is_pressed { "ON" } else { "OFF" }
            ));
            statusbar_clone2.set_status_text(
                &format!("Toggle: {}", if is_pressed { "ON" } else { "OFF" }),
                0,
            );
        });

        // Checkbox
        let status_clone3 = status_label.clone();
        let statusbar_clone3 = statusbar.clone();
        enable_check.on_toggled(move |event_data| {
            let is_checked = event_data.is_checked();
            println!(
                "Checkbox: {}",
                if is_checked { "CHECKED" } else { "UNCHECKED" }
            );
            status_clone3.set_label(&format!(
                "Enabled: {}",
                if is_checked { "YES" } else { "NO" }
            ));
            statusbar_clone3.set_status_text(
                &format!(
                    "Features: {}",
                    if is_checked { "Enabled" } else { "Disabled" }
                ),
                0,
            );
        });

        // Slider
        let status_clone4 = status_label.clone();
        let gauge_clone = progress_gauge.clone();
        let statusbar_clone4 = statusbar.clone();
        value_slider.on_scroll_changed(move |event_data| {
            let value = event_data.get_position().unwrap_or(0);
            println!("Slider value: {}", value);
            status_clone4.set_label(&format!("Slider: {}", value));
            gauge_clone.set_value(value);
            statusbar_clone4.set_status_text(&format!("Value: {}", value), 0);
        });

        // Spin control
        let status_clone5 = status_label.clone();
        number_spin.on_value_changed(move |event_data| {
            let value = event_data.get_value();
            println!("Spin control value: {}", value);
            status_clone5.set_label(&format!("Number: {}", value));
        });

        // Text controls
        let status_clone6 = status_label.clone();
        input_text.on_text_updated(move |_event_data| {
            println!("Input text updated");
            status_clone6.set_label("Input text updated");
        });

        // Search control
        let status_clone7 = status_label.clone();
        search_ctrl.on_search_button_clicked(move |_event_data| {
            println!("Search button clicked");
            status_clone7.set_label("Search performed");
        });

        // Radio buttons
        let status_clone8 = status_label.clone();
        option1_radio.on_selected(move |_event_data| {
            println!("Option 1 selected");
            status_clone8.set_label("Option 1 selected");
        });

        let status_clone9 = status_label.clone();
        option2_radio.on_selected(move |_event_data| {
            println!("Option 2 selected");
            status_clone9.set_label("Option 2 selected");
        });

        // Choice combo
        let status_clone10 = status_label.clone();
        choice_combo.on_selection_changed(move |event_data| {
            let selection = event_data.get_selection().unwrap_or(0);
            println!("Choice selected: {}", selection);
            status_clone10.set_label(&format!("Choice: {}", selection));
        });

        // List box
        let status_clone11 = status_label.clone();
        items_list.on_selection_changed(move |event_data| {
            let selection = event_data.get_selection().unwrap_or(0);
            println!("List item selected: {}", selection);
            status_clone11.set_label(&format!("Item: {}", selection));
        });

        // Set up dialog button handlers
        let dialog_ok = &dialog_ui.dialog_ok_btn;
        let dialog_cancel = &dialog_ui.dialog_cancel_btn;

        let _dialog_clone_ok = dialog_ui.test_dialog.clone();
        dialog_ok.on_click(move |_event_data| {
            println!("Dialog OK clicked");
            _dialog_clone_ok.end_modal(ID_OK);
        });

        let _dialog_clone_cancel = dialog_ui.test_dialog.clone();
        dialog_cancel.on_click(move |_event_data| {
            println!("Dialog Cancel clicked");
            _dialog_clone_cancel.end_modal(ID_CANCEL);
        });

        // Set some initial values to demonstrate the controls
        input_text.set_value("Welcome to wxDragon XRC!");

        multiline_text.set_value("This is a comprehensive\nXRC demonstration with:\n• Multiple XRC files\n• Embedded panels\n• Modal dialogs\n• Various controls");

        search_ctrl.set_value("Search widgets...");

        // Update the frame title to show it's using XRC
        frame_ui
            .main_frame
            .set_title("wxDragon XRC Multi-File Demo");

        let title_label = &frame_ui.title_label;
        title_label.set_label("wxDragon XRC Multi-File Demo");

        // Show the main frame
        frame_ui.main_frame.center_on_screen();
        frame_ui.main_frame.show(true);

        // Force layout after frame is shown to ensure proper sizing with toolbar/statusbar
        frame_ui.main_frame.layout();

        // Additional layout calls to ensure content panel gets proper size
        content_panel.layout();
        panel_ui.widget_panel.layout();

        // Debug: Print final sizes after all layout calls
        println!("🔧 Debug final sizes after frame shown and all layouts:");
        let frame_size = frame_ui.main_frame.get_size();
        println!("  frame size: {}x{}", frame_size.width, frame_size.height);
        let main_panel_size = frame_ui.main_panel.get_size();
        println!(
            "  main_panel size: {}x{}",
            main_panel_size.width, main_panel_size.height
        );
        let content_size = content_panel.get_size();
        println!(
            "  content_panel size: {}x{}",
            content_size.width, content_size.height
        );
        let widget_size = panel_ui.widget_panel.get_size();
        println!(
            "  widget_panel size: {}x{}",
            widget_size.width, widget_size.height
        );

        println!("✓ XRC Example application loaded successfully!");
        println!("📋 Widget showcase:");
        println!("• Text controls: TextCtrl, multi-line TextCtrl, SearchCtrl");
        println!("• Selection controls: CheckBox, RadioButton, ComboBox, ListBox");
        println!("• Action controls: Button, ToggleButton, Slider, Gauge, SpinCtrl");
        println!(
            "• Click 'Show Dialog' to test dialog functionality with working OK/Cancel buttons"
        );
        println!("• All widgets have event handlers that print to console");
        println!();
        println!("Ready! Try interacting with the widgets...");
    });
}
