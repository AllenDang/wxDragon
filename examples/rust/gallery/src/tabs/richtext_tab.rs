use wxdragon::prelude::*;
use wxdragon::scrollable::WxScrollable;

pub struct RichTextTabControls {
    pub panel: Panel,
    pub rich_text_ctrl: RichTextCtrl,
    pub bold_btn: Button,
    pub italic_btn: Button,
    pub underline_btn: Button,
    pub font_size_spin: SpinCtrl,
    #[allow(dead_code)]
    pub font_size_label: StaticText,
    pub clear_btn: Button,
    pub save_btn: Button,
    pub load_btn: Button,
    pub undo_btn: Button,
    pub redo_btn: Button,
    pub cut_btn: Button,
    pub copy_btn: Button,
    pub paste_btn: Button,
    pub select_all_btn: Button,
    pub text_color_btn: Button,
    pub bg_color_btn: Button,
    pub reset_color_btn: Button,
    pub add_content_btn: Button,
    pub scroll_to_end_btn: Button,
    pub scroll_to_beginning_btn: Button,
    pub check_visibility_btn: Button,
    pub status_label: StaticText,
}

pub fn create_richtext_tab(notebook: &Notebook, _frame: &Frame) -> RichTextTabControls {
    let panel = Panel::builder(notebook)
        .with_style(PanelStyle::TabTraversal)
        .build();

    // Create the RichTextCtrl
    let rich_text_ctrl = RichTextCtrl::builder(&panel)
        .with_value("Welcome to wxDragon RichText Control!\n\nThis rich text control now supports scrolling functionality.\n\nTry these features:\n\n1. Add lots of content using the 'Add Content' button\n2. Use 'Scroll to End' to jump to the bottom\n3. Use 'Scroll to Beginning' to jump to the top\n4. Check if specific positions are visible\n5. Test auto-scroll behavior for log-like applications\n\nYou can also apply colors to selected text:\n- Select some text and click 'Text Color' to make it red\n- Select some text and click 'Background' to give it a yellow background\n- Use 'Reset Colors' to restore black text on white background\n\nThe scrolling functionality works generically for any widget that implements the WxScrollable trait!")
        .with_style(RichTextCtrlStyle::MultiLine | RichTextCtrlStyle::AutoUrl)
        .build();
    rich_text_ctrl.set_tooltip("Rich text editor with formatting capabilities");

    // Create formatting buttons
    let bold_btn = Button::builder(&panel).with_label("Bold").build();
    bold_btn.set_tooltip("Make selected text bold");

    let italic_btn = Button::builder(&panel).with_label("Italic").build();
    italic_btn.set_tooltip("Make selected text italic");

    let underline_btn = Button::builder(&panel).with_label("Underline").build();
    underline_btn.set_tooltip("Underline selected text");

    // Font size controls
    let font_size_label = StaticText::builder(&panel).with_label("Font Size:").build();

    let font_size_spin = SpinCtrl::builder(&panel)
        .with_initial_value(12)
        .with_min_value(8)
        .with_max_value(72)
        .build();
    font_size_spin.set_tooltip("Change font size for selected text (8-72pt)");

    // File operation buttons
    let clear_btn = Button::builder(&panel).with_label("Clear All").build();
    clear_btn.set_tooltip("Clear all content");

    let save_btn = Button::builder(&panel).with_label("Save File...").build();
    save_btn.set_tooltip("Save content to file (RTF or TXT)");

    let load_btn = Button::builder(&panel).with_label("Open File...").build();
    load_btn.set_tooltip("Open and load file (RTF or TXT)");

    // Edit operation buttons
    let undo_btn = Button::builder(&panel).with_label("Undo").build();
    undo_btn.set_tooltip("Undo last action");

    let redo_btn = Button::builder(&panel).with_label("Redo").build();
    redo_btn.set_tooltip("Redo last undone action");

    let cut_btn = Button::builder(&panel).with_label("Cut").build();
    cut_btn.set_tooltip("Cut selected text to clipboard");

    let copy_btn = Button::builder(&panel).with_label("Copy").build();
    copy_btn.set_tooltip("Copy selected text to clipboard");

    let paste_btn = Button::builder(&panel).with_label("Paste").build();
    paste_btn.set_tooltip("Paste text from clipboard");

    let select_all_btn = Button::builder(&panel).with_label("Select All").build();
    select_all_btn.set_tooltip("Select all text");

    // Color control buttons
    let text_color_btn = Button::builder(&panel).with_label("Text Color").build();
    text_color_btn.set_tooltip("Change text color for selected text");

    let bg_color_btn = Button::builder(&panel).with_label("Background").build();
    bg_color_btn.set_tooltip("Change background color for selected text");

    let reset_color_btn = Button::builder(&panel).with_label("Reset Colors").build();
    reset_color_btn.set_tooltip("Reset text and background colors to default");

    // New scrolling buttons
    let add_content_btn = Button::builder(&panel).with_label("Add Content").build();
    add_content_btn.set_tooltip("Add more content to the RichTextCtrl");

    let scroll_to_end_btn = Button::builder(&panel).with_label("Scroll to End").build();
    scroll_to_end_btn.set_tooltip("Scroll to the end of the RichTextCtrl");

    let scroll_to_beginning_btn = Button::builder(&panel)
        .with_label("Scroll to Beginning")
        .build();
    scroll_to_beginning_btn.set_tooltip("Scroll to the beginning of the RichTextCtrl");

    let check_visibility_btn = Button::builder(&panel)
        .with_label("Check Visibility")
        .build();
    check_visibility_btn.set_tooltip("Check if specific positions are visible in the RichTextCtrl");

    // Status label
    let status_label = StaticText::builder(&panel)
        .with_label("Ready - Try the scrolling features!")
        .build();

    // Layout the controls
    let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

    // Button row 1: Formatting controls
    let format_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    format_sizer.add(&bold_btn, 0, SizerFlag::All, 5);
    format_sizer.add(&italic_btn, 0, SizerFlag::All, 5);
    format_sizer.add(&underline_btn, 0, SizerFlag::All, 5);
    format_sizer.add_spacer(10);
    format_sizer.add(
        &font_size_label,
        0,
        SizerFlag::AlignCenterVertical | SizerFlag::Left | SizerFlag::Right,
        5,
    );
    format_sizer.add(&font_size_spin, 0, SizerFlag::All, 5);

    // Button row 2: Edit operations
    let edit_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    edit_sizer.add(&undo_btn, 0, SizerFlag::All, 5);
    edit_sizer.add(&redo_btn, 0, SizerFlag::All, 5);
    edit_sizer.add_spacer(10);
    edit_sizer.add(&cut_btn, 0, SizerFlag::All, 5);
    edit_sizer.add(&copy_btn, 0, SizerFlag::All, 5);
    edit_sizer.add(&paste_btn, 0, SizerFlag::All, 5);
    edit_sizer.add(&select_all_btn, 0, SizerFlag::All, 5);

    // Button row 3: File operations
    let file_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    file_sizer.add(&clear_btn, 0, SizerFlag::All, 5);
    file_sizer.add(&load_btn, 0, SizerFlag::All, 5);
    file_sizer.add(&save_btn, 0, SizerFlag::All, 5);

    // Button row 4: Color operations
    let color_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    color_sizer.add(&text_color_btn, 0, SizerFlag::All, 5);
    color_sizer.add(&bg_color_btn, 0, SizerFlag::All, 5);
    color_sizer.add(&reset_color_btn, 0, SizerFlag::All, 5);

    // Button row 5: Scrolling operations
    let scroll_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    scroll_sizer.add(&add_content_btn, 0, SizerFlag::All, 5);
    scroll_sizer.add(&scroll_to_end_btn, 0, SizerFlag::All, 5);
    scroll_sizer.add(&scroll_to_beginning_btn, 0, SizerFlag::All, 5);
    scroll_sizer.add(&check_visibility_btn, 0, SizerFlag::All, 5);

    // Add everything to main sizer
    main_sizer.add_sizer(&format_sizer, 0, SizerFlag::Expand | SizerFlag::All, 5);
    main_sizer.add_sizer(&edit_sizer, 0, SizerFlag::Expand | SizerFlag::All, 5);
    main_sizer.add_sizer(&file_sizer, 0, SizerFlag::Expand | SizerFlag::All, 5);
    main_sizer.add_sizer(&color_sizer, 0, SizerFlag::Expand | SizerFlag::All, 5);
    main_sizer.add_sizer(&scroll_sizer, 0, SizerFlag::Expand | SizerFlag::All, 5);
    main_sizer.add(&rich_text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 5);
    main_sizer.add(&status_label, 0, SizerFlag::Expand | SizerFlag::All, 5);

    panel.set_sizer(main_sizer, true);

    RichTextTabControls {
        panel,
        rich_text_ctrl,
        bold_btn,
        italic_btn,
        underline_btn,
        font_size_spin,
        font_size_label,
        clear_btn,
        save_btn,
        load_btn,
        undo_btn,
        redo_btn,
        cut_btn,
        copy_btn,
        paste_btn,
        select_all_btn,
        text_color_btn,
        bg_color_btn,
        reset_color_btn,
        add_content_btn,
        scroll_to_end_btn,
        scroll_to_beginning_btn,
        check_visibility_btn,
        status_label,
    }
}

