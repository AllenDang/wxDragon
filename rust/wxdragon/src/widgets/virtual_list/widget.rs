use crate::prelude::*;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;

use super::{
    adaptive_pool::AdaptiveItemPool,
    error::{IntoVirtualListError, VirtualListError, VirtualListResult},
    item_context::ItemContext,
};

// Global registry for panel-to-context mapping removed
// TASK 3.1: Global registry removed - replaced with instance-owned registry for memory safety
// PHASE 3: ItemContext moved to item_context.rs module for better code organization

/// Trait for providing data to the virtual list
pub trait VirtualListDataSource: Send + Sync {
    /// Get the total number of items
    fn get_item_count(&self) -> usize;

    /// Get the data for a specific item
    fn get_item_data(&self, index: usize) -> Box<dyn Any + Send + Sync>;
}

/// Trait for rendering items in the virtual list
pub trait VirtualListItemRenderer: Send + Sync {
    /// Create a new panel for rendering an item
    fn create_item(&self, parent: &Panel) -> Panel;

    /// Update the content of a panel for a specific item with its data
    fn update_item(&self, panel: &Panel, index: usize, data: &dyn Any);
}

/// Layout modes for the virtual list
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VirtualListLayoutMode {
    /// Items arranged vertically (rows), each item stretches to container width
    Vertical,
    /// Items arranged horizontally (columns), each item stretches to container height
    Horizontal,
}

/// Item sizing behavior for cache invalidation strategy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemSizingMode {
    /// Items have fixed size regardless of container width (images, fixed text, etc.)
    /// Cache entries are preserved when width changes for better performance
    FixedSize,
    /// Items resize based on container width (text wrapping, responsive layouts, etc.)
    /// Cache entries are invalidated when width changes to ensure correct layout
    DynamicSize,
}

/// PHASE 1.2: Measurement confidence system for layout stability
#[derive(Debug, Clone, Copy)]
enum MeasurementState {
    /// Actually measured but not yet validated for stability
    Measured {
        value: i32,
        validated: bool, // Has this measurement been confirmed across multiple cycles?
    },
    /// Measurement verified as stable across multiple measurement cycles
    Verified {
        value: i32,
        stable_count: u8, // How many times this measurement has been confirmed
    },
}

impl MeasurementState {
    /// Upgrade measurement state with new information
    fn upgrade_with_measurement(self, new_value: i32, tolerance: i32) -> Self {
        match self {
            MeasurementState::Measured {
                value: old_value,
                validated,
            } => {
                let is_stable = (old_value - new_value).abs() <= tolerance;
                if is_stable && validated {
                    // Promote to verified
                    MeasurementState::Verified {
                        value: new_value,
                        stable_count: 1,
                    }
                } else if is_stable {
                    // Mark as validated
                    MeasurementState::Measured {
                        value: new_value,
                        validated: true,
                    }
                } else {
                    // New measurement differs significantly - start over
                    MeasurementState::Measured {
                        value: new_value,
                        validated: false,
                    }
                }
            }
            MeasurementState::Verified {
                value: old_value,
                stable_count,
            } => {
                let is_stable = (old_value - new_value).abs() <= tolerance;
                if is_stable {
                    // Increase stability count
                    MeasurementState::Verified {
                        value: new_value,
                        stable_count: (stable_count + 1).min(10), // Cap at 10
                    }
                } else {
                    // Measurement changed significantly - demote to measured
                    MeasurementState::Measured {
                        value: new_value,
                        validated: false,
                    }
                }
            }
        }
    }
}

/// Enhanced cache entry with measurement confidence system
#[derive(Debug, Clone, Copy)]
struct EnhancedCacheEntry {
    size: Size,
    measurement_state: MeasurementState,
    measured_at_dimension: i32,
}

/// PHASE 1.3: LRU-managed size cache with automatic eviction
#[derive(Debug, Clone)]
struct SizeCache {
    /// Cache entries keyed by item index
    entries: HashMap<usize, EnhancedCacheEntry>,
    /// LRU access order: most recently used at back, least recently used at front
    access_order: std::collections::VecDeque<usize>,
    /// Maximum number of entries to keep in cache
    max_size: usize,
    /// Current generation for cleanup purposes
    generation: u64,
}

impl SizeCache {
    /// Create a new size cache with specified maximum size
    fn new(max_size: usize) -> Self {
        Self {
            entries: HashMap::new(),
            access_order: std::collections::VecDeque::new(),
            max_size,
            generation: 0,
        }
    }

    /// Insert or update an entry, handling LRU eviction automatically
    fn insert(&mut self, index: usize, entry: EnhancedCacheEntry) {
        // If entry exists, remove it from access order first
        if self.entries.contains_key(&index) {
            self.access_order.retain(|&x| x != index);
        }

        // Insert/update the entry
        self.entries.insert(index, entry);
        self.access_order.push_back(index);

        // Evict oldest entries if we exceed max size
        while self.entries.len() > self.max_size {
            self.evict_oldest();
        }
    }

    /// Get an entry and mark it as recently used
    fn get(&mut self, index: usize) -> Option<&EnhancedCacheEntry> {
        if self.entries.contains_key(&index) {
            // Move to end of access order (most recently used)
            self.access_order.retain(|&x| x != index);
            self.access_order.push_back(index);
            self.entries.get(&index)
        } else {
            None
        }
    }

    /// Get an entry without updating access order (for read-only operations)
    fn peek(&self, index: usize) -> Option<&EnhancedCacheEntry> {
        self.entries.get(&index)
    }

    /// Remove specific entry
    fn remove(&mut self, index: usize) -> Option<EnhancedCacheEntry> {
        if let Some(entry) = self.entries.remove(&index) {
            self.access_order.retain(|&x| x != index);
            Some(entry)
        } else {
            None
        }
    }

    /// Evict the least recently used entry
    fn evict_oldest(&mut self) {
        if let Some(oldest_index) = self.access_order.pop_front() {
            self.entries.remove(&oldest_index);
        }
    }

    /// Clear all entries
    fn clear(&mut self) {
        self.entries.clear();
        self.access_order.clear();
        self.generation += 1;
    }

    /// Get number of cached entries
    fn len(&self) -> usize {
        self.entries.len()
    }

    /// Clean stale entries based on generation threshold
    fn clean_stale_entries(&mut self, max_generations_old: u64) {
        // For now, we'll use generation-based cleanup
        // In the future, this could be enhanced with timestamp-based cleanup
        if self.generation > max_generations_old {
            let cutoff_generation = self.generation - max_generations_old;
            // For simplicity, we'll implement a basic cleanup strategy
            // This could be enhanced to track per-entry generations if needed
            if cutoff_generation > 10 {
                // Clear cache if it's very stale
                self.clear();
            }
        }
    }
}

/// PHASE 2.3: Batched Layout Operations System
/// Reduces layout thrashing by collecting and executing panel operations in batches
struct PanelCreateOperation {
    index: usize,
    panel: Panel,
    data: Box<dyn Any + Send + Sync>,
}

struct PanelUpdateOperation {
    index: usize,
    panel: Panel,
    data: Box<dyn Any + Send + Sync>,
    new_size: Size,
}

struct PanelPositionOperation {
    panel: Panel,
    position: Point,
    final_size: Size,
}

struct PanelDestroyOperation {
    panel: Panel,
}

/// Batch collector for all panel operations to minimize layout thrashing
struct LayoutBatch {
    /// Panels to create with initial setup
    panels_to_create: Vec<PanelCreateOperation>,
    /// Panels to update content and/or size
    panels_to_update: Vec<PanelUpdateOperation>,
    /// Panels to position and show
    panels_to_position: Vec<PanelPositionOperation>,
    /// Panels to hide and return to pool
    panels_to_destroy: Vec<PanelDestroyOperation>,
    /// Whether to defer final layout until batch execution
    defer_layout: bool,
}

impl LayoutBatch {
    fn new() -> Self {
        Self {
            panels_to_create: Vec::new(),
            panels_to_update: Vec::new(),
            panels_to_position: Vec::new(),
            panels_to_destroy: Vec::new(),
            defer_layout: true,
        }
    }

    /// Add a panel creation operation
    fn add_create_operation(
        &mut self,
        index: usize,
        panel: Panel,
        data: Box<dyn Any + Send + Sync>,
    ) {
        self.panels_to_create
            .push(PanelCreateOperation { index, panel, data });
    }

    /// Add a panel content/size update operation
    fn add_update_operation(
        &mut self,
        index: usize,
        panel: Panel,
        data: Box<dyn Any + Send + Sync>,
        new_size: Size,
    ) {
        self.panels_to_update.push(PanelUpdateOperation {
            index,
            panel,
            data,
            new_size,
        });
    }

    /// Add a panel positioning operation
    fn add_position_operation(&mut self, panel: Panel, position: Point, final_size: Size) {
        self.panels_to_position.push(PanelPositionOperation {
            panel,
            position,
            final_size,
        });
    }

    /// Add a panel destruction operation
    fn add_destroy_operation(&mut self, panel: Panel) {
        self.panels_to_destroy.push(PanelDestroyOperation { panel });
    }

