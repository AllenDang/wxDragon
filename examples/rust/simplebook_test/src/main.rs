use wxdragon::dialogs::message_dialog::{MessageDialog, MessageDialogStyle};
use wxdragon::prelude::*;
use wxdragon::widgets::SimpleBook;

mod xrc_test;

fn main() {
    // Check command line arguments to decide which test to run
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "xrc" {
        println!("Running SimpleBook XRC test...");
        xrc_test::run_xrc_test();
    } else {
        println!("Running SimpleBook programmatic test...");
        run_programmatic_test();
    }
}

fn run_programmatic_test() {
    let _ = wxdragon::main(|_| {
        // Create main frame
        let frame = Frame::builder()
            .with_title("SimpleBook Test")
            .with_size(Size::new(500, 400))
            .build();

        let panel = Panel::builder(&frame).build();

        // Create SimpleBook
        let simplebook = SimpleBook::builder(&panel).with_id(ID_HIGHEST + 100).build();

        // Create pages
        create_pages(&simplebook);

        // Create navigation buttons
        let button_panel = Panel::builder(&panel).build();
        let prev_button = Button::builder(&button_panel)
            .with_id(ID_HIGHEST + 101)
            .with_label("Previous")
            .build();
        let next_button = Button::builder(&button_panel)
            .with_id(ID_HIGHEST + 102)
            .with_label("Next")
            .build();
        let info_button = Button::builder(&button_panel)
            .with_id(ID_HIGHEST + 103)
            .with_label("Page Info")
            .build();

        // Layout buttons horizontally
        let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        button_sizer.add(&prev_button, 0, SizerFlag::All, 5);
        button_sizer.add(&next_button, 0, SizerFlag::All, 5);
        button_sizer.add(&info_button, 0, SizerFlag::All, 5);
        button_panel.set_sizer(button_sizer, true);

        // Layout main panel vertically
        let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
        main_sizer.add(&simplebook, 1, SizerFlag::All | SizerFlag::Expand, 5);
        main_sizer.add(&button_panel, 0, SizerFlag::All | SizerFlag::Expand, 5);
        panel.set_sizer(main_sizer, true);

        // Set up event handlers using correct patterns
        setup_event_handlers(&frame, &simplebook, &prev_button, &next_button, &info_button);

        // Show the frame
        frame.create_status_bar(1, 0, ID_ANY as i32, "status");
        frame.set_status_text("SimpleBook Demo - Use buttons to navigate", 0);
        frame.show(true);
        frame.centre();
    });
}

fn create_pages(simplebook: &SimpleBook) {
    // Page 1: Welcome
    let page1 = Panel::builder(simplebook).build();
    let label1 = StaticText::builder(&page1)
        .with_label("Welcome to SimpleBook!\n\nThis is page 1. SimpleBook is a book control without visible tabs.\nNavigation is controlled programmatically using the buttons below.")
        .build();
    let sizer1 = BoxSizer::builder(Orientation::Vertical).build();
    sizer1.add(&label1, 1, SizerFlag::All | SizerFlag::Expand, 20);
    page1.set_sizer(sizer1, true);
    simplebook.add_page(&page1, "Welcome", true, None);

    // Page 2: Features
    let page2 = Panel::builder(simplebook).build();
    let label2 = StaticText::builder(&page2)
        .with_label("Features of SimpleBook:\n\n• No visual tabs - cleaner interface\n• Programmatic navigation\n• Lightweight compared to Notebook\n• Perfect for wizards and guided flows\n• Uses generic book control events")
        .build();
    let sizer2 = BoxSizer::builder(Orientation::Vertical).build();
    sizer2.add(&label2, 1, SizerFlag::All | SizerFlag::Expand, 20);
    page2.set_sizer(sizer2, true);
    simplebook.add_page(&page2, "Features", false, None);

    // Page 3: Controls
    let page3 = Panel::builder(simplebook).build();
    let label3 = StaticText::builder(&page3).with_label("Interactive Controls:").build();
    let checkbox = CheckBox::builder(&page3).with_label("Sample checkbox").build();
    let button = Button::builder(&page3).with_label("Sample button").build();
    let sizer3 = BoxSizer::builder(Orientation::Vertical).build();
    sizer3.add(&label3, 0, SizerFlag::All, 10);
    sizer3.add(&checkbox, 0, SizerFlag::All, 10);
    sizer3.add(&button, 0, SizerFlag::All, 10);
    page3.set_sizer(sizer3, true);
    simplebook.add_page(&page3, "Controls", false, None);

    // Page 4: End
    let page4 = Panel::builder(simplebook).build();
    let label4 = StaticText::builder(&page4)
        .with_label("End of Demo\n\nThis demonstrates how SimpleBook can be used for\nstep-by-step interfaces without cluttering the UI with tabs.\n\nGreat for settings dialogs, wizards, and guided workflows!")
        .build();
    let sizer4 = BoxSizer::builder(Orientation::Vertical).build();
    sizer4.add(&label4, 1, SizerFlag::All | SizerFlag::Expand, 20);
    page4.set_sizer(sizer4, true);
    simplebook.add_page(&page4, "End", false, None);
}

fn setup_event_handlers(
    frame: &Frame,
    simplebook: &SimpleBook,
    prev_button: &Button,
    next_button: &Button,
    info_button: &Button,
) {
    let simplebook_clone = simplebook.clone();

    // Previous button handler
    prev_button.on_click(move |_| {
        let current = simplebook_clone.selection();
        if current > 0 {
            simplebook_clone.set_selection((current - 1) as usize);
        }
    });

    let simplebook_clone2 = simplebook.clone();
    // Next button handler
    next_button.on_click(move |_| {
        let current = simplebook_clone2.selection();
        let page_count = simplebook_clone2.get_page_count() as i32;
        if current < page_count - 1 {
            simplebook_clone2.set_selection((current + 1) as usize);
        }
    });

    let simplebook_clone3 = simplebook.clone();
    let frame_clone = frame.clone();
    // Info button handler
    info_button.on_click(move |_| {
        let current = simplebook_clone3.selection();
        let page_count = simplebook_clone3.get_page_count();
        let message = format!("Current page: {} of {}\nTotal pages: {}", current + 1, page_count, page_count);

        let dialog = MessageDialog::builder(&frame_clone, &message, "Page Info")
            .with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation)
            .build();
        dialog.show_modal();
    });

    // Listen for page change events using the generated method
    let frame_clone2 = frame.clone();
    simplebook.on_page_changed(move |event| {
        if let (Some(old_page), Some(new_page)) = (event.get_old_selection(), event.get_selection()) {
            frame_clone2.set_status_text(&format!("Changed from page {} to page {}", old_page + 1, new_page + 1), 0);
        }
    });
}
