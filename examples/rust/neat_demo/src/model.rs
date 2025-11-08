use crate::server_node::ServerNode;
use std::{cell::RefCell, rc::Rc};
use wxdragon::prelude::*;

pub type ServerRc = Rc<RefCell<ServerNode>>;

#[derive(Default, Debug, Clone)]
pub struct ServerList {
    pub nodes: Vec<ServerRc>,
}

bitflags::bitflags! {
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeFields : u32 {
    const Remarks = 0;
    const TunnelPath = 1;
    const ClientID = 2;
    const ServerHost = 3;
    const ServerPort = 4;
    const ServerDomain = 5;
    const CAFile = 6;
    const DisableTLS = 7;
    const DangerousMode = 8;
}
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
    fn render(node: &ServerNode, col: NodeFields) -> Variant {
        match col {
            NodeFields::Remarks => Variant::from_string(node.remarks.clone().unwrap_or_default()),
            NodeFields::TunnelPath => Variant::from_string(node.tunnel_path.clone()),
            NodeFields::ClientID => Variant::from_string(node.client_id.clone().unwrap_or_default()),
            NodeFields::ServerHost => Variant::from_string(node.server_host.clone()),
            // DataViewTextRenderer expects a string; display port as string
            NodeFields::ServerPort => Variant::from_string(node.server_port.to_string()),
            NodeFields::ServerDomain => Variant::from_string(node.server_domain.clone().unwrap_or_default()),
            NodeFields::CAFile => Variant::from_string(node.ca_file.clone().unwrap_or_default()),
            NodeFields::DisableTLS => Variant::from_bool(node.disable_tls.unwrap_or(false)),
            NodeFields::DangerousMode => Variant::from_bool(node.dangerous_mode.unwrap_or(false)),
            _ => Variant::from_string(String::new()),
        }
    }

    let col = NodeFields::from_bits_retain(col);

    match item {
        None => {
            // Virtual root: show a summary in col 0, blanks elsewhere
            let count = data.borrow().nodes.len();
            if col == NodeFields::Remarks {
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
    match NodeFields::from_bits_retain(col) {
        NodeFields::Remarks => {
            if let Some(s) = var.get_string() {
                node.remarks = if s.trim().is_empty() { None } else { Some(s) };
                true
            } else {
                false
            }
        }
        NodeFields::TunnelPath => {
            if let Some(s) = var.get_string() {
                node.tunnel_path = s;
                true
            } else {
                false
            }
        }
        NodeFields::ClientID => {
            if let Some(s) = var.get_string() {
                let s = s.trim().to_string();
                node.client_id = if s.is_empty() { None } else { Some(s) };
                true
            } else {
                false
            }
        }
        NodeFields::ServerHost => {
            if let Some(s) = var.get_string() {
                node.server_host = s;
                true
            } else {
                false
            }
        }
        NodeFields::ServerPort => {
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
        NodeFields::ServerDomain => {
            if let Some(s) = var.get_string() {
                let s = s.trim().to_string();
                node.server_domain = if s.is_empty() { None } else { Some(s) };
                true
            } else {
                false
            }
        }
        NodeFields::CAFile => {
            if let Some(s) = var.get_string() {
                let s = s.trim().to_string();
                node.ca_file = if s.is_empty() { None } else { Some(s) };
                true
            } else {
                false
            }
        }
        NodeFields::DisableTLS => {
            if let Some(b) = var.get_bool() {
                // Store None for false to keep config concise
                node.disable_tls = if b { Some(true) } else { None };
                true
            } else {
                false
            }
        }
        NodeFields::DangerousMode => {
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
    let ord = match NodeFields::from_bits_retain(col) {
        NodeFields::Remarks => {
            let la = a.remarks.as_deref().unwrap_or(&a.server_host).to_lowercase();
            let lb = b.remarks.as_deref().unwrap_or(&b.server_host).to_lowercase();
            la.cmp(&lb)
        }
        NodeFields::TunnelPath => a.tunnel_path.to_lowercase().cmp(&b.tunnel_path.to_lowercase()),
        NodeFields::ClientID => a
            .client_id
            .as_deref()
            .unwrap_or("")
            .to_lowercase()
            .cmp(&b.client_id.as_deref().unwrap_or("").to_lowercase()),
        NodeFields::ServerHost => a.server_host.to_lowercase().cmp(&b.server_host.to_lowercase()),
        NodeFields::ServerPort => a.server_port.cmp(&b.server_port),
        NodeFields::ServerDomain => a
            .server_domain
            .as_deref()
            .unwrap_or("")
            .to_lowercase()
            .cmp(&b.server_domain.as_deref().unwrap_or("").to_lowercase()),
        NodeFields::CAFile => a
            .ca_file
            .as_deref()
            .unwrap_or("")
            .to_lowercase()
            .cmp(&b.ca_file.as_deref().unwrap_or("").to_lowercase()),
        NodeFields::DisableTLS => a.disable_tls.cmp(&b.disable_tls),
        NodeFields::DangerousMode => a.dangerous_mode.cmp(&b.dangerous_mode),
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
