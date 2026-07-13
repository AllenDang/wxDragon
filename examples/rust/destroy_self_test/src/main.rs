//! Regression test for issue #164: calling `destroy()` on a widget from its own
//! event handler used to crash (SIGSEGV / access violation) because child windows
//! were deleted immediately, freeing the event handler still on the dispatch stack.
//!
//! Click "Destroy me": the button should disappear without crashing.

use wxdragon::prelude::*;

fn main() {
    wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Destroy self (issue #164)")
            .with_size(Size::new(300, 200))
            .build();

        let panel = Panel::builder(&frame).build();

        let button = Button::builder(&panel).with_label("Destroy me").build();

        button.on_click(move |_| {
            button.destroy();
        });

        frame.show(true);
    })
    .unwrap();
}
