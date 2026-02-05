use wxdragon::prelude::*;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Hello, World!")
            .with_size(Size::new(300, 200))
            .build();

        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        let text_ctrl = TextCtrl::builder(&frame)
            .with_style(TextCtrlStyle::MultiLine)
            .with_size(Size::new(-1, 60))
            .build();
        let long_text = "x".repeat(1000);
        text_ctrl.set_value(&long_text);
        debug_assert_eq!(text_ctrl.get_value(), long_text);

        let button = Button::builder(&frame).with_label("Click me").build();

        button.on_click(|_| {
            log::info!("Button clicked");
        });

        button.on_destroy(|evt| {
            log::info!("Button is being destroyed");
            evt.skip(true);
        });

        sizer.add(&text_ctrl, 0, SizerFlag::Expand | SizerFlag::All, 8);
        sizer.add_stretch_spacer(1);
        let flag = SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical;
        sizer.add(&button, 0, flag, 0);
        sizer.add_stretch_spacer(1);

        frame.set_sizer(sizer, true);

        frame.on_destroy(|evt| {
            log::info!("Frame is being destroyed");
            evt.skip(true);
        });

        frame.show(true);
        frame.centre();

        // No need to preserve the frame - wxWidgets manages it

        // Frame is automatically managed after show()
    });
}

