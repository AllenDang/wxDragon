use wxdragon::prelude::*;

/// A custom data type to associate with tree items
#[derive(Debug, Clone)]
struct PersonData {
    name: String,
    age: u32,
    role: String,
}

/// Helper methods for displaying PersonData
impl PersonData {
    /// Creates a display string for the data
    fn to_display_string(&self) -> String {
        format!("Person: {}\nAge: {}\nRole: {}", self.name, self.age, self.role)
    }
}

/// Another custom data type to demonstrate multiple types
#[derive(Debug, Clone)]
struct ProjectData {
    name: String,
    priority: u32,
    deadline: String,
}

/// Helper methods for displaying ProjectData
impl ProjectData {
    /// Creates a display string for the data
    fn to_display_string(&self) -> String {
        format!(
            "Project: {}\nPriority: {}\nDeadline: {}",
            self.name, self.priority, self.deadline
        )
    }
}

pub struct TreeCtrlTabControls {
    pub panel: Panel,
    pub tree_ctrl: TreeCtrl,
    pub info_text: StaticText,
}

impl TreeCtrlTabControls {
    pub fn bind_events(&self) {
        // Widgets are Copy, so copy them for use in closures
        let tree_ctrl = self.tree_ctrl;
        let info_text = self.info_text;

        // Bind selection changed event for tree control
        self.tree_ctrl.on_selection_changed(move |event_data| {
            if let Some(item_id) = event_data.get_item() {
                // Get data from the selected item
                if let Some(item_data) = tree_ctrl.get_custom_data(&item_id) {
                    // Try to downcast to PersonData first
                    if let Some(person) = item_data.downcast_ref::<PersonData>() {
                        info_text.set_label(&person.to_display_string());
                    }
                    // Try to downcast to ProjectData
                    else if let Some(project) = item_data.downcast_ref::<ProjectData>() {
                        info_text.set_label(&project.to_display_string());
                    }
                    // Handle standard types
                    else if let Some(text) = item_data.downcast_ref::<String>() {
                        info_text.set_label(&format!("Text: {text}"));
                    } else if let Some(number) = item_data.downcast_ref::<i32>() {
                        info_text.set_label(&format!("Number: {number}"));
                    } else if item_data.downcast_ref::<()>().is_some() {
                        info_text.set_label("This item has empty data (unit type)");
                    } else {
                        // If we can't determine the type specifically, show a generic message
                        info_text.set_label("Item has data of an unknown type");
                    }
                } else {
                    info_text.set_label("Item has no associated data");
                }
            }
        });

        // Bind item activation (double-click) event
        self.tree_ctrl.on_item_activated(move |event_data| {
            if let Some(item_id) = event_data.get_item() {
                if tree_ctrl.has_custom_data(&item_id) {
                    info_text.set_label("Double-clicked on item with custom data");
                } else {
                    info_text.set_label("Double-clicked on item with no data");
                }
            }
        });
    }
}

