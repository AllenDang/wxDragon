//! Config Demo
//!
//! This demo showcases the wxConfig functionality for storing and retrieving
//! application configuration data.
//!
//! Features demonstrated:
//! - Creating a config object
//! - Reading and writing string, long, double, and bool values
//! - Using paths/groups to organize settings
//! - Enumerating entries and groups
//! - Deleting entries and groups

use wxdragon::prelude::*;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Config Demo")
            .with_size(Size::new(600, 500))
            .build();

        let panel = Panel::builder(&frame).build();
        let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Title
        let title = StaticText::builder(&panel).with_label("wxConfig Demo").build();

        // Create config object
        let config = Config::new("ConfigDemo", Some("wxDragon"), None, None, ConfigStyle::empty());

        // Info display
        let info_text = TextCtrl::builder(&panel)
            .with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly)
            .with_size(Size::new(-1, 200))
            .build();

        // Key/Value input
        let input_sizer = FlexGridSizer::builder(2, 2).with_gap(Size::new(5, 5)).build();

        let key_label = StaticText::builder(&panel).with_label("Key:").build();
        let key_input = TextCtrl::builder(&panel).build();
        key_input.set_value("TestKey");

        let value_label = StaticText::builder(&panel).with_label("Value:").build();
        let value_input = TextCtrl::builder(&panel).build();
        value_input.set_value("TestValue");

        input_sizer.add(&key_label, 0, SizerFlag::AlignCenterVertical, 0);
        input_sizer.add(&key_input, 1, SizerFlag::Expand, 0);
        input_sizer.add(&value_label, 0, SizerFlag::AlignCenterVertical, 0);
        input_sizer.add(&value_input, 1, SizerFlag::Expand, 0);
        input_sizer.add_growable_col(1, 1);

        // Button panel
        let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();

        let btn_write_string = Button::builder(&panel).with_label("Write String").build();
        let btn_write_long = Button::builder(&panel).with_label("Write Long").build();
        let btn_write_bool = Button::builder(&panel).with_label("Write Bool").build();
        let btn_read = Button::builder(&panel).with_label("Read").build();
        let btn_delete = Button::builder(&panel).with_label("Delete").build();
        let btn_list = Button::builder(&panel).with_label("List All").build();

        // Path controls
        let path_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        let path_label = StaticText::builder(&panel).with_label("Path:").build();
        let path_input = TextCtrl::builder(&panel).build();
        path_input.set_value("/");
        let btn_set_path = Button::builder(&panel).with_label("Set Path").build();

        path_sizer.add(&path_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
        path_sizer.add(&path_input, 1, SizerFlag::Expand, 0);
        path_sizer.add(&btn_set_path, 0, SizerFlag::Left, 5);

        // Status label
        let status_label = StaticText::builder(&panel).with_label("Status: Ready").build();

        // Initialize display
        {
            let mut info = String::new();
            info.push_str(&format!("App Name: {}\n", config.get_app_name()));
            info.push_str(&format!("Vendor Name: {}\n", config.get_vendor_name()));
            info.push_str(&format!("Current Path: {}\n", config.get_path()));
            info.push_str(&format!("Expanding Env Vars: {}\n", config.is_expanding_env_vars()));
            info.push_str("\nUse the buttons below to interact with the config.\n");
            info_text.set_value(&info);
        }

        // Event handlers
        {
            let config_clone = std::rc::Rc::new(config);

            // Write String
            {
                let config = config_clone.clone();
                btn_write_string.on_click(move |_| {
                    let key = key_input.get_value();
                    let value = value_input.get_value();
                    if config.write_string(&key, &value) {
                        config.flush(false);
                        status_label.set_label(&format!("Status: Wrote '{}' = '{}'", key, value));
                        log::info!("Wrote string: {} = {}", key, value);
                    } else {
                        status_label.set_label("Status: Write failed");
                        log::error!("Failed to write string");
                    }
                });
            }

            // Write Long
            {
                let config = config_clone.clone();
                btn_write_long.on_click(move |_| {
                    let key = key_input.get_value();
                    let value_str = value_input.get_value();
                    if let Ok(value) = value_str.parse::<i64>() {
                        if config.write_long(&key, value) {
                            config.flush(false);
                            status_label.set_label(&format!("Status: Wrote '{}' = {} (long)", key, value));
                            log::info!("Wrote long: {} = {}", key, value);
                        } else {
                            status_label.set_label("Status: Write failed");
                        }
                    } else {
                        status_label.set_label("Status: Invalid number");
                    }
                });
            }

            // Write Bool
            {
                let config = config_clone.clone();
                btn_write_bool.on_click(move |_| {
                    let key = key_input.get_value();
                    let value_str = value_input.get_value().to_lowercase();
                    let value = value_str == "true" || value_str == "1" || value_str == "yes";
                    if config.write_bool(&key, value) {
                        config.flush(false);
                        status_label.set_label(&format!("Status: Wrote '{}' = {} (bool)", key, value));
                        log::info!("Wrote bool: {} = {}", key, value);
                    } else {
                        status_label.set_label("Status: Write failed");
                    }
                });
            }

            // Read
            {
                let config = config_clone.clone();
                btn_read.on_click(move |_| {
                    let key = key_input.get_value();
                    if config.has_entry(&key) {
                        let entry_type = config.get_entry_type(&key);
                        let value = match entry_type {
                            ConfigEntryType::Boolean => {
                                format!("{} (bool)", config.read_bool(&key, false))
                            }
                            ConfigEntryType::Integer => {
                                format!("{} (long)", config.read_long(&key, 0))
                            }
                            ConfigEntryType::Float => {
                                format!("{} (double)", config.read_double(&key, 0.0))
                            }
                            _ => {
                                format!("\"{}\" (string)", config.read_string(&key, ""))
                            }
                        };
                        let current = info_text.get_value();
                        info_text.set_value(&format!("{}\nRead '{}' = {}", current, key, value));
                        status_label.set_label(&format!("Status: Read '{}' = {}", key, value));
                        log::info!("Read: {} = {}", key, value);
                    } else {
                        status_label.set_label(&format!("Status: Key '{}' not found", key));
                        log::warn!("Key not found: {}", key);
                    }
                });
            }

            // Delete
            {
                let config = config_clone.clone();
                btn_delete.on_click(move |_| {
                    let key = key_input.get_value();
                    if config.delete_entry(&key, true) {
                        config.flush(false);
                        status_label.set_label(&format!("Status: Deleted '{}'", key));
                        log::info!("Deleted: {}", key);
                    } else {
                        status_label.set_label(&format!("Status: Failed to delete '{}'", key));
                        log::error!("Failed to delete: {}", key);
                    }
                });
            }

            // List All
            {
                let config = config_clone.clone();
                btn_list.on_click(move |_| {
                    let mut info = String::new();
                    info.push_str(&format!("Current Path: {}\n", config.get_path()));
                    info.push_str(&format!("Number of entries: {}\n", config.get_number_of_entries(false)));
                    info.push_str(&format!("Number of groups: {}\n", config.get_number_of_groups(false)));
                    info.push_str("\nEntries:\n");

                    for entry in config.get_entries() {
                        let value = config.read_string(&entry, "<error>");
                        info.push_str(&format!("  {} = \"{}\"\n", entry, value));
                    }

                    info.push_str("\nGroups:\n");
                    for group in config.get_groups() {
                        info.push_str(&format!("  [{}]\n", group));
                    }

                    info_text.set_value(&info);
                    status_label.set_label("Status: Listed all entries and groups");
                    log::info!("Listed config contents");
                });
            }

            // Set Path
            {
                let config = config_clone.clone();
                btn_set_path.on_click(move |_| {
                    let path = path_input.get_value();
                    config.set_path(&path);
                    let actual_path = config.get_path();
                    status_label.set_label(&format!("Status: Path set to '{}'", actual_path));
                    log::info!("Set path to: {}", actual_path);
                });
            }
        }

        // Layout
        main_sizer.add(&title, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);
        main_sizer.add(&info_text, 1, SizerFlag::Expand | SizerFlag::All, 10);
        main_sizer.add_sizer(&input_sizer, 0, SizerFlag::Expand | SizerFlag::All, 10);

        button_sizer.add(&btn_write_string, 0, SizerFlag::All, 3);
        button_sizer.add(&btn_write_long, 0, SizerFlag::All, 3);
        button_sizer.add(&btn_write_bool, 0, SizerFlag::All, 3);
        button_sizer.add(&btn_read, 0, SizerFlag::All, 3);
        button_sizer.add(&btn_delete, 0, SizerFlag::All, 3);
        button_sizer.add(&btn_list, 0, SizerFlag::All, 3);

        main_sizer.add_sizer(&button_sizer, 0, SizerFlag::AlignCenterHorizontal, 0);
        main_sizer.add_sizer(&path_sizer, 0, SizerFlag::Expand | SizerFlag::All, 10);
        main_sizer.add(&status_label, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);

        panel.set_sizer(main_sizer, true);

        frame.show(true);
        frame.centre();
    });
}
