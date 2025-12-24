use crate::settings::{APP_TITLE, ICON_SIZE, MAIN_ICON, center_rect, create_bitmap_from_memory};
use wxdragon::prelude::*;

pub fn show_about_dialog(parent: &dyn WxWidget) {
    let title = format!("About {}", APP_TITLE);
    let (w, h) = (400, 250);
    let (x, y) = center_rect(parent, w, h);
    let dlg = Dialog::builder(parent, &title).with_position(x, y).with_size(w, h).build();

    let icon_bitmap = create_bitmap_from_memory(MAIN_ICON, Some((ICON_SIZE, ICON_SIZE))).unwrap();
    dlg.set_icon(&icon_bitmap);

    // main horizontal sizer
    let main_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    // left: icon
    let icon = StaticBitmap::builder(&dlg)
        .with_size(Size::new(ICON_SIZE as i32, ICON_SIZE as i32))
        .with_bitmap(Some(icon_bitmap))
        .build();
    let flags = SizerFlag::AlignCenterVertical | SizerFlag::All;
    main_sizer.add(&icon, 0, flags, 10);

    // right: text vertical sizer
    let right_sizer = BoxSizer::builder(Orientation::Vertical).build();
    let info = format!("{APP_TITLE}\n\nAn Neet demo server node GUI manager.\n\nCopyright © 2025 Neet.\nAll rights reserved.");
    let text = StaticText::builder(&dlg).with_label(&info).build();
    right_sizer.add(&text, 1, SizerFlag::Expand | SizerFlag::All, 20);
    let ok_btn = Button::builder(&dlg).with_id(ID_CANCEL).with_label("OK").build();
    let flags = SizerFlag::AlignCenterHorizontal | SizerFlag::All;
    right_sizer.add(&ok_btn, 0, flags, 10);

    main_sizer.add_sizer(&right_sizer, 1, SizerFlag::Expand | SizerFlag::All, 10);

    dlg.set_sizer(main_sizer, true);

    ok_btn.on_click(move |_| {
        dlg.end_modal(0);
        log::info!("About dialog closed.");
    });

    dlg.show_modal();
}
