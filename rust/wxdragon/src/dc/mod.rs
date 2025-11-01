/// Background mode constants for device contexts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundMode {
    /// Transparent background mode
    Transparent,
    /// Solid background mode
    Solid,
}

impl BackgroundMode {
    /// Convert to the raw FFI value
    pub fn to_raw(&self) -> i32 {
        match self {
            BackgroundMode::Transparent => wxdragon_sys::WXD_TRANSPARENT as i32,
            BackgroundMode::Solid => wxdragon_sys::WXD_SOLID as i32,
        }
    }
}

widget_style_enum!(
    name: PenStyle,
    doc: "Style flags for DC pen.",
    variants: {
        Solid: wxdragon_sys::WXD_PENSTYLE_SOLID, "Solid line style.",
        Dot: wxdragon_sys::WXD_PENSTYLE_DOT, "Dotted line style.",
        LongDash: wxdragon_sys::WXD_PENSTYLE_LONG_DASH, "Long dashed line style.",
        ShortDash: wxdragon_sys::WXD_PENSTYLE_SHORT_DASH, "Short dashed line style.",
        DotDash: wxdragon_sys::WXD_PENSTYLE_DOT_DASH, "Dot and dash line style.",
        Transparent: wxdragon_sys::WXD_PENSTYLE_TRANSPARENT, "Transparent pen.",
        Stipple: wxdragon_sys::WXD_PENSTYLE_STIPPLE, "Stippled pen.",
        UserDash: wxdragon_sys::WXD_PENSTYLE_USER_DASH, "User-defined dash pattern.",
        BDiagonalHatch: wxdragon_sys::WXD_PENSTYLE_BDIAGONAL_HATCH, "Backward diagonal hatch pattern.",
        CrossDiagHatch: wxdragon_sys::WXD_PENSTYLE_CROSSDIAG_HATCH, "Cross-diagonal hatch pattern.",
        FDiagonalHatch: wxdragon_sys::WXD_PENSTYLE_FDIAGONAL_HATCH, "Forward diagonal hatch pattern.",
        CrossHatch: wxdragon_sys::WXD_PENSTYLE_CROSS_HATCH, "Cross hatch pattern.",
        HorizontalHatch: wxdragon_sys::WXD_PENSTYLE_HORIZONTAL_HATCH, "Horizontal hatch pattern.",
        VerticalHatch: wxdragon_sys::WXD_PENSTYLE_VERTICAL_HATCH, "Vertical hatch pattern."
    },
    default_variant: Solid
);

widget_style_enum!(
    name: BrushStyle,
    doc: "Style flags for DC brush.",
    variants: {
        Solid: wxdragon_sys::WXD_BRUSHSTYLE_SOLID, "Solid brush.",
        Transparent: wxdragon_sys::WXD_BRUSHSTYLE_TRANSPARENT, "Transparent brush.",
        BDiagonalHatch: wxdragon_sys::WXD_BRUSHSTYLE_BDIAGONAL_HATCH, "Backward diagonal hatch pattern.",
        CrossDiagHatch: wxdragon_sys::WXD_BRUSHSTYLE_CROSSDIAG_HATCH, "Cross-diagonal hatch pattern.",
        FDiagonalHatch: wxdragon_sys::WXD_BRUSHSTYLE_FDIAGONAL_HATCH, "Forward diagonal hatch pattern.",
        CrossHatch: wxdragon_sys::WXD_BRUSHSTYLE_CROSS_HATCH, "Cross hatch pattern.",
        HorizontalHatch: wxdragon_sys::WXD_BRUSHSTYLE_HORIZONTAL_HATCH, "Horizontal hatch pattern.",
        VerticalHatch: wxdragon_sys::WXD_BRUSHSTYLE_VERTICAL_HATCH, "Vertical hatch pattern.",
        Stipple: wxdragon_sys::WXD_BRUSHSTYLE_STIPPLE, "Stippled brush.",
        StippleMaskOpaque: wxdragon_sys::WXD_BRUSHSTYLE_STIPPLE_MASK_OPAQUE, "Stippled brush with opaque mask.",
        StippleMask: wxdragon_sys::WXD_BRUSHSTYLE_STIPPLE_MASK, "Stippled brush with mask."
    },
    default_variant: Solid
);

/// Polygon fill modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonFillMode {
    /// Odd-even fill rule
    OddEven,
    /// Winding fill rule
    Winding,
}

