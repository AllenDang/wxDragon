//! Virtual List Configuration System
//!
//! This module provides intelligent auto-management of virtual list parameters
//! based on viewport size and usage patterns. All performance parameters are
//! calculated automatically, eliminating the need for manual tuning.

// No additional imports needed for this config module

/// Internal parameters managed automatically by the virtual list
/// These are calculated based on viewport size, item dimensions, and usage patterns
#[derive(Debug, Clone)]
pub(crate) struct VirtualListInternalParams {
    // Layout & Sizing (auto-calculated)
    /// Estimated height for unmeasured items (calculated from measurements)
    pub estimated_item_height: i32,
    /// Estimated width for unmeasured items in horizontal layout (calculated from measurements)
    pub estimated_item_width: i32,
    /// Temporary height used during panel measurement for vertical layout
    pub temporary_panel_height: i32,
    /// Temporary width used during panel measurement for horizontal layout
    pub temporary_panel_width: i32,
    /// Actual scrollbar size (from config or system default)
    pub scrollbar_size: i32,

    // Performance Tuning (auto-calculated)
    /// Number of extra items to render beyond visible area
    pub buffer_size: usize,
    /// Threshold in pixels for width change detection
    pub width_change_threshold: i32,
    /// Early termination threshold for layout calculations
    pub early_termination_threshold: i32,
    /// Target size for the adaptive item pool
    pub pool_target_size: usize,
    /// Maximum number of items to cache measurements for
    pub max_measurement_cache_size: usize,

    // Interaction (auto-calculated or from config)
    /// Actual keyboard scroll amount (adaptive or from config)
    pub keyboard_scroll_amount: i32,
    /// Safety padding for end-of-list positioning (prevents last item clipping)
    pub safety_padding: i32,
    /// Tolerance in pixels for measurement stability detection
    pub measurement_tolerance: i32,
}

impl Default for VirtualListInternalParams {
    fn default() -> Self {
        Self {
            // Layout & Sizing
            estimated_item_height: 80,
            estimated_item_width: 200,
            temporary_panel_height: 100,
            temporary_panel_width: 1000, // Give plenty of width for natural text flow
            scrollbar_size: 16,          // System default

            // Performance Tuning
            buffer_size: 2,
            width_change_threshold: 5,
            early_termination_threshold: 200,
            pool_target_size: 10,
            max_measurement_cache_size: 1000,

            // Interaction
            keyboard_scroll_amount: 300,
            safety_padding: 20,       // Pixels of padding to ensure last item visibility
            measurement_tolerance: 5, // Pixel tolerance for measurement stability
        }
    }
}

impl VirtualListInternalParams {
    /// Create new internal parameters optimized for the given viewport and user preferences
    pub fn new_for_viewport(
        viewport_size: (i32, i32), // (width, height)
        scrollbar_size: Option<i32>,
        keyboard_scroll_amount: Option<i32>,
        mouse_wheel_multiplier: f32,
    ) -> Self {
        let mut params = Self::default();
        params.auto_configure(viewport_size, scrollbar_size, keyboard_scroll_amount, mouse_wheel_multiplier);
        params
    }

    /// Automatically configure all parameters based on viewport size and user preferences
    pub fn auto_configure(
        &mut self,
        viewport_size: (i32, i32),
        scrollbar_size: Option<i32>,
        keyboard_scroll_amount: Option<i32>,
        _mouse_wheel_multiplier: f32,
    ) {
        let (viewport_width, viewport_height) = viewport_size;

        // Calculate layout parameters
        self.scrollbar_size = scrollbar_size.unwrap_or(16);

        // Calculate performance parameters based on viewport
        let visible_items_estimate = (viewport_height / self.estimated_item_height).max(1);

        // Buffer size: scale with viewport size, more items visible = larger buffer helpful
        self.buffer_size = match visible_items_estimate {
            1..=5 => 1,   // Small viewport, minimal buffer
            6..=15 => 2,  // Medium viewport, standard buffer
            16..=30 => 3, // Large viewport, larger buffer
            _ => 4,       // Very large viewport, maximum buffer
        };

        // Pool size: enough for visible items + buffer + some extra for smooth scrolling
        self.pool_target_size = ((visible_items_estimate as usize) + self.buffer_size * 2 + 5).min(50);

        // Cache size: scale with expected usage, larger viewports need more cache
        self.max_measurement_cache_size = ((visible_items_estimate as usize) * 10).clamp(100, 2000);

        // Width change threshold: smaller for narrow viewports (more sensitive to changes)
        self.width_change_threshold = match viewport_width {
            ..=400 => 3,     // Very narrow, very sensitive
            401..=800 => 5,  // Normal, standard sensitivity
            801..=1200 => 8, // Wide, less sensitive
            _ => 10,         // Very wide, least sensitive
        };

        // Early termination: scale with viewport height
        self.early_termination_threshold = viewport_height / 2;

        // Keyboard scroll: adaptive or from config
        self.keyboard_scroll_amount = keyboard_scroll_amount.unwrap_or_else(|| {
            // Adaptive: scroll about 1/3 of viewport height
            (viewport_height / 3).clamp(50, 500)
        });
    }

    // Adaptive methods removed - the auto-configuration in auto_configure()
    // proved sufficient without needing dynamic runtime adjustments.
    // The static configuration based on viewport size works effectively.
}
