//! Comprehensive demonstration of menu events in wxDragon
//!
//! This example demonstrates:
//! - wxEVT_MENU_OPEN: Menu opened events
//! - wxEVT_MENU_CLOSE: Menu closed events
//! - wxEVT_MENU_HIGHLIGHT: Menu item highlighted events
//! - wxEVT_CONTEXT_MENU: Context menu requested events
//! - wxEVT_MENU: Traditional menu selection events

use std::rc::Rc;
use wxdragon::prelude::*;

const ID_NEW: i32 = 1001;
const ID_OPEN: i32 = 1002;
const ID_SAVE: i32 = 1003;
const ID_EXIT: i32 = 1004;
const ID_ABOUT: i32 = 1005;
const ID_CUT: i32 = 2001;
const ID_COPY: i32 = 2002;
const ID_PASTE: i32 = 2003;

struct MenuEventsApp {
    frame: Frame,
    status_bar: StatusBar,
    menu_open_count: std::cell::RefCell<i32>,
}

impl MenuEventsApp {
    fn new() -> Self {
        // Create main frame
        let frame = Frame::builder()
            .with_title("Menu Events Demo")
            .with_size(Size::new(800, 600))
            .with_position(Point::new(100, 100))
            .build();

        // Create status bar with multiple fields
        let status_bar = StatusBar::builder(&frame)
            .with_fields_count(3)
            .with_status_widths(vec![-1, 200, 150])
            .add_initial_text(0, "Ready - Right-click for context menu")
            .add_initial_text(1, "Menu Status: Closed")
            .add_initial_text(2, "Opens: 0")
            .build();

        frame.set_existing_status_bar(Some(&status_bar));

        Self {
            frame,
            status_bar,
            menu_open_count: std::cell::RefCell::new(0),
        }
    }

    fn setup_menu(&self) {
        // File menu
        let file_menu = Menu::builder()
            .append_item(ID_NEW, "&New\tCtrl+N", "Create a new document")
            .append_item(ID_OPEN, "&Open\tCtrl+O", "Open an existing document")
            .append_item(ID_SAVE, "&Save\tCtrl+S", "Save the current document")
            .append_separator()
            .append_item(ID_EXIT, "E&xit\tAlt+F4", "Exit the application")
            .build();

        // Edit menu
        let edit_menu = Menu::builder()
            .append_item(ID_CUT, "Cu&t\tCtrl+X", "Cut selected text")
            .append_item(ID_COPY, "&Copy\tCtrl+C", "Copy selected text")
            .append_item(ID_PASTE, "&Paste\tCtrl+V", "Paste from clipboard")
            .build();

        // Help menu
        let help_menu = Menu::builder()
            .append_item(ID_ABOUT, "&About", "About this application")
            .build();

        let file_menu2 = Menu::builder()
            .append_item(ID_NEW, "&New\tCtrl+N", "Create a new document")
            .append_item(ID_OPEN, "&Open\tCtrl+O", "Open an existing document")
            .append_item(ID_SAVE, "&Save\tCtrl+S", "Save the current document")
            .append_separator()
            .append_item(ID_EXIT, "E&xit\tAlt+F4", "Exit the application")
            .build();
        edit_menu.append_submenu(file_menu2, "&File", "File operations");

        // Create and set menu bar
        let menu_bar = MenuBar::builder()
            .append(file_menu, "&File")
            .append(edit_menu, "&Edit")
            .append(help_menu, "&Help")
            .build();

        self.frame.set_menu_bar(menu_bar);
    }

