use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};
use wxdragon::dc::{AutoBufferedPaintDC, BrushStyle, PenStyle, Point, PolygonFillMode};
use wxdragon::event::WindowEventData;
use wxdragon::prelude::*;

/// Animation state for pie slice hover effect
#[derive(Debug, Clone, Copy, PartialEq)]
enum AnimationState {
    Idle,
    MoveOut,
    MoveIn,
}

/// Data for a single pie slice
#[derive(Debug, Clone)]
struct PieSlice {
    label: String,
    amount: i32,
    color: Colour,
    start_angle: f64,
    end_angle: f64,
    offset: f32, // 0.0 to 1.0, how much the slice is moved out
}

/// Shared animation data
#[derive(Debug)]
struct AnimationData {
    slices: Vec<PieSlice>,
    hovered_slice: Option<usize>,
    animation_state: AnimationState,
    animation_progress: f32, // 0.0 to 1.0
    start_time: Option<Instant>,
}

// Create the pie chart custom widget
custom_widget!(
    name: PieChart,
    fields: {
        labels: Vec<String> = vec!["Sample A".to_string(), "Sample B".to_string(), "Sample C".to_string()],
        amounts: Vec<i32> = vec![30, 45, 25],
        border_color: Colour = Colour::new(80, 80, 80, 255),
        border_width: i32 = 2,
        background_color: Colour = Colour::new(250, 250, 250, 255),
        hover_offset: i32 = 15,
        animation_duration: Duration = Duration::from_millis(200),
    },
    setup_impl: |config, panel| {
        // Set background style for flicker-free drawing
        panel.set_background_style(BackgroundStyle::Paint);

        // Initialize pie slices
        let slices = PieChart::calculate_slices(&config.labels, &config.amounts);

        // Create animation data
        let animation_data = Rc::new(RefCell::new(AnimationData {
            slices,
            hovered_slice: None,
            animation_state: AnimationState::Idle,
            animation_progress: 0.0,
            start_time: None,
        }));

        // Create timer for animations
        let timer = Rc::new(Timer::new(&panel));

        // Set up paint event
        let panel_paint = panel.clone();
        let animation_data_paint = animation_data.clone();
        let config_paint = config.clone();
        panel.on_paint(move |event| {
            let animation = animation_data_paint.borrow();
            PieChart::draw_pie_chart(&panel_paint, &config_paint, &animation);
            event.skip(true);
        });

        // Set up mouse motion event for hover detection
        let animation_data_motion = animation_data.clone();
        let panel_motion = panel.clone();
        let timer_motion = timer.clone();
        let config_motion = config.clone();
        panel.on_mouse_motion(move |event| {
            let mouse_pos = match event {
                WindowEventData::MouseMotion(ref motion_event) => {
                    motion_event.get_position()
                }
                _ => None,
            };

            if let Some(pos) = mouse_pos {
                let size = panel_motion.get_size();
                let center_x = size.width / 2;
                let center_y = size.height / 2;

                let hovered_slice = PieChart::get_slice_at_position(
                    pos.x, pos.y, center_x, center_y,
                    &animation_data_motion.borrow().slices, &config_motion
                );

                let mut animation = animation_data_motion.borrow_mut();

                if animation.hovered_slice != hovered_slice {
                    animation.hovered_slice = hovered_slice;

                    if hovered_slice.is_some() {
                        // Start move out animation
                        animation.animation_state = AnimationState::MoveOut;
                        animation.start_time = Some(Instant::now());
                        drop(animation);
                        timer_motion.start(16, false); // 60fps
                    } else {
                        // Start move in animation
                        animation.animation_state = AnimationState::MoveIn;
                        animation.start_time = Some(Instant::now());
                        drop(animation);
                        timer_motion.start(16, false); // 60fps
                    }

                    panel_motion.refresh(false, None);
                }
            }

            event.skip(true);
        });

        // Set up mouse leave event
        let animation_data_leave = animation_data.clone();
        let panel_leave = panel.clone();
        let timer_leave = timer.clone();
        panel.on_mouse_leave(move |event| {
            let mut animation = animation_data_leave.borrow_mut();

            if animation.hovered_slice.is_some() {
                animation.hovered_slice = None;
                animation.animation_state = AnimationState::MoveIn;
                animation.start_time = Some(Instant::now());
                drop(animation);

                timer_leave.start(16, false); // 60fps
                panel_leave.refresh(false, None);
            }

            event.skip(true);
        });

        // Set up timer event for animations
        let animation_data_timer = animation_data.clone();
        let config_timer = config.clone();
        let panel_timer = panel.clone();
        let timer_clone = timer.clone();
        timer.on_tick(move |event| {
            let mut animation = animation_data_timer.borrow_mut();

            if animation.animation_state == AnimationState::Idle {
                drop(animation);
                timer_clone.stop();
                return;
            }

            if let Some(start_time) = animation.start_time {
                let elapsed = start_time.elapsed();
                let duration_ms = config_timer.animation_duration.as_millis() as f32;
                let progress_ratio = (elapsed.as_millis() as f32 / duration_ms).min(1.0);

                animation.animation_progress = match animation.animation_state {
                    AnimationState::MoveOut => progress_ratio,
                    AnimationState::MoveIn => 1.0 - progress_ratio,
                    AnimationState::Idle => 0.0,
                };

                // Update slice offsets
                let hovered_slice = animation.hovered_slice;
                let progress = animation.animation_progress;
                for (i, slice) in animation.slices.iter_mut().enumerate() {
                    if Some(i) == hovered_slice {
                        slice.offset = progress;
                    } else {
                        slice.offset = 0.0;
                    }
                }

                panel_timer.refresh(false, None);

                // Animation complete
                if progress_ratio >= 1.0 {
                    animation.animation_state = AnimationState::Idle;
                    animation.start_time = None;
                    drop(animation);
                    timer_clone.stop();
                }
            }

            event.skip(true);
        });
    }
);

