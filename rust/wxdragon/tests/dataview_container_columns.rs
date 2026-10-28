//! GUI regression for the safe container-column API. Requires a desktop.
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wxdragon::DataViewToggleRenderer;
use wxdragon::prelude::*;

struct Node {
    container: bool,
    label: &'static str,
}

struct Tree {
    group: Box<Node>,
    leaf: Box<Node>,
    label_reads: Rc<Cell<usize>>,
    dropped: Rc<Cell<bool>>,
}

impl Drop for Tree {
    fn drop(&mut self) {
        self.dropped.set(true);
    }
}

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let passed = Rc::new(Cell::new(0));
    let passed_in_loop = passed.clone();
    let dropped = Rc::new(Cell::new(false));
    let dropped_in_loop = dropped.clone();
    let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
    let timer_in_loop = timer_store.clone();

    wxdragon::main(move |app| {
        let reads = Rc::new(Cell::new(0));
        let model = CustomDataViewTreeModel::new(
            Tree {
                group: Box::new(Node {
                    container: true,
                    label: "Group B — 支払い",
                }),
                leaf: Box::new(Node {
                    container: false,
                    label: "Leaf B",
                }),
                label_reads: reads.clone(),
                dropped: dropped_in_loop,
            },
            |tree: &Tree, node: Option<&Node>| {
                node.filter(|n| !n.container)
                    .map(|_| &*tree.group as *const Node as *mut Node)
            },
            |_: &Tree, node: Option<&Node>| node.is_none_or(|n| n.container),
            |tree: &Tree, node: Option<&Node>| match node {
                None => vec![&*tree.group as *const Node as *mut Node],
                Some(n) if n.container => vec![&*tree.leaf as *const Node as *mut Node],
                _ => Vec::new(),
            },
            |tree: &Tree, node: Option<&Node>, col| {
                let node = node.expect("invisible root must not request a value");
                if col == 0 {
                    Variant::from(true)
                } else {
                    if node.container {
                        tree.label_reads.set(tree.label_reads.get() + 1);
                    }
                    Variant::from(node.label)
                }
            },
            None::<fn(&Tree, Option<&Node>, u32, &Variant) -> bool>,
            None::<fn(&Tree, Option<&Node>, u32) -> bool>,
            None::<fn(&Tree, &Node, &Node, u32, bool) -> i32>,
        );
        // Exercise setting the existing default before association, and cloning.
        model.set_container_columns(false);
        let cloned_model = model.clone();
        let frame = Frame::builder()
            .with_title("Container columns")
            .with_size(Size::new(400, 280))
            .build();
        let view = DataViewCtrl::builder(&frame).build();
        view.append_column(DataViewColumn::new(
            "Check",
            DataViewToggleRenderer::new(VariantType::Bool, DataViewCellMode::Activatable, DataViewAlign::Left),
            0,
            70,
            DataViewAlign::Left,
            DataViewColumnFlags::Resizable,
        ));
        view.append_column(DataViewColumn::new(
            "Label",
            DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, DataViewAlign::Left),
            1,
            280,
            DataViewAlign::Left,
            DataViewColumnFlags::Resizable,
        ));
        if let Some(column) = view.get_column(1) {
            view.set_expander_column(&column);
        }
        view.associate_model(&model);
        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        sizer.add(&view, 1, SizerFlag::Expand, 0);
        frame.set_sizer(sizer, true);
        frame.show(true);
        drop(model); // The cloned handle and control retain the native model.

        let phase = Cell::new(0);
        let timer = Timer::new(&frame);
        let timer_cleanup = timer_in_loop.clone();
        timer.on_tick(move |_| {
            let step = phase.get();
            phase.set(step + 1); // A caught assertion panic must not strand the loop.
            if step >= 2 {
                app.exit_main_loop();
                timer_cleanup.borrow_mut().take();
            }
            match step {
                0 => {
                    assert_eq!(reads.get(), 0, "default fetched a suppressed container label");
                    passed_in_loop.set(passed_in_loop.get() + 1);
                    cloned_model.set_container_columns(true);
                }
                1 => {
                    assert!(reads.get() > 0, "opt-in did not fetch the group label through Rust callbacks");
                    passed_in_loop.set(passed_in_loop.get() + 1);
                    cloned_model.set_container_columns(false);
                    reads.set(0);
                }
                2 => {
                    assert_eq!(reads.get(), 0, "disabled policy still fetched the container label");
                    passed_in_loop.set(passed_in_loop.get() + 1);
                }
                _ => {}
            }
        });
        timer.start(150, false);
        timer_in_loop.borrow_mut().replace(timer);
    })
    .expect("could not run container-column regressions");
    assert_eq!(passed.get(), 3, "container-column assertions did not finish");
    assert!(dropped.get(), "model userdata was not released");
}
