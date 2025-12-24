use wxdragon::event::WindowEvents;
use wxdragon::event::button_events::ButtonEvents;
use wxdragon::event::window_events::WindowEventData;
use wxdragon::prelude::*;
use wxdragon::sizers::box_sizer::BoxSizer;
use wxdragon::widgets::button::Button;
use wxdragon::widgets::static_text::StaticText;
use wxdragon::widgets::togglebutton::ToggleButton;

fn main() {
    println!("wxdragon events test - starting...");

    let _ = wxdragon::main(|_| {
        println!("Inside wxdragon::main callback");
        // Create a frame
        let frame = Frame::builder()
            .with_title("Events Demo")
            .with_size(Size::new(500, 400))
            .build();

        // Create a panel
        let panel = Panel::builder(&frame).build();

        // Create a vertical box sizer
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Create a regular button
        let button = Button::builder(&panel)
            .with_label("Click Me")
            .with_pos(Point::new(10, 10))
            .with_size(Size::new(200, 40))
            .build();

        // Create a toggle button
        let toggle_button = ToggleButton::builder(&panel)
            .with_label("Toggle Me")
            .with_pos(Point::new(10, 60))
            .with_size(Size::new(200, 40))
            .build();

        // Create a status area for window events
        let event_status = StaticText::builder(&panel)
            .with_label("Event Status: None")
            .with_size(Size::new(400, 30))
            .build();

        // Set up event handlers using the ButtonEvents trait

        // Regular button click event
        button.on_click(move |event| {
            println!("Button clicked! ID: {}", event.get_id());
        });

        // Toggle button events
        toggle_button.on_toggle(move |event| {
            println!("Toggle button toggled! ID: {}", event.get_id());
            if let Some(checked) = event.is_checked() {
                println!("New state: {}", if checked { "ON" } else { "OFF" });
            }
        });

        // Add window events using the WindowEvents trait
        button.on_mouse_left_down(move |mouse_event| {
            if let WindowEventData::MouseButton(ref mouse_data) = mouse_event {
                if let Some(pos) = mouse_data.get_position() {
                    event_status.set_label(&format!("Event Status: Left mouse down at ({}, {})", pos.x, pos.y));
                    println!("Left mouse down on button at ({}, {})", pos.x, pos.y);
                }

                // Skip this event to allow button click processing
                mouse_event.skip(true);
            }
        });

        button.on_mouse_right_down(move |mouse_event| {
            if let WindowEventData::MouseButton(ref mouse_data) = mouse_event {
                if let Some(pos) = mouse_data.get_position() {
                    event_status.set_label(&format!("Event Status: Right mouse down at ({}, {})", pos.x, pos.y));
                    println!("Right mouse down on button at ({}, {})", pos.x, pos.y);
                }

                // Skip this event to allow other handlers to process it
                mouse_event.skip(true);
            }
        });

        toggle_button.on_mouse_motion(move |motion_event| {
            if let WindowEventData::MouseMotion(ref motion_data) = motion_event {
                if let Some(pos) = motion_data.get_position() {
                    event_status.set_label(&format!("Event Status: Mouse motion at ({}, {})", pos.x, pos.y));
                    println!("Mouse motion at ({}, {})", pos.x, pos.y);
                }

                // Skip this event to allow other handlers to process it
                motion_event.skip(true);
            }
        });

        // Also test window events on the panel
        panel.on_size(move |_event_data| {
            event_status.set_label("Event Status: Panel resized");
            println!("Panel resized");
        });

        // Handle frame close event
        frame.on_close(move |_event_data| {
            println!("Frame close event received!");
            // Allow the close to proceed
            frame.destroy();
        });

        // Add widgets to sizer with spacing
        sizer.add_spacer(20);
        sizer.add(&button, 0, SizerFlag::Expand | SizerFlag::All, 10);
        sizer.add_spacer(10);
        sizer.add(&toggle_button, 0, SizerFlag::Expand | SizerFlag::All, 10);
        sizer.add_spacer(10);
        sizer.add(&event_status, 0, SizerFlag::Expand | SizerFlag::All, 10);
        sizer.add_spacer(20);

        // Set the sizer for the panel
        panel.set_sizer(sizer, true);

        // Show the frame
        frame.show(true);
        frame.centre();
    });
}
