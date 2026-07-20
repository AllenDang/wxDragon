use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("wxPropertyGrid Demo")
            .with_size(Size::new(640, 480))
            .build();
        let panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        let grid = PropertyGrid::builder(&panel)
            .with_style(PropertyGridStyle::BoldModified | PropertyGridStyle::Tooltips)
            .build();

        let appearance = grid
            .append(Property::category("Appearance", "appearance"))
            .expect("failed to append Appearance category");
        grid.append(Property::string("Title", "title", "wxDragon").under(&appearance));
        grid.append(Property::boolean("Visible", "visible", true).under(&appearance));
        grid.append(Property::enumeration("Theme", "theme", [("System", 0), ("Light", 1), ("Dark", 2)], 0).under(&appearance));

        let geometry = grid
            .append(Property::category("Geometry", "geometry"))
            .expect("failed to append Geometry category");
        grid.append(Property::int("Width", "width", 640).under(&geometry));
        grid.append(Property::int("Height", "height", 480).under(&geometry));
        grid.append(Property::uint("Object ID", "object_id", 5_000_000_000).under(&geometry));
        grid.append(Property::float("Opacity", "opacity", 1.0).under(&geometry));

        grid.set_help_string("width", "Width must be at least 100 pixels.");

        grid.on_changing(|event| {
            if event.property_name().as_deref() == Some("width")
                && event
                    .value()
                    .and_then(|value| value.get_i64())
                    .is_some_and(|value| value < 100)
            {
                event.veto(true);
            }
        });

        grid.on_changed(|event| {
            if let Some(name) = event.property_name() {
                println!("{name} changed to {:?}", event.value());
            }
        });

        sizer.add(&grid, 1, SizerFlag::Expand | SizerFlag::All, 8);
        panel.set_sizer(sizer, true);

        frame.show(true);
        frame.centre();
    });
}
