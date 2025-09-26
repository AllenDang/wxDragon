use wxdragon::event::window_events::WindowEventData;
use wxdragon::prelude::*;

fn main() {
    println!("ComboBox Modifier Key Test - Testing CTRL+BACKSPACE functionality");

    let _ = wxdragon::main(|_| {
        // Create the main frame
        let frame = Frame::builder()
            .with_title("ComboBox Modifier Key Test")
            .with_size(Size::new(500, 400))
            .build();

        // Create a panel
        let panel = Panel::builder(&frame)
            .with_style(PanelStyle::TabTraversal)
            .build();

        // Create a vertical box sizer
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Create instructions
        let instructions = StaticText::builder(&panel)
            .with_label("Instructions:\n• Type text in the ComboBox below\n• Use CTRL+BACKSPACE to delete words\n• Use other modifier keys to test detection")
            .build();

        // Create a ComboBox with some initial choices and ProcessEnter style for better event handling
        let combo_items = [
            "Type your own text here",
            "This is a sample text",
            "Word deletion test",
            "The quick brown fox jumps over the lazy dog",
        ];
        let combo_box = ComboBox::builder(&panel)
            .with_string_choices(&combo_items)
            .with_style(ComboBoxStyle::ProcessEnter)
            .with_value("Type here and press CTRL+BACKSPACE")
            .build();

        // Status label to show what's happening
        let status_label = StaticText::builder(&panel)
            .with_label("Status: Ready")
            .build();

        // Test key events on the ComboBox
        let status_clone = status_label.clone();
        let combo_clone = combo_box.clone();
        combo_box.on_key_down(move |event| {
            if let WindowEventData::Keyboard(ref key_event) = event {
                let mut should_handle = false;
                let mut status_text = String::new();

                if let Some(key_code) = key_event.get_key_code() {
                    // Check for CTRL+BACKSPACE (key code 8 is backspace)
                    if key_code == 8 && key_event.control_down() {
                        delete_word_back(&combo_clone);
                        status_text = "CTRL+BACKSPACE: Deleted word back".to_string();
                        should_handle = true;
                    }
                    // Log other modifier combinations for testing
                    else if key_event.control_down()
                        || key_event.shift_down()
                        || key_event.alt_down()
                        || key_event.meta_down()
                    {
                        let mut modifiers = Vec::new();
                        if key_event.control_down() {
                            modifiers.push("CTRL");
                        }
                        if key_event.shift_down() {
                            modifiers.push("SHIFT");
                        }
                        if key_event.alt_down() {
                            modifiers.push("ALT");
                        }
                        if key_event.meta_down() {
                            modifiers.push("META");
                        }
                        if key_event.cmd_down() {
                            modifiers.push("CMD");
                        }

                        status_text = format!(
                            "Key: {} with modifiers: {}",
                            key_code_to_name(key_code),
                            modifiers.join("+")
                        );
                    }
                }

                // Update status if we detected something interesting
                if !status_text.is_empty() {
                    status_clone.set_label(&status_text);
                    println!("{}", status_text);
                }

                // If we handled the event (like CTRL+BACKSPACE), don't pass it on
                if should_handle {
                    event.skip(false);
                } else {
                    event.skip(true);
                }
            }
        });

        // Also handle regular ComboBox events for completeness
        let status_clone2 = status_label.clone();
        combo_box.on_text_updated(move |_event_data| {
            status_clone2.set_label("Status: Text updated");
        });

        let status_clone3 = status_label.clone();
        combo_box.on_selection_changed(move |event_data| {
            if let Some(selection) = event_data.get_selection() {
                status_clone3.set_label(&format!("Status: Selected item {}", selection));
            }
        });

        // Add widgets to sizer with proper spacing
        sizer.add_spacer(10);
        sizer.add(&instructions, 0, SizerFlag::Expand | SizerFlag::All, 10);
        sizer.add_spacer(10);

        sizer.add(
            &StaticText::builder(&panel).with_label("ComboBox:").build(),
            0,
            SizerFlag::All,
            5,
        );
        sizer.add(&combo_box, 0, SizerFlag::Expand | SizerFlag::All, 5);
        sizer.add_spacer(10);
        sizer.add(&status_label, 0, SizerFlag::Expand | SizerFlag::All, 5);
        sizer.add_spacer(10);

        // Set the sizer on the panel
        panel.set_sizer(sizer, true);

        // Handle frame close event
        let frame_clone = frame.clone();
        frame.on_close(move |_event_data| {
            println!("Frame closing!");
            frame_clone.destroy();
        });

        // Show the frame
        frame.show(true);
        frame.centre();

        println!("ComboBox test is ready. Try typing and using CTRL+BACKSPACE!");
    });
}

/// Delete the word to the left of the cursor in a ComboBox
fn delete_word_back(combo_box: &ComboBox) {
    let current_text = combo_box.get_value();
    let cursor_pos = combo_box.get_insertion_point() as usize;

    if cursor_pos == 0 || current_text.is_empty() {
        return; // Nothing to delete
    }

    let chars: Vec<char> = current_text.chars().collect();
    if cursor_pos > chars.len() {
        return;
    }

    let mut word_start = cursor_pos;

    // Skip whitespace backwards
    while word_start > 0 && chars[word_start - 1].is_whitespace() {
        word_start -= 1;
    }

    // Find start of word (skip non-whitespace)
    while word_start > 0 && !chars[word_start - 1].is_whitespace() {
        word_start -= 1;
    }

    if word_start < cursor_pos {
        // Build new text without the deleted word
        let before: String = chars[..word_start].iter().collect();
        let after: String = chars[cursor_pos..].iter().collect();
        let new_text = before + &after;

        // Update the ComboBox
        combo_box.set_value(&new_text);
        combo_box.set_insertion_point(word_start as i64);

        println!(
            "Deleted word: cursor moved from {} to {}",
            cursor_pos, word_start
        );
    }
}

/// Convert key code to human-readable name for debugging
fn key_code_to_name(key_code: i32) -> String {
    match key_code {
        8 => "BACKSPACE".to_string(),
        13 => "ENTER".to_string(),
        27 => "ESCAPE".to_string(),
        32 => "SPACE".to_string(),
        127 => "DELETE".to_string(),
        314 => "DOWN".to_string(),
        315 => "UP".to_string(),
        316 => "LEFT".to_string(),
        317 => "RIGHT".to_string(),
        _ if (32..=126).contains(&key_code) => {
            format!("'{}'", key_code as u8 as char)
        }
        _ => format!("KEY_{}", key_code),
    }
}
