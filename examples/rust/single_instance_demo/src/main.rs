//! Single Instance Checker Demo
//!
//! This demo shows how to use SingleInstanceChecker to ensure only one
//! instance of an application runs at a time.
//!
//! Try running this demo twice - the second instance will detect that
//! the first is already running.

use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        // Create a single instance checker with a unique name for this app.
        // The name should be unique to avoid conflicts with other applications.
        // Using a dot prefix makes it a hidden file on Unix systems.
        let checker = SingleInstanceChecker::new(".wxDragon_SingleInstanceDemo", None);

        match checker {
            Some(checker) => {
                if checker.is_another_running() {
                    // Another instance is already running - show a message and exit
                    show_already_running_dialog();
                    return;
                }

                // IMPORTANT: Keep the checker alive for the entire application lifetime.
                // We use Box::leak to prevent it from being dropped. The OS will clean up
                // the lock file when the process exits.
                Box::leak(Box::new(checker));

                // No other instance is running - show the main application window
                show_main_window();
            }
            None => {
                // Failed to create the checker - this is rare but can happen
                // (e.g., permission issues on Unix with lock files)
                // We can either exit or continue without the check
                show_checker_failed_dialog();
            }
        }
    });
}

/// Show a dialog informing the user that another instance is already running
fn show_already_running_dialog() {
    // Create a hidden frame as parent for the dialog
    let frame = Frame::builder().with_title("Hidden").with_size(Size::new(1, 1)).build();

    let dialog = MessageDialog::builder(
        &frame,
        "Another instance of this application is already running.\n\nOnly one instance is allowed at a time.",
        "Application Already Running",
    )
    .with_style(MessageDialogStyle::OK | MessageDialogStyle::IconWarning)
    .build();

    dialog.show_modal();

    // Destroy the hidden frame so the app exits
    frame.destroy();
}

/// Show a dialog informing the user that the instance checker failed to initialize
fn show_checker_failed_dialog() {
    // Create a hidden frame as parent for the dialog
    let frame = Frame::builder().with_title("Hidden").with_size(Size::new(1, 1)).build();

    let dialog = MessageDialog::builder(
        &frame,
        "Failed to initialize the single instance checker.\n\n\
         This could be due to permission issues. The application will continue,\n\
         but multiple instances may be able to run simultaneously.",
        "Warning",
    )
    .with_style(MessageDialogStyle::OK | MessageDialogStyle::IconWarning)
    .build();

    dialog.show_modal();

    // Destroy the hidden frame before showing main window
    frame.destroy();

    // Continue to show the main window anyway
    show_main_window_without_checker();
}

/// Show the main application window
fn show_main_window() {
    let frame = Frame::builder()
        .with_title("Single Instance Demo - Primary Instance")
        .with_size(Size::new(450, 300))
        .build();

    let panel = Panel::builder(&frame).build();
    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    // Status message
    let status_text = StaticText::builder(&panel)
        .with_label("This is the primary instance of the application.")
        .build();
    sizer.add(&status_text, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 20);

    // Info text
    let info_text = StaticText::builder(&panel)
        .with_label(
            "Try running this demo again while this window is open.\n\
             The second instance will detect that this one is already\n\
             running and will show a warning dialog instead.",
        )
        .build();
    sizer.add(&info_text, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);

    // Add a separator
    let line = StaticLine::builder(&panel).build();
    sizer.add(&line, 0, SizerFlag::Expand | SizerFlag::All, 10);

    // Technical info
    let tech_text = StaticText::builder(&panel)
        .with_label(
            "Technical Details:\n\
             - On Windows: Uses a named mutex\n\
             - On Unix/macOS: Uses a lock file in the home directory",
        )
        .build();
    sizer.add(&tech_text, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);

    // Close button
    let close_button = Button::builder(&panel).with_label("Close").build();
    sizer.add(&close_button, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 20);

    close_button.on_click(move |_| {
        frame.close(false);
    });

    panel.set_sizer(sizer, true);
    frame.centre();
    frame.show(true);
}

/// Show the main window without an instance checker (fallback)
fn show_main_window_without_checker() {
    let frame = Frame::builder()
        .with_title("Single Instance Demo - No Lock")
        .with_size(Size::new(450, 250))
        .build();

    let panel = Panel::builder(&frame).build();
    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    let status_text = StaticText::builder(&panel)
        .with_label("Running without single instance protection.")
        .build();
    sizer.add(&status_text, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 20);

    let info_text = StaticText::builder(&panel)
        .with_label("The instance checker failed to initialize.\nMultiple instances may run simultaneously.")
        .build();
    sizer.add(&info_text, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);

    let close_button = Button::builder(&panel).with_label("Close").build();
    sizer.add(&close_button, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 20);

    close_button.on_click(move |_| {
        frame.close(false);
    });

    panel.set_sizer(sizer, true);
    frame.centre();
    frame.show(true);
}
