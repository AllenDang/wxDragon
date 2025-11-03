//! DataView widget family implementation.
//!
//! This module provides Rust wrappers for wxWidget's DataView controls:
//! - `DataViewCtrl`: Base control for displaying data in a customizable view.
//! - `DataViewListCtrl`: Simplified list-based DataView control.
//! - `DataViewTreeCtrl`: Tree-based DataView control.
//!
//! It also provides supporting types for renderers, columns, and data models.
//!
//! # Important Notes
//!
//! When using the DataView controls, remember these key points:
//!
//! 1. Define columns in the DataViewListModel *before* adding any rows
//! 2. Keep track of the DataViewListModel as long as the control exists
//! 3. The order of operations matters:
//!    - Create the DataViewCtrl
//!    - Add visual columns to the control
//!    - Create and configure the model
//!    - Associate the model with the control
//!    - Add data to the model

// Copyright (c) 2024 wxDragon.
// Copyright (c) 2024-2024 Jovial Linux User.
// All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//!      DataView related widgets and types.

// Make submodules public so their contents can be re-exported and used.
pub mod column;
pub mod ctrl;
pub mod enums;
pub mod events;
pub mod item;
pub mod list_ctrl;
pub mod model;
pub mod renderer;
pub mod tree_ctrl;
pub mod variant;

// Re-export key types for easier access, e.g., wxdragon::widgets::dataview::DataViewCtrl
pub use column::DataViewColumn;
pub use ctrl::{DataViewCtrl, DataViewCtrlBuilder, DataViewStyle};
pub use enums::{DataViewAlign, DataViewCellMode, DataViewColumnFlags, DataViewColumnFlags as DataViewColumnFlag};
pub use events::{DataViewEvent, DataViewEventHandler, DataViewEventType, DataViewTreeEventHandler};
pub use item::DataViewItem;
pub use list_ctrl::{DataViewListCtrl, DataViewListCtrlBuilder};
pub use model::{
    CustomDataViewTreeModel, CustomDataViewVirtualListModel, DataViewItemAttr, DataViewListModel, DataViewModel,
    DataViewVirtualListModel,
};
pub use renderer::{
    DataViewBitmapRenderer, DataViewCheckIconTextRenderer, DataViewChoiceRenderer, DataViewCustomRenderer,
    DataViewCustomRendererBuilder, DataViewDateRenderer, DataViewIconTextRenderer, DataViewProgressRenderer, DataViewRenderer,
    DataViewSpinRenderer, DataViewTextRenderer, DataViewToggleRenderer, RenderContext,
};
pub use tree_ctrl::{DataViewTreeCtrl, DataViewTreeCtrlBuilder, DataViewTreeCtrlStyle};
pub use variant::{Variant, VariantType};
