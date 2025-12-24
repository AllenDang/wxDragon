use wxdragon::dialogs::message_dialog::{MessageDialog, MessageDialogStyle};
use wxdragon::prelude::*;
use wxdragon_macros::include_xrc;

// Generate the XRC struct from the XRC file
include_xrc!("../ui/simplebook_frame.xrc", SimpleBookXrcUI);

pub fn run_xrc_test() {
    let _ = wxdragon::main(|_| {
        // Load the UI from XRC
        let ui = SimpleBookXrcUI::new(None, false);

        // Set up event handlers
        setup_xrc_event_handlers(&ui);

        // Initialize status
        ui.main_statusbar
            .set_status_text("SimpleBook XRC Demo - Use buttons to navigate", 0);

        // Show the frame
        ui.main_frame.show(true);
        ui.main_frame.centre();
    });
}

fn setup_xrc_event_handlers(ui: &SimpleBookXrcUI) {
    // Widgets are Copy, so use them directly
    let simplebook = ui.demo_simplebook;
    let frame = ui.main_frame;
    let statusbar = ui.main_statusbar;

    // Previous button handler
    ui.prev_button.on_click(move |_| {
        let current = simplebook.selection();
        if current > 0 {
            simplebook.set_selection((current - 1) as usize);
        }
    });

    // Next button handler
    ui.next_button.on_click(move |_| {
        let current = simplebook.selection();
        let page_count = simplebook.get_page_count() as i32;
        if current < page_count - 1 {
            simplebook.set_selection((current + 1) as usize);
        }
    });

    // Info button handler
    ui.info_button.on_click(move |_| {
        let current = simplebook.selection();
        let page_count = simplebook.get_page_count();
        let message = format!(
            "XRC SimpleBook Demo\n\nCurrent page: {} of {}\nTotal pages: {}\n\nThis SimpleBook was loaded from XRC!",
            current + 1,
            page_count,
            page_count
        );

        let dialog = MessageDialog::builder(&frame, &message, "SimpleBook XRC Info")
            .with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation)
            .build();
        dialog.show_modal();
    });

    // Listen for page change events
    ui.demo_simplebook.on_page_changed(move |event| {
        if let (Some(old_page), Some(new_page)) = (event.get_old_selection(), event.get_selection()) {
            statusbar.set_status_text(
                &format!("XRC Demo: Changed from page {} to page {}", old_page + 1, new_page + 1),
                0,
            );
        }
    });

    // Demo some interaction with the controls loaded from XRC
    ui.demo_button.on_click(move |_| {
        println!("XRC-loaded button clicked!");
    });

    // CheckBox events would need a different approach - using WindowEvents
    // For now, just demonstrating that the widget was loaded successfully
}
