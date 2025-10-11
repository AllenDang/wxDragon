use std::rc::Rc;
use wxdragon::prelude::*;
use wxdragon::DataViewCtrl;
use wxdragon::DataViewStyle;

mod mymodels;

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    // Keep the model alive for the duration of the app
    let model = Rc::new(mymodels::create_music_tree_model().expect("Failed to create tree model"));
    let model_clone = model.clone();
    let _ = wxdragon::main(move |_| {
        let frame = Frame::builder()
            .with_title("wxDragon DataView Example")
            .with_size(Size::new(900, 600))
            .build();

        let panel = Panel::builder(&frame).build();

        // Create the generic DataView control (works with custom models)
        let dataview = DataViewCtrl::builder(&panel)
            .with_size(Size::new(860, 520))
            .with_style(DataViewStyle::RowLines | DataViewStyle::VerticalRules)
            .build();

        // Create columns and text renderers
        let title_col = DataViewColumn::new(
            "Title",
            &DataViewTextRenderer::new(
                VariantType::String,
                DataViewCellMode::Inert,
                DataViewAlign::Left,
            ),
            0,
            250,
            DataViewAlign::Left,
            DataViewColumnFlags::Resizable,
        );

        let artist_col = DataViewColumn::new(
            "Artist",
            &DataViewTextRenderer::new(
                VariantType::String,
                DataViewCellMode::Inert,
                DataViewAlign::Left,
            ),
            1,
            200,
            DataViewAlign::Left,
            DataViewColumnFlags::Resizable,
        );

        let year_col = DataViewColumn::new(
            "Year",
            &DataViewTextRenderer::new(
                VariantType::String,
                DataViewCellMode::Inert,
                DataViewAlign::Center,
            ),
            2,
            100,
            DataViewAlign::Center,
            DataViewColumnFlags::Resizable,
        );

        let judg_col = DataViewColumn::new(
            "Judgement",
            &DataViewTextRenderer::new(
                VariantType::String,
                DataViewCellMode::Inert,
                DataViewAlign::Left,
            ),
            3,
            120,
            DataViewAlign::Left,
            DataViewColumnFlags::Resizable,
        );

        dataview.append_column(&title_col);
        dataview.append_column(&artist_col);
        dataview.append_column(&year_col);
        dataview.append_column(&judg_col);

        // Associate the model with the control
        dataview.associate_model(&*model_clone);

        // Expand the visible root and the first child ('Pop music') on startup.
        // The invalid/default DataViewItem represents the invisible root; its
        // first child is the visible root 'My Music'.
        let invalid = DataViewItem::default();
        // Get the visible root (first top-level item)
        let root_item = dataview.get_nth_child(&invalid, 0);
        if root_item.is_valid() {
            dataview.expand(&root_item);
            dataview.ensure_visible(&root_item);
            // Expand the first child of the visible root (Pop music)
            let first_child = dataview.get_nth_child(&root_item, 0);
            if first_child.is_valid() {
                dataview.expand(&first_child);
                dataview.ensure_visible(&first_child);
                // Select the second grandchild ("Yesterday")
                let grandchild = dataview.get_nth_child(&first_child, 1);
                if grandchild.is_valid() {
                    dataview.select(&grandchild);
                    dataview.ensure_visible(&grandchild);
                }
            }
        }

        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        sizer.add(&dataview, 1, SizerFlag::Expand | SizerFlag::All, 8);
        panel.set_sizer(sizer, true);

        frame.show(true);
        frame.centre();
    });
}
