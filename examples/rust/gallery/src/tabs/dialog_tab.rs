use wxdragon::prelude::*;

use std::time::Duration;
use std::{sync::Mutex, thread};

#[allow(dead_code)]
pub struct DialogTabControls {
    pub panel: Panel,
    pub show_msg_dialog_btn: Button,
    pub open_file_btn: Button,
    pub save_file_btn: Button,
    pub file_dialog_status_label: StaticText,
    pub get_text_btn: Button,
    pub get_password_btn: Button,
    pub text_entry_status_label: StaticText,
    pub choose_colour_btn: Button,
    pub colour_dialog_status_label: StaticText,
    pub font_button: Button,
    pub font_sample_text: StaticText,
    pub progress_button: Button,
    // Added for FilePickerCtrl
    pub file_picker_ctrl_label: StaticText,
    pub file_picker_ctrl: FilePickerCtrl,
    pub file_picker_status_label: StaticText,
    // Added for DirPickerCtrl
    pub dir_picker_ctrl_label: StaticText,
    pub dir_picker_ctrl: DirPickerCtrl,
    pub dir_picker_status_label: StaticText,
    // Added for FontPickerCtrl
    pub font_picker_ctrl_label: StaticText,
    pub font_picker_ctrl: FontPickerCtrl,
    pub font_picker_status_label: StaticText,
    // Added for NotificationMessage
    pub show_notification_btn: Button,
    pub notification_status_label: StaticText,
    // Added for SingleChoiceDialog
    pub show_single_choice_dialog_btn: Button,
    pub single_choice_dialog_status_label: StaticText,
    // Added for MultiChoiceDialog
    pub show_multi_choice_dialog_btn: Button,
    pub multi_choice_dialog_status_label: StaticText,
    pub dlg_dir_button: Button,
}

// Just make NotificationMessage alive long enough to show it
static NOTIFICATION_MESSAGE: Mutex<Option<NotificationMessage>> = Mutex::new(None);