impl PolygonFillMode {
    /// Convert to the raw FFI value
    pub fn to_raw(&self) -> i32 {
        match self {
            PolygonFillMode::OddEven => wxdragon_sys::WXD_ODDEVEN_RULE as i32,
            PolygonFillMode::Winding => wxdragon_sys::WXD_WINDING_RULE as i32,
        }
    }
}

/// Flood fill modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloodFillMode {
    /// Fill surface with same color
    Surface,
    /// Fill until border color
    Border,
}

impl FloodFillMode {
    /// Convert to the raw FFI value
    pub fn to_raw(&self) -> i32 {
        match self {
            FloodFillMode::Surface => wxdragon_sys::WXD_FLOOD_SURFACE as i32,
            FloodFillMode::Border => wxdragon_sys::WXD_FLOOD_BORDER as i32,
        }
    }
}

/// Logical drawing functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalFunction {
    Clear,
    Xor,
    Invert,
    OrReverse,
    AndReverse,
    Copy,
    And,
    AndInvert,
    NoOp,
    Nor,
    Equiv,
    SrcInvert,
    OrInvert,
    Nand,
    Or,
    Set,
}

impl LogicalFunction {
    /// Convert to the raw FFI value
    pub fn to_raw(&self) -> i32 {
        match self {
            LogicalFunction::Clear => wxdragon_sys::WXD_CLEAR as i32,
            LogicalFunction::Xor => wxdragon_sys::WXD_XOR as i32,
            LogicalFunction::Invert => wxdragon_sys::WXD_INVERT as i32,
            LogicalFunction::OrReverse => wxdragon_sys::WXD_OR_REVERSE as i32,
            LogicalFunction::AndReverse => wxdragon_sys::WXD_AND_REVERSE as i32,
            LogicalFunction::Copy => wxdragon_sys::WXD_COPY as i32,
            LogicalFunction::And => wxdragon_sys::WXD_AND as i32,
            LogicalFunction::AndInvert => wxdragon_sys::WXD_AND_INVERT as i32,
            LogicalFunction::NoOp => wxdragon_sys::WXD_NO_OP as i32,
            LogicalFunction::Nor => wxdragon_sys::WXD_NOR as i32,
            LogicalFunction::Equiv => wxdragon_sys::WXD_EQUIV as i32,
            LogicalFunction::SrcInvert => wxdragon_sys::WXD_SRC_INVERT as i32,
            LogicalFunction::OrInvert => wxdragon_sys::WXD_OR_INVERT as i32,
            LogicalFunction::Nand => wxdragon_sys::WXD_NAND as i32,
            LogicalFunction::Or => wxdragon_sys::WXD_OR as i32,
            LogicalFunction::Set => wxdragon_sys::WXD_SET as i32,
        }
    }

    /// Convert from raw FFI value
    pub fn from_raw(value: i32) -> Self {
        match value {
            _ if value == wxdragon_sys::WXD_CLEAR as i32 => LogicalFunction::Clear,
            _ if value == wxdragon_sys::WXD_XOR as i32 => LogicalFunction::Xor,
            _ if value == wxdragon_sys::WXD_INVERT as i32 => LogicalFunction::Invert,
            _ if value == wxdragon_sys::WXD_OR_REVERSE as i32 => LogicalFunction::OrReverse,
            _ if value == wxdragon_sys::WXD_AND_REVERSE as i32 => LogicalFunction::AndReverse,
            _ if value == wxdragon_sys::WXD_COPY as i32 => LogicalFunction::Copy,
            _ if value == wxdragon_sys::WXD_AND as i32 => LogicalFunction::And,
            _ if value == wxdragon_sys::WXD_AND_INVERT as i32 => LogicalFunction::AndInvert,
            _ if value == wxdragon_sys::WXD_NO_OP as i32 => LogicalFunction::NoOp,
            _ if value == wxdragon_sys::WXD_NOR as i32 => LogicalFunction::Nor,
            _ if value == wxdragon_sys::WXD_EQUIV as i32 => LogicalFunction::Equiv,
            _ if value == wxdragon_sys::WXD_SRC_INVERT as i32 => LogicalFunction::SrcInvert,
            _ if value == wxdragon_sys::WXD_OR_INVERT as i32 => LogicalFunction::OrInvert,
            _ if value == wxdragon_sys::WXD_NAND as i32 => LogicalFunction::Nand,
            _ if value == wxdragon_sys::WXD_OR as i32 => LogicalFunction::Or,
            _ if value == wxdragon_sys::WXD_SET as i32 => LogicalFunction::Set,
            _ => LogicalFunction::Copy,
        }
    }
}

