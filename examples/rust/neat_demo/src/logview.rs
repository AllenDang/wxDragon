use wxdragon::prelude::*;

#[allow(dead_code)]
pub struct LogViewPanel {
    pub panel: Panel,
    pub text_ctrl: TextCtrl,
}

impl LogViewPanel {
    pub fn new(parent: &Window) -> Self {
        let panel = Panel::builder(parent).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        let text_ctrl = TextCtrl::builder(&panel)
            .with_size(Size::new(-1, 200))
            .with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly)
            .build();
        sizer.add(
            &text_ctrl,
            1,
            SizerFlag::Expand | SizerFlag::All,
            crate::settings::WIDGET_MARGIN,
        );
        panel.set_sizer(sizer, true);
        Self { panel, text_ctrl }
    }
}