pub fn create_dialog_tab(notebook: &Notebook, _frame: &Frame) -> DialogTabControls {
    let dialog_panel = Panel::builder(notebook)
        .with_style(PanelStyle::TabTraversal)
        .build();

    // Message Dialog section
    let show_msg_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("Message Dialog:")
        .build();
    let show_msg_dialog_btn = Button::builder(&dialog_panel)
        .with_label("Show Info")
        .build();
    show_msg_dialog_btn.set_tooltip("Click to show an informational message dialog.");

    // File Dialog section
    let file_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("File Dialog:")
        .build();
    let open_file_btn = Button::builder(&dialog_panel)
        .with_label("Open File...")
        .build();
    open_file_btn.set_tooltip("Click to show an open file dialog.");
    let save_file_btn = Button::builder(&dialog_panel)
        .with_label("Save File...")
        .build();
    save_file_btn.set_tooltip("Click to show a save file dialog.");
    let file_dialog_status_label = StaticText::builder(&dialog_panel)
        .with_label("File Dialog Status: -")
        .build();

    // Text Entry Dialog section
    let text_entry_label = StaticText::builder(&dialog_panel)
        .with_label("Text Entry:")
        .build();
    let get_text_btn = Button::builder(&dialog_panel)
        .with_label("Get Input...")
        .build();
    get_text_btn.set_tooltip("Click to show a text entry dialog.");
    let get_password_btn = Button::builder(&dialog_panel)
        .with_label("Get Password...")
        .build();
    get_password_btn.set_tooltip("Click to show a password entry dialog.");
    let text_entry_status_label = StaticText::builder(&dialog_panel)
        .with_label("Text Entry Status: -")
        .build();

    // Colour Dialog section
    let colour_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("Colour Dialog:")
        .build();
    let choose_colour_btn = Button::builder(&dialog_panel)
        .with_label("Choose Colour...")
        .build();
    choose_colour_btn.set_tooltip("Click to show a colour picker dialog.");
    let colour_dialog_status_label = StaticText::builder(&dialog_panel)
        .with_label("Colour Dialog Status: -")
        .build();

    // Add FontDialog button after the ColourDialog
    // Create button to choose a font
    let font_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("Font Dialog:")
        .build();
    let font_button = Button::builder(&dialog_panel)
        .with_label("Choose Font...")
        .build();
    font_button.set_tooltip("Click to show a font picker dialog.");
    let font_sample_text = StaticText::builder(&dialog_panel)
        .with_label("Font Sample")
        .build();

    // Progress Dialog section
    let progress_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("Progress Dialog:")
        .build();
    let progress_button = Button::builder(&dialog_panel)
        .with_label("Show Progress...")
        .build();
    progress_button.set_tooltip("Click to show a progress dialog demonstration.");

    // FilePickerCtrl section
    let file_picker_ctrl_label = StaticText::builder(&dialog_panel)
        .with_label("FilePickerCtrl:")
        .build();
    let file_picker_ctrl = FilePickerCtrl::builder(&dialog_panel)
        .with_message("Select a file with FilePickerCtrl")
        .with_wildcard("Rust files (*.rs)|*.rs|Text files (*.txt)|*.txt|All files (*.*)|*.*")
        .with_style(FilePickerCtrlStyle::Open | FilePickerCtrlStyle::FileMustExist)
        .build();
    file_picker_ctrl.set_tooltip("Select a file to see its path below.");
    let file_picker_status_label = StaticText::builder(&dialog_panel)
        .with_label("FilePickerCtrl Path: -")
        .build();

    // Layout using Main Vertical BoxSizer and child FlexGridSizer
    let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
    let label_flags = SizerFlag::AlignRight | SizerFlag::AlignCenterVertical;
    let control_flags = SizerFlag::Expand; // General flag for controls spanning the column

    let grid_sizer = FlexGridSizer::builder(0, 2) // 0 rows means flexible, 2 columns
        .with_vgap(5)
        .with_hgap(5)
        .build();
    grid_sizer.add_growable_col(0, 1); // Label column (flex factor 1)
    grid_sizer.add_growable_col(1, 3); // Control column (flex factor 3)

    // Add Message Dialog controls
    grid_sizer.add(&show_msg_dialog_label, 0, label_flags, 0);
    grid_sizer.add(&show_msg_dialog_btn, 1, control_flags, 0);

    // Event binding is now handled in bind_events()

    // Add File Dialog controls
    grid_sizer.add(&file_dialog_label, 0, label_flags, 0);
    let file_btns_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    // Event bindings for file dialog buttons are handled in bind_events()
    file_btns_sizer.add(
        &open_file_btn,
        0,
        SizerFlag::AlignCenterVertical | SizerFlag::All,
        2,
    );
    file_btns_sizer.add(
        &save_file_btn,
        0,
        SizerFlag::AlignCenterVertical | SizerFlag::All,
        2,
    );
    file_btns_sizer.add_spacer(10);
    file_btns_sizer.add(
        &file_dialog_status_label,
        1,
        SizerFlag::Expand | SizerFlag::All,
        2,
    );
    grid_sizer.add_sizer(&file_btns_sizer, 1, SizerFlag::Expand, 0);

    // Add Text Entry Dialog controls
    grid_sizer.add(&text_entry_label, 0, label_flags, 0);
    let text_entry_btns_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    // Event bindings for text entry buttons are handled in bind_events()
    text_entry_btns_sizer.add(
        &get_text_btn,
        0,
        SizerFlag::AlignCenterVertical | SizerFlag::All,
        2,
    );
    text_entry_btns_sizer.add(
        &get_password_btn,
        0,
        SizerFlag::AlignCenterVertical | SizerFlag::All,
        2,
    );
    text_entry_btns_sizer.add_spacer(10);
    text_entry_btns_sizer.add(
        &text_entry_status_label,
        1,
        SizerFlag::Expand | SizerFlag::All,
        2,
    );
    grid_sizer.add_sizer(&text_entry_btns_sizer, 1, SizerFlag::Expand, 0);

    // Add Colour Dialog controls
    grid_sizer.add(&colour_dialog_label, 0, label_flags, 0);
    let colour_dialog_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    // Event binding for colour dialog button is handled in bind_events()
    colour_dialog_sizer.add(
        &choose_colour_btn,
        0,
        SizerFlag::AlignCenterVertical | SizerFlag::All,
        2,
    );
    colour_dialog_sizer.add_spacer(10);
    colour_dialog_sizer.add(
        &colour_dialog_status_label,
        1,
        SizerFlag::Expand | SizerFlag::All,
        2,
    );
    grid_sizer.add_sizer(&colour_dialog_sizer, 1, SizerFlag::Expand, 0);

    // Add FontDialog controls
    grid_sizer.add(&font_dialog_label, 0, label_flags, 0);
    let font_dialog_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    // Event binding for font dialog button is handled in bind_events()
    font_dialog_sizer.add(
        &font_button,
        0,
        SizerFlag::AlignCenterVertical | SizerFlag::All,
        2,
    );
    font_dialog_sizer.add_spacer(10);
    font_dialog_sizer.add(&font_sample_text, 1, SizerFlag::Expand | SizerFlag::All, 2);
    grid_sizer.add_sizer(&font_dialog_sizer, 1, SizerFlag::Expand, 0);

    // Add ProgressDialog controls
    grid_sizer.add(&progress_dialog_label, 0, label_flags, 0);
    let progress_dialog_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    // Event binding for progress dialog button is handled in bind_events()
    progress_dialog_sizer.add(
        &progress_button,
        0,
        SizerFlag::AlignCenterVertical | SizerFlag::All,
        2,
    );
    grid_sizer.add_sizer(&progress_dialog_sizer, 1, SizerFlag::Expand, 0);

    // Add FilePickerCtrl controls
    grid_sizer.add(&file_picker_ctrl_label, 0, label_flags, 0);
    let fpc_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    // Event binding for file picker control is handled in bind_events()
    fpc_sizer.add(&file_picker_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 2);
    fpc_sizer.add_spacer(5); // Small spacer
    fpc_sizer.add(
        &file_picker_status_label,
        1,
        SizerFlag::Expand | SizerFlag::All,
        2,
    );
    grid_sizer.add_sizer(&fpc_sizer, 1, SizerFlag::Expand, 0);

    // --- DirPickerCtrl ---
    let dir_picker_ctrl_label = StaticText::builder(&dialog_panel)
        .with_label("Dir Picker Ctrl:")
        .build();
    let dir_picker_ctrl = DirPickerCtrl::builder(&dialog_panel)
        .with_style(DirPickerCtrlStyle::Default | DirPickerCtrlStyle::DirMustExist)
        .with_message("Choose a directory")
        .build();
    let dir_picker_status_label = StaticText::builder(&dialog_panel)
        .with_label("DirPicker: No directory selected yet.")
        .build();

    grid_sizer.add(&dir_picker_ctrl_label, 0, label_flags, 0);
    let dpc_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    // Event binding for directory picker control is handled in bind_events()
    dpc_sizer.add(&dir_picker_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 2);
    dpc_sizer.add_spacer(5);
    dpc_sizer.add(
        &dir_picker_status_label,
        1,
        SizerFlag::Expand | SizerFlag::All,
        2,
    );
    grid_sizer.add_sizer(&dpc_sizer, 1, SizerFlag::Expand, 0);

    // --- FontPickerCtrl ---
    let font_picker_ctrl_label = StaticText::builder(&dialog_panel)
        .with_label("Font Picker Ctrl:")
        .build();

    // Create a fresh font specifically for the FontPickerCtrl to avoid sharing
    let font_for_picker = Font::builder()
        .with_point_size(12)
        .with_face_name("Swiss")
        .build();

    let font_picker_ctrl = FontPickerCtrl::builder(&dialog_panel)
        .with_initial_font(font_for_picker)
        .build();
    let font_picker_status_label = StaticText::builder(&dialog_panel)
        .with_label("Font: Default")
        .build();

    // Event binding for font picker control is handled in bind_events()

    // Add the label to the grid sizer directly like other rows
    grid_sizer.add(&font_picker_ctrl_label, 0, label_flags, 0);

    // Create a horizontal sizer for the controls only
    let font_pc_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    font_pc_sizer.add(&font_picker_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 2);
    font_pc_sizer.add_spacer(5);
    // Use the original font_picker_status_label here, not a clone
    font_pc_sizer.add(
        &font_picker_status_label,
        1,
        SizerFlag::Expand | SizerFlag::All,
        2,
    );
    grid_sizer.add_sizer(&font_pc_sizer, 1, SizerFlag::Expand, 0);

    // --- NotificationMessage ---
    let notification_label = StaticText::builder(&dialog_panel)
        .with_label("Notification:")
        .build();
    let show_notification_btn = Button::builder(&dialog_panel)
        .with_label("Show Notification")
        .build();
    let notification_status_label = StaticText::builder(&dialog_panel)
        .with_label("Notification Status: -")
        .build();

    grid_sizer.add(&notification_label, 0, label_flags, 0);
    let notification_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    // Event binding for notification button is handled in bind_events()
    notification_sizer.add(
        &show_notification_btn,
        0,
        SizerFlag::AlignCenterVertical | SizerFlag::All,
        2,
    );
    notification_sizer.add_spacer(10);
    notification_sizer.add(
        &notification_status_label,
        1,
        SizerFlag::Expand | SizerFlag::All,
        2,
    );
    grid_sizer.add_sizer(&notification_sizer, 1, SizerFlag::Expand, 0);

    // --- SingleChoiceDialog ---
    let single_choice_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("Single Choice Dialog:")
        .build();
    let show_single_choice_dialog_btn = Button::builder(&dialog_panel)
        .with_label("Show Choices")
        .build();
    show_single_choice_dialog_btn.set_tooltip("Click to show a single choice dialog.");
    let single_choice_dialog_status_label = StaticText::builder(&dialog_panel)
        .with_label("Choice: None yet")
        .build();

    grid_sizer.add(&single_choice_dialog_label, 0, label_flags, 0);
    let single_choice_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    // Event binding will be handled in bind_events()
    single_choice_sizer.add(
        &show_single_choice_dialog_btn,
        0,
        SizerFlag::AlignCenterVertical | SizerFlag::All,
        2,
    );
    single_choice_sizer.add_spacer(10);
    single_choice_sizer.add(
        &single_choice_dialog_status_label,
        1,
        SizerFlag::Expand | SizerFlag::All,
        2,
    );
    grid_sizer.add_sizer(&single_choice_sizer, 1, SizerFlag::Expand, 0);

    // --- MultiChoiceDialog ---
    let multi_choice_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("Multi Choice Dialog:")
        .build();
    let show_multi_choice_dialog_btn = Button::builder(&dialog_panel)
        .with_label("Show Multi Choices")
        .build();
    show_multi_choice_dialog_btn.set_tooltip("Click to show a multi choice dialog.");
    let multi_choice_dialog_status_label = StaticText::builder(&dialog_panel)
        .with_label("Choices: None yet")
        .build();

    grid_sizer.add(&multi_choice_dialog_label, 0, label_flags, 0);
    let multi_choice_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    // Event binding will be handled in bind_events()
    multi_choice_sizer.add(
        &show_multi_choice_dialog_btn,
        0,
        SizerFlag::AlignCenterVertical | SizerFlag::All,
        2,
    );
    multi_choice_sizer.add_spacer(10);
    multi_choice_sizer.add(
        &multi_choice_dialog_status_label,
        1,
        SizerFlag::Expand | SizerFlag::All,
        2,
    );
    grid_sizer.add_sizer(&multi_choice_sizer, 1, SizerFlag::Expand, 0);

    // Create a button for directory dialog
    let dir_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("Dir Dialog:")
        .build();
    let dlg_dir_button = Button::builder(&dialog_panel)
        .with_label("Choose Directory")
        .build();

    // Add the directory dialog to the grid sizer
    grid_sizer.add(&dir_dialog_label, 0, label_flags, 0);
    let dir_dialog_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    dir_dialog_sizer.add(
        &dlg_dir_button,
        0,
        SizerFlag::AlignCenterVertical | SizerFlag::All,
        2,
    );
    grid_sizer.add_sizer(&dir_dialog_sizer, 1, SizerFlag::Expand, 0);

    main_sizer.add_sizer(&grid_sizer, 1, SizerFlag::Expand | SizerFlag::All, 10);
    dialog_panel.set_sizer(main_sizer, true);
    dialog_panel.fit(); // Fit the panel to its contents

    DialogTabControls {
        panel: dialog_panel,
        show_msg_dialog_btn,
        open_file_btn,
        save_file_btn,
        file_dialog_status_label,
        get_text_btn,
        get_password_btn,
        text_entry_status_label,
        choose_colour_btn,
        colour_dialog_status_label,
        font_button,
        font_sample_text,
        progress_button,
        file_picker_ctrl_label,
        file_picker_ctrl,
        file_picker_status_label,
        dir_picker_ctrl_label,
        dir_picker_ctrl,
        dir_picker_status_label,
        font_picker_ctrl_label,
        font_picker_ctrl,
        font_picker_status_label,
        show_notification_btn,
        notification_status_label,
        show_single_choice_dialog_btn,
        single_choice_dialog_status_label,
        show_multi_choice_dialog_btn,
        multi_choice_dialog_status_label,
        dlg_dir_button,
    }
}

