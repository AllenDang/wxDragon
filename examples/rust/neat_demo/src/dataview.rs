use crate::MenuId;
use crate::model::{NodeFields, ServerList, find_node_via_raw_ptr};
use crate::selection_ctx;
use crate::server_node::ServerNode;
use crate::settings::WIDGET_MARGIN;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use wxdragon::*;

pub fn create_data_view_panel(parent: &dyn WxWidget, model: &CustomDataViewTreeModel, frame: &Frame) -> Panel {
    // Copy frame since Frame is Copy (thin pointer wrapper)
    let frame = *frame;
    // Create a panel for the parent
    let panel = Panel::builder(parent).build();

    // Create a data view control
    let dataview = DataViewCtrl::builder(&panel)
        .with_size(Size::new(760, 500))
        .with_style(DataViewStyle::Multiple | DataViewStyle::RowLines | DataViewStyle::VerticalRules)
        .build();

    // Helper to create a sortable, resizable text column mapping to a model column index
    fn create_text_column(title: &str, model_col: NodeFields, width: i32, align: DataViewAlign) -> DataViewColumn {
        DataViewColumn::new(
            title,
            &DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, align),
            model_col.bits() as usize,
            width,
            align,
            DataViewColumnFlags::Resizable | DataViewColumnFlags::Sortable,
        )
    }

    use bitflags::Flags;
    use std::collections::HashMap;
    let name_map: HashMap<NodeFields, &'static str> = NodeFields::iter_defined_names().map(|(name, flag)| (flag, name)).collect();

    let align = DataViewAlign::Left;
    let align2 = DataViewAlign::Center;

    let remarks_col = create_text_column(name_map[&NodeFields::Remarks], NodeFields::Remarks, 200, align);
    let path_col = create_text_column(name_map[&NodeFields::TunnelPath], NodeFields::TunnelPath, 260, align);
    let host_col = create_text_column(name_map[&NodeFields::ServerHost], NodeFields::ServerHost, 160, align);
    let port_col = create_text_column(name_map[&NodeFields::ServerPort], NodeFields::ServerPort, 90, align2);
    let domain_col = create_text_column(name_map[&NodeFields::ServerDomain], NodeFields::ServerDomain, 160, align);

    dataview.append_column(&remarks_col);
    dataview.append_column(&host_col);
    dataview.append_column(&port_col);
    dataview.append_column(&domain_col);
    dataview.append_column(&path_col);
    dataview.associate_model(model);

    dataview.on_item_context_menu(move |event: DataViewEvent| {
        let point = event.get_position();
        log::info!("Right click at position: {:?}", point);
        let point = point.map(|p| dataview.client_to_screen(p));

        let endabled = selection_ctx::has_pending_details();

        // Context menu
        let mut dataview_menu = Menu::builder()
            .append_item(MenuId::ViewDetails.into(), "View details", "View node details")
            .append_item(MenuId::ExportNode.into(), "Export Node", "Export node")
            .append_item(MenuId::ShowQrCode.into(), "Show QR Code", "Show QR code for node")
            .append_separator()
            .append_item(MenuId::Delete.into(), "Delete", "Delete node")
            .append_separator()
            .append_item(MenuId::New.into(), "New", "Create new node")
            .build();

        dataview_menu.enable_item(MenuId::ViewDetails.into(), endabled);
        dataview_menu.enable_item(MenuId::ExportNode.into(), endabled);
        dataview_menu.enable_item(MenuId::ShowQrCode.into(), endabled);
        dataview_menu.enable_item(MenuId::Delete.into(), endabled);

        panel.popup_menu(&mut dataview_menu, point);
    });

    dataview.on_item_activated(move |event: DataViewEvent| {
        // FIXME: Remove this comment after verifying the get_row() works as intended
        let row = event.get_row();
        log::info!("Item activated for row: {row:?}");

        // Synchronously dispatch the standard ViewDetails menu command to the frame
        let _ = frame.process_menu_command(MenuId::ViewDetails.into());
    });

    let model_for_selection = model.clone();
    dataview.on_selection_changed(move |event: DataViewEvent| {
        let weak_opt = if let Some(item) = event.get_item()
            && let Some(needle_ptr) = item.get_id::<ServerNode>()
        {
            // Capture a weak reference to the Rc<RefCell<ServerNode>> in the model to avoid copying large data
            model_for_selection
                .with_userdata_mut::<Rc<RefCell<ServerList>>, Option<Weak<RefCell<ServerNode>>>>(|list_rc| {
                    find_node_via_raw_ptr(&*list_rc, needle_ptr).map(|rc| Rc::downgrade(&rc))
                })
                .flatten()
        } else {
            None
        };
        let name = weak_opt
            .as_ref()
            .and_then(|w| w.upgrade())
            .map(|rc| rc.borrow().remarks.clone().unwrap_or_else(|| "<unnamed>".to_string()));
        log::info!("Selection changed, selected item: {name:?}");
        // Stash the weak pointer (if any) so the real menu handler can prefill the dialog
        selection_ctx::set_pending_details(weak_opt);
    });

    // Layout
    let sizer = BoxSizer::builder(Orientation::Vertical).build();
    sizer.add(&dataview, 1, SizerFlag::Expand | SizerFlag::All, WIDGET_MARGIN);
    panel.set_sizer(sizer, true);

    panel
}
