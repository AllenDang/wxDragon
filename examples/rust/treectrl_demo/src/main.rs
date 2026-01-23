//! TreeCtrl Demo - Tests all TreeCtrl APIs
//!
//! This example demonstrates all the TreeCtrl methods including the newly added ones.

use wxdragon::prelude::*;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("TreeCtrl Demo - All APIs")
            .with_size(Size::new(900, 700))
            .build();

        let main_sizer = BoxSizer::builder(Orientation::Horizontal).build();

        // Left panel: TreeCtrl
        let left_panel = Panel::builder(&frame).build();
        let left_sizer = BoxSizer::builder(Orientation::Vertical).build();

        let tree = TreeCtrl::builder(&left_panel)
            .with_style(TreeCtrlStyle::HasButtons | TreeCtrlStyle::LinesAtRoot | TreeCtrlStyle::EditLabels)
            .with_size(Size::new(350, 600))
            .build();

        log::info!("TreeCtrl created, is_valid: {}", tree.is_valid());

        // Build initial tree structure
        build_sample_tree(&tree);

        left_sizer.add(&tree, 1, SizerFlag::Expand | SizerFlag::All, 5);
        left_panel.set_sizer(left_sizer, true);

        // Right panel: Buttons and info
        let right_panel = Panel::builder(&frame).build();
        let right_sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Info display
        let info_text = StaticText::builder(&right_panel)
            .with_label("Select an item to see info")
            .build();
        right_sizer.add(&info_text, 0, SizerFlag::Expand | SizerFlag::All, 5);

        // Create button groups
        add_expand_collapse_buttons(&right_panel, &right_sizer, tree);
        add_selection_buttons(&right_panel, &right_sizer, tree);
        add_navigation_buttons(&right_panel, &right_sizer, tree);
        add_item_state_buttons(&right_panel, &right_sizer, tree);
        add_styling_buttons(&right_panel, &right_sizer, tree);
        add_management_buttons(&right_panel, &right_sizer, tree);
        add_misc_buttons(&right_panel, &right_sizer, tree);

        right_panel.set_sizer(right_sizer, true);

        // Tree event handlers
        let info_text_clone = info_text;
        let tree_clone = tree;
        tree.on_selection_changed(move |evt| {
            if let Some(item) = evt.get_item() {
                if let Some(text) = tree_clone.get_item_text(&item) {
                    let count = tree_clone.get_children_count(&item, false);
                    let is_exp = tree_clone.is_expanded(&item);
                    let is_bold = tree_clone.is_bold(&item);
                    info_text_clone.set_label(&format!(
                        "Selected: '{}'\nChildren: {}, Expanded: {}, Bold: {}",
                        text, count, is_exp, is_bold
                    ));
                }
            }
        });

        main_sizer.add(&left_panel, 1, SizerFlag::Expand | SizerFlag::All, 5);
        main_sizer.add(&right_panel, 0, SizerFlag::Expand | SizerFlag::All, 5);

        frame.set_sizer(main_sizer, true);
        frame.show(true);
        frame.centre();
    });
}

fn build_sample_tree(tree: &TreeCtrl) {
    log::info!("Building sample tree...");
    let root = match tree.add_root("Root", None, None) {
        Some(r) => {
            log::info!("Root added successfully");
            r
        }
        None => {
            log::error!("Failed to add root to tree - tree.is_valid() = {}", tree.is_valid());
            panic!("Failed to add root item to tree");
        }
    };

    // Add some branches
    let branch1 = tree.append_item(&root, "Branch 1", None, None).unwrap();
    tree.append_item(&branch1, "Leaf 1.1", None, None);
    tree.append_item(&branch1, "Leaf 1.2", None, None);
    tree.append_item(&branch1, "Leaf 1.3", None, None);

    let branch2 = tree.append_item(&root, "Branch 2", None, None).unwrap();
    let sub_branch = tree.append_item(&branch2, "Sub-branch 2.1", None, None).unwrap();
    tree.append_item(&sub_branch, "Deep Leaf", None, None);
    tree.append_item(&branch2, "Leaf 2.2", None, None);

    let branch3 = tree.append_item(&root, "Branch 3", None, None).unwrap();
    tree.append_item(&branch3, "Leaf 3.1", None, None);

    tree.expand(&root);
    tree.expand(&branch1);
}

