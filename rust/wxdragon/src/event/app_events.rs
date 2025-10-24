//! Application-level events (macOS-specific)

/// Trait for handling application-level events
///
/// This trait is implemented by the App handle and provides methods for
/// registering callbacks for application-level events. These events
/// are triggered by the operating system when users interact with the
/// application in platform-specific ways.
///
/// # Platform Support
///
/// All methods in this trait compile on all platforms but may only be functional
/// on certain platforms. On unsupported platforms, the methods are no-ops.
///
/// # Multiple Handlers
///
/// Like other wxdragon event handlers, you can register multiple callbacks for the
/// same event type. All registered callbacks will be called in the order they were registered.
///
/// # Example
///
/// ```no_run
/// use wxdragon::prelude::*;
///
/// wxdragon::main(|app| {
///     // Register handler for file opens
///     app.on_open_files(|files| {
///         for file in files {
///             println!("Opening: {}", file);
///         }
///     });
///
///     // Register handler for URL opens
///     app.on_open_url(|url| {
///         println!("Opening URL: {}", url);
///     });
///
///     // Register handler for app activation
///     app.on_reopen_app(|| {
///         println!("App reopened");
///     });
///
///     let frame = Frame::builder()
///         .with_title("My App")
///         .build();
///     frame.show(true);
/// })
/// .unwrap();
/// ```
pub trait AppEvents {
    /// Binds a handler for when files are opened with the application
    ///
    /// This callback is invoked when files are opened with your application through:
    /// - Right-click "Open With" in file manager (macOS, Windows, Linux)
    /// - Drag-and-drop onto your application icon
    /// - Double-clicking files associated with your application
    ///
    /// # Arguments
    /// * `callback` - A closure that receives a `Vec<String>` of file paths
    ///
    /// # Platform Support
    /// - **macOS**: Fully supported via Apple Events
    /// - **Windows**: Planned for future implementation
    /// - **Linux**: Planned for future implementation
    ///
    /// # Example
    /// ```no_run
    /// use wxdragon::prelude::*;
    ///
    /// wxdragon::main(|app| {
    ///     app.on_open_files(|files| {
    ///         println!("Opening {} files", files.len());
    ///         for file in files {
    ///             println!("  - {}", file);
    ///         }
    ///     });
    ///
    ///     let frame = Frame::builder().with_title("My App").build();
    ///     frame.show(true);
    /// })
    /// .unwrap();
    /// ```
    fn on_open_files<F>(&self, callback: F)
    where
        F: Fn(Vec<String>) + Send + 'static;

    /// Binds a handler for when a URL is opened with the application
    ///
    /// This callback is invoked when a URL is opened with your application,
    /// typically through a custom URL scheme (e.g., `myapp://action`).
    ///
    /// # Arguments
    /// * `callback` - A closure that receives a `String` containing the URL
    ///
    /// # Platform Support
    /// - **macOS**: Fully supported via Apple Events
    /// - **Windows**: Planned for future implementation
    /// - **Linux**: Planned for future implementation
    fn on_open_url<F>(&self, callback: F)
    where
        F: Fn(String) + Send + 'static;

    /// Binds a handler for when user requests a new document
    ///
    /// This callback is invoked when the user selects "New" from the File menu
    /// or uses the standard keyboard shortcut (Cmd+N on macOS, Ctrl+N on Windows/Linux).
    ///
    /// # Arguments
    /// * `callback` - A closure that takes no arguments
    ///
    /// # Platform Support
    /// - **macOS**: Fully supported
    /// - **Windows**: No-op (typically handled via menu events)
    /// - **Linux**: No-op (typically handled via menu events)
    fn on_new_file<F>(&self, callback: F)
    where
        F: Fn() + Send + 'static;

    /// Binds a handler for when the application is reopened/activated
    ///
    /// This callback is invoked when the user reactivates the application:
    /// - macOS: Clicking the Dock icon while app is running
    /// - Windows: Clicking taskbar icon while app is running
    /// - Linux: Clicking launcher icon while app is running
    ///
    /// This is commonly used to show the main window if it was hidden.
    ///
    /// # Arguments
    /// * `callback` - A closure that takes no arguments
    ///
    /// # Platform Support
    /// - **macOS**: Fully supported
    /// - **Windows**: Planned for future implementation
    /// - **Linux**: Planned for future implementation
    ///
    /// # Example
    /// ```no_run
    /// use wxdragon::prelude::*;
    ///
    /// wxdragon::main(|app| {
    ///     // Register a reopen handler
    ///     app.on_reopen_app(|| {
    ///         println!("App reopened");
    ///     });
    /// })
    /// .unwrap();
    /// ```
    fn on_reopen_app<F>(&self, callback: F)
    where
        F: Fn() + Send + 'static;

    /// Binds a handler for when files should be printed
    ///
    /// This callback is invoked when files are requested to be printed
    /// with your application.
    ///
    /// # Arguments
    /// * `callback` - A closure that receives a `Vec<String>` of file paths to print
    ///
    /// # Platform Support
    /// - **macOS**: Fully supported
    /// - **Windows**: Planned for future implementation
    /// - **Linux**: Planned for future implementation
    fn on_print_files<F>(&self, callback: F)
    where
        F: Fn(Vec<String>) + Send + 'static;
}
