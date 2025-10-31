use wxdragon::prelude::*;
use wxdragon::widgets::scrolled_window::ScrollBarConfig;

// Define a custom data type for our list items
#[derive(Clone)]
#[allow(dead_code)]
struct ProductInfo {
    sku: String,
    price: f64,
    in_stock: bool,
    reorder_level: i32,
}

#[allow(dead_code)]
pub struct ListsTabControls {
    pub panel: ScrolledWindow,
    pub list_box: ListBox,
    pub listbox_status_label: StaticText,
    pub checklistbox: CheckListBox,
    pub checklistbox_status_label: StaticText,
    pub choice_ctrl: Choice,
    pub choice_status_label: StaticText,
    pub combo_box: ComboBox,
    pub combo_status_label: StaticText,
    pub list_ctrl: ListCtrl,
    pub list_ctrl_status_label: StaticText,
    pub editable_listbox: EditableListBox,
    pub editable_listbox_status_label: StaticText,
    pub rearrangelist: RearrangeList,
    pub rearrangelist_status_label: StaticText,
}

pub fn create_lists_tab(notebook: &Notebook, _frame: &Frame) -> ListsTabControls {
    // Create the ScrolledWindow as the main container for this tab
    let scrolled_list_window = ScrolledWindow::builder(notebook)
        // .with_style(PanelStyle::TabTraversal) // Removed - ScrolledWindow doesn't take PanelStyle
        .build();

    // Main panel inside the scrolled window, this is where TAB_TRAVERSAL should apply if needed for its children
    let panel = Panel::builder(&scrolled_list_window)
        .with_style(PanelStyle::TabTraversal) // Panel takes PanelStyle
        .build();
    let _sizer = BoxSizer::builder(Orientation::Vertical).build();

    // --- Create controls, parenting them to the *inner_list_panel* ---
    let list_box_items = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Date".to_string(),
        "Elderberry".to_string(),
    ];
    let list_box = ListBox::builder(&panel)
        .with_choices(list_box_items)
        // .with_style(ListBoxStyle::Default | ListBoxStyle::Sort | ListBoxStyle::AlwaysScrollbar | ListBoxStyle::HorizontalScrollbar) // Old complex
        .with_style(ListBoxStyle::Sort) // Simplified to test
        .build();
    let listbox_status_label = StaticText::builder(&panel).with_label("List Selection: None").build();
    let checklistbox = CheckListBox::builder(&panel)
        .with_id(109)
        .with_choices(vec![
            "Option A".to_string(),
            "Option B".to_string(),
            "Option C".to_string(),
            "Option D".to_string(),
            "Option E".to_string(),
            "Option F".to_string(),
            "Option G".to_string(),
        ])
        // .with_style(LB_SORT) // Old
        .with_style(CheckListBoxStyle::Sort) // Correct type
        .build();
    checklistbox.check(1, true);
    let checklistbox_status_label = StaticText::builder(&panel)
        .with_label("CheckList Status: B (Checked)")
        .build();
    let choice_items = ["Red", "Green", "Blue", "Yellow", "Purple", "Orange", "Cyan", "Magenta"];
    let choice_ctrl = Choice::builder(&panel)
        .with_choices(choice_items.iter().map(|s| s.to_string()).collect())
        .with_style(ChoiceStyle::Sort) // Now we can use the enum
        .build();
    choice_ctrl.set_selection(0);
    let choice_status_label = StaticText::builder(&panel).with_label("Choice Selection: Red").build();
    let combo_items = [
        "Cabbage", "Carrot", "Cucumber", "Celery", "Broccoli", "Spinach", "Kale", "Lettuce",
    ];
    let combo_box = ComboBox::builder(&panel)
        .with_string_choices(&combo_items)
        // .with_style(CB_SORT) // Old
        .with_style(ComboBoxStyle::Sort | ComboBoxStyle::ProcessEnter) // Ensure ProcessEnter for TEXT_ENTER event
        .build();
    let combo_status_label = StaticText::builder(&panel).with_label("Combo Status: Initial Combo").build();

    // --- ADDED: ListCtrl Example ---
    let list_ctrl = ListCtrl::builder(&panel)
        .with_id(ID_HIGHEST + 7) // ID_LIST_CTRL
        // Enable in-place label editing to allow calling edit_label without assertions on MSW
        .with_style(
            ListCtrlStyle::Report
                | ListCtrlStyle::SingleSel
                | ListCtrlStyle::HRules
                | ListCtrlStyle::VRules
                | ListCtrlStyle::EditLabels,
        )
        .build();

    // Add columns
    list_ctrl.insert_column(0, "ID", ListColumnFormat::Right, 60);
    list_ctrl.insert_column(1, "Description", ListColumnFormat::Left, 150);
    list_ctrl.insert_column(2, "Quantity", ListColumnFormat::Right, 100);
    list_ctrl.insert_column(3, "Notes", ListColumnFormat::Left, -1); // Fill remaining space

    // --- ImageList Setup for ListCtrl ---
    let list_ctrl_image_list = ImageList::new(16, 16, true, 3);
    let mut list_ctrl_icons: Vec<i32> = Vec::new();
    if let Some(bmp) = ArtProvider::get_bitmap(ArtId::NormalFile, ArtClient::FrameIcon, Some(Size::new(16, 16))) {
        list_ctrl_icons.push(list_ctrl_image_list.add_bitmap(&bmp));
    } else {
        list_ctrl_icons.push(-1);
    } // 0: File
    if let Some(bmp) = ArtProvider::get_bitmap(ArtId::Folder, ArtClient::FrameIcon, Some(Size::new(16, 16))) {
        list_ctrl_icons.push(list_ctrl_image_list.add_bitmap(&bmp));
    } else {
        list_ctrl_icons.push(-1);
    } // 1: Folder
    if let Some(bmp) = ArtProvider::get_bitmap(ArtId::Information, ArtClient::FrameIcon, Some(Size::new(16, 16))) {
        list_ctrl_icons.push(list_ctrl_image_list.add_bitmap(&bmp));
    } else {
        list_ctrl_icons.push(-1);
    } // 2: Info

    list_ctrl.set_image_list(list_ctrl_image_list, image_list_type::SMALL);
    // --- End ImageList Setup ---

    // Insert items (only sets the text for column 0)
    let _item1_idx = list_ctrl.insert_item(0, "P001", Some(list_ctrl_icons[0])); // File icon
    let _item2_idx = list_ctrl.insert_item(1, "P002", Some(list_ctrl_icons[1])); // Folder icon
    let _item3_idx = list_ctrl.insert_item(2, "P003", Some(list_ctrl_icons[2])); // Info icon
    let _item4_idx = list_ctrl.insert_item(3, "P004", Some(list_ctrl_icons[0])); // File icon
    let _item5_idx = list_ctrl.insert_item(4, "P005", None); // No icon

    // Create product info for each item
    let product1 = ProductInfo {
        sku: "ABC123".to_string(),
        price: 19.99,
        in_stock: true,
        reorder_level: 5,
    };

    let product2 = ProductInfo {
        sku: "DEF456".to_string(),
        price: 29.99,
        in_stock: false,
        reorder_level: 10,
    };

    let product3 = ProductInfo {
        sku: "GHI789".to_string(),
        price: 39.99,
        in_stock: true,
        reorder_level: 15,
    };

    // Attach the data to the items
    list_ctrl.set_custom_data(0u64, product1);
    list_ctrl.set_custom_data(1u64, product2);
    list_ctrl.set_custom_data(2u64, product3);

    // Set up some colors - use valid RGB values
    let blue_color = Colour::new(230, 240, 255, 255); // Light blue
    let yellow_color = Colour::new(255, 255, 230, 255); // Light yellow
    let red_text = Colour::new(200, 0, 0, 255); // Dark red
    let green_text = Colour::new(0, 150, 0, 255); // Dark green

    // Format specific rows
    list_ctrl.set_item_background_colour(0, &blue_color);
    list_ctrl.set_item_background_colour(2, &yellow_color);
    list_ctrl.set_item_background_colour(4, &blue_color);

    // Format specific cell text
    list_ctrl.set_item_text_colour(1, &red_text);
    list_ctrl.set_item_text_colour(3, &green_text);

    // Set item data - allows storing arbitrary integer data with each row
    list_ctrl.set_custom_data(0u64, 1001);
    list_ctrl.set_custom_data(1u64, 2002);
    list_ctrl.set_custom_data(2u64, 3003);
    list_ctrl.set_custom_data(3u64, 4004);
    list_ctrl.set_custom_data(4u64, 5005);

    // Set up selection
    list_ctrl.set_item_state(0, ListItemState::Selected, ListItemState::Selected);

    // Set up status display
    let list_ctrl_status_label = StaticText::builder(&panel)
        .with_label("ListCtrl Status: None")
        .with_size(Size::new(400, -1)) // Set wider width, auto height
        .build();

    // Add buttons to interact with the list control
    let list_ctrl_button_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    let add_button = Button::builder(&panel).with_label("Add Item").build();

    let remove_button = Button::builder(&panel).with_label("Remove Selected").build();

    let select_button = Button::builder(&panel).with_label("Select First").build();

    let edit_button = Button::builder(&panel).with_label("Edit Selected").build();

    let cleanup_button = Button::builder(&panel).with_label("Test Cleanup").build();

    let populate_button = Button::builder(&panel).with_label("Populate More").build();

    list_ctrl_button_sizer.add(&add_button, 0, SizerFlag::All, 5);
    list_ctrl_button_sizer.add(&remove_button, 0, SizerFlag::All, 5);
    list_ctrl_button_sizer.add(&select_button, 0, SizerFlag::All, 5);
    list_ctrl_button_sizer.add(&edit_button, 0, SizerFlag::All, 5);
    list_ctrl_button_sizer.add(&cleanup_button, 0, SizerFlag::All, 5);
    list_ctrl_button_sizer.add(&populate_button, 0, SizerFlag::All, 5);

    // --- ADDED: EditableListBox Example ---
    let editable_listbox = EditableListBox::builder(&panel)
        .with_label("Editable List")
        .with_style(EditableListBoxStyle::AllowNew | EditableListBoxStyle::AllowEdit | EditableListBoxStyle::AllowDelete)
        .build();

    // Add some initial items to the editable listbox
    editable_listbox.set_strings(&["Task 1", "Task 2", "Task 3"]);

    let editable_listbox_status_label = StaticText::builder(&panel)
        .with_label("EditableListBox: Use buttons to modify list")
        .build();

    // --- ADDED: RearrangeList Example ---
    let rearrangelist_items = vec![
        "First Item".to_string(),
        "Second Item".to_string(),
        "Third Item".to_string(),
        "Fourth Item".to_string(),
        "Fifth Item".to_string(),
    ];

    // Initial order with some items checked and some unchecked
    // Positive values = checked items, Negative values = unchecked (using bitwise complement)
    let initial_order = vec![0, !1, 2, !3, 4]; // 0,2,4 checked; 1,3 unchecked

    let rearrangelist = RearrangeList::builder(&panel)
        .with_id(ID_HIGHEST + 10)
        .with_items(rearrangelist_items)
        .with_order(initial_order)
        .with_style(RearrangeListStyle::Default)
        .build();

    let rearrangelist_status_label = StaticText::builder(&panel)
        .with_label("RearrangeList: Use selection or buttons to modify")
        .build();

    // Add buttons to interact with the RearrangeList
    let rearrangelist_button_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    let move_up_button = Button::builder(&panel).with_label("Move Up").build();
    let move_down_button = Button::builder(&panel).with_label("Move Down").build();

    rearrangelist_button_sizer.add(&move_up_button, 0, SizerFlag::All, 5);
    rearrangelist_button_sizer.add(&move_down_button, 0, SizerFlag::All, 5);

    // --- Sizer for *inner_list_panel* ---
    let list_sizer_main = BoxSizer::builder(Orientation::Vertical).build();
    let list_row_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    let list_box_col = BoxSizer::builder(Orientation::Vertical).build();
    // TODO: Re-evaluate create_section_title usage
    // listbox_sizer.add(&create_section_title(&panel, "ListBox"), 0, SizerFlag::Expand | SizerFlag::Bottom, 5);
    list_box_col.add(&list_box, 1, SizerFlag::Expand | SizerFlag::All, 5);
    list_box_col.add(&listbox_status_label, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 5);
    list_row_sizer.add_sizer(&list_box_col, 1, SizerFlag::Expand | SizerFlag::All, 5);
    let check_list_col = BoxSizer::builder(Orientation::Vertical).build();
    check_list_col.add(&checklistbox, 1, SizerFlag::Expand | SizerFlag::All, 5);
    check_list_col.add(
        &checklistbox_status_label,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );
    list_row_sizer.add_sizer(&check_list_col, 1, SizerFlag::Expand | SizerFlag::All, 5);
    list_sizer_main.add_sizer(&list_row_sizer, 1, SizerFlag::Expand | SizerFlag::All, 5);

    let choice_row_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    let choice_col = BoxSizer::builder(Orientation::Vertical).build();
    choice_col.add(&choice_ctrl, 0, SizerFlag::All, 5);
    choice_col.add(&choice_status_label, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 5);
    choice_row_sizer.add_sizer(&choice_col, 1, SizerFlag::Expand | SizerFlag::All, 5);

    let combo_col = BoxSizer::builder(Orientation::Vertical).build();
    combo_col.add(&combo_box, 0, SizerFlag::All, 5);
    combo_col.add(&combo_status_label, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 5);
    choice_row_sizer.add_sizer(&combo_col, 1, SizerFlag::Expand | SizerFlag::All, 5);

    // ADDED: Add the choice/combo row sizer to the main vertical sizer
    list_sizer_main.add_sizer(&choice_row_sizer, 0, SizerFlag::Expand | SizerFlag::All, 5);

    // Add ListCtrl and its status label
    let list_ctrl_col_sizer = BoxSizer::builder(Orientation::Vertical).build();
    list_ctrl_col_sizer.add(&list_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 5); // ListCtrl takes available space
    list_ctrl_col_sizer.add_sizer(
        &list_ctrl_button_sizer,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );
    // Make sure status label has enough space and uses the full width
    list_ctrl_col_sizer.add(&list_ctrl_status_label, 0, SizerFlag::Expand | SizerFlag::All, 5);
    list_sizer_main.add_sizer(&list_ctrl_col_sizer, 1, SizerFlag::Expand | SizerFlag::All, 5); // Add ListCtrl sizer to main, taking space

    // Add EditableListBox and its status label
    let editable_listbox_sizer = BoxSizer::builder(Orientation::Vertical).build();
    editable_listbox_sizer.add(&editable_listbox, 1, SizerFlag::Expand | SizerFlag::All, 5);
    editable_listbox_sizer.add(
        &editable_listbox_status_label,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );
    list_sizer_main.add_sizer(&editable_listbox_sizer, 1, SizerFlag::Expand | SizerFlag::All, 5);

    // Add RearrangeList and its status label
    let rearrangelist_sizer = BoxSizer::builder(Orientation::Vertical).build();
    rearrangelist_sizer.add(&rearrangelist, 1, SizerFlag::Expand | SizerFlag::All, 5);
    rearrangelist_sizer.add_sizer(
        &rearrangelist_button_sizer,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );
    rearrangelist_sizer.add(
        &rearrangelist_status_label,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        5,
    );

    // Add RearrangeList sizer after EditableListBox
    list_sizer_main.add_sizer(&rearrangelist_sizer, 1, SizerFlag::Expand | SizerFlag::All, 5);

    panel.set_sizer(list_sizer_main, true);
    // Fit the inner panel to its contents initially
    panel.fit();

    // --- Configure the ScrolledWindow ---
    // Calculate virtual size needed based on the inner panel's best size
    let inner_size = panel.get_best_size();
    // Set scroll rate (pixels per scroll unit)
    scrolled_list_window.set_scroll_rate(10, 10);
    // Set scrollbars based on inner panel size (make virtual size a bit larger to ensure scrolling)
    scrolled_list_window.set_scrollbars(ScrollBarConfig {
        pixels_per_unit_x: 10,                     // pixels per unit X
        pixels_per_unit_y: 10,                     // pixels per unit Y
        no_units_x: (inner_size.width + 50) / 10,  // number of units X (ensure > visible width)
        no_units_y: (inner_size.height + 50) / 10, // number of units Y (ensure > visible height)
        x_pos: 0,                                  // initial X position
        y_pos: 0,                                  // initial Y position
        no_refresh: true,                          // no refresh immediately
    });

    // --- Event Binding ---
    // ListBox Event Binding (Refactored)
    let listbox_status_label_clone = listbox_status_label.clone();
    list_box.on_selection_changed(move |event_data| {
        if let Some(selection_str) = event_data.get_string() {
            listbox_status_label_clone.set_label(&format!("List Selection: {selection_str}"));
        }
        if let Some(index) = event_data.get_selection() {
            println!(
                "ListBox Selected - Index: {}, String: {:?}",
                index,
                event_data.get_string().unwrap_or_default()
            );
        }
    });

    list_box.on_item_double_clicked(|event_data| {
        if let Some(index) = event_data.get_selection() {
            println!(
                "ListBox DoubleClicked - Index: {}, String: {:?}",
                index,
                event_data.get_string().unwrap_or_default()
            );
        }
    });

    // CheckListBox Event Binding
    let checklistbox_status_label_clone = checklistbox_status_label.clone();
    let checklistbox_clone = checklistbox.clone();
    checklistbox.on_selected(move |event_data| {
        if let Some(index) = event_data.get_selection() {
            let is_checked = checklistbox_clone.is_checked(index);
            let item_text = event_data.get_string().unwrap_or_else(|| "?".to_string());
            checklistbox_status_label_clone.set_label(&format!(
                "CheckList Sel: {} ('{}' {})",
                index,
                item_text,
                if is_checked { "Checked" } else { "Unchecked" }
            ));
        } else {
            checklistbox_status_label_clone.set_label("CheckList Sel: None");
        }
    });

    // Choice selection event (Refactored)
    let choice_status_label_clone = choice_status_label.clone();
    choice_ctrl.on_selection_changed(move |event_data| {
        if let Some(selected_string) = event_data.get_string() {
            choice_status_label_clone.set_label(&format!("Choice Selection: {selected_string}"));
        } else {
            choice_status_label_clone.set_label("Choice Selection: None");
        }
    });

    // ComboBox events
    let combo_status_label_clone = combo_status_label.clone();
    combo_box.on_selection_changed(move |event_data| {
        if let Some(selected_string) = event_data.get_string() {
            combo_status_label_clone.set_label(&format!("Combo Selected: {selected_string}"));
        } else {
            combo_status_label_clone.set_label("Combo Selected: None");
        }
    });

    let combo_status_label_clone = combo_status_label.clone();
    combo_box.on_text_updated(move |event_data| {
        if let Some(current_text) = event_data.get_string() {
            combo_status_label_clone.set_label(&format!("Combo Text: {current_text}"));
        }
    });

    combo_box.on_enter_pressed(move |event_data| {
        if let Some(current_text) = event_data.get_string() {
            println!("ComboBox Enter: {current_text}");
        }
    });

    // EditableListBox events - basic selection event
    let status_label_clone1 = editable_listbox_status_label.clone();
    editable_listbox.on_selection_changed(move |_event_data| {
        // Simple update without accessing complex methods
        status_label_clone1.set_label("Item selected in EditableListBox");
    });

    // Bind event for when editing begins
    let status_label_clone2 = editable_listbox_status_label.clone();
    editable_listbox.on_begin_label_edit(move |_event_data| {
        status_label_clone2.set_label("Editing item...");
    });

    // Bind event for when editing ends
    let status_label_clone3 = editable_listbox_status_label.clone();
    editable_listbox.on_end_label_edit(move |_event_data| {
        status_label_clone3.set_label("Item edited");
    });

    // Edit button
    let list_ctrl_clone = list_ctrl.clone();
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    edit_button.on_click(move |_| {
        let selected_item = list_ctrl_clone.get_first_selected_item();
        if selected_item >= 0 {
            // Start editing the label of the selected item
            let text_ctrl = list_ctrl_clone.edit_label(selected_item as i64);
            text_ctrl.set_value("New Label");
            // You can access the TextCtrl here if needed
            list_ctrl_status_label_clone.set_label(&format!("Editing item {selected_item}"));
        } else {
            list_ctrl_status_label_clone.set_label("Please select an item to edit");
        }
    });

    // Bind button events
    let list_ctrl_clone = list_ctrl.clone();
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    add_button.on_click(move |_| {
        println!("Add Item button clicked");
        let count = list_ctrl_clone.get_item_count();
        let new_item_text = format!("P{:03}", count + 1);
        let new_idx = list_ctrl_clone.insert_item(count as i64, &new_item_text, None);
        list_ctrl_clone.set_item_text_by_column(new_idx as i64, 1, "New Description");
        list_ctrl_clone.set_item_text_by_column(new_idx as i64, 2, "0");
        list_ctrl_status_label_clone.set_label(&format!("Added new item {new_idx}"));
    });

    // Remove button
    let list_ctrl_clone = list_ctrl.clone();
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    remove_button.on_click(move |_| {
        let selected_item = list_ctrl_clone.get_first_selected_item();
        if selected_item >= 0 {
            // Delete the selected item
            if list_ctrl_clone.delete_item(selected_item as i64) {
                list_ctrl_status_label_clone.set_label(&format!("Removed item {selected_item}"));
            } else {
                list_ctrl_status_label_clone.set_label(&format!("Failed to remove item {selected_item}"));
            }
        } else {
            list_ctrl_status_label_clone.set_label("Please select an item to remove");
        }
    });

    // Select first button
    let list_ctrl_clone = list_ctrl.clone();
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    select_button.on_click(move |_| {
        // Ensure first item exists
        if list_ctrl_clone.get_item_count() > 0 {
            // Select the first item
            list_ctrl_clone.set_item_state(0, ListItemState::Selected, ListItemState::Selected);
            // Ensure it's visible
            list_ctrl_clone.ensure_visible(0);
            list_ctrl_status_label_clone.set_label("Selected first item");
        } else {
            list_ctrl_status_label_clone.set_label("List is empty");
        }
    });

    // Bind events for item editing
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    list_ctrl.on_begin_label_edit(move |_| {
        list_ctrl_status_label_clone.set_label("Started editing label...");
    });

    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    list_ctrl.on_end_label_edit(move |event_data| {
        // Check if edit was cancelled - note: we need to handle this correctly
        let cancelled = event_data.is_edit_cancelled().unwrap_or(true);
        if cancelled {
            list_ctrl_status_label_clone.set_label("Label edit cancelled");
        } else {
            let _item_index = event_data.get_item_index();
            let label = event_data.get_label().unwrap_or_else(|| String::from("<no label>"));
            list_ctrl_status_label_clone.set_label(&format!("Label changed to: {label}"));
        }
    });

    // Cleanup button handler
    let list_ctrl_clone = list_ctrl.clone();
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    cleanup_button.on_click(move |_| {
        println!("Cleanup button clicked - calling explicit cleanup");
        list_ctrl_clone.cleanup_custom_data();
        list_ctrl_status_label_clone.set_label("Manual cleanup completed");
    });

    let list_ctrl_clone_populate = list_ctrl.clone();
    populate_button.on_click(move |_| {
        println!("Populate ListCtrl with more items button clicked");
        for i in 5..15 {
            let item_text = format!("Item {}", i + 1);
            list_ctrl_clone_populate.insert_item(i as i64, &item_text, None);
        }
    });

    // Set column text for all items
    for i in 0..5 {
        let item_idx = i as i64;

        // Set the main item label (column 0)
        let label = format!("P{:03}", i + 1);
        list_ctrl.set_item_text(item_idx, &label);

        // Get in-stock status based on even/odd
        let is_in_stock = i % 2 == 0;

        // Set colors based on availability
        if is_in_stock {
            list_ctrl.set_item_background_colour(item_idx, &blue_color);
        } else {
            list_ctrl.set_item_background_colour(item_idx, &yellow_color);
            list_ctrl.set_item_text_colour(item_idx, &red_text);
        }
    }

    // ComboBox events (Refactored)
    let combo_status_label_selected = combo_status_label.clone();
    combo_box.on_selection_changed(move |event_data| {
        if let Some(selected_string) = event_data.get_string() {
            combo_status_label_selected.set_label(&format!("Combo Selected: {selected_string}"));
        } else {
            combo_status_label_selected.set_label("Combo Selected: None");
        }
    });

    let combo_status_label_text_changed = combo_status_label.clone();
    combo_box.on_text_updated(move |event_data| {
        if let Some(current_text) = event_data.get_string() {
            combo_status_label_text_changed.set_label(&format!("Combo Text: {current_text}"));
        }
    });

    combo_box.on_enter_pressed(move |event_data| {
        if let Some(text_val) = event_data.get_string() {
            println!("ComboBox Enter: {text_val}");
        } else {
            println!("ComboBox Enter: (no text in event data)");
        }
    });

    // RearrangeList Event Binding
    let rearrangelist_status_label_clone = rearrangelist_status_label.clone();
    let rearrangelist_clone = rearrangelist.clone();
    rearrangelist.on_selected(move |event_data| {
        if let Some(index) = event_data.get_selection() {
            let is_checked = rearrangelist_clone.is_checked(index);
            let item_text = rearrangelist_clone
                .get_string(index)
                .unwrap_or_else(|| "<unknown>".to_string());
            rearrangelist_status_label_clone.set_label(&format!(
                "Selected: {} ('{}' {})",
                index,
                item_text,
                if is_checked { "Checked" } else { "Unchecked" }
            ));
        }
    });

    let rearrangelist_status_label_clone = rearrangelist_status_label.clone();
    let rearrangelist_clone = rearrangelist.clone();
    rearrangelist.on_toggled(move |event_data| {
        if let Some(index) = event_data.get_selection() {
            let is_checked = rearrangelist_clone.is_checked(index);
            let item_text = rearrangelist_clone
                .get_string(index)
                .unwrap_or_else(|| "<unknown>".to_string());
            rearrangelist_status_label_clone.set_label(&format!(
                "Toggled: {} ('{}' is now {})",
                index,
                item_text,
                if is_checked { "Checked" } else { "Unchecked" }
            ));
        }
    });

    let rearrangelist_status_label_clone = rearrangelist_status_label.clone();
    let rearrangelist_clone = rearrangelist.clone();
    rearrangelist.on_rearranged(move |_| {
        let order = rearrangelist_clone.get_current_order();
        rearrangelist_status_label_clone.set_label(&format!("Rearranged: current order is {order:?}"));
    });

    // Button event handlers for RearrangeList
    let rearrangelist_clone = rearrangelist.clone();
    let rearrangelist_status_label_clone = rearrangelist_status_label.clone();
    move_up_button.on_click(move |_| {
        if rearrangelist_clone.move_current_up() {
            rearrangelist_status_label_clone.set_label("Moved item up");
        } else {
            rearrangelist_status_label_clone.set_label("Could not move item up");
        }
    });

    let rearrangelist_clone = rearrangelist.clone();
    let rearrangelist_status_label_clone = rearrangelist_status_label.clone();
    move_down_button.on_click(move |_| {
        if rearrangelist_clone.move_current_down() {
            rearrangelist_status_label_clone.set_label("Moved item down");
        } else {
            rearrangelist_status_label_clone.set_label("Could not move item down");
        }
    });

    // Return the controls struct
    ListsTabControls {
        panel: scrolled_list_window,
        list_box,
        listbox_status_label,
        checklistbox,
        checklistbox_status_label,
        choice_ctrl,
        choice_status_label,
        combo_box,
        combo_status_label,
        list_ctrl,
        list_ctrl_status_label,
        editable_listbox,
        editable_listbox_status_label,
        rearrangelist,
        rearrangelist_status_label,
    }
}
