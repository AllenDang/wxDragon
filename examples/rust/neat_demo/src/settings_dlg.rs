use crate::settings::{ICON_SIZE, MAIN_ICON, center_rect, create_bitmap_from_memory};
use wxdragon::prelude::*;

pub fn settings_dlg(frame_clone: &dyn WxWidget) {
    let (w, h) = (600, 400);
    let (x, y) = center_rect(frame_clone, w, h);

    // Create a generic dialog using the new builder
    let dialog = Dialog::builder(frame_clone, "Settings")
        .with_style(DialogStyle::DefaultDialogStyle | DialogStyle::ResizeBorder)
        .with_position(x, y)
        .with_size(w, h)
        .build();

    let icon_bitmap = create_bitmap_from_memory(MAIN_ICON, Some((ICON_SIZE, ICON_SIZE))).unwrap();
    dialog.set_icon(&icon_bitmap);

    // Add main panel to the dialog
    let panel = Panel::builder(&dialog).build();

    // Create Notebook for tabs
    let notebook = Notebook::builder(&panel).build();

    // Create tab pages using separate functions
    let common_panel = create_common_tab(&notebook);
    let tun2proxy_panel = create_tun2proxy_tab(&notebook);
    let httpproxy_panel = create_httpproxy_tab(&notebook);
    let logging_panel = create_logging_tab(&notebook);

    let image_list = ImageList::new(16, 16, true, 4);
    let info_icon = ArtProvider::get_bitmap(ArtId::Information, ArtClient::Menu, Some(Size::new(16, 16))).unwrap();
    image_list.add_bitmap(&info_icon);
    let question_icon = ArtProvider::get_bitmap(ArtId::Removable, ArtClient::Menu, Some(Size::new(16, 16))).unwrap();
    image_list.add_bitmap(&question_icon);
    let goup_icon = ArtProvider::get_bitmap(ArtId::GoUp, ArtClient::Menu, Some(Size::new(16, 16))).unwrap();
    image_list.add_bitmap(&goup_icon);
    let addbookmark_icon = ArtProvider::get_bitmap(ArtId::AddBookmark, ArtClient::Menu, Some(Size::new(16, 16))).unwrap();
    image_list.add_bitmap(&addbookmark_icon);

    notebook.set_image_list(image_list);

    // Add tabs to notebook
    notebook.add_page(&common_panel, "Common", true, Some(0));
    notebook.add_page(&tun2proxy_panel, "Tun2proxy", false, Some(1));
    notebook.add_page(&httpproxy_panel, "HttpProxy", false, Some(2));
    notebook.add_page(&logging_panel, "Logging", false, Some(3));

    // OK & Cancel buttons
    let ok_button = Button::builder(&panel).with_label("OK").build();
    let cancel_button = Button::builder(&panel).with_label("Cancel").with_id(ID_CANCEL).build();
    ok_button.on_click(move |_data| {
        dialog.end_modal(ID_OK);
    });
    cancel_button.on_click(move |_data| {
        dialog.end_modal(ID_CANCEL);
    });

    // Layout the panel content
    let panel_sizer = BoxSizer::builder(Orientation::Vertical).build();
    panel_sizer.add(&notebook, 1, SizerFlag::Expand | SizerFlag::All, 10);
    let btn_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    btn_sizer.add(&cancel_button, 0, SizerFlag::AlignCentre | SizerFlag::All, 10);
    btn_sizer.add(&ok_button, 0, SizerFlag::AlignCentre | SizerFlag::All, 10);
    panel_sizer.add_sizer(&btn_sizer, 0, SizerFlag::AlignCentre | SizerFlag::All, 0);
    panel.set_sizer(panel_sizer, true);

    // Layout the dialog
    let dialog_sizer = BoxSizer::builder(Orientation::Vertical).build();
    dialog_sizer.add(&panel, 1, SizerFlag::Expand, 0);
    dialog.set_sizer(dialog_sizer, true);

    // Show the dialog modally
    let result = dialog.show_modal();
    log::info!("Dialog returned: {}", result);

    // Dialog is automatically cleaned up when it goes out of scope
}

