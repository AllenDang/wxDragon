use wxdragon::prelude::*;

/// AUI tab controls for the gallery example
pub struct AuiTabControls {
    pub panel: Panel,
    toolbar: AuiToolBar,
}

impl AuiTabControls {
    pub fn bind_events(&self) {
        // Use a simple approach for the demo that doesn't try to access self inside the closure
        let toolbar_ref = self.toolbar.clone();

        // Use on_menu directly since we've added it to AuiToolBar via the event handlers macro
        toolbar_ref.on_menu(move |event| {
            let id = event.get_id();
            if id == 1001 {
                log::info!("Save Perspective button clicked");
                log::debug!("(In a real app, we would save the perspective string)");
            } else if id == 1002 {
                log::info!("Load Perspective button clicked");
                log::debug!("(In a real app, we would load the saved perspective)");
            }
        });
    }
}

/// Creates the AUI tab for the gallery.
pub fn create_aui_tab(parent: &Notebook) -> AuiTabControls {
    // Create the panel that will be the container for our demo
    let panel = Panel::builder(parent)
        .with_id(ID_ANY as i32)
        .with_size(Size::new(800, 600)) // Make sure the panel is large enough
        .build();

    // Create a proper AuiToolBar with appropriate styling
    let toolbar = AuiToolBar::builder(&panel)
        .with_id(ID_ANY as i32)
        .with_style(
            AuiToolBarStyle::Text |  // Show text labels
            AuiToolBarStyle::Gripper | // Show gripper for dragging
            AuiToolBarStyle::HorzLayout | // Use horizontal layout
            AuiToolBarStyle::Overflow, // Allow overflow for small windows
        )
        .with_size(Size::new(800, 40))
        .build();

    // Add tools with IDs so we can identify them in the event handler
    toolbar.add_tool(1001, "Save Perspective", "Save the current layout", WidgetItemKind::Normal);
    toolbar.add_separator();
    toolbar.add_tool(1002, "Load Perspective", "Restore a saved layout", WidgetItemKind::Normal);
    toolbar.add_stretch_spacer(1); // Add stretch space to push tools to the left

    // Realize the toolbar to finalize its layout
    toolbar.realize();

    // Create some text controls with plenty of text to make them visible
    let text1 = TextCtrl::builder(&panel)
        .with_id(ID_ANY as i32)
        .with_size(Size::new(200, 200))
        .with_value("Text Control 1 - Left Pane\nThis pane can be moved by dragging its caption.\nYou can also float it by dragging it away from the window edges.")
        .build();

    let text2 = TextCtrl::builder(&panel)
        .with_id(ID_ANY as i32)
        .with_size(Size::new(200, 200))
        .with_value("Text Control 2 - Bottom Pane\nThis pane can be moved by dragging its caption.\nTry dragging this pane to other edges of the window.")
        .build();

    let text3 = TextCtrl::builder(&panel)
        .with_id(ID_ANY as i32)
        .with_size(Size::new(200, 200))
        .with_value(
            "Text Control 3 - Center Pane\nThis is the main content area.\nOther panes will dock around this central pane.",
        )
        .build();

    // Create the AuiManager using the builder pattern to ensure proper lifecycle management
    let mgr = AuiManager::builder(&panel).build();

    // Add the panes to the manager with different directions
    // Add the toolbar as a proper toolbar
    mgr.add_pane_with_info(
        &toolbar,
        AuiPaneInfo::new()
            .with_name("toolbar")
            .with_caption("Toolbar")
            .caption_visible(true)
            .top()
            .layer(0) // Layer 0 is topmost
            .row(0) // First row
            .position(0) // First position in row
            .pane_border(true)
            .gripper(false)
            .gripper_top(false)
            .resizable(false) // Make it fixed height
            .dockable(true)
            .movable(true)
            .min_size(200, 40)
            .best_size(800, 40)
            .toolbar_pane(), // Use toolbar_pane() since this is actually a toolbar
    );

    mgr.add_pane_with_info(
        &text1,
        AuiPaneInfo::new()
            .with_name("text1")
            .with_caption("Left Pane")
            .caption_visible(true)
            .left()
            .layer(1)
            .position(1)
            .pane_border(true)
            .gripper(false)
            .floatable(true)
            .dockable(true)
            .movable(true)
            .min_size(200, 200)
            .best_size(300, 300)
            .close_button(true)
            .maximize_button(true),
    );

    mgr.add_pane_with_info(
        &text2,
        AuiPaneInfo::new()
            .with_name("text2")
            .with_caption("Bottom Pane")
            .caption_visible(true)
            .bottom()
            .layer(1)
            .position(1)
            .pane_border(true)
            .gripper(false)
            .floatable(true)
            .dockable(true)
            .movable(true)
            .min_size(200, 200)
            .best_size(300, 300)
            .close_button(true)
            .maximize_button(true),
    );

    mgr.add_pane_with_info(
        &text3,
        AuiPaneInfo::new()
            .with_name("text3")
            .with_caption("Center Pane")
            .caption_visible(true)
            .center_pane()
            .pane_border(true)
            .floatable(false) // Center pane shouldn't be floatable
            .dockable(true)
            .movable(false) // Center pane shouldn't be movable
            .min_size(200, 200)
            .best_size(300, 300)
            .close_button(false) // Center pane shouldn't be closable
            .maximize_button(true),
    );

    // Commit all changes
    mgr.update();

    // Create and return the controls structure
    AuiTabControls { panel, toolbar }
}
