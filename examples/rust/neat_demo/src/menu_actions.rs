use wxdragon::prelude::*;

use crate::selection_ctx;
use crate::{MenuId, about_dlg, details_dlg, model::ServerList, server_node, settings_dlg, show_qrcode_dlg};
use std::{cell::RefCell, rc::Rc};

/// Dispatch a menu command ID to the same logic used by Frame::on_menu.
/// This allows other UI elements (e.g., double-click on DataView) to reuse menu actions.
pub fn handle_menu_command(frame: &Frame, model: &CustomDataViewTreeModel, id: i32) {
    match id {
        x if x == i32::from(MenuId::Quit) => {
            log::info!("Menu/Toolbar: Quit clicked!");
            frame.close(true);
        }
        x if x == i32::from(MenuId::About) => {
            about_dlg::show_about_dialog(frame);
        }
        x if x == i32::from(MenuId::Settings) => {
            log::info!("Menu/Toolbar: Settings clicked!");
            settings_dlg::settings_dlg(frame);
        }
        x if x == i32::from(MenuId::ViewDetails) => {
            log::info!("Menu/Toolbar: View Details clicked!");
            // If a pending selection has been provided (e.g., by a double-click), use it to prefill
            if let Some(weak) = selection_ctx::get_pending_details() {
                if let Some(rc) = weak.upgrade() {
                    // Scope the immutable borrow just for prefill usage
                    let updated = {
                        let init_borrow = rc.borrow();
                        details_dlg::details_dlg(frame, Some(&*init_borrow))
                    };
                    if let Some(updated_node) = updated {
                        // Commit the changes back to the model
                        {
                            let mut m = rc.borrow_mut();
                            *m = updated_node;
                        }
                        // Notify the view that this item changed
                        let ptr: *const server_node::ServerNode = {
                            let b = rc.borrow();
                            &*b as *const _
                        };
                        model.item_changed::<server_node::ServerNode>(ptr);
                    }
                } else {
                    // Node no longer exists; open dialog without prefill (no commit target)
                    let _ = details_dlg::details_dlg(frame, None);
                }
            } else {
                // No pending selection; treat as read-only view (or nothing to edit)
                let _ = details_dlg::details_dlg(frame, None);
            }
        }
        x if x == i32::from(MenuId::New) => {
            log::info!("Menu/Toolbar: New clicked!");
            if let Some(node) = details_dlg::details_dlg(frame, None) {
                let added =
                    model.with_userdata_mut::<Rc<RefCell<ServerList>>, Option<*const server_node::ServerNode>>(|list_rc| {
                        let rc = Rc::new(RefCell::new(node));
                        let ptr: *const server_node::ServerNode = {
                            let b = rc.borrow();
                            &*b as *const _
                        };
                        list_rc.borrow_mut().nodes.push(rc);
                        Some(ptr)
                    });
                if let Some(Some(ptr)) = added {
                    model.item_added::<server_node::ServerNode>(None, ptr);
                }
            }
        }
        x if x == i32::from(MenuId::Delete) => {
            log::info!("Menu/Toolbar: Delete clicked!");
            if let Some(weak) = selection_ctx::get_pending_details() {
                if let Some(rc) = weak.upgrade() {
                    // Capture raw pointer for model notification before removal
                    let child_ptr: *const server_node::ServerNode = {
                        let b = rc.borrow();
                        &*b as *const _
                    };

                    // Remove from underlying data
                    let removed = model.with_userdata_mut::<Rc<RefCell<ServerList>>, bool>(|list_rc| {
                        let mut list = list_rc.borrow_mut();
                        if let Some(idx) = list.nodes.iter().position(|n| Rc::ptr_eq(n, &rc)) {
                            list.nodes.remove(idx);
                            true
                        } else {
                            false
                        }
                    });

                    if let Some(true) = removed {
                        // Notify view
                        model.item_deleted::<server_node::ServerNode>(None, child_ptr);
                        // Clear selection context as the item is gone
                        selection_ctx::set_pending_details(None);
                    } else {
                        log::warn!("Delete requested, but selected node was not found in model.");
                    }
                } else {
                    log::warn!("Delete requested, but the selected item no longer exists.");
                }
            } else {
                log::info!("No selection to delete.");
            }
        }
        x if x == i32::from(MenuId::ShowQrCode) => {
            log::info!("Menu/Toolbar: Show QR Code clicked!");
            show_qrcode_dlg::show_qrcode_dlg(frame);
        }
        _ => {
            log::warn!("Unhandled Menu ID: {id}");
        }
    }
}
