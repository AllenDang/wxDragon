use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Hello, World!")
            .with_size(Size::new(300, 200))
            .build();

        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        let button = Button::builder(&frame).with_label("Click me").build();

        button.on_click(|_| {
            println!("Button clicked");
        });

        sizer.add_stretch_spacer(1);
        let flag = SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical;
        sizer.add(&button, 0, flag, 0);
        sizer.add_stretch_spacer(1);

        frame.set_sizer(sizer, true);

        frame.show(true);
        frame.centre();

        // No need to preserve the frame - wxWidgets manages it

        // Frame is automatically managed after show()
    });
}
