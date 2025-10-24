use crate::dc::DeviceContext;
use crate::window::WxWidget;

/// An automatically buffered paint DC that provides flicker-free drawing.
///
/// This DC automatically chooses between wxPaintDC (on platforms with native
/// double-buffering) and wxBufferedPaintDC (on platforms without) to provide
/// optimal performance while eliminating flicker.
///
/// Use this instead of PaintDC for smooth animations and frequent redraws.
///
/// # Usage
///
/// ```rust,no_run
/// # use wxdragon::prelude::*;
///
/// # let frame = Frame::builder()
/// #     .with_title("AutoBufferedPaintDC Example")
/// #     .with_size(Size::new(400, 300))
/// #     .build();
/// # let panel = Panel::builder(&frame).build();
/// # let panel_clone = panel.clone();
/// panel.on_paint(move |_event| {
///     let dc = AutoBufferedPaintDC::new(&panel_clone);
///     
///     // Clear the background
///     dc.set_background(Colour::rgb(255, 255, 255));
///     dc.clear();
///     
///     // Draw your animation content
///     dc.set_pen(Colour::rgb(255, 0, 0), 2, PenStyle::Solid);
///     dc.draw_circle(100, 100, 50);
/// });
/// ```
///
/// # Important Notes
///
/// For best results, make sure to call `set_background_style(BackgroundStyle::Paint)`
/// on your widget during initialization. This tells wxWidgets to not erase the
/// background automatically, which is essential for flicker-free drawing.
///
/// Also consider handling the erase background event with an empty handler to
/// prevent unwanted background erasing.
pub struct AutoBufferedPaintDC {
    dc_ptr: *mut wxdragon_sys::wxd_AutoBufferedPaintDC_t,
}

impl AutoBufferedPaintDC {
    /// Create a new AutoBufferedPaintDC for the specified window during a paint event
    ///
    /// # Arguments
    /// * `window` - The window to draw on
    ///
    /// # Note
    /// This DC must be created in response to a paint event, and only one
    /// AutoBufferedPaintDC should exist at a time for a given window.
    ///
    /// For optimal flicker-free drawing, make sure to:
    /// 1. Call `window.set_background_style(BackgroundStyle::Paint)` during initialization
    /// 2. Handle the erase background event with an empty handler (optional but recommended)
    pub fn new<W: WxWidget>(window: &W) -> Self {
        let ptr = window.handle_ptr();
        let dc_ptr = unsafe { wxdragon_sys::wxd_AutoBufferedPaintDC_Create(ptr) };
        Self { dc_ptr }
    }
}

impl DeviceContext for AutoBufferedPaintDC {
    fn dc_ptr(&self) -> *mut wxdragon_sys::wxd_DC_t {
        unsafe { wxdragon_sys::wxd_AutoBufferedPaintDC_AsDC(self.dc_ptr) }
    }
}

impl Drop for AutoBufferedPaintDC {
    fn drop(&mut self) {
        unsafe {
            wxdragon_sys::wxd_AutoBufferedPaintDC_Destroy(self.dc_ptr);
        }
    }
}
