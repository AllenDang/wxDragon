//! UIActionSimulator Demo
//!
//! This demo showcases the wxUIActionSimulator functionality, which allows
//! simulating user interface actions such as mouse clicks and key presses.
//!
//! Features demonstrated:
//! - Mouse movement and clicks
//! - Keyboard input simulation
//! - Text typing simulation
//! - Drag and drop simulation

use wxdragon::prelude::*;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("UIActionSimulator Demo")
            .with_size(Size::new(500, 400))
            .build();

        let panel = Panel::builder(&frame).build();
        let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Title
        let title = StaticText::builder(&panel).with_label("UIActionSimulator Demo").build();

        // Instructions
        let instructions = StaticText::builder(&panel)
            .with_label("Click buttons below to simulate UI actions.\nWatch the text control and log output.")
            .build();

        // Text control for demonstrating typing
        let text_ctrl = TextCtrl::builder(&panel)
            .with_style(TextCtrlStyle::MultiLine)
            .with_size(Size::new(-1, 100))
            .build();

        // Button panel
        let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();

        let btn_type_text = Button::builder(&panel).with_label("Type Text").build();

        let btn_click_demo = Button::builder(&panel).with_label("Click Demo").build();

        let btn_key_combo = Button::builder(&panel).with_label("Ctrl+A").build();

        let btn_mouse_move = Button::builder(&panel).with_label("Move Mouse").build();

        // Status display
        let status_label = StaticText::builder(&panel).with_label("Status: Ready").build();

        // Click counter for demo
        let click_counter = Button::builder(&panel).with_label("Click Counter: 0").build();

        // Set up event handlers
        {
            let text_ctrl_clone = text_ctrl.clone();
            let status_label_clone = status_label.clone();
            btn_type_text.on_click(move |_| {
                log::info!("Simulating text typing...");
                status_label_clone.set_label("Status: Typing text...");

                // Focus the text control first
                text_ctrl_clone.set_focus();

                // Create simulator and type text
                let sim = UIActionSimulator::new();
                if sim.is_ok() {
                    // Type some text
                    if sim.text("Hello from UIActionSimulator! ") {
                        log::info!("Text typing simulation successful");
                        status_label_clone.set_label("Status: Text typed successfully!");
                    } else {
                        log::warn!("Text typing simulation failed");
                        status_label_clone.set_label("Status: Text typing failed");
                    }
                } else {
                    log::error!("Failed to create UIActionSimulator");
                    status_label_clone.set_label("Status: Simulator not available");
                }
            });
        }

        {
            let click_counter_clone = click_counter.clone();
            let click_counter_for_demo = click_counter.clone();
            let status_label_clone = status_label.clone();
            let counter = std::cell::RefCell::new(0);

            click_counter.on_click(move |_| {
                let mut count = counter.borrow_mut();
                *count += 1;
                click_counter_clone.set_label(&format!("Click Counter: {}", *count));
                log::info!("Counter clicked! Count: {}", *count);
            });

            btn_click_demo.on_click(move |_| {
                log::info!("Demonstrating mouse click simulation...");
                status_label_clone.set_label("Status: Click simulation (check counter)");

                let sim = UIActionSimulator::new();
                if sim.is_ok() {
                    // Get the click counter button's position and size
                    let btn_pos = click_counter_for_demo.get_position();
                    let btn_size = click_counter_for_demo.get_size();

                    // Calculate center of button in client coordinates
                    let center_x = btn_pos.x + btn_size.width / 2;
                    let center_y = btn_pos.y + btn_size.height / 2;

                    // Convert to screen coordinates
                    let screen_pos = click_counter_for_demo.client_to_screen(Point::new(btn_size.width / 2, btn_size.height / 2));

                    log::info!(
                        "Click counter button: pos=({}, {}), size=({}, {}), center=({}, {}), screen=({}, {})",
                        btn_pos.x, btn_pos.y, btn_size.width, btn_size.height,
                        center_x, center_y, screen_pos.x, screen_pos.y
                    );

                    // Move mouse to the center of the click counter button and click
                    if sim.mouse_move(screen_pos.x, screen_pos.y) {
                        log::info!("Mouse moved to click counter button");
                        if sim.mouse_click(MouseButton::Left) {
                            log::info!("Mouse click simulated on click counter!");
                            status_label_clone.set_label("Status: Clicked the counter button!");
                        } else {
                            log::warn!("Mouse click failed");
                            status_label_clone.set_label("Status: Click failed");
                        }
                    } else {
                        log::warn!("Mouse move failed");
                        status_label_clone.set_label("Status: Mouse move failed");
                    }
                } else {
                    log::error!("Failed to create UIActionSimulator");
                    status_label_clone.set_label("Status: Simulator not available");
                }
            });
        }

        {
            let text_ctrl_clone = text_ctrl.clone();
            let status_label_clone = status_label.clone();
            btn_key_combo.on_click(move |_| {
                log::info!("Simulating Ctrl+A (select all)...");
                status_label_clone.set_label("Status: Simulating Ctrl+A...");

                // Focus the text control
                text_ctrl_clone.set_focus();

                let sim = UIActionSimulator::new();
                if sim.is_ok() {
                    // Simulate Ctrl+A to select all text
                    // 'A' key code is 65
                    if sim.char_with_modifiers(65, KeyModifier::CONTROL) {
                        log::info!("Ctrl+A simulation successful");
                        status_label_clone.set_label("Status: Ctrl+A simulated (text selected)");
                    } else {
                        log::warn!("Ctrl+A simulation failed");
                        status_label_clone.set_label("Status: Key combo failed");
                    }
                } else {
                    log::error!("Failed to create UIActionSimulator");
                    status_label_clone.set_label("Status: Simulator not available");
                }
            });
        }

        {
            let status_label_clone = status_label.clone();
            btn_mouse_move.on_click(move |_| {
                log::info!("Demonstrating mouse movement...");
                status_label_clone.set_label("Status: Moving mouse...");

                let sim = UIActionSimulator::new();
                if sim.is_ok() {
                    // Move mouse to center of screen (approximate)
                    // In a real app, you'd calculate actual positions
                    let positions = [(100, 100), (200, 200), (300, 150), (250, 250)];

                    for (x, y) in positions.iter() {
                        if sim.mouse_move(*x, *y) {
                            log::info!("Mouse moved to ({}, {})", x, y);
                            // Small delay would be needed in real usage
                        }
                    }
                    status_label_clone.set_label("Status: Mouse movement demo complete");
                } else {
                    log::error!("Failed to create UIActionSimulator");
                    status_label_clone.set_label("Status: Simulator not available");
                }
            });
        }

        // Layout
        main_sizer.add(&title, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);
        main_sizer.add(&instructions, 0, SizerFlag::All, 10);
        main_sizer.add(&text_ctrl, 0, SizerFlag::Expand | SizerFlag::All, 10);

        button_sizer.add(&btn_type_text, 0, SizerFlag::All, 5);
        button_sizer.add(&btn_click_demo, 0, SizerFlag::All, 5);
        button_sizer.add(&btn_key_combo, 0, SizerFlag::All, 5);
        button_sizer.add(&btn_mouse_move, 0, SizerFlag::All, 5);

        main_sizer.add_sizer(&button_sizer, 0, SizerFlag::AlignCenterHorizontal, 0);
        main_sizer.add(&click_counter, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);
        main_sizer.add(&status_label, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);

        // Info about the API
        let info_text = StaticText::builder(&panel)
            .with_label(
                "API Methods:\n\
                 - mouse_move(x, y): Move mouse to screen coordinates\n\
                 - mouse_click(button): Click mouse button\n\
                 - mouse_down/up(button): Press/release mouse button\n\
                 - mouse_dbl_click(button): Double-click\n\
                 - mouse_drag_drop(x1,y1,x2,y2,button): Drag operation\n\
                 - key_down/up(keycode, modifiers): Press/release key\n\
                 - char_key(keycode): Press and release key\n\
                 - char_with_modifiers(keycode, modifiers): Key with modifiers\n\
                 - text(string): Type a string\n\
                 - select(text): Select item in focused control",
            )
            .build();
        main_sizer.add(&info_text, 0, SizerFlag::All, 10);

        panel.set_sizer(main_sizer, true);

        frame.show(true);
        frame.centre();
    });
}