impl PieChart {
    /// Calculate pie slices from labels and amounts
    fn calculate_slices(labels: &[String], amounts: &[i32]) -> Vec<PieSlice> {
        let total: i32 = amounts.iter().sum();
        if total == 0 {
            return Vec::new();
        }

        let mut slices = Vec::new();
        let mut current_angle = 0.0;

        for (i, (label, &amount)) in labels.iter().zip(amounts.iter()).enumerate() {
            let percentage = amount as f64 / total as f64;
            let angle_span = percentage * 360.0;

            let slice = PieSlice {
                label: label.clone(),
                amount,
                color: Self::generate_color(i),
                start_angle: current_angle,
                end_angle: current_angle + angle_span,
                offset: 0.0,
            };

            slices.push(slice);
            current_angle += angle_span;
        }

        slices
    }

    /// Generate a color for the slice based on its index
    fn generate_color(index: usize) -> Colour {
        let colors = [
            Colour::new(255, 99, 132, 255),  // Red
            Colour::new(54, 162, 235, 255),  // Blue
            Colour::new(255, 205, 86, 255),  // Yellow
            Colour::new(75, 192, 192, 255),  // Teal
            Colour::new(153, 102, 255, 255), // Purple
            Colour::new(255, 159, 64, 255),  // Orange
            Colour::new(199, 199, 199, 255), // Grey
            Colour::new(83, 102, 255, 255),  // Indigo
        ];

        colors[index % colors.len()]
    }