/// Mapping modes for coordinate systems
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapMode {
    Text,
    Lometric,
    Twips,
    Metric,
}

impl MapMode {
    /// Convert to the raw FFI value
    pub fn to_raw(&self) -> i32 {
        match self {
            MapMode::Text => wxdragon_sys::WXD_MM_TEXT as i32,
            MapMode::Lometric => wxdragon_sys::WXD_MM_LOMETRIC as i32,
            MapMode::Twips => wxdragon_sys::WXD_MM_TWIPS as i32,
            MapMode::Metric => wxdragon_sys::WXD_MM_METRIC as i32,
        }
    }

    /// Convert from raw FFI value
    pub fn from_raw(value: i32) -> Self {
        match value {
            _ if value == wxdragon_sys::WXD_MM_TEXT as i32 => MapMode::Text,
            _ if value == wxdragon_sys::WXD_MM_LOMETRIC as i32 => MapMode::Lometric,
            _ if value == wxdragon_sys::WXD_MM_TWIPS as i32 => MapMode::Twips,
            _ if value == wxdragon_sys::WXD_MM_METRIC as i32 => MapMode::Metric,
            _ => MapMode::Text,
        }
    }
}

/// Text alignment constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextAlignment {
    bits: i32,
}

impl TextAlignment {
    pub const INVALID: Self = Self {
        bits: wxdragon_sys::WXD_ALIGN_INVALID as i32,
    };
    pub const LEFT: Self = Self {
        bits: wxdragon_sys::WXD_ALIGN_LEFT as i32,
    };
    pub const TOP: Self = Self {
        bits: wxdragon_sys::WXD_ALIGN_TOP as i32,
    };
    pub const RIGHT: Self = Self {
        bits: wxdragon_sys::WXD_ALIGN_RIGHT as i32,
    };
    pub const BOTTOM: Self = Self {
        bits: wxdragon_sys::WXD_ALIGN_BOTTOM as i32,
    };
    pub const CENTER_HORIZONTAL: Self = Self {
        bits: wxdragon_sys::WXD_ALIGN_CENTRE_HORIZONTAL as i32,
    };
    pub const CENTER_VERTICAL: Self = Self {
        bits: wxdragon_sys::WXD_ALIGN_CENTRE_VERTICAL as i32,
    };
    pub const CENTER: Self = Self {
        bits: wxdragon_sys::WXD_ALIGN_CENTRE as i32,
    };

    pub const fn bits(&self) -> i32 {
        self.bits
    }

    pub const fn from_bits(bits: i32) -> Self {
        Self { bits }
    }
}

impl std::ops::BitOr for TextAlignment {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl std::ops::BitOrAssign for TextAlignment {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

/// Gradient direction constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientDirection {
    North,
    South,
    East,
    West,
}

impl GradientDirection {
    /// Convert to the raw FFI value
    pub fn to_raw(&self) -> i32 {
        match self {
            GradientDirection::North => wxdragon_sys::WXD_NORTH as i32,
            GradientDirection::South => wxdragon_sys::WXD_SOUTH as i32,
            GradientDirection::East => wxdragon_sys::WXD_EAST as i32,
            GradientDirection::West => wxdragon_sys::WXD_WEST as i32,
        }
    }
}

/// Point structure for drawing operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Point> for wxdragon_sys::wxd_Point {
    fn from(point: Point) -> Self {
        wxdragon_sys::wxd_Point { x: point.x, y: point.y }
    }
}

impl From<wxdragon_sys::wxd_Point> for Point {
    fn from(point: wxdragon_sys::wxd_Point) -> Self {
        Point { x: point.x, y: point.y }
    }
}

/// Rectangle structure for drawing operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }
}

impl From<Rect> for wxdragon_sys::wxd_Rect {
    fn from(rect: Rect) -> Self {
        wxdragon_sys::wxd_Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }
}

impl From<wxdragon_sys::wxd_Rect> for Rect {
    fn from(rect: wxdragon_sys::wxd_Rect) -> Self {
        Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }
}