    /// Execute all batched operations efficiently to minimize layout thrashing
    fn execute_batch(
        &mut self,
        item_renderer: &dyn VirtualListItemRenderer,
        state: &mut VirtualListState,
    ) -> BatchExecutionResult {
        let mut created_panels = Vec::new();
        let mut updated_measurements = Vec::new();

        // PHASE 1: Create and setup new panels (no layout yet)
        for create_op in &self.panels_to_create {
            // CRITICAL FIX: Set correct initial size based on layout mode
            let initial_size = match state.layout_mode {
                VirtualListLayoutMode::Vertical => Size::new(
                    state.viewport_size.width,
                    state.internal_params.temporary_panel_height,
                ),
                VirtualListLayoutMode::Horizontal => Size::new(
                    2000, // Give plenty of width for natural text flow measurement
                    state.viewport_size.height,
                ),
            };
            create_op.panel.set_size(initial_size);

            // CRITICAL FIX: Hide panel until properly positioned
            create_op.panel.show(false);

            // Update content
            item_renderer.update_item(&create_op.panel, create_op.index, create_op.data.as_ref());

            // Store context
            state.store_item_context(&create_op.panel, create_op.index, create_op.data.as_ref());

            created_panels.push((create_op.index, create_op.panel.clone()));
        }

        // PHASE 2: Update existing panels (no layout yet)
        for update_op in &self.panels_to_update {
            // Update size first
            update_op.panel.set_size(update_op.new_size);

            // Update content
            item_renderer.update_item(&update_op.panel, update_op.index, update_op.data.as_ref());

            updated_measurements.push((update_op.index, update_op.panel.clone()));
        }

        // PHASE 3: Single layout pass for all modified panels
        if self.defer_layout {
            // Batch layout all created panels
            for (_, panel) in &created_panels {
                panel.layout();
            }

            // Batch layout all updated panels
            for (_, panel) in &updated_measurements {
                panel.layout();
            }
        }

        // PHASE 4: Measure all panels and collect final sizes
        let mut final_measurements = Vec::new();

        for (index, panel) in &created_panels {
            let measured_size = panel.get_best_size();
            final_measurements.push((*index, measured_size));
        }

        for (index, panel) in &updated_measurements {
            let measured_size = panel.get_best_size();
            final_measurements.push((*index, measured_size));
        }

        // PHASE 5: Apply final sizes and positions in one batch
        for position_op in &self.panels_to_position {
            // Apply final size
            position_op.panel.set_size(position_op.final_size);

            // Apply position
            position_op
                .panel
                .move_window(position_op.position.x, position_op.position.y);

            // Show panel
            position_op.panel.show(true);
        }

        // PHASE 6: Hide and cleanup destroyed panels
        for destroy_op in &self.panels_to_destroy {
            destroy_op.panel.show(false);
            state.remove_panel_context(&destroy_op.panel);
        }

        // Clear batch for next use
        self.panels_to_create.clear();
        self.panels_to_update.clear();
        self.panels_to_position.clear();
        self.panels_to_destroy.clear();

        BatchExecutionResult { final_measurements }
    }
}

/// Result of executing a layout batch
struct BatchExecutionResult {
    /// Final measurements for all processed panels (index, measured_size)
    final_measurements: Vec<(usize, Size)>,
}

/// Context for batch execution operations to reduce parameter count
struct BatchExecutionContext<'a> {
    items_to_show: &'a [(usize, i32, i32)],
    new_visible_items: &'a HashSet<usize>,
    current_visible_items: &'a HashSet<usize>,
    data_source: &'a Rc<dyn VirtualListDataSource>,
    item_renderer: &'a Rc<dyn VirtualListItemRenderer>,
    parent: &'a Panel,
    scroll_position: i32,
    current_viewport_dimension: i32,
    dimension_changed: bool,
}

/// Internal state for the virtual list
#[derive(Clone)]
pub struct VirtualListState {
    // Configuration
    layout_mode: VirtualListLayoutMode,
    internal_params: super::config::VirtualListInternalParams,

    // Content sources
    data_source: Option<Rc<dyn VirtualListDataSource>>,
    item_renderer: Option<Rc<dyn VirtualListItemRenderer>>,

    // Pooling and layout
    item_pool: AdaptiveItemPool,

    // Currently visible items (index -> panel)
    item_to_panel: HashMap<usize, Panel>,

    // PHASE 1.2: Enhanced size caching with measurement confidence system
    item_size_cache: SizeCache,
    item_sizing_mode: ItemSizingMode,
    cache_generation: u64,

    // TASK 2.4: Measurement deduplication tracking
    current_update_cycle: u64,
    measured_in_current_cycle: HashSet<usize>,

    // Viewport tracking
    viewport_size: Size,
    scroll_position: Point,
    total_content_size: Size,
    visible_range: std::ops::Range<usize>,

    // Track viewport width changes to force content re-layout when needed
    previous_viewport_width: i32,

    // TASK 3.1: Owned panel context registry (replaces global static)
    panel_context_registry: HashMap<i32, ItemContext>,
}

impl std::fmt::Debug for VirtualListState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VirtualListState")
            .field("layout_mode", &self.layout_mode)
            .field("data_source", &self.data_source.is_some())
            .field("item_renderer", &self.item_renderer.is_some())
            .field("item_pool", &self.item_pool)
            .field(
                "item_to_panel",
                &format!("{} items", self.item_to_panel.len()),
            )
            .field(
                "item_size_cache",
                &format!("{} cached sizes", self.item_size_cache.len()),
            )
            .field("cache_generation", &self.cache_generation)
            .field("viewport_size", &self.viewport_size)
            .field("scroll_position", &self.scroll_position)
            .field("total_content_size", &self.total_content_size)
            .field("visible_range", &self.visible_range)
            .finish()
    }
}

impl VirtualListState {
    /// Store or update a measurement with progressive enhancement
    fn store_progressive_measurement(
        &mut self,
        index: usize,
        measured_size: Size,
        current_dimension: i32,
    ) {
        let size_value = match self.layout_mode {
            VirtualListLayoutMode::Vertical => measured_size.height,
            VirtualListLayoutMode::Horizontal => measured_size.width,
        };

        let tolerance = self.internal_params.measurement_tolerance;

        let measurement_state = if let Some(existing_entry) = self.item_size_cache.peek(index) {
            // Upgrade existing measurement
            existing_entry
                .measurement_state
                .upgrade_with_measurement(size_value, tolerance)
        } else {
            // New measurement - start as unvalidated
            MeasurementState::Measured {
                value: size_value,
                validated: false,
            }
        };

        let enhanced_entry = EnhancedCacheEntry {
            size: measured_size,
            measurement_state,
            measured_at_dimension: current_dimension,
        };

        self.item_size_cache.insert(index, enhanced_entry);
    }

    fn new(
        layout_mode: VirtualListLayoutMode,
        scrollbar_size: Option<i32>,
        keyboard_scroll_amount: Option<i32>,
        mouse_wheel_multiplier: f32,
    ) -> Self {
        // Create intelligent internal params that auto-configure based on default viewport
        let internal_params = super::config::VirtualListInternalParams::new_for_viewport(
            (800, 600), // Default viewport for initialization
            scrollbar_size,
            keyboard_scroll_amount,
            mouse_wheel_multiplier,
        );

        Self {
            layout_mode,
            internal_params: internal_params.clone(),
            data_source: None,
            item_renderer: None,
            item_pool: AdaptiveItemPool::new(layout_mode, &internal_params),
            item_to_panel: HashMap::new(),
            item_size_cache: SizeCache::new(100),
            item_sizing_mode: ItemSizingMode::DynamicSize, // Default to dynamic sizing for safety
            cache_generation: 0,

            // TASK 2.4: Initialize measurement deduplication tracking
            current_update_cycle: 0,
            measured_in_current_cycle: HashSet::new(),

            viewport_size: Size::new(0, 0),
            scroll_position: Point::new(0, 0),
            total_content_size: Size::new(0, 0),
            visible_range: 0..0,
            previous_viewport_width: 0,

            // TASK 3.1: Initialize owned panel context registry
            panel_context_registry: HashMap::new(),
        }
    }

    fn set_data_source(&mut self, data_source: Rc<dyn VirtualListDataSource>) {
        self.data_source = Some(data_source);
        // Clear entire cache since data source changed completely
        self.item_size_cache.clear();
    }

    fn set_item_renderer(&mut self, item_renderer: Rc<dyn VirtualListItemRenderer>) {
        self.item_renderer = Some(item_renderer);
        // Clear entire cache since item renderer changed completely
        self.item_size_cache.clear();
    }

    fn invalidate_layout(&mut self) {
        // Clear current visible items
        self.hide_all_items();

        // Intelligently invalidate cache based on item sizing mode
        self.selective_cache_invalidation();

        // Total content size will be recalculated when items are actually rendered
        // in update_visible_items where we have access to the parent panel
    }

