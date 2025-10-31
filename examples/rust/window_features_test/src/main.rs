use wxdragon::prelude::*;

fn test_window_functions(window: &Frame) {
    println!("Testing new window functions...");

    // Test text measurement functions
    println!("=== Text Measurement Tests ===");
    let text = "Hello, wxDragon!";
    let size = window.get_text_extent(text);
    println!("Text '{}' size: {} x {}", text, size.width, size.height);

    let (full_size, descent, external_leading) = window.get_full_text_extent(text, None);
    println!(
        "Full text extent: {} x {}, descent: {}, external leading: {}",
        full_size.width, full_size.height, descent, external_leading
    );

    let char_height = window.get_char_height();
    let char_width = window.get_char_width();
    println!("Character height: {char_height}, width: {char_width}");

    // Test z-order management
    println!("\n=== Z-Order Management Tests ===");
    println!("Raising window to top of Z-order...");
    window.raise();

    println!("Lowering window to bottom of Z-order...");
    window.lower();

    // Test mouse capture functions
    println!("\n=== Mouse Capture Tests ===");
    println!("Window has capture: {}", window.has_capture());

    // Test comprehensive cursor functionality
    println!("\n=== Comprehensive Cursor Tests ===");
    test_cursor_functionality(window);
}

fn test_cursor_functionality(window: &Frame) {
    println!("Testing stock cursors...");

    // Test all stock cursors
    let stock_cursors = [
        (StockCursor::Arrow, "Arrow"),
        (StockCursor::Hand, "Hand"),
        (StockCursor::Cross, "Cross"),
        (StockCursor::IBeam, "I-Beam"),
        (StockCursor::Wait, "Wait"),
        (StockCursor::SizeNS, "Size N-S"),
        (StockCursor::SizeWE, "Size W-E"),
        (StockCursor::SizeNWSE, "Size NW-SE"),
        (StockCursor::SizeNESW, "Size NE-SW"),
        (StockCursor::Magnifier, "Magnifier"),
        (StockCursor::NoEntry, "No Entry"),
        (StockCursor::Bullseye, "Bullseye"),
        (StockCursor::Watch, "Watch"),
        (StockCursor::Blank, "Blank"),
        (StockCursor::ArrowWait, "Arrow with Wait"),
    ];

    for (cursor_type, name) in &stock_cursors {
        if let Some(cursor) = Cursor::from_stock(*cursor_type) {
            println!("✓ Created {} cursor (valid: {})", name, cursor.is_ok());

            // Test cursor properties
            let hotspot = cursor.get_hotspot();
            println!("  Hotspot: ({}, {})", hotspot.x, hotspot.y);

            // Test setting cursor on window
            window.set_cursor(Some(&cursor));
            println!("  Set {name} cursor on window");

            // Brief pause to see the cursor change
            std::thread::sleep(std::time::Duration::from_millis(500));
        } else {
            println!("✗ Failed to create {name} cursor");
        }
    }

    // Test cursor copy
    if let Some(hand_cursor) = Cursor::from_stock(StockCursor::Hand) {
        if let Some(copied_cursor) = hand_cursor.copy() {
            println!("✓ Successfully copied hand cursor (valid: {})", copied_cursor.is_ok());
        } else {
            println!("✗ Failed to copy hand cursor");
        }
    }

    // Test global cursor functions
    println!("\nTesting global cursor functions...");

    if let Some(wait_cursor) = Cursor::from_stock(StockCursor::Wait) {
        println!("Setting global cursor to wait...");
        set_cursor(Some(&wait_cursor));
        std::thread::sleep(std::time::Duration::from_millis(1000));

        println!("Resetting global cursor to default...");
        set_cursor(None);
    }

    // Test busy cursor functionality
    println!("\nTesting busy cursor functionality...");
    println!("Is busy: {}", is_busy());

    println!("Beginning busy cursor with default wait cursor...");
    begin_busy_cursor(None);
    println!("Is busy: {}", is_busy());
    std::thread::sleep(std::time::Duration::from_millis(1000));

    println!("Ending busy cursor...");
    end_busy_cursor();
    println!("Is busy: {}", is_busy());

    // Test BusyCursor RAII wrapper
    println!("\nTesting BusyCursor RAII wrapper...");
    {
        let _busy = BusyCursor::new(None);
        println!("In busy scope - is busy: {}", is_busy());
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    println!("After busy scope - is busy: {}", is_busy());

    // Test custom busy cursor
    if let Some(watch_cursor) = Cursor::from_stock(StockCursor::Watch) {
        println!("\nTesting custom busy cursor...");
        let _busy = BusyCursor::new(Some(&watch_cursor));
        println!("Using custom watch cursor - is busy: {}", is_busy());
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    println!("After custom busy cursor - is busy: {}", is_busy());

    // Reset window cursor to default
    window.set_cursor(None);
    println!("\nCursor testing completed! Window cursor reset to default.");
}

fn main() {
    println!("=== wxDragon Window Features & Comprehensive Cursor Test ===");

    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Window Features & Cursor Test")
            .with_size(Size::new(800, 600))
            .with_position(Point::new(100, 100))
            .build();

        frame.show(true);
        frame.centre();

        // Test all the new window functions
        test_window_functions(&frame);
    });
}
