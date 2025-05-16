use wxdragon::prelude::*;

struct DropTargetPanel;

impl DropTargetPanel {
    fn new(parent: &Frame) {
        let panel = Panel::builder(parent)
            .with_size(Size::new(500, 400))
            .build();

        // Add a vertical sizer
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Create a title
        let title = StaticText::builder(&panel)
            .with_label("Drop files or text here:")
            .build();
        sizer.add_spacer(10);
        sizer.add(&title, 0, SizerFlag::Expand | SizerFlag::All, 5);
        sizer.add_spacer(10);

        // Create two drop areas - one for text, one for files
        let text_area = Panel::builder(&panel)
            .with_style(PanelStyle::BorderSunken)
            .with_size(Size::new(450, 120))
            .build();
        
        let file_area = Panel::builder(&panel)
            .with_style(PanelStyle::BorderSunken)
            .with_size(Size::new(450, 120))
            .build();
        
        // Create a status text for feedback
        let status_text = StaticText::builder(&panel)
            .with_label("Drag and drop files or text here")
            .build();
        
        // Setup the text drop area
        let text_sizer = BoxSizer::builder(Orientation::Vertical).build();
        let text_info = StaticText::builder(&text_area)
            .with_label("TEXT DROP ZONE\n\nDrag text here")
            .build();

        text_sizer.add_spacer(20);
        text_sizer.add(&text_info, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical | SizerFlag::All, 10);
        text_area.set_sizer(text_sizer, true);
        text_area.set_background_color(Colour::rgb(220, 240, 220));

        // Setup the file drop area
        let file_sizer = BoxSizer::builder(Orientation::Vertical).build();
        let file_info = StaticText::builder(&file_area)
            .with_label("FILE DROP ZONE\n\nDrag files here")
            .build();

        file_sizer.add_spacer(20);
        file_sizer.add(&file_info, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical | SizerFlag::All, 10);
        file_area.set_sizer(file_sizer, true);
        file_area.set_background_color(Colour::rgb(220, 220, 240));

        // Add the drop areas to the main sizer
        sizer.add(&text_area, 1, SizerFlag::Expand | SizerFlag::All, 10);
        sizer.add(&file_area, 1, SizerFlag::Expand | SizerFlag::All, 10);
        sizer.add(&status_text, 0, SizerFlag::Expand | SizerFlag::All, 10);
        
        // Set the panel's sizer
        panel.set_sizer(sizer, true);

        // Setup Text Drop Target with full callbacks
        // Clone the widgets to share ownership with the closures
        let status_text_clone = status_text.clone();
        let text_area_clone = text_area.clone();
        
        println!("Creating TextDropTarget...");
        TextDropTarget::builder(&text_area)
            .with_on_enter({
                let status_text = status_text_clone.clone();
                let text_area = text_area_clone.clone();
                move |x, y, _def_result| {
                    println!("TEXT target: OnEnter at ({}, {})", x, y);
                    status_text.set_label("Text entering drop zone");
                    text_area.set_background_color(Colour::rgb(180, 255, 180));
                    DragResult::Copy // Override to Copy
                }
            })
            .with_on_drag_over({
                let status_text = status_text_clone.clone();
                move |x, y, def_result| {
                    println!("TEXT target: OnDragOver at ({}, {})", x, y);
                    let msg = format!("Text dragging over at ({}, {})", x, y);
                    status_text.set_label(&msg);
                    def_result // Use default behavior
                }
            })
            .with_on_leave({
                let status_text = status_text_clone.clone();
                let text_area = text_area_clone.clone();
                move || {
                    println!("TEXT target: OnLeave");
                    status_text.set_label("Text left drop zone");
                    text_area.set_background_color(Colour::rgb(220, 240, 220));
                }
            })
            .with_on_drop({
                let status_text = status_text_clone.clone();
                move |x, y| {
                    println!("TEXT target: OnDrop at ({}, {})", x, y);
                    status_text.set_label("Text dropped, waiting for data...");
                    true // Accept the drop - IMPORTANT TO RETURN TRUE
                }
            })
            .with_on_data({
                let status_text = status_text_clone.clone();
                move |x, y, _def_result| {
                    println!("TEXT target: OnData at ({}, {})", x, y);
                    status_text.set_label("Getting text data...");
                    println!("TEXT target: Returning DragResult::Copy from OnData");
                    DragResult::Copy // Must return Copy to continue to OnDropText
                }
            })
            .with_on_drop_text({
                let status_text = status_text_clone.clone();
                move |text, x, y| {
                    println!("TEXT dropped: '{}' at ({}, {})", text, x, y);
                    let msg = format!("Received text: {}", text);
                    status_text.set_label(&msg);
                    
                    // Show a message dialog with the received text for debugging
                    MessageDialog::builder(None, &msg, "Text Received")
                        .with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation)
                        .build()
                        .show_modal();
                    
                    println!("TEXT target: OnDropText returning true");
                    true
                }
            })
            .build();