    fn hide_all_items(&mut self) {
        // Return all panels to the pool and clean up their context
        let panels_to_clean: Vec<Panel> =
            self.item_to_panel.drain().map(|(_, panel)| panel).collect();

        for panel in panels_to_clean {
            self.remove_panel_context(&panel);
            self.item_pool.return_item(panel);
        }
        self.visible_range = 0..0;
    }

    /// PHASE 1.1 REFACTORED: Main update method with focused concerns
    /// PHASE 2.3 ENHANCED: Now uses batched layout operations to reduce layout thrashing
    /// Complexity reduced from 300+ lines to ~80 lines using focused helper methods + batching
    fn update_visible_items(&mut self, parent: &Panel) {
        // Early return if we don't have both data source and renderer
        let (data_source, item_renderer) =
            if let (Some(ref ds), Some(ref ir)) = (&self.data_source, &self.item_renderer) {
                (ds.clone(), ir.clone())
            } else {
                return;
            };

        // Update current cycle for measurement deduplication
        self.current_update_cycle = self.current_update_cycle.wrapping_add(1);
        self.measured_in_current_cycle.clear();

        let total_items = data_source.get_item_count();
        if total_items == 0 {
            self.hide_all_items();
            return;
        }

        // Detect dimension changes for cache invalidation strategy
        let current_viewport_dimension = match self.layout_mode {
            VirtualListLayoutMode::Vertical => self.viewport_size.width,
            VirtualListLayoutMode::Horizontal => self.viewport_size.height,
        };
        let dimension_changed = current_viewport_dimension != self.previous_viewport_width;

        if dimension_changed {
            self.previous_viewport_width = current_viewport_dimension;
            // Apply intelligent cache invalidation strategy
            self.selective_cache_invalidation();
        }

        // STEP 1: Calculate which items should be visible
        let items_to_show =
            self.extract_visible_range_calculation(&data_source, &item_renderer, parent);

        // STEP 2: PHASE 2.3 - Execute all operations using batched layout system
        let new_visible_items: HashSet<usize> =
            items_to_show.iter().map(|(idx, _, _)| *idx).collect();
        let current_visible_items: HashSet<usize> = self.item_to_panel.keys().cloned().collect();

        // Only update if the set of visible items has changed OR if dimensions changed
        if current_visible_items != new_visible_items || dimension_changed {
            let scroll_position = match self.layout_mode {
                VirtualListLayoutMode::Vertical => self.scroll_position.y,
                VirtualListLayoutMode::Horizontal => self.scroll_position.x,
            };

            // PHASE 2.3: Execute all operations using batched layout system
            let context = BatchExecutionContext {
                items_to_show: &items_to_show,
                new_visible_items: &new_visible_items,
                current_visible_items: &current_visible_items,
                data_source: &data_source,
                item_renderer: &item_renderer,
                parent,
                scroll_position,
                current_viewport_dimension,
                dimension_changed,
            };
            self.execute_batched_layout_operations(context);

            // CRITICAL FIX: Update scrollbar state AFTER measurements are stored in cache
            let estimated_item_size = match self.layout_mode {
                VirtualListLayoutMode::Vertical => self.internal_params.estimated_item_height,
                VirtualListLayoutMode::Horizontal => self.internal_params.estimated_item_width,
            };
            self.update_scrollbar_state(total_items, estimated_item_size);
        }
    }

    /// Measure the actual size of an item by creating, updating, and laying out a temporary panel
    /// PHASE 2B: Enhanced measurement with deduplication tracking
    fn measure_item_size(
        &mut self,
        index: usize,
        data_source: &dyn VirtualListDataSource,
        item_renderer: &dyn VirtualListItemRenderer,
        parent: &Panel,
    ) -> Size {
        // TASK 2.4: Check if already measured in current cycle
        if self.measured_in_current_cycle.contains(&index) {
            // Already measured in this cycle - check cache
            if let Some(cached_size) = self.item_size_cache.peek(index) {
                return cached_size.size;
            }
        }

        // Check cache first (normal cache check)
        if let Some(cached_size) = self.item_size_cache.get(index) {
            // Mark as measured in current cycle even for cache hits
            self.measured_in_current_cycle.insert(index);
            return cached_size.size;
        }

        // Mark as measured in current cycle before performing expensive measurement
        self.measured_in_current_cycle.insert(index);

        // Create temporary panel for measurement
        let temp_panel = item_renderer.create_item(parent);

        // CRITICAL: Set proper size BEFORE updating content for accurate measurement
        let temp_size = match self.layout_mode {
            VirtualListLayoutMode::Vertical => Size::new(self.viewport_size.width, 100), // Fixed width, temporary height
            VirtualListLayoutMode::Horizontal => Size::new(
                self.internal_params.temporary_panel_width,
                self.viewport_size.height,
            ), // Temporary width, fixed height
        };
        temp_panel.set_size(temp_size);

        let item_data = data_source.get_item_data(index);
        item_renderer.update_item(&temp_panel, index, item_data.as_ref());

        // CRITICAL: Force layout so sizers can do their work
        temp_panel.layout();
        let measured_size = temp_panel.get_best_size();

        // Hide temp panel (cleanup)
        temp_panel.show(false);

        // Cache the result with layout-aware dimensions
        let cached_size = match self.layout_mode {
            VirtualListLayoutMode::Vertical => {
                Size::new(self.viewport_size.width, measured_size.height)
            }
            VirtualListLayoutMode::Horizontal => {
                Size::new(measured_size.width, self.viewport_size.height)
            }
        };

        let current_dimension = match self.layout_mode {
            VirtualListLayoutMode::Vertical => self.viewport_size.width,
            VirtualListLayoutMode::Horizontal => self.viewport_size.height,
        };

        // PHASE 1.2: Use progressive measurement system
        self.store_progressive_measurement(index, cached_size, current_dimension);

        cached_size
    }

    /// PHASE 2B: Measure an item that is about to become visible with deduplication  
    fn measure_item_size_for_visible(
        &mut self,
        index: usize,
        data_source: &dyn VirtualListDataSource,
        item_renderer: &dyn VirtualListItemRenderer,
        parent: &Panel,
    ) -> Size {
        // TASK 2.4: Check if already measured in current cycle
        if self.measured_in_current_cycle.contains(&index) {
            // Already measured in this cycle - return cached result
            if let Some(cached_size) = self.item_size_cache.peek(index) {
                return cached_size.size;
            }
        }

        // Mark as measured in current cycle before measuring
        self.measured_in_current_cycle.insert(index);

        // Perform measurement (same as measure_item_size but with deduplication tracking)
        self.measure_item_size(index, data_source, item_renderer, parent)
    }

    /// Calculate total content size using progressive measurement approach
    fn calculate_total_content_size_progressive(
        &self,
        total_items: usize,
        estimated_item_size: i32,
    ) -> Size {
        let total_size = match self.item_sizing_mode {
            ItemSizingMode::FixedSize => {
                // For fixed size items, use pure estimation without measurements
                // This ensures consistent size calculation regardless of which items have been measured
                (total_items as i32) * estimated_item_size
            }
            ItemSizingMode::DynamicSize => {
                // For dynamic size items, use mix of measurements and estimates
                let mut size = 0;
                for index in 0..total_items {
                    let item_size = if let Some(cached_size) = self.item_size_cache.peek(index) {
                        // Use actual measurement if we have it
                        match self.layout_mode {
                            VirtualListLayoutMode::Vertical => cached_size.size.height,
                            VirtualListLayoutMode::Horizontal => cached_size.size.width,
                        }
                    } else {
                        // Use estimate for non-measured items
                        estimated_item_size
                    };
                    size += item_size;
                }
                size
            }
        };

        match self.layout_mode {
            VirtualListLayoutMode::Vertical => Size::new(self.viewport_size.width, total_size),
            VirtualListLayoutMode::Horizontal => Size::new(total_size, self.viewport_size.height),
        }
    }

    // ===============================================
    // PHASE 1.1 REFACTORING: Focused Update Methods
    // ===============================================

