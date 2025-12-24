use crate::music_tree::{MusicNode, NodeType};
use wxdragon::prelude::*;

/// A simple custom dialog to edit a music node's 4 fields.
/// For branch nodes, only Title is editable; other fields are disabled.
pub struct NodeEditDialog {
    dlg: Dialog,
    title: TextCtrl,
    artist: TextCtrl,
    year: TextCtrl,
    quality: TextCtrl,
    node_type: NodeType,
}

impl NodeEditDialog {
    pub fn new(parent: &dyn WxWidget, node: &MusicNode) -> Self {
        // Make dialog resizable by adding ResizeBorder style (and avoid forcing a fixed size)
        let dlg = Dialog::builder(parent, "Edit Node")
            .with_style(DialogStyle::DefaultDialogStyle | DialogStyle::ResizeBorder)
            .build();

        let is_branch = matches!(node.node_type, NodeType::Branch);

        // Top-level vertical sizer
        let vbox = BoxSizer::builder(Orientation::Vertical).build();

        // Grid for labels + editors: use FlexGridSizer so left labels are fixed and right editors are growable
        let grid = FlexGridSizer::builder(4, 2).with_gap(Size::new(8, 8)).build();
        // Make the second column (index 1) grow horizontally; left column remains at best/min size
        grid.add_growable_col(1, 1);

        // Title
        let _lbl_title = StaticText::builder(&dlg).with_label("Title:").build();
        let title = TextCtrl::builder(&dlg).with_value(&node.title).build();
        grid.add(&_lbl_title, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
        grid.add(&title, 1, SizerFlag::Expand, 0);

        // Artist
        let _lbl_artist = StaticText::builder(&dlg).with_label("Artist:").build();
        let artist = TextCtrl::builder(&dlg)
            .with_value(&node.artist.clone().unwrap_or_default())
            .build();
        grid.add(&_lbl_artist, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
        grid.add(&artist, 1, SizerFlag::Expand, 0);

        // Year
        let _lbl_year = StaticText::builder(&dlg).with_label("Year:").build();
        let year_str = node.year.map(|v| v.to_string()).unwrap_or_default();
        let year = TextCtrl::builder(&dlg).with_value(&year_str).build();
        grid.add(&_lbl_year, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
        grid.add(&year, 1, SizerFlag::Expand, 0);

        // Quality
        let _lbl_quality = StaticText::builder(&dlg).with_label("Judgement:").build();
        let quality = TextCtrl::builder(&dlg)
            .with_value(&node.quality.clone().unwrap_or_default())
            .build();
        grid.add(&_lbl_quality, 0, SizerFlag::AlignRight | SizerFlag::AlignCenterVertical, 0);
        grid.add(&quality, 1, SizerFlag::Expand, 0);

        // Make second column grow
        // Note: GridSizer::AddGrowableCol isn't exposed; use Expand flags and outer Fit.

        vbox.add_sizer(&grid, 1, SizerFlag::Expand | SizerFlag::All, 10);

        // Buttons row
        let hbox = BoxSizer::builder(Orientation::Horizontal).build();
        hbox.add_stretch_spacer(1);
        let ok_btn = Button::builder(&dlg).with_id(ID_OK).with_label("OK").build();
        let cancel_btn = Button::builder(&dlg).with_id(ID_CANCEL).with_label("Cancel").build();
        hbox.add(&ok_btn, 0, SizerFlag::All, 4);
        hbox.add(&cancel_btn, 0, SizerFlag::All, 4);
        vbox.add_sizer(
            &hbox,
            0,
            SizerFlag::AlignRight | SizerFlag::Right | SizerFlag::Bottom | SizerFlag::All,
            10,
        );

        // Wire default behaviors
        ok_btn.on_click(move |_| {
            dlg.end_modal(ID_OK);
        });
        cancel_btn.on_click(move |_| {
            dlg.end_modal(ID_CANCEL);
        });

        // Disable non-applicable fields for branch nodes
        if is_branch {
            artist.enable(false);
            year.enable(false);
            quality.enable(false);
        }

        // Attach and fit; keep the earlier explicit min size (don't override it with best size)
        dlg.set_sizer_and_fit(vbox, true);
        dlg.set_min_size(Size::new(400, 256));
        dlg.set_size(Size::new(400, 256));
        dlg.layout();

        Self {
            dlg,
            title,
            artist,
            year,
            quality,
            node_type: node.node_type,
        }
    }

    pub fn show_modal(&self) -> i32 {
        self.dlg.show_modal()
    }

    pub fn value(&self) -> MusicNode {
        // Gather values into locals to control lifetimes of &str passed into MusicNode::new
        let title_s = self.title.get_value();
        let artist_s = self.artist.get_value();
        let year_s = self.year.get_value();
        let quality_s = self.quality.get_value();

        let artist_ref = if self.artist.is_enabled() {
            let trimmed = artist_s.trim();
            if trimmed.is_empty() { None } else { Some(trimmed) }
        } else {
            None
        };

        let year_opt = if self.year.is_enabled() {
            let trimmed = year_s.trim();
            if trimmed.is_empty() {
                None
            } else {
                trimmed.parse::<i32>().ok()
            }
        } else {
            None
        };

        let quality_ref = if self.quality.is_enabled() {
            let trimmed = quality_s.trim();
            if trimmed.is_empty() { None } else { Some(trimmed) }
        } else {
            None
        };

        MusicNode::new(self.node_type, title_s.trim(), artist_ref, year_opt, quality_ref)
    }
}

impl Drop for NodeEditDialog {
    fn drop(&mut self) {
        self.dlg.destroy();
    }
}