impl RichTextTabControls {
    pub fn bind_events(&self) {
        // Bold button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.bold_btn.on_click(move |_event| {
            let (from, to) = rich_text_clone.get_selection();
            if from != to {
                rich_text_clone.set_style_range(from, to, true, false, false);
                status_label_clone.set_label("Applied bold formatting to selection");
                log::info!("Applied bold to range {from}-{to}");
            } else {
                status_label_clone.set_label("Please select text first");
            }
        });

        // Italic button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.italic_btn.on_click(move |_event| {
            let (from, to) = rich_text_clone.get_selection();
            if from != to {
                rich_text_clone.set_style_range(from, to, false, true, false);
                status_label_clone.set_label("Applied italic formatting to selection");
                log::info!("Applied italic to range {from}-{to}");
            } else {
                status_label_clone.set_label("Please select text first");
            }
        });

        // Underline button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.underline_btn.on_click(move |_event| {
            let (from, to) = rich_text_clone.get_selection();
            if from != to {
                rich_text_clone.set_style_range(from, to, false, false, true);
                status_label_clone.set_label("Applied underline formatting to selection");
                log::info!("Applied underline to range {from}-{to}");
            } else {
                status_label_clone.set_label("Please select text first");
            }
        });

        // Font size spinner
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        let font_size_spin_clone = self.font_size_spin.clone();
        self.font_size_spin.on_value_changed(move |_event| {
            let size = font_size_spin_clone.value();
            let (from, to) = rich_text_clone.get_selection();
            if from != to {
                rich_text_clone.set_font_size(from, to, size);
                status_label_clone.set_label(&format!("Changed font size to {size}pt"));
                log::info!("Applied font size {size} to range {from}-{to}");
            } else {
                status_label_clone.set_label(&format!("Font size {size}pt - select text to apply"));
            }
        });

        // Clear button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.clear_btn.on_click(move |_event| {
            rich_text_clone.clear();
            status_label_clone.set_label("Cleared all content");
            log::info!("Cleared all content");
        });

        // Undo button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.undo_btn.on_click(move |_event| {
            if rich_text_clone.can_undo() {
                rich_text_clone.undo();
                status_label_clone.set_label("Undone last action");
                log::info!("Undone last action");
            } else {
                status_label_clone.set_label("Nothing to undo");
            }
        });

        // Redo button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.redo_btn.on_click(move |_event| {
            if rich_text_clone.can_redo() {
                rich_text_clone.redo();
                status_label_clone.set_label("Redone last action");
                log::info!("Redone last action");
            } else {
                status_label_clone.set_label("Nothing to redo");
            }
        });

        // Cut button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.cut_btn.on_click(move |_event| {
            let (from, to) = rich_text_clone.get_selection();
            if from != to {
                rich_text_clone.cut();
                status_label_clone.set_label("Cut selection to clipboard");
                log::info!("Cut selection to clipboard");
            } else {
                status_label_clone.set_label("Nothing to cut");
            }
        });

        // Copy button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.copy_btn.on_click(move |_event| {
            let (from, to) = rich_text_clone.get_selection();
            if from != to {
                rich_text_clone.copy();
                status_label_clone.set_label("Copied selection to clipboard");
                log::info!("Copied selection to clipboard");
            } else {
                status_label_clone.set_label("Nothing to copy");
            }
        });

        // Paste button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.paste_btn.on_click(move |_event| {
            // Always try to paste - the control will handle it if clipboard is empty
            rich_text_clone.paste();
            status_label_clone.set_label("Pasted from clipboard");
            log::info!("Pasted from clipboard");
        });

        // Select All button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.select_all_btn.on_click(move |_event| {
            let last_pos = rich_text_clone.get_last_position();
            rich_text_clone.set_selection(0, last_pos);
            status_label_clone.set_label("Selected all text");
            log::info!("Selected all text");
        });

        // Save button - Use file dialog to save
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        let panel_clone = self.panel.clone();
        self.save_btn.on_click(move |_event| {
            // Create a file save dialog
            let save_dialog = FileDialog::builder(&panel_clone)
                .with_message("Save RichText Content")
                .with_default_dir(".")
                .with_default_file("document.rtf")
                .with_wildcard(
                    "RTF files (*.rtf)|*.rtf|Text files (*.txt)|*.txt|All files (*.*)|*.*",
                )
                .with_style(FileDialogStyle::Save | FileDialogStyle::OverwritePrompt)
                .build();

            if save_dialog.show_modal() == ID_OK {
                if let Some(file_path) = save_dialog.get_path() {
                    // Determine file type based on extension
                    let file_type = if file_path.to_lowercase().ends_with(".rtf") {
                        RichTextFileType::Rtf
                    } else {
                        RichTextFileType::Text
                    };

                    if rich_text_clone.save_file(&file_path, file_type) {
                        status_label_clone.set_label(&format!(
                            "Saved content to {}",
                            std::path::Path::new(&file_path)
                                .file_name()
                                .and_then(|name| name.to_str())
                                .unwrap_or(&file_path)
                        ));
                        log::info!("Successfully saved content to {file_path}");
                    } else {
                        status_label_clone.set_label("Failed to save file");
                        log::warn!("Failed to save content to file: {file_path}");
                    }
                } else {
                    status_label_clone.set_label("Failed to get file path");
                }
            } else {
                status_label_clone.set_label("Save operation cancelled");
            }
        });

        // Load button - Use file dialog to load
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        let panel_clone = self.panel.clone();
        self.load_btn.on_click(move |_event| {
            // Create a file open dialog
            let open_dialog = FileDialog::builder(&panel_clone)
                .with_message("Open Text File")
                .with_default_dir(".")
                .with_wildcard("All supported files|*.rtf;*.txt|RTF files (*.rtf)|*.rtf|Text files (*.txt)|*.txt|All files (*.*)|*.*")
                .with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist)
                .build();

            if open_dialog.show_modal() == ID_OK {
                if let Some(file_path) = open_dialog.get_path() {
                    // Determine file type based on extension
                    let file_type = if file_path.to_lowercase().ends_with(".rtf") {
                        RichTextFileType::Rtf
                    } else {
                        RichTextFileType::Text
                    };

                    if rich_text_clone.load_file(&file_path, file_type) {
                        status_label_clone.set_label(&format!("Loaded content from {}", 
                            std::path::Path::new(&file_path).file_name()
                                .and_then(|name| name.to_str())
                                .unwrap_or(&file_path)));
                        log::info!("Successfully loaded content from {file_path}");
                    } else {
                        status_label_clone.set_label("Failed to load file");
                        log::warn!("Failed to load content from file: {file_path}");
                    }
                } else {
                    status_label_clone.set_label("Failed to get file path");
                }
            } else {
                status_label_clone.set_label("Load operation cancelled");
            }
        });

        // Text Color button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.text_color_btn.on_click(move |_event| {
            let (from, to) = rich_text_clone.get_selection();
            if from != to {
                // Set selection to red color
                let red_color = Colour::rgb(255, 0, 0);
                rich_text_clone.set_text_color_selection(red_color);
                status_label_clone.set_label("Changed text color to red");
                log::info!("Applied red text color to range {from}-{to}");
            } else {
                status_label_clone.set_label("Please select text first");
            }
        });

        // Background Color button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.bg_color_btn.on_click(move |_event| {
            let (from, to) = rich_text_clone.get_selection();
            if from != to {
                // Set selection to yellow background
                let yellow_color = Colour::rgb(255, 255, 0);
                rich_text_clone.set_background_color_selection(yellow_color);
                status_label_clone.set_label("Changed background color to yellow");
                log::info!("Applied yellow background color to range {from}-{to}");
            } else {
                status_label_clone.set_label("Please select text first");
            }
        });

        // Reset Colors button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.reset_color_btn.on_click(move |_event| {
            let (from, to) = rich_text_clone.get_selection();
            if from != to {
                // Reset to default black text and white background
                let black_color = Colour::rgb(0, 0, 0);
                let white_color = Colour::rgb(255, 255, 255);
                rich_text_clone.set_text_color_selection(black_color);
                rich_text_clone.set_background_color_selection(white_color);
                status_label_clone.set_label("Reset colors to default");
                log::info!("Reset colors to default for range {from}-{to}");
            } else {
                status_label_clone.set_label("Please select text first");
            }
        });

        // RichTextCtrl selection changed event
        let status_label_clone = self.status_label.clone();
        let rich_text_clone = self.rich_text_ctrl.clone();
        self.rich_text_ctrl.on_selection_changed(move |_event| {
            let (from, to) = rich_text_clone.get_selection();
            if from != to {
                status_label_clone.set_label(&format!(
                    "Selection: {} characters ({from}-{to})",
                    to - from
                ));
            } else {
                status_label_clone.set_label(&format!("Cursor at position {from}"));
            }
        });

        // Scrolling event handlers

        // Add Content button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.add_content_btn.on_click(move |_event| {
            let new_content = format!("\n\nAdded content at position {} - Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.", rich_text_clone.get_last_position());
            rich_text_clone.append_text(&new_content);

            // Demonstrate auto-scroll behavior 
            if rich_text_clone.auto_scroll_if_at_end(100) {
                status_label_clone.set_label("Added content and auto-scrolled to end");
            } else {
                status_label_clone.set_label("Added content (no auto-scroll, user had scrolled away)");
            }
            log::info!("Added content and checked auto-scroll behavior");
        });

        // Scroll to End button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.scroll_to_end_btn.on_click(move |_event| {
            rich_text_clone.scroll_to_end();
            let last_pos = rich_text_clone.get_last_position();
            status_label_clone.set_label(&format!("Scrolled to end (position {last_pos})"));
            log::info!("Scrolled to end position {last_pos}");
        });

        // Scroll to Beginning button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.scroll_to_beginning_btn.on_click(move |_event| {
            rich_text_clone.scroll_to_beginning();
            status_label_clone.set_label("Scrolled to beginning (position 0)");
            log::info!("Scrolled to beginning");
        });

        // Check Visibility button
        let rich_text_clone = self.rich_text_ctrl.clone();
        let status_label_clone = self.status_label.clone();
        self.check_visibility_btn.on_click(move |_event| {
            let test_position = 1000;
            let is_visible = rich_text_clone.is_position_visible(test_position);
            let last_pos = rich_text_clone.get_last_position();

            if last_pos < test_position {
                status_label_clone.set_label(&format!("Position {test_position} doesn't exist (last position: {last_pos})"));
            } else if is_visible {
                status_label_clone.set_label(&format!("Position {test_position} is visible on screen"));
            } else {
                status_label_clone.set_label(&format!("Position {test_position} is not visible - use scroll buttons to see it"));
            }
            log::info!("Checked visibility of position {test_position} (visible: {is_visible}, last_pos: {last_pos})");
        });
    }
}
