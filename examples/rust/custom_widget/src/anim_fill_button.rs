use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};
use wxdragon::dc::{AutoBufferedPaintDC, BrushStyle, PenStyle};
use wxdragon::prelude::*;

/// Animation state for the fill effect
#[derive(Debug, Clone, Copy, PartialEq)]
enum AnimationState {
    Idle,
    FillIn,
    FillOut,
}

/// Shared animation data
#[derive(Debug)]
struct AnimationData {
    state: AnimationState,
    progress: f32, // 0.0 to 1.0
    start_time: Option<Instant>,
    is_mouse_over: bool,
    is_mouse_pressed: bool,
}

// 使用新的 custom_widget! 宏创建动画填充按钮
custom_widget!(
    name: AniFillButton,
    fields: {
        text: String = "Animated Button".to_string(),
        border_color: Colour = Colour::new(100, 100, 100, 255),
        border_width: i32 = 2,
        border_radius: i32 = 8,
        background_color: Colour = Colour::new(240, 240, 240, 255),
        fill_background_color: Colour = Colour::new(100, 150, 255, 255),
        fill_background_duration: Duration = Duration::from_millis(300),
    },
    setup_impl: |config, panel| {
        // 设置背景样式为 Paint 以获得最佳的无闪烁绘制
        panel.set_background_style(BackgroundStyle::Paint);

        // 创建动画数据
        let animation_data = Rc::new(RefCell::new(AnimationData {
            state: AnimationState::Idle,
            progress: 0.0,
            start_time: None,
            is_mouse_over: false,
            is_mouse_pressed: false,
        }));

        // 创建定时器
        let timer = Rc::new(Timer::new(&panel));

        // 设置绘制事件
        let panel_paint = panel.clone();
        let animation_data_paint = animation_data.clone();
        let config_paint = config.clone();
        panel.on_paint(move |event| {
            let animation = animation_data_paint.borrow();
            AniFillButton::draw_custom_button(&panel_paint, &config_paint, animation.progress, animation.is_mouse_pressed);
            event.skip(true);
        });

        // 设置鼠标进入事件
        let animation_data_enter = animation_data.clone();
        let panel_enter = panel.clone();
        let timer_enter = timer.clone();
        panel.on_mouse_enter(move |event| {
            let mut animation = animation_data_enter.borrow_mut();
            if !animation.is_mouse_over {
                animation.is_mouse_over = true;
                animation.state = AnimationState::FillIn;
                animation.start_time = Some(Instant::now());
                drop(animation);

                timer_enter.start(16, false); // 60fps
                panel_enter.refresh(false, None);
            }
            event.skip(true);
        });

        // 设置鼠标离开事件
        let animation_data_leave = animation_data.clone();
        let panel_leave = panel.clone();
        let timer_leave = timer.clone();
        panel.on_mouse_leave(move |event| {
            let mut animation = animation_data_leave.borrow_mut();

            // 如果鼠标按下状态，强制释放
            if animation.is_mouse_pressed {
                animation.is_mouse_pressed = false;
            }

            // 处理鼠标离开的动画逻辑
            if animation.is_mouse_over {
                animation.is_mouse_over = false;
                animation.state = AnimationState::FillOut;
                animation.start_time = Some(Instant::now());
                drop(animation);

                timer_leave.start(16, false); // 60fps
                panel_leave.refresh(false, None);
            } else {
                drop(animation);
                panel_leave.refresh(false, None);
            }
            event.skip(true);
        });

        // 设置鼠标按下事件
        let animation_data_down = animation_data.clone();
        let panel_down = panel.clone();
        panel.on_mouse_left_down(move |event| {
            {
                let mut animation = animation_data_down.borrow_mut();
                animation.is_mouse_pressed = true;
            }
            panel_down.refresh(false, None);
            event.skip(true);
        });

        // 设置鼠标释放事件
        let animation_data_up = animation_data.clone();
        let panel_up = panel.clone();
        panel.on_mouse_left_up(move |event| {
            {
                let mut animation = animation_data_up.borrow_mut();
                animation.is_mouse_pressed = false;
            }
            panel_up.refresh(false, None);
            event.skip(true);
        });





        // 设置定时器事件
        let animation_data_timer = animation_data.clone();
        let config_timer = config.clone();
        let panel_timer = panel.clone();
        let timer_clone = timer.clone();
        timer.on_tick(move |event| {
            let mut animation = animation_data_timer.borrow_mut();

            if animation.state == AnimationState::Idle {
                drop(animation);
                timer_clone.stop();
                return;
            }

            if let Some(start_time) = animation.start_time {
                let elapsed = start_time.elapsed();
                let duration_ms = config_timer.fill_background_duration.as_millis() as f32;
                let progress_ratio = (elapsed.as_millis() as f32 / duration_ms).min(1.0);

                animation.progress = match animation.state {
                    AnimationState::FillIn => progress_ratio,
                    AnimationState::FillOut => 1.0 - progress_ratio,
                    AnimationState::Idle => 0.0,
                };

                panel_timer.refresh(false, None);

                // 动画完成时停止
                if progress_ratio >= 1.0 {
                    animation.state = AnimationState::Idle;
                    animation.start_time = None;
                    drop(animation);
                    timer_clone.stop();
                }
            }
            event.skip(true);
        });
    }
);

