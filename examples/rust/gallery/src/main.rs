#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use wxdragon::prelude::*;

mod tabs;
use tabs::advanced_tab::create_advanced_tab;
use tabs::aui_tab::create_aui_tab;
use tabs::basic_tab::create_basic_tab;
use tabs::book_controls_tab::create_book_controls_tab;
use tabs::color_tab::create_color_tab;
use tabs::dataview_tree_tab::create_dataview_tree_tab;
use tabs::dataview_virtual_tab::create_dataview_virtual_tab;
use tabs::dialog_tab::create_dialog_tab;
use tabs::lists_tab::create_lists_tab;
use tabs::media_tab::create_media_tab;
use tabs::richtext_tab::create_richtext_tab;
use tabs::treectrl_tab::create_treectrl_tab;
use tabs::treelistctrl_tab::create_treelistctrl_tab;

// Tool IDs - used in main.rs
const ID_TOOL_NEW: Id = ID_HIGHEST + 1;
const ID_TOOL_OPEN: Id = ID_HIGHEST + 2;
const ID_TOOL_SAVE: Id = ID_HIGHEST + 3;

// --- Main Application Logic ---

fn main() {
    let _ = wxdragon::main(|_| {
        // Create the main application frame
        let frame = Frame::builder()
            .with_title("wxDragon Notebook Example")
            .with_size(Size::new(1100, 900))
            .build();

        // --- Menu Bar Setup ---
        let file_menu = Menu::builder()
            .append_item(ID_EXIT, "E&xit\tAlt-X", "Quit this program")
            .build();
        let help_menu = Menu::builder()
            .append_item(ID_ABOUT, "&About...", "Show about dialog")
            .build();
        let menubar = MenuBar::builder()
            .append(file_menu, "&File")
            .append(help_menu, "&Help")
            .build();
        frame.set_menu_bar(menubar);

        // --- Status Bar Setup ---
        StatusBar::builder(&frame)
            .with_fields_count(3)
            .with_status_widths(vec![-1, 150, 100])
            .add_initial_text(0, "Ready")
            .add_initial_text(1, "Center Field")
            .add_initial_text(2, "Right Field")
            .build();

        // --- Create the Notebook ---
        let notebook = Notebook::builder(&frame).with_id(120).build();

        // --- Create and Set ImageList for Notebook ---
        let image_list = ImageList::new(16, 16, true, 3);

        let mut image_ids: Vec<i32> = Vec::new();

        if let Some(bmp_info) =
            ArtProvider::get_bitmap(ArtId::Information, ArtClient::Menu, Some(Size::new(16, 16)))
        {
            let idx = image_list.add_bitmap(&bmp_info);
            if idx != -1 {
                image_ids.push(idx);
            } else {
                eprintln!("Failed to add Information icon to ImageList");
            }
        } else {
            eprintln!("Failed to get ArtId::Information for Notebook");
        }
        if let Some(bmp_question) =
            ArtProvider::get_bitmap(ArtId::Question, ArtClient::Menu, Some(Size::new(16, 16)))
        {
            let idx = image_list.add_bitmap(&bmp_question);
            if idx != -1 {
                image_ids.push(idx);
            } else {
                eprintln!("Failed to add Question icon to ImageList");
            }
        } else {
            eprintln!("Failed to get ArtId::Question for Notebook");
        }
        if let Some(bmp_warning) =
            ArtProvider::get_bitmap(ArtId::Warning, ArtClient::Menu, Some(Size::new(16, 16)))
        {
            let idx = image_list.add_bitmap(&bmp_warning);
            if idx != -1 {
                image_ids.push(idx);
            } else {
                eprintln!("Failed to add Warning icon to ImageList");
            }
        } else {
            eprintln!("Failed to get ArtId::Warning for Notebook");
        }

        if !image_ids.is_empty() {
            notebook.set_image_list(image_list);
        } else {
            eprintln!("No images were added to the ImageList. Not setting it on the Notebook.");
        }

        // --- Create Tabs ---
        let (advanced_splitter, advanced_controls) = create_advanced_tab(&notebook);
        let basic_controls = create_basic_tab(&notebook, &frame);
        let list_controls = create_lists_tab(&notebook, &frame);
        let book_controls = create_book_controls_tab(&notebook);
        let dialog_controls = create_dialog_tab(&notebook, &frame);
        let media_controls = create_media_tab(&notebook);
        let tree_controls = create_treectrl_tab(&notebook);
        let treelist_controls = create_treelistctrl_tab(&notebook);
        let aui_controls = create_aui_tab(&notebook);
        let color_controls = create_color_tab(&notebook, &frame);
        let dataview_virtual_controls = create_dataview_virtual_tab(&notebook);
        let dataview_tree_controls = create_dataview_tree_tab(&notebook);
        let richtext_controls = create_richtext_tab(&notebook, &frame);

        // --- ToolBar Setup ---
        let tb_style = ToolBarStyle::Text | ToolBarStyle::Default;
        if let Some(toolbar) = frame.create_tool_bar(Some(tb_style), ID_ANY as i32) {
            // Get sizes for the toolbar icons (platform-dependent)
            let icon_size = ArtProvider::get_native_dip_size_hint(ArtClient::Toolbar);
            println!(
                "Native toolbar icon size: {}x{}",
                icon_size.width, icon_size.height
            );

            // New Tool
            if let Some(new_bundle) =
                ArtProvider::get_bitmap_bundle(ArtId::New, ArtClient::Toolbar, None)
            {
                toolbar.add_tool_bundle(ID_TOOL_NEW, "New", &new_bundle, "Create a new file");
                println!("Using BitmapBundle for New tool");
            } else if let Some(new_icon) =
                ArtProvider::get_bitmap(ArtId::New, ArtClient::Toolbar, None)
            {
                toolbar.add_tool(ID_TOOL_NEW, "New", &new_icon, "Create a new file");
                println!("Fallback to Bitmap for New tool");
            } else {
                eprintln!("Failed to get ArtId::New for toolbar");
            }

            // Open Tool
            if let Some(open_bundle) =
                ArtProvider::get_bitmap_bundle(ArtId::FileOpen, ArtClient::Toolbar, None)
            {
                toolbar.add_tool_bundle(
                    ID_TOOL_OPEN,
                    "Open",
                    &open_bundle,
                    "Open an existing file",
                );
                println!("Using BitmapBundle for Open tool");
            } else if let Some(open_icon) =
                ArtProvider::get_bitmap(ArtId::FileOpen, ArtClient::Toolbar, None)
            {
                toolbar.add_tool(ID_TOOL_OPEN, "Open", &open_icon, "Open an existing file");
                println!("Fallback to Bitmap for Open tool");
            } else {
                eprintln!("Failed to get ArtId::FileOpen for toolbar");
            }

            // Save Tool
            if let Some(save_bundle) =
                ArtProvider::get_bitmap_bundle(ArtId::FileSave, ArtClient::Toolbar, None)
            {
                toolbar.add_tool_bundle(
                    ID_TOOL_SAVE,
                    "Save",
                    &save_bundle,
                    "Save the current file",
                );
                println!("Using BitmapBundle for Save tool");
            } else if let Some(save_icon) =
                ArtProvider::get_bitmap(ArtId::FileSave, ArtClient::Toolbar, None)
            {
                toolbar.add_tool(ID_TOOL_SAVE, "Save", &save_icon, "Save the current file");
                println!("Fallback to Bitmap for Save tool");
            } else {
                eprintln!("Failed to get ArtId::FileSave for toolbar");
            }

            toolbar.realize();
        }

        // --- Add Pages to Notebook ---
        let mut current_image_idx = 0;
        let mut next_image_id = || {
            if image_ids.is_empty() {
                return None;
            }
            let id = image_ids[current_image_idx % image_ids.len()];
            current_image_idx += 1;
            Some(id)
        };

        notebook.add_page(
            &basic_controls.panel,
            "Basic Controls",
            true,
            next_image_id(),
        );
        notebook.add_page(&list_controls.panel, "Lists", false, next_image_id());
        notebook.add_page(&advanced_splitter, "Advanced", false, next_image_id());
        notebook.add_page(
            &book_controls.tab_panel,
            "Book Controls",
            false,
            next_image_id(),
        );
        notebook.add_page(&dialog_controls.panel, "Dialogs", false, next_image_id());
        notebook.add_page(&media_controls.panel, "Media", false, next_image_id());
        notebook.add_page(
            &tree_controls.panel,
            "Tree Controls",
            false,
            next_image_id(),
        );
        notebook.add_page(
            &treelist_controls.panel,
            "TreeList Controls",
            false,
            next_image_id(),
        );
        notebook.add_page(&aui_controls.panel, "AUI", false, next_image_id());
        notebook.add_page(&color_controls.panel, "Color", false, next_image_id());
        notebook.add_page(
            &dataview_virtual_controls.panel,
            "DataView Virtual",
            false,
            next_image_id(),
        );
        notebook.add_page(
            &dataview_tree_controls.panel,
            "DataView Tree",
            false,
            next_image_id(),
        );
        notebook.add_page(
            &richtext_controls.panel,
            "Rich Text",
            false,
            next_image_id(),
        );

        // --- Set Frame Sizer ---
        let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
        main_sizer.add(&notebook, 1, SizerFlag::Expand | SizerFlag::All, 1);
        frame.set_sizer(main_sizer, true);

        // --- Bind Event Handlers ---

        // Menu event handler using the new API
        let frame_clone_for_menu = frame.clone();
        frame.on_menu(move |event| match event.get_id() {
            id if id == ID_EXIT => {
                println!("Menu/Toolbar: Exit clicked!");
                frame_clone_for_menu.close(true);
            }
            id if id == ID_ABOUT => {
                println!("Menu: About clicked!");
            }
            id if id == ID_TOOL_NEW => {
                println!("Toolbar: New clicked!");
            }
            id if id == ID_TOOL_OPEN => {
                println!("Toolbar: Open clicked!");
            }
            id if id == ID_TOOL_SAVE => {
                println!("Toolbar: Save clicked!");
            }
            // Context menu IDs from DataView tab (handled by the tab itself)
            1001 | 1002 => {
                // These are handled by the DataViewCtrl's context menu handlers
                // Don't print "Unhandled" for these
            }
            _ => {
                println!("Unhandled Menu/Tool ID: {}", event.get_id());
                event.skip(true);
            }
        });

        // Notebook page changed event using the new API
        let notebook_clone_page_changed = notebook.clone();
        let frame_clone_page_changed = frame.clone();
        notebook.on_page_changed(move |event_data| {
            let new_page_index = event_data.get_selection().unwrap_or(0);
            let old_page_index = event_data.get_old_selection().unwrap_or(0);

            let new_page_text = notebook_clone_page_changed
                .get_page(new_page_index as usize)
                .map_or_else(
                    || "<unknown page>".to_string(),
                    |p| p.get_label().unwrap_or_default(),
                );
            let old_page_text = notebook_clone_page_changed
                .get_page(old_page_index as usize)
                .map_or_else(
                    || "<unknown page>".to_string(),
                    |p| p.get_label().unwrap_or_default(),
                );

            println!(
                "Notebook PageChanged: New={new_page_index}, Old={old_page_index}, NewLabel='{new_page_text}', OldLabel='{old_page_text}'"
            );
            frame_clone_page_changed.set_status_text(
                &format!(
                    "Switched from tab '{old_page_text}' to '{new_page_text}'"
                ),
                0,
            );
        });

        // Second notebook event binding for page changing
        let notebook_clone_page_changing = notebook.clone();
        notebook.on_page_changed(move |event_data| {
            let new_page_index = event_data.get_selection().unwrap_or(0);
            let old_page_index = event_data.get_old_selection().unwrap_or(0);

            let new_page_text = notebook_clone_page_changing
                .get_page(new_page_index as usize)
                .map_or_else(
                    || "<unknown page>".to_string(),
                    |p| p.get_label().unwrap_or_default(),
                );
            let old_page_text = notebook_clone_page_changing
                .get_page(old_page_index as usize)
                .map_or_else(
                    || "<unknown page>".to_string(),
                    |p| p.get_label().unwrap_or_default(),
                );

            println!(
                "Notebook PageChanging: New={new_page_index}, Old={old_page_index}, NewLabel='{new_page_text}', OldLabel='{old_page_text}'"
            );
        });

        // Bind tab-specific events
        basic_controls.bind_events();
        advanced_controls.bind_events();
        book_controls.bind_events();
        dialog_controls.bind_events(&frame);
        media_controls.bind_events();
        tree_controls.bind_events();
        treelist_controls.bind_events();
        aui_controls.bind_events();
        richtext_controls.bind_events();

        // --- Final Setup ---
        frame.show(true);
        frame.centre();
    });
}