fn add_expand_collapse_buttons(panel: &Panel, sizer: &BoxSizer, tree: TreeCtrl) {
    let label = StaticText::builder(panel).with_label("--- Expand/Collapse ---").build();
    sizer.add(&label, 0, SizerFlag::All, 5);

    let btn_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    let btn_expand_all = Button::builder(panel).with_label("Expand All").build();
    let tree1 = tree;
    btn_expand_all.on_click(move |_| {
        log::info!("ExpandAll");
        tree1.expand_all();
    });
    btn_sizer.add(&btn_expand_all, 0, SizerFlag::All, 2);

    let btn_collapse_all = Button::builder(panel).with_label("Collapse All").build();
    let tree2 = tree;
    btn_collapse_all.on_click(move |_| {
        log::info!("CollapseAll");
        tree2.collapse_all();
    });
    btn_sizer.add(&btn_collapse_all, 0, SizerFlag::All, 2);

    let btn_toggle = Button::builder(panel).with_label("Toggle Expand").build();
    let tree3 = tree;
    btn_toggle.on_click(move |_| {
        if let Some(item) = tree3.get_selection() {
            log::info!("Toggle expand/collapse for selected item");
            tree3.toggle(&item);
        }
    });
    btn_sizer.add(&btn_toggle, 0, SizerFlag::All, 2);

    let btn_is_expanded = Button::builder(panel).with_label("Is Expanded?").build();
    let tree4 = tree;
    btn_is_expanded.on_click(move |_| {
        if let Some(item) = tree4.get_selection() {
            let expanded = tree4.is_expanded(&item);
            log::info!("IsExpanded: {}", expanded);
        }
    });
    btn_sizer.add(&btn_is_expanded, 0, SizerFlag::All, 2);

    sizer.add_sizer(&btn_sizer, 0, SizerFlag::Expand, 0);
}

fn add_selection_buttons(panel: &Panel, sizer: &BoxSizer, tree: TreeCtrl) {
    let label = StaticText::builder(panel).with_label("--- Selection ---").build();
    sizer.add(&label, 0, SizerFlag::All, 5);

    let btn_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    let btn_is_selected = Button::builder(panel).with_label("Is Selected?").build();
    let tree1 = tree;
    btn_is_selected.on_click(move |_| {
        if let Some(item) = tree1.get_selection() {
            log::info!("IsSelected: {}", tree1.is_selected(&item));
        }
    });
    btn_sizer.add(&btn_is_selected, 0, SizerFlag::All, 2);

    let btn_unselect = Button::builder(panel).with_label("Unselect All").build();
    let tree2 = tree;
    btn_unselect.on_click(move |_| {
        log::info!("UnselectAll");
        tree2.unselect_all();
    });
    btn_sizer.add(&btn_unselect, 0, SizerFlag::All, 2);

    let btn_get_sels = Button::builder(panel).with_label("Get Selections").build();
    let tree3 = tree;
    btn_get_sels.on_click(move |_| {
        let sels = tree3.get_selections();
        log::info!("GetSelections count: {}", sels.len());
    });
    btn_sizer.add(&btn_get_sels, 0, SizerFlag::All, 2);

    sizer.add_sizer(&btn_sizer, 0, SizerFlag::Expand, 0);
}

