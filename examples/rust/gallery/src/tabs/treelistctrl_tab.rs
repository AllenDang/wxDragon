use wxdragon::prelude::*;
use wxdragon::widgets::list_ctrl::ListColumnFormat;
use wxdragon::widgets::treelistctrl::{
    CheckboxState, TreeListCtrl, TreeListCtrlEventData, TreeListCtrlStyle,
};

pub struct TreeListCtrlTabControls {
    pub panel: Panel,
    pub tree_list_ctrl: TreeListCtrl,
    pub info_text: StaticText,
    pub status_text: StaticText,
}

impl TreeListCtrlTabControls {
    pub fn bind_events(&self) {
        let tree_list_ctrl_clone = self.tree_list_ctrl.clone();
        let info_text_clone = self.info_text.clone();
        let status_text_clone = self.status_text.clone();

        // Bind selection changed event
        self.tree_list_ctrl
            .on_selection_changed(move |event: TreeListCtrlEventData| {
                if let Some(item) = event.get_item() {
                    let name = tree_list_ctrl_clone.get_item_text(&item, 0);
                    let size = tree_list_ctrl_clone.get_item_text(&item, 1);
                    let type_str = tree_list_ctrl_clone.get_item_text(&item, 2);
                    let modified = tree_list_ctrl_clone.get_item_text(&item, 3);

                    let info = format!(
                        "Selected Item:\nName: {}\nSize: {}\nType: {}\nModified: {}",
                        name, size, type_str, modified
                    );
                    info_text_clone.set_label(&info);
                    // Only update status if it's not already showing a checkbox state update
                    let current_status = status_text_clone.get_label();
                    if !current_status.contains("is now checked")
                        && !current_status.contains("is now unchecked")
                    {
                        status_text_clone.set_label(&format!("Selected: {}", name));
                    }
                }
            });

        // Bind checkbox events
        let tree_list_ctrl_clone2 = self.tree_list_ctrl.clone();
        let status_text_clone2 = self.status_text.clone();

        self.tree_list_ctrl
            .on_item_checked(move |event: TreeListCtrlEventData| {
                if let Some(item) = event.get_item() {
                    let name = tree_list_ctrl_clone2.get_item_text(&item, 0);

                    // Always use the actual current state from the widget, not the event
                    // because the event's is_checked() method can be unreliable
                    let current_state = tree_list_ctrl_clone2.get_checked_state(&item);
                    let state_text = match current_state {
                        CheckboxState::Checked => "checked",
                        CheckboxState::Unchecked => "unchecked",
                        CheckboxState::Undetermined => "undetermined",
                    };
                    status_text_clone2.set_label(&format!("Item '{}' is now {}", name, state_text));
                }
            });

        // Bind column sorted event
        let tree_list_ctrl_clone3 = self.tree_list_ctrl.clone();
        let status_text_clone3 = self.status_text.clone();
        self.tree_list_ctrl
            .on_column_sorted(move |event: TreeListCtrlEventData| {
                if let Some(column) = event.get_column() {
                    let column_names = ["Name", "Size", "Type", "Modified"];
                    let column_name = column_names.get(column as usize).unwrap_or(&"Unknown");

                    // Get current sort state
                    if let Some((_sort_col, ascending)) = tree_list_ctrl_clone3.get_sort_column() {
                        let direction = if ascending { "ascending" } else { "descending" };
                        status_text_clone3.set_label(&format!(
                            "Sorting by {} ({}) - direction: {}",
                            column_name, column, direction
                        ));
                    } else {
                        status_text_clone3.set_label(&format!(
                            "Column {} ({}) clicked for sorting",
                            column_name, column
                        ));
                    }
                }
            });

        // Bind item activated (double-click) event
        let tree_list_ctrl_clone4 = self.tree_list_ctrl.clone();
        let info_text_clone2 = self.info_text.clone();

        self.tree_list_ctrl
            .on_item_activated(move |event: TreeListCtrlEventData| {
                if let Some(item) = event.get_item() {
                    let name = tree_list_ctrl_clone4.get_item_text(&item, 0);
                    info_text_clone2.set_label(&format!("Double-clicked on: {}", name));
                }
            });
    }
}

