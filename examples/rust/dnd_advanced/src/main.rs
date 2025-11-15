use wxdragon::prelude::*;

struct DropTargetPanel;

impl DropTargetPanel {
    fn setup(parent: &Frame) {
        let panel = Panel::builder(parent).with_size(Size::new(500, 400)).build();

        // Add a vertical sizer
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Create a title
        let title = StaticText::builder(&panel).with_label("Drop files or text here:").build();
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
        text_sizer.add(
            &text_info,
            0,
            SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical | SizerFlag::All,
            10,
        );
        text_area.set_sizer(text_sizer, true);
        text_area.set_background_color(Colour::rgb(220, 240, 220));

        // Setup the file drop area
        let file_sizer = BoxSizer::builder(Orientation::Vertical).build();
        let file_info = StaticText::builder(&file_area)
            .with_label("FILE DROP ZONE\n\nDrag files here")
            .build();

        file_sizer.add_spacer(20);
        file_sizer.add(
            &file_info,
            0,
            SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical | SizerFlag::All,
            10,
        );
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

        log::info!("Creating TextDropTarget...");
        let panel_clone_text = panel.clone(); // Clone for the text closure
        TextDropTarget::builder(&text_area)
            .with_on_enter({
                let status_text = status_text_clone.clone();
                let text_area = text_area_clone.clone();
                move |x, y, _def_result| {
                    log::info!("TEXT target: OnEnter at ({x}, {y})");
                    status_text.set_label("Text entering drop zone");
                    text_area.set_background_color(Colour::rgb(180, 255, 180));
                    DragResult::Copy // Override to Copy
                }
            })
            .with_on_drag_over({
                let status_text = status_text_clone.clone();
                move |x, y, def_result| {
                    log::info!("TEXT target: OnDragOver at ({x}, {y})");
                    let msg = format!("Text dragging over at ({x}, {y})");
                    status_text.set_label(&msg);
                    def_result // Use default behavior
                }
            })
            .with_on_leave({
                let status_text = status_text_clone.clone();
                let text_area = text_area_clone.clone();
                move || {
                    log::info!("TEXT target: OnLeave");
                    status_text.set_label("Text left drop zone");
                    text_area.set_background_color(Colour::rgb(220, 240, 220));
                }
            })
            .with_on_drop({
                let status_text = status_text_clone.clone();
                move |x, y| {
                    log::info!("TEXT target: OnDrop at ({x}, {y})");
                    status_text.set_label("Text dropped, waiting for data...");
                    true // Accept the drop - IMPORTANT TO RETURN TRUE
                }
            })
            .with_on_data({
                let status_text = status_text_clone.clone();
                move |x, y, _def_result| {
                    log::info!("TEXT target: OnData at ({x}, {y})");
                    status_text.set_label("Getting text data...");
                    log::info!("TEXT target: Returning DragResult::Copy from OnData");
                    DragResult::Copy // Must return Copy to continue to OnDropText
                }
            })
            .with_on_drop_text({
                let status_text = status_text_clone.clone();
                let panel_for_text_dialog = panel_clone_text.clone();
                move |text, x, y| {
                    log::info!("TEXT dropped: '{text}' at ({x}, {y})");
                    let msg = format!("Received text: {text}");
                    status_text.set_label(&msg);

                    MessageDialog::builder(&panel_for_text_dialog, &msg, "Text Received")
                        .with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation)
                        .build()
                        .show_modal();

                    log::info!("TEXT target: OnDropText returning true");
                    true
                }
            })
            .build();

        // Setup File Drop Target with full callbacks
        let status_text_clone_files = status_text.clone();
        let file_area_clone_files = file_area.clone();
        let panel_clone_files = panel.clone();

        log::info!("Creating FileDropTarget...");
        FileDropTarget::builder(&file_area_clone_files)
            .with_on_enter({
                let status_text = status_text_clone_files.clone();
                let file_area = file_area_clone_files.clone();
                move |x, y, _def_result| {
                    log::info!("FILE target: OnEnter at ({x}, {y})");
                    status_text.set_label("Files entering drop zone");
                    file_area.set_background_color(Colour::rgb(180, 180, 255));
                    DragResult::Copy
                }
            })
            .with_on_drag_over({
                let status_text = status_text_clone_files.clone();
                move |x, y, def_result| {
                    log::info!("FILE target: OnDragOver at ({x}, {y})");
                    let msg = format!("Files dragging over at ({x}, {y})");
                    status_text.set_label(&msg);
                    def_result
                }
            })
            .with_on_leave({
                let status_text = status_text_clone_files.clone();
                let file_area = file_area_clone_files.clone();
                move || {
                    log::info!("FILE target: OnLeave");
                    status_text.set_label("Files left drop zone");
                    file_area.set_background_color(Colour::rgb(220, 220, 240));
                }
            })
            .with_on_drop({
                let status_text = status_text_clone_files.clone();
                move |x, y| {
                    log::info!("FILE target: OnDrop at ({x}, {y})");
                    status_text.set_label("Files dropped, waiting for data...");
                    log::info!("FILE target: OnDrop returning true");
                    true
                }
            })
            .with_on_data({
                let status_text = status_text_clone_files.clone();
                move |x, y, _def_result| {
                    log::info!("FILE target: OnData at ({x}, {y})");
                    status_text.set_label("Getting file data...");
                    log::info!("FILE target: Returning DragResult::Copy from OnData");
                    DragResult::Copy
                }
            })
            .with_on_drop_files({
                let status_text = status_text_clone_files.clone();
                let panel_for_files_dialog = panel_clone_files.clone();
                move |files, x, y| {
                    log::info!("FILES dropped: {} files at ({}, {})", files.len(), x, y);

                    for (i, file) in files.iter().enumerate() {
                        log::info!("  File {}: {}", i + 1, file);
                    }

                    let files_str = files.join(", ");
                    let msg = format!("Received {} files: {}", files.len(), files_str);
                    log::info!("Setting status text to: {msg}");
                    status_text.set_label(&msg);

                    MessageDialog::builder(&panel_for_files_dialog, &msg, "Files Received")
                        .with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation)
                        .build()
                        .show_modal();

                    log::info!("FILE target: OnDropFiles returning true");
                    true
                }
            })
            .build();
    }
}

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Advanced Drag and Drop Example")
            .with_size(Size::new(600, 500))
            .with_position(Point::new(100, 100))
            .build();

        // Create our panel and store it in the static variable
        // to keep it alive until the application exits
        DropTargetPanel::setup(&frame);

        // Show the frame
        frame.show(true);
    });
}