fn add_navigation_buttons(panel: &Panel, sizer: &BoxSizer, tree: TreeCtrl) {
    let label = StaticText::builder(panel).with_label("--- Navigation ---").build();
    sizer.add(&label, 0, SizerFlag::All, 5);

    let btn_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    let btn_parent = Button::builder(panel).with_label("Get Parent").build();
    let tree1 = tree;
    btn_parent.on_click(move |_| {
        if let Some(item) = tree1.get_selection() {
            log::info!("Current selection: {:?}", tree1.get_item_text(&item));
            if let Some(parent) = tree1.get_item_parent(&item) {
                log::info!("Parent: {:?}", tree1.get_item_text(&parent));
                tree1.select_item(&parent);
                tree1.ensure_visible(&parent);
            } else {
                log::info!("No parent (root item)");
            }
        } else {
            log::info!("No item selected");
        }
    });
    btn_sizer.add(&btn_parent, 0, SizerFlag::All, 2);

    let btn_prev = Button::builder(panel).with_label("Prev Sibling").build();
    let tree2 = tree;
    btn_prev.on_click(move |_| {
        if let Some(item) = tree2.get_selection() {
            log::info!("Current selection: {:?}", tree2.get_item_text(&item));
            if let Some(prev) = tree2.get_prev_sibling(&item) {
                log::info!("Prev sibling: {:?}", tree2.get_item_text(&prev));
                tree2.select_item(&prev);
                tree2.ensure_visible(&prev);
            } else {
                log::info!("No previous sibling");
            }
        } else {
            log::info!("No item selected");
        }
    });
    btn_sizer.add(&btn_prev, 0, SizerFlag::All, 2);

    let btn_next = Button::builder(panel).with_label("Next Sibling").build();
    let tree3 = tree;
    btn_next.on_click(move |_| {
        if let Some(item) = tree3.get_selection() {
            log::info!("Current selection: {:?}", tree3.get_item_text(&item));
            if let Some(next) = tree3.get_next_sibling(&item) {
                log::info!("Next sibling: {:?}", tree3.get_item_text(&next));
                tree3.select_item(&next);
                tree3.ensure_visible(&next);
            } else {
                log::info!("No next sibling");
            }
        } else {
            log::info!("No item selected");
        }
    });
    btn_sizer.add(&btn_next, 0, SizerFlag::All, 2);

    let btn_last = Button::builder(panel).with_label("Last Child").build();
    let tree4 = tree;
    btn_last.on_click(move |_| {
        if let Some(item) = tree4.get_selection() {
            log::info!("Current selection: {:?}", tree4.get_item_text(&item));
            if let Some(last) = tree4.get_last_child(&item) {
                log::info!("Last child: {:?}", tree4.get_item_text(&last));
                tree4.select_item(&last);
                tree4.ensure_visible(&last);
            } else {
                log::info!("No children");
            }
        } else {
            log::info!("No item selected");
        }
    });
    btn_sizer.add(&btn_last, 0, SizerFlag::All, 2);

    // Add first child button for completeness
    let btn_first = Button::builder(panel).with_label("First Child").build();
    let tree5 = tree;
    btn_first.on_click(move |_| {
        if let Some(item) = tree5.get_selection() {
            log::info!("Current selection: {:?}", tree5.get_item_text(&item));
            if let Some((first, _cookie)) = tree5.get_first_child(&item) {
                log::info!("First child: {:?}", tree5.get_item_text(&first));
                tree5.select_item(&first);
                tree5.ensure_visible(&first);
            } else {
                log::info!("No children");
            }
        } else {
            log::info!("No item selected");
        }
    });
    btn_sizer.add(&btn_first, 0, SizerFlag::All, 2);

    sizer.add_sizer(&btn_sizer, 0, SizerFlag::Expand, 0);
}

fn add_item_state_buttons(panel: &Panel, sizer: &BoxSizer, tree: TreeCtrl) {
    let label = StaticText::builder(panel).with_label("--- Item State ---").build();
    sizer.add(&label, 0, SizerFlag::All, 5);

    let btn_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    let btn_visible = Button::builder(panel).with_label("Is Visible?").build();
    let tree1 = tree;
    btn_visible.on_click(move |_| {
        if let Some(item) = tree1.get_selection() {
            log::info!("IsVisible: {}", tree1.is_visible(&item));
        }
    });
    btn_sizer.add(&btn_visible, 0, SizerFlag::All, 2);

    let btn_has_children = Button::builder(panel).with_label("Has Children?").build();
    let tree2 = tree;
    btn_has_children.on_click(move |_| {
        if let Some(item) = tree2.get_selection() {
            log::info!("ItemHasChildren: {}", tree2.item_has_children(&item));
        }
    });
    btn_sizer.add(&btn_has_children, 0, SizerFlag::All, 2);

    let btn_toggle_bold = Button::builder(panel).with_label("Toggle Bold").build();
    let tree3 = tree;
    btn_toggle_bold.on_click(move |_| {
        if let Some(item) = tree3.get_selection() {
            let is_bold = tree3.is_bold(&item);
            tree3.set_item_bold(&item, !is_bold);
            log::info!("Set bold: {}", !is_bold);
        }
    });
    btn_sizer.add(&btn_toggle_bold, 0, SizerFlag::All, 2);

    sizer.add_sizer(&btn_sizer, 0, SizerFlag::Expand, 0);
}

