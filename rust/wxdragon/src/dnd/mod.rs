//! Drag and drop functionality for wxDragon applications.
//!
//! This module provides classes for implementing drag and drop operations
//! in wxDragon applications, following the wxWidgets drag and drop pattern.

mod dropsource;
mod droptarget;
// Use the main data_object module instead of our own implementation
// mod dataobject;

pub use dropsource::DropSource;
pub use droptarget::{FileDropTarget, TextDropTarget};
// Re-export data objects from the main module
pub use crate::data_object::{BitmapDataObject, DataObject, FileDataObject, TextDataObject};

use std::fmt;

/// The result of a drag and drop operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DragResult {
    /// No effect (drag target didn't accept the data).
    None = 0,

    /// The data was copied.
    Copy = 1,

    /// The data was moved (ownership transferred).
    Move = 2,

    /// Link to the data.
    Link = 3,

    /// The drag operation was canceled by the user.
    Cancel = 4,

    /// Error in the drag operation.
    Error = 5,
}

impl From<wxdragon_sys::wxd_DragResult> for DragResult {
    fn from(value: wxdragon_sys::wxd_DragResult) -> Self {
        match value {
            wxdragon_sys::wxd_DragResult_WXD_DRAG_NONE => DragResult::None,
            wxdragon_sys::wxd_DragResult_WXD_DRAG_COPY => DragResult::Copy,
            wxdragon_sys::wxd_DragResult_WXD_DRAG_MOVE => DragResult::Move,
            wxdragon_sys::wxd_DragResult_WXD_DRAG_LINK => DragResult::Link,
            wxdragon_sys::wxd_DragResult_WXD_DRAG_CANCEL => DragResult::Cancel,
            _ => DragResult::Error,
        }
    }
}

impl From<DragResult> for wxdragon_sys::wxd_DragResult {
    fn from(val: DragResult) -> Self {
        match val {
            DragResult::None => wxdragon_sys::wxd_DragResult_WXD_DRAG_NONE,
            DragResult::Copy => wxdragon_sys::wxd_DragResult_WXD_DRAG_COPY,
            DragResult::Move => wxdragon_sys::wxd_DragResult_WXD_DRAG_MOVE,
            DragResult::Link => wxdragon_sys::wxd_DragResult_WXD_DRAG_LINK,
            DragResult::Cancel => wxdragon_sys::wxd_DragResult_WXD_DRAG_CANCEL,
            DragResult::Error => wxdragon_sys::wxd_DragResult_WXD_DRAG_ERROR,
        }
    }
}

impl fmt::Display for DragResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DragResult::None => write!(f, "None"),
            DragResult::Copy => write!(f, "Copy"),
            DragResult::Move => write!(f, "Move"),
            DragResult::Link => write!(f, "Link"),
            DragResult::Cancel => write!(f, "Cancel"),
            DragResult::Error => write!(f, "Error"),
        }
    }
}