    fn setup_menu_events(&self) {
        let status_bar = self.status_bar;
        let menu_count = self.menu_open_count.clone();
        let frame = self.frame;

        // Menu opened events with full functionality
        self.frame.on_menu_opened(move |event: MenuEventData| {
            let mut count = menu_count.borrow_mut();
            *count += 1;

            let menu_info = if event.is_popup() {
                "Menu Status: Popup Opened"
            } else {
                "Menu Status: Menu Bar Opened"
            };

            status_bar.set_status_text(menu_info, 1);
            status_bar.set_status_text(&format!("Opens: {}", *count), 2);

            log::trace!("📂 {}", event.format_for_logging());
        });

        // Menu closed events
        self.frame.on_menu_closed(move |event: MenuEventData| {
            let menu_info = if event.is_popup() {
                "Menu Status: Popup Closed"
            } else {
                "Menu Status: Menu Bar Closed"
            };

            status_bar.set_status_text(menu_info, 1);

            log::trace!("📁 {}", event.format_for_logging());
        });

        // Menu highlight events (for status bar help text)
        self.frame.on_menu_highlighted(move |event: MenuEventData| {
            let help_text = match event.get_id() {
                ID_NEW => "Create a new document",
                ID_OPEN => "Open an existing document",
                ID_SAVE => "Save the current document",
                ID_EXIT => "Exit the application",
                ID_CUT => "Cut selected text to clipboard",
                ID_COPY => "Copy selected text to clipboard",
                ID_PASTE => "Paste text from clipboard",
                ID_ABOUT => "Show application information",
                _ => "Ready - Right-click for context menu",
            };

            status_bar.set_status_text(help_text, 0);

            log::trace!("✨ Menu Highlighted - ID: {}, Help: {}", event.get_id(), help_text);
        });

        // Traditional menu selection events
        self.frame.on_menu_selected(move |event: MenuEventData| {
            match event.get_id() {
                ID_NEW => log::trace!("🆕 New document requested"),
                ID_OPEN => log::trace!("📂 Open document requested"),
                ID_SAVE => log::trace!("💾 Save document requested"),
                ID_EXIT => {
                    log::trace!("👋 Exit requested");
                    frame.close(true);
                }
                ID_CUT => log::trace!("✂️ Cut requested"),
                ID_COPY => log::trace!("📋 Copy requested"),
                ID_PASTE => log::trace!("📋 Paste requested"),
                ID_ABOUT => {
                    log::trace!("ℹ️ About dialog should be shown");
                    // In a real app, you'd show an About dialog here
                }
                _ => log::warn!("❓ Unknown menu item selected: {}", event.get_id()),
            }

            log::trace!("🎯 {}", event.format_for_logging());
        });
    }

    fn setup_context_menu(&self) {
        // Create main panel to handle context menu events
        let panel = Panel::builder(&self.frame).build();

        // Context menu event handling - now test the fixed FFI functions
        panel.on_context_menu(move |event: MenuEventData| {
            log::trace!("🖱️ Context menu event received!");
            log::trace!("   Event ID: {}", event.get_id());
            log::trace!("   Event type: {}", event.get_event_type_name());

            // Test the context position accessor
            if let Some(pos) = event.get_context_position() {
                log::trace!("   Position: ({}, {})", pos.x, pos.y);
            } else {
                log::trace!("   Position: Not available");
            }

            // Test the formatting function
            log::trace!("   Formatted: {}", event.format_for_logging());

            let view_id = 3001;
            let delete_id = 3002;
            let mut popup_menu = Menu::builder()
                .append_item(view_id, "View", "View item")
                .append_item(delete_id, "Delete", "Delete item")
                .build();

            let pos = event.get_context_position();
            panel.popup_menu(&mut popup_menu, pos);
        });

        // Set the panel as the frame's main child
        // In a real app, you'd set up proper sizers here
    }

    fn setup_frame_events(&self) {
        // Handle frame close event
        let frame = self.frame;
        self.frame.on_close(move |_| {
            log::trace!("🚪 Application closing...");
            frame.destroy();
        });

        // Track menu lifecycle for debugging
        self.frame.track_menu_lifecycle(|event_type, is_opening| {
            let action = if is_opening { "🔄 Opening" } else { "🔄 Closing" };
            log::trace!("{} menu lifecycle event: {}", action, event_type);
        });

        self.frame.on_menu_opened(|event: MenuEventData| {
            log::info!("✅ Menu opened event tracked: {}", event.format_for_logging());
        });
    }