fn add_styling_buttons(panel: &Panel, sizer: &BoxSizer, tree: TreeCtrl) {
    let label = StaticText::builder(panel).with_label("--- Styling ---").build();
    sizer.add(&label, 0, SizerFlag::All, 5);

    let btn_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    let btn_red_text = Button::builder(panel).with_label("Red Text").build();
    let tree1 = tree;
    btn_red_text.on_click(move |_| {
        if let Some(item) = tree1.get_selection() {
            tree1.set_item_text_colour(&item, Colour::new(255, 0, 0, 255));
            log::info!("Set text colour to red");
        }
    });
    btn_sizer.add(&btn_red_text, 0, SizerFlag::All, 2);

    let btn_blue_bg = Button::builder(panel).with_label("Blue Bg").build();
    let tree2 = tree;
    btn_blue_bg.on_click(move |_| {
        if let Some(item) = tree2.get_selection() {
            tree2.set_item_background_colour(&item, Colour::new(200, 200, 255, 255));
            log::info!("Set background colour to light blue");
        }
    });
    btn_sizer.add(&btn_blue_bg, 0, SizerFlag::All, 2);

    let btn_get_colors = Button::builder(panel).with_label("Get Colors").build();
    let tree3 = tree;
    btn_get_colors.on_click(move |_| {
        if let Some(item) = tree3.get_selection() {
            let tc = tree3.get_item_text_colour(&item);
            let bg = tree3.get_item_background_colour(&item);
            log::info!("Text: rgba({},{},{},{})", tc.r, tc.g, tc.b, tc.a);
            log::info!("Bg: rgba({},{},{},{})", bg.r, bg.g, bg.b, bg.a);
        }
    });
    btn_sizer.add(&btn_get_colors, 0, SizerFlag::All, 2);

    sizer.add_sizer(&btn_sizer, 0, SizerFlag::Expand, 0);
}

fn add_management_buttons(panel: &Panel, sizer: &BoxSizer, tree: TreeCtrl) {
    let label = StaticText::builder(panel).with_label("--- Item Management ---").build();
    sizer.add(&label, 0, SizerFlag::All, 5);

    let btn_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    let btn_prepend = Button::builder(panel).with_label("Prepend").build();
    let tree1 = tree;
    btn_prepend.on_click(move |_| {
        if let Some(item) = tree1.get_selection() {
            if let Some(new_item) = tree1.prepend_item(&item, "Prepended Item", None, None) {
                log::info!("Prepended new item");
                tree1.select_item(&new_item);
            }
        }
    });
    btn_sizer.add(&btn_prepend, 0, SizerFlag::All, 2);

    let btn_insert = Button::builder(panel).with_label("Insert At 0").build();
    let tree2 = tree;
    btn_insert.on_click(move |_| {
        if let Some(item) = tree2.get_selection() {
            if let Some(new_item) = tree2.insert_item_before(&item, 0, "Inserted at 0", None, None) {
                log::info!("Inserted item at position 0");
                tree2.select_item(&new_item);
            }
        }
    });
    btn_sizer.add(&btn_insert, 0, SizerFlag::All, 2);

    let btn_del_children = Button::builder(panel).with_label("Del Children").build();
    let tree3 = tree;
    btn_del_children.on_click(move |_| {
        if let Some(item) = tree3.get_selection() {
            tree3.delete_children(&item);
            log::info!("Deleted all children");
        }
    });
    btn_sizer.add(&btn_del_children, 0, SizerFlag::All, 2);

    let btn_set_text = Button::builder(panel).with_label("Rename").build();
    let tree4 = tree;
    btn_set_text.on_click(move |_| {
        if let Some(item) = tree4.get_selection() {
            tree4.set_item_text(&item, "Renamed Item");
            log::info!("Renamed item");
        }
    });
    btn_sizer.add(&btn_set_text, 0, SizerFlag::All, 2);

    sizer.add_sizer(&btn_sizer, 0, SizerFlag::Expand, 0);

    // Second row
    let btn_sizer2 = BoxSizer::builder(Orientation::Horizontal).build();

    let btn_count = Button::builder(panel).with_label("Get Count").build();
    let tree5 = tree;
    btn_count.on_click(move |_| {
        log::info!("Total items: {}", tree5.get_count());
    });
    btn_sizer2.add(&btn_count, 0, SizerFlag::All, 2);

    let btn_sort = Button::builder(panel).with_label("Sort Children").build();
    let tree6 = tree;
    btn_sort.on_click(move |_| {
        if let Some(item) = tree6.get_selection() {
            tree6.sort_children(&item);
            log::info!("Sorted children");
        }
    });
    btn_sizer2.add(&btn_sort, 0, SizerFlag::All, 2);

    let btn_reset = Button::builder(panel).with_label("Reset Tree").build();
    let tree7 = tree;
    btn_reset.on_click(move |_| {
        tree7.delete_all_items();
        build_sample_tree(&tree7);
        log::info!("Tree reset");
    });
    btn_sizer2.add(&btn_reset, 0, SizerFlag::All, 2);

    sizer.add_sizer(&btn_sizer2, 0, SizerFlag::Expand, 0);
}