impl AniFillButton {
    fn draw_custom_button(panel: &Panel, config: &AniFillButtonConfig, progress: f32, is_pressed: bool) {
        let dc = AutoBufferedPaintDC::new(panel);
        let size = panel.get_size();
        let width = size.width;
        let height = size.height;
        let radius = config.border_radius as f64;

        // 清除背景（使用透明画刷）
        dc.set_brush(Colour::new(0, 0, 0, 0), BrushStyle::Transparent);
        dc.draw_rectangle(0, 0, width, height);

        // 1. 绘制基础背景（不考虑pressed状态）
        dc.set_brush(config.background_color, BrushStyle::Solid);
        dc.set_pen(Colour::new(0, 0, 0, 0), 0, PenStyle::Transparent);
        dc.draw_rounded_rectangle(0, 0, width, height, radius);

        // 2. 绘制填充动画（从左到右的填充效果，不考虑pressed状态）
        if progress > 0.0 {
            let fill_width = (width as f32 * progress) as i32;

            // 使用裁剪区域实现填充效果
            dc.set_pen(Colour::new(0, 0, 0, 0), 0, PenStyle::Transparent);
            dc.set_brush(config.fill_background_color, BrushStyle::Solid);

            // 设置裁剪区域来限制填充范围
            dc.set_clipping_region(0, 0, fill_width, height);
            dc.draw_rounded_rectangle(0, 0, width, height, radius);
            dc.destroy_clipping_region();
        }

        // 3. 如果处于按下状态，对整个按钮区域应用darken效果
        if is_pressed {
            // 创建一个更强的半透明深色覆盖层来实现darken效果
            let overlay_color = Colour::new(0, 0, 0, 100); // 大约40%的透明度，更明显
            dc.set_brush(overlay_color, BrushStyle::Solid);
            dc.set_pen(Colour::new(0, 0, 0, 0), 0, PenStyle::Transparent);
            dc.draw_rounded_rectangle(0, 0, width, height, radius);
        }

        // 4. 绘制边框（圆角矩形）
        if config.border_width > 0 {
            let border_color = if is_pressed {
                Self::darken_color(config.border_color, 0.3)
            } else {
                config.border_color
            };
            dc.set_pen(border_color, config.border_width, PenStyle::Solid);
            dc.set_brush(Colour::new(0, 0, 0, 0), BrushStyle::Transparent);
            dc.draw_rounded_rectangle(0, 0, width, height, radius);
        }

        // 5. 绘制居中文字
        let text_color = if is_pressed {
            Colour::new(20, 20, 20, 255) // 按下时文字更深一些
        } else {
            Colour::new(50, 50, 50, 255)
        };
        dc.set_text_foreground(text_color);
        let text_size = dc.get_text_extent(&config.text);
        let text_x = (width - text_size.0) / 2;
        let text_y = (height - text_size.1) / 2;
        dc.draw_text(&config.text, text_x, text_y);
    }

    /// 加深颜色
    fn darken_color(color: Colour, factor: f32) -> Colour {
        color.darker(1.0 - factor.clamp(0.0, 1.0))
    }
}
