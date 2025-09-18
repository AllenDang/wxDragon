use super::{config::VirtualListInternalParams, VirtualListLayoutMode};
use crate::prelude::*;
use std::time::{Duration, Instant};

/// PHASE 2B: Enhanced adaptive item pool with intelligent optimization
#[derive(Clone)]
pub struct AdaptiveItemPool {
    available_panels: Vec<Panel>,
    target_pool_size: usize,
    max_pool_size: usize,

    // TASK 2.3: Performance tracking for optimization
    total_created: usize,
    total_reused: usize,
    total_requests: usize,
    last_optimization: Instant,
    optimization_interval: Duration,

    // TASK 2.3: Pool efficiency management
    pool_hit_rate_history: Vec<f64>,
    efficiency_target: f64,
}

impl std::fmt::Debug for AdaptiveItemPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdaptiveItemPool")
            .field(
                "available_panels",
                &format!("{} panels", self.available_panels.len()),
            )
            .field("target_pool_size", &self.target_pool_size)
            .field("max_pool_size", &self.max_pool_size)
            .field("total_created", &self.total_created)
            .field("total_reused", &self.total_reused)
            .field("hit_rate", &self.calculate_hit_rate())
            .field("efficiency_score", &self.calculate_efficiency_score())
            .finish()
    }
}

impl AdaptiveItemPool {
    /// Create a new adaptive item pool with intelligent optimization
    pub fn new(
        _layout_mode: VirtualListLayoutMode,
        internal_params: &VirtualListInternalParams,
    ) -> Self {
        let target_size = internal_params.pool_target_size;
        Self {
            available_panels: Vec::with_capacity(target_size),
            target_pool_size: target_size,
            max_pool_size: target_size * 2, // Allow growth up to 2x target

            // Performance tracking
            total_created: 0,
            total_reused: 0,
            total_requests: 0,
            last_optimization: Instant::now(),
            optimization_interval: Duration::from_secs(5), // Optimize every 5 seconds

            // Pool optimization
            pool_hit_rate_history: Vec::with_capacity(10),
            efficiency_target: 0.8, // Target 80% hit rate
        }
    }

    /// PHASE 2B: Enhanced panel acquisition with smart optimization
    pub fn get_or_create_item<F>(&mut self, _parent: &Panel, create_fn: F) -> Panel
    where
        F: FnOnce() -> Panel,
    {
        self.total_requests += 1;

        if let Some(panel) = self.available_panels.pop() {
            // TASK 2.3: Successful pool hit - reuse existing panel
            self.total_reused += 1;
            panel.show(true);

            // Clear any previous content/state for clean reuse
            // Note: More sophisticated cleanup could be added here
            panel
        } else {
            // TASK 2.3: Pool miss - create new panel
            self.total_created += 1;
            let new_panel = create_fn();

            // Auto-optimize pool if needed
            self.maybe_optimize_pool();

            new_panel
        }
    }

    /// PHASE 2B: Enhanced panel return with intelligent management
    pub fn return_item(&mut self, panel: Panel) {
        // Don't exceed maximum pool size - prevents memory bloat
        if self.available_panels.len() < self.max_pool_size {
            // Hide and clean panel for reuse
            panel.show(false);

            // TASK 2.3: Clean panel state for optimal reuse
            // Reset any styles or properties that might interfere
            // (More comprehensive cleanup could be added here)

            self.available_panels.push(panel);
        } else {
            // Pool is full - just let panel drop (get destroyed)
            panel.show(false);
        }
    }

    // Pool warming functionality removed - was never implemented in the virtual list logic
    // The pool auto-resizes based on usage patterns instead

    /// TASK 2.3: Auto-optimization based on usage patterns
    fn maybe_optimize_pool(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_optimization) < self.optimization_interval {
            return;
        }

        self.last_optimization = now;

        // Calculate current hit rate
        let current_hit_rate = self.calculate_hit_rate();
        self.pool_hit_rate_history.push(current_hit_rate);

        // Keep only recent history
        if self.pool_hit_rate_history.len() > 10 {
            self.pool_hit_rate_history.remove(0);
        }

        // Adjust target pool size based on performance
        let avg_hit_rate = self.pool_hit_rate_history.iter().sum::<f64>()
            / self.pool_hit_rate_history.len() as f64;

        if avg_hit_rate < self.efficiency_target - 0.1 {
            // Hit rate too low - increase target size
            self.target_pool_size = (self.target_pool_size + 2).min(self.max_pool_size);
        } else if avg_hit_rate > self.efficiency_target + 0.1 && self.target_pool_size > 4 {
            // Hit rate very high - can reduce target size
            self.target_pool_size = (self.target_pool_size - 1).max(4);
        }
    }

    /// Calculate current pool hit rate
    fn calculate_hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.total_reused as f64 / self.total_requests as f64
    }

    /// Calculate efficiency score for pool management
    fn calculate_efficiency_score(&self) -> f64 {
        let hit_rate = self.calculate_hit_rate();
        let size_efficiency = if self.max_pool_size > 0 {
            1.0 - (self.available_panels.len() as f64 / self.max_pool_size as f64)
        } else {
            1.0
        };

        // Combine hit rate and size efficiency
        (hit_rate * 0.7) + (size_efficiency * 0.3)
    }

    /// Clear all panels
    pub fn clear_all(&mut self) {
        // Hide all available panels
        for panel in &self.available_panels {
            panel.show(false);
        }

        // Clear all collections and reset stats
        self.available_panels.clear();

        // TASK 2.3: Reset optimization state but keep configuration
        self.pool_hit_rate_history.clear();
        self.last_optimization = Instant::now();
    }

    /// PHASE 2B: Enhanced pool statistics with optimization metrics
    pub fn get_pool_stats(&self) -> PoolStats {
        PoolStats {
            available_count: self.available_panels.len(),
            active_count: self.total_created - self.available_panels.len(), // Approximate active count
            target_size: self.target_pool_size,
            max_size: self.max_pool_size,
            total_created: self.total_created,
            total_reused: self.total_reused,
            total_requests: self.total_requests,
            hit_rate: self.calculate_hit_rate(),
            efficiency_score: self.calculate_efficiency_score(),
            utilization: if self.target_pool_size > 0 {
                self.available_panels.len() as f64 / self.target_pool_size as f64
            } else {
                0.0
            },
        }
    }
}

/// PHASE 2B: Enhanced statistics with comprehensive metrics
pub struct PoolStats {
    pub available_count: usize,
    pub active_count: usize,
    pub target_size: usize,
    pub max_size: usize,
    pub total_created: usize,
    pub total_reused: usize,
    pub total_requests: usize,
    pub hit_rate: f64,
    pub efficiency_score: f64,
    pub utilization: f64,
}

impl PoolStats {
    /// Check if the pool is in a healthy state
    pub fn is_healthy(&self) -> bool {
        self.hit_rate > 0.7 && self.utilization < 0.9 && self.efficiency_score > 0.6
    }

    /// Get performance rating (Poor, Good, Excellent)
    pub fn performance_rating(&self) -> &'static str {
        if self.efficiency_score > 0.8 {
            "Excellent"
        } else if self.efficiency_score > 0.6 {
            "Good"
        } else {
            "Poor"
        }
    }

    /// Calculate efficiency score (0.0 to 1.0) - enhanced version
    pub fn efficiency_score(&self) -> f64 {
        self.efficiency_score
    }
}
