// settings.rs
// Contains config structs for window settings

use serde::{Deserialize, Serialize};
use std::path::Path;

use wxdragon::prelude::*;

#[derive(Serialize, Deserialize, Default)]
pub struct WindowConfig {
    pub position: (i32, i32),
    pub size: (i32, i32),
}

pub(crate) const WIDGET_MARGIN: i32 = 2;
pub(crate) const APP_TITLE: &str = "Neet Demo";
pub(crate) const MAIN_ICON: &[u8] = include_bytes!("../assets/main.png");
pub(crate) const ICON_SIZE: u32 = 72;

impl WindowConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        std::fs::read_to_string(path.as_ref())
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(WindowConfig {
                position: (200, 250),
                size: (700, 400),
            })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        let _ = std::fs::write(path, serde_json::to_string_pretty(self).unwrap());
    }

    pub fn new(position: Point, size: Size) -> Self {
        Self {
            position: (position.x, position.y),
            size: (size.width, size.height),
        }
    }

    pub fn to_point(&self) -> Point {
        Point::new(self.position.0, self.position.1)
    }

    pub fn to_size(&self) -> Size {
        Size::new(self.size.0, self.size.1)
    }
}

pub fn load_settings() -> WindowConfig {
    let config_path: std::path::PathBuf = retrieve_config_path();
    WindowConfig::load(&config_path)
}

pub fn save_settings(cfg: &WindowConfig) {
    let config_path: std::path::PathBuf = retrieve_config_path();
    cfg.save(&config_path);
}

fn retrieve_config_path() -> std::path::PathBuf {
    let app_name = env!("CARGO_PKG_NAME");
    let config_path: std::path::PathBuf = ::dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(app_name);
    let _ = std::fs::create_dir_all(&config_path);
    config_path.join("settings.json")
}

pub fn create_bitmap_from_memory(data: &[u8], target_size: Option<(u32, u32)>) -> std::io::Result<Bitmap> {
    let img = image::load_from_memory(data).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    if let Some((w, h)) = target_size {
        use image::imageops::FilterType;
        let resized = img.resize_exact(w, h, FilterType::Lanczos3);
        convert_image_to_bitmap(&resized)
    } else {
        convert_image_to_bitmap(&img)
    }
}

pub fn convert_image_to_bitmap(image: &image::DynamicImage) -> std::io::Result<Bitmap> {
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    let icon_bitmap = Bitmap::from_rgba(&rgba, width, height).ok_or(std::io::Error::other("Failed to create bitmap"))?;
    Ok(icon_bitmap)
}

/// Center a rectangle of size (w, h) within the parent window
pub fn center_rect(parent: &dyn WxWidget, w: i32, h: i32) -> (i32, i32) {
    let parent_pos = parent.get_position();
    let parent_size = parent.get_size();
    let x = parent_pos.x + (parent_size.width - w) / 2;
    let y = parent_pos.y + (parent_size.height - h) / 2;
    (x, y)
}
