//! GUI regression for dialogs that scroll when they're too big for the screen. Requires a desktop.
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wxdragon::prelude::*;

/// A dialog far taller than any display, with OK and Cancel at the bottom. Returns the dialog and its OK button.
fn tall_dialog(parent: &Frame, mode: DialogLayoutAdaptation) -> (Dialog, Button) {
    let dialog = Dialog::builder(parent, "Tall").build();
    dialog.set_layout_adaptation_mode(mode);
    let sizer = BoxSizer::builder(Orientation::Vertical).build();
    for n in 0..300 {
        let check = CheckBox::builder(&dialog).with_label(&format!("Option {n}")).build();
        sizer.add(&check, 0, SizerFlag::All, 5);
    }
    let buttons = StdDialogButtonSizerBuilder::new().build();
    let ok = Button::builder(&dialog).with_id(ID_OK).with_label("OK").build();
    buttons.add_button(&ok);
    buttons.add_button(&Button::builder(&dialog).with_id(ID_CANCEL).with_label("Cancel").build());
    buttons.realize();
    sizer.add_sizer(&buttons, 0, SizerFlag::Expand | SizerFlag::All, 5);
    dialog.set_sizer_and_fit(sizer, true);
    (dialog, ok)
}

/// Whether the whole of `button` lies inside its dialog's client area, where it can be seen and clicked.
fn button_visible(dialog: Dialog, button: Button) -> bool {
    let bottom = button.get_position().y + button.get_size().height;
    bottom <= dialog.get_client_size().height
}

fn verify_layout_adaptation(frame: Frame) {
    assert!(!Dialog::is_layout_adaptation_enabled(), "layout adaptation should start off");
    Dialog::enable_layout_adaptation(true);
    assert!(Dialog::is_layout_adaptation_enabled());
    Dialog::enable_layout_adaptation(false);
    assert!(!Dialog::is_layout_adaptation_enabled());

    let (fixed, fixed_ok) = tall_dialog(&frame, DialogLayoutAdaptation::Disabled);
    let (adapted, adapted_ok) = tall_dialog(&frame, DialogLayoutAdaptation::Enabled);
    let (followed, _) = tall_dialog(&frame, DialogLayoutAdaptation::Default);
    assert_eq!(fixed.get_layout_adaptation_mode(), DialogLayoutAdaptation::Disabled);
    assert_eq!(adapted.get_layout_adaptation_mode(), DialogLayoutAdaptation::Enabled);
    assert_eq!(followed.get_layout_adaptation_mode(), DialogLayoutAdaptation::Default);

    // Showing is when wxWidgets checks the size against the display and adds the scrolled window.
    // Without it, the display cuts the dialog off and the buttons end up below its bottom edge.
    fixed.show(true);
    adapted.show(true);
    assert!(!button_visible(fixed, fixed_ok), "without adaptation, OK should be cut off");
    assert!(
        button_visible(adapted, adapted_ok),
        "with adaptation, OK should stay on screen"
    );
    fixed.destroy();
    adapted.destroy();
    followed.destroy();
}

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let finished = Rc::new(Cell::new(false));
    let finished_in_loop = finished.clone();
    let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
    let timer_in_loop = timer_store.clone();
    wxdragon::main(move |app| {
        let frame = Frame::builder().with_title("Dialog layout adaptation").build();
        // Install the exit timer first: caught assertion panics must not hang the test.
        let timer = Timer::new(&frame);
        let timer_cleanup = timer_in_loop.clone();
        timer.on_tick(move |_| {
            timer_cleanup.borrow_mut().take();
            app.exit_main_loop();
        });
        timer.start(100, true);
        timer_in_loop.borrow_mut().replace(timer);

        verify_layout_adaptation(frame);
        finished_in_loop.set(true);
    })
    .expect("could not initialize wxWidgets for dialog layout adaptation regressions");
    assert!(finished.get(), "dialog layout adaptation assertions did not finish");
}
