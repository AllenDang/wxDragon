use music_tree::{MusicNode, MusicTree, NodeType};
use std::cell::RefCell;
use std::rc::Rc;
use wxdragon::DataViewCtrl;
use wxdragon::DataViewStyle;
use wxdragon::prelude::*;

mod edit_dialog;
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
    let model = mymodels::create_music_tree_model(data_rc.clone());
    let _ = wxdragon::main(move |_| {
        let frame = Frame::builder()
            .with_title("wxDragon DataView Example")
            .with_size(Size::new(900, 600))
            .build();

        let panel = Panel::builder(&frame).build();

        // Create the generic DataView control (works with custom models), it behaves like a tree view
        let dataview = DataViewCtrl::builder(&panel)
            .with_size(Size::new(860, 520))
            .with_style(DataViewStyle::RowLines | DataViewStyle::VerticalRules)
            .build();

        fn create_column(title: &str, model_col: usize, width: i32) -> DataViewColumn {
            DataViewColumn::new(
                title,
                &DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, DataViewAlign::Left),
                model_col,
                width,
                DataViewAlign::Left,
                DataViewColumnFlags::Resizable | DataViewColumnFlags::Sortable,
            )
        }

        // Create columns and text renderers
        let title_col = create_column("Title", 0, 250);
        let artist_col = create_column("Artist", 1, 200);
        let year_col = create_column("Year", 2, 100);
        let judg_col = create_column("Judgement", 3, 120);

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
                log::info!("Showing context menu for item at position {:?}", event.get_position());

                // Build a context menu
                let edit_id = ID_HIGHEST + 1;
                let new_branch_id = ID_HIGHEST + 2;
                let new_leaf_id = ID_HIGHEST + 3;
                let delete_id = ID_HIGHEST + 4;
                let mut menu = Menu::builder().with_title("Manage Node").build();

                let _ = menu.append(edit_id, "Edit", "Edit this item", ItemKind::Normal);

                // If the clicked item is a branch, offer creation of new child nodes
                if let Some(ptr) = item.get_id::<MusicNode>() {
                    let node: &MusicNode = unsafe { &*ptr };
                    if matches!(node.node_type, NodeType::Branch) {
                        // Runtime append uses Menu::append with an explicit ItemKind
                        let h = "Add a new branch under this node";
                        let _ = menu.append(new_branch_id, "New Branch", h, ItemKind::Normal);
                        let _ = menu.append(new_leaf_id, "New Leaf", "Add a new leaf under this node", ItemKind::Normal);
                    }
                }

                // Always offer Delete (we'll guard root deletion in handler)
                let _ = menu.append(delete_id, "Delete", "Delete this item", ItemKind::Normal);

                // Handle menu selection
                let frame_for_selected = frame_for_dialog.clone();
                let mtm_for_selected = mtm_for_edit.clone();
                let dv_for_actions = dataview_for_menu.clone();
                // Move the DataViewItem itself into the closure; do NOT call .clone() as that would clone the inner pointer
                let item_for_actions = item;
                menu.on_selected(move |ev| match ev.get_id() {
                    id if id == edit_id => {
                        edit_item_with_dialog(&frame_for_selected, &mtm_for_selected, &item_for_actions);
                    }
                    id if id == new_branch_id => {
                        if let Some(parent_ptr) = item_for_actions.get_id::<MusicNode>() {
                            // Temporary node to drive dialog (only title enabled for branch)
                            let temp = MusicNode::new(NodeType::Branch, "", Option::<&str>::None, None, Option::<&str>::None);
                            let dlg = edit_dialog::NodeEditDialog::new(&frame_for_selected, &temp);
                            if dlg.show_modal() == ID_OK {
                                let edited = dlg.value();
                                let title_trimmed = edited.title.trim().to_string();
                                if !title_trimmed.is_empty() {
                                    let node = MusicNode::new(
                                        NodeType::Branch,
                                        &title_trimmed,
                                        Option::<&str>::None,
                                        None,
                                        Option::<&str>::None,
                                    );
                                    // Mutate underlying userdata via model, then notify view
                                    let created = mtm_for_selected
                                        .with_userdata_mut::<Rc<RefCell<MusicTree>>, Option<*const MusicNode>>(|tree_rc| {
                                            if let Some(parent_rc) = mymodels::find_node_via_raw_ptr(tree_rc, parent_ptr) {
                                                let child_rc = Rc::new(RefCell::new(node));
                                                let child_ptr: *const MusicNode = {
                                                    let b = child_rc.borrow();
                                                    &*b as *const MusicNode
                                                };
                                                MusicNode::push_child(&parent_rc, child_rc);
                                                Some(child_ptr)
                                            } else {
                                                None
                                            }
                                        });
                                    if let Some(Some(child_ptr)) = created {
                                        mtm_for_selected.item_added::<MusicNode>(Some(parent_ptr), child_ptr);
                                        dv_for_actions.expand(&item_for_actions);
                                        dv_for_actions.ensure_visible(&item_for_actions);
                                    }
                                }
                            }
                        }
                    }
                    id if id == new_leaf_id => {
                        if let Some(parent_ptr) = item_for_actions.get_id::<MusicNode>() {
                            let temp = MusicNode::new(NodeType::Leaf, "", Option::<&str>::None, None, Option::<&str>::None);
                            let dlg = edit_dialog::NodeEditDialog::new(&frame_for_selected, &temp);
                            if dlg.show_modal() == ID_OK {
                                let edited = dlg.value();
                                let title_trimmed = edited.title.trim().to_string();
                                if !title_trimmed.is_empty() {
                                    let node = MusicNode::new(
                                        NodeType::Leaf,
                                        &title_trimmed,
                                        edited.artist.as_deref(),
                                        edited.year,
                                        edited.quality.as_deref(),
                                    );

                                    // Mutate underlying userdata via model, then notify view
                                    let created = mtm_for_selected
                                        .with_userdata_mut::<Rc<RefCell<music_tree::MusicTree>>, Option<*const MusicNode>>(
                                            |tree_rc| {
                                                if let Some(parent_rc) = mymodels::find_node_via_raw_ptr(tree_rc, parent_ptr) {
                                                    let child_rc = Rc::new(RefCell::new(node));
                                                    let child_ptr: *const MusicNode = {
                                                        let b = child_rc.borrow();
                                                        &*b as *const MusicNode
                                                    };
                                                    music_tree::MusicNode::push_child(&parent_rc, child_rc);
                                                    Some(child_ptr)
                                                } else {
                                                    None
                                                }
                                            },
                                        );
                                    if let Some(Some(child_ptr)) = created {
                                        mtm_for_selected.item_added::<MusicNode>(Some(parent_ptr), child_ptr);
                                        dv_for_actions.expand(&item_for_actions);
                                        dv_for_actions.ensure_visible(&item_for_actions);
                                    }
                                }
                            }
                        }
                    }
                    id if id == delete_id => {
                        if let Some(child_ptr) = item_for_actions.get_id::<MusicNode>() {
                            // Mutate underlying userdata: remove the child from its parent if not root
                            let parent_opt = mtm_for_selected
                                .with_userdata_mut::<Rc<RefCell<music_tree::MusicTree>>, Option<*const MusicNode>>(|tree_rc| {
                                    let root_ptr: *const MusicNode = {
                                        let tree_b = tree_rc.borrow();
                                        let root_ref = tree_b.root.borrow();
                                        let p: *const MusicNode = &*root_ref;
                                        p
                                    };
                                    // Don't delete the visible root
                                    if root_ptr == child_ptr {
                                        return None;
                                    }
                                    // Find parent Rc
                                    let parent_rc_opt = {
                                        let tree_b = tree_rc.borrow();
                                        tree_b.parent_of(unsafe { &*child_ptr })
                                    };
                                    if let Some(parent_rc) = parent_rc_opt {
                                        // Remove child from parent's children vec
                                        let mut parent_b = parent_rc.borrow_mut();
                                        if let Some(children) = parent_b.children.as_mut()
                                            && let Some(idx) = children.iter().position(|rc| {
                                                let ptr: *const MusicNode = &*rc.borrow();
                                                ptr == child_ptr
                                            })
                                        {
                                            children.remove(idx);
                                            // Return parent pointer for notification
                                            let p_ptr: *const MusicNode = &*parent_b as *const MusicNode;
                                            return Some(p_ptr);
                                        }
                                    }
                                    None
                                });
                            if let Some(Some(parent_ptr)) = parent_opt {
                                // Notify view about deletion
                                mtm_for_selected.item_deleted::<MusicNode>(Some(parent_ptr), child_ptr);
                            }
                        }
                    }
                    _ => {}
                });

                // Show popup menu at mouse position
                dataview_for_menu.popup_menu(&mut menu, None);
            }
        });

        dataview.on_selection_changed(move |event| {
            if let Some(item) = event.get_item()
                && let Some(ptr) = item.get_id::<MusicNode>()
            {
                let node: &MusicNode = unsafe { &*ptr };
                log::info!("Selected item: {}", node.title);
            }
        });

        let frame_for_activate = frame.clone();
        let model_for_activate = model.clone();
        dataview.on_item_activated(move |event| {
            if let Some(item) = event.get_item() {
                edit_item_with_dialog(&frame_for_activate, &model_for_activate, &item);
            }
        });

        dataview.on_column_header_click(move |event| {
            let col = event.get_column();
            log::info!("Column header clicked: {:?}", col);
            event.skip(true);
        });

        let dv_for_header_menu = dataview.clone();
        dataview.on_column_header_right_click(move |event| {
            if let Some(col_idx) = event.get_column() {
                log::info!("Column header right-clicked: {}", col_idx);

                // Build a simple sort menu
                let sort_asc_id = ID_HIGHEST + 101;
                let sort_desc_id = ID_HIGHEST + 102;
                let mut menu = Menu::builder()
                    .with_title("Sort")
                    .append_item(sort_asc_id, "Sort ascending", "Sort ascending")
                    .append_item(sort_desc_id, "Sort descending", "Sort descending")
                    .build();

                let dv_for_select = dv_for_header_menu.clone();
                menu.on_selected(move |ev| match ev.get_id() {
                    id if id == sort_asc_id => {
                        let _ = dv_for_select.set_sorting_column(col_idx as usize, true);
                        if let Some((c, asc)) = dv_for_select.sorting_state() {
                            log::info!("Programmatic sort applied: col={}, ascending={}", c, asc);
                        }
                    }
                    id if id == sort_desc_id => {
                        let _ = dv_for_select.set_sorting_column(col_idx as usize, false);
                        if let Some((c, asc)) = dv_for_select.sorting_state() {
                            log::info!("Programmatic sort applied: col={}, ascending={}", c, asc);
                        }
                    }
                    _ => {}
                });

                // Show the menu at mouse position if available
                let pos = event.get_position();
                dv_for_header_menu.popup_menu(&mut menu, pos);
            } else {
                log::info!("Column header right-clicked but no column index available");
            }
            // Don't block default processing
            event.skip(true);
        });

        dataview.on_column_sorted(move |event| {
            let col = event.get_column();
            let ascending = event.get_sort_order();
            log::info!("Column sorting changed: {:?}, ascending: {:?}", col, ascending);
        });

        dataview.on_item_expanded(move |event| {
            if let Some(item) = event.get_item()
                && let Some(ptr) = item.get_id::<MusicNode>()
            {
                let node: &MusicNode = unsafe { &*ptr };
                log::info!("Item expanded: {}", node.title);
            }
        });

        dataview.on_item_collapsed(move |event| {
            if let Some(item) = event.get_item()
                && let Some(ptr) = item.get_id::<MusicNode>()
            {
                let node: &MusicNode = unsafe { &*ptr };
                log::info!("Item collapsed: {}", node.title);
            }
        });

        frame.show(true);
        frame.centre();
    });
}

