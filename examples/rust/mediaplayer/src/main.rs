use std::sync::{Arc, Mutex};
use wxdragon::prelude::*;

struct MediaPlayerFrame {
    frame: Frame,
    media_ctrl: MediaCtrl,
    statusbar: StatusBar,
    current_file: Arc<Mutex<Option<String>>>,
}

impl MediaPlayerFrame {
    fn new() -> Self {
        let frame = Frame::builder()
            .with_title("wxDragon Media Player")
            .with_size(Size::new(800, 600))
            .build();

        let panel = Panel::builder(&frame).build();

        let media_ctrl = MediaCtrl::builder(&panel)
            .with_size(Size::new(800, 500))
            .build();

        let button_panel = Panel::builder(&panel).build();

        // Create control buttons
        let play_button = Button::builder(&button_panel).with_label("Play").build();

        let pause_button = Button::builder(&button_panel).with_label("Pause").build();

        let stop_button = Button::builder(&button_panel).with_label("Stop").build();

        let open_button = Button::builder(&button_panel)
            .with_label("Open File")
            .build();

        // Button sizer
        let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        button_sizer.add(&play_button, 0, SizerFlag::Expand, 5);
        button_sizer.add(&pause_button, 0, SizerFlag::Expand, 5);
        button_sizer.add(&stop_button, 0, SizerFlag::Expand, 5);
        button_sizer.add_spacer(20);
        button_sizer.add(&open_button, 0, SizerFlag::Expand, 5);
        button_panel.set_sizer(button_sizer, true);

        // Main sizer
        let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
        main_sizer.add(&media_ctrl, 1, SizerFlag::Expand, 5);
        main_sizer.add(&button_panel, 0, SizerFlag::Expand, 5);
        panel.set_sizer(main_sizer, true);

        // Status bar
        let statusbar = StatusBar::builder(&frame).build();
        statusbar.set_status_text("Ready", 0);
        frame.set_existing_status_bar(Some(&statusbar));

        // Create and return the frame
        let player_frame = MediaPlayerFrame {
            frame,
            media_ctrl,
            statusbar,
            current_file: Arc::new(Mutex::new(None)),
        };

        // Set up event handlers
        MediaPlayerFrame::setup_event_handlers(
            &player_frame,
            &play_button,
            &pause_button,
            &stop_button,
            &open_button,
        );

        player_frame
    }

    fn setup_event_handlers(
        player: &MediaPlayerFrame,
        play_button: &Button,
        pause_button: &Button,
        stop_button: &Button,
        open_button: &Button,
    ) {
        // Play button event handler
        let media_ctrl = player.media_ctrl.clone();
        let statusbar = player.statusbar.clone();

        play_button.on_click(move |_| {
            if media_ctrl.play() {
                statusbar.set_status_text("Playing", 0);
            } else {
                statusbar.set_status_text("Failed to play", 0);
            }
        });

        // Pause button event handler
        let media_ctrl = player.media_ctrl.clone();
        let statusbar = player.statusbar.clone();

        pause_button.on_click(move |_| {
            if media_ctrl.pause() {
                statusbar.set_status_text("Paused", 0);
            } else {
                statusbar.set_status_text("Failed to pause", 0);
            }
        });

        // Stop button event handler
        let media_ctrl = player.media_ctrl.clone();
        let statusbar = player.statusbar.clone();

        stop_button.on_click(move |_| {
            if media_ctrl.stop() {
                statusbar.set_status_text("Stopped", 0);
            } else {
                statusbar.set_status_text("Failed to stop", 0);
            }
        });

        // Open button event handler
        let media_ctrl = player.media_ctrl.clone();
        let statusbar = player.statusbar.clone();
        let frame = player.frame.clone();
        let current_file = player.current_file.clone();

        open_button.on_click(move |_| {
            let file_dialog = FileDialog::builder(&frame)
                .with_message("Open Media File")
                .with_style(FileDialogStyle::Open)
                .with_wildcard(
                    "Video files (*.mp4;*.avi;*.mkv;*.mov;*.wmv)|*.mp4;*.avi;*.mkv;*.mov;*.wmv",
                )
                .build();

            if file_dialog.show_modal() == ID_OK {
                if let Some(path) = file_dialog.get_path() {
                    statusbar.set_status_text(&format!("Loading: {}", path), 0);

                    if media_ctrl.load(&path) {
                        // Store the current file path
                        let mut file = current_file.lock().unwrap();
                        *file = Some(path.clone());

                        // Update status
                        let file_name = path.split('/').next_back().unwrap_or(&path);
                        statusbar.set_status_text(&format!("Loaded: {}", file_name), 0);
                        frame.set_title(&format!("wxDragon Media Player - {}", file_name));
                    } else {
                        statusbar.set_status_text(&format!("Failed to load: {}", path), 0);
                    }
                }
            }
        });

        // Media state event handlers
        let media_ctrl = player.media_ctrl.clone();
        let statusbar = player.statusbar.clone();

        media_ctrl.on_state_changed(move |event| {
            // Just handle media state directly using raw event data
            // This is a temporary workaround until MediaStateEvent is implemented
            if let Some(state) = event.get_state() {
                let status = match state {
                    MediaState::Stopped => "Stopped",
                    MediaState::Paused => "Paused",
                    MediaState::Playing => "Playing",
                    _ => "Unknown",
                };
                statusbar.set_status_text(status, 0);
            }
        });
    }

    fn show(&self) {
        self.frame.centre();
        self.frame.show(true);
    }
}

fn main() {
    let _ = wxdragon::main(|_app| {
        let player = MediaPlayerFrame::new();
        player.show();
    });
}
