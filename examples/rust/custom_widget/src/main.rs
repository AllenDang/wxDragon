use wxdragon::prelude::*;

mod anim_fill_button;
use anim_fill_button::AniFillButton;

mod pie_chart;
use pie_chart::PieChart;

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Custom Widgets Demo - Animated Button & Pie Chart")
            .with_size(Size::new(800, 600))
            .build();

        let panel = Panel::builder(&frame).build();

        // Create animated button
        let button1 = AniFillButton::builder(&panel)
            .with_text("Animated Button".to_string())
            .with_size(Size::new(150, 50))
            .build();

        // Test event chaining: bind event in main.rs to verify compatibility with button's internal event handling
        button1.on_mouse_left_down(|event| {
            println!("🔥 Main.rs: Button clicked! External event handler triggered.");
            event.skip(true); // Important: pass event to subsequent handlers
        });

        // Create pie chart with sample data
        let pie_chart = PieChart::builder(&panel)
            .with_labels(vec![
                "Sales".to_string(),
                "Marketing".to_string(),
                "Development".to_string(),
                "Support".to_string(),
                "Operations".to_string(),
            ])
            .with_amounts(vec![35, 25, 20, 15, 5])
            .with_size(Size::new(400, 300))
            .build();

        // Create main sizer
        let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Add title
        let title = StaticText::builder(&panel)
            .with_label("Custom Widgets Demo - Hover over the button and pie chart!")
            .build();
        main_sizer.add(&title, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 10);

        main_sizer.add_spacer(20);

        // Create horizontal sizer for content
        let content_sizer = BoxSizer::builder(Orientation::Horizontal).build();

        // Left side - button
        let left_sizer = BoxSizer::builder(Orientation::Vertical).build();
        let button_label = StaticText::builder(&panel).with_label("Animated Fill Button:").build();
        left_sizer.add(&button_label, 0, SizerFlag::All, 5);
        left_sizer.add(&button1, 0, SizerFlag::All, 10);

        // Right side - pie chart
        let right_sizer = BoxSizer::builder(Orientation::Vertical).build();
        let chart_label = StaticText::builder(&panel).with_label("Interactive Pie Chart:").build();
        right_sizer.add(&chart_label, 0, SizerFlag::All, 5);
        right_sizer.add(&pie_chart, 1, SizerFlag::Expand | SizerFlag::All, 10);

        // Add to content sizer
        content_sizer.add_sizer(&left_sizer, 0, SizerFlag::All, 20);
        content_sizer.add_sizer(&right_sizer, 1, SizerFlag::Expand | SizerFlag::All, 20);

        main_sizer.add_sizer(&content_sizer, 1, SizerFlag::Expand | SizerFlag::All, 10);

        panel.set_sizer(main_sizer, true);

        frame.show(true);
        frame.centre();
    });
}