pub mod auto_buffered_paint_dc;
pub mod client_dc;
pub mod memory_dc;
pub mod paint_dc;
pub mod screen_dc;
pub mod window_dc;

pub use auto_buffered_paint_dc::AutoBufferedPaintDC;
pub use client_dc::ClientDC;
pub use memory_dc::MemoryDC;
pub use paint_dc::PaintDC;
pub use screen_dc::ScreenDC;
pub use window_dc::WindowDC;

// Re-export for convenience
pub use crate::bitmap::Bitmap;
pub use crate::color::Colour;
pub use crate::font::Font;

/// Configuration for a blit operation
#[derive(Debug, Clone, Copy)]
pub struct BlitConfig {
    pub dest_x: i32,
    pub dest_y: i32,
    pub width: i32,
    pub height: i32,
    pub src_x: i32,
    pub src_y: i32,
    pub logical_func: LogicalFunction,
    pub use_mask: bool,
    pub src_mask_x: i32,
    pub src_mask_y: i32,
}

impl BlitConfig {
    pub fn new(dest_x: i32, dest_y: i32, width: i32, height: i32, src_x: i32, src_y: i32) -> Self {
        Self {
            dest_x,
            dest_y,
            width,
            height,
            src_x,
            src_y,
            logical_func: LogicalFunction::Copy,
            use_mask: false,
            src_mask_x: -1,
            src_mask_y: -1,
        }
    }

    pub fn with_logical_func(mut self, logical_func: LogicalFunction) -> Self {
        self.logical_func = logical_func;
        self
    }

    pub fn with_mask(mut self, src_mask_x: i32, src_mask_y: i32) -> Self {
        self.use_mask = true;
        self.src_mask_x = src_mask_x;
        self.src_mask_y = src_mask_y;
        self
    }
}

/// Configuration for a stretch blit operation
#[derive(Debug, Clone, Copy)]
pub struct StretchBlitConfig {
    pub dest_x: i32,
    pub dest_y: i32,
    pub dest_width: i32,
    pub dest_height: i32,
    pub src_x: i32,
    pub src_y: i32,
    pub src_width: i32,
    pub src_height: i32,
    pub logical_func: LogicalFunction,
    pub use_mask: bool,
    pub src_mask_x: i32,
    pub src_mask_y: i32,
}

impl StretchBlitConfig {
    pub fn with_logical_func(mut self, logical_func: LogicalFunction) -> Self {
        self.logical_func = logical_func;
        self
    }

    pub fn with_mask(mut self, src_mask_x: i32, src_mask_y: i32) -> Self {
        self.use_mask = true;
        self.src_mask_x = src_mask_x;
        self.src_mask_y = src_mask_y;
        self
    }
}

/// Common trait implemented by all device context types
pub trait DeviceContext {
    /// Get a pointer to the underlying DC
    fn dc_ptr(&self) -> *mut wxdragon_sys::wxd_DC_t;

    /// Clear the device context
    fn clear(&self) {
        unsafe {
            wxdragon_sys::wxd_DC_Clear(self.dc_ptr());
        }
    }

    /// Set the background color of the device context
    fn set_background(&self, colour: Colour) {
        unsafe {
            wxdragon_sys::wxd_DC_SetBackground(self.dc_ptr(), colour.into());
        }
    }

    /// Set the background mode of the device context
    fn set_background_mode(&self, mode: BackgroundMode) {
        unsafe {
            wxdragon_sys::wxd_DC_SetBackgroundMode(self.dc_ptr(), mode.to_raw());
        }
    }

    /// Set the text background color
    fn set_text_background(&self, colour: Colour) {
        unsafe {
            wxdragon_sys::wxd_DC_SetTextBackground(self.dc_ptr(), colour.into());
        }
    }

    /// Set the text foreground color
    fn set_text_foreground(&self, colour: Colour) {
        unsafe {
            wxdragon_sys::wxd_DC_SetTextForeground(self.dc_ptr(), colour.into());
        }
    }

    /// Set the font for text drawing
    fn set_font(&self, font: &Font) {
        unsafe {
            wxdragon_sys::wxd_DC_SetFont(self.dc_ptr(), font.as_ptr());
        }
    }

    /// Set the pen for drawing outlines
    fn set_pen(&self, colour: Colour, width: i32, style: PenStyle) {
        unsafe {
            wxdragon_sys::wxd_DC_SetPen(self.dc_ptr(), colour.into(), width, style.bits() as i32);
        }
    }

