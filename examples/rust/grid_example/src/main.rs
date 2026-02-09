//! Grid example demonstrating wxGrid functionality and event handling.
//!
//! This demo exercises all Grid event types to verify they fire correctly.
//! Events are logged to both the status bar and a TextCtrl log panel.

use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("wxGrid Event Test")
            .with_size(Size::new(800, 600))
            .build();

        // Status bar for the most recent event
        frame.create_status_bar(1, 0, -1, "statusbar");

        // Main layout: grid on top, log panel on bottom
        let panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Create the grid
        let grid = Grid::builder(&panel).build();
        grid.create_grid(8, 5, GridSelectionMode::Cells);

        // Set column labels
        grid.set_col_label_value(0, "Name");
        grid.set_col_label_value(1, "Value");
        grid.set_col_label_value(2, "Type");
        grid.set_col_label_value(3, "Description");
        grid.set_col_label_value(4, "Status");

        // Populate sample data
        let data = [
            ("Alpha", "100", "Integer", "First item", "Active"),
            ("Beta", "Hello", "String", "Second item", "Inactive"),
            ("Gamma", "3.14", "Float", "Third item", "Active"),
            ("Delta", "true", "Bool", "Fourth item", "Pending"),
            ("Epsilon", "42", "Integer", "Fifth item", "Active"),
            ("Zeta", "N/A", "Null", "Sixth item", "Disabled"),
            ("Eta", "7.77", "Float", "Seventh item", "Active"),
            ("Theta", "test", "String", "Eighth item", "Inactive"),
        ];
        for (row, (name, value, typ, desc, status)) in data.iter().enumerate() {
            grid.set_cell_value(row as i32, 0, name);
            grid.set_cell_value(row as i32, 1, value);
            grid.set_cell_value(row as i32, 2, typ);
            grid.set_cell_value(row as i32, 3, desc);
            grid.set_cell_value(row as i32, 4, status);
        }

        grid.auto_size_columns(true);

        // Log panel (read-only multi-line text control)
        let log = TextCtrl::builder(&panel)
            .with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly)
            .build();

        sizer.add(&grid, 2, SizerFlag::Expand | SizerFlag::All, 4);
        sizer.add(&log, 1, SizerFlag::Expand | SizerFlag::All, 4);
        panel.set_sizer(sizer, true);

        // Helper: append a line to the log and update the status bar.
        // Both TextCtrl and Frame are Copy, so the closure is Copy too.
        let append_log = move |msg: String| {
            log.append_text(&format!("{msg}\n"));
            frame.set_status_text(&msg, 0);
        };

        // --- Cell click events ---

        grid.on_cell_left_click(move |e| {
            append_log(format!(
                "CellLeftClick  row={}, col={}, pos=({},{})",
                e.get_row(),
                e.get_col(),
                e.get_position().x,
                e.get_position().y,
            ));
        });

        grid.on_cell_right_click(move |e| {
            append_log(format!("CellRightClick  row={}, col={}", e.get_row(), e.get_col(),));
        });

        grid.on_cell_left_dclick(move |e| {
            append_log(format!("CellLeftDClick  row={}, col={}", e.get_row(), e.get_col(),));
        });

        grid.on_cell_right_dclick(move |e| {
            append_log(format!("CellRightDClick  row={}, col={}", e.get_row(), e.get_col(),));
        });

        // --- Label click events ---

        grid.on_label_left_click(move |e| {
            append_log(format!("LabelLeftClick  row={}, col={}", e.get_row(), e.get_col(),));
        });

        grid.on_label_right_click(move |e| {
            append_log(format!("LabelRightClick  row={}, col={}", e.get_row(), e.get_col(),));
        });

        grid.on_label_left_dclick(move |e| {
            append_log(format!("LabelLeftDClick  row={}, col={}", e.get_row(), e.get_col(),));
        });

        grid.on_label_right_dclick(move |e| {
            append_log(format!("LabelRightDClick  row={}, col={}", e.get_row(), e.get_col(),));
        });

        // --- Cell editing events ---

        grid.on_cell_changed(move |e| {
            append_log(format!("CellChanged  row={}, col={}", e.get_row(), e.get_col(),));
        });

        grid.on_select_cell(move |e| {
            append_log(format!(
                "SelectCell  row={}, col={}, selecting={}",
                e.get_row(),
                e.get_col(),
                e.selecting(),
            ));
        });

        grid.on_editor_shown(move |e| {
            append_log(format!("EditorShown  row={}, col={}", e.get_row(), e.get_col(),));
        });

        grid.on_editor_hidden(move |e| {
            append_log(format!("EditorHidden  row={}, col={}", e.get_row(), e.get_col(),));
        });

        // --- Resize events ---

        grid.on_row_size(move |e| {
            append_log(format!("RowSize  row={}", e.get_row()));
        });

        grid.on_col_size(move |e| {
            append_log(format!("ColSize  col={}", e.get_col()));
        });

        // --- Range selection ---

        grid.on_range_selected(move |e| {
            append_log(format!(
                "RangeSelected  row={}, col={}, selecting={}",
                e.get_row(),
                e.get_col(),
                e.selecting(),
            ));
        });

        // --- Drag ---

        grid.on_cell_begin_drag(move |e| {
            append_log(format!("CellBeginDrag  row={}, col={}", e.get_row(), e.get_col(),));
        });

        frame.show(true);
        frame.centre();
    });
}