pub fn create_treectrl_tab(parent: &Notebook) -> TreeCtrlTabControls {
    // Create the main panel
    let panel = Panel::builder(parent).with_style(PanelStyle::TabTraversal).build();

    // Create the tree control with some styles
    let tree_ctrl = TreeCtrl::builder(&panel)
        .with_style(TreeCtrlStyle::HasButtons | TreeCtrlStyle::LinesAtRoot)
        .build();

    // --- ImageList Setup ---
    let image_list = ImageList::new(16, 16, true, 5);
    let mut icons: Vec<i32> = Vec::new();

    // Define icons (ensure these ArtIds are valid and available)
    let art_ids = [
        ArtId::Folder,
        ArtId::FolderOpen,
        ArtId::NormalFile,
        ArtId::Information,
        ArtId::Error,
    ];
    for art_id in art_ids.iter() {
        if let Some(bmp) = ArtProvider::get_bitmap(*art_id, ArtClient::FrameIcon, Some(Size::new(16, 16))) {
            icons.push(image_list.add_bitmap(&bmp));
        } else {
            println!("Failed to load icon {art_id:?} for TreeCtrl ImageList");
            icons.push(-1); // Placeholder if icon loading fails
        }
    }
    // Icon indices: 0:Folder, 1:OpenedFolder, 2:File, 3:Info, 4:Error
    tree_ctrl.set_image_list(image_list);
    // --- End ImageList Setup ---

    // Create info text control to display data
    let info_text = StaticText::builder(&panel)
        .with_label("Select a tree item to see its data")
        .build();

    // Populate the tree with example data

    // 1. Create root item with PersonData
    let ceo_data = PersonData {
        name: "John Smith".to_string(),
        age: 52,
        role: "CEO".to_string(),
    };
    let root_id = tree_ctrl
        .add_root_with_data("Company Hierarchy", ceo_data, Some(icons[0]), Some(icons[1])) // Folder icons
        .unwrap();

    // 2. Add departments with different data types

    // Engineering department with ProjectData
    let eng_project = ProjectData {
        name: "Engineering Department".to_string(),
        priority: 1,
        deadline: "2024-12-31".to_string(),
    };
    let eng_id = tree_ctrl
        .append_item_with_data(&root_id, "Engineering", eng_project, Some(icons[0]), Some(icons[1])) // Folder icons
        .unwrap();

    // Add engineers with PersonData
    let eng_lead = PersonData {
        name: "Alice Johnson".to_string(),
        age: 38,
        role: "Lead Engineer".to_string(),
    };
    tree_ctrl
        .append_item_with_data(&eng_id, "Alice Johnson", eng_lead, Some(icons[2]), Some(icons[2])) // File icon
        .unwrap();

    let dev1 = PersonData {
        name: "Bob Williams".to_string(),
        age: 29,
        role: "Software Developer".to_string(),
    };
    tree_ctrl
        .append_item_with_data(&eng_id, "Bob Williams", dev1, Some(icons[2]), Some(icons[2])) // File icon
        .unwrap();
    tree_ctrl.expand(&eng_id);

    let dev2 = PersonData {
        name: "Carol Davis".to_string(),
        age: 32,
        role: "QA Engineer".to_string(),
    };
    tree_ctrl
        .append_item_with_data(&eng_id, "Carol Davis", dev2, Some(icons[2]), Some(icons[2])) // File icon
        .unwrap();

    // Marketing department with String data
    let marketing_id = tree_ctrl
        .append_item_with_data(
            &root_id,
            "Marketing",
            "Marketing department handles all promotional activities.".to_string(),
            Some(icons[0]),
            Some(icons[1]), // Folder icons
        )
        .unwrap();

    // Add marketing staff with mixed data types
    tree_ctrl
        .append_item_with_data(
            &marketing_id,
            "David Wilson",
            PersonData {
                name: "David Wilson".to_string(),
                age: 41,
                role: "Marketing Director".to_string(),
            },
            Some(icons[2]),
            Some(icons[2]), // File icon
        )
        .unwrap();

    tree_ctrl
        .append_item_with_data(
            &marketing_id,
            "Current Campaign",
            ProjectData {
                name: "Summer Sale".to_string(),
                priority: 2,
                deadline: "2024-08-31".to_string(),
            },
            Some(icons[3]),
            Some(icons[3]), // Info icon
        )
        .unwrap();

    // Finance department with i32 data (budget in thousands)
    let finance_id = tree_ctrl
        .append_item_with_data(&root_id, "Finance", 250, Some(icons[0]), Some(icons[1])) // Folder icons
        .unwrap();

    // Add finance staff
    let finance_lead = PersonData {
        name: "Eve Brown".to_string(),
        age: 45,
        role: "CFO".to_string(),
    };
    tree_ctrl
        .append_item_with_data(&finance_id, "Eve Brown", finance_lead, Some(icons[2]), Some(icons[2])) // File icon
        .unwrap();
    tree_ctrl.expand(&finance_id);

    // Expand the root item to show the structure
    tree_ctrl.select_item(&root_id);

    // Create sizers for layout
    let main_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    // Left side: Tree control
    let tree_sizer = BoxSizer::builder(Orientation::Vertical).build();
    tree_sizer.add(&tree_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 10);

    // Right side: Info panel
    let info_sizer = BoxSizer::builder(Orientation::Vertical).build();
    let info_title = StaticText::builder(&panel).with_label("Item Information:").build();

    info_sizer.add(&info_title, 0, SizerFlag::All, 5);
    info_sizer.add(&info_text, 1, SizerFlag::Expand | SizerFlag::All, 10);

    // Add both sides to main sizer
    main_sizer.add_sizer(&tree_sizer, 3, SizerFlag::Expand | SizerFlag::All, 10);
    main_sizer.add_sizer(&info_sizer, 2, SizerFlag::Expand | SizerFlag::All, 10);

    // Set the panel's sizer
    panel.set_sizer(main_sizer, true);

    tree_ctrl.expand(&root_id);

    // Return the controls
    TreeCtrlTabControls {
        panel,
        tree_ctrl,
        info_text,
    }
}
