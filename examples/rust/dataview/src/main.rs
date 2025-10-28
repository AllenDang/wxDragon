use std::cell::RefCell;
use std::rc::Rc;
use wxdragon::DataViewCtrl;
use wxdragon::DataViewStyle;
use wxdragon::prelude::*;

mod music_tree;
mod mymodels;

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    let file_name = "music_tree_dump.json";

    let data_path: std::path::PathBuf = {
        let base = dirs::config_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
        let base = base.join(std::env::var("CARGO_PKG_NAME").unwrap_or("wxdragon".to_string()));
        std::fs::create_dir_all(&base).unwrap();
        base.join(file_name)
    };

    let mut data = match music_tree::load_music_tree_from_file(&data_path) {
        Ok(tree) => tree,
        Err(e) => {
            let data_path = data_path.display();
            log::info!("Failed to load {data_path}: {e}");
            music_tree::generate_initial_music_tree()
        }
    };
    data.filepath = Some(data_path.clone());

    let data_rc: Rc<RefCell<music_tree::MusicTree>> = Rc::new(RefCell::new(data));
    let model = mymodels::create_music_tree_model(data_rc);
    let _ = wxdragon::main(move |_| {
        let frame = Frame::builder()
            .with_title("wxDragon DataView Example")
            .with_size(Size::new(900, 600))
            .build();

        let panel = Panel::builder(&frame).build();

        // Create the generic DataView control (works with custom models)
        let dataview = DataViewCtrl::builder(&panel)
            .with_size(Size::new(860, 520))
            .with_style(DataViewStyle::RowLines | DataViewStyle::VerticalRules)
            .build();

        // Create columns and text renderers
        let title_col = DataViewColumn::new(
            "Title",
            &DataViewTextRenderer::new(
                VariantType::String,
                DataViewCellMode::Inert,
                DataViewAlign::Left,
            ),
            0,
            250,
            DataViewAlign::Left,
            DataViewColumnFlags::Resizable,
        );

        let artist_col = DataViewColumn::new(
            "Artist",
            &DataViewTextRenderer::new(
                VariantType::String,
                DataViewCellMode::Inert,
                DataViewAlign::Left,
            ),
            1,
            200,
            DataViewAlign::Left,
            DataViewColumnFlags::Resizable,
        );

        let year_col = DataViewColumn::new(
            "Year",
            &DataViewTextRenderer::new(
                VariantType::String,
                DataViewCellMode::Inert,
                DataViewAlign::Center,
            ),
            2,
            100,
            DataViewAlign::Center,
            DataViewColumnFlags::Resizable,
        );

        let judg_col = DataViewColumn::new(
            "Judgement",
            &DataViewTextRenderer::new(
                VariantType::String,
                DataViewCellMode::Inert,
                DataViewAlign::Left,
            ),
            3,
            120,
            DataViewAlign::Left,
            DataViewColumnFlags::Resizable,
        );

        dataview.append_column(&title_col);
        dataview.append_column(&artist_col);
        dataview.append_column(&year_col);
        dataview.append_column(&judg_col);

        // Associate the model with the control
        dataview.associate_model(&model);

        // Expand the visible root and the first child ('Pop music') on startup.
        // The invalid/default DataViewItem represents the invisible root; its
        // first child is the visible root 'My Music'.
        let invalid = DataViewItem::default();
        // Get the visible root (first top-level item)
        let root_item = dataview.get_nth_child(&invalid, 0);
        if root_item.is_ok() {
            dataview.expand(&root_item);
            dataview.ensure_visible(&root_item);
            // Expand the first child of the visible root (Pop music)
            let first_child = dataview.get_nth_child(&root_item, 0);
            if first_child.is_ok() {
                dataview.expand(&first_child);
                dataview.ensure_visible(&first_child);
                // Select the second grandchild ("Yesterday")
                let grandchild = dataview.get_nth_child(&first_child, 1);
                if grandchild.is_ok() {
                    dataview.select(&grandchild);
                    dataview.ensure_visible(&grandchild);
                }
            }
        }

        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        sizer.add(&dataview, 1, SizerFlag::Expand | SizerFlag::All, 8);
        panel.set_sizer(sizer, true);

        // Right-click item context menu with an "Edit" entry
        let dataview_for_menu = dataview.clone();
        let frame_for_dialog = frame.clone();
        let mtm_for_edit = model.clone();

        dataview.on_item_context_menu(move |event| {
            // Keep the model alive as long as frame/control lives by holding it in mtm_slot.
            // Nothing to do here; mtm_slot captures the Rc. It will be cleared on frame destroy.
            if let Some(item) = event.get_item() {
                // Ensure the item is selected so we can retrieve it later from the menu handler
                dataview_for_menu.select(&item);

                // Build a simple context menu
                let edit_id = ID_HIGHEST + 1;
                let mut menu = Menu::builder()
                    .append_item(edit_id, "Edit", "Edit this item")
                    .build();

                // Handle menu selection
                let dataview_for_selected = dataview_for_menu.clone();
                let frame_for_selected = frame_for_dialog.clone();
                let mtm_for_selected = mtm_for_edit.clone();
                menu.on_selected(move |ev| {
                    match ev.get_id() {
                        id if id == edit_id => {
                            // Get the currently selected item
                            if let Some(sel_item) = dataview_for_selected.get_selection()
                                && let Some(ptr) = sel_item.get_id::<music_tree::MusicNode>()
                            {
                                // SAFETY: ptr is an opaque model ID set by us to a MusicNode address
                                let node: &music_tree::MusicNode = unsafe { &*ptr };
                                let current_title = node.title.clone();

                                // Show a simple text entry dialog to edit the title
                                let dlg = TextEntryDialog::builder(
                                    &frame_for_selected,
                                    "Edit title",
                                    "Edit",
                                )
                                .with_default_value(&current_title)
                                .build();
                                let ret = dlg.show_modal();
                                if ret == ID_OK
                                    && let Some(new_val) = dlg.get_value()
                                {
                                    let val = Variant::from_string(new_val);
                                    mtm_for_selected.set_value(ptr, 0, &val);
                                }
                            }
                        }
                        _ => {}
                    }
                });

                // Show popup menu at mouse position
                dataview_for_menu.popup_menu(&menu, None);

                // Explicitly destroy the standalone popup menu to release its wxEvtHandler and closures
                menu.destroy_meun();
            }
        });
        frame.show(true);
        frame.centre();
    });
}
