use wxdragon::event::WebViewEvents;
use wxdragon::prelude::*;
use wxdragon::sizers::SizerFlag;
use wxdragon::widgets::{
    WebView, WebViewFindFlags, WebViewReloadFlags, WebViewUserScriptInjectionTime, WebViewZoom, WEBVIEW_BACKEND_DEFAULT,
    WEBVIEW_BACKEND_EDGE,
};

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
        // Check available backends and prefer Edge on Windows
        println!("Checking available WebView backends...");
        println!(
            "  Edge backend available: {}",
            WebView::is_backend_available(WEBVIEW_BACKEND_EDGE)
        );
        println!(
            "  Default backend available: {}",
            WebView::is_backend_available(WEBVIEW_BACKEND_DEFAULT)
        );

        // Use Edge if available (modern Chromium-based), otherwise fall back to default
        // On Windows: Edge requires WebView2 runtime, otherwise falls back to IE
        // On macOS: Uses WebKit
        // On Linux: Uses WebKit2
        let backend = if WebView::is_backend_available(WEBVIEW_BACKEND_EDGE) {
            println!("Using Edge (WebView2) backend");
            Some(WEBVIEW_BACKEND_EDGE.to_string())
        } else {
            println!("Edge not available, using default backend");
            println!("WARNING: On Windows without WebView2, the IE backend will be used.");
            println!("         IE backend has limited compatibility with modern websites.");
            println!("         Install WebView2 runtime for better results:");
            println!("         https://developer.microsoft.com/en-us/microsoft-edge/webview2/");
            Some(WEBVIEW_BACKEND_DEFAULT.to_string())
        };

        let webview = WebView::builder(&panel).with_backend(backend).build();

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

        // Print backend info
        let backend = webview.get_backend();
        println!("WebView backend: {}", backend);

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

        // Zoom controls - using discrete zoom levels for IE compatibility
        // Note: get_zoom_factor()/set_zoom_factor() may not work on IE backend
        let wv = webview.clone();
        let backend_for_zoom_in = backend.clone();
        btn_zoom_in.on_click(move |_| {
            // Use discrete zoom levels which work on all backends including IE
            let current_zoom = wv.get_zoom();
            let new_zoom = match current_zoom {
                WebViewZoom::Tiny => WebViewZoom::Small,
                WebViewZoom::Small => WebViewZoom::Medium,
                WebViewZoom::Medium => WebViewZoom::Large,
                WebViewZoom::Large => WebViewZoom::Largest,
                WebViewZoom::Largest => WebViewZoom::Largest,
            };
            wv.set_zoom(new_zoom);
            println!("Zoom level: {:?} (backend: {})", wv.get_zoom(), backend_for_zoom_in);
        });

        let wv = webview.clone();
        let backend_for_zoom_out = backend.clone();
        btn_zoom_out.on_click(move |_| {
            let current_zoom = wv.get_zoom();
            let new_zoom = match current_zoom {
                WebViewZoom::Tiny => WebViewZoom::Tiny,
                WebViewZoom::Small => WebViewZoom::Tiny,
                WebViewZoom::Medium => WebViewZoom::Small,
                WebViewZoom::Large => WebViewZoom::Medium,
                WebViewZoom::Largest => WebViewZoom::Large,
            };
            wv.set_zoom(new_zoom);
            println!("Zoom level: {:?} (backend: {})", wv.get_zoom(), backend_for_zoom_out);
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
