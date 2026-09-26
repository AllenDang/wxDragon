//! Char hook: catching keys a focused control would otherwise swallow.
//!
//! A multiline text control treats Ctrl+Enter as its own, and a list box
//! answers Alt+Enter with a system beep. Neither reaches a `key_down`
//! handler. `wxEVT_CHAR_HOOK` is delivered to the window with focus and
//! every ancestor *before* the key reaches the control, so a frame can act
//! on those combinations and leave everything else alone.

use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Char hook demo")
            .with_size(Size::new(520, 380))
            .build();
        wxdragon::app::set_top_window(&frame);
        let panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        let status = StaticText::builder(&panel)
            .with_label("Try Ctrl+Enter in the box, or Alt+Enter in the list.")
            .build();
        sizer.add(&status, 0, SizerFlag::All, 8);
        let text = TextCtrl::builder(&panel).with_style(TextCtrlStyle::MultiLine).build();
        sizer.add(&text, 1, SizerFlag::Expand | SizerFlag::All, 8);
        let list = ListBox::builder(&panel)
            .with_choices(vec!["first".to_string(), "second".to_string(), "third".to_string()])
            .build();
        list.set_selection(0, true);
        sizer.add(&list, 1, SizerFlag::Expand | SizerFlag::All, 8);
        panel.set_sizer(sizer, true);

        let frame_sizer = BoxSizer::builder(Orientation::Vertical).build();
        frame_sizer.add(&panel, 1, SizerFlag::Expand, 0);
        frame.set_sizer(frame_sizer, true);

        // One handler on the frame sees keys aimed at either control.
        frame.on_char_hook(move |event| {
            let WindowEventData::Keyboard(ref key) = event else {
                event.skip(true);
                return;
            };
            const RETURN: i32 = 13;
            match key.get_key_code() {
                Some(RETURN) if key.control_down() => status.set_label("Ctrl+Enter, caught before the text box"),
                Some(RETURN) if key.alt_down() => status.set_label("Alt+Enter, caught before the list beeps"),
                // Everything else carries on to the control that has focus.
                _ => event.skip(true),
            }
        });

        frame.show(true);
        frame.centre();
    });
}
