//! Grid example demonstrating wxGrid functionality

use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("wxGrid Example")
            .with_size(Size::new(700, 500))
            .build();

        // Create the grid
        let grid = Grid::builder(&frame)
            .with_pos(Point::new(0, 0))
            .with_size(Size::new(700, 500))
            .build();

        // Initialize the grid with 10 rows and 5 columns
        grid.create_grid(10, 5, GridSelectionMode::Cells);

        // Set column labels
        grid.set_col_label_value(0, "Name");
        grid.set_col_label_value(1, "Value");
        grid.set_col_label_value(2, "Type");
        grid.set_col_label_value(3, "Description");
        grid.set_col_label_value(4, "Status");

        // Set some sample data
        grid.set_cell_value(0, 0, "Item 1");
        grid.set_cell_value(0, 1, "100");
        grid.set_cell_value(0, 2, "Integer");
        grid.set_cell_value(0, 3, "First item");
        grid.set_cell_value(0, 4, "Active");

        grid.set_cell_value(1, 0, "Item 2");
        grid.set_cell_value(1, 1, "Hello");
        grid.set_cell_value(1, 2, "String");
        grid.set_cell_value(1, 3, "Second item");
        grid.set_cell_value(1, 4, "Inactive");

        grid.set_cell_value(2, 0, "Item 3");
        grid.set_cell_value(2, 1, "3.14159");
        grid.set_cell_value(2, 2, "Float");
        grid.set_cell_value(2, 3, "Third item");
        grid.set_cell_value(2, 4, "Active");

        // Auto-size columns to fit content
        grid.auto_size_columns(true);

        // Set up event handlers
        grid.on_cell_left_click(|event| {
            println!("Cell clicked at row: {}, col: {}", event.get_row(), event.get_col());
        });

        grid.on_cell_changed(|event| {
            println!("Cell changed at row: {}, col: {}", event.get_row(), event.get_col());
        });

        grid.on_select_cell(|event| {
            println!("Cell selected at row: {}, col: {}", event.get_row(), event.get_col());
        });

        frame.show(true);
        frame.centre();
    });
}
