use crate::MenuId;
use crate::settings::WIDGET_MARGIN;
use std::rc::Rc;
use wxdragon::*;

pub fn create_data_model() -> DataViewListModel {
    let model = DataViewListModel::new();
    model.append_column("Name");
    model.append_column("Progress");
    model.append_column("Status");

    // Create some data for testing
    let data = [
        ("Alice", 25, "In Progress"),
        ("Bob", 75, "Almost Done"),
        ("Carol", 100, "Complete"),
    ];

    for (row, (name, progress, status)) in data.iter().enumerate() {
        model.append_row();
        model.set_value(row, 0, Variant::from_string(name));
        model.set_value(row, 1, Variant::from_i32(*progress));
        model.set_value(row, 2, Variant::from_string(status));
    }

    model
}

pub fn create_data_view_panel(parent: &Window, model: &Rc<DataViewListModel>) -> Panel {
    // Create a panel for the parent
    let panel = Panel::builder(parent).build();

    // Create a data view control
    let dataview = DataViewCtrl::builder(&panel)
        .with_size(Size::new(760, 500))
        .with_style(DataViewStyle::Multiple | DataViewStyle::RowLines)
        .build();

    // Timer to update progress, simulate dynamic updates
    use std::cell::RefCell;
    use std::rc::Rc;
    let timer = Rc::new(Timer::new(&panel));
    timer.start(50, false); // 50ms, repeat
    let model_timer = Rc::downgrade(model);
    let progress = Rc::new(RefCell::new(0));
    let progress_clone = Rc::clone(&progress);
    let dataview_timer = dataview.clone();
    timer.on_tick(move |_event| {
        let mut p = progress_clone.borrow_mut();
        *p = (*p + 2) % 101;
        let model = match model_timer.upgrade() {
            Some(m) => m,
            None => return,
        };
        model.set_value(0, 1, Variant::from_i32(*p));
        dataview_timer.refresh(false, None);
        // log::trace!("Progress updated to {}%", *p);
    });
    panel.on_destroy(move |_data| {
        timer.stop();
    });

    // Progress renderer
    let progress_renderer = DataViewCustomRenderer::builder()
        .variant_type(VariantType::Int32)
        .mode(DataViewCellMode::Inert)
        .align(DataViewAlign::Center)
        .with_get_size(|_variant, _default_size| Size::new(100, 20))
        .with_render(|rect, ctx, _state, variant| {
            if let Some(progress) = variant.get_i32() {
                ctx.set_brush(Colour::rgb(240, 240, 240), BrushStyle::Solid);
                ctx.draw_rectangle(rect.x, rect.y, rect.width, rect.height);
                let fill_width = (rect.width as f32 * (progress as f32 / 100.0)) as i32;
                let color = if progress >= 100 {
                    Colour::rgb(76, 175, 80)
                } else if progress >= 50 {
                    Colour::rgb(255, 193, 7)
                } else {
                    Colour::rgb(244, 67, 54)
                };
                ctx.set_brush(color, BrushStyle::Solid);
                ctx.draw_rectangle(rect.x, rect.y, fill_width, rect.height);
                ctx.set_text_foreground(Colour::rgb(0, 0, 0));
                let text = format!("{progress}%");
                let (text_width, text_height) = ctx.get_text_extent(&text);
                let text_x = rect.x + (rect.width - text_width) / 2;
                let text_y = rect.y + (rect.height - text_height) / 2;
                ctx.draw_text(&text, text_x, text_y);
            }
            true
        })
        .build();

    // Status renderer
    let status_renderer = DataViewCustomRenderer::builder()
        .variant_type(VariantType::String)
        .mode(DataViewCellMode::Inert)
        .align(DataViewAlign::Center)
        .with_get_size(|variant, default_size| {
            if let Some(status) = variant.get_string() {
                let base_width = 120;
                let extra_width = status.len().saturating_sub(8) as i32 * 8;
                Size::new(base_width + extra_width, default_size.height)
            } else {
                default_size
            }
        })
        .with_render(|rect, ctx, _state, variant| {
            if let Some(status) = variant.get_string() {
                let (bg_color, text_color) = match status.as_str() {
                    "Complete" => (Colour::rgb(200, 230, 201), Colour::rgb(27, 94, 32)),
                    "Almost Done" => (Colour::rgb(255, 236, 179), Colour::rgb(230, 81, 0)),
                    _ => (Colour::rgb(255, 205, 210), Colour::rgb(183, 28, 28)),
                };
                ctx.set_brush(bg_color, BrushStyle::Solid);
                ctx.draw_rectangle(rect.x, rect.y, rect.width, rect.height);
                ctx.set_text_foreground(text_color);
                let (text_width, text_height) = ctx.get_text_extent(&status);
                let text_x = rect.x + (rect.width - text_width) / 2;
                let text_y = rect.y + (rect.height - text_height) / 2;
                ctx.draw_text(&status, text_x, text_y);
            }
            true
        })
        .build();

    // Columns
    let name_column = DataViewColumn::new(
        "Name",
        &DataViewTextRenderer::new(
            VariantType::String,
            DataViewCellMode::Inert,
            DataViewAlign::Left,
        ),
        0,
        100,
        DataViewAlign::Left,
        DataViewColumnFlags::Resizable,
    );
    let progress_column = DataViewColumn::new(
        "Progress",
        &progress_renderer,
        1,
        120,
        DataViewAlign::Center,
        DataViewColumnFlags::Resizable,
    );
    let status_column = DataViewColumn::new(
        "Status",
        &status_renderer,
        2,
        120,
        DataViewAlign::Center,
        DataViewColumnFlags::Resizable,
    );
    dataview.append_column(&name_column);
    dataview.append_column(&progress_column);
    dataview.append_column(&status_column);
    dataview.associate_model(model.as_ref());

    let dataview_menu_panel = panel.clone();
    dataview.on_item_context_menu(move |event: DataViewEventData| {
        let point = event.get_position();
        log::debug!("Right click at position: {:?}", point);

        let row = event.get_row();
        log::debug!("Context menu for row: {row:?}");

        // Context menu
        let dataview_menu = Menu::builder()
            .append_item(
                MenuId::ViewDetails.into(),
                "View details",
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
            .append_item(MenuId::New.into(), "New", "Create new node")
            .build();

        dataview_menu_panel.popup_menu(&dataview_menu, point);
    });

    // Layout
    let sizer = BoxSizer::builder(Orientation::Vertical).build();
    sizer.add(
        &dataview,
        1,
        SizerFlag::Expand | SizerFlag::All,
        WIDGET_MARGIN,
    );
    panel.set_sizer(sizer, true);

    panel
}
