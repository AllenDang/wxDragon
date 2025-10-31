use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        // Create the main frame
        let frame = Frame::builder()
            .with_title("Clipboard Example")
            .with_size(Size::new(500, 500))
            .build();

        // Create a panel to hold controls
        let panel = Panel::builder(&frame).build();

        // Create a vertical sizer for the panel
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Create a text control for input/output
        let text_ctrl = TextCtrl::builder(&panel)
            .with_size(Size::new(300, 100))
            .with_style(TextCtrlStyle::MultiLine)
            .build();

        // Add the text control to the sizer with some spacing
        sizer.add(&text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 10);

        // Create a horizontal sizer for text clipboard buttons
        let text_button_sizer = BoxSizer::builder(Orientation::Horizontal).build();

        // Create copy button
        let copy_button = Button::builder(&panel).with_label("Copy to Clipboard").build();

        // Create paste button
        let paste_button = Button::builder(&panel).with_label("Paste from Clipboard").build();

        // Add buttons to the horizontal sizer
        text_button_sizer.add(&copy_button, 0, SizerFlag::All, 5);
        text_button_sizer.add(&paste_button, 0, SizerFlag::All, 5);

        // Add the button sizer to the main sizer
        sizer.add_sizer(&text_button_sizer, 0, SizerFlag::AlignCenterHorizontal, 0);

        // Create a horizontal sizer for bitmap clipboard buttons
        let bitmap_button_sizer = BoxSizer::builder(Orientation::Horizontal).build();

        // Create a button to test bitmap clipboard
        let bitmap_button = Button::builder(&panel).with_label("Copy Test Bitmap").build();

        // Create a button to paste bitmap
        let paste_bitmap_button = Button::builder(&panel).with_label("Paste Bitmap").build();

        // Add buttons to the horizontal sizer
        bitmap_button_sizer.add(&bitmap_button, 0, SizerFlag::All, 5);
        bitmap_button_sizer.add(&paste_bitmap_button, 0, SizerFlag::All, 5);

        // Add the button sizer to the main sizer
        sizer.add_sizer(&bitmap_button_sizer, 0, SizerFlag::AlignCenterHorizontal, 0);

        // Create a static bitmap to display pasted image
        // Creating an initial black bitmap
        let initial_bitmap = Bitmap::new(1, 1).unwrap();
        let static_bitmap = StaticBitmap::builder(&panel)
            .with_size(Size::new(100, 100))
            .with_bitmap(Some(initial_bitmap))
            .build();

        // Add the static bitmap with some spacing
        sizer.add(&static_bitmap, 1, SizerFlag::Expand | SizerFlag::All, 10);

        // Create a horizontal sizer for file clipboard buttons
        let file_button_sizer = BoxSizer::builder(Orientation::Horizontal).build();

        // Create a button to copy a file to clipboard
        let copy_file_button = Button::builder(&panel).with_label("Copy File to Clipboard").build();

        // Create a button to paste files from clipboard
        let paste_file_button = Button::builder(&panel).with_label("Paste Files from Clipboard").build();

        // Add buttons to the horizontal sizer
        file_button_sizer.add(&copy_file_button, 0, SizerFlag::All, 5);
        file_button_sizer.add(&paste_file_button, 0, SizerFlag::All, 5);

        // Add the button sizer to the main sizer
        sizer.add_sizer(&file_button_sizer, 0, SizerFlag::AlignCenterHorizontal, 0);

        // Create a text control to display file paths
        let file_text_ctrl = TextCtrl::builder(&panel)
            .with_size(Size::new(300, 100))
            .with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly)
            .build();

        // Add the file text control to the sizer with some spacing
        sizer.add(&file_text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 10);

        // Set the panel's sizer
        panel.set_sizer(sizer, true);

        // Create a clipboard reference
        let clipboard = Clipboard::get();

        // Implement button event handlers
        let text_ctrl_copy = text_ctrl.clone();
        copy_button.on_click(move |_| {
            let text = text_ctrl_copy.get_value();
            if clipboard.set_text(&text) {
                println!("Text copied to clipboard: {text}");
            } else {
                println!("Failed to copy text to clipboard");
            }
        });

        let text_ctrl_paste = text_ctrl.clone();
        paste_button.on_click(move |_| {
            if let Some(text) = clipboard.get_text() {
                text_ctrl_paste.set_value(&text);
                println!("Text pasted from clipboard: {text}");
            } else {
                println!("No text on clipboard or clipboard access failed");
            }
        });

        // For bitmap button, create a test bitmap (red square)
        bitmap_button.on_click(move |_| {
            // Create a 50x50 red bitmap
            let mut data = Vec::new();
            for _ in 0..(50 * 50) {
                data.push(255); // R
                data.push(0); // G
                data.push(0); // B
                data.push(255); // A
            }
            let bitmap = Bitmap::from_rgba(&data, 50, 50).unwrap();

            // Create a bitmap data object and copy to clipboard
            let mut bitmap_data = BitmapDataObject::new(&bitmap);

            // Try to set the bitmap to clipboard
            if let Some(_locker) = clipboard.locker() {
                if clipboard.set_data(&mut bitmap_data) {
                    println!("Bitmap copied to clipboard");
                } else {
                    println!("Failed to copy bitmap to clipboard");
                }
            }
        });

        // For paste bitmap button - this should now work correctly with StaticBitmap's proper Clone
        let static_bitmap_clone = static_bitmap.clone();
        paste_bitmap_button.on_click(move |_| {
            // Check if bitmap format is supported
            if !clipboard.is_format_supported(DataFormat::BITMAP) {
                println!("No bitmap on clipboard");
                return;
            }

            // Create a bitmap data object to receive the data
            let bitmap_data = BitmapDataObject::new(&Bitmap::new(1, 1).unwrap());

            // Get the data from clipboard
            if let Some(_locker) = clipboard.locker() {
                if clipboard.get_data(&bitmap_data) {
                    if let Some(bitmap) = bitmap_data.get_bitmap() {
                        // Now static_bitmap_clone is a proper StaticBitmap, not just a Window
                        static_bitmap_clone.set_bitmap(&bitmap);
                        println!("Bitmap pasted from clipboard");
                    }
                } else {
                    println!("Failed to get bitmap from clipboard");
                }
            }
        });

        // Copy file to clipboard
        let clipboard_file = clipboard;
        copy_file_button.on_click(move |_| {
            // Create a file data object and add a file
            let mut file_data = FileDataObject::new();

            // Use a file that likely exists on most systems
            let file_path = if cfg!(target_os = "windows") {
                "C:\\Windows\\System32\\notepad.exe"
            } else if cfg!(target_os = "macos") {
                "/Applications/Safari.app"
            } else {
                "/etc/hosts" // On Linux/Unix systems
            };

            // Add the file to the data object
            file_data.add_file(file_path);
            println!("Adding file to clipboard: {file_path}");

            // Try to set the file to clipboard
            if let Some(_locker) = clipboard_file.locker() {
                if clipboard_file.set_data(&mut file_data) {
                    println!("File copied to clipboard");
                } else {
                    println!("Failed to copy file to clipboard");
                }
            }
        });

        // Paste files from clipboard
        let clipboard_paste = clipboard;
        let file_text_ctrl_paste = file_text_ctrl.clone();
        paste_file_button.on_click(move |_| {
            if let Some(_locker) = clipboard_paste.locker() {
                // Check if file format is supported
                if clipboard_paste.is_format_supported(DataFormat::FILENAME) {
                    // Create a file data object to receive the data
                    let file_data = FileDataObject::new();

                    // Try getting the data - this should now work with our fixed implementation
                    if clipboard_paste.get_data(&file_data) {
                        // Get all files from the data object
                        let files = file_data.get_files();

                        if files.is_empty() {
                            println!("No files on clipboard");
                            file_text_ctrl_paste.set_value("No files on clipboard");
                        } else {
                            // Display the file paths in the text control
                            let file_list = files.join("\n");
                            println!("Files pasted from clipboard:\n{file_list}");
                            file_text_ctrl_paste.set_value(&file_list);
                        }
                    } else {
                        // Fallback to text for files
                        if let Some(text) = clipboard_paste.get_text() {
                            println!("Retrieved file path as text: {text}");
                            file_text_ctrl_paste.set_value(&text);
                        } else {
                            println!("Failed to get files from clipboard");
                            file_text_ctrl_paste.set_value("Failed to get files from clipboard");
                        }
                    }
                } else {
                    println!("No files on clipboard");
                    file_text_ctrl_paste.set_value("No files on clipboard");
                }
            }
        });

        // Show the frame
        frame.show(true);
        frame.centre();
    });
}
