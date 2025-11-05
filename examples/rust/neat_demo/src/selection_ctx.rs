use std::{cell::RefCell, rc::Weak};

use crate::server_node::ServerNode;

thread_local! {
    // Store a weak reference to the node in the model to avoid copying large data
    static PENDING_DETAILS_NODE: RefCell<Option<Weak<RefCell<ServerNode>>>> = const { RefCell::new(None) };
}

pub fn set_pending_details(node: Option<Weak<RefCell<ServerNode>>>) {
    PENDING_DETAILS_NODE.with(|c| {
        *c.borrow_mut() = node;
    });
}

#[allow(dead_code)]
pub fn take_pending_details() -> Option<Weak<RefCell<ServerNode>>> {
    PENDING_DETAILS_NODE.with(|c| c.borrow_mut().take())
}

/// Returns true if there is a pending selection stored.
/// Note: this only checks presence (Some/None) and does not verify that the Weak can be upgraded.
pub fn has_pending_details() -> bool {
    PENDING_DETAILS_NODE.with(|c| c.borrow().is_some())
}

/// Returns a clone of the pending selection without consuming it.
/// This lets callers access the current selection context while keeping
/// the stash populated for subsequent menu enable/disable logic.
pub fn get_pending_details() -> Option<Weak<RefCell<ServerNode>>> {
    PENDING_DETAILS_NODE.with(|c| c.borrow().as_ref().cloned())
}
