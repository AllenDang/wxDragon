//! Geometry types for wxDragon (Point, Size, Rect)
//!
//! This module contains the basic geometry types used throughout
//! the wxDragon library, providing conversions to and from the FFI types.

use wxdragon_sys as ffi;

/// Standard window position type.
///
/// Represents a point in 2D space with x, y coordinates.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new Point.
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Default position (-1, -1) that lets the system choose the position.
    pub const DEFAULT_POSITION: Point = Point { x: -1, y: -1 };
}

impl From<Point> for ffi::wxd_Point {
    fn from(p: Point) -> Self {
        ffi::wxd_Point { x: p.x, y: p.y }
    }
}

impl From<ffi::wxd_Point> for Point {
    fn from(p: ffi::wxd_Point) -> Self {
        Point { x: p.x, y: p.y }
    }
}

/// Standard window size type.
///
/// Represents a size in 2D space with width and height.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    /// Creates a new Size.
    pub fn new(width: i32, height: i32) -> Self {
        Size { width, height }
    }

    /// Default size (-1, -1) that lets the system choose the size.
    pub const DEFAULT_SIZE: Size = Size { width: -1, height: -1 };
}

impl From<Size> for ffi::wxd_Size {
    fn from(s: Size) -> Self {
        ffi::wxd_Size {
            width: s.width,
            height: s.height,
        }
    }
}

impl From<ffi::wxd_Size> for Size {
    fn from(s: ffi::wxd_Size) -> Self {
        Size {
            width: s.width,
            height: s.height,
        }
    }
}

/// Represents a rectangle with position (x, y) and dimensions (width, height).
///
/// Combines Point and Size to define a rectangular area.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    /// Creates a new rectangle.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }

    /// Creates a rectangle from a position and size.
    pub fn from_point_and_size(pos: Point, size: Size) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            width: size.width,
            height: size.height,
        }
    }

    /// Returns the position component of the rectangle.
    pub fn position(&self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Returns the size component of the rectangle.
    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}

impl From<Rect> for ffi::wxd_Rect {
    fn from(rect: Rect) -> Self {
        ffi::wxd_Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }
}

impl From<ffi::wxd_Rect> for Rect {
    fn from(wxd_rect: ffi::wxd_Rect) -> Self {
        Self {
            x: wxd_rect.x,
            y: wxd_rect.y,
            width: wxd_rect.width,
            height: wxd_rect.height,
        }
    }
}
