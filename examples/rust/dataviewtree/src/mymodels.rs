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
        get_value_cb,
        Some(set_value_cb),
        Some(move |_: &Rc<RefCell<MusicTree>>, _: Option<&MusicNode>, _: u32| true),
        Some(compare_cb),
    )
}

fn get_value_cb(data: &Rc<RefCell<MusicTree>>, item: Option<&MusicNode>, col: u32) -> Variant {
    fn _get_value(r: &MusicNode, col: u32) -> Variant {
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
    match item {
        None => {
            let tree = data.borrow();
            let r_b = tree.root.borrow();
            let r = &*r_b;
            _get_value(r, col)
        }
        Some(n) => _get_value(n, col),
    }
}

fn set_value_cb(data: &Rc<RefCell<MusicTree>>, item: Option<&MusicNode>, col: u32, var: &Variant) -> bool {
    let target_rc = match item {
        None => data.borrow().root.clone(),
        Some(n) => {
            let ptr: *const MusicNode = n as *const _;
            match find_node_via_raw_ptr(data, ptr) {
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
}

fn compare_cb(_tree: &Rc<RefCell<MusicTree>>, a: &MusicNode, b: &MusicNode, col: u32, asc: bool) -> i32 {
    if col == 0 {
        let va = a.title.to_lowercase();
        let vb = b.title.to_lowercase();
        if asc { va.cmp(&vb) as i32 } else { vb.cmp(&va) as i32 }
    } else if col == 1 {
        let va = a.artist.as_deref().unwrap_or("").to_lowercase();
        let vb = b.artist.as_deref().unwrap_or("").to_lowercase();
        if asc { va.cmp(&vb) as i32 } else { vb.cmp(&va) as i32 }
    } else if col == 2 {
        let va = a.year.unwrap_or(0);
        let vb = b.year.unwrap_or(0);
        // return negative if a < b, positive if a > b
        let diff = va.wrapping_sub(vb);
        if asc { diff } else { -diff }
    } else {
        0
    }
}

// Resolve target Rc<RefCell<MusicNode>> from item pointer (or root when None)
pub fn find_node_via_raw_ptr(tree: &Rc<RefCell<MusicTree>>, needle: *const MusicNode) -> Option<Rc<RefCell<MusicNode>>> {
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
