//! Grid example demonstrating wxGrid functionality and event handling.
//!
//! This demo exercises all Grid event types, selection queries, cell formatting,
//! coordinate conversion, sorting indicators, cell spanning, frozen rows/cols,
//! cursor movement, and many other wxGrid APIs.

use wxdragon::prelude::*;

/// Format selected-cells, selected-blocks, row-blocks and col-blocks into a
/// single log string so we can verify the new query APIs on every selection
/// change.
fn selection_summary(grid: &Grid) -> String {
    let cells = grid.get_selected_cells();
    let blocks = grid.get_selected_blocks();
    let row_blocks = grid.get_selected_row_blocks();
    let col_blocks = grid.get_selected_col_blocks();

    let mut parts: Vec<String> = Vec::new();

    if !cells.is_empty() {
        let cell_strs: Vec<String> = cells.iter().map(|c| format!("({},{})", c.row, c.col)).collect();
        parts.push(format!("cells=[{}]", cell_strs.join(",")));
    }

    if !blocks.is_empty() {
        let block_strs: Vec<String> = blocks
            .iter()
            .map(|b| format!("({},{})..({},{})", b.top_row, b.left_col, b.bottom_row, b.right_col))
            .collect();
        parts.push(format!("blocks=[{}]", block_strs.join(",")));
    }

    if !row_blocks.is_empty() {
        let rb_strs: Vec<String> = row_blocks
            .iter()
            .map(|b| format!("({},{})..({},{})", b.top_row, b.left_col, b.bottom_row, b.right_col))
            .collect();
        parts.push(format!("row_blocks=[{}]", rb_strs.join(",")));
    }

    if !col_blocks.is_empty() {
        let cb_strs: Vec<String> = col_blocks
            .iter()
            .map(|b| format!("({},{})..({},{})", b.top_row, b.left_col, b.bottom_row, b.right_col))
            .collect();
        parts.push(format!("col_blocks=[{}]", cb_strs.join(",")));
    }

    if parts.is_empty() {
        "  selection: (none)".to_string()
    } else {
        format!("  selection: {}", parts.join(" | "))
    }
}

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("wxGrid Full API Test")
            .with_size(Size::new(900, 700))
            .build();

        frame.create_status_bar(1, 0, -1, "statusbar");

        let panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Create the grid with 10 rows, 6 cols
        let grid = Grid::builder(&panel).build();
        grid.create_grid(10, 6, GridSelectionMode::Cells);

        // --- Column labels ---
        grid.set_col_label_value(0, "Name");
        grid.set_col_label_value(1, "Value");
        grid.set_col_label_value(2, "Type");
        grid.set_col_label_value(3, "Description");
        grid.set_col_label_value(4, "Status");
        grid.set_col_label_value(5, "Extra");

        // --- Corner label ---
        grid.set_corner_label_value("#");
        let corner = grid.get_corner_label_value();
        println!("[grid] corner label = {:?}", corner);

        // --- Label alignment ---
        grid.set_col_label_alignment(0x0004, 0x0004); // wxALIGN_CENTER
        let (h, v) = grid.get_col_label_alignment();
        println!("[grid] col label alignment = ({}, {})", h, v);
        grid.set_row_label_alignment(0x0001, 0x0004); // wxALIGN_LEFT, wxALIGN_CENTER_VERTICAL
        let (h, v) = grid.get_row_label_alignment();
        println!("[grid] row label alignment = ({}, {})", h, v);

        // --- Corner label alignment ---
        grid.set_corner_label_alignment(0x0004, 0x0004);
        let (h, v) = grid.get_corner_label_alignment();
        println!("[grid] corner label alignment = ({}, {})", h, v);

        // --- Col label text orientation ---
        let orient = grid.get_col_label_text_orientation();
        println!("[grid] col label text orientation = {}", orient);

        // --- Default label sizes ---
        println!(
            "[grid] default row label size = {}, col label size = {}",
            grid.get_default_row_label_size(),
            grid.get_default_col_label_size()
        );

        // --- Populate sample data ---
        let data = [
            ("Alpha", "100", "Integer", "First item", "Active", "A"),
            ("Beta", "Hello", "String", "Second item", "Inactive", "B"),
            ("Gamma", "3.14", "Float", "Third item", "Active", "C"),
            ("Delta", "true", "Bool", "Fourth item", "Pending", "D"),
            ("Epsilon", "42", "Integer", "Fifth item", "Active", "E"),
            ("Zeta", "N/A", "Null", "Sixth item", "Disabled", "F"),
            ("Eta", "7.77", "Float", "Seventh item", "Active", "G"),
            ("Theta", "test", "String", "Eighth item", "Inactive", "H"),
            ("Iota", "999", "Integer", "Ninth item", "Active", "I"),
            ("Kappa", "end", "String", "Tenth item", "Done", "J"),
        ];
        for (row, (name, value, typ, desc, status, extra)) in data.iter().enumerate() {
            grid.set_cell_value(row as i32, 0, name);
            grid.set_cell_value(row as i32, 1, value);
            grid.set_cell_value(row as i32, 2, typ);
            grid.set_cell_value(row as i32, 3, desc);
            grid.set_cell_value(row as i32, 4, status);
            grid.set_cell_value(row as i32, 5, extra);
        }

        // --- Auto-size ---
        grid.auto_size_columns(true);
        grid.auto_size_rows(true);
        for col in 0..6 {
            grid.auto_size_col_label_size(col);
        }
        for row in 0..10 {
            grid.auto_size_row_label_size(row);
        }
        println!("[grid] auto-size done");

        // ===== NEW API TESTS (Group A: read-only queries) =====
        if let Some(f) = grid.get_cell_font(0, 0) {
            println!("[grid] cell(0,0) font point size = {}", f.get_point_size());
        }
        if let Some(f) = grid.get_default_cell_font() {
            println!("[grid] default cell font size = {}", f.get_point_size());
        }
        if let Some(f) = grid.get_label_font() {
            println!("[grid] label font size = {}", f.get_point_size());
        }
        let (h, v) = grid.get_col_label_alignment();
        println!("[grid] col label align = ({}, {})", h, v);
        let (h, v) = grid.get_row_label_alignment();
        println!("[grid] row label align = ({}, {})", h, v);
        let corner = grid.get_corner_label_value();
        println!("[grid] corner label = {:?}", corner);
        let (h, v) = grid.get_corner_label_alignment();
        println!("[grid] corner label align = ({}, {})", h, v);
        println!("[grid] col label text orient = {}", grid.get_col_label_text_orientation());
        println!(
            "[grid] corner label text orient = {}",
            grid.get_corner_label_text_orientation()
        );
        println!("[grid] default row label size = {}", grid.get_default_row_label_size());
        println!("[grid] default col label size = {}", grid.get_default_col_label_size());
        println!("[grid] is_using_native_header = {}", grid.is_using_native_header());
        let (span, nr, nc) = grid.get_cell_size(0, 0);
        println!("[grid] cell(0,0) span={:?} nr={} nc={}", span, nr, nc);
        println!("[grid] cell(0,3) overflow = {}", grid.get_cell_overflow(0, 3));
        println!("[grid] default overflow = {}", grid.get_default_cell_overflow());
        println!("[grid] sorting col = {}", grid.get_sorting_column());
        println!("[grid] sort ascending = {}", grid.is_sort_order_ascending());
        println!("[grid] is_sorting_by(0) = {}", grid.is_sorting_by(0));
        println!(
            "[grid] frozen rows={}, cols={}",
            grid.get_number_frozen_rows(),
            grid.get_number_frozen_cols()
        );
        println!("[grid] col_min_acceptable = {}", grid.get_col_minimal_acceptable_width());
        println!("[grid] row_min_acceptable = {}", grid.get_row_minimal_acceptable_height());
        println!("[grid] highlight pen = {}", grid.get_cell_highlight_pen_width());
        println!("[grid] highlight RO pen = {}", grid.get_cell_highlight_ro_pen_width());
        let hc = grid.get_cell_highlight_colour();
        println!("[grid] highlight colour = ({},{},{},{})", hc.r, hc.g, hc.b, hc.a);
        println!("[grid] editable = {}", grid.is_editable());
        println!("[grid] can_enable_cell_ctrl = {}", grid.can_enable_cell_control());
        println!("[grid] read_only(0,0) = {}", grid.is_read_only(0, 0));
        println!("[grid] horz clipped = {}", grid.are_horz_grid_lines_clipped());
        println!("[grid] vert clipped = {}", grid.are_vert_grid_lines_clipped());
        println!(
            "[grid] scroll_line x={}, y={}",
            grid.get_scroll_line_x(),
            grid.get_scroll_line_y()
        );
        println!("[grid] first visible row={}", grid.get_first_fully_visible_row());
        println!("[grid] first visible col={}", grid.get_first_fully_visible_column());
        println!("[grid] can_drag_cell={}", grid.can_drag_cell());
        println!("[grid] can_drag_col_move={}", grid.can_drag_col_move());
        println!("[grid] can_drag_grid_size={}", grid.can_drag_grid_size());
        let rect = grid.cell_to_rect(0, 0);
        println!("[grid] cell(0,0) rect=({},{},{},{})", rect.x, rect.y, rect.width, rect.height);
        println!("[grid] row_at(0)={}, row_pos(0)={}", grid.get_row_at(0), grid.get_row_pos(0));
        println!("[grid] col_at(0)={}, col_pos(0)={}", grid.get_col_at(0), grid.get_col_pos(0));
        println!("[grid] === Group A (read-only queries) done ===");

        // ===== NEW API TESTS (Group B: mutators) =====
        grid.set_corner_label_value("#");
        println!("[grid] B: set_corner_label_value OK");
        grid.set_cell_highlight_pen_width(2);
        println!("[grid] B: set_cell_highlight_pen_width OK");
        grid.set_cell_highlight_colour(Colour::new(255, 0, 0, 255));
        println!("[grid] B: set_cell_highlight_colour OK");
        grid.set_read_only(0, 0, true);
        println!("[grid] B: set_read_only OK");
        grid.set_sorting_column(1, true);
        println!("[grid] B: set_sorting_column OK");
        grid.set_tab_behaviour(TabBehaviour::Wrap);
        println!("[grid] B: set_tab_behaviour OK");
        grid.clip_horz_grid_lines(true);
        grid.clip_vert_grid_lines(true);
        println!("[grid] B: clip_grid_lines OK");
        grid.set_scroll_line_x(20);
        grid.set_scroll_line_y(20);
        println!("[grid] B: set_scroll_line OK");
        grid.enable_drag_row_size(true);
        grid.enable_drag_col_size(true);
        grid.enable_drag_cell(true);
        println!("[grid] B: drag settings OK");
        if let Some(bold_font) = Font::builder().with_weight(FontWeight::Bold).build() {
            grid.set_cell_font(0, 0, &bold_font);
            println!("[grid] B: set_cell_font OK");
        }
        if let Some(label_font) = Font::builder().with_point_size(10).with_weight(FontWeight::Bold).build() {
            grid.set_label_font(&label_font);
            println!("[grid] B: set_label_font OK");
        }
        grid.set_cell_overflow(0, 3, false);
        println!("[grid] B: set_cell_overflow OK");
        grid.set_col_minimal_acceptable_width(30);
        grid.set_row_minimal_acceptable_height(20);
        grid.set_col_minimal_width(0, 60);
        grid.set_row_minimal_height(0, 25);
        println!("[grid] B: minimal sizes OK");
        grid.set_margins(5, 5);
        println!("[grid] B: set_margins OK");
        println!("[grid] === Group B (mutators) done ===");

        // --- Log panel ---
        let log = TextCtrl::builder(&panel)
            .with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly)
            .build();

        sizer.add(&grid, 2, SizerFlag::Expand | SizerFlag::All, 4);
        sizer.add(&log, 1, SizerFlag::Expand | SizerFlag::All, 4);
        panel.set_sizer(sizer, true);

        let append_log = move |msg: String| {
            log.append_text(&format!("{msg}\n"));
            frame.set_status_text(&msg, 0);
        };

        // --- Cell click: also test coordinate conversion ---

        grid.on_cell_left_click(move |e| {
            let pos = e.get_position();
            let col_from_x = grid.x_to_col(pos.x, false);
            let row_from_y = grid.y_to_row(pos.y, false);
            let cell_rect = grid.cell_to_rect(e.get_row(), e.get_col());
            let cursor = grid.get_grid_cursor_coords();
            append_log(format!(
                "CellLeftClick  row={}, col={}, pos=({},{}), x_to_col={}, y_to_row={}, rect=({},{},{},{}), cursor=({},{})",
                e.get_row(),
                e.get_col(),
                pos.x,
                pos.y,
                col_from_x,
                row_from_y,
                cell_rect.x,
                cell_rect.y,
                cell_rect.width,
                cell_rect.height,
                cursor.row,
                cursor.col,
            ));
        });

        grid.on_cell_right_click(move |e| {
            // Test XY-to-cell on right click
            let pos = e.get_position();
            let cell = grid.xy_to_cell(pos.x, pos.y);
            let edge_col = grid.x_to_edge_of_col(pos.x);
            let edge_row = grid.y_to_edge_of_row(pos.y);
            append_log(format!(
                "CellRightClick  row={}, col={}, xy_to_cell=({},{}), edge_col={}, edge_row={}",
                e.get_row(),
                e.get_col(),
                cell.row,
                cell.col,
                edge_col,
                edge_row,
            ));
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

        // Log selection + scrolling/visibility state on every cell select
        grid.on_select_cell(move |e| {
            let row = e.get_row();
            let col = e.get_col();
            let visible = grid.is_visible(row, col, true);
            let first_row = grid.get_first_fully_visible_row();
            let first_col = grid.get_first_fully_visible_column();
            append_log(format!(
                "SelectCell  row={}, col={}, visible={}, first_vis_row={}, first_vis_col={}",
                row, col, visible, first_row, first_col,
            ));
            append_log(selection_summary(&grid));
        });

        grid.on_editor_shown(move |e| {
            let ro = grid.is_current_cell_read_only();
            let shown = grid.is_cell_edit_control_shown();
            append_log(format!(
                "EditorShown  row={}, col={}, current_ro={}, edit_shown={}",
                e.get_row(),
                e.get_col(),
                ro,
                shown,
            ));
        });

        grid.on_editor_hidden(move |e| {
            append_log(format!("EditorHidden  row={}, col={}", e.get_row(), e.get_col(),));
        });

        // --- Resize events ---

        grid.on_row_size(move |e| {
            let row = e.get_row();
            if row >= 0 && row < grid.get_number_rows() {
                let size = grid.get_row_size(row);
                append_log(format!("RowSize  row={}, new_size={}", row, size));
            } else {
                append_log(format!("RowSize  row={} (out of range)", row));
            }
        });

        grid.on_col_size(move |e| {
            let col = e.get_col();
            if col >= 0 && col < grid.get_number_cols() {
                let size = grid.get_col_size(col);
                append_log(format!("ColSize  col={}, new_size={}", col, size));
            } else {
                append_log(format!("ColSize  col={} (out of range)", col));
            }
        });

        // --- Range selection ---

        grid.on_range_selected(move |e| {
            let block_rect = grid.block_to_device_rect(e.get_row(), e.get_col(), e.get_row(), e.get_col());
            append_log(format!(
                "RangeSelected  row={}, col={}, selecting={}, device_rect=({},{},{},{})",
                e.get_row(),
                e.get_col(),
                e.selecting(),
                block_rect.x,
                block_rect.y,
                block_rect.width,
                block_rect.height,
            ));
            append_log(selection_summary(&grid));
        });

        // --- Drag ---

        grid.on_cell_begin_drag(move |e| {
            append_log(format!("CellBeginDrag  row={}, col={}", e.get_row(), e.get_col(),));
        });

        frame.show(true);
        frame.centre();
    });
}
