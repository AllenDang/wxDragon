use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Serialize, Deserialize,
)]
pub enum NodeType {
    Branch,
    #[default]
    Leaf,
}

#[derive(Debug, Clone, Default)]
pub struct MusicNode {
    pub node_type: NodeType,
    pub title: String,
    pub artist: Option<String>,
    pub year: Option<i32>,
    pub quality: Option<String>,
    // Children are optional: Branch has Some(Vec<..>), Leaf has None.
    // Each child is Rc<RefCell<MusicNode>> to allow interior mutations.
    pub children: Option<Vec<Rc<RefCell<MusicNode>>>>,
}

impl Drop for MusicNode {
    fn drop(&mut self) {
        // log::trace!("Dropping MusicNode: {}", self.title);
    }
}

impl MusicNode {
    pub fn new(
        node_type: NodeType,
        title: &str,
        artist: Option<&str>,
        year: Option<i32>,
        quality: Option<&str>,
    ) -> Rc<RefCell<Self>> {
        let (artist_s, year_o, quality_s, children) = match node_type {
            NodeType::Branch => (None, None, None, Some(Vec::new())),
            NodeType::Leaf => (
                artist.map(|s| s.to_string()),
                year,
                quality.map(|s| s.to_string()),
                None,
            ),
        };
        Rc::new(RefCell::new(Self {
            node_type,
            title: title.to_string(),
            artist: artist_s,
            year: year_o,
            quality: quality_s,
            children,
        }))
    }

    /// Append a child to this node's children.
    pub fn push_child(parent: &Rc<RefCell<MusicNode>>, child: Rc<RefCell<MusicNode>>) {
        let mut p = parent.borrow_mut();
        if p.children.is_none() {
            p.children = Some(Vec::new());
        }
        if let Some(children) = p.children.as_mut() {
            children.push(child);
        }
    }
}

#[derive(Debug, Clone)]
pub struct MusicTree {
    pub root: Rc<RefCell<MusicNode>>,
    pub filepath: Option<PathBuf>,
}

impl Drop for MusicTree {
    fn drop(&mut self) {
        log::trace!("Dropping MusicTree {self:p}");
        if let Some(path) = &self.filepath {
            save_music_tree_to_file(self, path).ok();
        }
    }
}

// ----------------------------
// Serde support (DTO approach)
// ----------------------------
// Directly serializing Rc/RefCell/Weak is not practical and may introduce cycles.
// We snapshot the data into a plain, tree-shaped DTO and reconstruct the
// runtime structure on deserialization.

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MusicNodeDto {
    node_type: NodeType,
    title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    artist: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    year: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    quality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    children: Option<Vec<MusicNodeDto>>,
}

impl MusicNode {
    fn to_dto(&self) -> MusicNodeDto {
        let (artist, year, quality) = match self.node_type {
            NodeType::Branch => (None, None, None),
            NodeType::Leaf => (self.artist.clone(), self.year, self.quality.clone()),
        };
        let children = match self.node_type {
            NodeType::Branch => Some(
                self.children
                    .as_ref()
                    .map(|v| v.iter().map(|c| c.borrow().to_dto()).collect())
                    .unwrap_or_default(),
            ),
            NodeType::Leaf => None,
        };
        MusicNodeDto {
            node_type: self.node_type,
            title: self.title.clone(),
            artist,
            year,
            quality,
            children,
        }
    }

    fn from_dto(dto: &MusicNodeDto) -> Rc<RefCell<MusicNode>> {
        let node = MusicNode::new(
            dto.node_type,
            &dto.title,
            dto.artist.as_deref(),
            dto.year,
            dto.quality.as_deref(),
        );

        if matches!(dto.node_type, NodeType::Branch) {
            if let Some(children) = &dto.children {
                for child_dto in children {
                    let child = MusicNode::from_dto(child_dto);
                    MusicNode::push_child(&node, child);
                }
            }
        }
        node
    }
}

impl Serialize for MusicTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let dto: MusicNodeDto = self.root.borrow().to_dto();
        dto.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MusicTree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let dto = MusicNodeDto::deserialize(deserializer)?;
        Ok(MusicTree {
            root: MusicNode::from_dto(&dto),
            filepath: None,
        })
    }
}

impl MusicTree {
    /// Find the parent of `target` by doing a DFS from the root.
    /// Returns None if `target` is the root or not found.
    pub fn parent_of(&self, target: &MusicNode) -> Option<Rc<RefCell<MusicNode>>> {
        let target_ptr: *const MusicNode = target as *const _;

        // Early return if target is root
        {
            let root_borrow = self.root.borrow();
            let root_ptr: *const MusicNode = &*root_borrow;
            if root_ptr == target_ptr {
                return None;
            }
        }

        fn dfs(
            cur: &Rc<RefCell<MusicNode>>,
            target_ptr: *const MusicNode,
        ) -> Option<Rc<RefCell<MusicNode>>> {
            // Clone children Rc list to limit RefCell borrow scope
            let children: Vec<Rc<RefCell<MusicNode>>> =
                cur.borrow().children.as_ref().cloned().unwrap_or_default();
            for child in children.iter() {
                let child_ptr: *const MusicNode = &*child.borrow();
                if child_ptr == target_ptr {
                    return Some(cur.clone());
                }
                if let Some(found) = dfs(child, target_ptr) {
                    return Some(found);
                }
            }
            None
        }

        dfs(&self.root, target_ptr)
    }
}

pub fn load_music_tree_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<MusicTree> {
    let data = std::fs::read_to_string(path.as_ref())?;
    let mut tree: MusicTree = serde_json::from_str(&data)?;
    tree.filepath = Some(path.as_ref().to_path_buf());
    Ok(tree)
}

pub fn save_music_tree_to_file<P: AsRef<Path>>(tree: &MusicTree, path: P) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(tree).map_err(std::io::Error::other)?;
    std::fs::write(path, json)
}

pub fn generate_initial_music_tree() -> MusicTree {
    // Build nodes
    let root = MusicNode::new(NodeType::Branch, "My Music", None, None, None);

    let pop = MusicNode::new(NodeType::Branch, "Pop music", None, None, None);
    MusicNode::push_child(
        &pop,
        MusicNode::new(
            NodeType::Leaf,
            "You are not alone",
            Some("Michael Jackson"),
            Some(1995),
            Some("good"),
        ),
    );
    MusicNode::push_child(
        &pop,
        MusicNode::new(
            NodeType::Leaf,
            "Yesterday",
            Some("The Beatles"),
            None,
            Some("good"),
        ),
    );
    MusicNode::push_child(
        &pop,
        MusicNode::new(
            NodeType::Leaf,
            "Take a bow",
            Some("Madonna"),
            Some(1994),
            Some("good"),
        ),
    );
    MusicNode::push_child(&root, pop);

    let classical = MusicNode::new(NodeType::Branch, "Classical music", None, None, None);
    MusicNode::push_child(
        &classical,
        MusicNode::new(
            NodeType::Leaf,
            "Ninth symphony",
            Some("Ludwig van Beethoven"),
            Some(1824),
            Some("good"),
        ),
    );
    MusicNode::push_child(
        &classical,
        MusicNode::new(
            NodeType::Leaf,
            "German Requiem",
            Some("Johannes Brahms"),
            Some(1868),
            Some("good"),
        ),
    );
    MusicNode::push_child(&root, classical);

    MusicTree {
        root,
        filepath: None,
    }
}
