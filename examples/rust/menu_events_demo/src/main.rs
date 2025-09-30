//! Comprehensive demonstration of menu events in wxDragon
//!
//! This example demonstrates:
//! - wxEVT_MENU_OPEN: Menu opened events
//! - wxEVT_MENU_CLOSE: Menu closed events
//! - wxEVT_MENU_HIGHLIGHT: Menu item highlighted events
//! - wxEVT_CONTEXT_MENU: Context menu requested events
//! - wxEVT_MENU: Traditional menu selection events

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

        // Create and set menu bar
        let menu_bar = MenuBar::builder()
            .append(file_menu, "&File")
            .append(edit_menu, "&Edit")
            .append(help_menu, "&Help")
            .build();

        self.frame.set_menu_bar(menu_bar);
    }

    fn setup_menu_events(&self) {
        let status_bar = self.status_bar.clone();
        let menu_count = self.menu_open_count.clone();

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

            println!("📂 {}", event.format_for_logging());
        });

        let status_bar_close = self.status_bar.clone();

        // Menu closed events
        self.frame.on_menu_closed(move |event: MenuEventData| {
            let menu_info = if event.is_popup() {
                "Menu Status: Popup Closed"
            } else {
                "Menu Status: Menu Bar Closed"
            };

            status_bar_close.set_status_text(menu_info, 1);

            println!("📁 {}", event.format_for_logging());
        });

        // Re-enable other menu event handlers now that crash is fixed
        let status_bar_highlight = self.status_bar.clone();

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

            status_bar_highlight.set_status_text(help_text, 0);

            println!(
                "✨ Menu Highlighted - ID: {}, Help: {}",
                event.get_id(),
                help_text
            );
        });

        // Traditional menu selection events
        self.frame.on_menu_selected(move |event: MenuEventData| {
            match event.get_id() {
                ID_NEW => println!("🆕 New document requested"),
                ID_OPEN => println!("📂 Open document requested"),
                ID_SAVE => println!("💾 Save document requested"),
                ID_EXIT => {
                    println!("👋 Exit requested");
                    // Note: In a real app, you'd call frame.close(false) here
                }
                ID_CUT => println!("✂️ Cut requested"),
                ID_COPY => println!("📋 Copy requested"),
                ID_PASTE => println!("📋 Paste requested"),
                ID_ABOUT => {
                    println!("ℹ️ About dialog should be shown");
                    // In a real app, you'd show an About dialog here
                }
                _ => println!("❓ Unknown menu item selected: {}", event.get_id()),
            }

            println!("🎯 {}", event.format_for_logging());
        });
    }

    fn setup_context_menu(&self) {
        // Create main panel to handle context menu events
        let panel = Panel::builder(&self.frame).build();

        // Context menu event handling - now test the fixed FFI functions
        let panel_clone = panel.clone();
        panel.on_context_menu(move |event: MenuEventData| {
            println!("🖱️ Context menu event received!");
            println!("   Event ID: {}", event.get_id());
            println!("   Event type: {}", event.get_event_type_name());

            // Test the context position accessor
            if let Some(pos) = event.get_context_position() {
                println!("   Position: ({}, {})", pos.x, pos.y);
            } else {
                println!("   Position: Not available");
            }

            // Test the formatting function
            println!("   Formatted: {}", event.format_for_logging());

            let view_id = 3001;
            let delete_id = 3002;
            let popup_menu = Menu::builder()
                .append_item(view_id, "View", "View item")
                .append_item(delete_id, "Delete", "Delete item")
                .build();

            let pos = event.get_context_position();
            panel_clone.popup_menu(&popup_menu, pos);
        });

        // Set the panel as the frame's main child
        // In a real app, you'd set up proper sizers here
    }

    fn setup_frame_events(&self) {
        // Handle frame close event
        let frame_clone = self.frame.clone();
        self.frame.on_close(move |_| {
            println!("🚪 Application closing...");
            frame_clone.destroy();
        });

        // Track menu lifecycle for debugging
        self.frame.track_menu_lifecycle(|event_type, is_opening| {
            let action = if is_opening {
                "🔄 Opening"
            } else {
                "🔄 Closing"
            };
            println!("{} menu lifecycle event: {}", action, event_type);
        });
    }

    fn run(&self) {
        self.setup_menu();
        self.setup_menu_events();
        self.setup_context_menu();
        self.setup_frame_events();

        self.frame.show(true);

        println!("🚀 Menu Events Demo Started!");
        println!("📋 Instructions:");
        println!("   • Click on menu items in the menu bar");
        println!("   • Hover over menu items to see highlight events");
        println!("   • Right-click anywhere in the window for context menu");
        println!("   • Watch the console and status bar for event information");
        println!("   • Close the window to exit");
        println!();
    }
}

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let _ = wxdragon::main(|_| {
        let app = MenuEventsApp::new();
        app.run();
    });
}