fn add_misc_buttons(panel: &Panel, sizer: &BoxSizer, tree: TreeCtrl) {
    let label = StaticText::builder(panel).with_label("--- Misc ---").build();
    sizer.add(&label, 0, SizerFlag::All, 5);

    let btn_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    let btn_scroll = Button::builder(panel).with_label("Scroll To").build();
    let tree1 = tree;
    btn_scroll.on_click(move |_| {
        if let Some(item) = tree1.get_selection() {
            tree1.scroll_to(&item);
            log::info!("Scrolled to selected item");
        }
    });
    btn_sizer.add(&btn_scroll, 0, SizerFlag::All, 2);

    let btn_ensure = Button::builder(panel).with_label("Ensure Visible").build();
    let tree2 = tree;
    btn_ensure.on_click(move |_| {
        if let Some(item) = tree2.get_selection() {
            tree2.ensure_visible(&item);
            log::info!("Ensured item is visible");
        }
    });
    btn_sizer.add(&btn_ensure, 0, SizerFlag::All, 2);

    let btn_bounds = Button::builder(panel).with_label("Get Bounds").build();
    let tree3 = tree;
    btn_bounds.on_click(move |_| {
        if let Some(item) = tree3.get_selection() {
            if let Some(rect) = tree3.get_bounding_rect(&item, false) {
                log::info!(
                    "Bounding rect: x={}, y={}, w={}, h={}",
                    rect.x,
                    rect.y,
                    rect.width,
                    rect.height
                );
            } else {
                log::info!("Could not get bounding rect (item not visible?)");
            }
        }
    });
    btn_sizer.add(&btn_bounds, 0, SizerFlag::All, 2);

    let btn_focused = Button::builder(panel).with_label("Get Focused").build();
    let tree4 = tree;
    btn_focused.on_click(move |_| {
        if let Some(item) = tree4.get_focused_item() {
            log::info!("Focused item: {:?}", tree4.get_item_text(&item));
        } else {
            log::info!("No focused item");
        }
    });
    btn_sizer.add(&btn_focused, 0, SizerFlag::All, 2);

    sizer.add_sizer(&btn_sizer, 0, SizerFlag::Expand, 0);

    // Hit test demo
    let btn_sizer2 = BoxSizer::builder(Orientation::Horizontal).build();

    let btn_hit_test = Button::builder(panel).with_label("Hit Test (100,100)").build();
    let tree5 = tree;
    btn_hit_test.on_click(move |_| {
        let (item, flags) = tree5.hit_test(Point::new(100, 100));
        if let Some(item) = item {
            log::info!("Hit test at (100,100): {:?}, flags: {:?}", tree5.get_item_text(&item), flags);
        } else {
            log::info!("Hit test at (100,100): No item, flags: {:?}", flags);
        }
    });
    btn_sizer2.add(&btn_hit_test, 0, SizerFlag::All, 2);

    let btn_set_has_children = Button::builder(panel).with_label("Fake Children").build();
    let tree6 = tree;
    btn_set_has_children.on_click(move |_| {
        if let Some(item) = tree6.get_selection() {
            let has = tree6.item_has_children(&item);
            tree6.set_item_has_children(&item, !has);
            log::info!("Set item has children (show +/- button): {}", !has);
        }
    });
    btn_sizer2.add(&btn_set_has_children, 0, SizerFlag::All, 2);

    sizer.add_sizer(&btn_sizer2, 0, SizerFlag::Expand, 0);

    // System utilities section
    let label2 = StaticText::builder(panel).with_label("--- System Utils ---").build();
    sizer.add(&label2, 0, SizerFlag::All, 5);

    let btn_sizer3 = BoxSizer::builder(Orientation::Horizontal).build();

    let btn_bell = Button::builder(panel).with_label("Bell").build();
    btn_bell.on_click(move |_| {
        log::info!("Playing system bell");
        bell();
    });
    btn_sizer3.add(&btn_bell, 0, SizerFlag::All, 2);

    let btn_browser = Button::builder(panel).with_label("Open Browser").build();
    btn_browser.on_click(move |_| {
        log::info!("Launching default browser");
        let result = launch_default_browser("https://github.com/aspect-rs/wxDragon", BrowserLaunchFlags::Default);
        log::info!("Browser launch result: {}", result);
    });
    btn_sizer3.add(&btn_browser, 0, SizerFlag::All, 2);

    let btn_browser_new = Button::builder(panel).with_label("Browser (New Window)").build();
    btn_browser_new.on_click(move |_| {
        log::info!("Launching default browser in new window");
        let result = launch_default_browser("https://www.rust-lang.org", BrowserLaunchFlags::NewWindow);
        log::info!("Browser launch result: {}", result);
    });
    btn_sizer3.add(&btn_browser_new, 0, SizerFlag::All, 2);

    sizer.add_sizer(&btn_sizer3, 0, SizerFlag::Expand, 0);
}
