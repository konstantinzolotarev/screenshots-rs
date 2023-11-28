mod image_utils;

use anyhow::{anyhow, Result};
use display_info::DisplayInfo;

pub use display_info;
pub use image;

#[cfg(target_os = "macos")]
mod darwin;
#[cfg(target_os = "macos")]
use darwin::*;

#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "windows")]
use win32::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::*;

/// This struct represents a screen capturer.
#[derive(Debug, Clone, Copy)]
pub struct Screen {
    pub display_info: DisplayInfo,
}

impl Screen {
    /// Get a screen from the [display_info].
    ///
    /// [display_info]:  https://docs.rs/display-info/latest/display_info/struct.DisplayInfo.html
    pub fn new(display_info: &DisplayInfo) -> Self {
        Screen {
            display_info: *display_info,
        }
    }

    /// Return all available screens.
    pub fn all() -> Result<Vec<Screen>> {
        let screens = DisplayInfo::all()?.iter().map(Screen::new).collect();
        Ok(screens)
    }

    /// Get a screen which includes the point with the given coordinates.
    pub fn from_point(x: i32, y: i32) -> Result<Screen> {
        let display_info = DisplayInfo::from_point(x, y)?;
        Ok(Screen::new(&display_info))
    }

    /// Capture a screenshot of the screen.
    pub fn capture(&self) -> Result<RawImage> {
        capture_screen(&self.display_info)
    }

    /// Captures a screenshot of the designated area of the screen.
    pub fn capture_area(&self, x: i32, y: i32, width: u32, height: u32) -> Result<RawImage> {
        let display_info = self.display_info;
        let screen_x2 = display_info.x + display_info.width as i32;
        let screen_y2 = display_info.y + display_info.height as i32;

        let mut x1 = x + display_info.x;
        let mut y1 = y + display_info.y;
        let mut x2 = x1 + width as i32;
        let mut y2 = y1 + height as i32;

        // x y 必须在屏幕范围内
        if x1 < display_info.x {
            x1 = display_info.x;
        } else if x1 > screen_x2 {
            x1 = screen_x2
        }

        if y1 < display_info.y {
            y1 = display_info.y;
        } else if y1 > screen_y2 {
            y1 = screen_y2;
        }

        if x2 > screen_x2 {
            x2 = screen_x2;
        }

        if y2 > screen_y2 {
            y2 = screen_y2;
        }

        if x1 >= x2 || y1 >= y2 {
            return Err(anyhow!("Area size is invalid"));
        }

        capture_screen_area(
            &display_info,
            x1 - display_info.x,
            y1 - display_info.y,
            (x2 - x1) as u32,
            (y2 - y1) as u32,
        )
    }
}

pub struct RawImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl RawImage {
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
        RawImage {
            width,
            height,
            data,
        }
    }
}
