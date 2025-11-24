use wxdragon::event::WebViewEvents;
use wxdragon::prelude::*;
use wxdragon::sizers::SizerFlag;
use wxdragon::widgets::{WebView, WebViewFindFlags, WebViewReloadFlags, WebViewUserScriptInjectionTime};

fn main() {
    wxdragon::main(|_app| {
        let frame = Frame::builder()
            .with_title("wxWebView Full Feature Test")
            .with_size(Size::new(1200, 800))
            .build();

        let panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Toolbar
        let toolbar_sizer = BoxSizer::builder(Orientation::Horizontal).build();

        let btn_back = Button::builder(&panel).with_label("Back").build();
        let btn_forward = Button::builder(&panel).with_label("Forward").build();
        let btn_reload = Button::builder(&panel).with_label("Reload").build();
        let btn_zoom_in = Button::builder(&panel).with_label("Zoom+").build();
        let btn_zoom_out = Button::builder(&panel).with_label("Zoom-").build();
        let btn_script = Button::builder(&panel).with_label("Run Script").build();
        let btn_devtools = Button::builder(&panel).with_label("DevTools").build();
        let btn_find = Button::builder(&panel).with_label("Find 'test'").build();

        toolbar_sizer.add(&btn_back, 0, SizerFlag::All, 5);
        toolbar_sizer.add(&btn_forward, 0, SizerFlag::All, 5);
        toolbar_sizer.add(&btn_reload, 0, SizerFlag::All, 5);
        toolbar_sizer.add(&btn_zoom_in, 0, SizerFlag::All, 5);
        toolbar_sizer.add(&btn_zoom_out, 0, SizerFlag::All, 5);
        toolbar_sizer.add(&btn_script, 0, SizerFlag::All, 5);
        toolbar_sizer.add(&btn_devtools, 0, SizerFlag::All, 5);
        toolbar_sizer.add(&btn_find, 0, SizerFlag::All, 5);

        sizer.add_sizer(&toolbar_sizer, 0, SizerFlag::Expand, 0);

        // WebView
        let webview = WebView::builder(&panel).build();

        // Enable dev tools and context menu
        webview.enable_access_to_dev_tools(true);
        webview.enable_context_menu(true);
        webview.enable_history(true);

        // Set custom user agent (optional)
        // webview.set_user_agent("Mozilla/5.0 (Custom) wxWebView/1.0");

        // Add a user script that runs at document start
        webview.add_user_script(
            "console.log('User script injected at document start!');",
            WebViewUserScriptInjectionTime::AtDocumentStart,
        );

        // Load initial URL
        webview.load_url("https://www.google.com");

        sizer.add(&webview, 1, SizerFlag::Expand, 0);

        panel.set_sizer(sizer, true);

        // Event handling - Navigation
        let wv = webview.clone();
        btn_back.on_click(move |_| {
            if wv.can_go_back() {
                wv.go_back();
            }
        });

        let wv = webview.clone();
        btn_forward.on_click(move |_| {
            if wv.can_go_forward() {
                wv.go_forward();
            }
        });

        let wv = webview.clone();
        btn_reload.on_click(move |_| {
            wv.reload(WebViewReloadFlags::NoCache);
        });

        // Zoom controls - using float zoom factor for precise control
        let wv = webview.clone();
        btn_zoom_in.on_click(move |_| {
            let current_zoom = wv.get_zoom_factor();
            wv.set_zoom_factor(current_zoom * 1.2);
            println!("Zoom factor: {}", wv.get_zoom_factor());
        });

        let wv = webview.clone();
        btn_zoom_out.on_click(move |_| {
            let current_zoom = wv.get_zoom_factor();
            wv.set_zoom_factor(current_zoom / 1.2);
            println!("Zoom factor: {}", wv.get_zoom_factor());
        });

        // Advanced script execution
        let wv = webview.clone();
        btn_script.on_click(move |_| {
            // Get page title
            if let Some(title) = wv.run_script("document.title") {
                println!("Page title: {}", title);
            }

            // Get page URL
            if let Some(url) = wv.run_script("window.location.href") {
                println!("Page URL: {}", url);
            }

            // Get page text length
            if let Some(len) = wv.run_script("document.body.innerText.length") {
                println!("Page text length: {} characters", len);
            }

            // Show current URL from API
            println!("Current URL (API): {}", wv.get_current_url());
            println!("Current title (API): {}", wv.get_current_title());
            println!("User agent: {}", wv.get_user_agent());
        });

        // Dev tools toggle
        let wv = webview.clone();
        btn_devtools.on_click(move |_| {
            if wv.is_access_to_dev_tools_enabled() {
                let opened = wv.show_dev_tools();
                println!(
                    "Dev tools {}",
                    if opened { "opened" } else { "already open or not available" }
                );
            } else {
                println!("Dev tools not enabled");
            }
        });

        // Find in page
        let wv = webview.clone();
        btn_find.on_click(move |_| {
            let flags = WebViewFindFlags::HIGHLIGHT_RESULT | WebViewFindFlags::WRAP;
            let count = wv.find("test", flags);
            println!("Found {} occurrences of 'test'", if count >= 0 { count } else { 0 });
        });

        // WebView events
        let wv = webview.clone();
        webview.on_loaded(move |_| {
            println!("Page loaded! URL: {}", wv.get_current_url());
            println!("Page title: {}", wv.get_current_title());
            println!("Is busy: {}", wv.is_busy());
        });

        webview.on_navigating(move |_| {
            println!("Navigating...");
        });

        frame.show(true);
    })
    .unwrap();
}
