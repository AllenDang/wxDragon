//! Regression test for issue #205: `ListCtrl::get_item_text` used to drop the final
//! character of every item and replace it with a NUL byte, because the Rust side
//! passed a buffer that had no room for the terminator ("SERVER" -> "SERVE\0").
//!
//! Run it: every round-trip below must come back byte-for-byte identical.
//! The frame lists the results so the check is also visible on screen.

use wxdragon::prelude::*;

/// Column 0 is filled with `insert_item`, column 1 with `set_item_text_by_column`,
/// so both write paths are covered.
const CASES: &[&str] = &[
    "SERVER",            // the exact case from issue #205
    "A",                 // single byte: truncation used to yield "\0"
    "",                  // empty: probe returns 0, must stay empty
    "Ünïcodé",           // multi-byte: a byte-level cut would corrupt the last char
    "日本語テキスト",    // 3-byte sequences
    "emoji 🐉 tail",     // 4-byte sequence not at the end
    "trailing spaces  ", // must not be trimmed
];

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("ListCtrl::get_item_text round-trip (issue #205)")
            .with_size(Size::new(560, 320))
            .build();

        let panel = Panel::builder(&frame).build();

        let list = ListCtrl::builder(&panel).with_style(ListCtrlStyle::Report).build();
        list.insert_column(0, "Column 0", ListColumnFormat::Left, 240);
        list.insert_column(1, "Column 1", ListColumnFormat::Left, 240);

        for (row, text) in CASES.iter().enumerate() {
            let row = row as i64;
            list.insert_item(row, text, None);
            list.set_item_text_by_column(row, 1, text);
        }

        let mut failures = Vec::new();
        for (row, expected) in CASES.iter().enumerate() {
            for col in 0..2 {
                let actual = list.get_item_text(row as i64, col);
                if actual != *expected {
                    failures.push(format!("row {row} col {col}: expected {expected:?}, got {actual:?}"));
                }
            }
        }

        let status = StaticText::builder(&panel)
            .with_label(&if failures.is_empty() {
                format!("PASS: {} round-trips matched", CASES.len() * 2)
            } else {
                format!("FAIL:\n{}", failures.join("\n"))
            })
            .build();

        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        sizer.add(&list, 1, SizerFlag::Expand | SizerFlag::All, 5);
        sizer.add(&status, 0, SizerFlag::Expand | SizerFlag::All, 5);
        panel.set_sizer(sizer, true);

        frame.show(true);

        assert!(
            failures.is_empty(),
            "get_item_text round-trip failed:\n{}",
            failures.join("\n")
        );
        println!("PASS: {} round-trips matched", CASES.len() * 2);
    })
    .unwrap();
}
