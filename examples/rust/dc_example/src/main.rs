use wxdragon::prelude::*;

struct DrawingPanel {
    panel: Panel,
}

impl DrawingPanel {
    fn new(parent: &Frame) -> Self {
        let panel = PanelBuilder::new(parent).build();
        panel.set_background_style(BackgroundStyle::Paint);

        // Register the paint handler with a move closure
        panel.on_paint(move |_event| {
            // Create a PaintDC when handling paint events
            let dc = AutoBufferedPaintDC::new(&panel);

            // Clear the DC with a white background
            dc.set_background(Colour::rgb(255, 255, 255));
            dc.set_background_mode(BackgroundMode::Solid);
            dc.clear();

            // Get the size of the panel
            let (width, height) = dc.get_size();

            // Draw a red rectangle
            dc.set_pen(Colour::rgb(255, 0, 0), 2, PenStyle::Solid);
            dc.set_brush(Colour::rgb(255, 200, 200), BrushStyle::Solid);
            dc.draw_rectangle(10, 10, 100, 50);

            // Draw a blue circle
            dc.set_pen(Colour::rgb(0, 0, 255), 2, PenStyle::Solid);
            dc.set_brush(Colour::rgb(200, 200, 255), BrushStyle::Solid);
            dc.draw_circle(width - 60, height - 60, 50);

            // Draw a green line
            dc.set_pen(Colour::rgb(0, 150, 0), 3, PenStyle::Solid);
            dc.draw_line(10, height - 10, width - 10, 10);

            // Draw some text
            dc.set_text_foreground(Colour::rgb(0, 0, 0));
            dc.draw_text("Hello, wxDragon DC!", 20, 80);

            // Draw a yellow ellipse with crosshatch pattern
            dc.set_pen(Colour::rgb(150, 150, 0), 1, PenStyle::Solid);
            dc.set_brush(Colour::rgb(255, 255, 0), BrushStyle::CrossHatch);
            dc.draw_ellipse(150, 120, 100, 60);
        });

        // Also handle SIZE events to refresh when the window size changes
        panel.on_size(move |_event| {
            // Force a repaint when window size changes
            panel.refresh(true, None);
        });

        Self { panel }
    }
}

impl std::ops::Deref for DrawingPanel {
    type Target = Panel;

    fn deref(&self) -> &Self::Target {
        &self.panel
    }
}

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("wxDragon DC Example")
            .with_size(Size::new(800, 600))
            .with_position(Point::new(100, 100))
            .build();

        let drawing_panel = DrawingPanel::new(&frame);

        // Initial paint
        drawing_panel.refresh(true, None);

        frame.show(true);
    });
}