    /// Calculate which items should be visible based on scroll position and viewport
    fn extract_visible_range_calculation(
        &mut self,
        data_source: &Rc<dyn VirtualListDataSource>,
        item_renderer: &Rc<dyn VirtualListItemRenderer>,
        parent: &Panel,
    ) -> Vec<(usize, i32, i32)> {
        let total_items = data_source.get_item_count();

        // Layout mode specific variables
        let (scroll_position, viewport_length, estimated_item_size) = match self.layout_mode {
            VirtualListLayoutMode::Vertical => (
                self.scroll_position.y,
                self.viewport_size.height,
                self.internal_params.estimated_item_height,
            ),
            VirtualListLayoutMode::Horizontal => (
                self.scroll_position.x,
                self.viewport_size.width,
                self.internal_params.estimated_item_width,
            ),
        };

        // CRITICAL FIX: Two-pass approach for consistent position calculation
        // PASS 1: Determine actual visible items without creating measurement inconsistencies
        let mut current_position = 0;
        let mut potentially_visible_items: Vec<usize> = Vec::new();

        // Use cache-first approach: prioritize existing measurements for consistency
        for index in 0..total_items {
            let item_size = if let Some(cached_size) = self.item_size_cache.peek(index) {
                // Use cached measurement - most consistent
                match self.layout_mode {
                    VirtualListLayoutMode::Vertical => cached_size.size.height,
                    VirtualListLayoutMode::Horizontal => cached_size.size.width,
                }
            } else {
                // Use estimate for all uncached items to avoid position shifts
                estimated_item_size
            };

            // Check if this item is potentially visible
            let item_start = current_position;
            let item_end = current_position + item_size;

            if item_end > scroll_position && item_start < scroll_position + viewport_length {
                potentially_visible_items.push(index);
            }

            current_position += item_size;

            // Early termination: stop well past the visible area
            // CRITICAL FIX: Don't early terminate when near the end of content to ensure last items are considered
            let is_near_end = index >= total_items.saturating_sub(5); // Always process last 5 items
            if !is_near_end
                && current_position
                    > scroll_position
                        + viewport_length
                        + self.internal_params.early_termination_threshold
            {
                break;
            }
        }

        // PASS 2: Measure only the confirmed visible items to avoid position inconsistencies
        let mut current_position = 0;
        let mut items_to_show: Vec<(usize, i32, i32)> = Vec::new();

        for index in 0..total_items {
            let item_size = if let Some(cached_size) = self.item_size_cache.peek(index) {
                // Use cached measurement
                match self.layout_mode {
                    VirtualListLayoutMode::Vertical => cached_size.size.height,
                    VirtualListLayoutMode::Horizontal => cached_size.size.width,
                }
            } else if potentially_visible_items.contains(&index) {
                // Only measure items that are confirmed to be visible
                let measured_size = self.measure_item_size_for_visible(
                    index,
                    data_source.as_ref(),
                    item_renderer.as_ref(),
                    parent,
                );
                match self.layout_mode {
                    VirtualListLayoutMode::Vertical => measured_size.height,
                    VirtualListLayoutMode::Horizontal => measured_size.width,
                }
            } else {
                // Use estimate for all other items
                estimated_item_size
            };

            // Check if this item is visible
            let item_start = current_position;
            let item_end = current_position + item_size;

            if item_end > scroll_position && item_start < scroll_position + viewport_length {
                items_to_show.push((index, item_start, item_size));
            }

            current_position += item_size;

            // Early termination: stop well past the visible area
            // CRITICAL FIX: Don't early terminate when near the end of content to ensure last items are considered
            let is_near_end = index >= total_items.saturating_sub(5); // Always process last 5 items
            if !is_near_end
                && current_position
                    > scroll_position
                        + viewport_length
                        + self.internal_params.early_termination_threshold
            {
                break;
            }
        }

        items_to_show
    }

    /// Update total content size and scrollbar state
    fn update_scrollbar_state(&mut self, total_items: usize, estimated_item_size: i32) {
        // Update total content size using mix of actual + estimated
        self.total_content_size =
            self.calculate_total_content_size_progressive(total_items, estimated_item_size);

        // TODO: Add actual scrollbar updates here in future (Phase 2.2)
        // For now, the content size update is sufficient
    }

    /// PHASE 2 OPTIMIZATION: Truly Selective Cache Invalidation
    /// Intelligently invalidate only cache entries that are actually affected by viewport changes
    fn selective_cache_invalidation(&mut self) {
        let current_dimension = match self.layout_mode {
            VirtualListLayoutMode::Vertical => self.viewport_size.width,
            VirtualListLayoutMode::Horizontal => self.viewport_size.height,
        };

        let previous_dimension = self.previous_viewport_width;
        let dimension_change = current_dimension - previous_dimension;

        // If dimension change is tiny, preserve all cache entries
        if dimension_change.abs() < self.internal_params.width_change_threshold {
            return;
        }

        match self.item_sizing_mode {
            ItemSizingMode::DynamicSize => {
                // For dynamic items, use intelligent selective invalidation
                self.selective_dynamic_cache_invalidation(current_dimension, previous_dimension);
            }
            ItemSizingMode::FixedSize => {
                // For fixed size items, only invalidate if layout mode changed or major viewport change
                self.selective_fixed_cache_invalidation(dimension_change);
            }
        }

        self.cache_generation = self.cache_generation.wrapping_add(1);
    }

    /// Selective invalidation for dynamic size items
    fn selective_dynamic_cache_invalidation(
        &mut self,
        current_dimension: i32,
        previous_dimension: i32,
    ) {
        let dimension_change = current_dimension - previous_dimension;
        let change_ratio = dimension_change.abs() as f64 / previous_dimension.max(1) as f64;

        // Strategy based on magnitude of change
        if change_ratio > 0.3 {
            // Major change (>30%) - invalidate all to avoid layout issues
            self.item_size_cache.clear();
        } else if change_ratio > 0.1 {
            // Moderate change (>10%) - invalidate cached items that were measured at very different widths
            self.invalidate_dimension_sensitive_items(current_dimension);
        } else {
            // Minor change (<10%) - only invalidate items that might wrap differently
            self.invalidate_potentially_affected_items(current_dimension, previous_dimension);
        }
    }

    /// Selective invalidation for fixed size items
    fn selective_fixed_cache_invalidation(&mut self, dimension_change: i32) {
        // For fixed size items, very conservative invalidation
        if dimension_change.abs() > 100 {
            // Only invalidate if change is substantial (>100px)
            // and only invalidate very old cache entries that might be stale
            let current_generation = self.cache_generation;

            // Keep cache but mark as potentially stale for future validation
            // In a more sophisticated implementation, we'd track cache entry ages
            if current_generation % 10 == 0 {
                // Every 10 generations, use LRU cache's built-in cleanup
                self.item_size_cache.clean_stale_entries(10);
            }
        }
        // Otherwise preserve all cache entries for maximum performance
    }

