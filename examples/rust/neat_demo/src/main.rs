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
mod settings;
mod settings_dlg;
mod show_qrcode_dlg;

use settings::{MAIN_ICON, WindowConfig, create_bitmap_from_memory};
use wxdragon::prelude::*;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    let model = std::rc::Rc::new(dataview::create_data_model());
    let model_clone = model.clone();
    let _ = wxdragon::main(move |_| {
        let win_cfg = settings::load_settings();

        let frame = Frame::builder()
            .with_title(settings::APP_TITLE)
            .with_position(win_cfg.to_point())
            .with_size(win_cfg.to_size())
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
            .append_item(
                MenuId::Open.into(),
                "Open Application",
                "Open the main application window",
            )
            .append_separator()
            .append_item(
                MenuId::Settings.into(),
                "Settings",
                "Open application settings",
            )
            .append_item(MenuId::About.into(), "About", "About this application")
            .append_separator()
            .append_item(MenuId::Quit.into(), "Quit", "Quit the application")
            .build();
        let taskbar = TaskBarIcon::builder()
            .with_icon_type(TaskBarIconType::CustomStatusItem)
            .build();
        taskbar.set_popup_menu(&tray_icon_menu);
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
            .append_item(
                MenuId::Settings.into(),
                "Settings",
                "Open application settings",
            )
            .append_separator()
            .append_item(
                MenuId::ScanQrCode.into(),
                "Scan QR Code from screen",
                "Scan QR code from screen",
            )
            .append_item(
                MenuId::ImportNodeFile.into(),
                "Import Node File",
                "Import node file",
            )
            .append_item(MenuId::New.into(), "New", "Create new node")
            .append_separator()
            .append_item(MenuId::Run.into(), "Run", "Run node")
            .append_item(MenuId::Stop.into(), "Stop", "Stop node")
            .append_separator()
            .append_item(MenuId::Quit.into(), "Quit\tCtrl+Q", "Quit the application")
            .build();

        // Node menu
        let node_menu = Menu::builder()
            .append_item(
                MenuId::ViewDetails.into(),
                "View Details",
                "View node details",
            )
            .append_item(MenuId::ExportNode.into(), "Export Node", "Export node")
            .append_item(
                MenuId::ShowQrCode.into(),
                "Show QR Code",
                "Show QR code for node",
            )
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

        let frame_for_menu = frame.clone();
        frame.on_menu(move |event| match event.get_id() {
            id if id == i32::from(MenuId::Quit) => {
                log::info!("Menu/Toolbar: Quit clicked!");
                frame_for_menu.close(true);
            }
            id if id == i32::from(MenuId::About) => {
                about_dlg::show_about_dialog(&frame_for_menu);
            }
            id if id == i32::from(MenuId::Settings) => {
                log::info!("Menu/Toolbar: Settings clicked!");
                settings_dlg::settings_dlg(&frame_for_menu);
            }
            id if id == i32::from(MenuId::ViewDetails) => {
                log::info!("Menu/Toolbar: View Details clicked!");
                details_dlg::details_dlg(&frame_for_menu);
            }
            id if id == i32::from(MenuId::New) => {
                log::info!("Menu/Toolbar: New clicked!");
                details_dlg::details_dlg(&frame_for_menu);
            }
            id if id == i32::from(MenuId::ShowQrCode) => {
                log::info!("Menu/Toolbar: Show QR Code clicked!");
                show_qrcode_dlg::show_qrcode_dlg(&frame_for_menu);
            }
            _ => {
                log::warn!("Unhandled Menu/Tool ID: {}", event.get_id());
                event.skip(true);
            }
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
        frame.on_destroy(move |_data| {
            let pos = frame_for_destroy.get_position();
            let size = frame_for_destroy.get_size();
            let cfg = WindowConfig::new(pos, size);
            settings::save_settings(&cfg);

            // Clean up the TaskBarIcon, it's important to call destroy() to remove the icon from the system tray,
            // or we can't exit the application main loop.
            taskbar.destroy();

            // Clean up the tray icon menu to release rust closures attached to menu items
            tray_icon_menu.destroy_meun();
        });

        // --- Main Panel Layout ---
        let main_panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Integrate DataView module (top, expands)
        let dataview_panel = dataview::create_data_view_panel(&main_panel, &model_clone);
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
}
