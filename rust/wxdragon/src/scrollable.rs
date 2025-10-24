//! Generic scrolling trait for widgets that support scrolling functionality.
//!
//! This module provides the `WxScrollable` trait that can be implemented by any widget
//! that has scrolling capabilities, such as `RichTextCtrl`, multiline `TextCtrl`,
//! `ScrolledWindow`, `ListCtrl`, etc.

use crate::window::WxWidget;
use wxdragon_sys as ffi;

/// Trait for widgets that support scrolling functionality.
///
/// This trait provides common scrolling operations for widgets that inherit
/// scrolling capabilities from wxScrollHelper or have built-in scrolling support.
/// Examples include wxRichTextCtrl, wxTextCtrl (multiline), wxScrolledWindow, etc.
///
/// # Design Philosophy
///
/// This trait follows the same inheritance pattern as `WxWidget` - widgets that contain
/// a `Window` field and use `implement_widget_traits_with_target!` automatically
/// inherit all `WxWidget` methods through `Deref`. Similarly, widgets that implement
/// `WxScrollable` gain all scrolling functionality.
///
/// # Example
///
/// ```ignore
/// # use wxdragon::prelude::*;
/// # use wxdragon::scrollable::WxScrollable;
///
/// # let parent: Frame = Frame::builder().build();
///
/// // Any widget that implements WxScrollable gets these methods automatically
/// let rich_text = RichTextCtrl::builder(&parent).build();
///
/// // Scroll to end (useful for log displays)
/// rich_text.scroll_to_end();
///
/// // Check if a position is visible
/// if !rich_text.is_position_visible(1000) {
///     rich_text.show_position(1000);
/// }
///
/// // Auto-scroll as you add content
/// rich_text.append_text("New log entry\n");
/// rich_text.scroll_to_end(); // Keep latest content visible
/// ```
pub trait WxScrollable: WxWidget {
    /// Scrolls to show the specified position in the widget.
    ///
    /// This is a high-level method that ensures the given position is visible
    /// by scrolling the widget as needed. For text controls, the position
    /// typically refers to a character index.
    ///
    /// # Arguments
    /// * `position` - The text position or line number to scroll to
    ///
    /// # Example
    /// ```ignore
    /// // Scroll to character position 500
    /// text_ctrl.show_position(500);
    ///
    /// // Scroll to the end
    /// let last_pos = text_ctrl.get_last_position();
    /// text_ctrl.show_position(last_pos);
    /// ```
    fn show_position(&self, position: i64) {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe {
                ffi::wxd_Window_ShowPosition(handle, position);
            }
        }
    }

    /// Scrolls the widget to show the specified position with more control.
    ///
    /// This is a lower-level method that provides more control over the scrolling
    /// behavior than `show_position()`. The key code parameter can be used to
    /// simulate different types of scrolling operations.
    ///
    /// # Arguments  
    /// * `position` - The position to scroll to
    /// * `key_code` - Key code that triggered the scroll (or 0 for programmatic scroll)
    ///
    /// # Example
    /// ```ignore
    /// // Programmatic scroll (no key press)
    /// text_ctrl.scroll_into_view(position, 0);
    ///
    /// // Simulate pressing Page Down to scroll
    /// text_ctrl.scroll_into_view(position, wxWidgets::WXK_PAGEDOWN);
    /// ```
    fn scroll_into_view(&self, position: i64, key_code: i32) {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe {
                ffi::wxd_Window_ScrollIntoView(handle, position, key_code);
            }
        }
    }

    /// Checks if the specified position is currently visible in the widget.
    ///
    /// This is useful for implementing efficient scrolling - you can check if
    /// content is already visible before scrolling to it.
    ///
    /// # Arguments
    /// * `position` - The position to check
    ///
    /// # Returns
    /// `true` if the position is visible, `false` otherwise
    ///
    /// # Example
    /// ```ignore
    /// // Only scroll if the position isn't already visible
    /// if !text_ctrl.is_position_visible(target_position) {
    ///     text_ctrl.show_position(target_position);
    /// }
    /// ```
    fn is_position_visible(&self, position: i64) -> bool {
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe { ffi::wxd_Window_IsPositionVisible(handle, position) }
        } else {
            false
        }
    }

    /// Scrolls to the end of the content (useful for log displays).
    ///
    /// This is a convenience method that scrolls to the last position in the widget,
    /// making it ideal for auto-scrolling log viewers, chat applications, or any
    /// scenario where you want to show the most recent content.
    ///
    /// # Example
    /// ```ignore
    /// // Auto-scroll log viewer
    /// log_viewer.append_text("New log entry\n");
    /// log_viewer.scroll_to_end(); // Show the latest entry
    ///
    /// // Chat application
    /// chat_box.append_text("User: Hello\n");
    /// chat_box.scroll_to_end(); // Keep conversation visible
    /// ```
    fn scroll_to_end(&self) {
        // For text controls, we can get the last position and scroll there
        // This is a generic implementation that works for most scrollable widgets
        let handle = self.handle_ptr();
        if !handle.is_null() {
            unsafe {
                // Try to get the last position (this works for text controls)
                let last_pos = ffi::wxd_Window_GetLastPosition(handle);
                if last_pos > 0 {
                    ffi::wxd_Window_ShowPosition(handle, last_pos);
                }
            }
        }
    }

    /// Scrolls to the beginning of the content.
    ///
    /// This scrolls to position 0, showing the start of the content.
    ///
    /// # Example
    /// ```ignore
    /// // Jump to the top of a document
    /// document_viewer.scroll_to_beginning();
    ///
    /// // Reset log viewer to show first entries
    /// log_viewer.scroll_to_beginning();
    /// ```
    fn scroll_to_beginning(&self) {
        self.show_position(0);
    }

    /// Convenience method for auto-scrolling behavior.
    ///
    /// This checks if the widget is currently scrolled near the end, and if so,
    /// automatically scrolls to the new end after content is added. This is
    /// perfect for implementing "sticky" auto-scroll behavior in log viewers.
    ///
    /// # Arguments
    /// * `threshold` - How close to the end (in characters) to consider "near the end"
    ///
    /// # Returns
    /// `true` if auto-scroll was performed, `false` if the user had scrolled away
    ///
    /// # Example
    /// ```ignore
    /// // Before adding new content, check if we should auto-scroll
    /// let should_auto_scroll = log_viewer.auto_scroll_if_at_end(100);
    /// log_viewer.append_text("New log entry\n");
    /// // Auto-scroll happens automatically if user was near the end
    /// ```
    fn auto_scroll_if_at_end(&self, threshold: i64) -> bool {
        let handle = self.handle_ptr();
        if handle.is_null() {
            return false;
        }

        unsafe {
            let last_pos = ffi::wxd_Window_GetLastPosition(handle);
            if last_pos <= threshold {
                return false; // Not enough content to matter
            }

            // Check if we're near the end
            let near_end_pos = last_pos - threshold;
            if ffi::wxd_Window_IsPositionVisible(handle, near_end_pos) {
                // We're near the end, so auto-scroll to the actual end
                ffi::wxd_Window_ShowPosition(handle, last_pos);
                true
            } else {
                false // User has scrolled away, don't auto-scroll
            }
        }
    }
}
