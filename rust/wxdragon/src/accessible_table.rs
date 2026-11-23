//! What screen readers see of a window that draws itself, such as a custom list or a status bar, through
//! [`WxWidget::set_accessible_elements`](crate::window::WxWidget::set_accessible_elements) and
//! [`WxWidget::set_accessible_table`](crate::window::WxWidget::set_accessible_table).

use crate::geometry::Rect;
use std::ffi::CString;
use wxdragon_sys as ffi;

/// A part of a window that draws itself, read by screen readers as static text, such as a status bar field or
/// one cell of a row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessibleElement {
    /// The text a screen reader reads.
    pub label: String,
    /// Where the element is, in the window's client coordinates.
    pub rect: Rect,
}

impl AccessibleElement {
    pub fn new(label: impl Into<String>, rect: Rect) -> Self {
        Self {
            label: label.into(),
            rect,
        }
    }
}

/// A row of a window that draws itself as a list or a table, such as an item of a custom list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessibleRow {
    /// The row's position among all of the rows. Only the shown rows are given, so this isn't its position among
    /// them.
    pub index: usize,
    /// Where the whole row is, in the window's client coordinates.
    pub rect: Rect,
    /// One element for each column, or just one if the window is a list. A screen reader reads them together as
    /// the row.
    pub cells: Vec<AccessibleElement>,
    pub selected: bool,
}

impl AccessibleRow {
    pub fn new(index: usize, rect: Rect, cells: Vec<AccessibleElement>) -> Self {
        Self {
            index,
            rect,
            cells,
            selected: false,
        }
    }

    pub fn with_selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

/// The C side of a slice of elements. The labels must outlive the elements that point into them.
pub(crate) struct FfiElements {
    _labels: Vec<CString>,
    pub(crate) elements: Vec<ffi::wxd_AccessibleElement>,
}

impl FfiElements {
    pub(crate) fn new(elements: &[AccessibleElement]) -> Self {
        // A C string ends at its first NUL, so one inside a label would cut it short.
        let labels: Vec<CString> = elements
            .iter()
            .map(|element| CString::new(element.label.replace('\0', "")).unwrap_or_default())
            .collect();
        let elements = elements
            .iter()
            .zip(&labels)
            .map(|(element, label)| ffi::wxd_AccessibleElement {
                label: label.as_ptr(),
                rect: element.rect.into(),
            })
            .collect();
        Self {
            _labels: labels,
            elements,
        }
    }
}

pub(crate) fn to_ffi_index(index: usize) -> i64 {
    i64::try_from(index).unwrap_or(i64::MAX)
}
