//! Toggle-first/text-second tree for the VoiceOver regression in #160.
use std::cell::{Cell, RefCell};
use wxdragon::DataViewToggleRenderer;
use wxdragon::prelude::*;

struct Node {
    index: usize,
    label: RefCell<String>,
    checked: Cell<bool>,
}

impl Node {
    fn container(&self) -> bool {
        self.index.is_multiple_of(2)
    }
}

struct Tree {
    // Boxes keep item addresses stable while wxWidgets holds their IDs.
    nodes: [Box<Node>; 4],
    empty_toggle: bool,
}

pub fn add_tree(panel: &Panel, sizer: &BoxSizer) {
    let instructions = StaticText::builder(panel)
        .with_label("Checkable groups (#160): checkbox first, text/expander second.\nUse arrows to select and expand groups; compare container columns on/off.")
        .build();
    let flags = SizerFlag::Expand | SizerFlag::All;
    sizer.add(&instructions, 0, flags, 8);
    let model = CustomDataViewTreeModel::new(
        Tree {
            nodes: std::array::from_fn(|index| {
                Box::new(Node {
                    index,
                    label: RefCell::new(["Group A", "Leaf A", "Group B", "Leaf B"][index].into()),
                    checked: Cell::new(true),
                })
            }),
            empty_toggle: false,
        },
        |tree: &Tree, node: Option<&Node>| {
            node.filter(|n| !n.container())
                .map(|n| &*tree.nodes[n.index - 1] as *const Node as *mut Node)
        },
        |_: &Tree, node: Option<&Node>| node.is_none_or(Node::container),
        |tree: &Tree, node: Option<&Node>| match node {
            None => vec![
                &*tree.nodes[0] as *const Node as *mut Node,
                &*tree.nodes[2] as *const Node as *mut Node,
            ],
            Some(n) if n.container() => vec![&*tree.nodes[n.index + 1] as *const Node as *mut Node],
            _ => Vec::new(),
        },
        |tree: &Tree, node: Option<&Node>, col| {
            let Some(node) = node else { return Variant::new() };
            if col == 1 {
                Variant::from(node.label.borrow().as_str())
            } else if tree.empty_toggle && node.container() {
                Variant::new()
            } else {
                Variant::from(node.checked.get())
            }
        },
        Some(|_: &Tree, node: Option<&Node>, col, value: &Variant| {
            if let (Some(node), Some(checked)) = (node, value.get_bool())
                && col == 0
            {
                node.checked.set(checked);
                return true;
            }
            false
        }),
        None::<fn(&Tree, Option<&Node>, u32) -> bool>,
        None::<fn(&Tree, &Node, &Node, u32, bool) -> i32>,
    );
    model.set_container_columns(true);
    let view = DataViewCtrl::builder(panel).with_size(Size::new(-1, 180)).build();
    view.append_column(DataViewColumn::new(
        "Check",
        DataViewToggleRenderer::new(VariantType::Bool, DataViewCellMode::Activatable, DataViewAlign::Left),
        0,
        80,
        DataViewAlign::Left,
        DataViewColumnFlags::Resizable,
    ));
    view.append_column(DataViewColumn::new(
        "Group / item",
        DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, DataViewAlign::Left),
        1,
        330,
        DataViewAlign::Left,
        DataViewColumnFlags::Resizable | DataViewColumnFlags::Sortable,
    ));
    if let Some(column) = view.get_column(1) {
        view.set_expander_column(&column);
    }
    view.associate_model(&model);
    sizer.add(&view, 1, flags, 8);

    let enabled = Cell::new(true);
    let policy_model = model.clone();
    let policy = Button::builder(panel)
        .with_label("Toggle container columns (initially on)")
        .build();
    policy.on_click(move |_| {
        enabled.set(!enabled.get());
        policy_model.set_container_columns(enabled.get());
    });
    sizer.add(&policy, 0, flags, 4);

    let renamed = Cell::new(false);
    let rename_model = model.clone();
    let rename = Button::builder(panel).with_label("Rename Group B (Unicode)").build();
    rename.on_click(move |_| {
        renamed.set(!renamed.get());
        if let Some(item) = rename_model.with_userdata_mut::<Tree, _>(|tree| {
            tree.nodes[2]
                .label
                .replace(if renamed.get() { "Group B — 支払い" } else { "Group B" }.into());
            &*tree.nodes[2] as *const Node
        }) {
            rename_model.item_value_changed(item, 1);
        }
    });
    sizer.add(&rename, 0, flags, 4);

    let empty = Button::builder(panel)
        .with_label("Toggle empty group checkbox values")
        .build();
    empty.on_click(move |_| {
        if let Some(items) = model.with_userdata_mut::<Tree, _>(|tree| {
            tree.empty_toggle = !tree.empty_toggle;
            [&*tree.nodes[0] as *const Node, &*tree.nodes[2] as *const Node]
        }) {
            model.items_changed(&items);
        }
    });
    sizer.add(&empty, 0, flags, 4);
}
