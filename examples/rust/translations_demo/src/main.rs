//! Translations Demo
//!
//! This example demonstrates wxDragon's internationalization (i18n) support
//! using the Translations API.
//!
//! Note: For actual translations to work, you need .mo files (compiled gettext
//! message catalogs) in the appropriate directory structure:
//!   locale/<lang>/LC_MESSAGES/<domain>.mo
//!
//! For example:
//!   locale/fr/LC_MESSAGES/myapp.mo
//!   locale/de/LC_MESSAGES/myapp.mo

use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        // Initialize the translations system
        setup_translations();

        // Create the main frame
        let frame = Frame::builder()
            .with_title("Translations Demo")
            .with_size(Size::new(600, 500))
            .build();

        let panel = Panel::builder(&frame).build();
        let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

        // --- Language Selection Section ---
        let lang_box = StaticBox::builder(&panel).with_label("Language Selection").build();
        let lang_sizer = StaticBoxSizerBuilder::new_with_box(&lang_box, Orientation::Vertical).build();

        let lang_choice_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        let lang_label = StaticText::builder(&panel).with_label("Select Language:").build();

        let language_choices: Vec<String> = vec![
            "System Default",
            "English (US)",
            "French",
            "German",
            "Spanish",
            "Chinese (Simplified)",
            "Japanese",
            "Russian",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let lang_choice = Choice::builder(&panel).with_choices(language_choices).build();
        lang_choice.set_selection(0);

        lang_choice_sizer.add(&lang_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
        lang_choice_sizer.add(&lang_choice, 1, SizerFlag::Expand | SizerFlag::Left, 5);
        lang_sizer.add_sizer(&lang_choice_sizer, 0, SizerFlag::Expand | SizerFlag::All, 5);

        // Current language display
        let current_lang_label = StaticText::builder(&panel).with_label("Current: System Default").build();
        lang_sizer.add(&current_lang_label, 0, SizerFlag::All, 5);

        main_sizer.add_sizer(&lang_sizer, 0, SizerFlag::Expand | SizerFlag::All, 10);

        // --- Translation Test Section ---
        let trans_box = StaticBox::builder(&panel).with_label("Translation Test").build();
        let trans_sizer = StaticBoxSizerBuilder::new_with_box(&trans_box, Orientation::Vertical).build();

        // Input text
        let input_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        let input_label = StaticText::builder(&panel).with_label("Text to translate:").build();
        let input_text = TextCtrl::builder(&panel).with_value("Hello").build();
        input_sizer.add(&input_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
        input_sizer.add(&input_text, 1, SizerFlag::Expand | SizerFlag::Left, 5);
        trans_sizer.add_sizer(&input_sizer, 0, SizerFlag::Expand | SizerFlag::All, 5);

        // Translate button
        let translate_btn = Button::builder(&panel).with_label("Translate").build();
        trans_sizer.add(&translate_btn, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 5);

        // Output text
        let output_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        let output_label = StaticText::builder(&panel).with_label("Result:").build();
        let output_text = TextCtrl::builder(&panel).with_style(TextCtrlStyle::ReadOnly).build();
        output_sizer.add(&output_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
        output_sizer.add(&output_text, 1, SizerFlag::Expand | SizerFlag::Left, 5);
        trans_sizer.add_sizer(&output_sizer, 0, SizerFlag::Expand | SizerFlag::All, 5);

        main_sizer.add_sizer(&trans_sizer, 0, SizerFlag::Expand | SizerFlag::All, 10);

        // --- Plural Translation Section ---
        let plural_box = StaticBox::builder(&panel).with_label("Plural Translation Test").build();
        let plural_sizer = StaticBoxSizerBuilder::new_with_box(&plural_box, Orientation::Vertical).build();

        let count_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        let count_label = StaticText::builder(&panel).with_label("Item count:").build();
        let count_spin = SpinCtrl::builder(&panel)
            .with_min_value(0)
            .with_max_value(100)
            .with_initial_value(1)
            .build();
        count_sizer.add(&count_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
        count_sizer.add(&count_spin, 0, SizerFlag::Left, 5);
        plural_sizer.add_sizer(&count_sizer, 0, SizerFlag::All, 5);

        let plural_result = StaticText::builder(&panel).with_label("1 item selected").build();
        plural_sizer.add(&plural_result, 0, SizerFlag::All, 5);

        main_sizer.add_sizer(&plural_sizer, 0, SizerFlag::Expand | SizerFlag::All, 10);

        // --- Translations Info Section ---
        let info_box = StaticBox::builder(&panel).with_label("Translations Info").build();
        let info_sizer = StaticBoxSizerBuilder::new_with_box(&info_box, Orientation::Vertical).build();

        let info_text = TextCtrl::builder(&panel)
            .with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly)
            .with_size(Size::new(-1, 100))
            .build();

        // Display initial info
        let info = get_translations_info();
        info_text.set_value(&info);

        info_sizer.add(&info_text, 1, SizerFlag::Expand | SizerFlag::All, 5);

        let refresh_btn = Button::builder(&panel).with_label("Refresh Info").build();
        info_sizer.add(&refresh_btn, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 5);

        main_sizer.add_sizer(&info_sizer, 1, SizerFlag::Expand | SizerFlag::All, 10);

        panel.set_sizer(main_sizer, true);

        // --- Event Handlers ---

        // Language selection changed
        lang_choice.on_selection_changed(move |_| {
            let selection = lang_choice.get_selection().unwrap_or(0);

            // Use language code strings that match our directory names
            let lang_code = match selection {
                0 => "", // System default
                1 => "en_US",
                2 => "fr",
                3 => "de",
                4 => "es",
                5 => "zh_CN",
                6 => "ja",
                7 => "ru",
                _ => "",
            };

            // To change language at runtime, we need to create a new Translations
            // instance, set the language, and reload catalogs
            let translations = Translations::new();

            if !lang_code.is_empty() {
                println!("Setting language to: {}", lang_code);
                translations.set_language_str(lang_code);
            }

            // Get absolute path for catalog lookup
            let locale_path = std::env::current_dir()
                .map(|p| p.join("examples/rust/translations_demo/locale"))
                .unwrap_or_else(|_| std::path::PathBuf::from("./locale"));
            add_catalog_lookup_path_prefix(locale_path.to_str().unwrap_or("./locale"));

            // Reload the catalog for the new language
            if translations.add_catalog("translations_demo") {
                println!("SUCCESS: Catalog loaded for language: {}", lang_code);
            } else {
                println!("FAILED: No catalog found for language: {}", lang_code);
            }

            // Set as global
            Translations::set_global(translations);

            // Update label
            let lang_name = match selection {
                0 => "System Default",
                1 => "English (US)",
                2 => "French",
                3 => "German",
                4 => "Spanish",
                5 => "Chinese (Simplified)",
                6 => "Japanese",
                7 => "Russian",
                _ => "Unknown",
            };
            current_lang_label.set_label(&format!("Current: {}", lang_name));

            println!("Language changed to: {}", lang_name);
        });

        // Translate button clicked
        translate_btn.on_click(move |_| {
            let text = input_text.get_value();

            // Try using the translations API directly for more control
            let translated = if let Some(translations) = Translations::get() {
                println!("Got translations instance");
                if translations.is_loaded("translations_demo") {
                    println!("Catalog 'translations_demo' is loaded");
                } else {
                    println!("Catalog 'translations_demo' is NOT loaded");
                }
                translations
                    .get_string(&text, "translations_demo")
                    .or_else(|| translations.get_string(&text, ""))
                    .unwrap_or_else(|| text.clone())
            } else {
                println!("No translations instance available");
                text.clone()
            };

            output_text.set_value(&translated);

            if translated == text {
                println!("Translation for '{}': '{}' (no translation found)", text, translated);
            } else {
                println!("Translation for '{}': '{}'", text, translated);
            }
        });

        // Spin control value changed - update plural text
        count_spin.on_value_changed(move |_| {
            let count = count_spin.value() as u32;
            let text = translate_plural("1 item selected", "%d items selected", count);
            // Replace %d with actual count for display (standard gettext format)
            let display_text = text.replace("%d", &count.to_string());
            plural_result.set_label(&display_text);
        });

        // Refresh info button
        refresh_btn.on_click(move |_| {
            let info = get_translations_info();
            info_text.set_value(&info);
        });

        frame.show(true);
        frame.centre();
    });
}

/// Set up the translations system
fn setup_translations() {
    // Get the locale path - try to find it relative to the current directory
    let locale_path = std::env::current_dir()
        .map(|p| p.join("examples/rust/translations_demo/locale"))
        .unwrap_or_else(|_| std::path::PathBuf::from("./locale"));

    println!("Looking for translations in: {:?}", locale_path);

    // Add catalog lookup path
    add_catalog_lookup_path_prefix(locale_path.to_str().unwrap_or("./locale"));

    // Create a new translations instance
    let translations = Translations::new();

    // Add the wxWidgets standard catalog (for OK, Cancel, etc.)
    if translations.add_std_catalog() {
        println!("wxWidgets standard catalog loaded");
    } else {
        println!("wxWidgets standard catalog not available");
    }

    // Try to add a custom catalog (may not load if system language doesn't match)
    if translations.add_catalog("translations_demo") {
        println!("Custom catalog 'translations_demo' loaded");
    } else {
        println!("Custom catalog 'translations_demo' not found (will load when language is changed)");
    }

    // Set as the global translations instance
    Translations::set_global(translations);

    println!("Translations system initialized");
}

/// Get information about the current translations state
fn get_translations_info() -> String {
    let mut info = String::new();

    info.push_str("=== Translations Info ===\n\n");

    if let Some(translations) = Translations::get() {
        info.push_str("Translations instance: Active\n");

        // Check if standard catalog is loaded
        if translations.is_loaded("wxstd") {
            info.push_str("wxWidgets standard catalog: Loaded\n");
        } else {
            info.push_str("wxWidgets standard catalog: Not loaded\n");
        }

        // Check custom catalog
        if translations.is_loaded("translations_demo") {
            info.push_str("Custom catalog: Loaded\n");
        } else {
            info.push_str("Custom catalog: Not loaded\n");
        }

        // Try to get available translations
        let available = translations.get_available_translations("wxstd");
        if available.is_empty() {
            info.push_str("Available translations: None found\n");
        } else {
            info.push_str(&format!("Available translations: {}\n", available.join(", ")));
        }

        // Try to get a header value
        if let Some(plural_forms) = translations.get_header_value("Plural-Forms", "") {
            info.push_str(&format!("Plural-Forms: {}\n", plural_forms));
        }
    } else {
        info.push_str("Translations instance: Not active\n");
    }

    info.push_str("\n=== Usage Notes ===\n");
    info.push_str("To see actual translations, you need:\n");
    info.push_str("1. Create .po files with translations\n");
    info.push_str("2. Compile them to .mo files using msgfmt\n");
    info.push_str("3. Place in locale/<lang>/LC_MESSAGES/\n");

    info
}
