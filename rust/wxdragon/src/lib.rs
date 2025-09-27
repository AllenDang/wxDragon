// Re-export items from the sys crate for convenience
pub use wxdragon_sys as ffi;

#[macro_use]
pub mod macros;
pub mod app;
pub mod appearance;
pub mod appprogress;
pub mod art_provider;
pub mod bitmap;
pub mod bitmap_bundle;
pub mod clipboard;
pub mod color;
pub mod cursor;
pub mod data_object;
pub mod datetime;
pub mod dc;
pub mod dialogs;
pub mod dnd;
pub mod event;
pub mod font;
pub mod font_data;
pub mod geometry;
pub mod id;
pub mod menus;
pub mod prelude;
pub mod scrollable;
pub mod sizers;
pub mod sysopt;
pub mod timer;
pub mod types;
pub mod utils;
pub mod widgets;
pub mod window;
#[cfg(feature = "xrc")]
pub mod xrc;

// Re-export the prelude for convenience - users should use this
pub use prelude::*;

// Re-export procedural macros from wxdragon-macros
#[cfg(feature = "xrc")]
pub use wxdragon_macros::include_xrc;

// Re-export XRC macros (these are macro_rules! macros, not procedural macros)
// include_xrc_dialog!, include_xrc_panel! are available via #[macro_use]

pub use widgets::dataview::{
    CustomDataViewVirtualListModel, DataViewAlign, DataViewBitmapRenderer, DataViewCellMode,
    DataViewCheckIconTextRenderer, DataViewChoiceRenderer, DataViewColumn, DataViewColumnFlag,
    DataViewCtrl, DataViewCtrlBuilder, DataViewCustomRenderer, DataViewCustomRendererBuilder,
    DataViewDateRenderer, DataViewIconTextRenderer, DataViewItem, DataViewListCtrl,
    DataViewListCtrlBuilder, DataViewListModel, DataViewModel, DataViewProgressRenderer,
    DataViewRenderer, DataViewSpinRenderer, DataViewStyle, DataViewTextRenderer,
    DataViewToggleRenderer, DataViewTreeCtrl, DataViewTreeCtrlBuilder, DataViewTreeCtrlStyle,
    DataViewVirtualListModel, RenderContext, Variant,
};

// Re-export DC functionality for custom renderers
pub use dc::{BrushStyle, DeviceContext, PenStyle};

// Re-export window functionality including downcasting
pub use window::{BackgroundStyle, ExtraWindowStyle, Window, WxWidget, WxWidgetDowncast};
