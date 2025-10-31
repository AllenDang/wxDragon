use crate::settings::center_rect;
use wxdragon::prelude::*;

pub fn details_dlg(parent: &dyn WxWidget) {
    let (w, h) = (600, 400);
    let (x, y) = center_rect(parent, w, h);

    let dialog = Dialog::builder(parent, "Node details of 'ot-0'")
        .with_style(DialogStyle::DefaultDialogStyle | DialogStyle::ResizeBorder)
        .with_position(x, y)
        .with_size(w, h)
        .build();

    let left_width = 140;
    let right_width = w - left_width - 10;
    let panel = Panel::builder(&dialog).build();
    let label_size = Size::new(left_width, -1);
    let input_size = Size::new(right_width, -1);

    let remarks_label = StaticText::builder(&panel)
        .with_style(StaticTextStyle::AlignRight)
        .with_label("Remarks")
        .with_size(label_size)
        .build();
    let remarks_input = TextCtrl::builder(&panel).with_size(input_size).with_value("").build();

    let tunnel_label = StaticText::builder(&panel)
        .with_style(StaticTextStyle::AlignRight)
        .with_label("Tunnel Path")
        .with_size(label_size)
        .build();
    let tunnel_input = TextCtrl::builder(&panel).with_size(input_size).with_value("").build();

    let disable_tls_label = StaticText::builder(&panel)
        .with_style(StaticTextStyle::AlignRight)
        .with_label("")
        .with_size(label_size)
        .build();
    let disable_tls_checkbox = CheckBox::builder(&panel)
        .with_size(input_size)
        .with_label("Disable TLS")
        .build();

    let client_id_label = StaticText::builder(&panel)
        .with_style(StaticTextStyle::AlignRight)
        .with_label("Client ID")
        .with_size(label_size)
        .build();
    let client_id_input = TextCtrl::builder(&panel).with_size(input_size).build();

    let server_host_label = StaticText::builder(&panel)
        .with_style(StaticTextStyle::AlignRight)
        .with_label("Server Host")
        .with_size(label_size)
        .build();
    let server_host_input = TextCtrl::builder(&panel).with_size(input_size).with_value("").build();

    let server_port_label = StaticText::builder(&panel)
        .with_style(StaticTextStyle::AlignRight)
        .with_label("Server Port")
        .with_size(label_size)
        .build();
    let server_port_input = SpinCtrl::builder(&panel)
        .with_size(input_size)
        .with_initial_value(443)
        .with_min_value(1)
        .with_max_value(u16::MAX as i32)
        .build();

    let server_domain_label = StaticText::builder(&panel)
        .with_style(StaticTextStyle::AlignRight)
        .with_label("Server Domain")
        .with_size(label_size)
        .build();
    let server_domain_input = TextCtrl::builder(&panel).with_size(input_size).with_value("").build();

    let ca_file_label = StaticText::builder(&panel)
        .with_style(StaticTextStyle::AlignRight)
        .with_label("CA File/Content")
        .with_size(label_size)
        .build();
    let ca_file_input = TextCtrl::builder(&panel).with_size(input_size).with_value("").build();

    let dangerous_label = StaticText::builder(&panel)
        .with_style(StaticTextStyle::AlignRight)
        .with_label("")
        .with_size(label_size)
        .build();
    let dangerous_checkbox = CheckBox::builder(&panel)
        .with_size(input_size)
        .with_label("Dangerous Mode")
        .build();

    let grid = FlexGridSizer::builder(10, 2).with_vgap(8).with_hgap(10).build();
    grid.add(&remarks_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&remarks_input, 1, SizerFlag::Expand, 0);
    grid.add(&tunnel_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&tunnel_input, 1, SizerFlag::Expand, 0);
    grid.add(
        &disable_tls_label,
        0,
        SizerFlag::AlignRight | SizerFlag::AlignCenterVertical,
        0,
    );
    grid.add(
        &disable_tls_checkbox,
        1,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );
    grid.add(&client_id_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&client_id_input, 1, SizerFlag::Expand, 0);
    grid.add(
        &server_host_label,
        0,
        SizerFlag::AlignRight | SizerFlag::AlignCenterVertical,
        0,
    );
    grid.add(&server_host_input, 1, SizerFlag::Expand, 0);
    grid.add(
        &server_port_label,
        0,
        SizerFlag::AlignRight | SizerFlag::AlignCenterVertical,
        0,
    );
    grid.add(&server_port_input, 1, SizerFlag::Expand, 0);
    grid.add(
        &server_domain_label,
        0,
        SizerFlag::AlignRight | SizerFlag::AlignCenterVertical,
        0,
    );
    grid.add(&server_domain_input, 1, SizerFlag::Expand, 0);
    grid.add(&ca_file_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&ca_file_input, 1, SizerFlag::Expand, 0);
    grid.add(&dangerous_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(
        &dangerous_checkbox,
        1,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );

    let submit_btn = Button::builder(&panel).with_label("Submit").build();
    let cancel_btn = Button::builder(&panel).with_label("Cancel").with_id(ID_CANCEL).build();
    let dialog_clone = dialog.clone();
    submit_btn.on_click(move |_data| {
        dialog_clone.end_modal(ID_OK);
    });
    let dialog_clone2 = dialog.clone();
    cancel_btn.on_click(move |_data| {
        dialog_clone2.end_modal(ID_CANCEL);
    });

    let panel_sizer = BoxSizer::builder(Orientation::Vertical).build();
    panel_sizer.add_sizer(&grid, 1, SizerFlag::Expand | SizerFlag::All, 10);
    let btn_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    btn_sizer.add(&cancel_btn, 0, SizerFlag::AlignCentre | SizerFlag::All, 10);
    btn_sizer.add(&submit_btn, 0, SizerFlag::AlignCentre | SizerFlag::All, 10);
    panel_sizer.add_sizer(&btn_sizer, 0, SizerFlag::AlignCentre | SizerFlag::All, 0);
    panel.set_sizer(panel_sizer, true);

    let dialog_sizer = BoxSizer::builder(Orientation::Vertical).build();
    dialog_sizer.add(&panel, 1, SizerFlag::Expand, 0);
    dialog.set_sizer(dialog_sizer, true);

    let result = dialog.show_modal();
    log::info!("Details dialog returned: {}", result);
}