    /// Set the brush for filling shapes
    fn set_brush(&self, colour: Colour, style: BrushStyle) {
        unsafe {
            wxdragon_sys::wxd_DC_SetBrush(self.dc_ptr(), colour.into(), style.bits() as i32);
        }
    }

    /// Draw a point at the specified coordinates
    fn draw_point(&self, x: i32, y: i32) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawPoint(self.dc_ptr(), x, y);
        }
    }

    /// Draw a line from (x1, y1) to (x2, y2)
    fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawLine(self.dc_ptr(), x1, y1, x2, y2);
        }
    }

    /// Draw a rectangle with the specified dimensions
    fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawRectangle(self.dc_ptr(), x, y, width, height);
        }
    }

    /// Draw a circle with the specified center and radius
    fn draw_circle(&self, x: i32, y: i32, radius: i32) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawCircle(self.dc_ptr(), x, y, radius);
        }
    }

    /// Draw an ellipse inside the specified rectangle
    fn draw_ellipse(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawEllipse(self.dc_ptr(), x, y, width, height);
        }
    }

    /// Draw a rounded rectangle with the specified dimensions and corner radius
    fn draw_rounded_rectangle(&self, x: i32, y: i32, width: i32, height: i32, radius: f64) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawRoundedRectangle(self.dc_ptr(), x, y, width, height, radius);
        }
    }

    /// Draw text at the specified position
    fn draw_text(&self, text: &str, x: i32, y: i32) {
        use std::ffi::CString;
        if let Ok(c_text) = CString::new(text) {
            unsafe {
                wxdragon_sys::wxd_DC_DrawText(self.dc_ptr(), c_text.as_ptr(), x, y);
            }
        }
    }

    /// Draw a bitmap at the specified position
    fn draw_bitmap(&self, bitmap: &Bitmap, x: i32, y: i32, transparent: bool) {
        unsafe { wxdragon_sys::wxd_DC_DrawBitmap(self.dc_ptr(), bitmap.as_const_ptr(), x, y, transparent) };
    }

    /// Get the size of the device context
    fn get_size(&self) -> (i32, i32) {
        unsafe {
            let size = wxdragon_sys::wxd_DC_GetSize(self.dc_ptr());
            (size.width, size.height)
        }
    }

    /// Get the text extent (width and height) for the specified string
    fn get_text_extent(&self, text: &str) -> (i32, i32) {
        use std::ffi::CString;
        if let Ok(c_text) = CString::new(text) {
            let mut width = 0;
            let mut height = 0;
            unsafe { wxdragon_sys::wxd_DC_GetTextExtent(self.dc_ptr(), c_text.as_ptr(), &mut width, &mut height) };
            (width, height)
        } else {
            (0, 0)
        }
    }

    /// Set a clipping region to restrict drawing operations
    fn set_clipping_region(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            wxdragon_sys::wxd_DC_SetClippingRegion(self.dc_ptr(), x, y, width, height);
        }
    }

    /// Remove the current clipping region
    fn destroy_clipping_region(&self) {
        unsafe { wxdragon_sys::wxd_DC_DestroyClippingRegion(self.dc_ptr()) };
    }

    /// Draw a polygon using the specified points
    fn draw_polygon(&self, points: &[Point], x_offset: i32, y_offset: i32, fill_mode: PolygonFillMode) {
        if points.is_empty() {
            return;
        }

        let mut ffi_points: Vec<wxdragon_sys::wxd_Point> = points.iter().map(|p| (*p).into()).collect();

        unsafe {
            wxdragon_sys::wxd_DC_DrawPolygon(
                self.dc_ptr(),
                ffi_points.len() as i32,
                ffi_points.as_mut_ptr(),
                x_offset,
                y_offset,
                fill_mode.to_raw(),
            )
        };
    }

    /// Draw an elliptic arc
    fn draw_elliptic_arc(&self, x: i32, y: i32, width: i32, height: i32, start_angle: f64, end_angle: f64) {
        unsafe { wxdragon_sys::wxd_DC_DrawEllipticArc(self.dc_ptr(), x, y, width, height, start_angle, end_angle) };
    }

    /// Draw multiple connected lines
    fn draw_lines(&self, points: &[Point], x_offset: i32, y_offset: i32) {
        if points.is_empty() {
            return;
        }

        let mut ffi_points: Vec<wxdragon_sys::wxd_Point> = points.iter().map(|p| (*p).into()).collect();

        unsafe {
            wxdragon_sys::wxd_DC_DrawLines(
                self.dc_ptr(),
                ffi_points.len() as i32,
                ffi_points.as_mut_ptr(),
                x_offset,
                y_offset,
            )
        };
    }

    /// Draw an arc from (x1, y1) to (x2, y2) with center at (xc, yc)
    fn draw_arc(&self, x1: i32, y1: i32, x2: i32, y2: i32, xc: i32, yc: i32) {
        unsafe {
            wxdragon_sys::wxd_DC_DrawArc(self.dc_ptr(), x1, y1, x2, y2, xc, yc);
        }
    }

    /// Draw a smooth spline through the given points
    fn draw_spline(&self, points: &[Point]) {
        if points.is_empty() {
            return;
        }

        let mut ffi_points: Vec<wxdragon_sys::wxd_Point> = points.iter().map(|p| (*p).into()).collect();

        unsafe { wxdragon_sys::wxd_DC_DrawSpline(self.dc_ptr(), ffi_points.len() as i32, ffi_points.as_mut_ptr()) };
    }

    /// Draw rotated text at the specified position
    fn draw_rotated_text(&self, text: &str, x: i32, y: i32, angle: f64) {
        use std::ffi::CString;
        if let Ok(c_text) = CString::new(text) {
            unsafe { wxdragon_sys::wxd_DC_DrawRotatedText(self.dc_ptr(), c_text.as_ptr(), x, y, angle) };
        }
    }

    /// Draw text with alignment within a rectangle
    fn draw_label(&self, text: &str, rect: Rect, alignment: TextAlignment, index_accel: i32) {
        use std::ffi::CString;
        if let Ok(c_text) = CString::new(text) {
            unsafe { wxdragon_sys::wxd_DC_DrawLabel(self.dc_ptr(), c_text.as_ptr(), rect.into(), alignment.bits(), index_accel) };
        }
    }

    /// Copy a portion of one DC to another using configuration struct
    fn blit(&self, source: &dyn DeviceContext, config: BlitConfig) -> bool {
        unsafe {
            wxdragon_sys::wxd_DC_Blit(
                self.dc_ptr(),
                config.dest_x,
                config.dest_y,
                config.width,
                config.height,
                source.dc_ptr(),
                config.src_x,
                config.src_y,
                config.logical_func.to_raw(),
                config.use_mask,
                config.src_mask_x,
                config.src_mask_y,
            )
        }
    }

    /// Copy and stretch a portion of one DC to another using configuration struct
    fn stretch_blit(&self, source: &dyn DeviceContext, config: StretchBlitConfig) -> bool {
        unsafe {
            wxdragon_sys::wxd_DC_StretchBlit(
                self.dc_ptr(),
                config.dest_x,
                config.dest_y,
                config.dest_width,
                config.dest_height,
                source.dc_ptr(),
                config.src_x,
                config.src_y,
                config.src_width,
                config.src_height,
                config.logical_func.to_raw(),
                config.use_mask,
                config.src_mask_x,
                config.src_mask_y,
            )
        }
    }

    /// Set clipping region from a set of points
    fn set_clipping_region_from_points(&self, points: &[Point]) {
        if points.is_empty() {
            return;
        }

        let mut ffi_points: Vec<wxdragon_sys::wxd_Point> = points.iter().map(|p| (*p).into()).collect();

        unsafe {
            wxdragon_sys::wxd_DC_SetClippingRegionFromPoints(self.dc_ptr(), ffi_points.len() as i32, ffi_points.as_mut_ptr())
        };
    }

    /// Get the clipping box coordinates
    fn get_clipping_box(&self) -> Rect {
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut height = 0;

        unsafe { wxdragon_sys::wxd_DC_GetClippingBox(self.dc_ptr(), &mut x, &mut y, &mut width, &mut height) };

        Rect::new(x, y, width, height)
    }

    /// Set the device origin
    fn set_device_origin(&self, x: i32, y: i32) {
        unsafe { wxdragon_sys::wxd_DC_SetDeviceOrigin(self.dc_ptr(), x, y) };
    }

    /// Set the logical origin
    fn set_logical_origin(&self, x: i32, y: i32) {
        unsafe { wxdragon_sys::wxd_DC_SetLogicalOrigin(self.dc_ptr(), x, y) };
    }

    /// Set the user scale factors
    fn set_user_scale(&self, x_scale: f64, y_scale: f64) {
        unsafe { wxdragon_sys::wxd_DC_SetUserScale(self.dc_ptr(), x_scale, y_scale) };
    }

    /// Set the logical scale factors
    fn set_logical_scale(&self, x_scale: f64, y_scale: f64) {
        unsafe { wxdragon_sys::wxd_DC_SetLogicalScale(self.dc_ptr(), x_scale, y_scale) };
    }

    /// Set the mapping mode
    fn set_map_mode(&self, mode: MapMode) {
        unsafe { wxdragon_sys::wxd_DC_SetMapMode(self.dc_ptr(), mode.to_raw()) };
    }

    /// Get the device origin
    fn get_device_origin(&self) -> Point {
        let origin = unsafe { wxdragon_sys::wxd_DC_GetDeviceOrigin(self.dc_ptr()) };
        origin.into()
    }

    /// Get the logical origin
    fn get_logical_origin(&self) -> Point {
        let origin = unsafe { wxdragon_sys::wxd_DC_GetLogicalOrigin(self.dc_ptr()) };
        origin.into()
    }

    /// Get the user scale factors
    fn get_user_scale(&self) -> (f64, f64) {
        let mut x_scale = 0.0;
        let mut y_scale = 0.0;

        unsafe { wxdragon_sys::wxd_DC_GetUserScale(self.dc_ptr(), &mut x_scale, &mut y_scale) };

        (x_scale, y_scale)
    }

    /// Get the logical scale factors
    fn get_logical_scale(&self) -> (f64, f64) {
        let mut x_scale = 0.0;
        let mut y_scale = 0.0;

        unsafe { wxdragon_sys::wxd_DC_GetLogicalScale(self.dc_ptr(), &mut x_scale, &mut y_scale) };

        (x_scale, y_scale)
    }

    /// Get the current mapping mode
    fn get_map_mode(&self) -> MapMode {
        let mode = unsafe { wxdragon_sys::wxd_DC_GetMapMode(self.dc_ptr()) };
        MapMode::from_raw(mode)
    }

    /// Convert device coordinates to logical coordinates (X)
    fn device_to_logical_x(&self, x: i32) -> i32 {
        unsafe { wxdragon_sys::wxd_DC_DeviceToLogicalX(self.dc_ptr(), x) }
    }

    /// Convert device coordinates to logical coordinates (Y)
    fn device_to_logical_y(&self, y: i32) -> i32 {
        unsafe { wxdragon_sys::wxd_DC_DeviceToLogicalY(self.dc_ptr(), y) }
    }

    /// Convert logical coordinates to device coordinates (X)
    fn logical_to_device_x(&self, x: i32) -> i32 {
        unsafe { wxdragon_sys::wxd_DC_LogicalToDeviceX(self.dc_ptr(), x) }
    }

    /// Convert logical coordinates to device coordinates (Y)
    fn logical_to_device_y(&self, y: i32) -> i32 {
        unsafe { wxdragon_sys::wxd_DC_LogicalToDeviceY(self.dc_ptr(), y) }
    }

    /// Get the size in millimeters
    fn get_size_mm(&self) -> (i32, i32) {
        let size = unsafe { wxdragon_sys::wxd_DC_GetSizeMM(self.dc_ptr()) };
        (size.width, size.height)
    }

    /// Get full text extent including descent and external leading
    fn get_full_text_extent(&self, text: &str, font: Option<&Font>) -> (i32, i32, i32, i32) {
        use std::ffi::CString;
        if let Ok(c_text) = CString::new(text) {
            let mut width = 0;
            let mut height = 0;
            let mut descent = 0;
            let mut external_leading = 0;

            unsafe {
                wxdragon_sys::wxd_DC_GetFullTextExtent(
                    self.dc_ptr(),
                    c_text.as_ptr(),
                    &mut width,
                    &mut height,
                    &mut descent,
                    &mut external_leading,
                    font.map(|f| f.as_ptr()).unwrap_or(std::ptr::null_mut()),
                );
            }
            (width, height, descent, external_leading)
        } else {
            (0, 0, 0, 0)
        }
    }

    /// Get text extent for multi-line text
    fn get_multi_line_text_extent(&self, text: &str, font: Option<&Font>) -> (i32, i32, i32) {
        use std::ffi::CString;
        if let Ok(c_text) = CString::new(text) {
            let mut width = 0;
            let mut height = 0;
            let mut height_line = 0;

            unsafe {
                wxdragon_sys::wxd_DC_GetMultiLineTextExtent(
                    self.dc_ptr(),
                    c_text.as_ptr(),
                    &mut width,
                    &mut height,
                    &mut height_line,
                    font.map(|f| f.as_ptr()).unwrap_or(std::ptr::null_mut()),
                );
            }
            (width, height, height_line)
        } else {
            (0, 0, 0)
        }
    }

    /// Get the character height for the current font
    fn get_char_height(&self) -> i32 {
        unsafe { wxdragon_sys::wxd_DC_GetCharHeight(self.dc_ptr()) }
    }

    /// Get the character width for the current font
    fn get_char_width(&self) -> i32 {
        unsafe { wxdragon_sys::wxd_DC_GetCharWidth(self.dc_ptr()) }
    }

    /// Get the current background color
    fn get_background(&self) -> Colour {
        let colour = unsafe { wxdragon_sys::wxd_DC_GetBackground(self.dc_ptr()) };
        Colour::new(colour.r, colour.g, colour.b, colour.a)
    }

    /// Get the current background mode
    fn get_background_mode(&self) -> BackgroundMode {
        let mode = unsafe { wxdragon_sys::wxd_DC_GetBackgroundMode(self.dc_ptr()) };
        if mode == wxdragon_sys::WXD_TRANSPARENT as i32 {
            BackgroundMode::Transparent
        } else {
            BackgroundMode::Solid
        }
    }

    /// Get the current text background color
    fn get_text_background(&self) -> Colour {
        let colour = unsafe { wxdragon_sys::wxd_DC_GetTextBackground(self.dc_ptr()) };
        Colour::new(colour.r, colour.g, colour.b, colour.a)
    }

    /// Get the current text foreground color
    fn get_text_foreground(&self) -> Colour {
        let colour = unsafe { wxdragon_sys::wxd_DC_GetTextForeground(self.dc_ptr()) };
        Colour::new(colour.r, colour.g, colour.b, colour.a)
    }

    /// Get the pixels per inch (DPI)
    fn get_ppi(&self) -> (i32, i32) {
        let ppi = unsafe { wxdragon_sys::wxd_DC_GetPPI(self.dc_ptr()) };
        (ppi.width, ppi.height)
    }

    /// Get the content scale factor (for high-DPI displays)
    fn get_content_scale_factor(&self) -> f64 {
        unsafe { wxdragon_sys::wxd_DC_GetContentScaleFactor(self.dc_ptr()) }
    }

    /// Fill a rectangle with a linear gradient
    fn gradient_fill_linear(&self, rect: Rect, initial_colour: Colour, dest_colour: Colour, direction: GradientDirection) {
        unsafe {
            wxdragon_sys::wxd_DC_GradientFillLinear(
                self.dc_ptr(),
                rect.into(),
                initial_colour.into(),
                dest_colour.into(),
                direction.to_raw(),
            )
        };
    }

    /// Fill a rectangle with a concentric gradient
    fn gradient_fill_concentric(&self, rect: Rect, initial_colour: Colour, dest_colour: Colour, circle_center: Point) {
        unsafe {
            wxdragon_sys::wxd_DC_GradientFillConcentric(
                self.dc_ptr(),
                rect.into(),
                initial_colour.into(),
                dest_colour.into(),
                circle_center.into(),
            )
        };
    }

    /// Flood fill starting from a point
    fn flood_fill(&self, x: i32, y: i32, colour: Colour, style: FloodFillMode) -> bool {
        unsafe { wxdragon_sys::wxd_DC_FloodFill(self.dc_ptr(), x, y, colour.into(), style.to_raw()) }
    }

    /// Set the logical function for drawing operations
    fn set_logical_function(&self, function: LogicalFunction) {
        unsafe { wxdragon_sys::wxd_DC_SetLogicalFunction(self.dc_ptr(), function.to_raw()) };
    }

    /// Get the current logical function
    fn get_logical_function(&self) -> LogicalFunction {
        let function = unsafe { wxdragon_sys::wxd_DC_GetLogicalFunction(self.dc_ptr()) };
        LogicalFunction::from_raw(function)
    }
}