impl DialogTabControls {
    pub fn bind_events(&self, frame: &Frame) {
        // Message Dialog button
        let show_msg_btn_clone = self.show_msg_dialog_btn.clone();
        self.show_msg_dialog_btn.on_click(move |_event| {
            MessageDialog::builder(
                &show_msg_btn_clone,                // Parent
                "This is a sample message dialog.", // Message
                "Message Dialog Title",             // Caption
            )
            .with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation)
            .build()
            .show_modal();
        });

        // File Dialog buttons
        let frame_clone_open = frame.clone();
        let status_label_clone_open = self.file_dialog_status_label.clone();
        self.open_file_btn.on_click(move |_event| {
            let dialog = FileDialog::builder(&frame_clone_open)
                .with_message("Choose a file to open")
                .with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist)
                .with_default_dir(".")
                .with_wildcard(
                    "Rust files (*.rs)|*.rs|Text files (*.txt)|*.txt|All files (*.*)|*.*",
                )
                .build();
            if dialog.show_modal() == wxdragon::id::ID_OK {
                let path_option = dialog.get_path();
                let path_str = path_option.unwrap_or_else(|| "(None)".to_string());
                status_label_clone_open.set_label(&format!("Opened: {path_str}"));
                println!("File Dialog: Opened file: {path_str}");
            } else {
                status_label_clone_open.set_label("Open cancelled.");
                println!("File Dialog: Open cancelled.");
            }
        });

        let frame_clone_save = frame.clone();
        let status_label_clone_save = self.file_dialog_status_label.clone();
        self.save_file_btn.on_click(move |_event| {
            let dialog = FileDialog::builder(&frame_clone_save)
                .with_message("Choose a file to save to")
                .with_style(FileDialogStyle::Save | FileDialogStyle::OverwritePrompt)
                .with_default_dir(".")
                .with_default_file("my_document.txt")
                .with_wildcard("Text files (*.txt)|*.txt|All files (*.*)|*.*")
                .build();
            if dialog.show_modal() == wxdragon::id::ID_OK {
                let path_option = dialog.get_path();
                let path_str = path_option.unwrap_or_else(|| "(None)".to_string());
                status_label_clone_save.set_label(&format!("Saved to: {path_str}"));
                println!("File Dialog: Saved to file: {path_str}");
            } else {
                status_label_clone_save.set_label("Save cancelled.");
                println!("File Dialog: Save cancelled.");
            }
        });

        // Text Entry Dialog buttons
        let frame_clone_text = frame.clone();
        let status_label_clone_text = self.text_entry_status_label.clone();
        self.get_text_btn.on_click(move |_event| {
            let dialog = TextEntryDialog::builder(
                &frame_clone_text,
                "Please enter your name:",
                "Text Input",
            )
            .with_default_value("wxDragon User")
            .build();
            if dialog.show_modal() == wxdragon::id::ID_OK {
                if let Some(text) = dialog.get_value() {
                    status_label_clone_text.set_label(&format!("Entered: {text}"));
                    println!("Text Entry Dialog: Entered text: {text}");
                } else {
                    status_label_clone_text.set_label("Input: No value.");
                    println!("Text Entry Dialog: Input: No value returned.");
                }
            } else {
                status_label_clone_text.set_label("Input cancelled.");
                println!("Text Entry Dialog: Input cancelled.");
            }
        });

        let frame_clone_pass = frame.clone();
        let status_label_clone_pass = self.text_entry_status_label.clone();
        self.get_password_btn.on_click(move |_event| {
            let dialog = TextEntryDialog::builder(
                &frame_clone_pass,
                "Please enter your password:",
                "Password Input",
            )
            .with_style(
                TextEntryDialogStyle::Password
                    | TextEntryDialogStyle::Centre
                    | TextEntryDialogStyle::Ok
                    | TextEntryDialogStyle::Cancel,
            )
            .build();
            if dialog.show_modal() == wxdragon::id::ID_OK {
                if let Some(text) = dialog.get_value() {
                    status_label_clone_pass
                        .set_label(&format!("Password entered (length: {})", text.len()));
                    println!(
                        "Text Entry Dialog: Password entered (length: {}). Value: 'REDACTED'",
                        text.len()
                    );
                } else {
                    status_label_clone_pass.set_label("Password input: No value.");
                    println!("Text Entry Dialog: Password input: No value returned.");
                }
            } else {
                status_label_clone_pass.set_label("Password input cancelled.");
                println!("Text Entry Dialog: Password input cancelled.");
            }
        });

        // Colour Dialog button
        let frame_clone_colour = frame.clone();
        let status_label_clone_colour = self.colour_dialog_status_label.clone();
        let colour_sample_panel_clone = self.colour_dialog_status_label.clone();
        self.choose_colour_btn.on_click(move |_event| {
            let dialog = ColourDialog::builder(&frame_clone_colour)
                .with_title("Select a Colour")
                .with_initial_colour(colours::BLUE)
                .build();
            if dialog.show_modal() == wxdragon::id::ID_OK {
                if let Some(colour) = dialog.get_colour() {
                    status_label_clone_colour.set_label(&format!("Chosen: {colour:?}"));
                    colour_sample_panel_clone.set_background_color(colour);
                    colour_sample_panel_clone.refresh(true, None);
                    println!("Colour Dialog: Chosen colour: {colour:?}");
                }
            } else {
                status_label_clone_colour.set_label("Colour choice cancelled.");
                println!("Colour Dialog: Colour choice cancelled.");
            }
        });

        // Font Dialog Button
        let frame_clone_font = frame.clone();
        let font_sample_text_clone = self.font_sample_text.clone();
        self.font_button.on_click(move |_event| {
            let font_dialog = FontDialog::builder(&frame_clone_font).build();
            if font_dialog.show_modal() == wxdragon::id::ID_OK {
                if let Some(font) = font_dialog.get_font() {
                    // Create a copy of the font before setting it on the text control
                    let font_copy = font.to_owned();
                    font_sample_text_clone.set_font(&font_copy);
                    font_sample_text_clone.set_label("Font Changed!");
                    println!(
                        "Font Dialog: Font chosen: {:?}, Family: {:?}, Size: {}, Style: {:?}, Weight: {:?}",
                        font_copy.get_face_name(),
                        font_copy.get_family(),
                        font_copy.get_point_size(),
                        font_copy.get_style(),
                        font_copy.get_weight()
                    );
                } else {
                    println!("Font Dialog: No font chosen or error retrieving font.");
                }
            } else {
                println!("Font Dialog: Font choice cancelled.");
            }
        });

        // Progress Dialog button
        let frame_clone_progress = frame.clone();
        self.progress_button.on_click(move |_event| {
            let dialog =
                ProgressDialog::builder(&frame_clone_progress, "Processing...", "Please wait", 100)
                    .with_style(
                        ProgressDialogStyle::CanAbort
                            | ProgressDialogStyle::Smooth
                            | ProgressDialogStyle::AutoHide,
                    )
                    .build();

            let mut keep_going = true;
            for i in 0..=100 {
                thread::sleep(Duration::from_millis(50));
                if i == 50 {
                    keep_going = dialog.update(i, Some("Halfway there!"));
                } else {
                    keep_going = dialog.update(i, None);
                }
                if !keep_going {
                    println!("Progress Dialog: Aborted by user at {i}%");
                    break;
                }
            }
            if keep_going {
                println!("Progress Dialog: Completed.");
            }
        });

        // NotificationMessage button
        let frame_clone_notify = frame.clone();
        let status_label_clone_notify = self.notification_status_label.clone();
        self.show_notification_btn.on_click(move |_event| {
            let notification_result = NotificationMessage::builder()
                .with_title("wxDragon Notification")
                .with_message("This is a notification message from the gallery example!")
                .with_parent(&frame_clone_notify)
                .with_style(NotificationStyle::Information)
                .build();

            match notification_result {
                Ok(notification) => {
                    notification.show(wxdragon::widgets::notification_message::TIMEOUT_AUTO);
                    status_label_clone_notify.set_label("Notification shown.");
                    println!("NotificationMessage: Shown.");
                    NOTIFICATION_MESSAGE.lock().unwrap().replace(notification);
                }
                Err(e) => {
                    status_label_clone_notify.set_label(&format!("Notify Err: {e:?}"));
                    println!("NotificationMessage Error: {e:?}");
                }
            }
        });

        // Event handler for FilePickerCtrl
        let fpc_status_label_clone = self.file_picker_status_label.clone();
        let fpc_clone = self.file_picker_ctrl.clone();
        self.file_picker_ctrl.on_file_changed(move |_event| {
            let path = fpc_clone.get_path();
            fpc_status_label_clone.set_label(&format!("FilePicker Path: {path}"));
            println!("FilePickerCtrl changed: {path}");
        });

        // Event handler for DirPickerCtrl
        let dpc_status_label_clone = self.dir_picker_status_label.clone();
        let dpc_clone = self.dir_picker_ctrl.clone();
        self.dir_picker_ctrl.on_dir_changed(move |_event| {
            let path = dpc_clone.get_path();
            dpc_status_label_clone.set_label(&format!("DirPicker Path: {path}"));
            println!("DirPickerCtrl changed: {path}");
        });

        // Event handler for FontPickerCtrl
        let font_pc_status_label_clone = self.font_picker_status_label.clone();
        let font_picker_ctrl_clone = self.font_picker_ctrl.clone();
        self.font_picker_ctrl.on_font_changed(move |_event| {
            let selected_font = font_picker_ctrl_clone.get_selected_font();
            let mut font_desc = "FontPicker: No font selected".to_string();
            if let Some(font) = selected_font {
                let font_desc_base = format!(
                    "{}, {}pt, {:?}, {:?}",
                    font.get_face_name(),
                    font.get_point_size(),
                    font.get_style(),
                    font.get_weight()
                );
                font_desc = format!("Font: {font_desc_base}");
            }
            font_pc_status_label_clone.set_label(&font_desc);
            println!("{font_desc}");
        });

        // SingleChoiceDialog button
        let frame_clone_choice = frame.clone();
        let status_label_clone_choice = self.single_choice_dialog_status_label.clone();
        self.show_single_choice_dialog_btn.on_click(move |_event| {
            let choices = [
                "Red", "Green", "Blue", "Yellow", "Purple", "Orange", "Black", "White",
            ];

            let dialog = SingleChoiceDialog::builder(
                &frame_clone_choice,
                "Please select a color:",
                "Color Choice",
                &choices,
            )
            .build();

            if dialog.show_modal() == wxdragon::id::ID_OK {
                if let Some(selection) = dialog.get_string_selection() {
                    status_label_clone_choice.set_label(&format!("Choice: {selection}"));
                    println!("SingleChoiceDialog: Selected '{selection}'");

                    // Alternatively, get selection index
                    let index = dialog.get_selection();
                    println!("SingleChoiceDialog: Selected index {index}");
                } else {
                    status_label_clone_choice.set_label("Choice: None selected");
                    println!("SingleChoiceDialog: No selection retrieved");
                }
            } else {
                status_label_clone_choice.set_label("Choice: Cancelled");
                println!("SingleChoiceDialog: Cancelled");
            }
        });

        // MultiChoiceDialog button
        let frame_clone_multi = frame.clone();
        let status_label_clone_multi = self.multi_choice_dialog_status_label.clone();
        self.show_multi_choice_dialog_btn.on_click(move |_event| {
            let choices = [
                "Red", "Green", "Blue", "Yellow", "Purple", "Orange", "Black", "White",
            ];

            let dialog = MultiChoiceDialog::builder(
                &frame_clone_multi,
                "Please select one or more colors:",
                "Multiple Color Choices",
                &choices,
            )
            .build();

            // Set some initial selections if desired
            dialog.set_selections(&[0, 2]); // Select Red and Blue initially

            if dialog.show_modal() == wxdragon::id::ID_OK {
                let selections = dialog.get_selections();
                let string_selections = dialog.get_string_selections();

                if !selections.is_empty() {
                    let indices_str = selections
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    let strings_str = string_selections.join(", ");

                    status_label_clone_multi.set_label(&format!("Choices: {strings_str}"));
                    println!("MultiChoiceDialog: Selected indices [{indices_str}]");
                    println!("MultiChoiceDialog: Selected strings [{strings_str}]");
                } else {
                    status_label_clone_multi.set_label("Choices: None selected");
                    println!("MultiChoiceDialog: No selections made");
                }
            } else {
                status_label_clone_multi.set_label("Choices: Cancelled");
                println!("MultiChoiceDialog: Cancelled");
            }
        });

        // Dir Dialog Button
        let frame_clone_dir = frame.clone();
        self.dlg_dir_button.on_click(move |_| {
            let dialog = DirDialog::builder(&frame_clone_dir, "Choose a directory", "")
                .with_style(DirDialogStyle::Default as i64 | DirDialogStyle::MustExist as i64)
                .build();

            if dialog.show_modal() == wxdragon::id::ID_OK {
                if let Some(path) = dialog.get_path() {
                    println!("Directory: {path}");
                } else {
                    println!("Directory: None selected");
                }
            } else {
                println!("Directory Dialog canceled");
            }
        });

        frame.on_destroy(move |_| {
            let _ = NOTIFICATION_MESSAGE.lock().unwrap().take();
        });
    }
}
