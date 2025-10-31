use wxdragon::prelude::*;

const SWATCH_SIZE: Size = Size { width: 35, height: 20 };
const COLORS_PER_ROW: i32 = 5;

#[allow(dead_code)]
pub struct ColorTabControls {
    pub panel: Panel,
    swatches: Vec<StaticText>,
}

pub fn create_color_tab(notebook: &Notebook, _frame: &Frame) -> ColorTabControls {
    let color_panel = Panel::builder(notebook).with_style(PanelStyle::TabTraversal).build();

    // Create a scrolled window for the colors
    let scrolled_win = ScrolledWindow::builder(&color_panel)
        .with_style(ScrolledWindowStyle::VScroll)
        .build();

    let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

    // Title text
    let title = StaticText::builder(&color_panel)
        .with_label("Tailwind CSS Color Palette")
        .build();

    // Style the title text with a larger, bold font
    if let Some(mut font) = title.get_font() {
        font.set_weight(FontWeight::Bold);
        font.set_point_size(font.get_point_size() + 2);
        title.set_font(&font);
    }

    main_sizer.add(&title, 0, SizerFlag::Expand | SizerFlag::All, 5);

    // Add the scrolled window to the main sizer
    main_sizer.add(&scrolled_win, 1, SizerFlag::Expand | SizerFlag::All, 0);

    // Create a grid sizer for the color swatches
    let color_sizer = FlexGridSizer::builder(0, COLORS_PER_ROW).build();

    let mut swatches = Vec::new();

    // Define all color families with their colors
    let families = [
        (
            "Gray",
            &[
                (colours::gray::GRAY_50, "50"),
                (colours::gray::GRAY_100, "100"),
                (colours::gray::GRAY_200, "200"),
                (colours::gray::GRAY_300, "300"),
                (colours::gray::GRAY_400, "400"),
                (colours::gray::GRAY_500, "500"),
                (colours::gray::GRAY_600, "600"),
                (colours::gray::GRAY_700, "700"),
                (colours::gray::GRAY_800, "800"),
                (colours::gray::GRAY_900, "900"),
            ],
        ),
        (
            "Red",
            &[
                (colours::red::RED_50, "50"),
                (colours::red::RED_100, "100"),
                (colours::red::RED_200, "200"),
                (colours::red::RED_300, "300"),
                (colours::red::RED_400, "400"),
                (colours::red::RED_500, "500"),
                (colours::red::RED_600, "600"),
                (colours::red::RED_700, "700"),
                (colours::red::RED_800, "800"),
                (colours::red::RED_900, "900"),
            ],
        ),
        (
            "Blue",
            &[
                (colours::blue::BLUE_50, "50"),
                (colours::blue::BLUE_100, "100"),
                (colours::blue::BLUE_200, "200"),
                (colours::blue::BLUE_300, "300"),
                (colours::blue::BLUE_400, "400"),
                (colours::blue::BLUE_500, "500"),
                (colours::blue::BLUE_600, "600"),
                (colours::blue::BLUE_700, "700"),
                (colours::blue::BLUE_800, "800"),
                (colours::blue::BLUE_900, "900"),
            ],
        ),
        (
            "Green",
            &[
                (colours::green::GREEN_50, "50"),
                (colours::green::GREEN_100, "100"),
                (colours::green::GREEN_200, "200"),
                (colours::green::GREEN_300, "300"),
                (colours::green::GREEN_400, "400"),
                (colours::green::GREEN_500, "500"),
                (colours::green::GREEN_600, "600"),
                (colours::green::GREEN_700, "700"),
                (colours::green::GREEN_800, "800"),
                (colours::green::GREEN_900, "900"),
            ],
        ),
        (
            "Purple",
            &[
                (colours::purple::PURPLE_50, "50"),
                (colours::purple::PURPLE_100, "100"),
                (colours::purple::PURPLE_200, "200"),
                (colours::purple::PURPLE_300, "300"),
                (colours::purple::PURPLE_400, "400"),
                (colours::purple::PURPLE_500, "500"),
                (colours::purple::PURPLE_600, "600"),
                (colours::purple::PURPLE_700, "700"),
                (colours::purple::PURPLE_800, "800"),
                (colours::purple::PURPLE_900, "900"),
            ],
        ),
        (
            "Amber",
            &[
                (colours::amber::AMBER_50, "50"),
                (colours::amber::AMBER_100, "100"),
                (colours::amber::AMBER_200, "200"),
                (colours::amber::AMBER_300, "300"),
                (colours::amber::AMBER_400, "400"),
                (colours::amber::AMBER_500, "500"),
                (colours::amber::AMBER_600, "600"),
                (colours::amber::AMBER_700, "700"),
                (colours::amber::AMBER_800, "800"),
                (colours::amber::AMBER_900, "900"),
            ],
        ),
        (
            "Teal",
            &[
                (colours::teal::TEAL_50, "50"),
                (colours::teal::TEAL_100, "100"),
                (colours::teal::TEAL_200, "200"),
                (colours::teal::TEAL_300, "300"),
                (colours::teal::TEAL_400, "400"),
                (colours::teal::TEAL_500, "500"),
                (colours::teal::TEAL_600, "600"),
                (colours::teal::TEAL_700, "700"),
                (colours::teal::TEAL_800, "800"),
                (colours::teal::TEAL_900, "900"),
            ],
        ),
        (
            "Pink",
            &[
                (colours::pink::PINK_50, "50"),
                (colours::pink::PINK_100, "100"),
                (colours::pink::PINK_200, "200"),
                (colours::pink::PINK_300, "300"),
                (colours::pink::PINK_400, "400"),
                (colours::pink::PINK_500, "500"),
                (colours::pink::PINK_600, "600"),
                (colours::pink::PINK_700, "700"),
                (colours::pink::PINK_800, "800"),
                (colours::pink::PINK_900, "900"),
            ],
        ),
    ];

    // For each family
    for (family_name, colors) in families.iter() {
        // Add a family separator
        let family_sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Family header
        let header = StaticText::builder(&scrolled_win).with_label(family_name).build();

        // Style the family header with bold font
        if let Some(mut font) = header.get_font() {
            font.make_bold();
            header.set_font(&font);
        }

        family_sizer.add(&header, 0, SizerFlag::All, 2);

        // Create a grid for this color family
        let family_grid = FlexGridSizer::builder(2, 5).build();

        // Add each color in the family
        for (color, shade) in colors.iter() {
            // Create a swatch panel for the color
            let swatch_panel = Panel::builder(&scrolled_win).with_size(SWATCH_SIZE).build();
            swatch_panel.set_background_color(*color);

            // Add a tooltip with the color details
            let rgb = format!("RGB: {}, {}, {}", color.r, color.g, color.b);
            swatch_panel.set_tooltip(&format!("{}-{}\n{}", family_name.to_lowercase(), shade, rgb));

            // Add to the family grid
            family_grid.add(&swatch_panel, 0, SizerFlag::Expand | SizerFlag::All, 1);

            // Add to our collection
            swatches.push(StaticText::builder(&swatch_panel).build());
        }

        family_sizer.add_sizer(&family_grid, 0, SizerFlag::All, 2);
        color_sizer.add_sizer(&family_sizer, 0, SizerFlag::All, 4);
    }

    scrolled_win.set_sizer(color_sizer, true);
    scrolled_win.set_scroll_rate(10, 10);
    color_panel.set_sizer(main_sizer, true);

    ColorTabControls {
        panel: color_panel,
        swatches,
    }
}
