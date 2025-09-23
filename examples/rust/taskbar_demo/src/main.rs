use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        // Create a main frame to keep the app running
        let frame = Frame::builder()
            .with_title("TaskBar Demo - wxDragon")
            .with_size(Size::new(500, 300))
            .with_position(Point::new(100, 100))
            .build();

        // Add content to the frame
        let panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        let title = StaticText::builder(&panel)
            .with_label("TaskBar Demo - wxDragon")
            .build();

        let info = StaticText::builder(&panel)
            .with_label("This application demonstrates TaskBarIcon functionality.")
            .build();

        let status = StaticText::builder(&panel)
            .with_label("Creating TaskBarIcon...")
            .build();

        sizer.add(&title, 0, SizerFlag::All | SizerFlag::Expand, 10);
        sizer.add(&info, 0, SizerFlag::All | SizerFlag::Expand, 10);
        sizer.add(&status, 0, SizerFlag::All | SizerFlag::Expand, 10);
        panel.set_sizer(sizer, true);

        // Create popup menu for taskbar icon
        let popup_menu = Menu::builder()
            .append_item(1001, "Open Application", "Open the main application window")
            .append_separator()
            .append_item(1002, "Settings", "Open application settings")
            .append_item(1003, "About", "About this application")
            .append_separator()
            .append_item(1004, "Exit", "Exit the application")
            .build();

        // Menu item IDs are defined inline in the event handler

        // Create a taskbar icon
        let taskbar = TaskBarIcon::builder()
            .with_icon_type(TaskBarIconType::CustomStatusItem)
            .build();

        // Set the popup menu to show automatically when the taskbar icon is clicked
        taskbar.set_popup_menu(&popup_menu);

        // Bind menu event handler to the TaskBarIcon itself (not the frame)
        taskbar.on_menu({
            let status = status.clone();
            let frame = frame.clone();
            move |event| {
                let menu_id = event.get_id();
                match menu_id {
                    1001 => {
                        // Open Application
                        println!("📂 Open Application clicked!");
                        status.set_label("Menu: Open Application clicked!");
                    }
                    1002 => {
                        // Settings
                        println!("⚙️ Settings clicked!");
                        status.set_label("Menu: Settings clicked!");
                    }
                    1003 => {
                        // About
                        println!("ℹ️ About clicked!");
                        status.set_label("Menu: About clicked!");
                    }
                    1004 => {
                        // Exit
                        println!("🚪 Exit clicked!");
                        status.set_label("Menu: Exit clicked - closing application...");

                        // Close the frame, which will trigger the on_close event
                        frame.close(true);
                    }
                    _ => {
                        println!("Unknown menu item clicked: {menu_id}");
                    }
                }
            }
        });

        // Create system icon
        let icon =
            ArtProvider::get_bitmap(ArtId::Warning, ArtClient::Menu, Some(Size::new(16, 16)));

        if let Some(icon) = icon {
            let success = taskbar.set_icon(&icon, "TaskBar Demo - wxDragon");

            if success && taskbar.is_icon_installed() {
                status.set_label("SUCCESS: TaskBarIcon visible in system tray!");
            } else {
                status.set_label("FAILED: Could not set taskbar icon");
            }
        } else {
            status.set_label("ERROR: Could not create system icon bitmap");

            // Try fallback bitmap
            if let Some(fallback) = Bitmap::new(16, 16) {
                let success = taskbar.set_icon(&fallback, "TaskBar Demo - Fallback");
                if success {
                    status.set_label("Fallback bitmap set successfully");
                }
            }
        }

        status.set_label("TaskBar icon created. Click the system tray icon to see the popup menu!");

        // Show the frame
        frame.show(true);
        frame.centre();

        let frame_clone = frame.clone();
        frame.on_close(move |evt| {
            if let wxdragon::WindowEventData::General(event) = &evt
                && event.can_veto()
            {
                use MessageDialogStyle as MDS;
                let res = MessageDialog::builder(
                    &frame_clone,
                    "Are you sure you want to close the application?",
                    "Confirm Close",
                )
                .with_style(MDS::OK | MDS::Cancel | MDS::IconInformation)
                .build()
                .show_modal();

                if res != wxdragon::ID_OK {
                    // User cancelled (clicked No or Cancel), prevent the close
                    event.veto();
                }
            }
        });

        frame.on_destroy(move |_evt| {
            taskbar.destroy(); // Clean up the TaskBarIcon
            println!("Application on_destroy.");
        });
    });
}