fn create_common_tab(parent: &dyn WxWidget) -> Panel {
    let panel = Panel::builder(parent).build();

    // Label size for alignment
    let label_size = Size::new(150, -1);

    // Listen Host
    let host_label = StaticText::builder(&panel)
        .with_label("Listen Host:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let host_input = TextCtrl::builder(&panel).with_size(Size::new(200, -1)).build();

    // Listen Port
    let port_label = StaticText::builder(&panel)
        .with_style(StaticTextStyle::AlignRight)
        .with_label("Listen Port:")
        .with_size(label_size)
        .build();
    let port_input = SpinCtrl::builder(&panel)
        .with_initial_value(5080)
        .with_min_value(1)
        .with_max_value(u16::MAX as i32)
        .build();

    // Listen User
    let user_label = StaticText::builder(&panel)
        .with_style(StaticTextStyle::AlignRight)
        .with_label("Listen User:")
        .with_size(label_size)
        .build();
    let user_input = TextCtrl::builder(&panel).with_size(Size::new(200, -1)).build();

    // Listen Password
    let password_label = StaticText::builder(&panel)
        .with_label("Listen Password:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let password_input = TextCtrl::builder(&panel)
        .with_size(Size::new(200, -1))
        .with_style(TextCtrlStyle::Password)
        .build();

    // Connection Pool Max Size
    let pool_label = StaticText::builder(&panel)
        .with_label("Connection Pool Max Size:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let pool_input = SpinCtrl::builder(&panel)
        .with_initial_value(200)
        .with_min_value(10)
        .with_max_value(10000)
        .with_size(Size::new(100, -1))
        .build();

    // Cache DNS Label + CheckBox
    let cache_dns_label = StaticText::builder(&panel)
        .with_label("    ")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let cache_dns_checkbox = CheckBox::builder(&panel).with_value(false).with_label("Cache DNS").build();

    // Using FlexGridSizer for proper left-right alignment
    let grid = FlexGridSizer::builder(7, 2).with_vgap(10).with_hgap(16).build();
    grid.add(&host_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&host_input, 0, SizerFlag::Expand, 0);
    grid.add(&port_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&port_input, 0, SizerFlag::Expand, 0);
    grid.add(&user_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&user_input, 0, SizerFlag::Expand, 0);
    grid.add(&password_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&password_input, 0, SizerFlag::Expand, 0);
    grid.add(&pool_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&pool_input, 0, SizerFlag::Expand, 0);
    grid.add(&cache_dns_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(
        &cache_dns_checkbox,
        0,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );

    let sizer = BoxSizer::builder(Orientation::Vertical).build();
    sizer.add_sizer(&grid, 0, SizerFlag::Expand | SizerFlag::All, 16);
    panel.set_sizer(sizer, true);
    panel
}

fn create_tun2proxy_tab(parent: &dyn WxWidget) -> Panel {
    let panel = Panel::builder(parent).build();

    let label_size = Size::new(150, -1);

    // Enable Tun2proxy
    let enable_label = StaticText::builder(&panel)
        .with_label("    ")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let enable_checkbox = CheckBox::builder(&panel).with_label("Enable Tun2proxy").build();

    // Exit on Fatal Error
    let exit_label = StaticText::builder(&panel)
        .with_label("   ")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let exit_checkbox = CheckBox::builder(&panel)
        .with_value(true)
        .with_label("Exit on Fatal Error")
        .build();

    // Max Sessions
    let max_sessions_label = StaticText::builder(&panel)
        .with_label("Max Sessions:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let max_sessions_input = SpinCtrl::builder(&panel)
        .with_initial_value(200)
        .with_min_value(1)
        .with_max_value(10000)
        .build();

    // Remote DNS Address
    let dns_addr_label = StaticText::builder(&panel)
        .with_label("Remote DNS Address:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let dns_addr_input = TextCtrl::builder(&panel)
        .with_size(Size::new(200, -1))
        .with_value("8.8.8.8")
        .build();

    // DNS Strategy (dropdown)
    let dns_strategy_label = StaticText::builder(&panel)
        .with_label("DNS Strategy:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let dns_strategy_choices = vec!["virtual", "over-tcp", "direct"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();
    let dns_strategy_choice = Choice::builder(&panel)
        .with_choices(dns_strategy_choices)
        .with_selection(Some(1))
        .with_size(Size::new(200, -1))
        .build();

    // 使用 FlexGridSizer 排列
    let grid = FlexGridSizer::builder(5, 2).with_vgap(10).with_hgap(16).build();
    grid.add(&enable_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&enable_checkbox, 0, SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical, 0);
    grid.add(&exit_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&exit_checkbox, 0, SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical, 0);
    grid.add(
        &max_sessions_label,
        0,
        SizerFlag::AlignRight | SizerFlag::AlignCenterVertical,
        0,
    );
    grid.add(&max_sessions_input, 0, SizerFlag::Expand, 0);
    grid.add(&dns_addr_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&dns_addr_input, 0, SizerFlag::Expand, 0);
    grid.add(
        &dns_strategy_label,
        0,
        SizerFlag::AlignRight | SizerFlag::AlignCenterVertical,
        0,
    );
    grid.add(&dns_strategy_choice, 0, SizerFlag::Expand, 0);

    let sizer = BoxSizer::builder(Orientation::Vertical).build();
    sizer.add_sizer(&grid, 0, SizerFlag::Expand | SizerFlag::All, 16);
    panel.set_sizer(sizer, true);
    panel
}

fn create_httpproxy_tab(parent: &dyn WxWidget) -> Panel {
    let panel = Panel::builder(parent).build();

    let label_size = Size::new(170, -1);

    // Enable HttpProxy
    let enable_label = StaticText::builder(&panel).with_label("").with_size(label_size).build();
    let enable_checkbox = CheckBox::builder(&panel).with_label("Enable HttpProxy").build();

    // Source Type (dropdown)
    let source_type_label = StaticText::builder(&panel)
        .with_label("Source Type:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let source_type_choices = vec!["http", "socks5"].into_iter().map(String::from).collect::<Vec<String>>();
    let source_type_choice = Choice::builder(&panel)
        .with_choices(source_type_choices)
        .with_selection(Some(0))
        .with_size(Size::new(120, -1))
        .build();
    source_type_choice.enable(false);

    // Local Addr
    let local_addr_label = StaticText::builder(&panel)
        .with_label("Local Addr:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let local_addr_input = TextCtrl::builder(&panel).with_size(Size::new(200, -1)).build();

    // Server Addr
    let server_addr_label = StaticText::builder(&panel)
        .with_label("Server Addr:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let server_addr_input = TextCtrl::builder(&panel).with_size(Size::new(200, -1)).build();

    // Username
    let username_label = StaticText::builder(&panel)
        .with_label("Username:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let username_input = TextCtrl::builder(&panel).with_size(Size::new(200, -1)).build();

    // Password
    let password_label = StaticText::builder(&panel)
        .with_label("Password:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let password_input = TextCtrl::builder(&panel)
        .with_size(Size::new(200, -1))
        .with_style(TextCtrlStyle::Password)
        .build();

    let grid = FlexGridSizer::builder(6, 2).with_vgap(10).with_hgap(16).build();
    grid.add(&enable_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&enable_checkbox, 0, SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical, 0);
    grid.add(
        &source_type_label,
        0,
        SizerFlag::AlignRight | SizerFlag::AlignCenterVertical,
        0,
    );
    grid.add(&source_type_choice, 0, SizerFlag::Expand, 0);
    grid.add(
        &local_addr_label,
        0,
        SizerFlag::AlignRight | SizerFlag::AlignCenterVertical,
        0,
    );
    grid.add(&local_addr_input, 0, SizerFlag::Expand, 0);
    grid.add(
        &server_addr_label,
        0,
        SizerFlag::AlignRight | SizerFlag::AlignCenterVertical,
        0,
    );
    grid.add(&server_addr_input, 0, SizerFlag::Expand, 0);
    grid.add(&username_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&username_input, 0, SizerFlag::Expand, 0);
    grid.add(&password_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&password_input, 0, SizerFlag::Expand, 0);

    let sizer = BoxSizer::builder(Orientation::Vertical).build();
    sizer.add_sizer(&grid, 0, SizerFlag::Expand | SizerFlag::All, 16);
    panel.set_sizer(sizer, true);
    panel
}

fn create_logging_tab(parent: &dyn WxWidget) -> Panel {
    let panel = Panel::builder(parent).build();

    let label_size = Size::new(180, -1);
    let choice_size = Size::new(200, -1);
    let log_levels = vec!["Off", "Error", "Warn", "Info", "Debug", "Trace"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

    let global_label = StaticText::builder(&panel)
        .with_label("Global Log Level:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let global_choice = Choice::builder(&panel)
        .with_choices(log_levels.clone())
        .with_selection(Some(0))
        .with_size(choice_size)
        .build();

    let rustls_label = StaticText::builder(&panel)
        .with_label("Rustls Log Level:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let rustls_choice = Choice::builder(&panel)
        .with_choices(log_levels.clone())
        .with_selection(Some(0))
        .with_size(choice_size)
        .build();

    let tokio_label = StaticText::builder(&panel)
        .with_label("Tokio_tungstenite Log Level:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let tokio_choice = Choice::builder(&panel)
        .with_choices(log_levels.clone())
        .with_selection(Some(0))
        .with_size(choice_size)
        .build();

    let tungstenite_label = StaticText::builder(&panel)
        .with_label("Tungstenite Log Level:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let tungstenite_choice = Choice::builder(&panel)
        .with_choices(log_levels.clone())
        .with_selection(Some(0))
        .with_size(choice_size)
        .build();

    let ipstack_label = StaticText::builder(&panel)
        .with_label("Ipstack Log Level:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let ipstack_choice = Choice::builder(&panel)
        .with_choices(log_levels.clone())
        .with_selection(Some(0))
        .with_size(choice_size)
        .build();

    let neat_label = StaticText::builder(&panel)
        .with_label("Neat demo Log Level:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let neat_choice = Choice::builder(&panel)
        .with_choices(log_levels.clone())
        .with_selection(Some(0))
        .with_size(choice_size)
        .build();

    let tun2proxy_label = StaticText::builder(&panel)
        .with_label("Tun2proxy Log Level:")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let tun2proxy_choice = Choice::builder(&panel)
        .with_choices(log_levels.clone())
        .with_selection(Some(0))
        .with_size(choice_size)
        .build();

    // Log Auto Scroll
    let auto_scroll_label = StaticText::builder(&panel)
        .with_label("    ")
        .with_style(StaticTextStyle::AlignRight)
        .with_size(label_size)
        .build();
    let auto_scroll_checkbox = CheckBox::builder(&panel)
        .with_value(false)
        .with_label("Log Auto Scroll")
        .build();

    let grid = FlexGridSizer::builder(8, 2).with_vgap(10).with_hgap(16).build();
    grid.add(&global_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&global_choice, 0, SizerFlag::Expand, 0);
    grid.add(&rustls_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&rustls_choice, 0, SizerFlag::Expand, 0);
    grid.add(&tokio_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&tokio_choice, 0, SizerFlag::Expand, 0);
    grid.add(
        &tungstenite_label,
        0,
        SizerFlag::AlignRight | SizerFlag::AlignCenterVertical,
        0,
    );
    grid.add(&tungstenite_choice, 0, SizerFlag::Expand, 0);
    grid.add(&ipstack_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&ipstack_choice, 0, SizerFlag::Expand, 0);
    grid.add(&neat_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&neat_choice, 0, SizerFlag::Expand, 0);
    grid.add(&tun2proxy_label, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
    grid.add(&tun2proxy_choice, 0, SizerFlag::Expand, 0);
    grid.add(
        &auto_scroll_label,
        0,
        SizerFlag::AlignRight | SizerFlag::AlignCenterVertical,
        0,
    );
    grid.add(
        &auto_scroll_checkbox,
        0,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical,
        0,
    );

    let sizer = BoxSizer::builder(Orientation::Vertical).build();
    sizer.add_sizer(&grid, 0, SizerFlag::Expand | SizerFlag::All, 16);
    panel.set_sizer(sizer, true);
    panel
}