pub fn create_treelistctrl_tab(parent: &Notebook) -> TreeListCtrlTabControls {
    // Create the main panel
    let panel = Panel::builder(parent)
        .with_style(PanelStyle::TabTraversal)
        .build();

    // Create the tree list control with checkboxes
    let tree_list_ctrl = TreeListCtrl::builder(&panel)
        .with_style(TreeListCtrlStyle::Default | TreeListCtrlStyle::Checkbox)
        .build();

    // Add columns with different alignments
    tree_list_ctrl.append_column("Name", 200, ListColumnFormat::Left);
    tree_list_ctrl.append_column("Size", 100, ListColumnFormat::Right);
    tree_list_ctrl.append_column("Type", 120, ListColumnFormat::Left);
    tree_list_ctrl.append_column("Modified", 150, ListColumnFormat::Left);

    // Create info and status text controls
    let info_text = StaticText::builder(&panel)
        .with_label("Select a tree list item to see its details")
        .build();

    let status_text = StaticText::builder(&panel).with_label("Ready").build();

    // Populate the tree list with example data
    let root = tree_list_ctrl.get_root_item();

    // Documents folder
    let documents = tree_list_ctrl.append_item(&root, "Documents").unwrap();
    tree_list_ctrl.set_item_text(&documents, 1, "Folder");
    tree_list_ctrl.set_item_text(&documents, 2, "Folder");
    tree_list_ctrl.set_item_text(&documents, 3, "2024-01-15");

    // Add files to Documents
    let report = tree_list_ctrl
        .append_item(&documents, "Annual Report.pdf")
        .unwrap();
    tree_list_ctrl.set_item_text(&report, 1, "2.5 MB");
    tree_list_ctrl.set_item_text(&report, 2, "PDF Document");
    tree_list_ctrl.set_item_text(&report, 3, "2024-01-10");
    tree_list_ctrl.check_item(&report, CheckboxState::Checked);

    let presentation = tree_list_ctrl
        .append_item(&documents, "Presentation.pptx")
        .unwrap();
    tree_list_ctrl.set_item_text(&presentation, 1, "5.2 MB");
    tree_list_ctrl.set_item_text(&presentation, 2, "PowerPoint");
    tree_list_ctrl.set_item_text(&presentation, 3, "2024-01-12");

    let notes = tree_list_ctrl
        .append_item(&documents, "Meeting Notes.txt")
        .unwrap();
    tree_list_ctrl.set_item_text(&notes, 1, "1.2 KB");
    tree_list_ctrl.set_item_text(&notes, 2, "Text File");
    tree_list_ctrl.set_item_text(&notes, 3, "2024-01-14");
    tree_list_ctrl.check_item(&notes, CheckboxState::Checked);

    // Pictures folder
    let pictures = tree_list_ctrl.append_item(&root, "Pictures").unwrap();
    tree_list_ctrl.set_item_text(&pictures, 1, "Folder");
    tree_list_ctrl.set_item_text(&pictures, 2, "Folder");
    tree_list_ctrl.set_item_text(&pictures, 3, "2024-01-08");

    // Add images to Pictures
    let vacation = tree_list_ctrl.append_item(&pictures, "Vacation").unwrap();
    tree_list_ctrl.set_item_text(&vacation, 1, "Folder");
    tree_list_ctrl.set_item_text(&vacation, 2, "Folder");
    tree_list_ctrl.set_item_text(&vacation, 3, "2023-12-20");

    let beach = tree_list_ctrl.append_item(&vacation, "beach.jpg").unwrap();
    tree_list_ctrl.set_item_text(&beach, 1, "3.2 MB");
    tree_list_ctrl.set_item_text(&beach, 2, "JPEG Image");
    tree_list_ctrl.set_item_text(&beach, 3, "2023-12-20");

    let sunset = tree_list_ctrl.append_item(&vacation, "sunset.jpg").unwrap();
    tree_list_ctrl.set_item_text(&sunset, 1, "2.8 MB");
    tree_list_ctrl.set_item_text(&sunset, 2, "JPEG Image");
    tree_list_ctrl.set_item_text(&sunset, 3, "2023-12-20");
    tree_list_ctrl.check_item(&sunset, CheckboxState::Checked);

    let family = tree_list_ctrl.append_item(&pictures, "family.png").unwrap();
    tree_list_ctrl.set_item_text(&family, 1, "4.1 MB");
    tree_list_ctrl.set_item_text(&family, 2, "PNG Image");
    tree_list_ctrl.set_item_text(&family, 3, "2024-01-01");

    // Downloads folder
    let downloads = tree_list_ctrl.append_item(&root, "Downloads").unwrap();
    tree_list_ctrl.set_item_text(&downloads, 1, "Folder");
    tree_list_ctrl.set_item_text(&downloads, 2, "Folder");
    tree_list_ctrl.set_item_text(&downloads, 3, "2024-01-16");

    let installer = tree_list_ctrl
        .append_item(&downloads, "software-installer.exe")
        .unwrap();
    tree_list_ctrl.set_item_text(&installer, 1, "15.3 MB");
    tree_list_ctrl.set_item_text(&installer, 2, "Application");
    tree_list_ctrl.set_item_text(&installer, 3, "2024-01-16");

    let archive = tree_list_ctrl.append_item(&downloads, "data.zip").unwrap();
    tree_list_ctrl.set_item_text(&archive, 1, "125.7 MB");
    tree_list_ctrl.set_item_text(&archive, 2, "ZIP Archive");
    tree_list_ctrl.set_item_text(&archive, 3, "2024-01-15");

    // Expand some folders to show the structure
    tree_list_ctrl.expand(&documents);
    tree_list_ctrl.expand(&pictures);
    tree_list_ctrl.expand(&vacation);

    // Enable column sorting by setting an initial sort column (column 0 = Name, ascending)
    tree_list_ctrl.set_sort_column(0, true);

    // Create layout with sizers
    let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

    // Title
    let title = StaticText::builder(&panel)
        .with_label("TreeListCtrl Demo - File Explorer with Checkboxes")
        .build();
    main_sizer.add(&title, 0, SizerFlag::All | SizerFlag::AlignCentre, 5);

    // Horizontal layout for tree list and info panel
    let content_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    // Left side: Tree list control
    content_sizer.add(&tree_list_ctrl, 2, SizerFlag::Expand | SizerFlag::All, 10);

    // Right side: Info panel
    let info_sizer = BoxSizer::builder(Orientation::Vertical).build();

    let info_title = StaticText::builder(&panel)
        .with_label("Item Information:")
        .build();
    info_sizer.add(&info_title, 0, SizerFlag::All, 5);
    info_sizer.add(&info_text, 1, SizerFlag::Expand | SizerFlag::All, 5);

    let status_title = StaticText::builder(&panel).with_label("Status:").build();
    info_sizer.add(&status_title, 0, SizerFlag::All, 5);
    info_sizer.add(&status_text, 0, SizerFlag::Expand | SizerFlag::All, 5);

    // Add instructions
    let instructions = StaticText::builder(&panel)
        .with_label("Instructions:\n• Click items to select\n• Check/uncheck boxes\n• Double-click to activate\n• Click column headers to sort\n• Sorting toggles between ascending/descending")
        .build();
    info_sizer.add(&instructions, 0, SizerFlag::All, 5);

    content_sizer.add_sizer(&info_sizer, 1, SizerFlag::Expand | SizerFlag::All, 10);

    main_sizer.add_sizer(&content_sizer, 1, SizerFlag::Expand | SizerFlag::All, 5);

    // Set the panel's sizer
    panel.set_sizer(main_sizer, true);

    // Return the controls
    TreeListCtrlTabControls {
        panel,
        tree_list_ctrl,
        info_text,
        status_text,
    }
}
