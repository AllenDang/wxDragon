use wxdragon::prelude::*;

#[derive(Clone)]
struct MainFrame {
    frame: Frame,
}

impl MainFrame {
    fn new() -> Self {
        let frame = Frame::builder()
            .with_title("StyledTextCtrl Test - Enhanced")
            .with_size(Size::new(800, 600))
            .build();

        let stc = StyledTextCtrl::builder(&frame).with_size(Size::new(780, 580)).build();

        // Set some sample text
        stc.set_text("// Welcome to wxDragon StyledTextCtrl!\n\n");
        stc.append_text("This is a comprehensive text editor control.\n\n");
        stc.append_text("Features demonstrated:\n");
        stc.append_text("- Text manipulation\n");
        stc.append_text("- Clipboard operations\n");
        stc.append_text("- Markers and bookmarks\n");
        stc.append_text("- Styling and syntax highlighting\n");
        stc.append_text("- Advanced navigation\n\n");
        stc.append_text("Try:\n");
        stc.append_text("- Cmd+A to select all\n");
        stc.append_text("- Cmd+C to copy\n");
        stc.append_text("- Cmd+V to paste\n");
        stc.append_text("- Cmd+Z to undo\n");
        stc.append_text("- Cmd+Plus/Minus to zoom\n\n");

        // Add a marker at line 0 (first line)
        stc.marker_define(0, 1, Colour::rgb(255, 0, 0), Colour::rgb(255, 255, 0)); // Circle marker
        stc.marker_add(0, 0);

        // Add another marker at line 10
        stc.marker_define(1, 2, Colour::rgb(0, 0, 255), Colour::rgb(192, 192, 192)); // Square marker
        stc.marker_add(10, 1);

        // Set up some basic styling
        stc.style_set_foreground(0, Colour::rgb(0, 0, 0)); // Default style
        stc.style_set_background(0, Colour::rgb(255, 255, 255));
        stc.style_set_size(0, 12);

        // Style for comments (simulate simple syntax highlighting)
        stc.style_set_foreground(1, Colour::rgb(0, 128, 0));
        stc.style_set_italic(1, true);

        // Apply comment style to the first line
        stc.start_styling(0);
        stc.set_styling(43, 1); // Style the first comment line

        // Set up event handlers
        stc.on_stc_zoom(move |_| {
            println!("Zoom level changed to: {}", stc.get_zoom());
        });

        stc.on_stc_modified(move |_| {
            let length = stc.get_length();
            let lines = stc.get_line_count();
            println!("Text modified - Length: {length}, Lines: {lines}");
        });

        stc.on_stc_margin_click(move |event| {
            let pos = event.get_position().unwrap_or(0);
            let line = stc.line_from_position(pos);
            println!("Margin clicked at line: {line}");

            // Toggle marker on clicked line
            let markers = stc.marker_get(line);
            if markers & 1 != 0 {
                stc.marker_delete(line, 0);
                println!("Removed marker from line {line}");
            } else {
                stc.marker_add(line, 0);
                println!("Added marker to line {line}");
            }
        });

        frame.show(true);

        // Demo some functions
        println!("=== StyledTextCtrl Demo ===");
        println!("Initial text length: {}", stc.get_length());
        println!("Number of lines: {}", stc.get_line_count());
        println!("Character at position 0: {}", stc.get_char_at(0));
        println!("Current position: {}", stc.get_current_pos());
        println!("Selection mode: {}", stc.get_selection_mode());

        // Test line operations
        println!("Line 0 text: '{}'", stc.get_line_text(0).trim());
        println!("Line 0 length: {}", stc.get_line_length(0));

        // Position the cursor at the end
        let end_pos = stc.get_length();
        stc.set_current_pos(end_pos);
        stc.ensure_caret_visible();

        Self { frame }
    }
}

fn main() {
    let _ = wxdragon::main(|_| {
        let main_frame = MainFrame::new();
        main_frame.frame.show(true);
        main_frame.frame.centre();
    });
}
