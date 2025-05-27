use wxdragon::prelude::*;

pub struct BookControlsTab {
    pub tab_panel: Panel,
    pub treebook: Treebook,
}

pub fn create_book_controls_tab(notebook: &Notebook) -> BookControlsTab {
    let tab_panel = Panel::builder(notebook)
        .with_style(PanelStyle::TabTraversal)
        .build();

    let treebook = Treebook::builder(&tab_panel)
        .with_id(ID_HIGHEST + 20) // Example ID
        .build();

    // Page 1: Info Page
    let info_page_panel = Panel::builder(&treebook).build();
    let info_label = StaticText::builder(&info_page_panel)
        .with_label("This is the Treebook's information page.")
        .build();
    let info_button = Button::builder(&info_page_panel)
        .with_label("Info Button")
        .build();
    let info_page_sizer = BoxSizer::builder(Orientation::Vertical).build();
    info_page_sizer.add(&info_label, 0, SizerFlag::All | SizerFlag::Expand, 10);
    info_page_sizer.add(
        &info_button,
        0,
        SizerFlag::All | SizerFlag::AlignCenterHorizontal,
        5,
    );
    info_page_panel.set_sizer(info_page_sizer, true);
    info_page_panel.fit();
    treebook.add_page(&info_page_panel, "Info", true, -1);

    // Page 2: Settings Page
    let settings_page_panel = Panel::builder(&treebook).build();
    let settings_label = StaticText::builder(&settings_page_panel)
        .with_label("Treebook settings would go here.")
        .build();
    let settings_button = Button::builder(&settings_page_panel)
        .with_label("Settings Button")
        .build();
    let settings_page_sizer = BoxSizer::builder(Orientation::Vertical).build();
    settings_page_sizer.add(&settings_label, 0, SizerFlag::All | SizerFlag::Expand, 10);
    settings_page_sizer.add(
        &settings_button,
        0,
        SizerFlag::All | SizerFlag::AlignCenterHorizontal,
        5,
    );
    settings_page_panel.set_sizer(settings_page_sizer, true);
    settings_page_panel.fit();
    let _settings_page_index = treebook.add_page(&settings_page_panel, "Settings", false, -1);

    // Sub-Page for Settings Page
    let advanced_settings_panel = Panel::builder(&treebook).build();
    let advanced_label = StaticText::builder(&advanced_settings_panel)
        .with_label("Advanced Treebook settings.")
        .build();
    let advanced_button = Button::builder(&advanced_settings_panel)
        .with_label("Advanced Button")
        .build();
    let advanced_sizer = BoxSizer::builder(Orientation::Vertical).build();
    advanced_sizer.add(&advanced_label, 0, SizerFlag::All | SizerFlag::Expand, 10);
    advanced_sizer.add(
        &advanced_button,
        0,
        SizerFlag::All | SizerFlag::AlignCenterHorizontal,
        5,
    );
    advanced_settings_panel.set_sizer(advanced_sizer, true);
    advanced_settings_panel.fit();
    treebook.add_sub_page(&advanced_settings_panel, "Advanced", false, -1);

    // Sizer for the main tab panel, to make the Treebook expand
    let main_tab_sizer = BoxSizer::builder(Orientation::Vertical).build();
    main_tab_sizer.add(&treebook, 1, SizerFlag::Expand | SizerFlag::All, 5);

    tab_panel.set_sizer(main_tab_sizer, true);
    tab_panel.fit();

    BookControlsTab {
        tab_panel,
        treebook,
    }
}

impl BookControlsTab {
    pub fn bind_events(&self) {
        // Use on_page_changed instead of bind(EventType::TREEBOOK_PAGE_CHANGED, ...)
        self.treebook.on_page_changed(|event| {
            println!(
                "TREEBOOK_PAGE_CHANGED Event: OldSel={}, NewSel={}, Event={:?}",
                event.get_old_selection().unwrap_or(-2),
                event.get_selection().unwrap_or(-2),
                event
            );
        });
    }
}
