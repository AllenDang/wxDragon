//! Minimal Dark Mode Demo for wxDragon
//!
//! This example demonstrates the dark mode support added in wxDragon,
//! based on wxWidgets 3.3.0+ appearance features.

use wxdragon::prelude::*;

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let _ = wxdragon::main(|_| {
        println!("=== wxDragon Dark Mode Demo ===");

        // Check system appearance before setting app appearance
        println!("\n1. System Appearance Information:");
        if let Some(system_appearance) = get_system_appearance() {
            println!("   System is using dark mode: {}", system_appearance.is_dark());
            println!(
                "   System using dark background: {}",
                system_appearance.is_using_dark_background()
            );
            println!("   System appearance name: '{}'", system_appearance.get_name());
        } else {
            println!("   System appearance detection not available");
            println!("   (Requires wxWidgets 3.3.0 or later)");
        }

        // Try to enable system appearance following
        println!("\n2. Setting Application Appearance to System:");
        match set_appearance(Appearance::System) {
            AppearanceResult::Ok => {
                println!("   ✓ Successfully enabled system appearance following");
                println!("   Dark mode will be used automatically on Windows when system uses dark theme");
            }
            AppearanceResult::Failure => {
                println!("   ✗ System appearance following not supported");
                println!("   This requires wxWidgets 3.3.0 or later");
            }
            AppearanceResult::CannotChange => {
                println!("   ⚠ Cannot change appearance now (windows may already exist)");
            }
        }

        // Create a simple window to show the result
        println!("\n3. Creating a simple window to demonstrate the appearance:");
        let frame = Frame::builder()
            .with_title("wxDragon Dark Mode Demo")
            .with_size(Size::new(400, 200))
            .build();

        // Try testing other appearance modes after window creation (should fail)
        println!("\n4. Testing appearance changes after window creation:");

        println!("   Trying to set Light mode:");
        match set_appearance(Appearance::Light) {
            AppearanceResult::Ok => println!("   ✓ Light mode set successfully"),
            AppearanceResult::Failure => println!("   ✗ Light mode not supported"),
            AppearanceResult::CannotChange => println!("   ⚠ Cannot change appearance (windows already exist)"),
        }

        println!("   Trying to set Dark mode:");
        match set_appearance(Appearance::Dark) {
            AppearanceResult::Ok => println!("   ✓ Dark mode set successfully"),
            AppearanceResult::Failure => println!("   ✗ Dark mode not supported"),
            AppearanceResult::CannotChange => println!("   ⚠ Cannot change appearance (windows already exist)"),
        }

        println!("   Trying to set System mode again:");
        match set_appearance(Appearance::System) {
            AppearanceResult::Ok => println!("   ✓ System mode set successfully"),
            AppearanceResult::Failure => println!("   ✗ System mode not supported"),
            AppearanceResult::CannotChange => println!("   ⚠ Cannot change appearance (windows already exist)"),
        }

        println!("\n5. Conclusion:");
        println!("   The window should now be visible.");
        if is_system_dark_mode() {
            println!("   Since your system is using dark mode, the window should appear with dark theme (on Windows).");
        } else {
            println!("   Since your system is using light mode, the window should appear with light theme.");
        }
        println!("   Close the window to exit the demo.");

        frame.show(true);
        set_top_window(&frame);
    });
}
