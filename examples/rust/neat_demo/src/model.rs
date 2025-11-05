use crate::server_node::ServerNode;
use std::{cell::RefCell, rc::Rc};
use wxdragon::prelude::*;

pub type ServerRc = Rc<RefCell<ServerNode>>;

#[derive(Default, Debug, Clone)]
pub struct ServerList {
    pub nodes: Vec<ServerRc>,
}

/// Build a CustomDataViewTreeModel that exposes ServerList as a flat list under a virtual root.
/// - Root (None) is a container
/// - Each ServerNode is a leaf item
///   Columns (suggested):
///   0: Remarks (falls back to Host when empty)
///   1: Tunnel Path
///   2: Client ID
///   3: Server Host
///   4: Server Port
///   5: Server Domain
///   6: CA File/Content
///   7: TLS ("TLS"/"Plain", derived from `disable_tls`)
///   8: Dangerous ("Yes"/"No")
pub fn create_server_tree_model(data: Rc<RefCell<ServerList>>) -> CustomDataViewTreeModel {
    CustomDataViewTreeModel::new(
        data,
        // parent: top-level leaves have None (root) as parent
        move |_data: &Rc<RefCell<ServerList>>, _item: Option<&ServerNode>| None,
        // is_container: the virtual root (None) is a container; leaves are not
        move |_data: &Rc<RefCell<ServerList>>, item: Option<&ServerNode>| item.is_none(),
        // get_children: root returns all server nodes; leaves have no children
        move |data: &Rc<RefCell<ServerList>>, item: Option<&ServerNode>| match item {
            None => data
                .borrow()
                .nodes
                .iter()
                .map(|rc| &*rc.borrow() as *const ServerNode as *mut ServerNode)
                .collect(),
            Some(_leaf) => Vec::new(),
        },
        get_value_cb,
        Some(set_value_cb),
        Some(move |_: &Rc<RefCell<ServerList>>, _item: Option<&ServerNode>, _col: u32| true),
        Some(compare_cb),
    )
}

fn get_value_cb(data: &Rc<RefCell<ServerList>>, item: Option<&ServerNode>, col: u32) -> Variant {
    fn render(node: &ServerNode, col: u32) -> Variant {
        match col {
            0 => Variant::from_string(node.remarks.clone().unwrap_or_default()),
            1 => Variant::from_string(node.tunnel_path.clone()),
            2 => Variant::from_string(node.client_id.clone().unwrap_or_default()),
            3 => Variant::from_string(node.server_host.clone()),
            // DataViewTextRenderer expects a string; display port as string
            4 => Variant::from_string(node.server_port.to_string()),
            5 => Variant::from_string(node.server_domain.clone().unwrap_or_default()),
            6 => Variant::from_string(node.ca_file.clone().unwrap_or_default()),
            7 => Variant::from_bool(node.disable_tls.unwrap_or(false)),
            8 => Variant::from_bool(node.dangerous_mode.unwrap_or(false)),
            _ => Variant::from_string(String::new()),
        }
    }

    match item {
        None => {
            // Virtual root: show a summary in col 0, blanks elsewhere
            let count = data.borrow().nodes.len();
            if col == 0 {
                Variant::from_string(format!("{count} node(s)"))
            } else {
                Variant::from_string(String::new())
            }
        }
        Some(n) => render(n, col),
    }
}

fn set_value_cb(data: &Rc<RefCell<ServerList>>, item: Option<&ServerNode>, col: u32, var: &Variant) -> bool {
    let needle_ptr: *const ServerNode = match item {
        None => return false, // root is not editable
        Some(n) => n as *const _,
    };

    let target_rc = match find_node_via_raw_ptr(data, needle_ptr) {
        Some(rc) => rc,
        None => return false,
    };

    let mut node = target_rc.borrow_mut();
    match col {
        0 => {
            if let Some(s) = var.get_string() {
                node.remarks = if s.trim().is_empty() { None } else { Some(s) };
                true
            } else {
                false
            }
        }
        1 => {
            if let Some(s) = var.get_string() {
                node.tunnel_path = s;
                true
            } else {
                false
            }
        }
        2 => {
            if let Some(s) = var.get_string() {
                let s = s.trim().to_string();
                node.client_id = if s.is_empty() { None } else { Some(s) };
                true
            } else {
                false
            }
        }
        3 => {
            if let Some(s) = var.get_string() {
                node.server_host = s;
                true
            } else {
                false
            }
        }
        4 => {
            if let Some(v) = var.get_i32() {
                if v >= 0 && v <= u16::MAX as i32 {
                    node.server_port = v as u16;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
        5 => {
            if let Some(s) = var.get_string() {
                let s = s.trim().to_string();
                node.server_domain = if s.is_empty() { None } else { Some(s) };
                true
            } else {
                false
            }
        }
        6 => {
            if let Some(s) = var.get_string() {
                let s = s.trim().to_string();
                node.ca_file = if s.is_empty() { None } else { Some(s) };
                true
            } else {
                false
            }
        }
        7 => {
            if let Some(b) = var.get_bool() {
                // Store None for false to keep config concise
                node.disable_tls = if b { Some(true) } else { None };
                true
            } else {
                false
            }
        }
        8 => {
            if let Some(b) = var.get_bool() {
                node.dangerous_mode = if b { Some(true) } else { None };
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn compare_cb(_data: &Rc<RefCell<ServerList>>, a: &ServerNode, b: &ServerNode, col: u32, asc: bool) -> i32 {
    let ord = match col {
        0 => {
            let la = a.remarks.as_deref().unwrap_or(&a.server_host).to_lowercase();
            let lb = b.remarks.as_deref().unwrap_or(&b.server_host).to_lowercase();
            la.cmp(&lb)
        }
        1 => a.tunnel_path.to_lowercase().cmp(&b.tunnel_path.to_lowercase()),
        2 => a
            .client_id
            .as_deref()
            .unwrap_or("")
            .to_lowercase()
            .cmp(&b.client_id.as_deref().unwrap_or("").to_lowercase()),
        3 => a.server_host.to_lowercase().cmp(&b.server_host.to_lowercase()),
        4 => a.server_port.cmp(&b.server_port),
        5 => a
            .server_domain
            .as_deref()
            .unwrap_or("")
            .to_lowercase()
            .cmp(&b.server_domain.as_deref().unwrap_or("").to_lowercase()),
        6 => a
            .ca_file
            .as_deref()
            .unwrap_or("")
            .to_lowercase()
            .cmp(&b.ca_file.as_deref().unwrap_or("").to_lowercase()),
        7 => a.disable_tls.cmp(&b.disable_tls),
        8 => a.dangerous_mode.cmp(&b.dangerous_mode),
        _ => std::cmp::Ordering::Equal,
    };
    let v = match ord {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };
    if asc { v } else { -v }
}

/// Resolve target Rc<RefCell<ServerNode>> from raw pointer.
pub fn find_node_via_raw_ptr(list: &Rc<RefCell<ServerList>>, needle: *const ServerNode) -> Option<ServerRc> {
    let list_ref = list.borrow();
    for rc in list_ref.nodes.iter() {
        let ptr: *const ServerNode = &*rc.borrow();
        if ptr == needle {
            return Some(rc.clone());
        }
    }
    None
}
