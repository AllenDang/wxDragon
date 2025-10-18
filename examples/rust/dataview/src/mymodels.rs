use std::cell::RefCell;
use std::rc::{Rc, Weak};
use wxdragon::widgets::dataview::model::CustomDataViewTreeModel;
use wxdragon::widgets::dataview::variant::Variant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum NodeType {
    Branch,
    #[default]
    Leaf,
}

#[derive(Debug, Clone, Default)]
pub struct MyMusicTreeNode {
    pub node_type: NodeType,
    pub title: String,
    pub artist: String,
    pub year: Option<i32>,
    pub quality: String,
    // Parent is a Weak pointer stored in a RefCell so we can set it after
    // constructing the child Rc.
    pub parent: RefCell<Option<Weak<MyMusicTreeNode>>>,
    // Children vector is mutable via RefCell, but the node itself is stored
    // in an `Rc<MyMusicTreeNode>` so `Rc::as_ptr` yields a pointer to the
    // inner `MyMusicTreeNode` allocation (no extra RefCell wrapper).
    pub children: RefCell<Vec<Rc<MyMusicTreeNode>>>,
}

impl Drop for MyMusicTreeNode {
    fn drop(&mut self) {
        log::debug!("Dropping MyMusicTreeNode: {}", self.title);
    }
}

impl MyMusicTreeNode {
    pub fn new(
        node_type: NodeType,
        title: &str,
        artist: &str,
        year: Option<i32>,
        quality: &str,
    ) -> Rc<Self> {
        Rc::new(Self {
            node_type,
            parent: RefCell::new(None),
            title: title.to_string(),
            artist: artist.to_string(),
            year,
            quality: quality.to_string(),
            children: RefCell::new(Vec::new()),
        })
    }

    /// Append a child and set its parent weak pointer.
    pub fn push_child(self: &Rc<MyMusicTreeNode>, child: Rc<MyMusicTreeNode>) {
        *child.parent.borrow_mut() = Some(Rc::downgrade(self));
        self.children.borrow_mut().push(child);
    }
}

#[derive(Debug, Clone)]
pub struct MyMusicData {
    pub root: Rc<MyMusicTreeNode>,
}

impl Drop for MyMusicData {
    fn drop(&mut self) {
        log::debug!("Dropping MyMusicData {self:p}");
    }
}

pub fn create_music_tree_model() -> Option<CustomDataViewTreeModel> {
    // Build nodes
    let root = MyMusicTreeNode::new(NodeType::Branch, "My Music", "", None, "");

    let pop = MyMusicTreeNode::new(NodeType::Branch, "Pop music", "", None, "");
    MyMusicTreeNode::push_child(
        &pop,
        MyMusicTreeNode::new(
            NodeType::Leaf,
            "You are not alone",
            "Michael Jackson",
            Some(1995),
            "good",
        ),
    );
    MyMusicTreeNode::push_child(
        &pop,
        MyMusicTreeNode::new(NodeType::Leaf, "Yesterday", "The Beatles", None, "good"),
    );
    MyMusicTreeNode::push_child(
        &pop,
        MyMusicTreeNode::new(NodeType::Leaf, "Take a bow", "Madonna", Some(1994), "good"),
    );
    MyMusicTreeNode::push_child(&root, pop);

    let classical = MyMusicTreeNode::new(NodeType::Branch, "Classical music", "", None, "");
    MyMusicTreeNode::push_child(
        &classical,
        MyMusicTreeNode::new(
            NodeType::Leaf,
            "Ninth symphony",
            "Ludwig van Beethoven",
            Some(1824),
            "good",
        ),
    );
    MyMusicTreeNode::push_child(
        &classical,
        MyMusicTreeNode::new(
            NodeType::Leaf,
            "German Requiem",
            "Johannes Brahms",
            Some(1868),
            "good",
        ),
    );
    MyMusicTreeNode::push_child(&root, classical);

    // Package userdata (model will Box this value internally). Move the Rc
    // into the data so the model owns the tree userdata.
    let data = MyMusicData { root };

    let model = CustomDataViewTreeModel::new(
        data,
        move |_data: &MyMusicData, item: Option<&MyMusicTreeNode>| match item {
            None => None,
            Some(node) => {
                // Borrow the parent RefCell and inspect the Option<Weak<_>>
                match node.parent.borrow().as_ref() {
                    Some(w) => w
                        .upgrade()
                        .map(|rc| Rc::as_ptr(&rc) as *mut MyMusicTreeNode),
                    None => None,
                }
            }
        },
        move |_data, item: Option<&MyMusicTreeNode>| match item {
            None => true,
            Some(node) => matches!(node.node_type, NodeType::Branch),
        },
        move |data: &MyMusicData, item: Option<&MyMusicTreeNode>| match item {
            None => {
                vec![Rc::as_ptr(&data.root) as *mut MyMusicTreeNode]
            }
            Some(node) => node
                .children
                .borrow()
                .iter()
                .map(|b| Rc::as_ptr(b) as *mut MyMusicTreeNode)
                .collect(),
        },
        move |data: &MyMusicData, item: Option<&MyMusicTreeNode>, col| match item {
            None => {
                let r = &*data.root;
                match col {
                    0 => Variant::String(r.title.clone()),
                    1 => Variant::String(r.artist.clone()),
                    2 => match r.year {
                        Some(v) => Variant::String(v.to_string()),
                        None => Variant::String(String::new()),
                    },
                    3 => Variant::String(r.quality.clone()),
                    _ => Variant::String(String::new()),
                }
            }
            Some(n) => match col {
                0 => Variant::String(n.title.clone()),
                1 => Variant::String(n.artist.clone()),
                2 => match n.year {
                    Some(v) => Variant::String(v.to_string()),
                    None => Variant::String(String::new()),
                },
                3 => Variant::String(n.quality.clone()),
                _ => Variant::String(String::new()),
            },
        },
        Some(move |_: &MyMusicData, _: Option<&MyMusicTreeNode>, _: u32, _: &Variant| false),
        Some(move |_: &MyMusicData, _: Option<&MyMusicTreeNode>, _: u32| true),
        Some(
            move |_: &MyMusicData,
                  a: &MyMusicTreeNode,
                  b: &MyMusicTreeNode,
                  col: u32,
                  asc: bool| {
                if col == 2 {
                    let va = a.year.unwrap_or(0);
                    let vb = b.year.unwrap_or(0);
                    // return negative if a < b, positive if a > b
                    let diff = va.wrapping_sub(vb);
                    if asc {
                        diff
                    } else {
                        -diff
                    }
                } else {
                    0
                }
            },
        ),
    );

    Some(model)
}
