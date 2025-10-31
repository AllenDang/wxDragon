use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("GridSizer Test")
            .with_size(Size::new(500, 400))
            .build();

        let panel = Panel::builder(&frame).build();

        // Create a main vertical sizer
        let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Add a title
        let title = StaticText::builder(&panel).with_label("GridSizer Demo").build();
        main_sizer.add(&title, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);

        // Create a 3x3 grid sizer with gaps
        let grid_sizer = GridSizer::builder(3, 3).with_vgap(10).with_hgap(10).build();

        // Add buttons to fill the grid
        for i in 1..=9 {
            let button = Button::builder(&panel).with_label(&format!("Button {i}")).build();
            grid_sizer.add(&button, 0, SizerFlag::Expand, 0);
        }

        main_sizer.add_sizer(&grid_sizer, 1, SizerFlag::Expand | SizerFlag::All, 20);

        // Add some info about the grid
        let info = StaticText::builder(&panel)
            .with_label(&format!(
                "Grid: {}x{}, VGap: {}, HGap: {}",
                grid_sizer.get_rows(),
                grid_sizer.get_cols(),
                grid_sizer.get_vgap(),
                grid_sizer.get_hgap()
            ))
            .build();
        main_sizer.add(&info, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);

        // Create another grid with different dimensions
        let grid_sizer2 = GridSizer::builder(2, 4).with_gap(Size::new(5, 8)).build();

        // Add some checkboxes to the second grid
        for i in 1..=8 {
            let checkbox = CheckBox::builder(&panel).with_label(&format!("Option {i}")).build();
            grid_sizer2.add(&checkbox, 0, SizerFlag::Expand, 0);
        }

        main_sizer.add_sizer(&grid_sizer2, 0, SizerFlag::Expand | SizerFlag::All, 20);

        // Add a separator and demonstrate FlexGridSizer with the new enum
        let flex_title = StaticText::builder(&panel)
            .with_label("FlexGridSizer with FlexGrowMode enum:")
            .build();
        main_sizer.add(&flex_title, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);

        let flex_sizer = FlexGridSizer::builder(2, 2).with_vgap(5).with_hgap(5).build();

        // Set grow mode using the new enum
        flex_sizer.set_non_flexible_grow_mode(FlexGrowMode::All);
        flex_sizer.add_growable_col(1, 1);

        // Add some text controls to the flex grid
        for i in 1..=4 {
            let text_ctrl = TextCtrl::builder(&panel).with_value(&format!("Text {i}")).build();
            flex_sizer.add(&text_ctrl, 0, SizerFlag::Expand, 0);
        }

        main_sizer.add_sizer(&flex_sizer, 0, SizerFlag::Expand | SizerFlag::All, 20);

        panel.set_sizer(main_sizer, true);
        frame.show(true);
        frame.centre();
    });
}
