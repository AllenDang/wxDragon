#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Represents a set of menu IDs.
#[derive(strum::EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MenuId {
    Settings = 1001,
    ScanQrCode = 1002,
    ImportNodeFile = 1003,
    New = 1004,
    Run = 1005,
    Stop = 1006,
    Open = 1007,
    Quit = 1008,
    ViewDetails = 3001,
    ExportNode = 3002,
    ShowQrCode = 3003,
    Delete = 3004,
    Copy = 3005,
    Paste = 3006,
    About = 4001,
}

impl From<MenuId> for i32 {
    fn from(id: MenuId) -> i32 {
        id as i32
    }
}

mod about_dlg;
mod dataview;
mod details_dlg;
mod logview;
mod menu_actions;
mod model;
mod selection_ctx;
mod server_node;
mod settings;
mod settings_dlg;
mod show_qrcode_dlg;

use model::{ServerList, create_server_tree_model};
use settings::{MAIN_ICON, WindowConfig, create_bitmap_from_memory};
use std::{cell::RefCell, rc::Rc};
use wxdragon::prelude::*;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    let cfg = Rc::new(RefCell::new(settings::load_settings()));
    let cfg_clone = cfg.clone();
    let _ = wxdragon::main(move |_| {
        // Build model once from settings.servers
        let mut nodes = cfg_clone.borrow().servers.clone();

        // Demo seed data: when servers key is missing (None), add two example nodes
        if nodes.is_none() {
            let seed1 = crate::server_node::ServerNode {
                remarks: Some("Sample Server 1".to_string()),
                tunnel_path: "/".to_string(),
                disable_tls: None,
                client_id: Some("client-001".to_string()),
                server_host: "example.com".to_string(),
                server_port: 443,
                server_domain: Some("example.com".to_string()),
                ca_file: None,
                dangerous_mode: None,
            };
            let seed2 = crate::server_node::ServerNode {
                remarks: Some("Local Dev".to_string()),
                tunnel_path: "/dev".to_string(),
                // Use Some(true) to indicate TLS disabled (plain) in config storage style
                disable_tls: Some(true),
                client_id: None,
                server_host: "127.0.0.1".to_string(),
                server_port: 8080,
                server_domain: None,
                ca_file: None,
                dangerous_mode: None,
            };
            nodes = Some(vec![seed1, seed2]);
        }
        let nodes = nodes
            .unwrap_or_default()
            .into_iter()
            .map(|n| Rc::new(RefCell::new(n)))
            .collect();
        let data = Rc::new(RefCell::new(ServerList { nodes }));
        let model = create_server_tree_model(data);

        let win_cfg = cfg_clone.borrow().window.as_ref().cloned().unwrap_or_default();

        let frame = Frame::builder()
            .with_title(settings::APP_TITLE)
            .with_position(win_cfg.get_point())
            .with_size(win_cfg.get_size())
            .build();

        let icon_bitmap = create_bitmap_from_memory(MAIN_ICON, None).unwrap();
        frame.set_icon(&icon_bitmap);

        // --- Status Bar Setup ---
        StatusBar::builder(&frame)
            .with_fields_count(3)
            .with_status_widths(vec![-1, 150, 100])
            .add_initial_text(0, "Ready")
            .add_initial_text(1, "Center Field")
            .add_initial_text(2, "Right Field")
            .build();

        // Create popup menu for taskbar icon
        let mut tray_icon_menu = Menu::builder()
            .append_item(MenuId::Open.into(), "Open Application", "Open the main application window")
            .append_separator()
            .append_item(MenuId::Settings.into(), "Settings", "Open application settings")
            .append_item(MenuId::About.into(), "About", "About this application")
            .append_separator()
            .append_item(MenuId::Quit.into(), "Quit", "Quit the application")
            .build();
        let taskbar = TaskBarIcon::builder()
            .with_icon_type(TaskBarIconType::CustomStatusItem)
            .build();
        taskbar.set_popup_menu(&mut tray_icon_menu);
        let frame_taskbar = frame.clone();
        taskbar.on_menu(move |event| {
            let menu_id = event.get_id();
            match menu_id {
                x if x == MenuId::Open as i32 => {
                    log::info!("📂 Open Application clicked!");
                    frame_taskbar.show(true);
                    frame_taskbar.iconize(false);
                    frame_taskbar.raise();
                }
                x if x == MenuId::Settings as i32 => {
                    log::info!("⚙️ Settings clicked!");
                    settings_dlg::settings_dlg(&frame_taskbar);
                }
                x if x == MenuId::About as i32 => {
                    log::info!("ℹ️ About clicked!");
                    about_dlg::show_about_dialog(&frame_taskbar);
                }
                x if x == MenuId::Quit as i32 => {
                    log::info!("🚪 Quit clicked!");
                    frame_taskbar.close(true);
                }
                _ => {
                    log::warn!("Unknown menu item clicked: {menu_id}");
                }
            }
        });

        let success = taskbar.set_icon(&icon_bitmap, "Neat server node manager");

        if success && taskbar.is_icon_installed() {
            log::info!("TaskBarIcon successfully installed in system tray.");
        } else {
            log::error!("Failed to set taskbar icon.");
        }

        // --- Menu Bar Setup ---
        // Main menu
        let main_menu = Menu::builder()
            .append_item(MenuId::Settings.into(), "Settings", "Open application settings")
            .append_separator()
            .append_item(
                MenuId::ScanQrCode.into(),
                "Scan QR Code from screen",
                "Scan QR code from screen",
            )
            .append_item(MenuId::ImportNodeFile.into(), "Import Node File", "Import node file")
            .append_item(MenuId::New.into(), "New", "Create new node")
            .append_separator()
            .append_item(MenuId::Run.into(), "Run", "Run node")
            .append_item(MenuId::Stop.into(), "Stop", "Stop node")
            .append_separator()
            .append_item(MenuId::Quit.into(), "Quit\tCtrl+Q", "Quit the application")
            .build();

        // Node menu
        let node_menu = Menu::builder()
            .append_item(MenuId::ViewDetails.into(), "View Details", "View node details")
            .append_item(MenuId::ExportNode.into(), "Export Node", "Export node")
            .append_item(MenuId::ShowQrCode.into(), "Show QR Code", "Show QR code for node")
            .append_separator()
            .append_item(MenuId::Delete.into(), "Delete", "Delete node")
            .append_separator()
            .append_item(MenuId::Copy.into(), "Copy\tCtrl+C", "Copy node")
            .append_item(MenuId::Paste.into(), "Paste\tCtrl+V", "Paste node")
            .build();

        // Help menu
        let help_menu = Menu::builder()
            .append_item(MenuId::About.into(), "About", "Show about dialog")
            .build();

        let menubar = MenuBar::builder()
            .append(main_menu, "Main")
            .append(node_menu, "Node")
            .append(help_menu, "Help")
            .build();
        frame.set_menu_bar(menubar);

        // Dynamically enable/disable Node menu items when the menu bar opens
        // Disable actions that require a selection if none is present
        let frame_for_menu_open = frame.clone();
        frame.on_menu_opened(move |event: wxdragon::MenuEventData| {
            // Only handle the menubar case here; popup menus use a different path
            if event.is_popup() {
                return;
            }
            if let Some(mbar) = frame_for_menu_open.get_menu_bar() {
                let has_sel = selection_ctx::has_pending_details();
                // Items that require a selection
                let gated = [
                    MenuId::ViewDetails,
                    MenuId::ExportNode,
                    MenuId::ShowQrCode,
                    MenuId::Delete,
                    MenuId::Copy,
                ];
                for id in gated {
                    // Enable only if there is a pending selection
                    let _ = mbar.enable_item(id.into(), has_sel);
                }
            }
        });

        let frame_for_menu = frame.clone();
        let model_for_menu = model.clone();
        frame.on_menu(move |event| {
            let id = event.get_id();
            menu_actions::handle_menu_command(&frame_for_menu, &model_for_menu, id);
        });

        let frame_clone = frame.clone();
        frame.on_close(move |evt| {
            if let wxdragon::WindowEventData::General(event) = &evt
                && event.can_veto()
            {
                // If the close event is the window's default behavior (not from the taskbar menu or main menu)
                // we veto the close and hide the window instead
                log::debug!("Close event vetoed, hiding window instead of closing.");
                event.veto();
                frame_clone.show(false);
            }
        });

        let frame_for_destroy = frame.clone();
        let model_for_destroy = model.clone();
        let cfg_for_destroy = cfg_clone.clone();
        frame.on_destroy(move |_data| {
            let pos = frame_for_destroy.get_position();
            let size = frame_for_destroy.get_size();
            let win = WindowConfig::new(pos, size);
            let mut cfg = cfg_for_destroy.borrow_mut();
            cfg.window = Some(win);
            // Persist current servers from the model back to settings
            if let Some(servers) =
                model_for_destroy.with_userdata_mut::<Rc<RefCell<ServerList>>, Vec<server_node::ServerNode>>(|list_rc| {
                    list_rc.borrow().nodes.iter().map(|rc| rc.borrow().clone()).collect()
                })
            {
                cfg.servers = Some(servers);
            }

            // Clean up the TaskBarIcon, it's important to call destroy() to remove the icon from the system tray,
            // or we can't exit the application main loop.
            taskbar.destroy();

            // Clean up the tray icon menu to release rust closures attached to menu items
            tray_icon_menu.destroy_menu();
        });

        // --- Main Panel Layout ---
        let main_panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Integrate DataView module (top, expands)
        let dataview_panel = dataview::create_data_view_panel(&main_panel, &model, &frame);
        sizer.add(
            &dataview_panel,
            1,
            SizerFlag::Expand | SizerFlag::All,
            settings::WIDGET_MARGIN,
        );

        // Integrate LogView module (bottom, fixed height)
        let logview_panel = logview::LogViewPanel::new(&main_panel);
        sizer.add(
            &logview_panel.panel,
            0,
            SizerFlag::Expand | SizerFlag::All,
            settings::WIDGET_MARGIN,
        );

        main_panel.set_sizer(sizer, true);

        frame.show(true);
    });

    // Save settings on exit
    settings::save_settings(&cfg.borrow_mut());
}
