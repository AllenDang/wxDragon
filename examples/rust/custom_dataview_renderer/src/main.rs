use wxdragon::*;

fn main() {
    let _ = wxdragon::main(|_| {
        // Create the main frame
        let frame = Frame::builder()
            .with_title("Custom DataView Renderer Example")
            .with_size(Size::new(800, 600))
            .build();

        // Create a panel for the frame
        let panel = Panel::builder(&frame).build();

        // Create a data view control
        let dataview = DataViewCtrl::builder(&panel)
            .with_size(Size::new(760, 500))
            .with_style(DataViewStyle::Multiple | DataViewStyle::RowLines)
            .build();

        // Create a simple model
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

        // Create reusable custom renderers using auto-generated IDs
        // Callbacks are now stored directly in the renderer and cleaned up automatically
        let progress_renderer = DataViewCustomRenderer::builder()
            .variant_type(VariantType::Int32)
            .mode(DataViewCellMode::Inert)
            .align(DataViewAlign::Center)
            .with_get_size(|_variant, _default_size| Size::new(100, 20))
            .with_render(|rect, ctx, _state, variant| {
                if let Some(progress) = variant.get_i32() {
                    // Draw progress bar background
                    ctx.set_brush(Colour::rgb(240, 240, 240), BrushStyle::Solid);
                    ctx.draw_rectangle(rect.x, rect.y, rect.width, rect.height);

                    // Draw progress bar fill
                    let fill_width = (rect.width as f32 * (progress as f32 / 100.0)) as i32;

                    // Color based on progress
                    let color = if progress >= 100 {
                        Colour::rgb(76, 175, 80) // Green for complete
                    } else if progress >= 50 {
                        Colour::rgb(255, 193, 7) // Yellow for in progress
                    } else {
                        Colour::rgb(244, 67, 54) // Red for low progress
                    };

                    ctx.set_brush(color, BrushStyle::Solid);
                    ctx.draw_rectangle(rect.x, rect.y, fill_width, rect.height);

                    // Draw progress text
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

        let status_renderer = DataViewCustomRenderer::builder()
            .variant_type(VariantType::String)
            .mode(DataViewCellMode::Inert)
            .align(DataViewAlign::Center)
            .with_get_size(|variant, default_size| {
                if let Some(status) = variant.get_string() {
                    let base_width = 120;
                    let extra_width = status.len().saturating_sub(8) as i32 * 8; // 8 pixels per extra char
                    Size::new(base_width + extra_width, default_size.height)
                } else {
                    default_size
                }
            })
            .with_render(|rect, ctx, _state, variant| {
                if let Some(status) = variant.get_string() {
                    // Choose colors based on status
                    let (bg_color, text_color) = match status.as_str() {
                        "Complete" => (Colour::rgb(200, 230, 201), Colour::rgb(27, 94, 32)),
                        "Almost Done" => (Colour::rgb(255, 236, 179), Colour::rgb(230, 81, 0)),
                        _ => (Colour::rgb(255, 205, 210), Colour::rgb(183, 28, 28)),
                    };

                    // Draw background
                    ctx.set_brush(bg_color, BrushStyle::Solid);
                    ctx.draw_rectangle(rect.x, rect.y, rect.width, rect.height);

                    // Draw text
                    ctx.set_text_foreground(text_color);
                    let (text_width, text_height) = ctx.get_text_extent(&status);
                    let text_x = rect.x + (rect.width - text_width) / 2;
                    let text_y = rect.y + (rect.height - text_height) / 2;
                    ctx.draw_text(&status, text_x, text_y);
                }
                true
            })
            .build();

        // Create columns with different renderers
        let name_column = DataViewColumn::new(
            "Name",
            &DataViewTextRenderer::new(VariantType::String, DataViewCellMode::Inert, DataViewAlign::Left),
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

        // Add columns to the control
        dataview.append_column(&name_column);
        dataview.append_column(&progress_column);
        dataview.append_column(&status_column);

        // Associate the model with the control
        dataview.associate_model(&model);

        // Create a vertical sizer for layout
        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        sizer.add(&dataview, 1, SizerFlag::Expand | SizerFlag::All, 10);
        panel.set_sizer(sizer, true);

        frame.show(true);

        // No need to keep renderers alive manually - they're now managed properly!
        // The custom renderers will live as long as the columns that reference them.
    });
}