    /// Invalidate items that were cached at significantly different dimensions
    fn invalidate_dimension_sensitive_items(&mut self, current_dimension: i32) {
        let threshold = current_dimension as f64 * 0.15; // 15% difference threshold

        let items_to_invalidate: Vec<usize> = self
            .item_size_cache
            .entries
            .iter()
            .filter_map(|(&index, cached_entry)| {
                // Use the stored measured_at_dimension for more accurate comparison
                let dimension_diff =
                    (cached_entry.measured_at_dimension - current_dimension).abs() as f64;
                if dimension_diff > threshold {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        for index in items_to_invalidate {
            self.item_size_cache.remove(index);
        }
    }

    /// Invalidate only items that are likely to be affected by small dimension changes
    fn invalidate_potentially_affected_items(
        &mut self,
        current_dimension: i32,
        previous_dimension: i32,
    ) {
        // For small changes, use heuristics to identify items that might reflow

        // Invalidate items near text wrapping boundaries
        let critical_widths = [300, 400, 500, 600, 800, 1000, 1200]; // Common breakpoints

        let crossed_boundary = critical_widths.iter().any(|&width| {
            (previous_dimension <= width && current_dimension > width)
                || (previous_dimension > width && current_dimension <= width)
        });

        if crossed_boundary {
            // Only invalidate items that might be affected by text wrapping
            // Priority: recently visible items and items near current scroll position
            let visible_items: std::collections::HashSet<usize> =
                self.item_to_panel.keys().copied().collect();

            // Calculate scroll-based priority range
            let estimated_item_height = self.internal_params.estimated_item_height;
            let current_scroll_item = if estimated_item_height > 0 {
                (self.scroll_position.y / estimated_item_height) as usize
            } else {
                0
            };

            let priority_range =
                current_scroll_item.saturating_sub(50)..=(current_scroll_item + 100);

            let items_to_invalidate: Vec<usize> = self
                .item_size_cache
                .entries
                .keys()
                .filter(|&&index| {
                    // Invalidate if recently visible or in priority scroll range
                    visible_items.contains(&index) || priority_range.contains(&index)
                })
                .copied()
                .collect();

            for index in items_to_invalidate {
                self.item_size_cache.remove(index);
            }
        }
        // Otherwise, preserve all cache entries for minimal changes
    }

    // Content type detection removed - replaced with user-controlled item_sizing_mode
    // Users now explicitly specify whether their items have dynamic sizing

    /// PHASE 3: Store item context using owned registry (replaces global registry)
    fn store_item_context(&mut self, panel: &Panel, index: usize, data: &dyn Any) {
        // Store current item index and data in panel context registry
        // This ensures event handlers always get the current item context
        let panel_id = panel.get_id();

        // For now, we'll store a simplified version of the data
        // In a real implementation, data sources would provide Clone + Send + Sync data
        let stored_data: Arc<dyn Any + Send + Sync> =
            if let Some(string_data) = data.downcast_ref::<String>() {
                Arc::new(string_data.clone())
            } else {
                Arc::new(format!("Item {index}"))
            };

        let context = ItemContext {
            index,
            data: stored_data,
        };

        // TASK 3.1: Use owned registry instead of global static
        self.panel_context_registry.insert(panel_id, context);
    }

    /// PHASE 3: Remove panel context using owned registry
    fn remove_panel_context(&mut self, panel: &Panel) {
        let panel_id = panel.get_id();
        // TASK 3.1: Use owned registry instead of global static
        self.panel_context_registry.remove(&panel_id);
    }

    fn hit_test(&self, point: Point) -> Option<usize> {
        // Simple hit test based on currently visible items
        for (&index, panel) in &self.item_to_panel {
            let panel_pos = panel.get_position();
            let panel_size = panel.get_size();

            if point.x >= panel_pos.x
                && point.x < panel_pos.x + panel_size.width
                && point.y >= panel_pos.y
                && point.y < panel_pos.y + panel_size.height
            {
                return Some(index);
            }
        }

        None
    }

    fn clear_all_items(&mut self) {
        // Clear all items and return panels to pool
        self.hide_all_items();
        self.item_pool.clear_all();
    }

    /// PHASE 3: Get item context for a panel using owned registry
    fn get_item_context_for_panel(&self, panel: &Panel) -> Option<ItemContext> {
        let panel_id = panel.get_id();
        self.panel_context_registry.get(&panel_id).cloned()
    }

    /// PHASE 3: Get item index for a panel (convenience method)
    fn get_index_for_panel(&self, panel: &Panel) -> Option<usize> {
        self.get_item_context_for_panel(panel).map(|ctx| ctx.index)
    }

    /// PHASE 3: Get typed data for a panel (convenience method)
    fn get_data_for_panel<T>(&self, panel: &Panel) -> Option<T>
    where
        T: Clone + 'static,
    {
        self.get_item_context_for_panel(panel)
            .and_then(|ctx| ctx.data.downcast_ref::<T>().cloned())
    }

    /// PHASE 2.3: Execute all panel operations using batched layout system to eliminate layout thrashing
    /// This method replaces the individual cleanup_invisible_panels, update_panel_content_batch,
    /// handle_dimension_change_batch, and position_visible_panels methods with a unified batching approach
    fn execute_batched_layout_operations(&mut self, context: BatchExecutionContext) {
        let mut batch = LayoutBatch::new();

        // BATCH PHASE 1: Collect panels to destroy (no longer visible)
        let items_to_hide: Vec<usize> = context
            .current_visible_items
            .difference(context.new_visible_items)
            .cloned()
            .collect();

        for item_index in items_to_hide {
            if let Some(panel) = self.item_to_panel.remove(&item_index) {
                batch.add_destroy_operation(panel.clone());
                // Return panel to pool for reuse (will be done after batch execution)
                self.item_pool.return_item(panel);
            }
        }

        // BATCH PHASE 2: Collect panel creation/update operations
        for (data_index, _item_start, _) in context.items_to_show {
            if !self.item_to_panel.contains_key(data_index) {
                // NEW PANEL: Need to create this item
                let item_panel = self.item_pool.get_or_create_item(context.parent, || {
                    context.item_renderer.create_item(context.parent)
                });

                let item_data = context.data_source.get_item_data(*data_index);
                batch.add_create_operation(*data_index, item_panel.clone(), item_data);

                // Track this panel for this data index
                self.item_to_panel.insert(*data_index, item_panel);
            } else if context.dimension_changed {
                // EXISTING PANEL: Update due to dimension change
                if let Some(panel) = self.item_to_panel.get(data_index) {
                    let item_data = context.data_source.get_item_data(*data_index);
                    let initial_size = match self.layout_mode {
                        VirtualListLayoutMode::Vertical => Size::new(
                            self.viewport_size.width,
                            self.internal_params.temporary_panel_height,
                        ),
                        VirtualListLayoutMode::Horizontal => Size::new(
                            2000, // Give plenty of width for natural text flow measurement
                            self.viewport_size.height,
                        ),
                    };
                    batch.add_update_operation(*data_index, panel.clone(), item_data, initial_size);
                }
            }
        }

        // BATCH PHASE 3: Execute create/update operations first to get measurements
        let execution_result = batch.execute_batch(context.item_renderer.as_ref(), self);

        // BATCH PHASE 4: Update cache with fresh measurements
        let mut fresh_measurements = std::collections::HashMap::new();
        for (index, measured_size) in &execution_result.final_measurements {
            // PHASE 1.2: Store progressive measurement in cache
            self.store_progressive_measurement(
                *index,
                *measured_size,
                context.current_viewport_dimension,
            );
            fresh_measurements.insert(*index, *measured_size);
        }

        // BATCH PHASE 5: Position all items using fresh measurements and cache
        let mut position_batch = LayoutBatch::new();

        for (data_index, item_start, _) in context.items_to_show {
            if let Some(panel) = self.item_to_panel.get(data_index) {
                // Get final size: prioritize fresh measurements, then cache, then estimates
                let final_size = if let Some(fresh_size) = fresh_measurements.get(data_index) {
                    // Use fresh measurement from this update cycle
                    match self.layout_mode {
                        VirtualListLayoutMode::Vertical => {
                            Size::new(self.viewport_size.width, fresh_size.height)
                        }
                        VirtualListLayoutMode::Horizontal => {
                            Size::new(fresh_size.width, self.viewport_size.height)
                        }
                    }
                } else if let Some(cached_size) = self.item_size_cache.peek(*data_index) {
                    // Use cached measurement
                    match self.layout_mode {
                        VirtualListLayoutMode::Vertical => {
                            Size::new(self.viewport_size.width, cached_size.size.height)
                        }
                        VirtualListLayoutMode::Horizontal => {
                            Size::new(cached_size.size.width, self.viewport_size.height)
                        }
                    }
                } else {
                    // Fallback to estimated size
                    match self.layout_mode {
                        VirtualListLayoutMode::Vertical => Size::new(
                            self.viewport_size.width,
                            self.internal_params.estimated_item_height,
                        ),
                        VirtualListLayoutMode::Horizontal => Size::new(
                            self.internal_params.estimated_item_width,
                            self.viewport_size.height,
                        ),
                    }
                };

                let position = match self.layout_mode {
                    VirtualListLayoutMode::Vertical => {
                        Point::new(0, item_start - context.scroll_position)
                    }
                    VirtualListLayoutMode::Horizontal => {
                        Point::new(item_start - context.scroll_position, 0)
                    }
                };

                position_batch.add_position_operation(panel.clone(), position, final_size);
            }
        }

        // BATCH PHASE 6: Execute positioning with correct sizes
        position_batch.execute_batch(context.item_renderer.as_ref(), self);
    }
}

// TASK 3.1: Add Drop implementation for automatic cleanup
impl Drop for VirtualListState {
    fn drop(&mut self) {
        // Clear the panel context registry to prevent memory leaks
        // This ensures all contexts are cleaned up when the virtual list is destroyed
        self.panel_context_registry.clear();
    }
}

custom_widget!(
    name: VirtualList,
    fields: {
        layout_mode: VirtualListLayoutMode = VirtualListLayoutMode::Vertical,
        scrollbar_size: Option<i32> = None,
        keyboard_scroll_amount: Option<i32> = None,
        mouse_wheel_multiplier: f32 = 1.0,
        state: Rc<RefCell<VirtualListState>> = Rc::new(RefCell::new(VirtualListState::new(
            VirtualListLayoutMode::Vertical,
            None,  // scrollbar_size
            None,  // keyboard_scroll_amount
            1.0,   // mouse_wheel_multiplier
        ))),
    },
    setup_impl: |config, panel| {
        // Set up the panel for virtual list behavior with proper clipping
        panel.set_background_style(BackgroundStyle::System); // Use default system background

        // Enable clipping of child windows to prevent items from appearing outside bounds
        panel.add_style(WindowStyle::ClipChildren);

        // TODO: Add a simple border to define the virtual list area when border API is available

        // VirtualList setup complete

        // Create scrollbars based on layout mode
        let panel_size = panel.get_size();
        let scrollbar_size = config.scrollbar_size.unwrap_or(16); // User preference or system default

        let (content_area, scrollbar) = match config.layout_mode {
            VirtualListLayoutMode::Vertical => {
                // Create vertical scrollbar on the right side
                let scrollbar = crate::widgets::ScrollBar::builder(&panel)
                    .with_style(crate::widgets::ScrollBarStyle::Vertical)
                    .with_pos(Point::new(panel_size.width - scrollbar_size, 0))
                    .with_size(Size::new(scrollbar_size, panel_size.height))
                    .build();

                // Content area is full height, reduced width
                let content_area = Size::new(panel_size.width - scrollbar_size, panel_size.height);
                (content_area, Some((scrollbar, true))) // true = vertical
            },
            VirtualListLayoutMode::Horizontal => {
                // Create horizontal scrollbar at the bottom
                let scrollbar = crate::widgets::ScrollBar::builder(&panel)
                    .with_style(crate::widgets::ScrollBarStyle::Default) // Default = horizontal
                    .with_pos(Point::new(0, panel_size.height - scrollbar_size))
                    .with_size(Size::new(panel_size.width, scrollbar_size))
                    .build();

                // Content area is full width, reduced height
                let content_area = Size::new(panel_size.width, panel_size.height - scrollbar_size);
                (content_area, Some((scrollbar, false))) // false = horizontal
            }
        };

        // Store scrollbar references for event handling
        let (v_scrollbar, h_scrollbar) = if let Some((scrollbar_widget, is_vertical)) = scrollbar {
            if is_vertical {
                // Initialize vertical scrollbar with proper values
                scrollbar_widget.set_scrollbar(0, 20, 100, 20, true);
                scrollbar_widget.enable(true);  // Explicitly enable scrollbar
                scrollbar_widget.show(true);    // Explicitly show scrollbar
                (Some(scrollbar_widget), None)
            } else {
                // Initialize horizontal scrollbar with proper values
                scrollbar_widget.set_scrollbar(0, 20, 100, 20, true);
                scrollbar_widget.enable(true);  // Explicitly enable scrollbar
                scrollbar_widget.show(true);    // Explicitly show scrollbar
                (None, Some(scrollbar_widget))
            }
        } else {
            (None, None)
        };

        // Initialize the state with the actual layout mode and content area
        {
            config.state.borrow_mut().layout_mode = config.layout_mode;
            // Use content area (excluding scrollbar) as viewport
            config.state.borrow_mut().viewport_size = content_area;
        }

        // Function to update scrollbar properties
        let v_scrollbar_update = v_scrollbar.clone();
        let h_scrollbar_update = h_scrollbar.clone();
        let layout_mode_update = config.layout_mode;
        let state_update = config.state.clone();
        let update_scrollbars = move || {
            let state = state_update.clone();
            match layout_mode_update {
                VirtualListLayoutMode::Vertical => {
                    if let Some(ref vscrollbar) = v_scrollbar_update {
                        let max_scroll = (state.borrow().total_content_size.height - state.borrow().viewport_size.height).max(1);
                        let current_pos = if max_scroll > 0 {
                            (state.borrow().scroll_position.y * 100 / max_scroll).min(100)
                        } else {
                            0
                        };

                        // Set scrollbar: position, thumb_size, range, page_size, refresh
                        // thumb_size represents visible portion, page_size is for page up/down
                        let thumb_size = if state.borrow().total_content_size.height > 0 {
                            ((state.borrow().viewport_size.height * 100) / state.borrow().total_content_size.height).clamp(1, 99)
                        } else {
                            95
                        };

                        vscrollbar.set_scrollbar(current_pos, thumb_size, 100, thumb_size, true);
                    }
                },
                VirtualListLayoutMode::Horizontal => {
                    if let Some(ref hscrollbar) = h_scrollbar_update {
                        // Calculate actual content scroll range without artificial buffer
                        let max_scroll = (state.borrow().total_content_size.width - state.borrow().viewport_size.width).max(1);
                        let current_pos = if max_scroll > 0 {
                            (state.borrow().scroll_position.x * 100 / max_scroll).min(100)
                        } else {
                            0
                        };



                        // Set scrollbar: position, thumb_size, range, page_size, refresh
                        // Use smaller thumb size to ensure full range accessibility
                        let thumb_size = if state.borrow().total_content_size.width > 0 {
                            ((state.borrow().viewport_size.width * 100) / state.borrow().total_content_size.width).clamp(1, 95)
                        } else {
                            90
                        };

                        hscrollbar.set_scrollbar(current_pos, thumb_size, 100, thumb_size, true);
                    }
                }
            }
        };

        // Set up paint event for rendering visible items
        let update_scrollbars_paint = update_scrollbars.clone();
        panel.on_paint(move |event| {
            // Paint events should only draw existing content, not recalculate layout
            update_scrollbars_paint();
            event.skip(true);
        });

        // Set up resize event to update viewport and reposition scrollbars
        let panel_resize = panel.clone();
        let state_resize = config.state.clone();
        let layout_mode_resize = config.layout_mode;
        let vscrollbar_resize = v_scrollbar.clone();
        let hscrollbar_resize = h_scrollbar.clone();
        let update_scrollbars_resize = update_scrollbars.clone();

        panel.on_size(move |event| {
            let new_size = panel_resize.get_size();
            let scrollbar_size = 16;

            // Reposition scrollbars and calculate new content area
            let content_area = match layout_mode_resize {
                VirtualListLayoutMode::Vertical => {
                    if let Some(ref vscrollbar) = vscrollbar_resize {
                        vscrollbar.move_window(new_size.width - scrollbar_size, 0);
                        vscrollbar.set_size(Size::new(scrollbar_size, new_size.height));
                    }
                    Size::new(new_size.width - scrollbar_size, new_size.height)
                },
                VirtualListLayoutMode::Horizontal => {
                    if let Some(ref hscrollbar) = hscrollbar_resize {
                        hscrollbar.move_window(0, new_size.height - scrollbar_size);
                        hscrollbar.set_size(Size::new(new_size.width, scrollbar_size));
                    }
                    Size::new(new_size.width, new_size.height - scrollbar_size)
                }
            };

            let state = state_resize.clone();
            let old_viewport_size = state.borrow().viewport_size;
            state.borrow_mut().viewport_size = content_area;

            // Auto-reconfigure internal parameters when viewport size changes significantly
            if (old_viewport_size.width - content_area.width).abs() > 50 ||
               (old_viewport_size.height - content_area.height).abs() > 50 {
                state.borrow_mut().internal_params.auto_configure(
                    (content_area.width, content_area.height),
                    config.scrollbar_size,
                    config.keyboard_scroll_amount,
                    config.mouse_wheel_multiplier,
                );
            }

            let mut cloned_state = state.borrow().clone();
            cloned_state.update_visible_items(&panel_resize);
            *state.borrow_mut() = cloned_state;

            // Update scrollbars after viewport size change
            update_scrollbars_resize();
            panel_resize.refresh(false, None);
            event.skip(true);
        });

        // Set up mouse wheel scrolling with proper wheel delta handling
        let panel_scroll = panel.clone();
        let state_scroll = config.state.clone();
        let panel_scroll_wheel = panel_scroll.clone();
        let state_scroll_wheel = state_scroll.clone();
        let update_scrollbars_wheel = update_scrollbars.clone();

        // Mouse wheel with proper bidirectional scrolling using wheel rotation
        panel.on_mouse_wheel(move |event| {
            // Extract wheel rotation from event data for bidirectional scrolling
            let (wheel_rotation, _wheel_delta) = match event {
                crate::event::window_events::WindowEventData::MouseButton(ref mouse_event) => {
                    (mouse_event.event.get_wheel_rotation(), mouse_event.event.get_wheel_delta())
                },
                crate::event::window_events::WindowEventData::General(ref general_event) => {
                    (general_event.get_wheel_rotation(), general_event.get_wheel_delta())
                },
                _ => (0, 120)
            };

            // Calculate scroll amount - negative rotation means scroll down (increase scroll position)
            // Positive rotation means scroll up (decrease scroll position)
            let scroll_amount = -wheel_rotation; // Invert to match expected scroll direction

            if scroll_amount == 0 {
                event.skip(true);
                return;
            }

            let state = state_scroll_wheel.clone();

            let layout_mode = state.borrow().layout_mode;
            match layout_mode {
                VirtualListLayoutMode::Vertical => {
                    let max_scroll = (state.borrow().total_content_size.height - state.borrow().viewport_size.height).max(0);
                    let tentative_scroll_y = (state.borrow().scroll_position.y + scroll_amount)
                        .max(0)
                        .min(max_scroll);

                    // CRITICAL FIX: Mouse wheel end-of-list handling to prevent clipping
                    let new_scroll_y = if max_scroll > 0 {
                        // Calculate base content size (without padding) to determine when end-of-list logic is needed
                        let data_source_opt = state.borrow().data_source.clone();
                        let base_max_scroll = if let Some(ref data_source) = data_source_opt {
                            let total_items = data_source.get_item_count();
                            if total_items > 0 {
                                // Calculate base content height (same logic as calculate_total_content_size_progressive)
                                let mut base_total_height = 0;
                                for i in 0..total_items {
                                    let item_height = if let Some(cached_size) = state.borrow().item_size_cache.peek(i) {
                                        cached_size.size.height
                                    } else {
                                        state.borrow().internal_params.estimated_item_height
                                    };
                                    base_total_height += item_height;
                                }
                                (base_total_height - state.borrow().viewport_size.height).max(0)
                            } else {
                                max_scroll
                            }
                        } else {
                            max_scroll
                        };

                        // Only trigger end-of-list logic when actually trying to scroll past the base content
                        // This eliminates false triggers from ratio-based detection
                        let scrolling_down = scroll_amount > 0;
                        if tentative_scroll_y >= base_max_scroll && scrolling_down {
                            let data_source_opt = state.borrow().data_source.clone();
                            let item_renderer_opt = state.borrow().item_renderer.clone();

                            if let (Some(data_source), Some(item_renderer)) = (data_source_opt, item_renderer_opt) {
                                let total_items = data_source.get_item_count();
                                if total_items > 0 {
                                    let last_item_index = total_items - 1;

                                    // Force measurement of the last item to get its actual size
                                    let mut cloned_state = state.borrow().clone();
                                    let last_item_size = cloned_state.measure_item_size(
                                        last_item_index,
                                        data_source.as_ref(),
                                        item_renderer.as_ref(),
                                        &panel_scroll_wheel,
                                    );
                                    *state.borrow_mut() = cloned_state;

                                    // Calculate total content size up to and including the last item
                                    let mut total_height = 0;
                                    for i in 0..total_items {
                                        let item_height = if let Some(cached_size) = state.borrow().item_size_cache.peek(i) {
                                            cached_size.size.height
                                        } else if i == last_item_index {
                                            last_item_size.height
                                        } else {
                                            state.borrow().internal_params.estimated_item_height
                                        };
                                        total_height += item_height;
                                    }

                                                                         // Add safety padding to ensure last item is fully visible
                                     let safety_padding = state.borrow().internal_params.safety_padding;
                                     let padded_total_height = total_height + safety_padding;

                                     // CRITICAL FIX: Update total_content_size to match the padded calculation
                                     // This ensures consistency between scrollbar and mouse wheel scrolling
                                     state.borrow_mut().total_content_size.height = padded_total_height;

                                     // Calculate scroll position to show the end with safety padding
                                     (padded_total_height - state.borrow().viewport_size.height).max(0)
                                } else {
                                    tentative_scroll_y
                                }
                            } else {
                                tentative_scroll_y
                            }
                        } else {
                            // Normal scrolling: allow reaching the base content boundary to trigger end-of-list logic
                            tentative_scroll_y.max(0).min(base_max_scroll.max(max_scroll))
                        }
                    } else {
                        tentative_scroll_y
                    };

                    let old_y = state.borrow().scroll_position.y;
                    if new_scroll_y != old_y {
                        state.borrow_mut().scroll_position.y = new_scroll_y;

                        let mut cloned_state = state.borrow().clone();
                        cloned_state.update_visible_items(&panel_scroll_wheel);
                        *state.borrow_mut() = cloned_state;

                        // Use central scrollbar update function for consistent behavior
                        update_scrollbars_wheel();
                        panel_scroll_wheel.refresh(false, None);
                    }
                }
                VirtualListLayoutMode::Horizontal => {
                    // Calculate actual content scroll range without artificial buffer
                    let max_scroll = (state.borrow().total_content_size.width - state.borrow().viewport_size.width).max(0);
                    let new_scroll_x = (state.borrow().scroll_position.x + scroll_amount)
                        .max(0)
                        .min(max_scroll);

                    let old_x = state.borrow().scroll_position.x;
                    if new_scroll_x != old_x {
                        state.borrow_mut().scroll_position.x = new_scroll_x;

                        let mut cloned_state = state.borrow().clone();
                        cloned_state.update_visible_items(&panel_scroll_wheel);
                        *state.borrow_mut() = cloned_state;

                        // Use central scrollbar update function for consistent behavior
                        update_scrollbars_wheel();
                        panel_scroll_wheel.refresh(false, None);
                    }
                }
            }

            event.skip(true);
        });

        // Also keep keyboard controls as backup/alternative navigation
        // Arrow Down = scroll forward, Arrow Up = scroll backward
        let update_scrollbars_key = update_scrollbars.clone();
        panel.on_key_down(move |event| {
            let key_code = match event {
                crate::event::window_events::WindowEventData::Keyboard(ref kbd_event) => {
                    kbd_event.get_key_code().unwrap_or(0)
                }
                _ => 0
            };

            let scroll_delta = match key_code {
                314 => Some(300),  // Down arrow - scroll forward
                315 => Some(-300), // Up arrow - scroll backward
                _ => None
            };

            if let Some(delta) = scroll_delta {
                let state = state_scroll.clone();

                let layout_mode = state.borrow().layout_mode;
                match layout_mode {
                    VirtualListLayoutMode::Vertical => {
                        let max_scroll = (state.borrow().total_content_size.height - state.borrow().viewport_size.height).max(0);
                        let new_scroll_y = (state.borrow().scroll_position.y + delta)
                            .max(0)
                            .min(max_scroll);

                        let old_y = state.borrow().scroll_position.y;
                        if new_scroll_y != old_y {
                            state.borrow_mut().scroll_position.y = new_scroll_y;

                            let mut cloned_state = state.borrow().clone();
                            cloned_state.update_visible_items(&panel_scroll);
                            *state.borrow_mut() = cloned_state;

                            // Use central scrollbar update function for consistent behavior
                            update_scrollbars_key();
                            panel_scroll.refresh(false, None);
                        }
                    }
                    VirtualListLayoutMode::Horizontal => {
                        // Calculate actual content scroll range without artificial buffer
                        let max_scroll = (state.borrow().total_content_size.width - state.borrow().viewport_size.width).max(0);
                        let new_scroll_x = (state.borrow().scroll_position.x + delta)
                            .max(0)
                            .min(max_scroll);

                        let old_x = state.borrow().scroll_position.x;
                        if new_scroll_x != old_x {
                            state.borrow_mut().scroll_position.x = new_scroll_x;

                            let mut cloned_state = state.borrow().clone();
                            cloned_state.update_visible_items(&panel_scroll);
                            *state.borrow_mut() = cloned_state;

                            // Use central scrollbar update function for consistent behavior
                            update_scrollbars_key();
                            panel_scroll.refresh(false, None);
                        }
                    }
                };
            }

            event.skip(true);
        });

        // Simplified scrollbar event handling - only thumb track for drag support
        if let Some(ref vscrollbar) = v_scrollbar {
            let panel_vscroll = panel.clone();
            let state_vscroll = config.state.clone();
            let update_scrollbars_vscroll = update_scrollbars.clone();

            // Only handle thumb track for smooth drag scrolling
            vscrollbar.on_thumb_track(move |event| {
                if let Some(position) = event.get_position() {
                    let state = state_vscroll.clone();

                    // CRITICAL FIX: Store the desired scroll RATIO instead of absolute position
                    // This prevents mismatch when total_content_size changes during update_visible_items
                    let effective_max_position = 95.0;
                    let scroll_ratio = (position as f32 / effective_max_position).clamp(0.0, 1.0);

                    // CRITICAL FIX: Last-item-aware scrolling for end-of-list positioning
                    if scroll_ratio >= 0.95 {
                        // User is dragging near the end - ensure last item is fully visible
                        let data_source_opt = state.borrow().data_source.clone();
                        let item_renderer_opt = state.borrow().item_renderer.clone();

                        if let (Some(data_source), Some(item_renderer)) = (data_source_opt, item_renderer_opt) {
                            let total_items = data_source.get_item_count();
                            if total_items > 0 {
                                let last_item_index = total_items - 1;

                                // Force measurement of the last item to get its actual size
                                let mut cloned_state = state.borrow().clone();
                                let last_item_size = cloned_state.measure_item_size(
                                    last_item_index,
                                    data_source.as_ref(),
                                    item_renderer.as_ref(),
                                    &panel_vscroll,
                                );
                                *state.borrow_mut() = cloned_state;

                                // Calculate total content size up to and including the last item
                                let mut total_height = 0;
                                for i in 0..total_items {
                                    let item_height = if let Some(cached_size) = state.borrow().item_size_cache.peek(i) {
                                        cached_size.size.height
                                    } else if i == last_item_index {
                                        last_item_size.height
                                    } else {
                                        state.borrow().internal_params.estimated_item_height
                                    };
                                    total_height += item_height;
                                }

                                // CRITICAL FIX: Add safety padding to total height to ensure last item has space
                                let safety_padding = state.borrow().internal_params.safety_padding;
                                let padded_total_height = total_height + safety_padding;

                                // Calculate scroll position to show the end with the safety padding
                                let target_scroll_y = (padded_total_height - state.borrow().viewport_size.height).max(0);

                                state.borrow_mut().scroll_position.y = target_scroll_y;

                                // CRITICAL FIX: Update total_content_size to match the padded calculation
                                // This ensures mouse wheel scrolling uses the same bounds as scrollbar dragging
                                state.borrow_mut().total_content_size.height = padded_total_height;

                                let mut cloned_state = state.borrow().clone();
                                cloned_state.update_visible_items(&panel_vscroll);
                                *state.borrow_mut() = cloned_state;
                            }
                        }
                    } else {
                        // Normal scrolling for non-end positions
                        let old_max_scroll = (state.borrow().total_content_size.height - state.borrow().viewport_size.height).max(0);
                        let target_scroll_y = if old_max_scroll > 0 {
                            (scroll_ratio * old_max_scroll as f32).round() as i32
                        } else {
                            0
                        };

                        let old_y = state.borrow().scroll_position.y;
                        if target_scroll_y != old_y {
                            state.borrow_mut().scroll_position.y = target_scroll_y;

                            let mut cloned_state = state.borrow().clone();
                            cloned_state.update_visible_items(&panel_vscroll);
                            *state.borrow_mut() = cloned_state;
                        }
                    }

                    // For DynamicSize mode, ensure a final layout pass after rapid scrolling
                    if state.borrow().item_sizing_mode == ItemSizingMode::DynamicSize {
                        // Force layout on all visible panels to ensure proper text wrapping
                        for panel in state.borrow().item_to_panel.values() {
                            panel.layout();
                        }
                    }

                    // Use central scrollbar update function for consistent behavior
                    update_scrollbars_vscroll();
                    panel_vscroll.refresh(false, None);
                }
            });
        }

        if let Some(ref hscrollbar) = h_scrollbar {
            let panel_hscroll = panel.clone();
            let state = config.state.clone();
            let update_scrollbars_hscroll = update_scrollbars.clone();

            // Only handle thumb track for horizontal scrollbar drag
            hscrollbar.on_thumb_track(move |event| {
                if let Some(position) = event.get_position() {

                    // CRITICAL FIX: Store the desired scroll RATIO instead of absolute position
                    // This prevents mismatch when total_content_size changes during update_visible_items
                    let effective_scrollbar_range = 99.0;
                    let scroll_ratio = (position as f32 / effective_scrollbar_range).clamp(0.0, 1.0);

                    // Calculate initial scroll position
                    let old_max_scroll = (state.borrow().total_content_size.width - state.borrow().viewport_size.width).max(0);
                    let initial_scroll_x = if old_max_scroll > 0 {
                        (scroll_ratio * old_max_scroll as f32).round() as i32
                    } else {
                        0
                    };

                    let old_x = state.borrow().scroll_position.x;
                    if initial_scroll_x != old_x {
                        state.borrow_mut().scroll_position.x = initial_scroll_x;

                        let mut cloned_state = state.borrow().clone();
                        cloned_state.update_visible_items(&panel_hscroll);
                        *state.borrow_mut() = cloned_state;

                        // CRITICAL FIX: Force immediate content size recalculation with fresh measurements
                        // The issue was that update_visible_items measures items but cache update happens later
                        let total_items =  state.borrow().data_source.as_ref().map(|data_source| data_source.get_item_count());
                        let estimated_item_size = state.borrow().internal_params.estimated_item_width;
                        if let Some(total_items) = total_items {
                            let new_content_size = state.borrow().calculate_total_content_size_progressive(total_items, estimated_item_size);
                            state.borrow_mut().total_content_size = new_content_size;
                        }
                        // Now recalculate scroll position with the corrected content size
                        let new_max_scroll = (state.borrow().total_content_size.width - state.borrow().viewport_size.width).max(0);
                        let final_scroll_x = if new_max_scroll > 0 {
                            (scroll_ratio * new_max_scroll as f32).round() as i32
                        } else {
                            0
                        }.max(0).min(new_max_scroll);

                        // Update scroll position to account for content size changes
                        state.borrow_mut().scroll_position.x = final_scroll_x;

                        // For DynamicSize mode, ensure a final layout pass after rapid scrolling
                        if state.borrow().item_sizing_mode == ItemSizingMode::DynamicSize {
                            // Force layout on all visible panels to ensure proper text wrapping
                            for panel in state.borrow().item_to_panel.values() {
                                panel.layout();
                            }
                        }

                        // Use central scrollbar update function for consistent behavior
                        update_scrollbars_hscroll();
                        panel_hscroll.refresh(false, None);
                    }
                }
            });
        }

        // Set up virtual list event handling
        // TODO: Hit testing will be implemented later without interfering with child widgets

        // VirtualList setup completed
    }
);

impl VirtualList {
    /// Set the data source for the virtual list
    pub fn set_data_source<T: VirtualListDataSource + 'static>(&self, data_source: T) {
        // Setting data source

        self.config()
            .state
            .borrow_mut()
            .set_data_source(Rc::new(data_source));

        // Total content size will be recalculated on next update

        self.refresh(false, None);

        // Trigger initial update
        self.trigger_initial_update();
    }

    /// Set the item renderer for the virtual list
    pub fn set_item_renderer<T: VirtualListItemRenderer + 'static>(&self, item_renderer: T) {
        // Setting item renderer

        self.config()
            .state
            .borrow_mut()
            .set_item_renderer(Rc::new(item_renderer));

        // Total content size will be recalculated on next update

        self.refresh(false, None);

        // Trigger initial update
        self.trigger_initial_update();
    }

    /// Set the item sizing mode to control cache invalidation behavior
    ///
    /// - `ItemSizingMode::FixedSize`: Use for items with fixed size (images, fixed text)
    ///   Cache entries are preserved when width changes for better performance
    ///
    /// - `ItemSizingMode::DynamicSize`: Use for items that resize based on container width
    ///   (text wrapping, responsive layouts). Cache entries are invalidated when width changes
    pub fn set_item_sizing_mode(&self, sizing_mode: ItemSizingMode) {
        self.config().state.borrow_mut().item_sizing_mode = sizing_mode;
    }

    /// Manually trigger an initial update to ensure items become visible
    fn trigger_initial_update(&self) {
        // Force item creation without changing the viewport size (it's already set to content area)
        let mut cloned_state = self.config().state.borrow().clone();
        cloned_state.update_visible_items(self);
        *self.config().state.borrow_mut() = cloned_state;

        // Force a redraw
        self.refresh(false, None);
    }

    /// PHASE 3: Scroll to show a specific item with error handling
    pub fn scroll_to_item(&self, index: usize) -> VirtualListResult<()> {
        let state = self.config().state.clone();

        let total_items = state
            .borrow()
            .data_source
            .clone()
            .map(|ds| ds.get_item_count());
        let Some(total_items) = total_items else {
            return Err(VirtualListError::data_source_error("No data source set"));
        };

        // Validate index bounds
        if index >= total_items {
            return Err(VirtualListError::invalid_index(index, total_items));
        }

        // Calculate position using mix of actual measurements and estimates
        let estimated_item_height = 80; // Same estimate as in update_visible_items
        let mut y_position = 0;

        for i in 0..index {
            let item_height = if let Some(size) = state.borrow().item_size_cache.peek(i) {
                // Use actual measurement if available
                match state.borrow().layout_mode {
                    VirtualListLayoutMode::Vertical => size.size.height,
                    VirtualListLayoutMode::Horizontal => size.size.width,
                }
            } else {
                // Use estimate for unmeasured items
                estimated_item_height
            };

            y_position += item_height;
        }

        let new_scroll_position = match state.borrow().layout_mode {
            VirtualListLayoutMode::Vertical => Point::new(0, y_position),
            VirtualListLayoutMode::Horizontal => Point::new(y_position, 0),
        };

        state.borrow_mut().scroll_position = new_scroll_position;
        self.refresh(false, None);

        Ok(())
    }

    /// Refresh the virtual list (recalculate everything)
    pub fn refresh_virtual_list(&self) {
        self.config().state.borrow_mut().invalidate_layout();
        self.refresh(false, None);
    }

    /// Get the currently visible range of items
    pub fn get_visible_range(&self) -> std::ops::Range<usize> {
        self.config().state.borrow().visible_range.clone()
    }

    /// Get pool statistics for debugging/monitoring
    pub fn get_pool_stats(&self) -> super::adaptive_pool::PoolStats {
        self.config().state.borrow().item_pool.get_pool_stats()
    }

    /// PHASE 3: Hit test to find which item is at a given point with validation
    pub fn hit_test(&self, point: Point) -> VirtualListResult<Option<usize>> {
        // Validate point coordinates
        if point.x < 0 || point.y < 0 {
            return Err(VirtualListError::invalid_config(format!(
                "Invalid hit test coordinates: ({}, {})",
                point.x, point.y
            )));
        }

        let state = self.config().state.borrow();
        Ok(state.hit_test(point))
    }

    /// Clear all items and reset the virtual list
    pub fn clear(&self) {
        self.config().state.borrow_mut().clear_all_items();
        self.refresh(false, None);
    }

    /// Get the total content size
    pub fn get_total_content_size(&self) -> Size {
        // Return the cached total content size or a default
        self.config().state.borrow().total_content_size
    }

    /// PHASE 3: Get item context for a panel with error handling (replaces global registry access)
    pub fn get_item_context_for_panel(&self, panel: &Panel) -> VirtualListResult<ItemContext> {
        self.config()
            .state
            .borrow()
            .get_item_context_for_panel(panel)
            .into_vl_error("get_item_context_for_panel")
    }

    /// PHASE 3: Get item index for a panel with error handling (convenience method)
    pub fn get_index_for_panel(&self, panel: &Panel) -> VirtualListResult<usize> {
        self.config()
            .state
            .borrow()
            .get_index_for_panel(panel)
            .into_vl_error("get_index_for_panel")
    }

    /// PHASE 3: Get typed data for a panel with error handling (convenience method)
    pub fn get_data_for_panel<T>(&self, panel: &Panel) -> VirtualListResult<T>
    where
        T: Clone + 'static,
    {
        self.config()
            .state
            .borrow()
            .get_data_for_panel(panel)
            .into_vl_error("get_data_for_panel")
    }
}