    /// Check which slice is at the given position
    fn get_slice_at_position(
        x: i32,
        y: i32,
        center_x: i32,
        center_y: i32,
        slices: &[PieSlice],
        _config: &PieChartConfig,
    ) -> Option<usize> {
        let dx = x - center_x;
        let dy = y - center_y;
        let distance = ((dx * dx + dy * dy) as f64).sqrt();

        // Calculate radius (use smaller dimension minus some padding)
        let size = std::cmp::min(center_x * 2, center_y * 2);
        let radius = (size as f64 * 0.4).max(50.0);

        // Check if point is within the pie circle
        if distance > radius {
            return None;
        }

        // Calculate angle from center
        let mut angle = (dy as f64).atan2(dx as f64) * 180.0 / std::f64::consts::PI;
        if angle < 0.0 {
            angle += 360.0;
        }

        // Adjust angle to start from top (90 degrees offset)
        angle = (angle + 90.0) % 360.0;

        // Find which slice contains this angle
        for (i, slice) in slices.iter().enumerate() {
            if angle >= slice.start_angle && angle <= slice.end_angle {
                return Some(i);
            }
        }

        None
    }

    /// Draw the pie chart
    fn draw_pie_chart(panel: &Panel, config: &PieChartConfig, animation: &AnimationData) {
        let dc = AutoBufferedPaintDC::new(panel);
        let size = panel.get_size();
        let width = size.width;
        let height = size.height;

        // Clear background
        dc.set_brush(config.background_color, BrushStyle::Solid);
        dc.set_pen(Colour::new(0, 0, 0, 0), 0, PenStyle::Transparent);
        dc.draw_rectangle(0, 0, width, height);

        let center_x = width / 2;
        let center_y = height / 2;

        // Calculate radius
        let size_min = std::cmp::min(width, height);
        let radius = (size_min as f64 * 0.4).max(50.0);

        // Draw each slice
        for slice in &animation.slices {
            Self::draw_pie_slice(&dc, slice, center_x, center_y, radius, config);
        }

        // Draw labels
        dc.set_text_foreground(Colour::new(50, 50, 50, 255));
        let mut y_offset = 10;
        for slice in &animation.slices {
            let label_text = format!(
                "{}: {} ({}%)",
                slice.label,
                slice.amount,
                ((slice.end_angle - slice.start_angle) / 360.0 * 100.0) as i32
            );

            // Draw color indicator
            dc.set_brush(slice.color, BrushStyle::Solid);
            dc.set_pen(config.border_color, 1, PenStyle::Solid);
            dc.draw_rectangle(10, y_offset, 15, 15);

            // Draw label text
            dc.draw_text(&label_text, 30, y_offset);
            y_offset += 20;
        }
    }

    /// Draw a single pie slice
    fn draw_pie_slice(
        dc: &AutoBufferedPaintDC,
        slice: &PieSlice,
        center_x: i32,
        center_y: i32,
        radius: f64,
        config: &PieChartConfig,
    ) {
        let start_rad = (slice.start_angle - 90.0) * std::f64::consts::PI / 180.0;
        let end_rad = (slice.end_angle - 90.0) * std::f64::consts::PI / 180.0;

        // Calculate offset for hover effect
        let offset_distance = config.hover_offset as f64 * slice.offset as f64;
        let mid_angle = (start_rad + end_rad) / 2.0;
        let offset_x = offset_distance * mid_angle.cos();
        let offset_y = offset_distance * mid_angle.sin();

        let adjusted_center_x = center_x as f64 + offset_x;
        let adjusted_center_y = center_y as f64 + offset_y;

        // Create points for the pie slice
        let mut points = Vec::new();

        // Start from center
        points.push(Point::new(adjusted_center_x as i32, adjusted_center_y as i32));

        // Add arc points
        let num_segments = 32;
        let angle_step = (end_rad - start_rad) / num_segments as f64;

        for i in 0..=num_segments {
            let angle = start_rad + i as f64 * angle_step;
            let x = adjusted_center_x + radius * angle.cos();
            let y = adjusted_center_y + radius * angle.sin();
            points.push(Point::new(x as i32, y as i32));
        }

        // Draw the filled slice
        dc.set_brush(slice.color, BrushStyle::Solid);
        dc.set_pen(config.border_color, config.border_width, PenStyle::Solid);
        dc.draw_polygon(&points, 0, 0, PolygonFillMode::Winding);
    }
}