    fn setup_dataview(&self) {
        // Create DataViewCtrl
        let dataview = DataViewCtrl::builder(&self.frame)
            .with_pos(Point::new(10, 10))
            .with_size(Size::new(400, 200))
            .with_style(DataViewStyle::Multiple | DataViewStyle::RowLines)
            .build();

        // Create data model
        let model = DataViewListModel::new();
        model.append_column("Name");
        model.append_column("Age");
        model.append_column("City");
        model.append_column("Status");

        let demo_data = Rc::new(vec![
            ("Alice", 23, "Beijing", "Active"),
            ("Bob", 31, "Shanghai", "Offline"),
            ("Carol", 27, "Guangzhou", "Active"),
        ]);
        for (row, (name, age, city, status)) in demo_data.iter().enumerate() {
            model.append_row();
            model.set_value(row, 0, Variant::from_string(name));
            model.set_value(row, 1, Variant::from_i32(*age));
            model.set_value(row, 2, Variant::from_string(city));
            model.set_value(row, 3, Variant::from_string(status));
        }

        // Create columns
        let name_col = DataViewColumn::new(
            "Name",
            &DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, DataViewAlign::Left),
            0,
            100,
            DataViewAlign::Left,
            DataViewColumnFlags::Resizable,
        );
        let age_col = DataViewColumn::new(
            "Age",
            &DataViewTextRenderer::new(VariantType::Int32, DataViewCellMode::Inert, DataViewAlign::Center),
            1,
            60,
            DataViewAlign::Center,
            DataViewColumnFlags::Resizable,
        );
        let city_col = DataViewColumn::new(
            "City",
            &DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, DataViewAlign::Left),
            2,
            100,
            DataViewAlign::Left,
            DataViewColumnFlags::Resizable,
        );
        let status_col = DataViewColumn::new(
            "Status",
            &DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, DataViewAlign::Center),
            3,
            80,
            DataViewAlign::Center,
            DataViewColumnFlags::Resizable,
        );

        dataview.append_column(&name_col);
        dataview.append_column(&age_col);
        dataview.append_column(&city_col);
        dataview.append_column(&status_col);
        dataview.associate_model(&model);

        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        sizer.add_stretch_spacer(1);
        let flag = SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical;
        sizer.add(&dataview, 0, flag, 0);
        sizer.add_stretch_spacer(1);
        self.frame.set_sizer(sizer, true);
        let demo_data_clone = demo_data.clone();
        let frame = self.frame;
        dataview.on_item_context_menu(move |event: DataViewEvent| {
            log::trace!("🖱️ Context menu event received!");
            log::trace!("   Event ID: {}", event.get_id());
            let Some(item) = event.get_item() else {
                log::trace!("   No item associated with event");
                return;
            };

            let Some(row) = event.get_row() else {
                log::trace!("   No row associated with event");
                return;
            };
            log::trace!("   Item ID Pointer: {:?}", item);
            log::trace!("   Row Index: {}", row);

            if let Some(col) = event.get_column() {
                log::trace!("  Column: {}", col);
            }

            let view_id = 3001;
            let delete_id = 3002;
            let mut popup_menu = Menu::builder()
                .append_item(view_id, "View", "View item")
                .append_item(delete_id, "Delete", "Delete item")
                .build();

            // Handle popup menu selections locally so we can use the captured row
            let demo_for_handler = demo_data_clone.clone();
            popup_menu.on_selected(move |menu_evt| {
                let id = menu_evt.get_id();
                if id == view_id {
                    // Show dialog with details for this row
                    let r = row as usize;
                    if r < demo_for_handler.len() {
                        let (n, age, city, status) = demo_for_handler[r];
                        let msg = format!("Name: {n}, Age: {age}, City: {city}, Status: {status}");
                        use MessageDialogStyle as MDS;
                        MessageDialog::builder(&frame, &msg, "Item Details")
                            .with_style(MDS::OK | MDS::IconInformation)
                            .build()
                            .show_modal();
                    }
                } else if id == delete_id {
                    log::trace!("Delete requested for row {}", row);
                    // In a real app, you'd remove the row from the model here
                }
            });

            let pos = event.get_position().map(|p| dataview.client_to_screen(p));
            dataview.popup_menu(&mut popup_menu, pos);

            // Clean up the popup menu after use to release rust closures attached to menu items
            popup_menu.destroy_menu();
        });
    }

    fn run(&self) {
        self.setup_menu();
        self.setup_menu_events();
        self.setup_context_menu();
        self.setup_dataview();
        self.setup_frame_events();

        self.frame.show(true);

        log::trace!("🚀 Menu Events Demo Started!");
        log::trace!("📋 Instructions:");
        log::trace!("   • Click on menu items in the menu bar");
        log::trace!("   • Hover over menu items to see highlight events");
        log::trace!("   • Right-click anywhere in the window for context menu");
        log::trace!("   • Watch the console and status bar for event information");
        log::trace!("   • Close the window to exit");
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let _ = wxdragon::main(|_a| {
        let app = MenuEventsApp::new();
        app.run();
    });
}