fn edit_item_with_dialog(parent: &dyn WxWidget, model: &CustomDataViewTreeModel, item: &DataViewItem) {
    if let Some(ptr) = item.get_id::<MusicNode>() {
        // SAFETY: ptr is an opaque model ID set by us to a MusicNode address
        let node: &MusicNode = unsafe { &*ptr };
        let dlg = edit_dialog::NodeEditDialog::new(parent, node);
        let ret = dlg.show_modal();
        if ret == ID_OK {
            let edited = dlg.value();
            // Mutate underlying data first, then notify the view that this item changed.
            let _changed = model.with_userdata_mut::<Rc<RefCell<MusicTree>>, bool>(|tree_rc| {
                if let Some(target_rc) = mymodels::find_node_via_raw_ptr(tree_rc, ptr) {
                    let mut target = target_rc.borrow_mut();
                    target.title = edited.title.clone();
                    if matches!(target.node_type, NodeType::Leaf) {
                        target.artist = edited.artist.clone();
                        target.year = edited.year;
                        target.quality = edited.quality.clone();
                    }
                    true
                } else {
                    false
                }
            });
            // Notify that this item's values changed so the row re-renders.
            model.item_changed::<MusicNode>(ptr); // or call `model.items_changed::<MusicNode>(&[ptr]);`
        }
    }
}
