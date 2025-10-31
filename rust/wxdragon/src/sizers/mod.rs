// This file declares the modules within the `sizers` directory
// and potentially re-exports key types for easier access.

pub mod base;
pub mod box_sizer;
pub mod flex_grid_sizer;
pub mod grid_bag_sizer;
pub mod grid_sizer;
pub mod staticbox_sizer;
pub mod wrap_sizer;

// Re-export common types and constants
pub use base::{Orientation, Sizer, SizerFlag, WxSizer};
pub use box_sizer::{BoxSizer, BoxSizerBuilder};
pub use flex_grid_sizer::{FlexGridSizer, FlexGridSizerBuilder, FlexGrowMode};
pub use grid_bag_sizer::{DEFAULT_GB_POSITION, DEFAULT_GB_SPAN, GBPosition, GBSpan, GridBagSizer, GridBagSizerBuilder};
pub use grid_sizer::{GridSizer, GridSizerBuilder};
pub use staticbox_sizer::{StaticBoxSizer, StaticBoxSizerBuilder};
pub use wrap_sizer::{WrapSizer, WrapSizerBuilder, WrapSizerFlag};

// Note: Carefully check which constants are actually defined in box_sizer.rs
// and ensure they match the ones re-exported here.
// Specifically, ensure ALIGN_TOP, ALIGN_BOTTOM, ALIGN_CENTER_VERTICAL,
// ALIGN_CENTER_HORIZONTAL, ALIGN_CENTRE are correctly defined and exported from box_sizer.rs
// Removed ALIGN_CENTER alias re-export, use ALIGN_CENTRE directly.

// The core Sizer struct/trait is likely defined in src/sizer.rs or src/lib.rs and accessed from there.
