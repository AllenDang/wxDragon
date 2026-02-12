use wxdragon::prelude::*;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Printing Demo")
            .with_size(Size::new(400, 300))
            .build();

        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        let button = Button::builder(&frame).with_label("Print...").build();

        button.on_click({
            let frame_ptr = frame;
            move |_| {
                let mut printer = Printer::new(None);
                let my_printout = TestPrintout;
                if !printer.print(Some(&frame_ptr), "My Print Document", my_printout, true) {
                    log::warn!("Printing failed or was cancelled");
                } else {
                    log::info!("Printing successful");
                }
            }
        });

        sizer.add_stretch_spacer(1);
        sizer.add(&button, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 20);
        sizer.add_stretch_spacer(1);

        frame.set_sizer(sizer, true);
        frame.show(true);
        frame.centre();
    });
}

struct TestPrintout;

impl Printout for TestPrintout {
    fn on_print_page(&mut self, dc: &GenericDC, _page_num: i32) -> bool {
        log::info!("on_print_page called for page {}", _page_num);

        // Draw something on the DC
        dc.draw_text("Hello from wxDragon Printing!", 50, 50);
        dc.draw_circle(100, 150, 40);
        dc.draw_rectangle(200, 120, 100, 60);

        true
    }

    fn get_page_info(&mut self) -> (i32, i32, i32, i32) {
        (1, 1, 1, 1) // Min, Max, From, To
    }

    fn has_page(&mut self, page_num: i32) -> bool {
        page_num == 1
    }
}
