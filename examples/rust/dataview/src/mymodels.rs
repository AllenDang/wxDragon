use crate::music_tree::{MusicNode, MusicTree, NodeType};
use std::{cell::RefCell, rc::Rc};
use wxdragon::prelude::*;

pub fn create_music_tree_model(data: Rc<RefCell<MusicTree>>) -> CustomDataViewTreeModel {
    CustomDataViewTreeModel::new(
        data,
        move |data: &Rc<RefCell<MusicTree>>, item: Option<&MusicNode>| match item {
            None => None,
            Some(node) => data
                .borrow()
                .parent_of(node)
                .map(|rc| &*rc.borrow() as *const MusicNode as *mut MusicNode),
        },
        move |_data, item: Option<&MusicNode>| match item {
            None => true,
            Some(node) => matches!(node.node_type, NodeType::Branch),
        },
        move |data: &Rc<RefCell<MusicTree>>, item: Option<&MusicNode>| match item {
            None => vec![&*data.borrow().root.borrow() as *const MusicNode as *mut MusicNode],
            Some(node) => node
                .children
                .as_ref()
                .into_iter()
                .flat_map(|v| v.iter())
                .map(|b| &*b.borrow() as *const MusicNode as *mut MusicNode)
                .collect(),
        },
        move |data: &Rc<RefCell<MusicTree>>, item: Option<&MusicNode>, col| match item {
            None => {
                let tree = data.borrow();
                let r_b = tree.root.borrow();
                let r = &*r_b;
                match col {
                    0 => Variant::from_string(r.title.clone()),
                    1 => Variant::from_string(r.artist.clone().unwrap_or_default()),
                    2 => match r.year {
                        Some(v) => Variant::from_string(v.to_string()),
                        None => Variant::from_string(String::new()),
                    },
                    3 => Variant::from_string(r.quality.clone().unwrap_or_default()),
                    _ => Variant::from_string(String::new()),
                }
            }
            Some(n) => match col {
                0 => Variant::from_string(n.title.clone()),
                1 => Variant::from_string(n.artist.clone().unwrap_or_default()),
                2 => match n.year {
                    Some(v) => Variant::from_string(v.to_string()),
                    None => Variant::from_string(String::new()),
                },
                3 => Variant::from_string(n.quality.clone().unwrap_or_default()),
                _ => Variant::from_string(String::new()),
            },
        },
        Some(
            move |data: &Rc<RefCell<MusicTree>>,
                  item: Option<&MusicNode>,
                  col: u32,
                  var: &Variant| {
                let target_rc = match item {
                    None => data.borrow().root.clone(),
                    Some(n) => {
                        let ptr: *const MusicNode = n as *const _;
                        match find_rc(data, ptr) {
                            Some(rc) => rc,
                            None => return false,
                        }
                    }
                };

                let mut node = target_rc.borrow_mut();
                match col {
                    0 => {
                        if let Some(s) = var.get_string() {
                            node.title = s;
                            true
                        } else {
                            false
                        }
                    }
                    1 => {
                        if matches!(node.node_type, NodeType::Branch) {
                            return false;
                        }
                        if let Some(s) = var.get_string() {
                            node.artist = if s.is_empty() { None } else { Some(s) };
                            true
                        } else {
                            false
                        }
                    }
                    2 => {
                        if matches!(node.node_type, NodeType::Branch) {
                            return false;
                        }
                        if let Some(s) = var.get_string() {
                            let s_trim = s.trim().to_string();
                            if s_trim.is_empty() {
                                node.year = None;
                                return true;
                            }
                            match s_trim.parse::<i32>() {
                                Ok(v) => {
                                    node.year = Some(v);
                                    true
                                }
                                Err(_) => false,
                            }
                        } else {
                            false
                        }
                    }
                    3 => {
                        if matches!(node.node_type, NodeType::Branch) {
                            return false;
                        }
                        if let Some(s) = var.get_string() {
                            node.quality = if s.is_empty() { None } else { Some(s) };
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            },
        ),
        Some(move |_: &Rc<RefCell<MusicTree>>, _: Option<&MusicNode>, _: u32| true),
        Some(
            move |_: &Rc<RefCell<MusicTree>>, a: &MusicNode, b: &MusicNode, col: u32, asc: bool| {
                if col == 2 {
                    let va = a.year.unwrap_or(0);
                    let vb = b.year.unwrap_or(0);
                    // return negative if a < b, positive if a > b
                    let diff = va.wrapping_sub(vb);
                    if asc { diff } else { -diff }
                } else {
                    0
                }
            },
        ),
    )
}

// Resolve target Rc<RefCell<MusicNode>> from item pointer (or root when None)
fn find_rc(
    tree: &Rc<RefCell<MusicTree>>,
    needle: *const MusicNode,
) -> Option<Rc<RefCell<MusicNode>>> {
    let tree = tree.borrow();
    let root_ptr: *const MusicNode = &*tree.root.borrow();
    if root_ptr == needle {
        return Some(tree.root.clone());
    }
    dfs(&tree.root, needle)
}

fn dfs(cur: &Rc<RefCell<MusicNode>>, target: *const MusicNode) -> Option<Rc<RefCell<MusicNode>>> {
    let children = cur.borrow().children.as_ref().cloned().unwrap_or_default();
    for child in children.iter() {
        let ptr: *const MusicNode = &*child.borrow();
        if ptr == target {
            return Some(child.clone());
        }
        if let Some(found) = dfs(child, target) {
            return Some(found);
        }
    }
    None
}
