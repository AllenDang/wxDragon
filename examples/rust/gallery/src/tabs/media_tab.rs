use image::GenericImageView;
use wxdragon::prelude::*;

pub struct MediaControls {
    pub panel: Panel,
    pub animation_ctrl: AnimationCtrl,
    // Add other media controls here later
}

impl MediaControls {
    pub fn bind_events(&self) {
        // Bind events for media controls if any
        self.animation_ctrl.on_mouse_left_down(|_event| {
            log::debug!("AnimationCtrl clicked - this event might not be standard for it, just for testing.");
        });
    }
}

pub fn create_media_tab(notebook: &Notebook) -> MediaControls {
    let panel = Panel::builder(notebook).build();
    // Use a 2-column FlexGridSizer so each row is: [image | text]
    let grid = FlexGridSizer::builder(0, 2).with_gap(Size::new(10, 10)).build();
    // Make right column grow to give more space to text
    grid.add_growable_col(1, 1);

    let animation_bytes = include_bytes!("../../asset/dancing-ferris.gif");
    // Determine animation size
    let animation_size = match image::load_from_memory(animation_bytes) {
        Ok(anim_image) => {
            let (w, h) = anim_image.dimensions();
            log::info!("Loaded animation dimensions: {w}x{h}");
            Size::new(w as i32, h as i32)
        }
        Err(e) => {
            log::warn!("Failed to load animation metadata to get size: {e}. Falling back to default.");
            Size::new(100, 100) // Fallback size
        }
    };

    let animation_ctrl = AnimationCtrl::builder(&panel)
        .with_animation_file("") // Pass empty string for file as we load from bytes
        .with_size(animation_size) // Use determined or fallback size
        .build();

    if animation_ctrl.load_from_bytes(animation_bytes) {
        log::info!("Animation loaded from bytes successfully.");
        if animation_ctrl.play() {
            log::info!("Animation started successfully from bytes.");
        } else {
            log::warn!("Failed to start animation even after loading from bytes.");
        }
    } else {
        log::warn!("Failed to load animation from bytes.");
    }

    // Row 1: Animation (left) and description (right)
    grid.add(&animation_ctrl, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 10);

    let info_text = StaticText::builder(&panel)
        .with_label("Animation loaded from embedded bytes. Dancing Ferris should appear to the left")
        .build();
    grid.add(
        &info_text,
        0,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical | SizerFlag::All,
        10,
    );

    // --- StaticBitmap Demo ---
    let static_bitmap_image_bytes = include_bytes!("../../asset/simple.png"); // Path relative to media_tab.rs
    match image::load_from_memory_with_format(static_bitmap_image_bytes, image::ImageFormat::Png) {
        Ok(img) => {
            let rgba_data = img.to_rgba8();
            let (width, height) = img.dimensions();
            if let Some(bitmap_obj) = Bitmap::from_rgba(rgba_data.as_raw(), width, height) {
                let static_bitmap_ctrl = StaticBitmap::builder(&panel)
                    .with_bitmap(Some(bitmap_obj))
                    .with_size(Size::new(width as i32, height as i32))
                    .build();

                // Row 2: PNG image (left) and label (right)
                grid.add(&static_bitmap_ctrl, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 5);

                let bmp_label = StaticText::builder(&panel)
                    .with_label("StaticBitmap (simple.png from bytes)")
                    .build();
                grid.add(
                    &bmp_label,
                    0,
                    SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical | SizerFlag::All,
                    5,
                );
            } else {
                log::warn!("[MediaTab] Failed to create Bitmap object for StaticBitmap.");
                // Maintain 2-column structure even on error
                grid.add_spacer(1);
                let bmp_error_label = StaticText::builder(&panel)
                    .with_label("StaticBitmap: Error creating Bitmap obj")
                    .build();
                grid.add(
                    &bmp_error_label,
                    0,
                    SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical | SizerFlag::All,
                    5,
                );
            }
        }
        Err(e) => {
            log::warn!("[MediaTab] Failed to load static bitmap: {e}");
            // Maintain 2-column structure even on error
            grid.add_spacer(1);
            let bmp_error_label = StaticText::builder(&panel)
                .with_label("StaticBitmap: Failed to load from bytes")
                .build();
            grid.add(
                &bmp_error_label,
                0,
                SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical | SizerFlag::All,
                5,
            );
        }
    }
    // spacing between sections handled by grid gap

    // SVG Demo
    // Row 3: SVG image (left) and label (right)
    let svg_info_text = StaticText::builder(&panel).with_label("SVG icon").build();
    let svg_icon_bytes = include_bytes!("../../asset/icon_baby.svg");
    let svg_icon_bundle = BitmapBundle::from_svg_data(svg_icon_bytes, Size::new(64, 64)).unwrap();
    let static_bitmap_ctrl = StaticBitmap::builder(&panel)
        .with_bitmap_bundle(Some(svg_icon_bundle))
        .with_size(Size::new(24, 24))
        .build();

    grid.add(&static_bitmap_ctrl, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 5);
    grid.add(
        &svg_info_text,
        0,
        SizerFlag::AlignLeft | SizerFlag::AlignCenterVertical | SizerFlag::All,
        5,
    );

    // Finalize layout
    panel.set_sizer(grid, true);

    MediaControls { panel, animation_ctrl }
}