        // Setup File Drop Target with full callbacks
        // Clone the widgets to share ownership with the closures
        let status_text_clone = status_text.clone();
        let file_area_clone = file_area.clone();
        
        println!("Creating FileDropTarget...");
        FileDropTarget::builder(&file_area)
            .with_on_enter({
                let status_text = status_text_clone.clone();
                let file_area = file_area_clone.clone();
                move |x, y, _def_result| {
                    println!("FILE target: OnEnter at ({}, {})", x, y);
                    status_text.set_label("Files entering drop zone");
                    file_area.set_background_color(Colour::rgb(180, 180, 255));
                    DragResult::Copy
                }
            })
            .with_on_drag_over({
                let status_text = status_text_clone.clone();
                move |x, y, def_result| {
                    println!("FILE target: OnDragOver at ({}, {})", x, y);
                    let msg = format!("Files dragging over at ({}, {})", x, y);
                    status_text.set_label(&msg);
                    def_result
                }
            })
            .with_on_leave({
                let status_text = status_text_clone.clone();
                let file_area = file_area_clone.clone();
                move || {
                    println!("FILE target: OnLeave");
                    status_text.set_label("Files left drop zone");
                    file_area.set_background_color(Colour::rgb(220, 220, 240));
                }
            })
            .with_on_drop({
                let status_text = status_text_clone.clone();
                move |x, y| {
                    println!("FILE target: OnDrop at ({}, {})", x, y);
                    status_text.set_label("Files dropped, waiting for data...");
                    println!("FILE target: OnDrop returning true");
                    true // Accept the drop - IMPORTANT TO RETURN TRUE
                }
            })
            .with_on_data({
                let status_text = status_text_clone.clone();
                move |x, y, _def_result| {
                    println!("FILE target: OnData at ({}, {})", x, y);
                    status_text.set_label("Getting file data...");
                    println!("FILE target: Returning DragResult::Copy from OnData");
                    DragResult::Copy // Must return Copy to continue to OnDropFiles
                }
            })
            .with_on_drop_files({
                let status_text = status_text_clone.clone();
                move |files, x, y| {
                    println!("FILES dropped: {} files at ({}, {})", files.len(), x, y);
                    
                    // Print each file path for debugging
                    for (i, file) in files.iter().enumerate() {
                        println!("  File {}: {}", i+1, file);
                    }
                    
                    let files_str = files.join(", ");
                    let msg = format!("Received {} files: {}", files.len(), files_str);
                    println!("Setting status text to: {}", msg);
                    status_text.set_label(&msg);
                    
                    // Show a message dialog with the received files for debugging
                    MessageDialog::builder(None, &msg, "Files Received")
                        .with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation)
                        .build()
                        .show_modal();
                    
                    println!("FILE target: OnDropFiles returning true");
                    true
                }
            })
            .build();
    }
}

fn main() {
    wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Advanced Drag and Drop Example")
            .with_size(Size::new(600, 500))
            .with_position(Point::new(100, 100))
            .build();

        // Create our panel and store it in the static variable
        // to keep it alive until the application exits
        DropTargetPanel::new(&frame);
        
        // Show the frame
        frame.show(true);
    });
} 